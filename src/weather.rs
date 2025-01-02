pub async fn get() -> anyhow::Result<Vec<String>> {
    let mut weathers = vec![];

    let url = "https://www.tianqi.com/nanjing/7/";
    let cli = reqwest::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
    );
    let html = cli.get(url).headers(headers).send().await?.text().await?;
    let document = scraper::Html::parse_document(&html);
    let selector = scraper::Selector::parse(r#"ul[class="weaul"] > li"#).unwrap();
    for element in document.select(&selector) {
        let weather = element.text().collect::<String>().trim().to_string().trim_end_matches("\n查看天气详情").to_string().replace("\n", " ");
        weathers.push(weather);
    }
    Ok(weathers)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_weather() {
        let weathers = tokio_test::block_on(get()).unwrap();
        println!("{:?}", weathers);
    }
}