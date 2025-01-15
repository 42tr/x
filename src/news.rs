use chrono::{TimeZone, Utc};
use chrono_tz::Tz;

pub async fn get() -> anyhow::Result<(Vec<String>, Vec<String>, Vec<String>)> {
    let token = get_token().await?;
    println!("xueqiu token: {}", token);
    let mut max_id = 0;
    let timestamp = Utc::now().timestamp_millis() - 24 * 60 * 60 * 1000;
    let (mut times, mut links, mut titles) = (vec![], vec![], vec![]);
    loop {
        let url = format!("https://xueqiu.com/statuses/livenews/list.json?count=15&max_id={max_id}");

        let cli = reqwest::Client::new();
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
        );
        headers.insert("Content-Type", 
            reqwest::header::HeaderValue::from_str("application/json").unwrap(),
        );
        headers.insert("Cookie", reqwest::header::HeaderValue::from_str(token.as_str()).unwrap());
        headers.insert("Referer", reqwest::header::HeaderValue::from_str("https://xueqiu.com/").unwrap());
        println!("xueqiu request: {:?}", headers);
        let content = cli.get(url.clone()).headers(headers.clone()).send().await?.text().await?;
        println!("xueqiu response: {}", content);
        let resp: Rsp = cli.get(url).headers(headers).send().await?.json().await?;
        if resp.items.len() > 0 && resp.items[0].created_at < timestamp {
            break;
        }
        max_id = resp.next_max_id;
        let shanghai: Tz = "Asia/Shanghai".parse().unwrap();
        // news.extend(resp.items.into_iter());
        resp.items.iter().for_each(|news| {
            times.push(shanghai.timestamp_opt(news.created_at / 1000, 0).unwrap().format("%m-%d %H:%M").to_string());
            links.push(news.target.to_string());
            titles.push(news.text.to_string());
        });
    }
    Ok((times, links, titles))
}


#[derive(Debug, serde::Deserialize)]
struct Rsp {
    next_max_id: i32,
    items: Vec<News>,
}

#[derive(Debug, serde::Deserialize)]
pub struct News {
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
    let token = headers.get_all("set-cookie").into_iter().map(|x| {
        x.to_str().unwrap()
    }).collect::<Vec<_>>().join("; ");
    Ok(token.to_string())
}