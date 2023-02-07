use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use psh::{CharSet, Psh, PshWebDb, ZeroizingString};

mod components;

use components::alias_input::AliasInput;
use components::secret_input::SecretInput;
use components::triswitch::Triswitch;

const MP: &str = "password";

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(Copy, Clone, Debug, PartialEq)]
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
    let alias_handle_user_choice = use_state(|| AliasHandle::Store);
    let use_secret = use_state(|| true);
    let secret = use_state(|| String::new());
    let charset = use_state(|| CharSet::Standard);
    let charset_user_choice = use_state(|| CharSet::Standard);
    let known_alias = use_state(|| false);
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
        let alias_handle = alias_handle.clone();
        let alias_handle_user_choice = alias_handle_user_choice.clone();
        let charset = charset.clone();
        let charset_user_choice = charset_user_choice.clone();
        let password_msg = password_msg.clone();
        Callback::from(move |(input, known): (String, bool)| {
            password_msg.set("".to_string());
            alias.set(input.clone());
            known_alias.set(known);
            if known {
                let alias = ZeroizingString::new(input);
                use_secret.set(psh.borrow().get().unwrap().alias_uses_secret(&alias));
                alias_handle.set(AliasHandle::Store);
                charset.set(psh.borrow().get().unwrap().get_charset(&alias));
            }
            else {
                use_secret.set(true);
                if *alias_handle_user_choice == AliasHandle::Remove {
                    alias_handle.set(AliasHandle::Store);
                } else {
                    alias_handle.set(*alias_handle_user_choice);
                }
                charset.set(*charset_user_choice);
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
        let alias_handle_user_choice = alias_handle_user_choice.clone();
        Callback::from(move |value: String| {
            match value.as_str() {
                "0" => {
                    alias_handle.set(AliasHandle::Store);
                    alias_handle_user_choice.set(AliasHandle::Store);
                }
                "1" => {
                    alias_handle.set(AliasHandle::Ignore);
                    alias_handle_user_choice.set(AliasHandle::Ignore);
                }
                "2" => {
                    alias_handle.set(AliasHandle::Remove);
                    alias_handle_user_choice.set(AliasHandle::Remove);
                }
                _ => unreachable!()
            }
        })
    };
    let set_charset = {
        let charset = charset.clone();
        let charset_user_choice = charset_user_choice.clone();
        Callback::from(move |value: String| {
            match value.as_str() {
                "0" => {
                    charset.set(CharSet::Standard);
                    charset_user_choice.set(CharSet::Standard);
                }
                "1" => {
                    charset.set(CharSet::RequireAll);
                    charset_user_choice.set(CharSet::RequireAll);
                }
                "2" => {
                    charset.set(CharSet::Reduced);
                    charset_user_choice.set(CharSet::Reduced);
                }
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
        let alias_handle_user_choice = alias_handle_user_choice.clone();
        let secret = secret.clone();
        let charset = charset.clone();
        let charset_user_choice = charset_user_choice.clone();
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
                alias_handle_user_choice.set(AliasHandle::Store);
                charset.set(CharSet::Standard);
                charset_user_choice.set(CharSet::Standard);
            }
        })
    };
    let match_alias_handle = {
        match *alias_handle {
            AliasHandle::Store => 0,
            AliasHandle::Ignore => 1,
            AliasHandle::Remove => 2,
        }
    };
    let match_charset = {
        match *charset {
            CharSet::Standard => 0,
            CharSet::RequireAll => 1,
            CharSet::Reduced => 2,
        }
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
            <Triswitch
                checked={match_alias_handle}
                disabled={vec![false, *known_alias, !*known_alias]}
                name="alias_handle"
                title="How to handle alias"
                labels={vec![
                    "Store".to_string(),
                    "Don't store".to_string(),
                    "Remove".to_string()]}
                on_switch={set_alias_handle.clone()}
            />
            <Triswitch
                checked={match_charset}
                disabled={vec![*known_alias, *known_alias, *known_alias]}
                name="charset"
                title="Character set to use"
                labels={vec![
                    "Standard".to_string(),
                    "Require All".to_string(),
                    "Reduced".to_string()]}
                on_switch={set_charset.clone()}
            />
            <div class="element">
                <div class="row">
                    <p><b>{ &*password_msg }</b></p>
                </div>
            </div>
        </main>
    }
}
