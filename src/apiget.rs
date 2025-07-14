use crate::ApiContext;
use serde_json::Value;

pub async fn player_count(ctx: &ApiContext) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/count?password={}", ctx.base_url, ctx.password);
    let res = ctx.client.get(&url).send().await?;
    res.json().await
}

pub async fn player_list(ctx: &ApiContext) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/list?password={}", ctx.base_url, ctx.password);
    let res = ctx.client.get(&url).send().await?;
    res.json().await
}

pub async fn player_banlist(ctx: &ApiContext) -> Result<Value, reqwest::Error> {
    let url = format!("{}/player/banlist?password={}", ctx.base_url, ctx.password);
    let res = ctx.client.get(&url).send().await?;
    res.json().await
}

pub async fn housing_list(ctx: &ApiContext) -> Result<Value, reqwest::Error> {
    let url = format!("{}/housing/list?password={}", ctx.base_url, ctx.password);
    let res = ctx.client.get(&url).send().await?;
    res.json().await
}

pub async fn version(ctx: &ApiContext) -> Result<Value, reqwest::Error> {
    let url = format!("{}/version?password={}", ctx.base_url, ctx.password);
    let res = ctx.client.get(&url).send().await?;
    res.json().await
}

