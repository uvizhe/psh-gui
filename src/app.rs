use std::time::Duration;

use once_cell::sync::OnceCell;
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew::platform::{spawn_local, time::sleep};

use psh::{CharSet, Psh, PshStore, ZeroizingString};
use psh_webdb::PshWebDb;

mod components;

use components::alias_input::AliasInput;
use components::secret_input::SecretInput;
use components::triswitch::Triswitch;
use components::collapsible::Collapsible;
#[cfg(feature = "keyboard")]
use components::keyboard::Keyboard;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[derive(PartialEq)]
enum AppState {
    New,
    Unlocking,
    Initialized,
}

#[derive(Copy, Clone, Debug, PartialEq)]
enum AliasHandle {
    Store,
    Ignore,
    Remove,
}

fn alias_handle_id(handle: AliasHandle) -> usize {
    match handle {
        AliasHandle::Store => 0,
        AliasHandle::Ignore => 1,
        AliasHandle::Remove => 2,
    }
}

fn charset_id(charset: CharSet) -> usize {
    match charset {
        CharSet::Standard => 0,
        CharSet::RequireAll => 1,
        CharSet::Reduced => 2,
    }
}

fn collect_aliases(psh: &Psh) -> Vec<String> {
    psh.aliases()
        .iter()
        .map(|x| x.to_string())
        .collect()
}

fn initialize_psh(master_password: String, cb: Callback<Option<Psh>>) {
    spawn_local(async move {
        // XXX: The process is very resource-intensive and freezes Yew completely so
        // we give Yew 10 milliseconds to update UI before it becomes unresponsive :(
        sleep(Duration::from_millis(10)).await;

        let res = Psh::new(
            ZeroizingString::new(master_password),
            PshWebDb::new(),
        );
        if let Ok(psh) = res {
            cb.emit(Some(psh));
        } else {
            log(&format!("Failed to initialize Psh: {}", res.err().unwrap()));
            cb.emit(None);
        }
    });
}

pub enum Msg {
    OnInputFocus(NodeRef),
    OnPasswordInput(String),
    OnPassword2Input(String),
    OnAliasInput((String, bool)),
    OnSecretInput(String),
    Login,
    Process,
    Initialize(Option<Psh>),
    SetCharset(String),
    SetAliasHandle(String),
    OnOptionsCollapsibleClick(bool),
    #[cfg(feature = "keyboard")]
    OnKbInput(String),
    #[cfg(feature = "keyboard")]
    OnKbCollapsibleClick(bool),
}

