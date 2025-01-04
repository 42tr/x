pub async fn get() -> anyhow::Result<String> {
    let url = "https://open.iciba.com/dsapi/";
    let rsp: Rsp = reqwest::Client::new().get(url).send().await?.json().await?;


    Ok(rsp.content.clone() + "<br>" + &rsp.note.clone())
}

#[derive(Debug, serde::Deserialize)]
struct Rsp {
    content: String,
    note: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get() {
        let content = tokio_test::block_on(get()).unwrap();
        println!("{:?}", content);
    }
}