use poll_promise::Promise;

use crate::{components::{default_central_panel, default_top_bar, list_of_lines}, App};


#[derive(Default)]
pub struct ProductManagementScreen {
    lines: Vec<Vec<String>>,
    promise: Option<Promise<String>>,
}


pub fn show_product_management_screen(app: &mut App, ctx: &egui::Context) {
    
    default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("This is the product management panel. Here you can enter items into the product table.");
        let ctx_clone = ctx.clone();
        if ui.button("Insert products").clicked() {
            // let query = format!("INSERT INTO products {})", lines_to_csv(&app.product_management_screen.lines));
            // println!("query: {}", query);
            // let promise = Promise::spawn_thread("Insert products", move || {
            //     let good_csv = EZDB::client_networking::query_table("127.0.0.1:3004", "admin", "admin", &query);
            //     ctx_clone.request_repaint(); // wake up UI thread
            //     match good_csv {
            //         Ok(csv) => match csv {
            //             EZDB::client_networking::Response::Message(message) => message,
            //             EZDB::client_networking::Response::Table(table) => table.to_string(),
            //         },
            //         Err(e) => format!("Could not retreive data because: {e}"),
            //     }
            // });
            // app.product_management_screen.promise = Some(promise);
        }
        list_of_lines(
            ui,
            ctx,
            &mut app.product_management_screen.lines, 
            vec!["id".to_owned(), "name".to_owned(), "description".to_owned(), "price".to_owned(), "picture".to_owned()],
            vec!["".to_owned(), "".to_owned(), "id".to_owned(), "name".to_owned(), "description".to_owned(), "price".to_owned(), "picture".to_owned()],
        );
    });
}