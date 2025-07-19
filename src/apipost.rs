use crate::ApiContext;
use serde_json::Value;
use urlencoding::encode;
use reqwest::header::{HeaderMap, HeaderValue, CONTENT_LENGTH};

pub async fn chat(ctx: &ApiContext, message: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!(
        "{}/chat?password={}&message={}",
        ctx.base_url,
        encode(&ctx.password),
        encode(message)
    );

    let mut headers = HeaderMap::new();
    headers.insert(CONTENT_LENGTH, HeaderValue::from_static("0"));

    let res = ctx.client
        .post(&url)
        .headers(headers)
        .body("")
        .send()
        .await?;

    res.json().await
}

pub async fn player_kick(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/kick?password={}", ctx.base_url, ctx.password);
    let res = ctx.client
        .post(&url)
        .form(&[("unique_id", unique_id)])
        .send()
        .await?;
    res.json().await
}

pub async fn player_ban(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/ban?password={}", ctx.base_url, ctx.password);
    let res = ctx.client
        .post(&url)
        .form(&[("unique_id", unique_id)])
        .send()
        .await?;
    res.json().await
}

pub async fn player_unban(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/ban?password={}", ctx.base_url, ctx.password);
    let res = ctx.client
        .post(&url)
        .form(&[("unique_id", unique_id)])
        .send()
        .await?;
    res.json().await
}

