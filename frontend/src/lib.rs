mod components;
use yew::{function_component, html, Html};

use components::navbar::Navbar;
#[function_component]
pub fn App() -> Html{

    html!{
        <>
            <h1> {"Ecommerce website wasm"}</h1>
            <Navbar/>
        </>
    }
}
