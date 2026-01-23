use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AliasDropdownProps {
    pub show: bool,
    pub selected: Option<usize>,
    pub matched_aliases: Vec<String>,
    pub on_click: Callback<String>,
    pub on_hover: Callback<Option<usize>>
}

#[function_component(AliasDropdown)]
pub fn alias_dropdown(props: &AliasDropdownProps) -> Html {
    {
        let selected = props.selected.clone();
        let selected2 = selected.clone();
        use_effect_with_deps(
            move |_| {
                if let Some(idx) = selected {
                    let el = web_sys::window().unwrap()
                        .document().unwrap()
                        .get_element_by_id(&format!("alias-{}", idx)).unwrap();
                    let options = web_sys::ScrollIntoViewOptions::new();
                    options.set_block(web_sys::ScrollLogicalPosition::Nearest);
                    el.scroll_into_view_with_scroll_into_view_options(&options);
                }
            },
            selected2,
        )
    }

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
            let alias_idx: usize = el.id()
                .rsplit_once("-").unwrap()
                .1.parse().unwrap();
            on_hover.emit(Some(alias_idx));
        })
    };

    html! {
        <div
            class={classes!(
                "dropdown",
                if !props.show || props.matched_aliases.len() == 0 {
                    Some("invisible")
                } else {
                    None
                }
            )}
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
                            id={format!("alias-{}", idx)}
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
