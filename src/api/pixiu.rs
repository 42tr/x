use sqlx::MySqlPool;

#[derive(sqlx::FromRow, Debug, serde::Deserialize, serde::Serialize)]
pub struct FundInfo {
    id: Option<u32>,
    amount: f32,
    name: String,
    class: String,
    timestamp: i64,
    source: String,
}

#[derive(sqlx::FromRow, Debug, serde::Deserialize, serde::Serialize)]
pub struct DebtInfo {
    id: Option<u32>,
    name: String,
    amount: f32,
    repayment: f32,
    last_timestamp: i64,
}

#[derive(sqlx::FromRow, Debug, serde::Deserialize, serde::Serialize)]
pub struct PropertyInfo {
    id: Option<u32>,
    name: String,
    amount: f32,
}

#[derive(sqlx::FromRow, Debug, serde::Deserialize, serde::Serialize)]
pub struct SumInfo {
    name: String,
    value: f32,
}

pub async fn init(pool: &MySqlPool) -> anyhow::Result<()> {
    let sql = "CREATE TABLE IF NOT EXISTS pixiu_fund_info (
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        amount FLOAT NOT NULL,
        class VARCHAR(255) NOT NULL,
        timestamp BIGINT NOT NULL,
        source VARCHAR(255) NOT NULL
    )";
    sqlx::query(sql).execute(pool).await?;
    let sql = "CREATE TABLE IF NOT EXISTS pixiu_debt_info (
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        amount FLOAT NOT NULL,
        repayment FLOAT NOT NULL,
        last_timestamp BIGINT NOT NULL
    )";
    sqlx::query(sql).execute(pool).await?;
    let sql = "CREATE TABLE IF NOT EXISTS pixiu_property_info (
        id INT UNSIGNED NOT NULL AUTO_INCREMENT PRIMARY KEY,
        name VARCHAR(255) NOT NULL,
        amount FLOAT NOT NULL
    )";
    sqlx::query(sql).execute(pool).await?;
    Ok(())
}

