mod components;
mod router;
use yew::{function_component, html, Html};
use yew_router::{ BrowserRouter, Switch};

use components::navbar::Navbar;
use router::{Route, switch};


#[function_component]
pub fn App() -> Html{

    html!{
        <>
            //<h1> {"Ecommerce website wasm"}</h1>
            //<h1> {"I will be back in 10 minutes"}</h1>
            <Navbar/>
            <BrowserRouter>
                <Switch<Route> render={switch} />
                
            </BrowserRouter>
        </>
    }
}
