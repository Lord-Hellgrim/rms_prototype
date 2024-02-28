
use std::fmt::Display;

use egui::RichText;
use poll_promise::Promise;
use EZDB;

use crate::{components::{self, default_top_bar}, utilities::lines_to_csv};


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
    pub id: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub price: String,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: "id".to_owned(),
            name: "name".to_owned(),
            description: "description".to_owned(),
            location: "location".to_owned(),
            price: "price".to_owned(),
        }       
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{};{}", self.id, self.name, self.description, self.location, self.price)
    }
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
    pub table_text: String,
    pub table_title: String,
    pub table_confirmation: String,
    pub promise: Option<poll_promise::Promise<String>>,
}

#[derive(Default)]
pub struct SalesScreen {
    pub skiplist: [u8;5],
}

// ##################### THIS IS THE STATE OF THE APPLICATION ###################################################
pub struct App {
    pub label: String,
    pub screen: Screen,
    pub value: f32,
    pub lines: Vec<Product>,
    pub login_screen: LoginScreen,
    pub admin_screen: AdminScreen,
    pub sales_screen: SalesScreen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            screen: Screen::Admin,
            value: 2.7,
            lines: Vec::new(),
            login_screen: LoginScreen::default(),
            admin_screen: AdminScreen::default(),
            sales_screen: SalesScreen::default(),

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
            Screen::Sales => show_sales_screen(self, ctx),
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


// ########################### ADMIN SCREEN #########################################################

pub fn show_admin_screen(app: &mut App, ctx: &egui::Context) {
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
                app.admin_screen.table_confirmation = text.clone();
            }
        }

        ui.heading("ADMINISTRATION");
        
        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label("add table to database");
                ui.label(&app.admin_screen.table_confirmation);
            });

            ui.horizontal(|ui| {

                if ui.button("upload table").clicked() {
                    let ctx_clone = ctx.clone();
                let table_title = app.admin_screen.table_title.clone();
                println!("{}\n\n{}", app.admin_screen.table_text, app.admin_screen.table_text.len());
                let table_text = app.admin_screen.table_text.clone();
                let promise = Promise::spawn_thread("upload table", move || {
                    let confirmation = EZDB::client_networking::upload_table(
                        "127.0.0.1:3004",
                        "admin",
                        "admin",
                        &table_title,
                        &table_text
                    );
                    ctx_clone.request_repaint();
                    match confirmation {
                        Ok(ok) => format!("Upload successful: {}", ok),
                        Err(e) => format!("Upload failed because: {}", e),
                    }
                });
                app.admin_screen.promise = Some(promise);

                
                }

                if ui.button("update table").clicked() {
                    let ctx_clone = ctx.clone();
                let table_title = app.admin_screen.table_title.clone();
                println!("{}\n\n{}", app.admin_screen.table_text, app.admin_screen.table_text.len());
                let table_text = app.admin_screen.table_text.clone();
                let promise = Promise::spawn_thread("upload table", move || {
                    let confirmation = EZDB::client_networking::update_table(
                        "127.0.0.1:3004",
                        "admin",
                        "admin",
                        &table_title,
                        &table_text
                    );
                    ctx_clone.request_repaint();
                    match confirmation {
                        Ok(ok) => format!("Upload successful: {}", ok),
                        Err(e) => format!("Upload failed because: {}", e),
                    }
                });
                app.admin_screen.promise = Some(promise);

                
                }


            });

            if let Some(promise) = &app.admin_screen.promise {
                if let Some(text) = promise.ready() {
                    app.admin_screen.table_confirmation = text.clone();
                }
            }
            ui.text_edit_singleline(&mut app.admin_screen.table_title);
            ui.text_edit_multiline(&mut app.admin_screen.table_text);
        });
        

        ui.separator();
        
    });
}

// ###################### END OF ADMIN SCREEN ##############################################################

// ####################### SALES SCREEN ####################################################################

pub fn show_sales_screen(app: &mut App, ctx: &egui::Context) {
    default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("SALES SCREEN");

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                if ui.button("add line").clicked() {
                    app.lines.push(Product::default());
                }
                if ui.button("remove line").clicked() {
                    if app.lines.len() != 0 {
                        app.lines.pop();
                    }
                }
    
                if ui.button("Print lines as csv").clicked() {
                    println!("{}",lines_to_csv(&app.lines, &app.sales_screen.skiplist));
                }
            });
            ui.horizontal(|ui| {
                if ui.button("show id").clicked() {app.sales_screen.skiplist[0] ^= 0xFF;}
                if ui.button("show name").clicked() {app.sales_screen.skiplist[1] ^= 0xFF;}
                if ui.button("show description").clicked() {app.sales_screen.skiplist[2] ^= 0xFF;}
                if ui.button("show price").clicked() {app.sales_screen.skiplist[3] ^= 0xFF;}
                if ui.button("show location").clicked() {app.sales_screen.skiplist[4] ^= 0xFF;}
            });

            for i in 0..app.lines.len() {
                ui.horizontal(|ui| {
                    if app.sales_screen.skiplist[0] != 0 {
                        ui.add(egui::TextEdit::singleline(&mut app.lines[i].id).desired_width(75.0));
                    }
                    if app.sales_screen.skiplist[1] != 0 {
                        ui.add(egui::TextEdit::singleline(&mut app.lines[i].name).desired_width(75.0));
                    }
                    if app.sales_screen.skiplist[2] != 0 {
                        ui.add(egui::TextEdit::singleline(&mut app.lines[i].description).desired_width(75.0));
                    }
                    if app.sales_screen.skiplist[3] != 0 {
                        ui.add(egui::TextEdit::singleline(&mut app.lines[i].price).desired_width(75.0));
                    }
                    if app.sales_screen.skiplist[4] != 0 {                        
                        ui.add(egui::TextEdit::singleline(&mut app.lines[i].location).desired_width(75.0));
                    }
                });
            }
        });
    });
}

// ####################### END OF SALES SCREEN #############################################################

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