use scx_stats::prelude::*;
use scx_stats::serde_json::json;

#[test]
fn env_base_path_overrides_default() {
    let base = std::env::temp_dir().join(format!("scx_stats_env_{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);

    // SAFETY: this integration test binary runs this test on its own.
    unsafe {
        std::env::set_var("SCX_STATS_BASE_PATH", &base);
    }

    let data =
        StatsServerData::<(), ()>::new().add_stats("top", Box::new(|_, _| Ok(json!({"ok": true}))));
    let server = StatsServer::<(), ()>::new(data).launch().unwrap();

    let socket = base.join("root/stats");
    assert!(
        socket.exists(),
        "socket not created at expected path: {socket:?}"
    );

    let mut client = StatsClient::new().connect(None).unwrap();
    let resp = client
        .request::<scx_stats::serde_json::Value>("stats", vec![])
        .unwrap();
    assert_eq!(resp, json!({"ok": true}));

    drop(server);
    let _ = std::fs::remove_dir_all(&base);
}
