use env_logger::Env;
use ws::listen;

fn main() {
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    if let Err(error) = listen("127.0.0.1:8888", |out| {
        move |msg| {
            log::info!("Server got message '{:?}'. ", msg);

            let buf = rmp_serde::to_vec("SUCCESS").unwrap();

            out.send(buf)
        }
    }) {
        log::error!("Failed to create WebSocket due to {:?}", error);
    }
}
