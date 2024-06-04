use egui::RichText;
use poll_promise::Promise;

use crate::{app::Screen, components, utilities::String64, App};


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct LoginScreen {
    pub username: String64,
    pub password: String64,
    pub error: String,
    pub username_focus: bool,
}

impl Default for LoginScreen {
    fn default() -> Self {
        LoginScreen {
            username: String64::new(),
            password: String64::new(),
            error: "".to_owned(),
            username_focus: true,
        }
    }
}

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
                if app.login_screen.password == "admin".into() && app.login_screen.username == "admin".into() {
                    app.login_screen = LoginScreen::default();
                    app.current_screen = Screen::Admin;
                } else {
                    app.login_screen.error = "Wrong username or password".to_owned();
                }           
            }
            ui.label(RichText::new(app.login_screen.error.clone()).color(egui::Color32::RED));
        });

    });

    if ctx.input(|i| i.key_pressed(egui::Key::Enter)) {
        if app.login_screen.password == "admin".into() && app.login_screen.username == "admin".into() {
            app.login_screen = LoginScreen::default();
            app.current_screen = Screen::Admin;
        } else {
            app.login_screen.error = "Wrong username or password".to_owned();
        } 
        
    }
}