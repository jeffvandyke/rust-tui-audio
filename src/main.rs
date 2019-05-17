#![deny(clippy::all)]
#![deny(clippy::pedantic)]

mod app;

use app::*;

fn main() {
    let mut my_app = App::init().expect("App failed init");
    my_app.run().expect("Failure running app");
}
