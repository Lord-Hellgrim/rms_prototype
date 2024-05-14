use poll_promise::Promise;

use crate::{app::Product, components, utilities::lines_to_csv, App};



pub fn show_sales_screen(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

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