use pulldown_cmark::{html, Options, Parser};
use spin_sdk::http::{Request, Response};

pub fn render_markdown(markdown_input: &str) -> String {
    let options = Options::empty();
    let parser = Parser::new_ext(markdown_input, options);

    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    html_output
}

pub async fn handle_markdown_request(req: Request) -> Result<Response, spin_sdk::http::Error> {
    let body = req.body().to_vec().await?;
    let markdown_input = String::from_utf8(body).map_err(|_| spin_sdk::http::Error::BadRequest)?;

    let html_output = render_markdown(&markdown_input);

    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/html")
        .body(Some(html_output.into()))?)
}
