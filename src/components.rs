use crate::app::*;

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
                        ui.close_menu();
                    }
                    #[cfg(debug_assertions)]
                    if ui.button("Login").clicked() {
                        app.screen = Screen::Login;
                        ui.close_menu();
                    }
                    if ui.button("Purchase").clicked() {
                        app.screen = Screen::Purchase;
                        ui.close_menu();
                        
                    }
                    if ui.button("Sales").clicked() {
                        app.screen = Screen::Sales;
                        ui.close_menu();
                        
                    }
                    if ui.button("Transfer").clicked() {
                        app.screen = Screen::Transfer;
                        ui.close_menu();
                        
                    }
                    if ui.button("Table Creator").clicked() {
                        app.screen = Screen::TableCreator;
                        ui.close_menu();
                    }
                    if ui.button("Query sender").clicked() {
                        app.screen = Screen::QuerySender;
                        ui.close_menu();
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

pub fn default_central_panel(app: &mut App, ctx: &egui::Context) {
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