use egui::RichText;

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

// ##################### THIS IS THE STATE OF THE APPLICATION ###################################################
pub struct App {
    pub label: String,
    pub screen: Screen,
    pub value: f32,
    pub lines: Vec<Product>,
    pub login_screen: LoginScreen,
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
            Screen::Admin => show_default_layout(self, ctx),
            Screen::Login => show_login_layout(self, ctx),
            Screen::Purchase => show_default_layout(self, ctx),
            Screen::Transfer => show_default_layout(self, ctx),
            Screen::Sales => show_default_layout(self, ctx),
        };

    }
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

pub fn show_login_layout(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

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

    // if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
    //     if app.login_screen.password == "admin" && app.login_screen.username == "admin" {
    //         app.login_screen = LoginScreen::default();
    //         app.screen = Screen::Admin;
    //     } else {
    //         app.login_screen.error = "Wrong username or password".to_owned();
    //     } 
        
    // }
}




pub fn show_default_layout(mut app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, &mut app);
    
    components::default_central_panel(app, ctx);
    
}