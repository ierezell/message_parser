use crate::web::facebook::FacebookMultiFileSelectorComponent;
use crate::web::whatsapp::WhatsappMultiFileSelectorComponent;

use leptos::{component, view, IntoView};

#[component]
pub fn App() -> impl IntoView {
    view! {
        <FacebookMultiFileSelectorComponent/>
        <WhatsappMultiFileSelectorComponent/>
    }
}
