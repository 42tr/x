use chrono::Utc;
use sqlx::MySqlPool;

use crate::utils;

pub async fn obtain(pool: &MySqlPool) -> anyhow::Result<()> {
    let token = get_token().await?;
    // 获取当前时间戳
    let timestamp = Utc::now().timestamp_millis();
    let url = format!("https://stock.xueqiu.com/v5/stock/chart/kline.json?symbol=SH000001&begin={timestamp}&period=day&type=before&count=-284");
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
    let rsp: Rsp = cli.get(url).headers(headers).send().await?.json().await?;
    let mut stocks: Vec<(i64, f32)> = vec![];
    for item in rsp.data.item {
        if let (Value::Float(timestamp), Value::Float(price)) = (&item[0], &item[5]) {
            stocks.push((*timestamp as i64, *price));
        }
    }
    save(pool, &stocks).await?;
    Ok(())
}

async fn save(pool: &MySqlPool, prices: &Vec<(i64, f32)>) -> anyhow::Result<()> {
    if prices.len() == 0 {
        return Ok(());
    }
    let mut query = String::from("insert ignore into stock_info (timestamp, price) values ");
    let params: Vec<String> = prices.iter().map(|_| format!("(?, ?)")).collect();
    query.push_str(&params.join(", "));
    let mut q = sqlx::query(&query);
    for price in prices {
        q = q.bind(price.0).bind(price.1);
    }

    q.execute(pool).await?;
    Ok(())
}

pub async fn get_info(pool: &MySqlPool) -> anyhow::Result<(Vec<String>, Vec<f32>)> {
    let gold_info_list = sqlx::query_as!(
            StockInfoPO,
            "select * from (select * from stock_info order by timestamp desc limit 365) t order by timestamp",
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
struct StockInfoPO {
    timestamp: i64,
    price: f32,
}

#[derive(Debug, serde::Deserialize)]
struct Rsp {
    data: Data,
}
#[derive(Debug, serde::Deserialize)]
struct Data {
    item: Vec<Vec<Value>>,
}
#[derive(serde::Deserialize, Debug)]
#[serde(untagged)] // 允许不带标签的多态数据
enum Value {
    Float(f32),
    Null,
}

async fn get_token() -> anyhow::Result<String> {
    // let url = "https://xueqiu.com/?md5__1038=QqGxcDnDyiitnD05o4%2Br%3DQIhbOW%3DD9e8oDx";
    let url = "https://xueqiu.com/?md5__1038=QqGxcDnDyiitnD05o4%2Br%3DD9lRKTMqD5dx";
    let cli = reqwest::Client::new();
    let resp = cli.get(url).send().await?;
    let headers = resp.headers();
    let token = headers
        .get_all("set-cookie")
        .into_iter()
        .map(|x| x.to_str().unwrap())
        .collect::<Vec<_>>()
        .join("; ");
    Ok(token.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_token() {
        let token = tokio_test::block_on(get_token()).unwrap();
        println!("token: {}", token);
    }

    #[test]
    fn test_get_timestamp() {
        println!("now: {}", Utc::now().timestamp_millis());
    }
}
