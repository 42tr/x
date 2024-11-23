mod email;

fn main() {
    email::send(
        "1055894396@qq.com".to_string(),
        "email from rust".to_string(),
        "This is a test email from lettre".to_string(),
    );
}
