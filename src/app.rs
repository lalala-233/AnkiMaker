#[derive(serde::Deserialize, serde::Serialize, Default)]
pub struct App {}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.
        /*     let name = "NotoSans-Regular".to_string();
               let mut font = egui::FontDefinitions::default();
               font.font_data.insert(
                   name.clone(),
                   egui::FontData::from_static(include_bytes!("../NotoSansCJK-Regular.ttc")),
               );
               font.families
                   .get_mut(&egui::FontFamily::Proportional)
                   .unwrap()
                   .insert(0, name.clone());
               font.families
                   .get_mut(&egui::FontFamily::Monospace)
                   .unwrap()
                   .push(name);
               cc.egui_ctx.set_fonts(font);
        */
        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}
use eframe::egui;
impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let Self {} = self;
        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        //#[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close)
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Anki Maker，好用的软件");
            ui.hyperlink("https://github.com/emilk/eframe_template");
            ui.add(egui::github_link_file!(
                "https://github.com/emilk/eframe_template/blob/master/",
                "Source code."
            ));
        });
    }
}

pub fn app(app_name: &str) -> eframe::Result<()> {
    let app_options = eframe::NativeOptions::default();
    eframe::run_native(
        app_name,
        app_options,
        Box::new(|cc| Ok(Box::new(App::new(cc)))),
    )
}
