

#[derive(Debug)]
pub enum Screen {
    Admin,
    Login,
    Purchase,
    Sales,
    Transfer,
}


/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct App {
    // Example stuff:
    label: String,

    #[serde(skip)]
    screen: Screen,

    #[serde(skip)] // This how you opt-out of serialization of a field
    value: f32,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            screen: Screen::Login,
            value: 2.7,
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        match self.screen {
            Screen::Admin => show_default_layout(self, ctx, _frame),
            Screen::Login => show_default_layout(self, ctx, _frame),
            Screen::Purchase => show_default_layout(self, ctx, _frame),
            Screen::Transfer => show_default_layout(self, ctx, _frame),
            Screen::Sales => show_default_layout(self, ctx, _frame),
        };

        dbg!(&self.screen);
        
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}

#[inline]
pub fn default_top_bar(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        
        egui::menu::bar(ui, |ui| {
            // NOTE: no File->Quit on web pages!
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Admin").clicked() {
                        app.screen = Screen::Admin;
                    }
                    if ui.button("Login").clicked() {
                        app.screen = Screen::Login;
                    }
                    if ui.button("Purchase").clicked() {
                        app.screen = Screen::Purchase;
                    }
                    if ui.button("Sales").clicked() {
                        app.screen = Screen::Sales;
                    }
                    if ui.button("Transfer").clicked() {
                        app.screen = Screen::Transfer;
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.add_space(16.0);
            }

            egui::widgets::global_dark_light_mode_buttons(ui);
        });
    });

}


pub fn show_default_layout(mut app: &mut App, ctx: &egui::Context, _frame: &mut eframe::Frame) {
    default_top_bar(ctx, &mut app);
    
    egui::CentralPanel::default().show(ctx, |ui| {
        // The central panel the region left after adding TopPanel's and SidePanel's
        ui.heading("rms_prototype");
        
        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut app.label);
        });
        
        ui.add(egui::Slider::new(&mut app.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            app.value += 1.0;
        }

        ui.separator();
        
        ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
            powered_by_egui_and_eframe(ui);
            egui::warn_if_debug_build(ui);
        });
    });
}