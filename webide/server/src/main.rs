use app::*;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use server::shutdown_signal;
use std::time::Duration;
use tokio::time::timeout;

#[tokio::main]
async fn main() {
    let conf = get_configuration(None).unwrap();
    let addr = conf.leptos_options.site_addr;
    let leptos_options = conf.leptos_options;
    let routes = generate_route_list(App);

    let app = Router::new()
        .route("/healthz", axum::routing::get(|| async { "ok" }))
        .leptos_routes(&leptos_options, routes, {
            let leptos_options = leptos_options.clone();
            move || shell(leptos_options.clone())
        })
        .fallback(leptos_axum::file_and_error_handler(shell))
        .with_state(leptos_options);

    log!("listening on http://{}", &addr);
    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    
    let server = axum::serve(listener, app.into_make_service())
        .with_graceful_shutdown(shutdown_signal());

    // 设置 30 秒的关闭时间
    if let Err(e) = timeout(Duration::from_secs(30), server).await {
        eprintln!("Graceful shutdown timed out: {:?}", e);
    }
}
