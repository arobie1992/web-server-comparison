#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::Route;
use rocket::http::Status;
use rocket::response::{self, Response, Responder};
use std::error::Error;
use rocket::request::Request;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct HttpReq {
    url: String
}

#[derive(Deserialize, Serialize, Debug)]
struct Info {
	id: u8,
    status: String,
	data: Data
}

#[derive(Deserialize, Serialize, Debug)]
struct Data {
	requestors: Vec<String>
}

#[derive(Debug, Clone, PartialEq)]
pub struct InternalServerError;

impl<'r> Responder<'r, 'static> for InternalServerError {
    fn respond_to(self, _: &'r Request<'_>) -> response::Result<'static> {
        Response::build().status(Status::InternalServerError).ok()
    }
}

#[post("/call", data="<req>")]
async fn call(req: Json<HttpReq>) -> Result<Json<Info>, InternalServerError> {
    match make_get_req(&req.url).await {
        Ok(info) => {
            Result::Ok(Json(info))
        },
        Err(err) => {
            println!("Got error: {}", err);
            Err(InternalServerError{})
        },
    }
}

async fn make_get_req(url: &'_ str) -> Result<Info, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let info: Info = serde_json::from_str(&resp)?;
    Ok(info)
}

pub fn routes() -> Vec<Route> {
    routes![call]
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ErrorResp {
    status: u32,
    message: &'static str
}
