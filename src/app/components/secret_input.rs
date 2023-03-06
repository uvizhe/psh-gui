use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct SecretInputProps {
    pub text: String,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub focus: bool,
    pub id: String,
    pub hint: String,
    pub keyboard: bool,
    pub on_input: Callback<String>,
    pub on_focus: Callback<NodeRef>,
    pub on_enter: Callback<()>,
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
            <input type="password"
                value={props.text.clone()}
                id={props.id.clone()}
                name={props.id.clone()}
                key={props.id.clone()}
                oninput={on_input}
                onfocus={on_focus}
                onkeydown={on_key_down}
                ref={input_ref}
                placeholder={props.hint.clone()}
                disabled={props.disabled}
                inputmode={if props.keyboard { "none" } else { "text" }}
            />
        </div>
    }
}
