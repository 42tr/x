#[derive(Debug, serde::Deserialize)]
struct FundInfo {
    id: u32,
    amount: f32,
    name: String,
    class: String,
    timestamp: i64,
    source: String,
}

pub async fn init(pool: &MySqlPool) -> anyhow::Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS fund_info (
        id INTEGER UNSIGNED PRIMARY KEY AUTOINCREMENT,
        amount FLOAT NOT NULL,
        name TEXT NOT NULL,
        class TEXT NOT NULL,
        timestamp BIGINT NOT NULL,
        source TEXT NOT NULL
    )";
    sqlx::query(sql).execute(&pool).await?;
    Ok(())
}

pub async fn insert_fund_info(pool: &MySqlPool, info: FundInfo) -> anyhow::Result<()> {
    let sql =
        "INSERT INTO fund_info (amount, name, class, timestamp, source) VALUES (?, ?, ?, ?, ?)";
    sqlx::query(sql)
        .bind(info.amount)
        .bind(info.name)
        .bind(info.class)
        .bind(info.timestamp)
        .bind(info.source)
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn get_fund_info(
    pool: &MySqlPool,
    page: u32,
    per_page: u32,
) -> anyhow::Result<Vec<FundInfo>> {
    let offset = (page - 1) * per_page;
    let sql = format!(
        "SELECT * FROM fund_info order by timestamp desc limit {} offset {}",
        per_page, offset
    );
    let rows = sqlx::query_as(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn count(pool: &MySqlPool) -> anyhow::Result<u32> {
    let sql = "SELECT COUNT(*) FROM fund_info";
    let row = sqlx::query(sql).fetch_one(pool).await?;
    Ok(row.get(0))
}
