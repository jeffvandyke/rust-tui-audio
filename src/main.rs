#![deny(clippy::all)]

mod app;
mod data_buffer;

use app::*;

fn main() {
    let mut my_app = App::init().expect("App failed init");
    my_app.run().expect("Failure running app");
}
