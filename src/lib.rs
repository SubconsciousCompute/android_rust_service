#[allow(non_snake_case)]
pub mod android {
    use std::net::SocketAddr;

    use android_logger::Config;
    use axum::{routing::get, Router, Server};
    use jni::{objects::JClass, JNIEnv};

    #[no_mangle]
    pub extern "system" fn Java_com_example_rustapp_RustService_launch<
        'local,
    >(
        _: JNIEnv<'local>,
        _: JClass<'local>,
    ) {
        android_logger::init_once(
            Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("RustApp"),
        );

        launch();
    }

    #[tokio::main]
    async fn launch() {
        let app = Router::new().route("/", get(index));

        Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
            .serve(app.into_make_service())
            .await
            .unwrap();
    }

    async fn index() -> &'static str {
        "Hello, World!"
    }
}