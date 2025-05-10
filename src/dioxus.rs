#![doc = include_str!("../DIOXUS.md")]

use dioxus::prelude::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::str::FromStr;

#[cfg(target_arch = "wasm32")]
use web_sys::{
    HtmlElement, MediaQueryList, Storage, Window,
    js_sys::Date,
    wasm_bindgen::{JsCast, prelude::*},
};

#[cfg(target_arch = "wasm32")]
use crate::common::SYSTEM_THEME_QUERY;

use crate::common::{CustomTheme, DEFAULT_STORAGE_KEY, StorageType, Theme};

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: Signal<Theme>,
    pub resolved_theme: Signal<Theme>,
    pub system_theme: Signal<Theme>,
    pub set_theme: Callback<Theme>,
    pub forced_theme: Option<Theme>,
    pub custom_themes: Signal<HashMap<String, Rc<CustomTheme>>>,
    pub set_custom_theme: Callback<Rc<CustomTheme>>,
    pub reset_to_system: Callback<()>,
    pub preview_theme: Signal<Option<Theme>>,
    pub apply_preview: Callback<Theme>,
}

#[derive(Props, PartialEq, Clone)]
pub struct ThemeProviderProps {
    #[props(default)]
    pub children: Element,
    #[props(default)]
    pub default_theme: Theme,
    #[props(default)]
    pub storage_type: StorageType,
    #[props(default = DEFAULT_STORAGE_KEY)]
    pub storage_name: &'static str,
    #[props(default)]
    pub forced_theme: Option<Theme>,
    #[props(default)]
    pub custom_themes: HashMap<String, Rc<CustomTheme>>,
}

