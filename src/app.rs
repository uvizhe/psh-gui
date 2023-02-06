use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use psh::{CharSet, Psh, PshWebDb, ZeroizingString};

mod components;

use components::alias_input::AliasInput;
use components::secret_input::SecretInput;

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
    let psh = use_mut_ref(|| OnceCell::<Psh>::new());
    let known_aliases = use_state(|| Vec::<String>::new());
    let alias = use_state(|| String::new());
    let alias_handle = use_state(|| AliasHandle::Store);
    let use_secret = use_state(|| true);
    let secret = use_state(|| String::new());
    let charset = use_state(|| CharSet::Standard);
    let known_alias = use_state(|| false);
    //let password = use_state(|| ZeroizingString::new("".to_string()));
    let password_msg = use_state(|| String::new());

    {
        let psh = psh.clone();
        let known_aliases = known_aliases.clone();
        use_effect_with_deps(
            move |_| {
                let res = psh.borrow_mut().set(
                    Psh::new(
                        ZeroizingString::new(MP.to_string()),
                        PshWebDb::new()
                    ).unwrap()
                );
                if res.is_ok() {
                    known_aliases.set(collect_aliases(psh.borrow().get().unwrap()));
                } else {
                    log("Failed to initialize Psh");
                }
            },
            (),
        );
    }

    let on_alias_input: Callback<(String, bool)> = {
        let psh = psh.clone();
        let alias = alias.clone();
        let known_alias = known_alias.clone();
        let use_secret = use_secret.clone();
        let charset = charset.clone();
        let alias_handle = alias_handle.clone();
        let password_msg = password_msg.clone();
        Callback::from(move |(input, known): (String, bool)| {
            password_msg.set("".to_string());
            alias.set(input.clone());
            known_alias.set(known);
            if known {
                let alias = ZeroizingString::new(input);
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
    let on_secret_input: Callback<String> = {
        let secret = secret.clone();
        Callback::from(move |input: String| {
            secret.set(input.clone());
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
        let password_msg = password_msg.clone();
        let psh = psh.clone();
        let alias = alias.clone();
        let known_aliases = known_aliases.clone();
        let known_alias = known_alias.clone();
        let alias_handle = alias_handle.clone();
        let secret = secret.clone();
        let charset = charset.clone();
        Callback::from(move |_| {
            let alias_string = alias.trim().to_string();
            if !alias_string.is_empty() {
                let mut psh = psh.borrow_mut();
                let psh = psh.get_mut().unwrap();
                let secret_string =
                    if secret.to_string().is_empty() {
                        None
                    } else {
                        Some(ZeroizingString::new(secret.to_string()))
                    };
                let use_secret = secret_string.is_some();
                if *alias_handle != AliasHandle::Remove {
                    let pass = psh.derive_password(
                        &ZeroizingString::new(alias_string.clone()),
                        secret_string,
                        Some(*charset),
                    );
                    password_msg.set(pass.to_string());
                    if !known_aliases.contains(&alias_string) && *alias_handle == AliasHandle::Store {
                        let res = psh.append_alias_to_db(
                            &ZeroizingString::new(alias_string.clone()),
                            Some(use_secret),
                            Some(*charset),
                        );
                        if res.is_ok() {
                            known_aliases.set(collect_aliases(psh));
                        } else {
                            log("Failed to save alias");
                        }
                    }
                } else {
                    let res = psh.remove_alias_from_db(&ZeroizingString::new(alias_string.clone()));
                    if res.is_ok() {
                        alias.set(String::new());
                        password_msg.set(String::new());
                        known_aliases.set(collect_aliases(psh));
                    } else {
                        log("Failed to remove alias");
                    }
                }
                alias.set("".to_string());
                secret.set("".to_string());
                known_alias.set(false);
                alias_handle.set(AliasHandle::Store);
            }
        })
    };

    let known_aliases = (*known_aliases).clone();
    let password_msg = (*password_msg).clone();

    html! {
        <main class="container">
            <AliasInput
                clear={!password_msg.is_empty()}
                {known_aliases}
                on_input={on_alias_input.clone()}
            />
            <SecretInput
                clear={!password_msg.is_empty()}
                disabled={!*use_secret}
                on_input={on_secret_input.clone()}
            />
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
                    <p><b>{ &*password_msg }</b></p>
                </div>
            </div>
        </main>
    }
}
