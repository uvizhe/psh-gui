use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SecretInputProps {
    #[prop_or_default]
    pub clear: bool,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub focus: bool,
    pub id: String,
    pub hint: String,
    pub keyboard: bool,
    pub on_input: Callback<String>,
    pub on_focus: Callback<NodeRef>,
}

#[function_component(SecretInput)]
pub fn secret_input(props: &SecretInputProps) -> Html {
    let input_ref = use_node_ref();

    {
        let input_ref = input_ref.clone();
        let focus = props.focus.clone();
        use_effect_with_deps(
            move |_| {
                if focus {
                    let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    input.focus().unwrap();
                }
            },
            (),
        );
    }
    {
        let clear = props.clear.clone();
        let clear2 = props.clear.clone();
        let input_ref = input_ref.clone();
        let focus = props.focus.clone();
        use_effect_with_deps(
            move |_| {
                if clear {
                    let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    input.set_value("");
                    if focus {
                        input.focus().unwrap();
                    }
                }
            },
            clear2,
        );
    }

    let on_input = {
        let input_ref = input_ref.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let secret = input.value();
            on_input.emit(secret);
        })
    };

    let on_focus = {
        let input_ref = input_ref.clone();
        let on_focus = props.on_focus.clone();
        Callback::from(move |_| {
            on_focus.emit(input_ref.clone());
        })
    };

    html! {
        <div class="element">
            <input type="password"
                id={props.id.clone()}
                name={props.id.clone()}
                key={props.id.clone()}
                oninput={on_input}
                onfocus={on_focus}
                ref={input_ref}
                placeholder={props.hint.clone()}
                disabled={props.disabled}
                inputmode={if props.keyboard { "none" } else { "text" }}
            />
        </div>
    }
}
