use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AliasDropdownProps {
    pub show: bool,
    pub selected: Option<usize>,
    pub matched_aliases: Vec<String>,
    pub on_click: Callback<String>,
}

#[function_component(AliasDropdown)]
pub fn alias_dropdown(props: &AliasDropdownProps) -> Html {
    let on_click = {
        let on_click = props.on_click.clone();
        Callback::from(move |e: MouseEvent| {
            let el = e.target_dyn_into::<web_sys::HtmlDivElement>().unwrap();
            let alias = el.inner_html();
            on_click.emit(alias);
        })
    };

    html! {
        <div class={classes!("dropdown", if props.show { None } else { Some("invisible") })}>
            {
                (*props.matched_aliases).iter().enumerate().map(|(idx, alias)| {
                    let maybe_selected =
                        if props.selected.is_some() && idx == props.selected.unwrap() {
                            Some("selected")
                        } else {
                            None
                        };
                    html!{
                        <div
                            class={classes!("variant", maybe_selected)}
                            key={alias.clone()}
                            onclick={on_click.clone()}
                        >
                            {alias.clone()}
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
