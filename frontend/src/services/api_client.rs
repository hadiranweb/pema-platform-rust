use gloo_net::http::Request;
use crate::shared::models::page::Page;
use anyhow::Error;
use yew::use_context;
use crate::config::FrontendConfig;

pub async fn fetch_pages() -> Result<Vec<Page>, Error> {
    let config = use_context::<FrontendConfig>().expect("FrontendConfig not found");
    let url = format!("{}/api/pages", config.api_base_url);
    let response = Request::get(&url)
        .send()
        .await?;

    if response.ok() {
        let pages: Vec<Page> = response.json().await?;
        Ok(pages)
    } else {
        Err(anyhow::anyhow!("Failed to fetch pages"))
    }
}

pub async fn fetch_page_by_id(id: i32) -> Result<Page, Error> {
    let config = use_context::<FrontendConfig>().expect("FrontendConfig not found");
    let url = format!("{}/api/pages/{}", config.api_base_url, id);
    let response = Request::get(&url)
        .send()
        .await?;

    if response.ok() {
        let page: Page = response.json().await?;
        Ok(page)
    } else {
        Err(anyhow::anyhow!("Failed to fetch page"))
    }
}

