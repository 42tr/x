use anyhow::Ok;
use tokio::process::Command;
use std::{collections::HashMap, io::Write};

/// 获取最新章节
/// 
/// output:
/// Vec<(String, String, String)>: 漫画名称、最新章节名称、最新章节链接
/// 关注漫画 manga-pk25498 斗破苍穹、manga-fb45571 一人之下、manga-wq98740 妖神记、manga-pv587814 海贼王
pub async fn get_latest_chapters() -> anyhow::Result<Vec<(String, String, String)>> {
    let comics = vec![
        ("manga-pk25498", "斗破苍穹"),
        ("manga-fb45571", "一人之下"),
        ("manga-wq98740", "妖神记"),
        ("manga-pv587814", "海贼王"),
    ];

    let mut latest_chapter_list: Vec<(String, String, String)> = Vec::new();

    let latest_chapter_before = read_lastest_chapter()?;
    let mut latest_chapter_next: Vec<(String, String)> = vec![];
    let chapter_before: HashMap<String, String> = latest_chapter_before.into_iter().collect();
    
    for (id, name) in comics {
        let chapter_list= get_chapter_list(id).await?;
        let latest_chapter = &chapter_list.first().unwrap().0;
        println!("{} {}", name, latest_chapter);
        if chapter_before.contains_key(name) && !chapter_before.get(name).unwrap().eq(latest_chapter) {
            for (chapter_name, link) in &chapter_list {
                if chapter_name == chapter_before.get(name).unwrap() {
                    break;
                }
                latest_chapter_list.push((name.to_string(), chapter_name.clone(), link.clone()));
            }
        }
        latest_chapter_next.push((name.to_string(), latest_chapter.to_string()));
    }
    write_lastest_chapter(latest_chapter_next)?;
    Ok(latest_chapter_list)
}

const FILE_NAME: &str = "latest_chapter.txt";

fn write_lastest_chapter(data: Vec<(String, String)>) -> anyhow::Result<()> {
    let mut file = std::fs::File::create(FILE_NAME)?;

    // 将数据写入文件
    for (key, value) in &data {
        writeln!(file, "{} {}", key, value)?;
    }
    Ok(())
}

fn read_lastest_chapter() -> anyhow::Result<Vec<(String, String)>> {
    if !std::path::Path::new(FILE_NAME).exists() {
        return Ok(Vec::new());
    }
    let file = std::fs::File::open(FILE_NAME)?;
    let reader = std::io::BufReader::new(file);
    let mut result: Vec<(String, String)> = Vec::new();
    for line in std::io::BufRead::lines(reader) {
        let line = line?;
        if let Some((first, second)) = line.split_once(' ') {
            result.push((first.to_string(), second.to_string()));
        }
    }
    Ok(result)
}

/// 获取章节列表
/// 
/// input:
/// @id: 漫画的 id，如 manga-pk25498 斗破苍穹
/// output:
/// Vec<(String, String)>: 章节名称、章节链接
/// 
/// 直接使用 http 请求会bei拦截，需要使用 curl 命令，原理不详
async fn get_chapter_list(id: &str) -> anyhow::Result<Vec<(String, String)>> {
    let output = Command::new("curl").arg(format!("https://www.colamanga.com/{id}/")).output().await?;
    let mut resp = "".to_string();
    // 检查命令是否成功执行
    if output.status.success() {
        // 将输出从字节转换为字符串
        resp = String::from_utf8_lossy(&output.stdout).to_string();
    } else {
        // 处理错误信息
        let stderr = String::from_utf8_lossy(&output.stderr);
        eprintln!("Command failed with error: {}", stderr);
    }
    let mut chapter_list: Vec<(String, String)> = Vec::new();
    let document = scraper::Html::parse_document(&resp);
    let selector = scraper::Selector::parse(r#"a[class="fed-btns-info fed-rims-info fed-part-eone"]"#).unwrap();
    for element in document.select(&selector) {
        // assert_eq!("a", element.value().name());
        chapter_list.push((element.text().collect::<String>().trim().to_string(), element.attr("href").unwrap().to_string()));
    }
    Ok(chapter_list)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_chapter_list() {
        let id = "manga-pk25498";
        let chapters =  tokio_test::block_on(get_chapter_list(id)).unwrap();
        println!("{:?}", chapters);
    }
}