use yew::{html, Html};
use yew_router::Routable;

use crate::components::pages::{ home::Home, dashboard::Dashboard, products::Products, contact::Contact};

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/dashboard")]
    Dashboard,
    #[at("/products")]
    Products,
    #[at("/contact")]
    Contact
}

pub fn switch(routes: Route) -> Html {
    
    match routes {
        Route::Home => html!{ <Home/> },
        Route::Dashboard => html!{ <Dashboard/>},
        Route::Products => html!{ <Products/> },
        Route::Contact => html!{ <Contact/> }

        
    }
}
