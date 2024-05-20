use stylist::yew::styled_component;
use yew::{html, Html};

#[styled_component]
pub fn Navbar() -> Html{
    let titles = vec!["home".to_string(), "product".to_string(), "dashboard".to_string(), "contact".to_string(), "Blog".to_string(), "Contact".to_string()];
    html!{
        <div class={"navbar"}>
            <nav class={"nav"}>
                <ul class={"nav-links"}>
                    {list_to_html(titles)}
                </ul>
            </nav>
        </div>

    }
    
}

fn list_to_html(list: Vec<String>) -> Vec<Html> {
    list.into_iter().map(|items|html!{<li class={"navbar-list"}> <a href={"#"}> {items} </a> </li>}).collect()
    
}
