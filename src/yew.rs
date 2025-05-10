#![doc = include_str!("../YEW.md")]

use crate::common::{CustomTheme, DEFAULT_STORAGE_KEY, StorageType, Theme};
use std::collections::HashMap;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct ThemeContext {
    pub theme: UseStateHandle<Theme>,
    pub resolved_theme: UseStateHandle<Theme>,
    pub system_theme: UseStateHandle<Theme>,
    pub set_theme: Callback<Theme>,
    pub forced_theme: Option<Theme>,
    pub custom_themes: UseStateHandle<HashMap<String, Rc<CustomTheme>>>,
    pub set_custom_theme: Callback<Rc<CustomTheme>>,
    pub reset_to_system: Callback<()>,
    pub preview_theme: UseStateHandle<Option<Theme>>,
    pub apply_preview: Callback<Theme>,
}

#[derive(Properties, Clone, PartialEq)]
pub struct ThemeProviderProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub default_theme: Theme,
    #[prop_or_default]
    pub storage_type: StorageType,
    #[prop_or(DEFAULT_STORAGE_KEY)]
    pub storage_name: &'static str,
    #[prop_or_default]
    pub forced_theme: Option<Theme>,
    #[prop_or_default]
    pub custom_themes: HashMap<String, Rc<CustomTheme>>,
}

