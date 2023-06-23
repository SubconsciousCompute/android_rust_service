#[allow(non_snake_case)]
pub mod android {
    use std::{net::SocketAddr, thread::spawn};

    use android_logger::Config;
    use axum::{routing::get, Router, Server};
    use jni::{objects::JClass, JNIEnv};

    #[no_mangle]
    pub extern "system" fn Java_com_example_rustapp_RustService_startService<
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

        spawn(launch);
    }

    // TODO: Find better way to spawn axum server
    #[tokio::main]
    async fn launch() {
        Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
            .serve(
                Router::new()
                    .route("/", get(|| async { "Hello from Shepherd!" }))
                    .into_make_service(),
            )
            .await
            .unwrap();
    }
}
