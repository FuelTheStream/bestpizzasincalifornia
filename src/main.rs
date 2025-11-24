mod data;
mod components;
mod app;

use app::App;

fn main() {
    dioxus::launch(App);
}
