use scraper::Scraper;
use vercel_runtime::{
    http::internal_server_error, run, Body, Error, Request, Response, StatusCode,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    run(handler).await
}

pub async fn handler(_req: Request) -> Result<Response<Body>, Error> {
    let scraper = Scraper::from_env().await;
    let Ok(scraper) = scraper else {
        return internal_server_error(scraper.unwrap_err().to_string());
    };

    let res = scraper.run().await;
    if let Err(err) = res {
        return internal_server_error(err.to_string());
    }

    Ok(Response::builder()
        .status(StatusCode::NO_CONTENT)
        .body(Body::Empty)?)
}
