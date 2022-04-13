use hyper::body::Buf;
use hyper::{Body, Method, Request, Response, StatusCode};
use log::error;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use std::str::FromStr;

use crate::db_helper::DbHelper;

mod db_helper;

pub type ErasedError = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, ErasedError>;

#[derive(Debug, Serialize)]
struct ListItem {
    id: u16,
    title: String,
    count: Option<String>,
    completed: bool,
}

impl From<db_helper::Item> for ListItem {
    fn from(item: db_helper::Item) -> Self {
        ListItem {
            id: item.id,
            title: item.title,
            count: item.count,
            completed: item.completed,
        }
    }
}

pub async fn service(request: Request<Body>) -> Result<Response<Body>> {
    let path = request.uri().path().to_string();
    let mut segments = path.split("/");
    _ = segments.next();

    if let Some("v1") = segments.next() {
        Ok(router(request, segments).await.unwrap())
    } else {
        unknown_request(request).await
    }
}

async fn router(
    request: Request<Body>,
    mut segments: std::str::Split<'_, &str>,
) -> Result<Response<Body>> {
    match (segments.next(), request.method()) {
        (Some("items"), &Method::GET) => list(request).await,
        (Some("items"), &Method::POST) => add(request).await,
        (Some("items"), &Method::DELETE) => remove(request, segments).await,
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
    let db_helper = db_helper::DbHelper::new();

    let items = db_helper
        .get_all_items()
        .into_iter()
        .map(Into::into)
        .collect::<Vec<ListItem>>();

    Ok(Response::builder()
        .status(StatusCode::OK)
        .header("Content-Type", "application/json; charset=utf-8")
        .body(Body::from(serde_json::to_vec(&items)?))?)
}

pub async fn add(request: Request<Body>) -> Result<Response<Body>> {
    #[derive(Deserialize)]
    struct AddRequest<'j> {
        title: Cow<'j, str>,
        count: Option<&'j str>,
    }

    let body = hyper::body::aggregate(request).await?;

    let request_body: AddRequest = serde_json::from_slice(body.chunk())?;

    let db_helper = DbHelper::new();
    let response: ListItem = db_helper
        .add_new_item(&request_body.title, request_body.count)
        .into();

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
            let db_helper = DbHelper::new();
            db_helper.remove_item(id);

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
