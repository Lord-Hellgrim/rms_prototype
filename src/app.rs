use std::time::Duration;

use egui::RichText;
use poll_promise::Promise;
use EZDB;

use crate::components;


#[derive(Debug)]
pub enum Screen {
    Admin,
    Login,
    Purchase,
    Sales,
    Transfer,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub location: String,
    pub price: String,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginScreen {
    pub username: String,
    pub password: String,
    pub error: String,
    pub username_focus: bool,
}

impl Default for LoginScreen {
    fn default() -> Self {
        LoginScreen {
            username: "".to_owned(),
            password: "".to_owned(),
            error: "".to_owned(),
            username_focus: true,
        }
    }
}

#[derive(Default)]
pub struct AdminScreen {
    pub text: String,
    promise: Option<poll_promise::Promise<String>>,
}

// ##################### THIS IS THE STATE OF THE APPLICATION ###################################################
pub struct App {
    pub label: String,
    pub screen: Screen,
    pub value: f32,
    pub lines: Vec<Product>,
    pub login_screen: LoginScreen,
    pub admin_screen: AdminScreen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            screen: Screen::Login,
            value: 2.7,
            lines: Vec::new(),
            login_screen: LoginScreen::default(),
            admin_screen: AdminScreen::default(),
        }
    }
}

// ##############################################################################################################

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {

    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        match self.screen {
            Screen::Admin => show_admin_screen(self, ctx),
            Screen::Login => show_login_screen(self, ctx),
            Screen::Purchase => show_default_screen(self, ctx),
            Screen::Transfer => show_default_screen(self, ctx),
            Screen::Sales => show_default_screen(self, ctx),
        };

    }
}

// ################# LOGIN SCREEN #############################################################################

pub fn show_login_screen(app: &mut App, ctx: &egui::Context) {
    // components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.vertical(|ui| {
            ui.label("Username");
            if app.login_screen.username_focus {
                ui.text_edit_singleline(&mut app.login_screen.username).request_focus();
                app.login_screen.username_focus = false;
            } else {
                ui.text_edit_singleline(&mut app.login_screen.username);
            }
            ui.label("Password");
            ui.text_edit_singleline(&mut app.login_screen.password);
            if ui.button("Login").clicked() {
                if app.login_screen.password == "admin" && app.login_screen.username == "admin" {
                    app.login_screen = LoginScreen::default();
                    app.screen = Screen::Admin;
                } else {
                    app.login_screen.error = "Wrong username or password".to_owned();
                }           
            }
            ui.label(RichText::new(app.login_screen.error.clone()).color(egui::Color32::RED));
        });

    });

    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        if app.login_screen.password == "admin" && app.login_screen.username == "admin" {
            app.login_screen = LoginScreen::default();
            app.screen = Screen::Admin;
        } else {
            app.login_screen.error = "Wrong username or password".to_owned();
        } 
        
    }
}

// ########################### END OF LOGIN SCREEN ##################################################

pub fn show_admin_screen(mut app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

    
    egui::CentralPanel::default().show(ctx, |ui| {
        let ctx_clone = ctx.clone();
        if ui.button("Test poll_promise").clicked() {
            let promise = Promise::spawn_thread("test", move || {
                let good_csv = EZDB::client_networking::download_table("127.0.0.1:3004", "admin", "admin", "good_csv");
                ctx_clone.request_repaint(); // wake up UI thread
                match good_csv {
                    Ok(csv) => csv,
                    Err(e) => format!("Could not retreive data because: {e}"),
                }
            });
            app.admin_screen.promise = Some(promise);
        }

        if let Some(promise) = &app.admin_screen.promise {
            if let Some(text) = promise.ready() {
                app.admin_screen.text = text.clone();
            }
        }

        ui.heading("ADMINISTRATION");
        
        ui.horizontal(|ui| {
            let t = app.admin_screen.text.clone();
            ui.label(t.to_string());
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




pub fn show_default_screen(mut app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, &mut app);
    
    components::default_central_panel(app, ctx);
    
}
pub fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
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