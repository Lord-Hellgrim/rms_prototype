use poll_promise::Promise;

use crate::{components, App};


#[derive(Default)]
pub struct AdminScreen {
    pub table_text: String,
    pub table_title: String,
    pub table_confirmation: String,
    pub promise: Option<Promise<String>>,
}

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
            ui.label("Table name");
            ui.text_edit_singleline(&mut app.admin_screen.table_title);
            ui.label("EZCSV formatted string");
            ui.text_edit_multiline(&mut app.admin_screen.table_text);
        });
        

        ui.separator();
        
    });
}