use yew::prelude::*;
use yew::functional::use_state;
use yew::context::ContextProvider;
use yew_router::prelude::*;

use crate::components::{Header, Footer};
use crate::pages::{home::Home, catalog::Catalog, cart::Cart, login::Login, profile::Profile, signup::Signup};
use crate::models::AppState;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/catalog")]
    Catalog,
    #[at("/cart")]
    Cart,
    #[at("/login")]
    Login,
    #[at("/signup")]
    Signup,
    #[at("/profile")]
    Profile,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { <Home /> },
        Route::Catalog => html! { <Catalog /> },
        Route::Cart => html! { <Cart /> },
        Route::Login => html! { <Login /> },
            Route::Signup => html! { <Signup /> },
        Route::Profile => html! { <Profile /> },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    // Provide a global application state via context so pages/components can share cart and user state.
    let app_state = use_state(|| AppState::default());

    html! {
        <ContextProvider<UseStateHandle<AppState>> context={app_state.clone()}>
            <BrowserRouter>
                <div class="app">
                    <Header />
                    <main class="main-content">
                        <Switch<Route> render={switch} />
                    </main>
                    <Footer />
                </div>
            </BrowserRouter>
        </ContextProvider<UseStateHandle<AppState>>>
    }
}