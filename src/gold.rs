use chrono::{TimeZone, Utc};
use charts_rs::{ BarChart, Box, SeriesCategory, THEME_ANT, svg_to_png };

/// 生成折线图
/// 返回折线图的名称
pub async fn create_line() -> anyhow::Result<String> {
    let (dates, prices) = get_info().await?;

    let mut axis_min = 10000.0;
    prices.clone().into_iter().for_each(|price| {
        if price < axis_min {
            axis_min = price;
        }
    });
    axis_min = ((axis_min as i32 / 10) * 10) as f32;
    let mut bar_chart = BarChart::new_with_theme(
        vec![("RMB", prices).into(),],
        dates,
        THEME_ANT,
    );
    bar_chart.y_axis_configs[0].axis_min = Some(axis_min);

    bar_chart.width = 1200.0;
    bar_chart.title_text = "Gold Info".to_string();
    bar_chart.legend_margin = Some(Box {
        top: bar_chart.title_height,
        bottom: 5.0,
        ..Default::default()
    });
    bar_chart.series_list[0].category = Some(SeriesCategory::Line);
    bar_chart.series_list[0].y_axis_index = 1;
    bar_chart.series_list[0].label_show = false;
    let png = svg_to_png(&bar_chart.svg().unwrap()).unwrap();
    std::fs::write("gold_line.png", png).unwrap();
    Ok("gold_line.png".to_string())
}

/// 获取黄金价格
async fn get_info() -> anyhow::Result<(Vec<String>, Vec<f32>)> {
    let cli = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
    );
    headers.insert(
        "referer",
        reqwest::header::HeaderValue::from_str("https://quote.cngold.org/gjs/swhj_zghj.html").unwrap(),
    );
    let url = "https://api.jijinhao.com/quoteCenter/historys.htm?codes=JO_52683&style=3&pageSize=180";
    let resp = cli.get(url).headers(headers).send().await?.text().await?;
    let resp = resp.replace("var quote_json = ", "");
    let rsp: Rsp = serde_json::from_str(&resp).unwrap();
    let data = rsp.data.prices;
    let mut dates = vec![];
    let mut prices = vec![];
    data.iter().for_each(|d| {
        dates.push(Utc.timestamp_opt(d.time / 1000, 0).unwrap().format("%m-%d").to_string());
        prices.push(d.q1);
    });
    Ok((dates, prices))
}

#[derive(Debug, serde::Deserialize)]
pub struct Price {
    q1: f32,
    time: i64,
}

#[derive(Debug, serde::Deserialize)]
pub struct Data {
    #[serde(rename = "JO_52683")]
    prices: Vec<Price>,
}

#[derive(Debug, serde::Deserialize)]
pub struct Rsp {
    data: Data,
}