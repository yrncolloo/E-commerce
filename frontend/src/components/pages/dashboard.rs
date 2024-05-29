use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component]
pub fn Dashboard() -> Html{
    html!{
        <p> {"From dashboard"}</p>
    }
    
}
