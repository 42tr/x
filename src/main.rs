use tokio_cron_scheduler::{Job, JobScheduler};

mod email;
mod gold;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let sched = JobScheduler::new().await?;

    sched.add(
        Job::new_async("0 0 4 * * *", |_uuid, mut _l| {
            Box::pin(async move {
                println!("Start schedule !!!");
                send_email();
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

fn send_email() {
    let content = r#"
        <html>
            <body>
                <h2>黄金价格</h2>
                <img src="cid:gold_line.png" />
            </body>
        </html>
    "#;

    gold::create_line().unwrap();
    email::send_with_file(
        "1055894396@qq.com".to_string(),
        "新闻速递".to_string(),
        content.to_string(),
        vec!["gold_line.png".to_string()]).unwrap();
    std::fs::remove_file("gold_line.png").unwrap();
}