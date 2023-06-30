use std::{
    fs::{read_to_string, OpenOptions},
    io::Write,
    net::SocketAddr,
    path::Path,
    sync::mpsc::channel,
    // thread::sleep,
    // time::Duration,
};

use axum::{routing::get, Router, Server};
use chrono::Local;
use log::{error, trace};
use notify::{recommended_watcher, RecursiveMode, Watcher};
use serde_json::to_string;

pub fn watch<P>(fsmon_log: P, watch_paths: Vec<P>)
where
    P: AsRef<Path>,
{
    let (tx, rx) = channel();

    let mut watcher =
        match recommended_watcher(move |res| tx.send(res).unwrap()) {
            Ok(w) => w,
            Err(err) => {
                error!("Watcher Creation Error: {err}");
                return;
            }
        };

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
}

// fn watch_alter<P>(fsmon_log: P, watch_paths: Vec<P>)
// where
//     P: AsRef<Path>,
// {
//     let mut fs_events = OpenOptions::new()
//         .create(true)
//         .append(true)
//         .open(fsmon_log)
//         .unwrap();

//     let watcher = recommended_watcher(
//         move |res: Result<notify::Event, notify::Error>| match res {
//             Ok(event) => {
//                 if let Ok(event) = to_string(&event) {
//                     trace!("Event: {event}");
//                     _ = fs_events.write(
//                         format!("{} {event}\n", Local::now().to_rfc3339())
//                             .as_bytes(),
//                     );
//                 } else {
//                     error!("Error Deserializing Event!");
//                 }
//             }
//             Err(err) => error!("Watch Error: {err}"),
//         },
//     );

//     let mut watcher = match watcher {
//         Ok(w) => w,
//         Err(err) => {
//             error!("Watcher Creation Error: {err}");
//             return;
//         }
//     };

//     for wpath in watch_paths {
//         if let Err(err) =
//             watcher.watch(wpath.as_ref(), RecursiveMode::Recursive)
//         {
//             error!("Error Watching {} : {err}", wpath.as_ref().display());
//         }
//     }

//     loop {
//         sleep(Duration::from_secs(3600));
//     }
// }

#[tokio::main]
pub async fn launch(fsmon_log: impl AsRef<Path> + Clone + Send + 'static) {
    Server::bind(&SocketAddr::from(([0, 0, 0, 0], 3000)))
        .serve(
            Router::new()
                .route("/", get(|| async { "Hello from RustApp!" }))
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
