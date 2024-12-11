mod email;

fn main() {
    // email::send(
    //     "1055894396@qq.com".to_string(),
    //     "email from rust".to_string(),
    //     "This is a test email from lettre".to_string(),
    // );
    let cli = reqwest::blocking::Client::new();
    let mut headers = reqwest::header::HeaderMap::new();
    // headers.insert(
    //     "Accept-Encoding",
    //     reqwest::header::HeaderValue::from_str("identity").unwrap(),
    // );
    headers.insert(
        "User-Agent",
        reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/131.0.0.0 Safari/537.36").unwrap(),
    );
    headers.insert(
        "Sec-Ch-Ua",
        reqwest::header::HeaderValue::from_str(
            r#""Google Chrome";v="131", "Chromium";v="131", "Not_A Brand";v="24""#,
        )
        .unwrap(),
    );
    headers.insert(
        "Sec-Ch-Ua-Mobile", 
        reqwest::header::HeaderValue::from_str("?0").unwrap());
    headers.insert(
        "Sec-Ch-Ua-Platform", 
        reqwest::header::HeaderValue::from_str(r#""macOS""#).unwrap());
    headers.insert(
        "Sec-Ch-Ua-Mobile", 
        reqwest::header::HeaderValue::from_str("?0").unwrap());
    headers.insert(
        "Sec-Fetch-Dest",
        reqwest::header::HeaderValue::from_str("document").unwrap(),
    );
    headers.insert(
        "Sec-Fetch-Mode",
        reqwest::header::HeaderValue::from_str("navigate").unwrap());
    headers.insert(
        "Sec-Fetch-Site",
        reqwest::header::HeaderValue::from_str("same-origin").unwrap());
    headers.insert(
        "Sec-Fetch-User", 
        reqwest::header::HeaderValue::from_str("?1").unwrap());
    headers.insert(
        "Upgrade-Insecure-Requests",
        reqwest::header::HeaderValue::from_str("1").unwrap());
    headers.insert(
        "Referer",
        reqwest::header::HeaderValue::from_str("https://www.colamanga.com/show").unwrap());
    headers.insert(
        "Priority",
        reqwest::header::HeaderValue::from_str("u=0, i").unwrap());
    headers.insert(
        "Pragma",
        reqwest::header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(
        "Accept",
        reqwest::header::HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.7").unwrap());
    headers.insert(
        "Accept-Language",
        reqwest::header::HeaderValue::from_str("zh-CN,zh;q=0.9,en-US;q=0.8,en;q=0.7").unwrap());
    headers.insert(
        "Cache-Control", 
        reqwest::header::HeaderValue::from_str("no-cache").unwrap());
    headers.insert(
        "Cookie",
        reqwest::header::HeaderValue::from_str("_ga=GA1.1.1497295525.1703463875; _ga_T0HJJCQMVV=GS1.1.1729439048.12.0.1729439050.0.0.0; __na=N; WEBSITE_COUNTRY_TYPE=US; __cf__bkm=KCjxuMn6khlJVtYaihbfTdtz9seZL9FKV/Tm3UY/5q6gp/NMnih0fj02sYGG8ZzR; _va=13; top_banner_counter_1500=6").unwrap());
    let response = cli
        .get("https://www.colamanga.com/manga-fb45571/")
        .headers(headers)
        .send()
        .unwrap()
        .text()
        .unwrap();
    println!("{}", response);
}
