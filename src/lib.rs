mod service;

#[allow(non_snake_case)]
pub mod android {
    use std::thread::spawn;

    use android_logger::{self, init_once};
    use jni::{
        objects::{JClass, JString},
        JNIEnv,
    };

    use crate::service::{launch, watch};

    #[no_mangle]
    pub extern "system" fn Java_com_example_rustapp_RustService_startService(
        _: JNIEnv,
        _: JClass,
        _: JString,
    ) {
        init_once(
            android_logger::Config::default()
                .with_max_level(log::LevelFilter::Trace)
                .with_tag("RustApp"),
        );

        spawn(|| {
            watch(
                "/data/user/0/com.example.rustapp/files/fsmon_log.yaml",
                vec!["/storage/emulated/0/Documents"],
            );
        });

        spawn(|| {
            launch("/data/user/0/com.example.rustapp/files/fsmon_log.yaml");
        });
    }
}
