use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component]
pub fn Navbar() -> Html{
    let titles = vec!["Home".to_string(), "Shop".to_string(), "Collections".to_string(), "Accessories".to_string(), "Blog".to_string(), "Contact".to_string()];
    html!{
        <div class={"navbar"}>
            <ul>
                {list_to_html(titles)}
            </ul>
        </div>

    }
    
}

fn list_to_html(list: Vec<String>) -> Vec<Html> {
    list.into_iter().map(|items|html!{<li class={"navbar-list"}> {items} </li>}).collect()
    
}
