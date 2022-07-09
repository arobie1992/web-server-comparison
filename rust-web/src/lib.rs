#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::{Route, Catcher};
use rocket::http::Status;
use std::error::Error;

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct HttpReq {
    url: String
}

#[derive(Deserialize, Debug)]
struct Info {
	id: u8,
    status: String,
	data: Data
}

#[derive(Deserialize, Debug)]
struct Data {
	requestors: Vec<String>
}

#[post("/call", data="<req>")]
async fn call(req: Json<HttpReq>) -> Status {
    match make_get_req(&req.url).await {
        Ok(info) => {
            println!("Successfully got info: {:?}", info);
            Status::Ok
        },
        Err(err) => {
            println!("Got error: {}", err);
            Status::InternalServerError
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

#[catch(500)]
fn internal_server_error() -> Json<ErrorResp> {
    Json(ErrorResp{
        status: 500,
        message: "Internal server error"
    })
}

pub fn catchers() -> Vec<Catcher> {
    catchers![internal_server_error]
}