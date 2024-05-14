use egui::RichText;
use poll_promise::Promise;

use crate::{app::{LoginScreen, Screen}, components, App};




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