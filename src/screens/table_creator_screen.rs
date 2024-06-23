use poll_promise::Promise;
use EZDB::db_structure::KeyString;

use crate::{components, utilities::lines_to_ezcsv, App, screens::query_sender_screen};


pub struct TableCreatorScreen {
    table_name: String,
    header: String,
    lines: Vec<Vec<String>>,
    promise: Option<Promise<String>>,
}

impl Default for TableCreatorScreen {
    fn default() -> TableCreatorScreen {
        TableCreatorScreen {
            table_name: "test".to_owned(),
            header: "key,i-P;data,i-N;other,i-N".to_owned(),
            lines: Vec::new(),
            promise: None,
        }
    }
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
                let mut ezcsv = app.table_creator_screen.header.clone();
                ezcsv.push_str(&lines_to_ezcsv(&app.table_creator_screen.lines));
            }

            if ui.button("send table").clicked() {
                let ctx_clone = ctx.clone();
                let mut ezcsv = app.table_creator_screen.header.clone();
                ezcsv.push('\n');
                ezcsv.push_str(&lines_to_ezcsv(&app.table_creator_screen.lines));
                let table_name = app.table_creator_screen.table_name.clone();
                let promise = Promise::spawn_thread("send table", move || {
                    let answer = EZDB::client_networking::upload_table("127.0.0.1:3004", "admin", "admin", &table_name, &ezcsv);
                    std::thread::sleep(std::time::Duration::from_secs(3));
                    ctx_clone.request_repaint(); // wake up UI thread
                    match answer {
                        Ok(_) => "OK".to_owned(),
                        Err(e) => format!("Could not retreive data because: {e}"),
                    }
                });
                app.table_creator_screen.promise = Some(promise);
            };
            if let Some(promise) = &app.table_creator_screen.promise {
                match promise.ready() {
                    Some(s) => ui.label(s),
                    None => ui.spinner(),
                };
            };

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