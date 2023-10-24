use crate::web::facebook::FacebookMultiFileSelectorComponent;
use crate::web::whatsapp::WhatsappMultiFileSelectorComponent;

use leptos::{component, view, IntoView};
use leptos_router::{Route, Router, Routes, A};

#[component]
pub fn App() -> impl IntoView {
    view! {
      <Router>
        <nav>
          <A href="/facebook">Facebook</A>
          <br/>
          <A href="/whatsapp">Whatsapp</A>
        </nav>
        <main>
          <Routes>
            <Route path="/" view=|| view! {
              <A href="/facebook">Facebook</A>
              <br/>
              <A href="/whatsapp">Whatsapp</A>
            }
            />
            <Route path="/facebook" view=FacebookMultiFileSelectorComponent/>
            <Route path="/whatsapp" view=WhatsappMultiFileSelectorComponent/>
          </Routes>
        </main>
      </Router>
    }
}
