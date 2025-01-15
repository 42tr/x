use tokio_cron_scheduler::{Job, JobScheduler};

mod email;
mod gold;
mod stock;
mod news;
mod leetcode;
mod comic;
mod weather;
mod saying;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    send_email().await.unwrap();
    let sched = JobScheduler::new().await?;

    sched.add(
        Job::new_async("0 0 0 * * *", |_uuid, mut _l| {
            Box::pin(async move {
                // 打印当前时间
                println!("Start schedule !!! Current time: {:?}", std::time::SystemTime::now());
                send_email().await.unwrap();
            })
        })?
    ).await?;
    sched.start().await?;
    
    println!("Main thread start !!!");
    let authorize_code =
        std::env::var("EMAIL_AUTHORIZE_CODE").expect("Cannot get env EMAIL_AUTHORIZE_CODE");
    println!("Authorize code: {}", authorize_code);
    // 使主线程保持阻塞直到用户手动终止（例如按 Ctrl+C）
    tokio::signal::ctrl_c().await?;
    Ok(())

}

async fn send_email() -> anyhow::Result<()> {
    let (lc_name, lc_href) = leetcode::daily_question().await.expect("get leetcode question failed");
    let (news_times, news_links, news_titles) = news::get().await.expect("get news failed");
    let gold_line_png = gold::create_line().await.expect("get gold line failed");
    let stock_line_png = stock::create_line().await.expect("get stock line failed");
    let comics = comic::get_latest_chapters().await.expect("get comics failed");
    let weathers = weather::get().await.expect("get weather failed");

    let content = format!(r#"
        <html>
            <body>{}
                {}
                <h2>LeetCode</h2>
                <ul>
                    <li>
                        <a href="{lc_href}">{lc_name}</a>
                    </li>
                </ul>
                <h2>黄金价格</h2>
                <img src="cid:{gold_line_png}" />
                <h2>上证指数</h2>
                <img src="cid:{stock_line_png}" />
                {}
                <h2>新闻</h2>
                {}
            </body>
        </html>
    "#,
    saying::get().await.expect("get saying error"),
    concat_weather(weathers),
    concat_comic(comics),
    concat_news(news_times, news_links, news_titles));
    
    email::send_with_file(
        "1055894396@qq.com".to_string(),
        "每日速递".to_string(),
        content.to_string(),
        vec![gold_line_png.clone(), stock_line_png.clone()]).unwrap();
    std::fs::remove_file(gold_line_png).unwrap();
    std::fs::remove_file(stock_line_png).unwrap();
    Ok(())
}

fn concat_news(news_times: Vec<String>, news_links: Vec<String>, news_titles: Vec<String>) -> String {
    let mut news_content = String::new();
    news_content += "<ul>";
    for (i, news_time) in news_times.iter().enumerate() {
        news_content += &format!(r#"
            <li>
                {} <a href="{}">{}</a>
            </li>
        "#, news_time, news_links[i], news_titles[i]);
    }
    news_content + "</ul>"
}

fn concat_comic(comics: Vec<(String, String)>) -> String {
    if comics.is_empty() {
        return "".to_owned();
    }
    "<h2>漫画</h2><ul>".to_owned() + &comics.into_iter().map(|(name, chapter_name)| format!(r#"<li>{} {}</li>"#, name, chapter_name)).collect::<Vec<_>>().join("") + "</ul>"
}

fn concat_weather(weathers: Vec<String>) -> String {
    if weathers.is_empty() {
        return "".to_owned();
    }
    "<h2>天气</h2><ul>".to_owned() + &weathers.into_iter().map(|weather| format!(r#"<li>{}</li>"#, weather)).collect::<Vec<_>>().join("") + "</ul>"
}