use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct CollapsibleProps {
    pub name: String,
    #[prop_or_default]
    pub start_collapsed: bool,
    pub on_click: Callback<bool>,
}

#[function_component(Collapsible)]
pub fn collapsible(props: &CollapsibleProps) -> Html {
    let open = use_state(|| true);

    {
        let open = open.clone();
        let start_collapsed = props.start_collapsed.clone();
        use_effect_with_deps(
            move |_| {
                if start_collapsed {
                    open.set(false);
                }
            },
            (),
        );
    }

    let on_click = {
        let open = open.clone();
        let on_click = props.on_click.clone();
        Callback::from(move |_| {
            let new_state = !*open;
            on_click.emit(new_state);
            open.set(new_state);
        })
    };

    let visible_state = if *open { Some("open") } else { Some("closed") };

    html! {
        <div class="element collapsible" onclick={on_click}>
            <div class={classes!("collapsible-msg", visible_state)}>
            if *open {
                { &format!("Hide {}", props.name) }
            } else {
                { &format!("Show {}", props.name) }
            }
            </div>
        </div>
    }
}
