use app::App;

pub mod app;

pub fn app(app_name: &str) -> eframe::Result<()> {
    let app_options = eframe::NativeOptions::default();
    eframe::run_native(app_name, app_options, Box::new(|cc| Box::new(App::new(cc))))
}
