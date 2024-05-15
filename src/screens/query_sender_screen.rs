use poll_promise::Promise;
use EZDB::db_structure::KeyString;

use crate::{components, utilities::lines_to_ezcsv, App};


#[derive(Default)]
pub struct QuerySenderScreen {
    query_string: String,
    query_result: String,
    promise: Option<Promise<String>>,
}




pub fn show_query_creator_screen(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("EZTABLE CREATOR");

        ui.separator();

        ui.label("Query:");
        ui.text_edit_singleline(&mut app.query_sender_screen.query_string);
        if ui.button("send query").clicked() {
            let ctx_clone = ctx.clone();
            let query_string = app.query_sender_screen.query_string.clone();
            let promise = Promise::spawn_thread("query", move || {
                let answer = EZDB::client_networking::query_table("127.0.0.1:3004", "admin", "admin", &query_string);
                ctx_clone.request_repaint(); // wake up UI thread
                match answer {
                    Ok(csv) => {
                        csv
                    },
                    Err(e) => format!("Could not retreive data because: {e}"),
                }
            });
            app.query_sender_screen.promise = Some(promise);
        }

        if let Some(promise) = &app.query_sender_screen.promise {
            match promise.ready() {
                Some(result) => {
                    app.query_sender_screen.query_result = result.clone();
                    app.query_sender_screen.promise = None;
                },
                None => (),
            }
        }

        ui.text_edit_multiline(&mut app.query_sender_screen.query_result);
        
        
    });
}