#[component]
pub fn ThemeProvider(props: ThemeProviderProps) -> Element {
    let val: Option<String> = {
        #[cfg(target_arch = "wasm32")]
        {
            let window = web_sys::window().unwrap();
            let storage: Option<Storage> = match props.storage_type {
                StorageType::LocalStorage => window.local_storage().unwrap(),
                StorageType::SessionStorage => window.session_storage().unwrap(),
            };
            storage
                .and_then(|s| s.get_item(props.storage_name).ok())
                .expect("stored theme not found")
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            None
        }
    };

    let stored_theme = val
        .as_deref()
        .and_then(|s| Theme::from_str(s).ok())
        .unwrap_or(props.default_theme);

    let mut theme = use_signal(|| stored_theme.clone());
    #[cfg(target_arch = "wasm32")]
    let mut system_theme = use_signal(|| Theme::Light);
    #[cfg(not(target_arch = "wasm32"))]
    let system_theme = use_signal(|| Theme::Light);
    let mut resolved_theme = use_signal(|| Theme::Light);
    let mut preview_theme = use_signal(|| None::<Theme>);
    let mut custom_themes_state = use_signal(|| props.custom_themes.clone());

    #[cfg(target_arch = "wasm32")]
    let window = use_signal(|| web_sys::window().expect("window not found"));

    #[cfg(target_arch = "wasm32")]
    let document = window().document().expect("document not found");

    #[cfg(target_arch = "wasm32")]
    let html_element = use_signal(|| {
        document
            .document_element()
            .expect("html element not found")
            .dyn_into::<HtmlElement>()
            .expect("failed to cast to HtmlElement")
    });

    #[cfg(target_arch = "wasm32")]
    let forced_theme = props.forced_theme.clone();

    let update_resolved = {
        Callback::new(move |new_theme: Theme| {
            #[cfg(target_arch = "wasm32")]
            {
                let mq = window()
                    .match_media(SYSTEM_THEME_QUERY)
                    .unwrap()
                    .unwrap()
                    .unchecked_into::<MediaQueryList>();
                let sys_theme = if mq.matches() {
                    Theme::Dark
                } else {
                    Theme::Light
                };
                system_theme.set(sys_theme.clone());

                let binding = preview_theme();
                let final_theme = if let Some(forced) = &forced_theme {
                    forced.clone()
                } else if let Some(preview) = &binding {
                    preview.clone()
                } else {
                    match new_theme {
                        Theme::System => sys_theme.clone(),
                        other => other,
                    }
                };

                resolved_theme.set(final_theme.clone());
                let _ = html_element().set_attribute("data-theme", &final_theme.as_str());
                let _ = html_element().set_attribute("class", &final_theme.as_str());
                let _ = html_element()
                    .set_attribute("style", &format!("color-scheme: {};", final_theme.as_str()));
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                resolved_theme.set(match new_theme {
                    Theme::System => Theme::Light,
                    other => other,
                });
            }
        })
    };

    #[cfg(target_arch = "wasm32")]
    let storage_name = props.storage_name;
    #[cfg(target_arch = "wasm32")]
    let storage_type = props.storage_type;

    use_effect(move || {
        update_resolved.call(theme());

        #[cfg(target_arch = "wasm32")]
        {
            let mq = window()
                .match_media(SYSTEM_THEME_QUERY)
                .unwrap()
                .unwrap()
                .unchecked_into::<MediaQueryList>();

            let closure: Closure<dyn FnMut(web_sys::Event)> = Closure::wrap(Box::new(move |_| {
                update_resolved.call(theme());
            }));
            mq.add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                .unwrap();
            closure.forget();

            let on_storage: Closure<dyn FnMut(web_sys::StorageEvent)> =
                Closure::wrap(Box::new(move |_event| {
                    let val: Option<String> = match storage_type {
                        StorageType::LocalStorage => web_sys::window()
                            .unwrap()
                            .local_storage()
                            .unwrap()
                            .and_then(|s| s.get_item(storage_name).ok())
                            .expect("stored theme not found"),
                        StorageType::SessionStorage => web_sys::window()
                            .unwrap()
                            .session_storage()
                            .unwrap()
                            .and_then(|s| s.get_item(storage_name).ok())
                            .expect("stored theme not found"),
                    };

                    if let Some(s) = val {
                        if let Ok(t) = Theme::from_str(&s) {
                            theme.set(t.clone());
                            update_resolved.call(t);
                        }
                    }
                }));
            window()
                .add_event_listener_with_callback("storage", on_storage.as_ref().unchecked_ref())
                .unwrap();
            on_storage.forget();

            let interval_closure: Closure<dyn FnMut()> = Closure::wrap(Box::new(move || {
                let hour = Date::new_0().get_hours();
                let next = if (7..19).contains(&hour) {
                    Theme::Light
                } else {
                    Theme::Dark
                };
                theme.set(next.clone());
                update_resolved.call(next);
            }));

            let interval_id = window()
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    interval_closure.as_ref().unchecked_ref(),
                    60_000,
                )
                .unwrap();
            interval_closure.forget();

            window().clear_interval_with_handle(interval_id);
        }
    });

    let set_theme = {
        #[cfg(target_arch = "wasm32")]
        let storage_name = props.storage_name;
        #[cfg(target_arch = "wasm32")]
        let storage_type = props.storage_type;
        Callback::new(move |new_theme: Theme| {
            #[cfg(target_arch = "wasm32")]
            {
                let _ = match storage_type {
                    StorageType::LocalStorage => web_sys::window()
                        .unwrap()
                        .local_storage()
                        .unwrap()
                        .and_then(|ls| ls.set_item(storage_name, &new_theme.as_str()).ok()),
                    StorageType::SessionStorage => web_sys::window()
                        .unwrap()
                        .session_storage()
                        .unwrap()
                        .and_then(|ss| ss.set_item(storage_name, &new_theme.as_str()).ok()),
                };
            }
            theme.set(new_theme.clone());
            update_resolved.call(new_theme);
        })
    };

    let set_custom_theme = {
        Callback::new(move |new_theme: Rc<CustomTheme>| {
            #[cfg(target_arch = "wasm32")]
            if let Err(e) = new_theme.validate() {
                web_sys::console::error_1(&format!("Theme validation error: {}", e).into());
                return;
            }
            let mut themes = custom_themes_state();
            themes.insert(new_theme.name.clone(), new_theme);
            custom_themes_state.set(themes);
        })
    };

    let reset_to_system = {
        Callback::new(move |_| {
            set_theme.call(Theme::System);
        })
    };

    let apply_preview = {
        Callback::new(move |theme: Theme| {
            preview_theme.set(Some(theme.clone()));
            #[cfg(target_arch = "wasm32")]
            {
                let _ = html_element().set_attribute("data-theme", &theme.as_str());
                let _ = html_element().set_attribute("class", &theme.as_str());
                let _ = html_element()
                    .set_attribute("style", &format!("color-scheme: {};", theme.as_str()));
            }
        })
    };

    let context = Rc::new(ThemeContext {
        theme,
        resolved_theme,
        system_theme,
        set_theme,
        forced_theme: props.forced_theme,
        custom_themes: custom_themes_state,
        set_custom_theme,
        reset_to_system,
        preview_theme,
        apply_preview,
    });

    provide_context(context);

    props.children
}

pub fn use_theme() -> Rc<ThemeContext> {
    consume_context::<Rc<ThemeContext>>()
}
