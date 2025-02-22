use charts_rs::{svg_to_png, BarChart, Box, SeriesCategory, THEME_ANT};
use chrono::{Local, TimeZone};
use chrono_tz::Tz;

pub async fn send_message(content: &str) -> anyhow::Result<()> {
    let url = format!(
        "http://106.15.62.248:9901/tXfsoXKwoUD2U9gSoRrJbY/{}",
        content
    );
    reqwest::get(url).await?;
    Ok(())
}

/// 获取当前时间
pub fn currenttime() -> String {
    let timestamp = Local::now().timestamp();
    timestamp2time(timestamp, "%y-%m-%d %H:%M")
}

/// 时间戳转时间 %y-%m-%d %H:%M
pub fn timestamp2time(timestamp: i64, format: &str) -> String {
    let shanghai: Tz = "Asia/Shanghai".parse().unwrap();
    shanghai
        .timestamp_opt(timestamp, 0)
        .unwrap()
        .format(format)
        .to_string()
}

pub fn create_line_img(
    title: &str,
    series_name: &str,
    file_name: &str,
    keys: Vec<String>,
    values: Vec<f32>,
) -> anyhow::Result<()> {
    let axis_min = values.iter().copied().reduce(f32::min).unwrap_or(0.0);
    let mut bar_chart =
        BarChart::new_with_theme(vec![(series_name, values).into()], keys, THEME_ANT);
    bar_chart.y_axis_configs[0].axis_min = Some((axis_min / 100f32).floor() * 100f32);

    bar_chart.width = 1000.0;
    bar_chart.title_text = title.to_string();
    bar_chart.legend_margin = Some(Box {
        top: bar_chart.title_height,
        bottom: 5.0,
        ..Default::default()
    });
    bar_chart.series_list[0].category = Some(SeriesCategory::Line);
    bar_chart.series_list[0].y_axis_index = 1;
    bar_chart.series_list[0].label_show = false;
    let png = svg_to_png(&bar_chart.svg()?)?;
    std::fs::write(file_name, png).unwrap();
    Ok(())
}
