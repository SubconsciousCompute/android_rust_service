use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    net::SocketAddr,
    path::Path,
    sync::mpsc::channel,
    // thread::sleep,
};

use axum::{routing::get, Router, Server};
use chrono::Local;
use log::{error, trace};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use serde_json::to_string;

pub fn watch<P>(fsmon_log: P, watch_paths: Vec<P>) -> notify::Result<()>
where
    P: AsRef<Path>,
{
    let (tx, rx) = channel();

    let mut watcher = recommended_watcher(move |res| tx.send(res).unwrap())?;

    for wpath in watch_paths {
        if let Err(err) =
            watcher.watch(wpath.as_ref(), RecursiveMode::Recursive)
        {
            error!("Error Watching {} : {err}", wpath.as_ref().display());
        }
    }

    let mut fs_events = OpenOptions::new()
        .create(true)
        .append(true)
        .open(fsmon_log)
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
pub async fn launch(fsmon_log: impl AsRef<Path> + Clone + Send + 'static) {
    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(
            Router::new()
                .route("/", get(|| async { "Hello from Shepherd!" }))
                .route(
                    "/fs",
                    get(|| async {
                        read_to_string(fsmon_log).unwrap_or("Empty".to_string())
                    }),
                )
                .into_make_service(),
        )
        .await
        .unwrap();
}
