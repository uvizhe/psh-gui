use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct AliasInputProps {
    pub clear: bool,
    pub known_aliases: Vec<String>,
    pub keyboard: bool,
    pub on_input: Callback<(String, bool)>,
    pub on_focus: Callback<NodeRef>,
    pub on_enter: Callback<()>,
}

#[function_component(AliasInput)]
pub fn alias_input(props: &AliasInputProps) -> Html {
    let input_ref = use_node_ref();

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
    {
        let clear = props.clear.clone();
        let clear2 = props.clear.clone();
        let input_ref = input_ref.clone();
        use_effect_with_deps(
            move |_| {
                if clear {
                    let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    input.set_value("");
                }
            },
            clear2,
        );
    }

    let check_alias = {
        let input_ref = input_ref.clone();
        let known_aliases = props.known_aliases.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let alias = input.value();
            let known = known_aliases.contains(&alias);
            on_input.emit((alias, known));
        })
    };

    let on_focus = {
        let input_ref = input_ref.clone();
        let on_focus = props.on_focus.clone();
        Callback::from(move |_| {
            on_focus.emit(input_ref.clone());
        })
    };

    let on_key_down = {
        let on_enter = props.on_enter.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                on_enter.emit(());
            }
        })
    };

    html! {
        <div class="element">
            <input type="text"
                id="alias-input"
                oninput={check_alias}
                onfocus={on_focus}
                onkeydown={on_key_down}
                list="aliases"
                ref={input_ref}
                placeholder="Enter alias..."
                inputmode={if props.keyboard { "none" } else { "text" }}
            />
            <datalist id="aliases">
                {
                    props.known_aliases.iter().map(|alias| {
                        html!{<option key={alias.clone()} value={alias.clone()}/>}
                    }).collect::<Html>()
                }
            </datalist>
        </div>
    }
}
