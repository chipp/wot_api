use hyper::{
    service::{make_service_fn, service_fn},
    Server,
};
use log::info;
use wot_api::{service, ErasedError, Result};

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::init_timed();

    let make_svc = make_service_fn(move |_| async move {
        Ok::<_, ErasedError>(service_fn(move |req| async move { service(req).await }))
    });

    let addr = ([0, 0, 0, 0], 8080).into();
    let server = Server::bind(&addr).serve(make_svc);

    info!("Listening http://{}", addr);
    server.await?;

    Ok(())
}
