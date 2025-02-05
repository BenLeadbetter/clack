use clack_host::bundle::PluginBundle;
use clack_host::factory::PluginFactory;

#[test]
#[cfg_attr(miri, ignore)] // Miri does not support calling foreign function (dlopen)
pub fn it_works() {
    let bundle_path = format!(
        "{}/../target/debug/{}gain{}",
        env!("CARGO_MANIFEST_DIR"),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );
    let bundle = PluginBundle::load(bundle_path).unwrap();

    let desc = bundle
        .get_factory::<PluginFactory>()
        .unwrap()
        .plugin_descriptor(0)
        .unwrap();
    assert_eq!(desc.id().unwrap().to_bytes(), b"org.rust-audio.clack.gain");
}

#[test]
#[cfg_attr(miri, ignore)] // Miri does not support calling foreign function (dlopen)
pub fn it_works_concurrently() {
    let bundle_path = format!(
        "{}/../target/debug/{}gain{}",
        env!("CARGO_MANIFEST_DIR"),
        std::env::consts::DLL_PREFIX,
        std::env::consts::DLL_SUFFIX
    );

    std::thread::scope(|s| {
        for _ in 0..300 {
            s.spawn(|| {
                let bundle = PluginBundle::load(&bundle_path).unwrap();

                let desc = bundle
                    .get_factory::<PluginFactory>()
                    .unwrap()
                    .plugin_descriptor(0)
                    .unwrap();
                assert_eq!(desc.id().unwrap().to_bytes(), b"org.rust-audio.clack.gain");
            });
        }
    })
}
