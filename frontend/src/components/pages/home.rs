use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component]
pub fn Home() -> Html{
    html!{
        <p> {"From home page"}</p>
    }
    
}
