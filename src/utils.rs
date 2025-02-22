use chrono::TimeZone;
use chrono_tz::Tz;

pub async fn send_message(content: &str) -> anyhow::Result<()> {
    let url = format!(
        "http://106.15.62.248:9901/tXfsoXKwoUD2U9gSoRrJbY/{}",
        content
    );
    reqwest::get(url).await?;
    Ok(())
}

pub fn timestamp2time(timestamp: i64) -> anyhow::Result<String> {
    let shanghai: Tz = "Asia/Shanghai".parse()?;
    let time = shanghai
        .timestamp_opt(timestamp, 0)
        .single()
        .ok_or_else(|| anyhow::anyhow!("Invalid timestamp: {}", timestamp))?
        .format("%m-%d %H:%M")
        .to_string();
    Ok(time)
}
