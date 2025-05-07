use poll_promise::Promise;

use crate::{app::Product, components::{self, list_of_lines}, utilities::lines_to_csv, App};
use egui::Vec2b;


pub struct SalesScreen {
    lines: Vec<Vec<String>>,
    window_open: bool,
}

impl Default for SalesScreen {
    fn default() -> Self {
        SalesScreen {
            lines: vec![vec!["id".to_owned(), "name".to_owned(), "price".to_owned(), "location".to_owned(), "stock".to_owned()]],
            window_open: true,
        }
    }
}


pub fn show_sales_screen(app: &mut App, ctx: &egui::Context) {

    ctx.set_style(egui::Style{
        visuals: egui::Visuals {
            // extreme_bg_color: egui::Color32::from_rgb(0,0,255),
            ..Default::default()
        },
        ..Default::default()
    });

    components::default_top_bar(ctx, app);

    egui::CentralPanel::default().show(ctx, |ui| {

        let title = "🗖 Window Options".to_owned();
        let title_bar = true;
        let closable = true;
        let collapsible = true;
        let resizable = true;
        let constrain = true;
        let scroll2 = Vec2b::TRUE;
        let disabled_time = f64::NEG_INFINITY;
        let anchored = false;
        let anchor = egui::Align2::RIGHT_TOP;
        let anchor_offset = egui::Vec2::ZERO;

        let mut window = egui::Window::new(title)
            .id(egui::Id::new("demo_window_options")) // required since we change the title
            .resizable(resizable)
            .constrain(constrain)
            .collapsible(collapsible)
            .title_bar(title_bar)
            .scroll(scroll2)
            .enabled(true);
        if closable {
            window = window.open(&mut app.sales_screen.window_open);
        }
        if anchored {
            window = window.anchor(anchor, anchor_offset);
        }
        window.show(ctx, |ui| {
            ui.label("heyo!");
        });

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