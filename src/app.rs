use iota_streams::app::transport::tangle::client::Client;

#[path = "iota.rs"] mod iota;

pub struct TemplateApp {
    client: Client,
    health_data: Vec<String>,
    ann_link: String,
    sub_link: String,
    keyload_link: String,
    author_seed: String,
    sub_seed: String,
    response: String,
}

impl Default for TemplateApp {
    fn default() -> Self {
        Self {
            client: Client::new_from_url("https://api.lb-0.h.chrysalis-devnet.iota.cafe/"),
            health_data: vec![],
            ann_link: "Dummy Announcement Link".to_owned(),
            sub_link: "Dummy Sub Link".to_owned(),
            keyload_link: "Dummy Key Link".to_owned(),
            author_seed: "Enter Author Seed".to_owned(),
            sub_seed: "Enter Sub Seed".to_owned(),
            response: "Waiting".to_owned(),
        }
    }
}

impl TemplateApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Default::default()
    }
}

impl eframe::App for TemplateApp {

    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let Self { client, health_data, ann_link, sub_link, keyload_link, author_seed, sub_seed, response } = self;

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
            ui.label("");
            ui.label("Enter your author seed and click to create new IOTA Streams channel");
            ui.label("");
            ui.label("Author Seed:");
            ui.text_edit_singleline(author_seed);
            if ui.button("Create Channel").clicked() {
                *ann_link = iota::create_channel(author_seed.to_owned(), client.to_owned()).unwrap_or_default();
            }
            ui.label("");
            ui.label("Announcement Link:");
            ui.text_edit_singleline(ann_link);
            ui.label("");
            ui.label("");
            ui.label("Enter subscription link to process subscriber");
            ui.label("");
            ui.label("Subscription Link:");
            ui.text_edit_singleline(sub_link);
            if ui.button("Process Subscription").clicked() {
                *keyload_link = iota::process_subscription(author_seed.to_owned(), ann_link.to_owned(), sub_link.to_owned(), client.to_owned()).unwrap_or_default();
            }
            ui.label("");
            ui.label("Keyload Link:");
            ui.text_edit_singleline(keyload_link);
            ui.label("");
            ui.label("");
            if ui.button("Send Health Data").clicked() {
                *response = iota::send_health_data(author_seed.to_owned(), keyload_link.to_owned(), client.to_owned()).unwrap_or_default();
            }
            ui.text_edit_singleline(response);
        });

        egui::SidePanel::right("right_side_panel").show(ctx, |ui| {
            ui.heading("Subscriber Panel");
            ui.label("");
            ui.label("Enter your seed and the announcement link, then click to subscribe to the IOTA streams channel");
            ui.label("");
            ui.label("Subscriber Seed:");
            ui.text_edit_singleline(sub_seed);
            ui.label("Announcement Link:");
            ui.text_edit_singleline(ann_link);
            if ui.button("Subscribe To Channel").clicked() {
                *sub_link = iota::subscribe_to_channel(sub_seed.to_owned(), ann_link.to_owned(), client.to_owned()).unwrap_or_default();
            }
            ui.label("");
            ui.label("Subscription Link:");
            ui.text_edit_singleline(sub_link);
            ui.label("");
            ui.label("");
            if ui.button("Fetch and Process Health Data").clicked() {
                *health_data = iota::fetch_health_data(sub_seed.to_owned(), client.to_owned()).unwrap_or_default(); 
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Health Data Stream");
            ui.label("");
            ui.label("  Private data received by Subscriber from Author");
            ui.label("");
            for msg in health_data {
                ui.label(msg.to_string());
                ui.label("");
            }
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
