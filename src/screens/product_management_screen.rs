use mysql::prelude::Queryable;
use poll_promise::Promise;

use crate::{components::{default_central_panel, default_top_bar, list_of_lines}, utilities::lines_to_product_insert, App};


#[derive(Default)]
pub struct ProductManagementScreen {
    inserts: Vec<Vec<String>>,
    promise: Option<Promise<Vec<Product>>>,
    products_text: String,
}


#[derive(Debug, PartialEq)]
struct Product {
    id: i32,
    price: f32,
    name: String,
    description: String,
    picture: String,
}


pub fn show_product_management_screen(app: &mut App, ctx: &egui::Context) {
    
    default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.label("This is the product management panel. Here you can enter items into the product table.");
        let ctx_clone = ctx.clone();
        if ui.button("Insert products").clicked() {
            let mut conn = app.database_connection.get_conn().unwrap();
            let query = format!("INSERT INTO products (id, name, description, price, picture) VALUES {}", lines_to_product_insert(&app.product_management_screen.inserts));
            println!("{}", query);
            conn.query_drop(query).unwrap();
            ctx_clone.request_repaint(); // wake up UI thread
        }
        list_of_lines(
            ui,
            ctx,
            &mut app.product_management_screen.inserts, 
            vec!["id".to_owned(), "name".to_owned(), "description".to_owned(), "price".to_owned(), "picture".to_owned()],
            vec!["".to_owned(), "".to_owned(), "id".to_owned(), "name".to_owned(), "description".to_owned(), "price".to_owned(), "picture".to_owned()],
        );

        if ui.button("Show all products").clicked() {
            let mut conn = app.database_connection.get_conn().unwrap();
            let promise = poll_promise::Promise::spawn_thread("SELECT * FROM products", move || {
                conn.query_map(
                    "SELECT * from products",
                    |(id, price, name, description, picture)| {
                        Product { id, price, name, description, picture }
                    },
                ).unwrap()
            });
            app.product_management_screen.promise = Some(promise);
        }
        ui.text_edit_multiline(&mut app.product_management_screen.products_text);
        if let Some(promise) = &app.product_management_screen.promise {
            if let Some(products) = promise.ready() {
                app.product_management_screen.products_text = format!("{:?}", products);
            }
        }
    });
}
