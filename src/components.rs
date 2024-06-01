use egui::Ui;

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
                        app.current_screen = Screen::Admin;
                        ui.close_menu();
                    }
                    #[cfg(debug_assertions)]
                    if ui.button("Login").clicked() {
                        app.current_screen = Screen::Login;
                        ui.close_menu();
                    }
                    if ui.button("Purchase").clicked() {
                        app.current_screen = Screen::Purchase;
                        ui.close_menu();
                        
                    }
                    if ui.button("Sales").clicked() {
                        app.current_screen = Screen::Sales;
                        ui.close_menu();
                        
                    }
                    if ui.button("Transfer").clicked() {
                        app.current_screen = Screen::Transfer;
                        ui.close_menu();
                        
                    }
                    if ui.button("Table Creator").clicked() {
                        app.current_screen = Screen::TableCreator;
                        ui.close_menu();
                    }
                    if ui.button("Query sender").clicked() {
                        app.current_screen = Screen::QuerySender;
                        ui.close_menu();
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    match app.current_screen {
                        Screen::Admin => todo!(),
                        Screen::Login => todo!(),
                        Screen::Purchase => todo!(),
                        Screen::Sales => ui.label("To remove a line, double click the 'remove line' button of the line you want to remove"),
                        Screen::Transfer => todo!(),
                        Screen::TableCreator => todo!(),
                        Screen::QuerySender => todo!(),
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

pub fn list_of_lines(ui: &mut Ui, lines_ref: &mut Vec<Vec<String>>, default_line: Vec<String>) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            if ui.button("add line").clicked() {
                lines_ref.push(default_line);
            }
        });

        let mut remover: Option<usize> = None;
        for (index, line) in &mut lines_ref.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                ui.label(index.to_string());
                if ui.button("remove line").double_clicked() {
                    remover = Some(index);
                }
                for i in 0..line.len() {
                    ui.add(egui::TextEdit::singleline(&mut line[i]).desired_width(75.0));
                }
            });
        }

        if let Some(index) = remover {
            lines_ref.remove(index);
        }
    });
}