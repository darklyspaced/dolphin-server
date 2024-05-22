use askama::Template;

use crate::error::Result;

#[derive(Template)]
#[template(path = "login.html")]
struct LoginPage;

pub async fn login() -> Result<String> {
    let page = LoginPage;
    Ok(page.render()?)
}
