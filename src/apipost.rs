use crate::ApiContext;
use serde_json::Value;
use urlencoding::encode;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH};

pub async fn api_post(ctx: &ApiContext, url: &str) -> Result<Value, reqwest::Error> {
    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let res = ctx.client
        .post(url)
        .headers(headers)
        .body("")
        .send()
        .await?;

    res.json().await
}

pub async fn chat(ctx: &ApiContext, message: &str) -> Result<Value, reqwest::Error> {
    let url = format!(
        "{}/chat?password={}&message={}",
        ctx.base_url,
        encode(&ctx.password),
        encode(message)
    );
    api_post(ctx, &url).await
}

pub async fn player_kick(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!(
        "{}/player/kick?password={}&unique_id={}",
        ctx.base_url,
        encode(&ctx.password),
        encode(unique_id)
    );
    api_post(ctx, &url).await
}

pub async fn player_ban(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!(
        "{}/player/ban?password={}&unique_id={}",
        ctx.base_url,
        encode(&ctx.password),
        encode(unique_id)
    );
    api_post(ctx, &url).await
}

pub async fn player_unban(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!(
        "{}/player/unban?password={}&unique_id={}",
        ctx.base_url,
        encode(&ctx.password),
        encode(unique_id)
    );
    api_post(ctx, &url).await
}

