use std::time::Duration;

use anyhow::Result;
use aws_config::{environment::EnvironmentVariableCredentialsProvider, BehaviorVersion, Region};
use aws_sdk_s3::{config::SharedCredentialsProvider, presigning::PresigningConfig};
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    routing::get,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;
use web_shot::web_shot::Captureshot;

const BUCKET_NAME: &str = "web-shot-1255746465";

#[tokio::main]
async fn main() {
    let app_state = AppState::new();
    let app = Router::new().route("/", get(hello)).with_state(app_state);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();

    axum::serve(listener, app).await.unwrap();
}

#[derive(Clone)]
struct AppState {
    s3: aws_sdk_s3::Client,
}

impl AppState {
    fn new() -> Self {
        Self {
            s3: get_s3_client(),
        }
    }
}

#[derive(Serialize)]
struct Resp<T> {
    code: i8,
    message: String,
    data: Option<T>,
}

#[derive(Deserialize, Validate)]
struct Req {
    #[validate(url)]
    url: String,
    #[validate(range(min = 0))]
    width: Option<u32>,
    #[validate(range(min = 0))]
    height: Option<u32>,
    #[validate(range(min = 0, max = 100))]
    quality: Option<u32>,
    full_page: Option<bool>,
}

async fn hello(params: Query<Req>, State(state): State<AppState>) -> impl IntoResponse {
    let params = params.0;
    let web_shot = Captureshot::new(
        params.url.clone(),
        params.width.unwrap_or(1280),
        params.height.unwrap_or(1080),
        params.quality.unwrap_or(75),
        params.full_page.unwrap_or(false),
    );
    let s3_url = match shot_and_get_s3url(web_shot, state.s3).await {
        Ok(url) => url,
        Err(_) => return resp_err("err".to_string()),
    };

    Json(Resp {
        code: 0,
        message: "ok".to_string(),
        data: Some(s3_url),
    })
}

fn resp_err(message: String) -> Json<Resp<String>> {
    Json(Resp {
        code: -1,
        message,
        data: None,
    })
}

async fn shot_and_get_s3url(web_shot: Captureshot, s3: aws_sdk_s3::Client) -> Result<String> {
    let id = Uuid::new_v4().to_string();
    let filename = format!("{}.png", id);
    let image_bytes = web_shot.shot().await?.get_bytes().await?;
    s3.put_object()
        .bucket(BUCKET_NAME)
        .key(filename.clone())
        .body(image_bytes.into())
        .send()
        .await?;

    let expires_in = Duration::from_secs(10 * 60);
    let object_url = s3
        .get_object()
        .bucket(BUCKET_NAME)
        .key(filename)
        .presigned(PresigningConfig::expires_in(expires_in).unwrap())
        .await?;
    Ok(object_url.uri().to_string())
}

fn get_s3_client() -> aws_sdk_s3::Client {
    let region = Region::new("ap-guangzhou");
    let s3_config = aws_config::SdkConfig::builder()
        .behavior_version(BehaviorVersion::latest())
        .endpoint_url("https://cos.ap-guangzhou.myqcloud.com")
        .credentials_provider(SharedCredentialsProvider::new(
            EnvironmentVariableCredentialsProvider::default(),
        ))
        .region(region)
        .build();

    aws_sdk_s3::Client::new(&s3_config)
}
