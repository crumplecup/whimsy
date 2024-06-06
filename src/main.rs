use polite::Polite;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use whimsy::prelude::App;
// pub mod run_ui;
// pub mod state;

#[tokio::main]
async fn main() -> Polite<()> {
    if tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "whimsy=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .try_init()
        .is_ok()
    {};
    tracing::info!("Subscriber initialized.");

    let (app, event_loop) = App::boot().await?;
    app.run(event_loop).await?;
    Ok(())
}
