use std::fmt::Display;

use crate::{
    app::AppState,
    config_data::Config,
    error::{ConfigError, Result},
};
use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Path, State},
    http::StatusCode,
};

//#[derive(Template)]
//#[template(path = "config.html", print = "code")]
struct ConfigPage<T>
where
    T: Config + Clone,
    <T as IntoIterator>::Item: IntoIterator,
    <<T as IntoIterator>::Item as IntoIterator>::Item: Display,
{
    data: T,
}

pub async fn config(
    Path(panel): Path<String>,
    State(mut state): State<AppState>,
) -> Result<impl IntoResponse> {
    match panel.as_str() {
        "ap" => {
            state.ap.get_latest_data(state.pool.clone()).await;

            return Ok(StatusCode::OK);
        }
        //"trolley" => {}
        _ => Err(ConfigError::InvalidPanel.into()),
    }
}

impl<T> ::askama::Template for ConfigPage<T>
where
    T: Config + Clone,
    <T as IntoIterator>::Item: IntoIterator,
    <<T as IntoIterator>::Item as IntoIterator>::Item: Display,
{
    fn render_into(&self, writer: &mut (impl ::std::fmt::Write + ?Sized)) -> ::askama::Result<()> {
        include_bytes!("/Users/rohan/dev/rust/dolphin/server/templates/config.html");
        include_bytes!("/Users/rohan/dev/rust/dolphin/server/templates/base.html");
        writer.write_str("<!-- templates/base.html -->\n<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n    <script src=\"https://unpkg.com/htmx.org@1.9.6\"\n        integrity=\"sha384-FhXw7b6AlE/jyjlZH5iHa/tTe9EpJ1Y55RjcgPbjeWMskSxZt1v9qkxLJWNJaGni\"\n        crossorigin=\"anonymous\">\n    </script>\n    <title>Dolphin</title>\n    \n</head>\n    <body>\n        <div id=\"content\">\n            \n<table>\n        <thead>\n            ")?;
        {
            let _iter = self.data.clone().into_iter();
            for (row, _loop_item) in ::askama::helpers::TemplateLoop::new(_iter) {
                writer.write_str("\n                ")?;
                if *(&(_loop_item.first) as &bool) {
                    writer.write_str("\n                    <tr>\n                        ")?;
                    {
                        let _iter = (row).into_iter();
                        for (heading, _loop_item) in ::askama::helpers::TemplateLoop::new(_iter) {
                            ::std::write!(
                                writer,
                                "\n                            <th>{expr0}</th>\n                        ",
                                expr0 = &::askama::MarkupDisplay::new_unsafe(&(heading), ::askama::Html),
                            )?;
                        }
                    }
                    writer.write_str("\n                    <tr/>\n                ")?;
                }
                writer.write_str("\n            ")?;
            }
        }
        writer.write_str("\n        </thead>\n        <tbody>\n            ")?;
        {
            let _iter = self.data.clone().into_iter();
            for (row, _loop_item) in ::askama::helpers::TemplateLoop::new(_iter) {
                writer.write_str("\n                ")?;
                if *(&((_loop_item.index + 1) > 1) as &bool) {
                    writer.write_str("\n                    <tr>\n                        ")?;
                    {
                        let _iter = (row).into_iter();
                        for (val, _loop_item) in ::askama::helpers::TemplateLoop::new(_iter) {
                            ::std::write!(
                                writer,
                                "\n                            <td>{expr1}</td>\n                        ",
                                expr1 = &::askama::MarkupDisplay::new_unsafe(&(val), ::askama::Html),
                            )?;
                        }
                    }
                    writer.write_str("\n                    <tr/>\n                ")?;
                }
                writer.write_str("\n            ")?;
            }
        }
        writer.write_str("\n        </tbody>\n    </table>\n<form id=\"login\">\n    <input placeholder=\"username\" required type=text name=\"username\">\n    <input placeholder=\"password\" required type=password name=\"password\">\n    <button hx-post=\"/login\" hx-trigger=\"click\">Login</button>\n</form>")?;
        writer.write_str("\n\n        </div>\n    </body>\n</html>")?;
        ::askama::Result::Ok(())
    }
    const EXTENSION: ::std::option::Option<&'static ::std::primitive::str> = Some("html");
    const SIZE_HINT: ::std::primitive::usize = 908;
    const MIME_TYPE: &'static ::std::primitive::str = "text/html; charset=utf-8";
}
impl<T> ::std::fmt::Display for ConfigPage<T>
where
    T: Config + Clone,
    <T as IntoIterator>::Item: IntoIterator,
    <<T as IntoIterator>::Item as IntoIterator>::Item: Display,
{
    #[inline]
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        ::askama::Template::render_into(self, f).map_err(|_| ::std::fmt::Error {})
    }
}
impl<T> ::askama_axum::IntoResponse for ConfigPage<T>
where
    T: Config + Clone,
    <T as IntoIterator>::Item: IntoIterator,
    <<T as IntoIterator>::Item as IntoIterator>::Item: Display,
{
    #[inline]
    fn into_response(self) -> ::askama_axum::Response {
        ::askama_axum::into_response(&self)
    }
}
