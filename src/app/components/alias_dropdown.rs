use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AliasDropdownProps {
    pub show: bool,
    pub selected: Option<usize>,
    pub matched_aliases: Vec<String>,
    pub on_click: Callback<String>,
    pub on_hover: Callback<Option<String>>
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

    let on_mouseleave = {
        let on_hover = props.on_hover.clone();
        Callback::from(move |_| {
            on_hover.emit(None);
        })
    };

    let on_mouseover = {
        let on_hover = props.on_hover.clone();
        Callback::from(move |e: MouseEvent| {
            let el = e.target_dyn_into::<web_sys::HtmlDivElement>().unwrap();
            let alias = el.inner_html();
            on_hover.emit(Some(alias));
        })
    };

    html! {
        <div
            class={classes!("dropdown", if props.show { None } else { Some("invisible") })}
            onmouseleave={on_mouseleave.clone()}
        >
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
                            onmouseover={on_mouseover.clone()}
                        >
                            {alias.clone()}
                        </div>
                    }
                }).collect::<Html>()
            }
        </div>
    }
}
