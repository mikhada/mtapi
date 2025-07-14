use crate::ApiContext;
use serde_json::Value;

pub async fn ban_player(ctx: &ApiContext, unique_id: &str) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/ban?password={}", ctx.base_url, ctx.password);
    let res = ctx.client
        .post(&url)
        .form(&[("unique_id", unique_id)])
        .send()
        .await?;
    res.json().await
}

