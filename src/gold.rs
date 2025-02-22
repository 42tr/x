use sqlx::MySqlPool;

use crate::utils;

/// 获取最新的黄金价格
pub async fn obtain(pool: &MySqlPool) -> anyhow::Result<()> {
    let cli = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
            "User-Agent",
            reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
        );
    headers.insert(
        "referer",
        reqwest::header::HeaderValue::from_str("https://quote.cngold.org/gjs/swhj_zghj.html")
            .unwrap(),
    );
    let url =
        "https://api.jijinhao.com/quoteCenter/historys.htm?codes=JO_52683&style=3&pageSize=180";
    let resp = cli.get(url).headers(headers).send().await?.text().await?;
    let resp = resp.replace("var quote_json = ", "");
    let rsp: Rsp = serde_json::from_str(&resp)?;
    save(pool, &rsp.data.prices).await?;
    Ok(())
}

async fn save(pool: &MySqlPool, prices: &Vec<Price>) -> anyhow::Result<()> {
    if prices.len() == 0 {
        return Ok(());
    }
    let mut query = String::from("insert ignore into gold_info (timestamp, price) values ");
    let params: Vec<String> = prices.iter().map(|_| format!("(?, ?)")).collect();
    query.push_str(&params.join(", "));
    let mut q = sqlx::query(&query);
    for price in prices {
        q = q.bind(price.time).bind(price.q1);
    }

    q.execute(pool).await?;
    Ok(())
}

/// 获取黄金价格
pub async fn get_info(pool: &MySqlPool) -> anyhow::Result<(Vec<String>, Vec<f32>)> {
    let gold_info_list = sqlx::query_as!(
        GoldInfoPO,
        "select * from (select * from gold_info order by timestamp desc limit 365) t order by timestamp",
    )
    .fetch_all(pool)
    .await?;
    let (mut dates, mut prices) = (vec![], vec![]);
    for gold_info in gold_info_list {
        dates.push(utils::timestamp2time(gold_info.timestamp / 1000, "%m-%d"));
        prices.push(gold_info.price);
    }
    Ok((dates, prices))
}

#[derive(Debug, serde::Deserialize)]
struct GoldInfoPO {
    timestamp: i64,
    price: f32,
}

#[derive(Debug, serde::Deserialize)]
struct Price {
    q1: f32,
    time: i64,
}

#[derive(Debug, serde::Deserialize)]
struct Data {
    #[serde(rename = "JO_52683")]
    prices: Vec<Price>,
}

#[derive(Debug, serde::Deserialize)]
struct Rsp {
    data: Data,
}
