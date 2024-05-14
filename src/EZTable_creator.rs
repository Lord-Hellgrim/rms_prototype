use poll_promise::Promise;
use EZDB::db_structure::KeyString;

use crate::{components, utilities::lines_to_ezcsv, App};

#[derive(Default)]
pub struct TableCreatorScreen {
    table_name: String,
    header: String,
    lines: Vec<Vec<String>>,
    promise: Option<Promise<String>>,
}



pub fn show_table_creator_screen(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("EZTABLE CREATOR");

        ui.separator();

        ui.label("Table name:");
        ui.text_edit_singleline(&mut app.table_creator_screen.table_name);
        ui.label("Table header (must be EZCSV formatted");
        ui.text_edit_singleline(&mut app.table_creator_screen.header);
        ui.horizontal(|ui| {
            ui.label("lines");
            if ui.button("Add line").clicked() {
                let header_len = app.table_creator_screen.header.split(';').count();
                let mut temp = Vec::with_capacity(app.table_creator_screen.header.split(';').count());
                for i in 0..header_len {
                    temp.push(String::from(""));
                }
                app.table_creator_screen.lines.push(temp);
            }
            if ui.button("print lines").clicked() {
                println!("{}", lines_to_ezcsv(&app.table_creator_screen.lines));
            }
        });
        let mut remover: Option<usize> = None;
        for (index, line) in &mut app.table_creator_screen.lines.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                for i in 0..line.len() {
                    ui.add(egui::TextEdit::singleline(&mut line[i]).desired_width(75.0));
                }
                if ui.button("remove line").clicked() {
                    remover = Some(index);
                }
            });
        }

        if let Some(index) = remover {
            app.table_creator_screen.lines.remove(index);
        }
        
    });
}