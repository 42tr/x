use tokio_cron_scheduler::{Job, JobScheduler};

mod email;
mod gold;
mod stock;
mod leetcode;

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
    let (lc_name, lc_href) = leetcode::daily_question().await?;
    let content = format!(r#"
        <html>
            <body>
                <h2>LeetCode</h2>
                <ul>
                    <li>
                        <a href="{lc_href}">{lc_name}</a>
                    </li>
                </ul>
                <h2>黄金价格</h2>
                <img src="cid:gold_line.png" />
                <h2>上证指数</h2>
                <img src="cid:sse_line.png" />
            </body>
        </html>
    "#);

    gold::create_line().await?;
    stock::create_line().await?;
    email::send_with_file(
        "1055894396@qq.com".to_string(),
        "每日速递".to_string(),
        content.to_string(),
        vec!["gold_line.png".to_string(), "sse_line.png".to_string()]).unwrap();
    std::fs::remove_file("gold_line.png").unwrap();
    std::fs::remove_file("sse_line.png").unwrap();
    Ok(())
}