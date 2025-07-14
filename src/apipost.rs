use crate::ApiContext;
use serde_json::Value;

pub async fn chat(ctx: &ApiContext, message: &str) -> Result<serde_json::Value, reqwest::Error> {
    let url = format!("{}/chat?password={}", ctx.base_url, ctx.password);
    let res = ctx.client
        .post(&url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(format!("message={}", urlencoding::encode(message)))
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

