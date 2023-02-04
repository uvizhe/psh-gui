use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use psh::{CharSet, Psh, PshWebDb, ZeroizingString};

const MP: &str = "password";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Debug, PartialEq)]
enum AliasHandle {
    Store,
    Ignore,
    Remove,
}

fn collect_aliases(psh: &Psh) -> Vec<String> {
    psh.aliases()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

#[function_component(App)]
pub fn app() -> Html {
    let alias_input_ref = use_node_ref();
    let secret_input_ref = use_node_ref();

    let psh = use_mut_ref(|| OnceCell::<Psh>::new());
    let aliases = use_state(|| Vec::<String>::new());
    let alias = use_state(|| String::new());
    let alias_handle = use_state(|| AliasHandle::Store);
    let use_secret = use_state(|| true);
    let secret = use_state(|| String::new());
    let charset = use_state(|| CharSet::Standard);
    let known_alias = use_state(|| false);
    //let password = use_state(|| ZeroizingString::new("".to_string()));

    let password_msg = use_state(|| String::new());
    {
        let psh_ = psh.clone();
        let aliases_ = aliases.clone();
        use_effect_with_deps(
            move |_| {
                let res = psh_.borrow_mut().set(
                    Psh::new(
                        ZeroizingString::new(MP.to_string()),
                        PshWebDb::new()
                    ).unwrap()
                );
                if res.is_ok() {
                    aliases_.set(collect_aliases(psh_.borrow().get().unwrap()));
                } else {
                    log("Failed to initialize Psh");
                }
            },
            (),
        );
        let password_msg = password_msg.clone();
        let psh = psh.clone();
        let alias = alias.clone();
        let alias2 = alias.clone();
        let aliases = aliases.clone();
        let known_alias = known_alias.clone();
        let alias_handle = alias_handle.clone();
        let secret = secret.clone();
        let charset = charset.clone();
        let alias_input_ref = alias_input_ref.clone();
        let secret_input_ref = secret_input_ref.clone();
        use_effect_with_deps(
            move |_| {
                let alias_string = alias.trim().to_string();
                if !alias_string.is_empty() {
                    let mut psh = psh.borrow_mut();
                    let psh = psh.get_mut().unwrap();
                    let secret =
                        if secret.to_string().is_empty() {
                            None
                        } else {
                            Some(ZeroizingString::new(secret.to_string()))
                        };
                    let use_secret = secret.is_some();
                    if *alias_handle != AliasHandle::Remove {
                        let pass = psh.derive_password(
                            &ZeroizingString::new(alias_string.clone()),
                            secret,
                            Some(*charset),
                        );
                        password_msg.set(pass.to_string());
                        if !aliases.contains(&alias_string) && *alias_handle == AliasHandle::Store {
                            let res = psh.append_alias_to_db(
                                &ZeroizingString::new(alias_string.clone()),
                                Some(use_secret),
                                Some(*charset),
                            );
                            if res.is_ok() {
                                aliases.set(collect_aliases(psh));
                            } else {
                                log("Failed to save alias");
                            }
                        }
                    } else {
                        let res = psh.remove_alias_from_db(&ZeroizingString::new(alias_string.clone()));
                        if res.is_ok() {
                            alias.set(String::new());
                            password_msg.set(String::new());
                            aliases.set(collect_aliases(psh));
                        } else {
                            log("Failed to remove alias");
                        }
                    }
                    let alias_input = alias_input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    alias_input.set_value("");
                    let secret_input = secret_input_ref.cast::<web_sys::HtmlInputElement>().unwrap();
                    secret_input.set_value("");
                    known_alias.set(false);
                    alias_handle.set(AliasHandle::Store);
                }
            },
            alias2,
        );
    }

    let check_alias = {
        let psh = psh.clone();
        let aliases = aliases.clone();
        let known_alias = known_alias.clone();
        let use_secret = use_secret.clone();
        let charset = charset.clone();
        let alias_handle = alias_handle.clone();
        let alias_input_ref = alias_input_ref.clone();
        Callback::from(move |_| {
            let alias = alias_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value();
            let is_saved_alias = aliases.contains(&alias);
            known_alias.set(is_saved_alias);
            if is_saved_alias {
                let alias = ZeroizingString::new(alias);
                use_secret.set(psh.borrow().get().unwrap().alias_uses_secret(&alias));
                charset.set(psh.borrow().get().unwrap().get_charset(&alias));
            }
            else {
                use_secret.set(true);
                if *alias_handle == AliasHandle::Remove {
                    alias_handle.set(AliasHandle::Store);
                }
            }
        })
    };
    let set_alias_handle = {
        let alias_handle = alias_handle.clone();
        Callback::from(move |e: Event| {
            let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
            match value.as_str() {
                "Store" => alias_handle.set(AliasHandle::Store),
                "Ignore" => alias_handle.set(AliasHandle::Ignore),
                "Remove" => alias_handle.set(AliasHandle::Remove),
                _ => unreachable!()
            }
        })
    };
    let set_charset = {
        let charset = charset.clone();
        Callback::from(move |e: Event| {
            let value = e.target_dyn_into::<web_sys::HtmlInputElement>().unwrap().value();
            match value.as_str() {
                "Standard" => charset.set(CharSet::Standard),
                "All" => charset.set(CharSet::RequireAll),
                "Reduced" => charset.set(CharSet::Reduced),
                _ => unreachable!()
            }
        })
    };
    let process = {
        let alias = alias.clone();
        let alias_input_ref = alias_input_ref.clone();
        let secret_input_ref = secret_input_ref.clone();
        Callback::from(move |_| {
            alias.set(alias_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
            secret.set(secret_input_ref.cast::<web_sys::HtmlInputElement>().unwrap().value());
        })
    };

    html! {
        <main class="container">
            <div class="element">
                <input type="text"
                    id="alias-input"
                    oninput={check_alias}
                    list="aliases"
                    ref={alias_input_ref}
                    placeholder="Enter alias..."
                />
                <datalist id="aliases">
                    {
                        aliases.iter().map(|alias| {
                            html!{<option key={alias.clone()} value={alias.clone()}/>}
                        }).collect::<Html>()
                    }
                </datalist>
            </div>
            <div class="element">
                <input type="password"
                    id="secret-input"
                    ref={secret_input_ref}
                    placeholder="Enter secret..."
                    disabled={!*use_secret}
                />
            </div>
            <div class="element">
                <button type="button" onclick={process}>
                    { if *alias_handle != AliasHandle::Remove {"Get password"}
                        else {"Remove alias"} }
                </button>
            </div>
            <fieldset class="full-width">
                <legend>{"How to handle alias"}</legend>
                <div class="switch-wrapper">
                    <div class="switch">
                        <input type="radio"
                            id="store"
                            name="alias-handle"
                            value="Store"
                            onchange={set_alias_handle.clone()}
                            checked={*alias_handle == AliasHandle::Store}
                        />
                        <label for="store">{"Store"}</label>
                    </div>
                    <div class="switch">
                        <input type="radio"
                            id="ignore"
                            name="alias-handle"
                            value="Ignore"
                            onchange={set_alias_handle.clone()}
                            checked={*alias_handle == AliasHandle::Ignore}
                            disabled={*known_alias}
                        />
                        <label for="ignore">{"Don't store"}</label>
                    </div>
                    <div class="switch">
                        <input type="radio"
                            id="remove"
                            name="alias-handle"
                            value="Remove"
                            onchange={set_alias_handle}
                            checked={*alias_handle == AliasHandle::Remove}
                            disabled={!*known_alias}
                        />
                        <label for="remove">{"Remove"}</label>
                    </div>
                </div>
            </fieldset>
            <fieldset class="full-width" disabled={*known_alias}>
                <legend>{"Character set to use"}</legend>
                <div class="switch-wrapper">
                    <div class="switch">
                        <input type="radio"
                            id="standard"
                            name="charset"
                            value="Standard"
                            onchange={set_charset.clone()}
                            checked={*charset == CharSet::Standard}
                        />
                        <label for="standard">{"Standard"}</label>
                    </div>
                    <div class="switch">
                        <input type="radio"
                            id="all"
                            name="charset"
                            value="All"
                            onchange={set_charset.clone()}
                            checked={*charset == CharSet::RequireAll}
                        />
                        <label for="all">{"All"}</label>
                    </div>
                    <div class="switch">
                        <input type="radio"
                            id="reduced"
                            name="charset"
                            value="Reduced"
                            onchange={set_charset}
                            checked={*charset == CharSet::Reduced}
                        />
                        <label for="reduced">{"Reduced"}</label>
                    </div>
                </div>
            </fieldset>
            <div class="element">
                <div class="row">
                    <p><b>{ &*alias }</b></p>
                    <p><b>{ &*password_msg }</b></p>
                </div>
            </div>
        </main>
    }
}
