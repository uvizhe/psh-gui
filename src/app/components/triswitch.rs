use wasm_bindgen::JsCast;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct TriswitchProps {
    pub checked: Option<usize>,
    pub disabled: Vec<bool>,
    pub visible: bool,
    pub name: String,
    pub title: String,
    pub labels: Vec<String>,
    pub on_switch: Callback<String>,
}

#[function_component(Triswitch)]
pub fn triswitch(props: &TriswitchProps) -> Html {
    let triswitch_refs = [use_node_ref(), use_node_ref(), use_node_ref()];

    {
        let checked = props.checked.unwrap_or(0);
        let triswitch_refs = triswitch_refs.clone();
        use_effect_with_deps(
            move |_| {
                let option_div = triswitch_refs[checked].clone();
                let option = option_div.get().unwrap()
                    .first_child().expect("input doesn't exist")
                    .dyn_into::<web_sys::HtmlInputElement>().unwrap();
                option.set_checked(true);
            },
            checked,
        );
    }

    let on_click = {
        let on_switch = props.on_switch.clone();
        Callback::from(move |e: MouseEvent| {
            let radio = {
                // If a click was on wrapper element
                if let Some(wrapper) = e.target_dyn_into::<web_sys::HtmlDivElement>() {
                    wrapper.first_child().unwrap()
                        .dyn_into::<web_sys::HtmlInputElement>().unwrap()
                }
                // If it was a keyboard navigation
                else {
                    e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap()
                }
            };
            if !radio.disabled() {
                let value = radio.value();
                on_switch.emit(value);
            }
        })
    };

    let labels = props.labels.clone();
    let disabled = props.disabled.clone();
    let maybe_hidden = if props.visible { None } else { Some("hidden") };

    html! {
        <fieldset class={classes!("full-width", maybe_hidden)}>
            <legend>{props.title.clone()}</legend>
            <div class="switch-wrapper">
            {
                for (0..3).map(|i| {
                    html! {
                        <div class="switch"
                            ref={triswitch_refs[i].clone()}
                            onclick={on_click.clone()}
                        >
                            <input type="radio"
                                id={i.to_string()}
                                name={props.name.clone()}
                                value={i.to_string()}
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
