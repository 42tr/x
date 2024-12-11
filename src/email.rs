use lettre::smtp::authentication::Credentials;
use lettre::{SmtpClient, Transport};
use lettre_email::Email;

/// 发送邮件
///
/// receiver: 收件人邮箱
/// subject: 邮件主题
/// content: 邮件内容
pub fn send(receiver: String, subject: String, content: String) {
    // get AUTHORIZE CODE from env param
    let authorize_code =
        std::env::var("EMAIL_AUTHORIZE_CODE").expect("Cannot get env EMAIL_AUTHORIZE_CODE");
    // 发件人邮箱
    let mine_email = "1055894396@qq.com";
    // 服务器host
    let smtp_server = "smtp.qq.com";

    let email = Email::builder()
        .to(receiver)
        .from(mine_email)
        .subject(subject)
        .text(content)
        .build()
        .unwrap();

    let creds = Credentials::new(mine_email.to_string(), authorize_code);

    let mut mailer = SmtpClient::new_simple(smtp_server)
        .unwrap()
        .credentials(creds)
        .transport();

    let result = mailer.send(email.into());
    if result.is_ok() {
        println!("Email sent");
    } else {
        println!("Could not send email: {:?}", result);
    }
    print!("{:?}", result);
    mailer.close();
}
