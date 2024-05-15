
use std::fmt::Display;

use egui::RichText;
use poll_promise::Promise;

use crate::components;
use crate::utilities::*;

use crate::screens::*;
use self::admin_screen::show_admin_screen;
use self::admin_screen::AdminScreen;
use self::login_screen::show_login_screen;
use self::login_screen::LoginScreen;
use self::query_sender_screen::show_query_creator_screen;
use self::query_sender_screen::QuerySenderScreen;
use self::sales_screen::show_sales_screen;
use self::table_creator_screen::show_table_creator_screen;
use self::table_creator_screen::TableCreatorScreen;

#[derive(Debug)]
pub enum Screen {
    Admin,
    Login,
    Purchase,
    Sales,
    Transfer,
    TableCreator,
    QuerySender,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Product {
    pub id: String,
    pub name: String,
    pub description: String,
    pub location: String,
    pub price: String,
}

impl Default for Product {
    fn default() -> Self {
        Product {
            id: "id".to_owned(),
            name: "name".to_owned(),
            description: "description".to_owned(),
            location: "location".to_owned(),
            price: "price".to_owned(),
        }       
    }
}

impl Display for Product {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{};{};{};{};{}", self.id, self.name, self.description, self.location, self.price)
    }
}

#[derive(Default)]
pub struct SalesScreen {
    pub skiplist: [u8;5],
}

// ##################### THIS IS THE STATE OF THE APPLICATION ###################################################
pub struct App {
    pub label: String,
    pub screen: Screen,
    pub value: f32,
    pub lines: Vec<Product>,
    pub login_screen: LoginScreen,
    pub admin_screen: AdminScreen,
    pub sales_screen: SalesScreen,
    pub table_creator_screen: TableCreatorScreen,
    pub query_sender_screen: QuerySenderScreen,
}

impl Default for App {
    fn default() -> Self {
        Self {
            // Example stuff:
            label: "Hello World!".to_owned(),
            screen: Screen::Admin,
            value: 2.7,
            lines: Vec::new(),
            login_screen: LoginScreen::default(),
            admin_screen: AdminScreen::default(),
            sales_screen: SalesScreen::default(),
            table_creator_screen: TableCreatorScreen::default(),
            query_sender_screen: QuerySenderScreen::default(),
        }
    }
}

// ##############################################################################################################

impl App {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        // if let Some(storage) = cc.storage {
        //     return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        // }

        Default::default()
    }
}

impl eframe::App for App {
    /// Called by the framework to save state before shutdown.
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {

    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui


        match self.screen {
            Screen::Admin => show_admin_screen(self, ctx),
            Screen::Login => show_login_screen(self, ctx),
            Screen::Purchase => show_default_screen(self, ctx),
            Screen::Transfer => show_default_screen(self, ctx),
            Screen::Sales => show_sales_screen(self, ctx),
            Screen::TableCreator => show_table_creator_screen(self, ctx),
            Screen::QuerySender => show_query_creator_screen(self, ctx),
        };

    }
}

pub fn show_default_screen(mut app: &mut App, ctx: &egui::Context) {
    components::default_top_bar(ctx, &mut app);
    
    components::default_central_panel(app, ctx);
    
}

pub fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}