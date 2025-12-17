use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Posts {
    #[serde(rename(deserialize = "@count"))]
    pub count: u64,
    #[serde(rename(deserialize = "@offset"))]
    pub offset: u64,

    #[serde(default)]
    #[serde(rename(deserialize = "$value"))]
    pub posts: Vec<Post>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Post {
    #[serde(rename(deserialize = "@height"))]
    pub height: u64,
    #[serde(rename(deserialize = "@score"))]
    pub score: u64,
    #[serde(rename(deserialize = "@file_url"))]
    pub file_url: String,
    #[serde(rename(deserialize = "@parent_id"))]
    pub parent_id: String,
    #[serde(rename(deserialize = "@sample_url"))]
    pub sample_url: String,
    #[serde(rename(deserialize = "@sample_width"))]
    pub sample_width: u64,
    #[serde(rename(deserialize = "@sample_height"))]
    pub sample_height: u64,
    #[serde(rename(deserialize = "@preview_url"))]
    pub preview_url: String,
    #[serde(rename(deserialize = "@rating"))]
    pub rating: String,
    #[serde(rename(deserialize = "@tags"))]
    pub tags_concat: String,
    #[serde(rename(deserialize = "@id"))]
    pub id: u64,
    #[serde(rename(deserialize = "@width"))]
    pub width: u64,
    #[serde(rename(deserialize = "@change"))]
    pub change: u64,
    #[serde(rename(deserialize = "@md5"))]
    pub md5: String,
    #[serde(rename(deserialize = "@creator_id"))]
    pub creator_id: u64,
    #[serde(rename(deserialize = "@has_children"))]
    pub has_children: bool,
    #[serde(rename(deserialize = "@created_at"))]
    pub created_at: String,
    #[serde(rename(deserialize = "@source"))]
    pub source: String,
    #[serde(rename(deserialize = "@has_notes"))]
    pub has_notes: bool,
    #[serde(rename(deserialize = "@has_comments"))]
    pub has_comments: bool,
    #[serde(rename(deserialize = "@preview_width"))]
    pub preview_width: u64,
    #[serde(rename(deserialize = "@preview_height"))]
    pub preview_height: u64,
}

impl Post {
    pub fn tags(&self) -> Vec<&str> {
        self.tags_concat
            .trim()
            .split(' ')
            .map(|v| v.trim())
            .filter(|v| !v.is_empty())
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn single_post() {
        let post = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <post
            height="2894"
            score="27"
            file_url="https://api-cdn.rule34.xxx/images/6811/aa5d4c4640b1dee8f59bb9245f598a4c.png"
            parent_id=""
            sample_url="https://api-cdn.rule34.xxx/samples/6811/sample_aa5d4c4640b1dee8f59bb9245f598a4c.jpg"
            sample_width="850"
            sample_height="601"
            preview_url="https://api-cdn.rule34.xxx/thumbnails/6811/thumbnail_aa5d4c4640b1dee8f59bb9245f598a4c.jpg"
            rating="e"
            tags=" anal anal_sex big_ass big_breasts big_penis bigger_female blue_hair cowgirl_position cum_in_ass doggy_style kayo_kz kindred lamb_(league_of_legends) league_of_legends monster_girl neeko on_back on_top riot_games smaller_female smaller_futanari tail wide_hips "
            id="7775085"
            width="4093"
            change="1681853117"
            md5="aa5d4c4640b1dee8f59bb9245f598a4c"
            creator_id="2486912"
            has_children="false"
            created_at="Tue Apr 18 21:24:51 +0000 2023"
            status="active"
            source="https://twitter.com/Kayo_Kz/status/1648436562315853824?s=20"
            has_notes="false"
            has_comments="true"
            preview_width="150"
            preview_height="106"
        />
        "#;

        let post: Result<Post, _> = quick_xml::de::from_str(post);
        assert!(post.is_ok(), "{}", post.unwrap_err().to_string());

        let post = post.unwrap();
        assert_eq!(post.height, 2894);
        assert_eq!(
            post.source,
            "https://twitter.com/Kayo_Kz/status/1648436562315853824?s=20"
        );
    }

    #[test]
    fn multiple_posts() {
        let posts = r#"
        <?xml version="1.0" encoding="UTF-8"?>
        <posts count="4187" offset="0">
            <post height="2894" score="27" file_url="https://api-cdn.rule34.xxx/images/6811/aa5d4c4640b1dee8f59bb9245f598a4c.png" parent_id="" sample_url="https://api-cdn.rule34.xxx/samples/6811/sample_aa5d4c4640b1dee8f59bb9245f598a4c.jpg" sample_width="850" sample_height="601" preview_url="https://api-cdn.rule34.xxx/thumbnails/6811/thumbnail_aa5d4c4640b1dee8f59bb9245f598a4c.jpg" rating="e" tags=" anal anal_sex big_ass big_breasts big_penis bigger_female blue_hair cowgirl_position cum_in_ass doggy_style kayo_kz kindred lamb_(league_of_legends) league_of_legends monster_girl neeko on_back on_top riot_games smaller_female smaller_futanari tail wide_hips " id="7775085" width="4093" change="1681853117" md5="aa5d4c4640b1dee8f59bb9245f598a4c" creator_id="2486912" has_children="false" created_at="Tue Apr 18 21:24:51 +0000 2023" status="active" source="https://twitter.com/Kayo_Kz/status/1648436562315853824?s=20" has_notes="false" has_comments="true" preview_width="150" preview_height="106" />
            <post height="1414" score="11" file_url="https://api-cdn.rule34.xxx/images/6809/bf992238ce25a93f00113dfe1447dad1fbfb462d.png" parent_id="" sample_url="https://api-cdn.rule34.xxx/images/6809/bf992238ce25a93f00113dfe1447dad1fbfb462d.png" sample_width="1000" sample_height="1414" preview_url="https://api-cdn.rule34.xxx/thumbnails/6809/thumbnail_bf992238ce25a93f00113dfe1447dad1fbfb462d.jpg" rating="e" tags=" 1boy ? ?? animal_ears bare_shoulders beard bite_mark_on_ass black_pants blue_tongue blush border colored_tongue cunnilingus english_text facial_hair female forest glowing glowing_eyes highres holding holding_shovel hood hood_up kindred lamb_(league_of_legends) league_of_legends long_tongue nature one_eye_closed open_mouth oral outdoors pants saliva sheep_ears sheep_tail shiny_skin shovel speech_bubble strongbana tail tongue tongue_out tree white_border wolf_(league_of_legends) yorick " id="7772510" width="1000" change="1681846874" md5="3d7b166d7cf2ab6f8bd101b2749b9a5b" creator_id="48613" has_children="false" created_at="Tue Apr 18 11:10:50 +0000 2023" status="active" source="https://i.pximg.net/img-original/img/2022/05/09/14/28/14/98225528_p7.png" has_notes="false" has_comments="false" preview_width="106" preview_height="150"/>
            <post height="1414" score="13" file_url="https://api-cdn.rule34.xxx/images/6809/8fdebbe289a51992d09ce2bc50483bac066072c7.png" parent_id="" sample_url="https://api-cdn.rule34.xxx/images/6809/8fdebbe289a51992d09ce2bc50483bac066072c7.png" sample_width="1000" sample_height="1414" preview_url="https://api-cdn.rule34.xxx/thumbnails/6809/thumbnail_8fdebbe289a51992d09ce2bc50483bac066072c7.jpg" rating="e" tags=" animal_ears anus ass ass_biting bar_censor biting blush body_fur breasts censored english_text female fur_collar grey_fur grey_hair highres kindred lamb_(league_of_legends) large_breasts league_of_legends long_hair nipples open_mouth pussy sheep_ears sheep_tail shiny_skin speech_bubble strongbana sweat tail teeth upper_teeth_only wolf_(league_of_legends) " id="7772509" width="1000" change="1681846839" md5="06c0f2b1576b6d157d14dbd699dd857f" creator_id="48613" has_children="false" created_at="Tue Apr 18 11:10:46 +0000 2023" status="active" source="https://i.pximg.net/img-original/img/2022/05/09/14/28/14/98225528_p6.png" has_notes="false" has_comments="true" preview_width="106" preview_height="150"/>
        </posts>
        "#;

        let posts: Result<Posts, _> = quick_xml::de::from_str(posts);
        assert!(posts.is_ok(), "{}", posts.unwrap_err().to_string());

        assert_eq!(posts.unwrap().posts.len(), 3);
    }
}
