use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KbSlot {
    Pair(&'static str, &'static str),
    Sole(&'static str),
    Space,
    Shift,
    Backspace,
    Alt,
}

const KEYBOARD_LAYOUT: [[KbSlot; 10]; 4] = [
    [
        KbSlot::Pair("1", "!"), KbSlot::Pair("2", "@"), KbSlot::Pair("3", "#"),
        KbSlot::Pair("4", "$"), KbSlot::Pair("5", "%"), KbSlot::Pair("6", "^"),
        KbSlot::Pair("7", "&"), KbSlot::Pair("8", "*"), KbSlot::Pair("9", "("),
        KbSlot::Pair("0", ")"),
    ],
    [
        KbSlot::Sole("q"), KbSlot::Sole("w"), KbSlot::Sole("e"),
        KbSlot::Pair("r", "="), KbSlot::Pair("t", "-"), KbSlot::Pair("y", "+"),
        KbSlot::Pair("u", "{"), KbSlot::Pair("i", "}"), KbSlot::Pair("o", "["),
        KbSlot::Pair("p", "]"),
    ],
    [
        KbSlot::Pair("a", "~"), KbSlot::Pair("s", "`"), KbSlot::Sole("d"),
        KbSlot::Pair("f", "_"), KbSlot::Pair("g", "/"), KbSlot::Pair("h", "|"),
        KbSlot::Pair("j", "\\"), KbSlot::Pair("k", "<"), KbSlot::Pair("l", ">"),
        KbSlot::Shift,
    ],
    [
        KbSlot::Pair("z", "\""), KbSlot::Pair("x", "'"), KbSlot::Pair("c", ":"),
        KbSlot::Pair("v", ";"), KbSlot::Pair("b", ","), KbSlot::Pair("n", "."),
        KbSlot::Pair("m", "?"), KbSlot::Space, KbSlot::Alt,
        KbSlot::Backspace,
    ],
];

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub visible: bool,
    pub on_input: Callback<String>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let shift_is_pressed = use_state(|| false);
    let alt_is_pressed = use_state(|| false);

    let on_kb_input = {
        let shift_is_pressed = shift_is_pressed.clone();
        let alt_is_pressed = alt_is_pressed.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |key: KbSlot| {
            match key {
                KbSlot::Pair(a, b) => {
                    if *shift_is_pressed {
                        on_input.emit(a.to_uppercase());
                        shift_is_pressed.set(false);
                    } else if *alt_is_pressed {
                        on_input.emit(b.to_string());
                        alt_is_pressed.set(false);
                    } else {
                        on_input.emit(a.to_string());
                    }
                }
                KbSlot::Sole(a) => {
                    if *shift_is_pressed {
                        on_input.emit(a.to_uppercase());
                        shift_is_pressed.set(false);
                    } else if *alt_is_pressed { // TODO: disable keys that can't produce value
                        alt_is_pressed.set(false);
                    } else {
                        on_input.emit(a.to_string());
                    }
                }
                KbSlot::Space => on_input.emit(" ".to_string()),
                KbSlot::Shift => {
                    shift_is_pressed.set(!*shift_is_pressed);
                    alt_is_pressed.set(false);
                }
                KbSlot::Alt => {
                    alt_is_pressed.set(!*alt_is_pressed);
                    shift_is_pressed.set(false);
                }
                KbSlot::Backspace => {
                    shift_is_pressed.set(false);
                    alt_is_pressed.set(false);
                    on_input.emit("".to_string());
                }
            }
        })
    };

    let on_kb_click = {
        // Prevent stealing focus from input
        Callback::from(move |e: MouseEvent| {
            e.prevent_default();
        })
    };

    let maybe_hidden = if props.visible { None } else { Some("hidden") };

    html! {
        <div class={classes!("keyboard", maybe_hidden)} onmousedown={on_kb_click}>
        {
            KEYBOARD_LAYOUT.iter().map(|row| {
                html! {
                    <div class="kbrow">
                    {
                        row.iter().map(|slot| {
                            html!{
                                <KeyboardKey
                                    kbkey={*slot}
                                    upper={*shift_is_pressed}
                                    alt={*alt_is_pressed}
                                    on_click={on_kb_input.clone()}
                                />
                            }
                        }).collect::<Html>()
                    }
                    </div>
                }
            }).collect::<Html>()
        }
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct KeyboardKeyProps {
    pub kbkey: KbSlot,
    pub upper: bool,
    pub alt: bool,
    pub on_click: Callback<KbSlot>,
}

#[function_component(KeyboardKey)]
pub fn keyboard_key(props: &KeyboardKeyProps) -> Html {
    let on_kb_click = {
        let kbkey = props.kbkey.clone();
        let on_click = props.on_click.clone();
        Callback::from(move |e: MouseEvent| {
            // Prevent stealing focus from input
            e.prevent_default();

            on_click.emit(kbkey);
        })
    };

    match props.kbkey {
        KbSlot::Pair(a, b) => html! {
            <div class="keyb" onmousedown={on_kb_click}>
                <div
                    class={classes!(
                        if props.upper { Some("keyb-upper") } else { None },
                        if props.alt { Some("hidden") } else { Some("keyb-prim") }
                    )}
                >
                    {a}
                </div>
                <div
                    class={classes!(
                        "keyb-alt",
                        if props.alt { Some("keyb-prim") } else { None }
                    )}
                >
                    {b}
                </div>
            </div>
        },
        KbSlot::Sole(a) => html! {
            <div class="keyb" onmousedown={on_kb_click}>
                <div
                    class={classes!(
                        "keyb-prim",
                        if props.upper { Some("keyb-upper") } else { None },
                        if props.alt { Some("hidden") } else { None }
                    )}
                >
                    {a}
                </div>
            </div>
        },
        KbSlot::Space => html! {<div class="keyb space" onmousedown={on_kb_click}>{"⎵"}</div>},
        KbSlot::Shift => html! {<div class="keyb shift" onmousedown={on_kb_click}></div>
        },
        KbSlot::Backspace => html! {<div class="keyb backspace" onmousedown={on_kb_click}>{"⌫"}</div>},
        KbSlot::Alt => html! {<div class="keyb alt" onmousedown={on_kb_click}>{"Fn"}</div>},
    }
}
