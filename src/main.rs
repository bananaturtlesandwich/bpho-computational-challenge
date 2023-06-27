#![windows_subsystem = "windows"]

#[cfg(not(target_family = "wasm"))]
fn main() {
    eframe::run_native(
        "bpho computational challenge",
        eframe::NativeOptions {
            initial_window_size: Some(eframe::egui::vec2(800.0, 600.0)),
            run_and_return: false,
            ..Default::default()
        },
        Box::new(|ctx| Box::new(bpho_computational_challenge::App::new(ctx))),
    )
    .unwrap();
}

#[cfg(target_family = "wasm")]
fn main() {
    wasm_bindgen_futures::spawn_local(async {
        eframe::WebRunner::new()
            .start(
                "canvas", // hardcode it
                eframe::WebOptions::default(),
                Box::new(|ctx| Box::new(bpho_computational_challenge::App::new(ctx))),
            )
            .await
            .unwrap()
    })
}
