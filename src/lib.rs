#[allow(non_snake_case)]
pub mod android {
    use std::{
        fs::{read_to_string, OpenOptions},
        io::Write,
        net::SocketAddr,
        path::Path,
        sync::mpsc::channel,
        thread::spawn,
        // thread::{sleep, spawn},
        // time::Duration,
    };

    use android_logger::{self, init_once};
    use axum::{routing::get, Router, Server};
    use chrono::Local;
    use jni::{
        objects::{JClass, JString},
        JNIEnv,
    };
    use log::{error, trace};
    use notify::{recommended_watcher, RecursiveMode, Watcher};
    use serde_json::to_string;

    #[no_mangle]
    pub extern "system" fn Java_com_example_rustapp_RustService_startService<
        'local,
    >(
        _: JNIEnv<'local>,
        _: JClass<'local>,
        _: JString<'local>,
    ) {
        init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("RustApp"),
        );

        _ = std::fs::remove_file(Path::new(FILES_DIR).join("fs_events.yaml"));

        spawn(watch);
        spawn(launch);
    }

    const FILES_DIR: &str = "/data/user/0/com.example.rustapp/files";
    const WATCH_DIR: &str = "/storage/emulated/0/Documents";

    fn watch() -> notify::Result<()> {
        let (tx, rx) = channel();

        let mut watcher =
            recommended_watcher(move |res| tx.send(res).unwrap())?;

        if let Err(err) =
            watcher.watch(Path::new(WATCH_DIR), RecursiveMode::Recursive)
        {
            error!("Error Watching {WATCH_DIR} : {err}");
        }

        let mut fs_events = OpenOptions::new()
            .create(true)
            .append(true)
            .open(Path::new(FILES_DIR).join("fs_events.yaml"))
            .unwrap();

        while let Ok(res) = rx.recv() {
            match res {
                Ok(event) => {
                    if let Ok(event) = to_string(&event) {
                        trace!("Event: {event}");
                        _ = fs_events.write(
                            format!("{} {event}\n", Local::now().to_rfc3339())
                                .as_bytes(),
                        );
                    } else {
                        error!("Error Deserializing Event!");
                    }
                }
                Err(err) => error!("Watch Error: {err}"),
            }
        }

        Ok(())
    }

    // fn watch_alter() -> notify::Result<()> {
    //     let mut fs_events = OpenOptions::new()
    //         .create(true)
    //         .append(true)
    //         .open(Path::new(FILES_DIR).join("fs_events.yaml"))
    //         .unwrap();

    //     let mut watcher = recommended_watcher(
    //         move |res: Result<notify::Event, notify::Error>| match res {
    //             Ok(event) => {
    //                 if let Ok(event) = to_string(&event) {
    //                     trace!("Event: {event}");
    //                     _ = fs_events.write(
    //                         format!("{}: {event}", Local::now().to_rfc3339())
    //                             .as_bytes(),
    //                     );
    //                 } else {
    //                     error!("Error Deserializing Event!");
    //                 }
    //             }
    //             Err(err) => error!("Watch Error: {err}"),
    //         },
    //     )?;

    //     watcher.watch(Path::new(FILES_DIR), RecursiveMode::Recursive)?;

    //     loop {
    //         sleep(Duration::from_secs(3600));
    //     }
    // }

    // TODO: Find better way to spawn axum server
    #[tokio::main]
    async fn launch() {
        Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
            .serve(
                Router::new()
                    .route("/", get(|| async { "Hello from Shepherd!" }))
                    .route(
                        "/fs",
                        get(|| async {
                            read_to_string(
                                Path::new(FILES_DIR).join("fs_events.yaml"),
                            )
                            .unwrap_or("Empty".to_string())
                        }),
                    )
                    .into_make_service(),
            )
            .await
            .unwrap();
    }
}
