use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use log::info;
use std::str::FromStr;
use wot_api::{service, ErasedError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let make_svc = make_service_fn(move |_| async move {
        Ok::<_, ErasedError>(service_fn(move |req| async move { service(req).await }))
    });

    let port = if let Ok(port) = std::env::var("PORT") {
        u16::from_str(&port).unwrap()
    } else {
        8080
    };

    let addr = ([0, 0, 0, 0], port).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening http://{}", addr);
    server.await?;

    Ok(())
}
