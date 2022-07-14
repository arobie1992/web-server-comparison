use axum::{
    Json, 
    response::{IntoResponse, Response}, 
    http::StatusCode,
    Router, 
    routing::post,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use serde_json::json;

pub fn app() -> Router {
    Router::new()
        .route("/call", post(call_url))
}

async fn call_url(Json(http_req): Json<HttpReq>) -> Result<Json<Info>, ErrorResp> {
    match make_get_req(&http_req.url).await {
        Ok(info) => Ok(Json(info.into())),
        Err(error) => {
            Err(ErrorResp{message: error.to_string()}.into())
        }
    }
}

async fn make_get_req(url: &'_ str) -> Result<Info, Box<dyn Error>> {
    let resp = reqwest::get(url).await?.text().await?;
    let info: Info = serde_json::from_str(&resp)?;
    Ok(info)
}

#[derive(Serialize, Deserialize, Debug)]
struct ErrorResp {
    message: String
}

impl IntoResponse for ErrorResp {
    fn into_response(self) -> Response {
        let body = Json(json!({
            "error": self.message
        }));
        (StatusCode::INTERNAL_SERVER_ERROR, body).into_response()
    }
}

#[derive(Deserialize)]
struct HttpReq {
    url: String
}

#[derive(Serialize, Deserialize, Debug)]
struct Info {
	id: u8,
    status: String,
	data: Data
}

#[derive(Serialize, Deserialize, Debug)]
struct Data {
	requestors: Vec<String>
}