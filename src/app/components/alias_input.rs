use yew::prelude::*;

use super::alias_dropdown::AliasDropdown;

#[derive(Properties, PartialEq)]
pub struct AliasInputProps {
    pub text: String,
    pub known_aliases: Vec<String>,
    pub keyboard: bool,
    pub on_input: Callback<(String, bool)>,
    pub on_focus: Callback<NodeRef>,
    pub on_enter: Callback<()>,
}

#[function_component(AliasInput)]
pub fn alias_input(props: &AliasInputProps) -> Html {
    let focused = use_state_eq(|| false);
    let show_dropdown = use_state_eq(|| false);
    let dropdown_closed_on_select = use_state(|| false);
    let dropdown_selected_idx = use_state(|| None::<usize>);
    let alias_matches = use_memo(
        |(aliases, string)| {
            let matches: Vec<String> = aliases.iter()
                .filter(|a| a.contains(string))
                .map(|a| a.clone())
                .collect();
            matches
        },
        (props.known_aliases.clone(), props.text.clone())
    );
    let dropdown_last_idx = use_memo(
        |matches| {
            if matches.len() > 0 { Some(matches.len() - 1) }
            else { None }
        },
        alias_matches.clone()
    );
    let input_ref = use_node_ref();

    { // Show dropdown if input value changed
        let text = props.text.clone();
        let focused = focused.clone();
        let show_dropdown = show_dropdown.clone();
        let dropdown_closed_on_select = dropdown_closed_on_select.clone();
        use_effect_with_deps(
            move |_| {
                if *focused {
                    if !*dropdown_closed_on_select {
                        show_dropdown.set(true);
                    } else {
                        dropdown_closed_on_select.set(false);
                    }
                }
            },
            text,
        );
    }
    {
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |_| {
                let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                input.focus().unwrap();
            },
            (),
        );
    }

    let check_alias = {
        let known_aliases = props.known_aliases.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |alias: String| {
            let known = known_aliases.contains(&alias);
            on_input.emit((alias, known));
        })
    };

    let on_input = {
        let input_ref = input_ref.clone();
        let check_alias = check_alias.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let alias = input.value();
            check_alias.emit(alias.clone());
        })
    };

    let on_focus = {
        let focused = focused.clone();
        let show_dropdown = show_dropdown.clone();
        let input_ref = input_ref.clone();
        let on_focus = props.on_focus.clone();
        Callback::from(move |_| {
            focused.set(true);
            show_dropdown.set(true);
            on_focus.emit(input_ref.clone());
        })
    };

    let on_blur = {
        let focused = focused.clone();
        let show_dropdown = show_dropdown.clone();
        Callback::from(move |_| {
            focused.set(false);
            show_dropdown.set(false);
        })
    };

    let on_key_down = {
        let show_dropdown = show_dropdown.clone();
        let dropdown_closed_on_select = dropdown_closed_on_select.clone();
        let on_enter = props.on_enter.clone();
        let dropdown_selected_idx = dropdown_selected_idx.clone();
        let alias_matches = alias_matches.clone();
        let dropdown_last_idx = dropdown_last_idx.clone();
        let check_alias = check_alias.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                if let Some(selected_idx) = *dropdown_selected_idx {
                    let alias = alias_matches.get(selected_idx).unwrap();
                    show_dropdown.set(false);
                    dropdown_closed_on_select.set(true);
                    dropdown_selected_idx.set(None);
                    check_alias.emit(alias.clone());
                } else {
                    on_enter.emit(());
                }
            } else if e.key() == "ArrowDown" {
                if let Some(last_idx) = *dropdown_last_idx {
                    if let Some(prev_selected) = *dropdown_selected_idx {
                        if prev_selected < last_idx {
                            dropdown_selected_idx.set(Some(prev_selected + 1));
                        } else {
                            dropdown_selected_idx.set(Some(0));
                        }
                    } else {
                        dropdown_selected_idx.set(Some(0));
                    }
                    show_dropdown.set(true);
                }
            } else if e.key() == "ArrowUp" {
                if let Some(last_idx) = *dropdown_last_idx {
                    if let Some(prev_selected) = *dropdown_selected_idx {
                        if prev_selected > 0 {
                            dropdown_selected_idx.set(Some(prev_selected - 1));
                        } else {
                            dropdown_selected_idx.set(Some(last_idx));
                        }
                    } else {
                        dropdown_selected_idx.set(Some(last_idx));
                    }
                    show_dropdown.set(true);
                }
            } else {
                if e.key() == "Escape" && (*dropdown_selected_idx).is_none() {
                    show_dropdown.set(false);
                } else {
                    dropdown_selected_idx.set(None);
                    show_dropdown.set(true);
                }
            }
        })
    };

    let on_match_click = {
        let show_dropdown = show_dropdown.clone();
        let dropdown_closed_on_select = dropdown_closed_on_select.clone();
        let dropdown_selected_idx = dropdown_selected_idx.clone();
        let check_alias = check_alias.clone();
        Callback::from(move |alias: String| {
            show_dropdown.set(false);
            dropdown_closed_on_select.set(true);
            dropdown_selected_idx.set(None);
            check_alias.emit(alias.clone());
        })
    };

    let on_match_hover = {
        let dropdown_selected_idx = dropdown_selected_idx.clone();
        Callback::from(move |maybe_idx: Option<usize>| {
            if let Some(idx) = maybe_idx {
                dropdown_selected_idx.set(Some(idx));
            } else {
                dropdown_selected_idx.set(None);
            }
        })
    };

    html! {
        <div class="element">
            <input type="text"
                id="alias-input"
                value={props.text.clone()}
                oninput={on_input}
                onfocus={on_focus}
                onblur={on_blur}
                onkeydown={on_key_down}
                ref={input_ref}
                placeholder="Enter alias..."
                inputmode={if props.keyboard { "none" } else { "text" }}
            />
            <AliasDropdown
                show={*show_dropdown}
                selected={*dropdown_selected_idx}
                matched_aliases={(*alias_matches).clone()}
                on_click={on_match_click}
                on_hover={on_match_hover}
            />
        </div>
    }
}
