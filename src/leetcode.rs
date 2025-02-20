use anyhow::Ok;
use serde::Deserialize;

pub async fn daily_question() -> anyhow::Result<(String, String)> {
    let url = "https://leetcode.cn/graphql/";
    let body = r#"{"query":"\n    query CalendarTaskSchedule($days: Int!) {\n  calendarTaskSchedule(days: $days) {\n    contests {\n      id\n      name\n      slug\n      progress\n      link\n      premiumOnly\n    }\n    dailyQuestions {\n      id\n      name\n      slug\n      progress\n      link\n      premiumOnly\n    }\n    studyPlans {\n      id\n      name\n      slug\n      progress\n      link\n      premiumOnly\n    }\n  }\n}\n    ","variables":{"days":0},"operationName":"CalendarTaskSchedule"}"#;
    let rsp: Rsp = reqwest::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .body(body)
        .send()
        .await?
        .json()
        .await?;
    let question = &rsp.data.calendar_task_schedule.daily_questions[0];
    Ok((question.name.clone(), question.link.clone()))
}
#[derive(Deserialize)]
struct Rsp {
    data: Data,
}
#[derive(Deserialize)]
struct Data {
    #[serde(rename = "calendarTaskSchedule")]
    calendar_task_schedule: CalendarTaskSchedule,
}
#[derive(Deserialize)]
struct CalendarTaskSchedule {
    #[serde(rename = "dailyQuestions")]
    daily_questions: Vec<Question>,
}

#[derive(Deserialize)]
struct Question {
    name: String,
    link: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;

    #[test]
    fn test_dayly_question() {
        let (name, link) = tokio_test::block_on(daily_question()).unwrap();
        info!("name: {}, link: {}", name, link);
    }
}
