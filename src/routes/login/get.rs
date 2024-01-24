use actix_web::cookie::Cookie;
use actix_web::http::header::ContentType;
use actix_web::{HttpRequest, HttpResponse};

pub async fn login_form(request: HttpRequest) -> HttpResponse {
    let error_html = match request.cookie("_flash") {
        None => "".into(),
        Some(cookie) => {
            format!("<p><i>{}</i></p>", cookie.value())
        }
    };

    let mut response = HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"<!DOCTYPE html>
<html lang="en">
    <head>
        <!-- Equivalent to an HTTP header -->
        <meta http-equiv="content-type" content="text/html"; charset=utf-8">
        <title>Home</title>
    </head>
    <body>
        {error_html}
        <form action="/login" method="post">
            <label>
                Username
                <input 
                    type="text"
                    placeholder="Enter username"
                    name="username"
                >
            </label>

            <label>
                Password
                <input 
                    type="password"
                    placeholder="Enter password"
                    name="password"
                >
            </label>

            <button type="submit">Login</button>
        </form>
    </body>
</html>"#
        ));

    response
        // Sets the cookies `Max-Age` to be `0` for us
        .add_removal_cookie(&Cookie::new("_flash", ""))
        .unwrap();
    response
}
