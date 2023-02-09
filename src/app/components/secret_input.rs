use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SecretInputProps {
    pub clear: bool,
    #[prop_or_default]
    pub disabled: bool,
    pub hint: String,
    pub on_input: Callback<String>,
}

#[function_component(SecretInput)]
pub fn secret_input(props: &SecretInputProps) -> Html {
    let input_ref = use_node_ref();

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

    let on_input = {
        let input_ref = input_ref.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |_| {
            let input = input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
            let secret = input.value();
            on_input.emit(secret);
        })
    };

    html! {
        <div class="element">
            <input type="password"
                id="secret-input"
                oninput={on_input}
                ref={input_ref}
                placeholder={props.hint.clone()}
                disabled={props.disabled}
            />
        </div>
    }
}
