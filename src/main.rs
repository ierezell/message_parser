mod parsers;
mod web;

use leptos::*;

use web::app::App;

fn main() {
    mount_to_body(|| view! { <App/> });
}
