use chrono::{TimeZone, Utc};

pub async fn get_info() -> anyhow::Result<(Vec<String>, Vec<f32>)> {
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
    let resp: Rsp = cli.get(url).headers(headers).send().await?.json().await?;

    let mut dates = vec![];
    let mut prices = vec![];
    resp.data.item.iter().for_each(|item| {
        match &item[0] {
            Value::Float(f) => dates.push(
                Utc.timestamp_opt(*f as i64 / 1000, 0)
                    .unwrap()
                    .format("%m-%d")
                    .to_string(),
            ),
            _default => {}
        }
        match &item[5] {
            Value::Float(f) => prices.push(*f),
            _default => {}
        }
    });
    Ok((dates, prices))
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
    fn test_get_info() {
        let (name, link) = tokio_test::block_on(get_info()).unwrap();
        println!("name: {:?}, link: {:?}", name, link);
    }

    #[test]
    fn test_get_token() {
        let token = tokio_test::block_on(get_token()).unwrap();
        println!("token: {}", token);
    }
}
