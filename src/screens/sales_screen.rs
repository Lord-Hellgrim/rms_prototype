use poll_promise::Promise;

use crate::{app::Product, components::{self, list_of_lines}, utilities::lines_to_csv, App};


pub struct SalesScreen {
    lines: Vec<Vec<String>>,
}

impl Default for SalesScreen {
    fn default() -> Self {
        SalesScreen {
            lines: vec![vec!["first".to_owned(), "default".to_owned(), "line".to_owned(),]]
        }
    }
}


pub fn show_sales_screen(app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        ui.heading("SALES SCREEN");

        list_of_lines(ui, &mut app.sales_screen.lines, vec!["default".to_owned(), "line".to_owned(), "entry".to_owned(),]);
    });
}