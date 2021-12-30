use server2::{configuration::Configuration, App};

#[tokio::main]
async fn main() {
    // error   you need        to do something
    // warn    you might need  to do something
    // info    you need        to log this in production
    // debug   you might need  to log this in production
    // trace   everything that is happening (no performance concerns)
    if std::env::var_os("RUST_LOG").is_none() {
        std::env::set_var("RUST_LOG", "server2=debug,tower_http=debug");
    }

    tracing_subscriber::fmt::init();

    let config = Configuration {
        host: "0.0.0.0".to_string(),
        port: 8080,
    };

    App::new(config).run().await
}
