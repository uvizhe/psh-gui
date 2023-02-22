use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TriswitchProps {
    pub checked: Option<u8>,
    pub disabled: Vec<bool>,
    pub visible: bool,
    pub name: String,
    pub title: String,
    pub labels: Vec<String>,
    pub on_switch: Callback<String>,
}

#[function_component(Triswitch)]
pub fn triswitch(props: &TriswitchProps) -> Html {
    let triswitch_ref = use_node_ref();

    {
        let checked = props.checked.unwrap_or(0);
        let triswitch_ref = triswitch_ref.clone();
        use_effect_with_deps(
            move |_| {
                let option;
                if checked < 2 {
                    let first_option_wrapper = triswitch_ref.get().unwrap()
                        .first_child().expect("div wrapped for input doesn't exist");
                    if checked < 1 {
                        option = first_option_wrapper
                            .first_child().expect("input doesn't exist")
                            .dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    } else {
                        option = first_option_wrapper
                            .next_sibling().expect("div wrapped for input doesn't exist")
                            .first_child().expect("input doesn't exist")
                            .dyn_into::<web_sys::HtmlInputElement>().unwrap();
                    }
                } else {
                    option = triswitch_ref.get().unwrap()
                        .last_child().expect("div wrapped for input doesn't exist")
                        .first_child().expect("input doesn't exist")
                        .dyn_into::<web_sys::HtmlInputElement>().unwrap();
                }
                option.set_checked(true);
            },
            checked,
        );
    }

    let on_switch = {
        let on_switch = props.on_switch.clone();
        Callback::from(move |e: Event| {
            let radio = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap();
            let value = radio.value();
            on_switch.emit(value);
        })
    };

    let labels = props.labels.clone();
    let disabled = props.disabled.clone();
    let maybe_hidden = if props.visible { None } else { Some("hidden") };

    html! {
        <fieldset class={classes!("full-width", maybe_hidden)}>
            <legend>{props.title.clone()}</legend>
            <div class="switch-wrapper" ref={triswitch_ref}>
            {
                for (0..3).map(|i| {
                    html! {
                        <div class="switch">
                            <input type="radio"
                                id={i.to_string()}
                                name={props.name.clone()}
                                value={i.to_string()}
                                onchange={on_switch.clone()}
                                disabled={disabled[i]}
                            />
                            <label for={i.to_string()}>{&labels[i]}</label>
                        </div>
                    }
                })
            }
            </div>
        </fieldset>
    }
}
