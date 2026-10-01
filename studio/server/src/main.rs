use app::*;
use axum::Router;
use leptos::logging::log;
use leptos::prelude::*;
use leptos_axum::{LeptosRoutes, generate_route_list};
use server::shutdown_signal;
use std::time::Duration;

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

    let server = axum::serve(listener, app.into_make_service()).with_graceful_shutdown(async {
        shutdown_signal().await;
        // 收到退出信号后才开始计时：30 秒内未完成优雅关闭则强制退出
        tokio::spawn(async {
            tokio::time::sleep(Duration::from_secs(30)).await;
            eprintln!("Graceful shutdown timed out");
            std::process::exit(1);
        });
    });

    if let Err(e) = server.await {
        eprintln!("Server error: {e}");
    }
}
