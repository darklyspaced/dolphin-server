use askama::Template;

#[derive(Template)]
#[template(path = "index.html")]
struct LoginPage {}

pub async fn login() {}
