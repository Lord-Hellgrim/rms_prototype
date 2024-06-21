use poll_promise::Promise;

use crate::{app::Product, components::{self, list_of_lines}, utilities::lines_to_csv, App};


pub struct SalesScreen {
    lines: Vec<Vec<String>>,
}

impl Default for SalesScreen {
    fn default() -> Self {
        SalesScreen {
            lines: vec![vec!["id".to_owned(), "name".to_owned(), "price".to_owned(), "location".to_owned(), "stock".to_owned()]]
        }
    }
}


pub fn show_sales_screen(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("SALES SCREEN");

        list_of_lines(
            ui,
            ctx,
            &mut app.sales_screen.lines, 
            vec!["id".to_owned(), "name".to_owned(), "price".to_owned(), "location".to_owned(), "stock".to_owned()], 
            vec![" ".to_owned(), " ".to_owned(), "id".to_owned(), "name".to_owned(), "price".to_owned(), "location".to_owned(), "stock".to_owned()],
        );
    });

}