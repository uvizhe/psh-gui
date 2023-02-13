use gloo_timers::callback::Timeout;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KbSlot {
    Pair(&'static str, &'static str),
    Space,
    Empty,
}

const KEYBOARD_LAYOUT: [[KbSlot; 10]; 5] = [
    [
    KbSlot::Pair("`", "~"), KbSlot::Pair("'", "\""), KbSlot::Pair("-", "_"), KbSlot::Pair("=", "+"),
    KbSlot::Pair("[", "{"), KbSlot::Pair("]", "}"), KbSlot::Pair("\\", "|"), KbSlot::Space,
    KbSlot::Empty, KbSlot::Empty,
    ],
    [
    KbSlot::Pair("1", "!"), KbSlot::Pair("2", "@"), KbSlot::Pair("3", "#"), KbSlot::Pair("4", "$"),
    KbSlot::Pair("5", "%"), KbSlot::Pair("6", "^"), KbSlot::Pair("7", "&"), KbSlot::Pair("8", "*"),
    KbSlot::Pair("9", "("), KbSlot::Pair("0", ")"),
    ],
    [
    KbSlot::Pair("q", "Q"), KbSlot::Pair("w", "W"), KbSlot::Pair("e", "E"), KbSlot::Pair("r", "R"),
    KbSlot::Pair("t", "T"), KbSlot::Pair("y", "Y"), KbSlot::Pair("u", "U"), KbSlot::Pair("i", "I"),
    KbSlot::Pair("o", "O"), KbSlot::Pair("p", "P"),
    ],
    [
    KbSlot::Pair("a", "A"), KbSlot::Pair("s", "S"), KbSlot::Pair("d", "D"), KbSlot::Pair("f", "F"),
    KbSlot::Pair("g", "G"), KbSlot::Pair("h", "H"), KbSlot::Pair("j", "J"), KbSlot::Pair("k", "K"),
    KbSlot::Pair("l", "L"), KbSlot::Pair(";", ":"),
    ],
    [
    KbSlot::Pair("z", "Z"), KbSlot::Pair("x", "X"), KbSlot::Pair("c", "C"), KbSlot::Pair("v", "V"),
    KbSlot::Pair("b", "B"), KbSlot::Pair("n", "N"), KbSlot::Pair("m", "M"), KbSlot::Pair(",", "<"),
    KbSlot::Pair(".", ">"), KbSlot::Pair("/", "?"),
    ],
];

#[derive(Properties, PartialEq)]
pub struct KeyboardProps {
    pub on_input: Callback<String>,
}

#[function_component(Keyboard)]
pub fn keyboard(props: &KeyboardProps) -> Html {
    let temp_value = use_mut_ref(|| (KbSlot::Empty, false));
    let timeout = use_mut_ref(|| None::<Timeout>);

    let on_kb_input = {
        let temp_value = temp_value.clone();
        let timeout = timeout.clone();
        let on_input = props.on_input.clone();
        Callback::from(move |key: KbSlot| {
            let temp_value2 = temp_value.clone();
            let on_input2 = on_input.clone();
            let current_key = temp_value.borrow().0.clone();
            let current_is_primary = temp_value.borrow().1;
            // If new keyboard key is pressed
            if key != current_key {
                let emitted_value = match current_key {
                    KbSlot::Pair(a, b) => {
                        if current_is_primary { a }
                        else { b }
                    }
                    KbSlot::Space => " ",
                    KbSlot::Empty => "",
                };
                *temp_value.borrow_mut() = (key, true);
                if current_key != KbSlot::Empty {
                    on_input.emit(emitted_value.to_string());
                }
            }
            // If the same keyboard key is pressed
            else {
                *temp_value.borrow_mut() = (key, !current_is_primary);
            }
            // (Re)set timeout to emit current (last) key
            let current_key = temp_value.borrow().0.clone();
            let current_is_primary = temp_value.borrow().1;
            if timeout.borrow().is_some() {
                let timeout = timeout.borrow_mut().take().unwrap();
                timeout.cancel();
            }
            if current_key == KbSlot::Space {
                on_input.emit(" ".to_string());
                *temp_value.borrow_mut() = (KbSlot::Empty, false);
            } else {
                *timeout.borrow_mut() = Some(Timeout::new(1_000, move || {
                    let emitted_value = match current_key {
                        KbSlot::Pair(a, b) => {
                            if current_is_primary { a }
                            else { b }
                        }
                        KbSlot::Space | KbSlot::Empty => unreachable!(),
                    };
                    on_input2.emit(emitted_value.to_string());
                    // XXX: Potential race condition
                    *temp_value2.borrow_mut() = (KbSlot::Empty, false);
                }));
            }
        })
    };

    html! {
        <div class="element">
        {
            KEYBOARD_LAYOUT.iter().map(|row| {
                html! {
                    <div class="row">
                    {
                        row.iter().map(|slot| {
                            html!{<KeyboardKey kbkey={*slot} on_click={on_kb_input.clone()} />}
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
    pub on_click: Callback<KbSlot>,
}

#[function_component(KeyboardKey)]
pub fn keyboard_key(props: &KeyboardKeyProps) -> Html {
    let on_kb_click = {
        let kbkey = props.kbkey.clone();
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            on_click.emit(kbkey);
        })
    };

    match props.kbkey {
        KbSlot::Pair(a, b) =>  html! {<div class="keyb" onclick={on_kb_click}>{a}{b}</div>},
        KbSlot::Space =>  html! {<div class="keyb" onclick={on_kb_click}>{"⎵"}</div>},
        KbSlot::Empty => html! {<div class="keyb hidden">{"&nbsp;"}</div>},
    }
}
