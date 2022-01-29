use rocket::{
    request::{self, FromRequest},
    response::Redirect,
    Request,
};

pub mod mark_read;
pub mod update;

pub struct GetHeaders {
    referer: Option<String>,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for GetHeaders {
    type Error = String;

    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        let referer = req.headers().get("Referer").next().map(|s| s.to_owned());

        request::Outcome::Success(GetHeaders { referer })
    }
}

fn redirect_back(headers: GetHeaders) -> Redirect {
    match headers.referer {
        Some(url) => Redirect::to(url),
        None => Redirect::to("/"),
    }
}
