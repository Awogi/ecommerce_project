use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::{Header, Footer};
use crate::pages::{home::Home, catalog::Catalog, cart::Cart, login::Login, profile::Profile, signup::Signup};

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
    html! {
        <BrowserRouter>
            <div class="app">
                <Header />
                <main class="main-content">
                    <Switch<Route> render={switch} />
                </main>
                <Footer />
            </div>
        </BrowserRouter>
    }
}