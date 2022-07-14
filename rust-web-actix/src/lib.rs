use serde::{Deserialize, Serialize};
use std::{
    error::Error, 
    fmt::{Display, Result as FmtResult, Formatter}
};
use actix_web::{web::Json, http::StatusCode, HttpResponse, ResponseError};
use serde_json::{json, to_string_pretty};

pub async fn call_url(Json(http_req): Json<HttpReq>) -> Result<HttpResponse, ErrorResp> {
    match make_get_req(http_req).await {
        Ok(resp) => Ok(HttpResponse::Ok().json(resp)),
        Err(error) => Err(ErrorResp{message: error.to_string()})
    }
}

async fn make_get_req(http_req: HttpReq) -> Result<Info, Box<dyn Error>> {
    let resp = reqwest::get(http_req.url).await?.text().await?;
    let info: Info = serde_json::from_str(&resp)?;
    Ok(info)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ErrorResp {
    message: String
}

impl Display for ErrorResp {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{}", to_string_pretty(self).unwrap())
    }
}

impl ResponseError for ErrorResp {
    // builds the actual response to send back when an error occurs
    fn error_response(&self) -> HttpResponse {
        let err_json = json!({ "error": self.message });
        HttpResponse::build(StatusCode::INTERNAL_SERVER_ERROR).json(err_json)
    }
}

#[derive(Deserialize)]
pub struct HttpReq {
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Info {
	id: u8,
    status: String,
	data: Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
	requestors: Vec<String>
}