use log::info;
use sqlx::mysql::MySqlPool;
use tokio_cron_scheduler::{Job, JobScheduler};

mod comic;
mod email;
mod gold;
mod leetcode;
mod news;
mod saying;
mod stock;
mod utils;
mod weather;

const GOLD_INFO_IMG_NAME: &str = "gold_line.png";
const STOCK_INFO_IMG_NAME: &str = "sse_line.png";

/// 检查运行环境（环境变量）
///
/// EMAIL_AUTHORIZE_CODE 用于发送邮件
/// DATABASE_URL 用于连接数据库
fn check_runtime_environment() {
    dotenv::dotenv().ok(); // 从 .env 加载环境变量，开发环境用
    std::env::var("EMAIL_AUTHORIZE_CODE").expect("Cannot get env EMAIL_AUTHORIZE_CODE");
    std::env::var("DATABASE_URL").expect("Cannot get env DATABASE_URL");
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap(); // 初始化日志
    check_runtime_environment(); // 检查运行环境

    let database_url = std::env::var("DATABASE_URL").unwrap();
    let pool = MySqlPool::connect(&database_url).await?;
    news::obtain_latest_news(&pool).await?;
    gold::obtain(&pool).await?;
    send_email(&pool).await.unwrap();

    let sched = JobScheduler::new().await?;
    let pool1 = pool.clone();
    let pool2 = pool.clone();
    sched
        .add(Job::new_async("0 * * * * *", move |_uuid, mut _l| {
            let pool = pool1.clone();
            Box::pin(async move {
                info!(
                    "Start Obtain news !!! Current time: {}",
                    utils::currenttime()
                );
                news::obtain_latest_news(&pool).await.unwrap();
            })
        })?)
        .await?;
    sched
        .add(Job::new_async("0 0 0 * * *", move |_uuid, mut _l| {
            let pool = pool2.clone();
            Box::pin(async move {
                info!(
                    "Start schedule !!! Current time: {:?}",
                    std::time::SystemTime::now()
                );
                gold::obtain(&pool).await.expect("obtain gold info failed!");
                send_email(&pool).await.unwrap();
            })
        })?)
        .await?;
    sched.start().await?;

    info!("Main thread start !!!");

    // 使主线程保持阻塞直到用户手动终止（例如按 Ctrl+C）
    tokio::signal::ctrl_c().await?;
    Ok(())
}

async fn send_email(pool: &MySqlPool) -> anyhow::Result<()> {
    let (lc_name, lc_href) = leetcode::daily_question()
        .await
        .expect("get leetcode question failed");
    let (news_times, news_titles) = news::get(pool).await.expect("get news failed");
    let gold = gold::get_info(pool).await?;
    utils::create_line_img("Gold Info", "RMB", GOLD_INFO_IMG_NAME, gold.0, gold.1)?;
    let stock = stock::get_info().await?;
    utils::create_line_img("SSE Index", "Point", STOCK_INFO_IMG_NAME, stock.0, stock.1)?;
    let comics = comic::get_latest_chapters()
        .await
        .expect("get comics failed");
    let weathers = weather::get().await.expect("get weather failed");

    let content = format!(
        r#"
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
                <img src="cid:{GOLD_INFO_IMG_NAME}" />
                <h2>上证指数</h2>
                <img src="cid:{STOCK_INFO_IMG_NAME}" />
                {}
                <h2>新闻</h2>
                {}
            </body>
        </html>
    "#,
        saying::get().await.expect("get saying error"),
        concat_weather(weathers),
        concat_comic(comics),
        concat_news(news_times, news_titles)
    );

    email::send_with_file(
        "1055894396@qq.com".to_string(),
        "每日速递".to_string(),
        content.to_string(),
        vec![GOLD_INFO_IMG_NAME, STOCK_INFO_IMG_NAME],
    )?;
    std::fs::remove_file(GOLD_INFO_IMG_NAME).unwrap();
    std::fs::remove_file(STOCK_INFO_IMG_NAME).unwrap();
    Ok(())
}

fn concat_news(news_times: Vec<String>, news_titles: Vec<String>) -> String {
    let mut news_content = String::new();
    news_content += "<ul>";
    for (i, news_time) in news_times.iter().enumerate() {
        news_content += &format!(
            r#"
            <li>
                {} {}
            </li>
        "#,
            news_time, news_titles[i]
        );
    }
    news_content + "</ul>"
}

fn concat_comic(comics: Vec<(String, String)>) -> String {
    if comics.is_empty() {
        return "".to_owned();
    }
    "<h2>漫画</h2><ul>".to_owned()
        + &comics
            .into_iter()
            .map(|(name, chapter_name)| format!(r#"<li>{} {}</li>"#, name, chapter_name))
            .collect::<Vec<_>>()
            .join("")
        + "</ul>"
}

fn concat_weather(weathers: Vec<String>) -> String {
    if weathers.is_empty() {
        return "".to_owned();
    }
    "<h2>天气</h2><ul>".to_owned()
        + &weathers
            .into_iter()
            .map(|weather| format!(r#"<li>{}</li>"#, weather))
            .collect::<Vec<_>>()
            .join("")
        + "</ul>"
}
