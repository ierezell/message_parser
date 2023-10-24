mod parsers;
mod web;

use leptos::mount_to_body;

use web::app::App;

fn main() {
    mount_to_body(App);
}
