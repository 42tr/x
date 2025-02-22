use crate::utils;

/// 获取黄金价格
pub async fn get_info() -> anyhow::Result<(Vec<String>, Vec<f32>)> {
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
    let rsp: Rsp = serde_json::from_str(&resp).unwrap();
    let mut dates = vec![];
    let mut prices = vec![];
    for d in rsp.data.prices {
        dates.push(utils::timestamp2time(d.time / 1000, "%m-%d"));
        prices.push(d.q1);
    }
    Ok((dates, prices))
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