#[function_component(ThemeProvider)]
pub fn theme_provider(props: &ThemeProviderProps) -> Html {
    let ThemeProviderProps {
        children,
        default_theme,
        #[cfg(target_arch = "wasm32")]
        storage_type,
        #[cfg(not(target_arch = "wasm32"))]
            storage_type: _,
        #[cfg(target_arch = "wasm32")]
        storage_name,
        #[cfg(not(target_arch = "wasm32"))]
            storage_name: _,
        forced_theme,
        custom_themes,
    } = props.clone();

    let theme: UseStateHandle<Theme> = {
        #[cfg(target_arch = "wasm32")]
        let stored_theme = {
            use std::str::FromStr;
            use web_sys::window;

            let window = window().expect("no window");
            let storage: Option<web_sys::Storage> = match storage_type {
                StorageType::LocalStorage => window.local_storage().unwrap_or(None),
                StorageType::SessionStorage => window.session_storage().unwrap_or(None),
            };
            storage
                .and_then(|s| s.get_item(storage_name).ok().flatten())
                .and_then(|s| Theme::from_str(&s).ok())
                .unwrap_or(default_theme.clone())
        };

        #[cfg(not(target_arch = "wasm32"))]
        let stored_theme = default_theme.clone();

        use_state(|| stored_theme)
    };

    let system_theme = use_state(|| Theme::Light);
    let resolved_theme = use_state(|| Theme::Light);
    let custom_themes_state = use_state(|| custom_themes);
    let preview_theme = use_state(|| None::<Theme>);

    #[cfg(target_arch = "wasm32")]
    let html_element: UseStateHandle<web_sys::HtmlElement> = use_state(|| {
        use web_sys::wasm_bindgen::JsCast;
        web_sys::window()
            .expect("No window object")
            .document()
            .expect("No document object")
            .document_element()
            .unwrap()
            .dyn_into::<web_sys::HtmlElement>()
            .unwrap()
    });

    #[cfg(not(target_arch = "wasm32"))]
    let _html_element: UseStateHandle<web_sys::HtmlElement> =
        use_state(|| panic!("html_element is not available on server"));

    let update_resolved = {
        #[cfg(target_arch = "wasm32")]
        let system_theme = system_theme.clone();
        let resolved_theme = resolved_theme.clone();
        #[cfg(target_arch = "wasm32")]
        let html_element = html_element.clone();
        #[cfg(target_arch = "wasm32")]
        let forced_theme = forced_theme.clone();
        #[cfg(target_arch = "wasm32")]
        let preview_theme = preview_theme.clone();

        Callback::from(move |new_theme: Theme| {
            #[cfg(target_arch = "wasm32")]
            {
                use crate::common::SYSTEM_THEME_QUERY;

                let window = web_sys::window().unwrap();
                let media_query = window.match_media(SYSTEM_THEME_QUERY).unwrap().unwrap();
                let prefers_dark = media_query.matches();
                let sys_theme = if prefers_dark {
                    Theme::Dark
                } else {
                    Theme::Light
                };
                system_theme.set(sys_theme.clone());

                let final_theme = if let Some(ref forced) = forced_theme {
                    forced.clone()
                } else if let Some(preview) = &*preview_theme {
                    preview.clone()
                } else {
                    match new_theme {
                        Theme::System => sys_theme,
                        other => other,
                    }
                };

                resolved_theme.set(final_theme.clone());
                let _ = html_element.set_attribute("data-theme", &final_theme.as_str());
                let _ = html_element.set_attribute("class", &final_theme.as_str());
                let _ = html_element
                    .set_attribute("style", &format!("color-scheme: {};", final_theme.as_str()));
            }

            #[cfg(not(target_arch = "wasm32"))]
            {
                resolved_theme.set(new_theme);
            }
        })
    };

    {
        let update_resolved = update_resolved.clone();
        let theme_for_mq = theme.clone();
        #[cfg(target_arch = "wasm32")]
        let theme_for_storage = theme.clone();
        #[cfg(target_arch = "wasm32")]
        let theme_for_interval = theme.clone();

        use_effect_with((), move |_| {
            update_resolved.emit((*theme_for_mq).clone());

            #[cfg(target_arch = "wasm32")]
            {
                use crate::common::SYSTEM_THEME_QUERY;
                use std::str::FromStr;
                use web_sys::wasm_bindgen::JsCast;
                use web_sys::wasm_bindgen::closure::Closure;

                let window = web_sys::window().unwrap();

                let media_query = window.match_media(SYSTEM_THEME_QUERY).unwrap().unwrap();
                let closure = Closure::wrap(Box::new({
                    let update_resolved = update_resolved.clone();
                    move |_event: web_sys::Event| {
                        update_resolved.emit((*theme_for_mq).clone());
                    }
                }) as Box<dyn FnMut(_)>);
                media_query
                    .add_event_listener_with_callback("change", closure.as_ref().unchecked_ref())
                    .unwrap();
                closure.forget();

                let storage_closure = Closure::wrap(Box::new({
                    let theme = theme_for_storage.clone();
                    let update_resolved = update_resolved.clone();
                    move |_event: web_sys::StorageEvent| {
                        let window = web_sys::window().unwrap();
                        let storage = match storage_type {
                            StorageType::LocalStorage => window.local_storage().unwrap(),
                            StorageType::SessionStorage => window.session_storage().unwrap(),
                        };
                        if let Some(storage) = storage {
                            if let Ok(Some(value)) = storage.get_item(storage_name) {
                                if let Ok(parsed) = Theme::from_str(&value) {
                                    theme.set(parsed.clone());
                                    update_resolved.emit(parsed);
                                }
                            }
                        }
                    }
                }) as Box<dyn FnMut(_)>);
                window
                    .add_event_listener_with_callback(
                        "storage",
                        storage_closure.as_ref().unchecked_ref(),
                    )
                    .unwrap();
                storage_closure.forget();

                let interval_closure = Closure::wrap(Box::new({
                    let theme = theme_for_interval.clone();
                    let update_resolved = update_resolved.clone();
                    move || {
                        let hour = web_sys::js_sys::Date::new_0().get_hours();
                        let next = if (7..19).contains(&hour) {
                            Theme::Light
                        } else {
                            Theme::Dark
                        };
                        theme.set(next.clone());
                        update_resolved.emit(next);
                    }
                }) as Box<dyn FnMut()>);
                let _id = window
                    .set_interval_with_callback_and_timeout_and_arguments_0(
                        interval_closure.as_ref().unchecked_ref(),
                        60_000,
                    )
                    .unwrap();
                interval_closure.forget();
            }
        });
    }

    let set_theme = {
        let theme = theme.clone();
        let update_resolved = update_resolved.clone();
        Callback::from(move |new_theme: Theme| {
            #[cfg(target_arch = "wasm32")]
            {
                let window = web_sys::window().unwrap();
                let storage = match storage_type {
                    StorageType::LocalStorage => window.local_storage().unwrap(),
                    StorageType::SessionStorage => window.session_storage().unwrap(),
                };
                if let Some(storage) = storage {
                    let _ = storage.set_item(storage_name, &new_theme.as_str());
                }
            }

            theme.set(new_theme.clone());
            update_resolved.emit(new_theme);
        })
    };

    let set_custom_theme = {
        let custom_themes_state = custom_themes_state.clone();
        Callback::from(move |new_custom_theme: Rc<CustomTheme>| {
            #[cfg(target_arch = "wasm32")]
            if let Err(error) = new_custom_theme.validate() {
                web_sys::console::error_1(&format!("Theme validation error: {}", error).into());
                return;
            }
            let mut themes = (*custom_themes_state).clone();
            themes.insert(new_custom_theme.name.clone(), new_custom_theme.clone());
            custom_themes_state.set(themes);
        })
    };

    let reset_to_system = {
        let set_theme = set_theme.clone();
        Callback::from(move |_| {
            set_theme.emit(Theme::System);
        })
    };

    let apply_preview = {
        let preview_theme = preview_theme.clone();
        #[cfg(target_arch = "wasm32")]
        let html_element = html_element.clone();
        Callback::from(move |theme: Theme| {
            preview_theme.set(Some(theme.clone()));
            #[cfg(target_arch = "wasm32")]
            {
                let _ = html_element.set_attribute("data-theme", &theme.as_str());
                let _ = html_element.set_attribute("class", &theme.as_str());
                let _ = html_element
                    .set_attribute("style", &format!("color-scheme: {};", theme.as_str()));
            }
        })
    };

    let context = Rc::new(ThemeContext {
        theme,
        resolved_theme,
        system_theme,
        set_theme,
        forced_theme,
        custom_themes: custom_themes_state,
        set_custom_theme,
        reset_to_system,
        preview_theme,
        apply_preview,
    });

    html! {
        <ContextProvider<Rc<ThemeContext>> context={context}>
            { for children.iter() }
        </ContextProvider<Rc<ThemeContext>>>
    }
}

#[hook]
pub fn use_theme() -> Rc<ThemeContext> {
    use_context::<Rc<ThemeContext>>().expect("No ThemeProvider found")
}
