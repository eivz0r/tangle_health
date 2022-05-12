
#[path = "iota.rs"] mod iota;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    author_seed: String,
    author_ann_address: String,
    sub_ann_address: String,
    sub_seed: String,
    response: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            // Example stuff:
            author_seed: "Enter Author Seed".to_owned(),
            author_ann_address: "Dummy Author Address".to_owned(),
            sub_ann_address: "Dummy Sub Address".to_owned(),
            sub_seed: "Enter Sub Seed".to_owned(),
            response: "Waiting".to_owned(),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { author_seed, author_ann_address, sub_ann_address, sub_seed, response } = self;

        // Examples of how to create different panels and windows.
        // Pick whichever suits you.
        // Tip: a good default choice is to just keep the `CentralPanel`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });
            });
        });

        egui::SidePanel::left("left_side_panel").show(ctx, |ui| {
            ui.heading("Author Panel");
            ui.label("Enter your author seed and click to create new iota streams channel");
            ui.text_edit_singleline(author_seed);
            if ui.button("Create Channel").clicked() {
                *author_ann_address = iota::create_channel(author_seed.to_string()).unwrap_or_default();
            }
            ui.text_edit_singleline(author_ann_address);
        });

        egui::SidePanel::right("right_side_panel").show(ctx, |ui| {
            ui.heading("Subscriber Panel");
            ui.label("Enter the announcement link and your seed, then click to subscribe to the IOTA streams channel");
            ui.text_edit_singleline(sub_seed);
            ui.text_edit_singleline(sub_ann_address);
            if ui.button("Subscribe To Channel").clicked() {
                *response = iota::subscribe_to_channel(sub_seed.to_string(), sub_ann_address.to_string()).unwrap_or_default();
            }
            ui.text_edit_singleline(response);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Health Data Stream");
            egui::warn_if_debug_build(ui);
        });

        if false {
            egui::Window::new("Window").show(ctx, |ui| {
                ui.label("Windows can be moved by dragging them.");
                ui.label("They are automatically sized based on contents.");
                ui.label("You can turn on resizing and scrolling if you like.");
                ui.label("You would normally chose either panels OR windows.");
            });
        }
    }
}
