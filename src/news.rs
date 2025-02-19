use anyhow::Ok;
use chrono::{TimeZone, Utc};
use chrono_tz::Tz;
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
        println!("xueqiu request: {:?}", headers);
        let content = cli
            .get(url.clone())
            .headers(headers.clone())
            .send()
            .await?
            .text()
            .await?;
        println!("xueqiu response: {}", content);
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
    save_news_list(pool, &news_list).await?;
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
    let shanghai: Tz = "Asia/Shanghai".parse().unwrap();
    news_list.iter().for_each(|news| {
        times.push(
            shanghai
                .timestamp_opt(news.timestamp / 1000, 0)
                .unwrap()
                .format("%m-%d %H:%M")
                .to_string(),
        );
        titles.push(news.content.to_string());
    });
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
    println!("token response: {:?}", headers);
    let token = headers
        .get_all("set-cookie")
        .into_iter()
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<_>>()
        .join("; ");
    Ok(token.to_string())
}

/// 插入数据库
/// 可能会有重复的，使用 insert ignore 插入
async fn save_news_list(pool: &MySqlPool, news_list: &Vec<NewsPO>) -> anyhow::Result<u64> {
    let mut query =
        String::from("insert ignore into news (id, content, timestamp, target) values ");
    let params: Vec<String> = news_list.iter().map(|_| format!("(?, ?, ?, ?)")).collect();
    query.push_str(&params.join(", "));
    let mut q = sqlx::query(&query);
    for news in news_list {
        q = q
            .bind(&news.id)
            .bind(&news.content)
            .bind(&news.timestamp)
            .bind(&news.target);
    }

    let result = q.execute(pool).await?;
    Ok(result.rows_affected())
}

async fn get_latest_id(pool: &MySqlPool) -> u32 {
    let id: u32 = sqlx::query_scalar("select id from news order by id desc limit 1")
        .fetch_one(pool)
        .await
        .unwrap_or_else(|_| 3812705);
    println!("latest id: {id}");
    id
}
