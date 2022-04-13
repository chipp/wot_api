use hyper::body::Buf;
use hyper::{Body, Request, Response, StatusCode};
use log::{error, info};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

pub type ErasedError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, ErasedError>;

#[derive(Debug, Serialize)]
struct ListItem<'j> {
    id: u16,
    title: &'j str,
    count: Option<&'j str>,
    completed: bool,
}

pub async fn service(request: Request<Body>) -> Result<Response<Body>> {
    let path = request.uri().path().to_string();
    let mut segments = path.split("/");
    _ = segments.next();

    if let Some("v1") = segments.next() {
        router(request, segments).await
    } else {
        unknown_request(request).await
    }
}

async fn router(
    request: Request<Body>,
    mut segments: std::str::Split<'_, &str>,
) -> Result<Response<Body>> {
    match segments.next() {
        Some("list") => list(request).await,
        Some("add") => add(request).await,
        Some("remove") => remove(request, segments).await,
        _ => unknown_request(request).await,
    }
}

async fn unknown_request(request: Request<Body>) -> Result<Response<Body>> {
    error!("Unsupported request: {:?}", request);

    let body = hyper::body::aggregate(request).await?;

    match std::str::from_utf8(body.chunk()) {
        Ok(body) if !body.is_empty() => error!("Body {}", body),
        _ => (),
    }

    let response = Response::builder()
        .status(StatusCode::BAD_REQUEST)
        .body(Body::from("invalid request"))?;

    Ok(response)
}

pub async fn list(_request: Request<Body>) -> Result<Response<Body>> {
    let items = vec![
        ListItem {
            id: 1,
            title: "Яйца",
            count: None,
            completed: false,
        },
        ListItem {
            id: 2,
            title: "Помидоры",
            count: Some("2 кг"),
            completed: true,
        },
    ];

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_vec(&items)?))?)
}

pub async fn add(request: Request<Body>) -> Result<Response<Body>> {
    #[derive(Deserialize)]
    struct AddRequest<'j> {
        title: &'j str,
        count: Option<&'j str>,
    }

    let body = hyper::body::aggregate(request).await?;

    let request_body: AddRequest = serde_json::from_slice(body.chunk())?;
    let response = ListItem {
        id: 1,
        title: request_body.title,
        count: request_body.count,
        completed: false,
    };

    Ok(Response::builder()
        .status(StatusCode::CREATED)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_vec(&response)?))?)
}

pub async fn remove(
    _request: Request<Body>,
    mut segments: std::str::Split<'_, &str>,
) -> Result<Response<Body>> {
    if let Some(id) = segments.next() {
        if let Ok(id) = u16::from_str(id) {
            info!("{:?}", id);
            Ok(Response::builder()
                .status(StatusCode::NO_CONTENT)
                .body(Body::empty())?)
        } else {
            Ok(Response::builder()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::from("invalid request"))?)
        }
    } else {
        Ok(Response::builder()
            .status(StatusCode::CREATED)
            .body(Body::empty())?)
    }
}
