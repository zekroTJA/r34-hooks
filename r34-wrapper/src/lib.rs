mod errors;
mod models;

pub use errors::*;
pub use models::*;

const API_ROOT: &str = "https://api.rule34.xxx/index.php";

pub struct Client {
    client: reqwest::Client,
    user_id: String,
    api_key: String,
}

impl Client {
    pub fn new(user_id: &str, api_key: &str) -> Self {
        Self {
            user_id: user_id.to_string(),
            api_key: api_key.to_string(),
            client: reqwest::Client::new(),
        }
    }

    pub async fn list_posts(
        &self,
        tags: &[String],
        page: Option<u64>,
        limit: Option<u64>,
    ) -> Result<Posts> {
        self.req(Some(tags), page, limit, None).await
    }

    pub async fn get_post(&self, id: u64) -> Result<Option<Post>> {
        let posts = self.req(None, None, None, Some(id)).await?;
        if posts.posts.is_empty() {
            return Ok(None);
        }
        Ok(Some(posts.posts[0].clone()))
    }

    async fn req(
        &self,
        tags: Option<&[String]>,
        page: Option<u64>,
        limit: Option<u64>,
        id: Option<u64>,
    ) -> Result<Posts> {
        let mut req = self.client.get(API_ROOT).query(&[
            ("page", "dapi"),
            ("s", "post"),
            ("q", "index"),
            ("user_id", &self.user_id),
            ("api_key", &self.api_key),
        ]);

        if let Some(tags) = tags {
            req = req.query(&[("tags", &tags.join(" "))]);
        }

        if let Some(page) = page {
            req = req.query(&[("pid", page)]);
        }

        if let Some(limit) = limit {
            req = req.query(&[("limit", limit)]);
        }

        if let Some(id) = id {
            req = req.query(&[("id", id)]);
        }

        let res = req.send().await.map_err(Error::RequestError)?;
        let res = res.error_for_status().map_err(Error::ResponseStatusError)?;

        let data = res.text().await.map_err(Error::ResponseBodyReadError)?;

        quick_xml::de::from_str(&data).map_err(Error::XmlParsingError)
    }
}
