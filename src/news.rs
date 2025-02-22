use crate::utils;

use anyhow::Ok;
use chrono::Utc;
use log::debug;
use sqlx::mysql::MySqlPool;

pub async fn obtain_latest_news(pool: &MySqlPool) -> anyhow::Result<()> {
    let id = get_latest_id(pool).await;
    let token = get_token().await?;
    let mut max_id = 0;
    let mut news_list: Vec<NewsPO> = vec![];
    loop {
        let url =
            format!("https://xueqiu.com/statuses/livenews/list.json?count=15&max_id={max_id}");

        let cli = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
        );
        headers.insert(
            "Content-Type",
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert(
            "Cookie",
            reqwest::header::HeaderValue::from_str(token.as_str()).unwrap(),
        );
        headers.insert(
            "Referer",
            reqwest::header::HeaderValue::from_str("https://xueqiu.com/").unwrap(),
        );
        debug!("xueqiu request: {:?}", headers);
        let content = cli
            .get(url.clone())
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;
        debug!("xueqiu response: {}", content);
        let resp: Rsp = cli.get(url).headers(headers).send().await?.json().await?;
        if resp.items.len() == 0 || resp.items[0].id <= id {
            break;
        }
        max_id = resp.next_max_id;
        resp.items.iter().for_each(|news| {
            news_list.push(NewsPO {
                id: news.id,
                content: news.text.clone(),
                timestamp: news.created_at,
                target: news.target.clone(),
            })
        });
    }
    for news in &news_list {
        if save_news(pool, news).await.is_ok() {
            utils::send_message(&news.content).await?;
        }
    }
    Ok(())
}

pub async fn get(pool: &MySqlPool) -> anyhow::Result<(Vec<String>, Vec<String>)> {
    let news_list = sqlx::query_as!(
        NewsPO,
        "select * from news where timestamp > ?",
        Utc::now().timestamp_millis() - 24 * 60 * 60 * 1000
    )
    .fetch_all(pool)
    .await?;
    let (mut times, mut titles) = (vec![], vec![]);
    for news in news_list {
        times.push(utils::timestamp2time(news.timestamp / 1000, "%m-%d %H:%M"));
        titles.push(news.content.to_string());
    }
    Ok((times, titles))
}

#[derive(Debug, serde::Deserialize)]
struct NewsPO {
    id: u32,
    content: String,
    timestamp: i64,
    target: String,
}

#[derive(Debug, serde::Deserialize)]
struct Rsp {
    next_max_id: u32,
    items: Vec<News>,
}

#[derive(Debug, serde::Deserialize)]
pub struct News {
    id: u32,
    text: String,
    created_at: i64,
    target: String,
}

async fn get_token() -> anyhow::Result<String> {
    // let url = "https://xueqiu.com/?md5__1038=QqGxcDnDyiitnD05o4%2Br%3DQIhbOW%3DD9e8oDx";
    let url = "https://xueqiu.com/?md5__1038=QqGxcDnDyiitnD05o4%2Br%3DD9lRKTMqD5dx";
    let cli = reqwest::Client::new();
    let resp = cli.get(url).send().await?;
    let headers = resp.headers();
    debug!("token response: {:?}", headers);
    let token = headers
        .get_all("set-cookie")
        .into_iter()
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<_>>()
        .join("; ");
    Ok(token.to_string())
}

async fn save_news(pool: &MySqlPool, news: &NewsPO) -> anyhow::Result<u64> {
    let mut query =
        sqlx::query("insert into news(id, content, timestamp, target) values (?, ?, ?, ?)");
    query = query
        .bind(&news.id)
        .bind(&news.content)
        .bind(&news.timestamp)
        .bind(&news.target);
    let result = query.execute(pool).await?;
    Ok(result.rows_affected())
}

async fn get_latest_id(pool: &MySqlPool) -> u32 {
    let id: u32 = sqlx::query_scalar("select id from news order by id desc limit 1")
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| 3812705);
    debug!("latest id: {id}");
    id
}
