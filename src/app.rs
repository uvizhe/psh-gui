use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use psh::{CharSet, Psh, PshStore, PshWebDb, ZeroizingString};

mod components;

use components::alias_input::AliasInput;
use components::secret_input::SecretInput;
use components::triswitch::Triswitch;

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
    // Psh instance
    let psh = use_mut_ref(|| OnceCell::<Psh>::new());
    // If psh is initialized (user entered right password)
    let initialized = use_state(|| false);
    // Master password
    let master_password = use_state(|| String::new());
    // Second master password value (from second input) on db initialization
    let master_password2 = use_state(|| String::new());
    // 'Master password is wrong' flag
    let mp_wrong = use_state(|| false);
    // Aliases that are stored in psh database
    let known_aliases = use_state(|| Vec::<String>::new());
    // Currently input alias
    let alias = use_state(|| String::new());
    // Currently selected alias handle
    let alias_handle = use_state(|| AliasHandle::Store);
    // Last user choice of alias handle
    let alias_handle_user_choice = use_state(|| AliasHandle::Store);
    // Whether current alias should use secret or not
    let use_secret = use_state(|| true);
    // Currently input secret
    let secret = use_state(|| String::new());
    // Currently selected charset
    let charset = use_state(|| CharSet::Standard);
    // Last user choice of charset
    let charset_user_choice = use_state(|| CharSet::Standard);
    // Whether current alias is stored in psh database or not
    let known_alias = use_state(|| false);
    // Derived password
    let password_msg = use_state(|| String::new());

    // Variables derived from state

    let db_initialized = *initialized || PshWebDb::new().exists();

    let mp_sufficient_len = (*master_password).len() >= 8;

    let mps_match = db_initialized || master_password == master_password2;

    let can_derive_password =  !(*alias).trim().is_empty()
        && ((*use_secret && !(*secret).is_empty())
            || !*use_secret
            || !*known_alias);

    let can_process =
        if *alias_handle == AliasHandle::Remove { *known_alias }
        else { can_derive_password };

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

    // Initialize state on first render
    /*
    let psh_db = psh_db.clone();
    use_effect_with_deps
        move |_| {
            psh_db
        },
        (),
    );
    */

    // Callbacks

    // Form input handlers

    let on_password_input: Callback<String> = {
        let master_password = master_password.clone();
        let mp_wrong = mp_wrong.clone();
        Callback::from(move |input: String| {
            master_password.set(input.clone());
            mp_wrong.set(false);
        })
    };

    let on_password2_input: Callback<String> = {
        let master_password2 = master_password2.clone();
        Callback::from(move |input: String| {
            master_password2.set(input.clone());
        })
    };

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
            alias.set(input.clone());
            known_alias.set(known);
            if known {
                alias_handle.set(AliasHandle::Store);
                let alias = ZeroizingString::new(input);
                let needs_secret = psh.borrow().get().unwrap().alias_uses_secret(&alias);
                use_secret.set(needs_secret);
                let alias_charset = psh.borrow().get().unwrap().get_charset(&alias);
                charset.set(alias_charset);
            }
            else {
                // Reset "remove" alias handle because it's only applicable to known aliases
                if *alias_handle_user_choice == AliasHandle::Remove {
                    alias_handle.set(AliasHandle::Store);
                } else {
                    alias_handle.set(*alias_handle_user_choice);
                }
                use_secret.set(true);
                charset.set(*charset_user_choice);
            }
            // Clear last derived password on new alias input
            password_msg.set("".to_string());
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

    // Form processing handlers

    let login = {
        let psh = psh.clone();
        let initialized = initialized.clone();
        let master_password = master_password.clone();
        let mp_wrong = mp_wrong.clone();
        let known_aliases = known_aliases.clone();
        Callback::from(move |_| {
            let res = Psh::new(
                ZeroizingString::new((*master_password).clone()),
                PshWebDb::new(),
            );
            if let Ok(psh_instance) = res {
                initialized.set(true);
                known_aliases.set(collect_aliases(&psh_instance));
                psh.borrow_mut().set(psh_instance).ok();
            } else {
                mp_wrong.set(true);
                log(&format!("Failed to initialize Psh: {}", res.err().unwrap()));
            }
            master_password.set("".to_string());
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
        let use_secret = use_secret.clone();
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
                let needs_secret = secret_string.is_some();
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
                            Some(needs_secret),
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
                use_secret.set(true);
                known_alias.set(false);
                alias_handle.set(AliasHandle::Store);
                alias_handle_user_choice.set(AliasHandle::Store);
                charset.set(CharSet::Standard);
                charset_user_choice.set(CharSet::Standard);
            }
        })
    };

    let known_aliases = (*known_aliases).clone();
    let password_msg = (*password_msg).clone();

    html! {
        <main class="container">
        if *initialized {
            <div class="element password">
                <strong>{ &*password_msg }</strong>
            </div>
            <AliasInput
                clear={!password_msg.is_empty()}
                {known_aliases}
                on_input={on_alias_input.clone()}
            />
            <SecretInput
                clear={!password_msg.is_empty()}
                disabled={!*use_secret}
                id="secret-input"
                hint="Enter secret..."
                on_input={on_secret_input.clone()}
            />
            <div class="element">
                <button type="button" onclick={process} disabled={!can_process}>
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
        } else {
            <SecretInput
                clear={*mp_wrong}
                id="mp-input"
                hint="Enter master password..."
                on_input={on_password_input.clone()}
            />
            if !db_initialized {
                <SecretInput
                    id="mp2-input"
                    hint="Repeat master password..."
                    on_input={on_password2_input.clone()}
                />
                <div class="element">
                    {"Warning: if you forget your Master Password you won't be able to get your passwords"}
                </div>
            }
            <div class="element">
                <button type="button" onclick={login} disabled={!mp_sufficient_len || !mps_match}>
                    {"Enter"}
                </button>
            </div>
            if *mp_wrong {
                <div class="element">{"Wrong master password"}</div>
            }
        }
        </main>
    }
}