pub async fn insert_fund_info(pool: &MySqlPool, info: FundInfo) -> anyhow::Result<()> {
    let sql =
        "INSERT INTO pixiu_fund_info (amount, name, class, timestamp, source) VALUES (?, ?, ?, ?, ?)";
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
    from: i64,
    to: i64,
    page: u32,
    size: u32,
    source: Option<String>,
    fund_type: Option<String>,
) -> anyhow::Result<Vec<FundInfo>> {
    let offset = (page - 1) * size;
    let mut sql = format!(
        "SELECT * FROM pixiu_fund_info WHERE timestamp BETWEEN {} AND {}",
        from, to
    );
    if let Some(source) = source {
        if !source.is_empty() {
            sql.push_str(&format!(" AND source = '{}'", source));
        }
    }
    if let Some(fund_type) = fund_type {
        if !fund_type.is_empty() {
            let types: Vec<String> = fund_type.split(',').map(|s| format!("'{}'", s)).collect();
            sql.push_str(&format!(" AND class IN ({})", types.join(",")));
        }
    }
    sql.push_str(&format!(
        " order by timestamp desc, id limit {} offset {}",
        size, offset
    ));

    let rows = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_sum_info(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    source: Option<String>,
    fund_type: Option<String>,
) -> anyhow::Result<Vec<SumInfo>> {
    let mut sql = format!(
        "select class as name, sum(ceil(-amount)) as value
        from pixiu_fund_info
        where timestamp BETWEEN {} AND {}",
        from, to
    );
    if let Some(source) = source {
        if !source.is_empty() {
            sql.push_str(&format!(" AND source = '{}'", source));
        }
    }
    if let Some(fund_type) = fund_type {
        if !fund_type.is_empty() {
            let types: Vec<String> = fund_type.split(',').map(|s| format!("'{}'", s)).collect();
            sql.push_str(&format!(" AND class IN ({})", types.join(",")));
        }
    }
    sql.push_str(" group by class having value > 0");
    let rows = sqlx::query_as(&sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_income_info(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    source: Option<String>,
    fund_type: Option<String>,
) -> anyhow::Result<f32> {
    let mut sql = format!(
        "SELECT ROUND(IFNULL(SUM(amount), 0), 2)
        FROM pixiu_fund_info
        WHERE timestamp BETWEEN {} AND {}
        AND amount > 0",
        from, to
    );
    if let Some(source) = source {
        if !source.is_empty() {
            sql.push_str(&format!(" AND source = '{}'", source));
        }
    }
    if let Some(fund_type) = fund_type {
        if !fund_type.is_empty() {
            let types: Vec<String> = fund_type.split(',').map(|s| format!("'{}'", s)).collect();
            sql.push_str(&format!(" AND class IN ({})", types.join(",")));
        }
    }
    let result: Option<f32> = sqlx::query_scalar(&sql).fetch_optional(pool).await?;
    Ok(result.unwrap_or(0.0))
}

pub async fn get_expense_info(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    source: Option<String>,
    fund_type: Option<String>,
) -> anyhow::Result<f32> {
    let mut sql = format!(
        "SELECT ROUND(IFNULL(SUM(amount), 0), 2)
        FROM pixiu_fund_info
        WHERE timestamp BETWEEN {} AND {}
        AND amount < 0",
        from, to
    );
    if let Some(source) = source {
        if !source.is_empty() {
            sql.push_str(&format!(" AND source = '{}'", source));
        }
    }
    if let Some(fund_type) = fund_type {
        if !fund_type.is_empty() {
            let types: Vec<String> = fund_type.split(',').map(|s| format!("'{}'", s)).collect();
            sql.push_str(&format!(" AND class IN ({})", types.join(",")));
        }
    }
    let result: Option<f32> = sqlx::query_scalar(&sql).fetch_optional(pool).await?;
    Ok(result.unwrap_or(0.0))
}

pub async fn count(
    pool: &MySqlPool,
    from: i64,
    to: i64,
    source: Option<String>,
    fund_type: Option<String>,
) -> anyhow::Result<i32> {
    let mut sql = format!(
        "SELECT COUNT(*) FROM pixiu_fund_info WHERE timestamp BETWEEN {} AND {}",
        from, to
    );
    if let Some(source) = source {
        if !source.is_empty() {
            sql.push_str(&format!(" AND source = '{}'", source));
        }
    }
    if let Some(fund_type) = fund_type {
        if !fund_type.is_empty() {
            let types: Vec<String> = fund_type.split(',').map(|s| format!("'{}'", s)).collect();
            sql.push_str(&format!(" AND class IN ({})", types.join(",")));
        }
    }
    let count: i32 = sqlx::query_scalar(&sql).fetch_one(pool).await?;
    Ok(count)
}

pub async fn get_debt_info(pool: &MySqlPool) -> anyhow::Result<Vec<DebtInfo>> {
    let sql = "SELECT * from pixiu_debt_info";
    let rows = sqlx::query_as(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_property_info(pool: &MySqlPool) -> anyhow::Result<Vec<PropertyInfo>> {
    let sql = "SELECT
        ppi.id,
        ppi.name,
        (ppi.amount + COALESCE(SUM(pfi.amount), 0)) AS amount
    FROM
        pixiu_property_info ppi
    LEFT JOIN
        pixiu_fund_info pfi
        ON pfi.source = ppi.name
    GROUP BY
        ppi.id, ppi.name, ppi.amount";
    let rows = sqlx::query_as(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_fund_sources(pool: &MySqlPool) -> anyhow::Result<Vec<String>> {
    let sql = "SELECT DISTINCT source FROM pixiu_fund_info";
    let rows = sqlx::query_scalar(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn get_fund_types(pool: &MySqlPool) -> anyhow::Result<Vec<String>> {
    let sql = "SELECT DISTINCT class FROM pixiu_fund_info";
    let rows = sqlx::query_scalar(sql).fetch_all(pool).await?;
    Ok(rows)
}

pub async fn delete_fund_info(pool: &MySqlPool, id: u32) -> anyhow::Result<()> {
    let sql = "DELETE FROM pixiu_fund_info WHERE id = ?";
    sqlx::query(sql).bind(id).execute(pool).await?;
    Ok(())
}

pub async fn update_fund_info(pool: &MySqlPool, id: u32, info: FundInfo) -> anyhow::Result<()> {
    let sql = "UPDATE pixiu_fund_info SET amount = ?, name = ?, class = ?, timestamp = ?, source = ? WHERE id = ?";
    sqlx::query(sql)
        .bind(info.amount)
        .bind(info.name)
        .bind(info.class)
        .bind(info.timestamp)
        .bind(info.source)
        .bind(id)
        .execute(pool)
        .await?;
    Ok(())
}
