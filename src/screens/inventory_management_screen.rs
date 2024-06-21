use EZDB::db_structure::KeyString;

use crate::{components::{default_central_panel, default_top_bar, list_of_lines}, App};


#[derive(Default)]
pub struct ProductManagementScreen {
    lines: Vec<Vec<String>>,
}


pub fn show_product_management_screen(app: &mut App, ctx: &egui::Context) {
    
    default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("This is the product management panel. Here you can enter items into the product table.");
        list_of_lines(
            ui,
            ctx,
            &mut app.product_management_screen.lines, 
            vec!["default".to_owned(), "line".to_owned(), "entry".to_owned(),],
            vec!["id".to_owned(), "name".to_owned(), "price".to_owned(), "picture".to_owned()],
        );
    });
}