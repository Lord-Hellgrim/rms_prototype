use egui::Ui;


use crate::app::*;

#[inline]
pub fn default_top_bar(ctx: &egui::Context, app: &mut App) {
    egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
        // The top panel is often a good place for a menu bar:
        
        egui::containers::menu::Bar::new().ui(ui, |ui| {
            // NOTE: no File->Quit on web pages!
            let is_web = cfg!(target_arch = "wasm32");
            if !is_web {
                ui.menu_button("File", |ui| {
                    if ui.button("Admin").clicked() {
                        app.current_screen = Screen::Admin;
                        ui.close();
                    }
                    #[cfg(debug_assertions)]
                    if ui.button("Login").clicked() {
                        app.current_screen = Screen::Login;
                        ui.close();
                    }
                    if ui.button("Purchase").clicked() {
                        app.current_screen = Screen::Purchase;
                        ui.close();
                        
                    }
                    if ui.button("Sales").clicked() {
                        app.current_screen = Screen::Sales;
                        ui.close();
                        
                    }
                    if ui.button("Transfer").clicked() {
                        app.current_screen = Screen::Transfer;
                        ui.close();
                        
                    }
                    if ui.button("Quit").clicked() {
                        ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                    }
                });
                ui.menu_button("Help", |ui| {
                    match app.current_screen {
                        Screen::Admin => ui.label("no contextual help yet"),
                        Screen::Login => ui.label("no contextual help yet"),
                        Screen::Purchase => ui.label("no contextual help yet"),
                        Screen::Sales => ui.label("To remove a line, double click the 'remove line' button of the line you want to remove"),
                        Screen::Transfer => ui.label("no contextual help yet"),

                    }
                });
                ui.add_space(16.0);
            }

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

pub fn list_of_lines(ui: &mut Ui, ctx: &egui::Context, lines_ref: &mut Vec<Vec<String>>, default_line: Vec<String>, header: Vec<String>) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            let res = ui.button("add line");
            let mut window = egui::containers::Window::new("context menu");
            if res.clicked() {
                lines_ref.push(default_line);
            } else if res.secondary_clicked() {
                println!("Right clicked!");
                window.current_pos(res.ctx.pointer_latest_pos().unwrap()).show(ctx, |ui| {
                    ui.label("Testing!");
                });
            }
        });
        
        let mut remover: Option<usize> = None;
        egui::Grid::new("some_unique_id").min_col_width(75.0).striped(true).show(ui, |ui| {
            for item in header {
                ui.label(&item);
            }

            ui.end_row();
            for (index, line) in &mut lines_ref.iter_mut().enumerate() {
                ui.label(index.to_string());
                if ui.button("remove line").double_clicked() {
                    remover = Some(index);
                }
                for i in 0..line.len() {
                    ui.add(egui::TextEdit::singleline(&mut line[i]).desired_width(75.0));
                }
                ui.end_row();
            }
        });

        if let Some(index) = remover {
            lines_ref.remove(index);
        }
    });
}
