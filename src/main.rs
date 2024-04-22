use whimsy::prelude::run;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
// pub mod run_ui;
// pub mod state;


#[tokio::main]
async fn main() {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "whimsy=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init().is_ok() {}; 
    tracing::info!("Subscriber initialized.");

    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let window = winit::window::WindowBuilder::new()
        .with_title("Whimsy")
        .build(&event_loop)
        .unwrap();

    run(window, event_loop).await;
}
