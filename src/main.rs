#![deny(clippy::all)]

mod app;
mod data_buffer;
mod ui;

use app::*;

fn main() {
    let mut my_app = App::init().expect("App failed init");
    let mut ui = ui::Ui::init().expect("Ui Initialization failure");
    my_app.run(&mut ui).expect("Failure running app");
}
