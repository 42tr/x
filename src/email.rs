use lettre::message::header::{self, Header};
use lettre::message::{MultiPart, SinglePart};
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};
use log::{error, info};

// /// 发送邮件
// ///
// /// receiver: 收件人邮箱
// /// subject: 邮件主题
// /// content: 邮件内容
// pub fn send(receiver: String, subject: String, content: String) -> anyhow::Result<()> {
//     // get AUTHORIZE CODE from env param
//     let authorize_code =
//         std::env::var("EMAIL_AUTHORIZE_CODE").expect("Cannot get env EMAIL_AUTHORIZE_CODE");
//     // 发件人邮箱
//     let mine_email = "1055894396@qq.com";
//     // 服务器host
//     let smtp_server = "smtp.qq.com";

//     let email = Message::builder()
//         .from(mine_email.parse()?)
//         .to(receiver.parse()?)
//         .subject(subject)
//         .header(lettre::message::header::ContentType::TEXT_HTML)
//         .body(content)?;

//     let creds = Credentials::new(mine_email.to_string(), authorize_code);

//     let mailer = SmtpTransport::relay(smtp_server)?.credentials(creds).build();
//     match mailer.send(&email) {
//         Ok(_) => println!("邮件发送成功!"),
//         Err(e) => panic!("发送邮件失败: {e:?}"),
//     }
//     Ok(())
// }

/// 发送邮件
///
/// receiver: 收件人邮箱
/// subject: 邮件主题
/// content: 邮件内容
pub fn send_with_file(
    receiver: String,
    subject: String,
    content: String,
    files: Vec<&str>,
) -> anyhow::Result<()> {
    // get AUTHORIZE CODE from env param
    let authorize_code =
        std::env::var("EMAIL_AUTHORIZE_CODE").expect("Cannot get env EMAIL_AUTHORIZE_CODE");
    // 发件人邮箱
    let mine_email = "1055894396@qq.com";
    // 服务器host
    let smtp_server = "smtp.qq.com";

    let mut multipart = MultiPart::mixed().singlepart(
        SinglePart::builder()
            .header(header::ContentType::TEXT_HTML)
            .body(content.to_string()),
    );
    files.iter().for_each(|file| {
        multipart = multipart.clone().singlepart(
            SinglePart::builder()
                .header(header::ContentType::parse("image/jpeg").expect("解析图片格式失败:"))
                .header(header::ContentDisposition::inline_with_name(file))
                // 注意demo.jpg用尖括号包裹起来了
                .header(
                    header::ContentId::parse(&format!("<{file}>")).expect("解析成content-id失败:"),
                )
                .body(std::fs::read(file).expect("读取文件失败:")),
        );
    });
    let email = Message::builder()
        .from(mine_email.parse()?)
        .to(receiver.parse()?)
        .subject(subject)
        .multipart(multipart)?;

    let creds = Credentials::new(mine_email.to_string(), authorize_code);

    let mailer = SmtpTransport::relay(smtp_server)?
        .credentials(creds)
        .build();
    match mailer.send(&email) {
        Ok(_) => info!("邮件发送成功!"),
        Err(e) => error!("发送邮件失败: {e:?}"),
    }
    Ok(())
}