pub struct App {
    // App state
    state: AppState,
    // Psh instance
    psh: OnceCell<Psh>,
    // Master password
    master_password: String,
    // Second master password value (from second input) on db initialization
    master_password2: String,
    // 'Master password is wrong' flag
    mp_wrong: bool,
    // Aliases that are stored in psh database
    known_aliases: Vec<String>,
    // Currently input alias
    alias: String,
    // Currently selected alias handle
    alias_handle: AliasHandle,
    // Last user choice of alias handle
    alias_handle_user_choice: AliasHandle,
    // Whether current alias should use secret or not
    use_secret: bool,
    // Currently input secret
    secret: String,
    // Currently selected charset
    charset: CharSet,
    // Last user choice of charset
    charset_user_choice: CharSet,
    // Whether current alias is stored in psh database or not
    known_alias: bool,
    // Derived password
    password_msg: String,
    // Password element NodeRef
    password_ref: NodeRef,
    // NodeRef of currently focused input
    input_ref: NodeRef,
    // Visibility of options
    options_visible: bool,
    // Visibility of keyboard
    #[cfg(feature = "keyboard")]
    kb_visible: bool,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            state: AppState::New,
            psh: OnceCell::new(),
            master_password: String::new(),
            master_password2: String::new(),
            mp_wrong: false,
            known_aliases: Vec::new(),
            alias: String::new(),
            alias_handle: AliasHandle::Store,
            alias_handle_user_choice: AliasHandle::Store,
            use_secret: true,
            secret: String::new(),
            charset: CharSet::Standard,
            charset_user_choice: CharSet::Standard,
            known_alias: false,
            password_msg: String::new(),
            password_ref: NodeRef::default(),
            input_ref: NodeRef::default(),
            options_visible: false,
            #[cfg(feature = "keyboard")]
            kb_visible: true,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnInputFocus(new_input_ref) => {
                // Remove "focused" class from previously focused input
                if self.input_ref.get().is_some() {
                    let input = self.input_ref.cast::<web_sys::HtmlElement>().unwrap();
                    input.set_class_name("");
                }

                // Add "focused" class to newly focused input
                if let Some(input) = new_input_ref.cast::<web_sys::HtmlElement>() {
                    input.set_class_name("focused");
                }

                self.input_ref = new_input_ref;
            }
            Msg::OnPasswordInput(input) => {
                self.master_password = input;
                self.mp_wrong = false;
            }
            Msg::OnPassword2Input(input) => {
                self.master_password2 = input;
            }
            Msg::OnAliasInput((input, known)) => {
                self.alias = input.clone();
                self.known_alias = known.clone();
                if known {
                    self.alias_handle = AliasHandle::Store;
                    let alias = ZeroizingString::new(input);
                    let needs_secret = self.psh.get().unwrap().alias_uses_secret(&alias);
                    self.use_secret = needs_secret;
                    let alias_charset = self.psh.get().unwrap().get_charset(&alias);
                    self.charset = alias_charset;
                }
                else {
                    // Reset "remove" alias handle because it's only applicable to known aliases
                    if self.alias_handle_user_choice == AliasHandle::Remove {
                        self.alias_handle = AliasHandle::Store;
                    } else {
                        self.alias_handle = self.alias_handle_user_choice;
                    }
                    self.use_secret = true;
                    self.charset = self.charset_user_choice;
                }
                // Clear last derived password on new alias input
                self.password_msg.clear()
            }
            Msg::OnSecretInput(input) => {
                self.secret = input;
            }
            Msg::OnOptionsCollapsibleClick(visible) => {
                self.options_visible = visible;
            }
            Msg::SetAliasHandle(value) => {
                match value.as_str() {
                    "0" => {
                        self.alias_handle = AliasHandle::Store;
                        self.alias_handle_user_choice = AliasHandle::Store;
                    }
                    "1" => {
                        self.alias_handle = AliasHandle::Ignore;
                        self.alias_handle_user_choice = AliasHandle::Ignore;
                    }
                    "2" => {
                        self.alias_handle = AliasHandle::Remove;
                        self.alias_handle_user_choice = AliasHandle::Remove;
                    }
                    _ => unimplemented!()
                }
            }
            Msg::SetCharset(value) => {
                match value.as_str() {
                    "0" => {
                        self.charset = CharSet::Standard;
                        self.charset_user_choice = CharSet::Standard;
                    }
                    "1" => {
                        self.charset = CharSet::RequireAll;
                        self.charset_user_choice = CharSet::RequireAll;
                    }
                    "2" => {
                        self.charset = CharSet::Reduced;
                        self.charset_user_choice = CharSet::Reduced;
                    }
                    _ => unreachable!()
                }
            }
            Msg::Login => {
                self.state = AppState::Unlocking;
                let master_password = self.master_password.clone();
                let callback = ctx.link().callback(Msg::Initialize);
                initialize_psh(master_password, callback);
            }
            Msg::Initialize(maybe_psh) => {
                if let Some(psh) = maybe_psh {
                    self.state = AppState::Initialized;
                    self.known_aliases = collect_aliases(&psh);
                    self.psh.set(psh).ok();
                } else {
                    self.state = AppState::New;
                    self.mp_wrong = true;
                }
                self.master_password.clear();
            }
            Msg::Process => {
                // No input is in focus now
                ctx.link().send_message(Msg::OnInputFocus(NodeRef::default()));

                let psh = self.psh.get_mut().unwrap();
                let alias_string = self.alias.trim().to_string();
                let secret_string =
                    if self.secret.to_string().is_empty() || !self.use_secret {
                        None
                    } else {
                        Some(ZeroizingString::new(self.secret.to_string()))
                    };
                let needs_secret = secret_string.is_some();
                if self.alias_handle != AliasHandle::Remove {
                    let pass = psh.derive_password(
                        &ZeroizingString::new(alias_string.clone()),
                        secret_string,
                        Some(self.charset),
                    );
                    self.password_msg = pass.to_string();
                    if !self.known_aliases.contains(&alias_string)
                        && self.alias_handle == AliasHandle::Store
                    {
                        let res = psh.append_alias_to_db(
                            &ZeroizingString::new(alias_string.clone()),
                            Some(needs_secret),
                            Some(self.charset),
                        );
                        if res.is_ok() {
                            self.known_aliases = collect_aliases(psh);
                        } else {
                            log("Failed to save alias");
                        }
                    }
                } else {
                    let res = psh.remove_alias_from_db(&ZeroizingString::new(alias_string.clone()));
                    if res.is_ok() {
                        self.known_aliases = collect_aliases(psh);
                    } else {
                        log("Failed to remove alias");
                    }
                }
                self.alias.clear();
                self.secret.clear();
                self.use_secret = true;
                self.known_alias = false;
                self.alias_handle = AliasHandle::Store;
                self.alias_handle_user_choice = AliasHandle::Store;
                self.charset = CharSet::Standard;
                self.charset_user_choice = CharSet::Standard;
                // Focus on Password element to move focus away from button
                let el = self.password_ref.cast::<web_sys::HtmlElement>().unwrap();
                el.focus().unwrap();
            }
            #[cfg(feature = "keyboard")]
            Msg::OnKbCollapsibleClick(visible) => {
                self.kb_visible = visible;
            }
            #[cfg(feature = "keyboard")]
            Msg::OnKbInput(value) => {
                if self.input_ref.get().is_some() {
                    let input = self.input_ref.cast::<web_sys::HtmlInputElement>().unwrap();

                    // Fill input with new value
                    let new_value =
                        if !value.is_empty() {
                            input.value() + &value
                        } else {
                            // Keyboard sends empty strings on Backspace key presses
                            let mut new_value = input.value();
                            new_value.pop();
                            new_value
                        };
                    input.set_value(&new_value);

                    // Find relative variable in store and change it as well
                    let id = input.id();
                    match id.as_str() {
                        "mp-input" => ctx.link().send_message(Msg::OnPasswordInput(new_value)),
                        "mp2-input" => ctx.link().send_message(Msg::OnPassword2Input(new_value)),
                        "alias-input" => {
                            let known = self.known_aliases.contains(&new_value);
                            ctx.link().send_message(Msg::OnAliasInput((new_value, known)));
                        }
                        "secret-input" => ctx.link().send_message(Msg::OnSecretInput(new_value)),
                        _ => unimplemented!()
                    }
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        #[cfg(feature = "keyboard")]
        let maybe_keyboard: Html = html!{
            <>
                <Collapsible
                    name="keyboard"
                    on_click={ctx.link().callback(Msg::OnKbCollapsibleClick)}
                />
                <Keyboard
                    visible={self.kb_visible}
                    on_input={ctx.link().callback(Msg::OnKbInput)}
                />
            </>
        };
        #[cfg(not(feature = "keyboard"))]
        let maybe_keyboard: Html = html!{};

        #[cfg(feature = "keyboard")]
        let keyboard_use = if self.kb_visible { true } else { false };
        #[cfg(not(feature = "keyboard"))]
        let keyboard_use = false;

        let spinner_overlay = {
            let db_exists = PshWebDb::new().exists();

            html! {
                <div class="overlay">
                    <div class="spinner"/>
                    <div>{ if db_exists {"Unlocking..."} else {"Initializing..."} }</div>
                </div>
            }
        };

        let entrance_view = {
            let db_exists = PshWebDb::new().exists();

            let mp_invalid = self.master_password.len() < 8
                || (!db_exists && self.master_password != self.master_password2);

            let maybe_warning = {
                let class = if db_exists {
                    let maybe_invisible = if self.mp_wrong { None } else { Some("invisible") };
                    classes!("element", maybe_invisible)
                } else {
                    classes!("element")
                };
                let message = if db_exists {
                    "Wrong master password"
                } else {
                    "Warning: if you forget your Master Password
                        you won't be able to retrieve your passwords"
                };
                html! { <div class={class}>{ message }</div> }
            };

            html! {
                <>
                    { maybe_warning }
                    <SecretInput
                        clear={self.mp_wrong}
                        focus=true
                        id="mp-input"
                        hint="Enter master password..."
                        keyboard={keyboard_use}
                        on_input={ctx.link().callback(Msg::OnPasswordInput)}
                        on_focus={ctx.link().callback(Msg::OnInputFocus)}
                    />
                    if !db_exists {
                        <SecretInput
                            id="mp2-input"
                            hint="Repeat master password..."
                            keyboard={keyboard_use}
                            on_input={ctx.link().callback(Msg::OnPassword2Input)}
                            on_focus={ctx.link().callback(Msg::OnInputFocus)}
                        />
                    }
                    <div class="element">
                        <button type="button"
                            onclick={ctx.link().callback(|_| Msg::Login)}
                            disabled={mp_invalid}
                        >
                            { if db_exists { "Unlock" } else { "Start using Psh"} }
                        </button>
                    </div>
                </>
            }
        };

        let main_view = {
            let can_derive_password = !self.alias.trim().is_empty()
                && ((self.use_secret && !self.secret.is_empty())
                    || !self.use_secret
                    || !self.known_alias);

            let can_process =
                if self.alias_handle == AliasHandle::Remove { self.known_alias }
                else { can_derive_password };

            html! {
                <>
                    <AliasInput
                        clear={self.alias.is_empty()}
                        known_aliases={self.known_aliases.clone()}
                        keyboard={keyboard_use}
                        on_input={ctx.link().callback(Msg::OnAliasInput)}
                        on_focus={ctx.link().callback(Msg::OnInputFocus)}
                    />
                    <SecretInput
                        clear={self.secret.is_empty() || !self.use_secret}
                        disabled={!self.use_secret}
                        id="secret-input"
                        hint="Enter secret..."
                        keyboard={keyboard_use}
                        on_input={ctx.link().callback(Msg::OnSecretInput)}
                        on_focus={ctx.link().callback(Msg::OnInputFocus)}
                    />
                    <div class="element">
                        <button type="button"
                            onclick={ctx.link().callback(|_| Msg::Process)}
                            disabled={!can_process}
                        >
                            {
                                if self.alias_handle != AliasHandle::Remove {"Get password"}
                                else {"Remove alias"}
                            }
                        </button>
                    </div>
                    <div class="element password" ref={self.password_ref.clone()} tabindex="-1">
                        <strong>{ &self.password_msg }</strong>
                    </div>
                    <Collapsible name="options"
                        start_collapsed=true
                        on_click={ctx.link().callback(Msg::OnOptionsCollapsibleClick)}
                    />
                    <Triswitch
                        checked={alias_handle_id(self.alias_handle)}
                        disabled={vec![false, self.known_alias, !self.known_alias]}
                        visible={self.options_visible}
                        name="alias_handle"
                        title="How to handle alias"
                        labels={vec![
                            "Store".to_string(),
                            "Don't store".to_string(),
                            "Remove".to_string()]}
                        on_switch={ctx.link().callback(Msg::SetAliasHandle)}
                    />
                    <Triswitch
                        checked={charset_id(self.charset)}
                        disabled={vec![self.known_alias, self.known_alias, self.known_alias]}
                        visible={self.options_visible}
                        name="charset"
                        title="Character set to use"
                        labels={vec![
                            "Standard".to_string(),
                            "Require All".to_string(),
                            "Reduced".to_string()]}
                        on_switch={ctx.link().callback(Msg::SetCharset)}
                    />
                </>
            }
        };

        html! {
            <main class="container">
            { match self.state {
                AppState::New => entrance_view,
                AppState::Unlocking => html!{
                    <>
                        { spinner_overlay }
                        { entrance_view }
                    </>
                },
                AppState::Initialized => main_view,
            }}
            { maybe_keyboard }
            </main>
        }
    }
}
