#![doc = include_str!("../YEW.md")]

use crate::common::{CustomTheme, DEFAULT_STORAGE_KEY, SYSTEM_THEME_QUERY, StorageType, Theme};
use gloo::events::EventListener;
use gloo::storage::{LocalStorage, SessionStorage, Storage};
use gloo::timers::callback::Interval;
use std::collections::HashMap;
use std::rc::Rc;
use web_sys::{HtmlElement, js_sys::Date, wasm_bindgen::JsCast, window};
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
        storage_type,
        storage_name,
        forced_theme,
        custom_themes,
    } = props.clone();

    let theme: UseStateHandle<Theme> = {
        let stored_theme = match storage_type {
            StorageType::LocalStorage => LocalStorage::get(storage_name)
                .ok()
                .flatten()
                .unwrap_or(default_theme.clone()),
            StorageType::SessionStorage => SessionStorage::get(storage_name)
                .ok()
                .flatten()
                .unwrap_or(default_theme.clone()),
        };
        use_state(|| stored_theme)
    };

    let system_theme = use_state(|| Theme::Light);
    let resolved_theme = use_state(|| Theme::Light);
    let custom_themes_state = use_state(|| custom_themes);
    let preview_theme = use_state(|| None::<Theme>);

    let html_element = use_state(|| {
        let window = window().expect("No window object");
        let document = window.document().expect("No document object");
        document
            .document_element()
            .unwrap()
            .dyn_into::<HtmlElement>()
            .unwrap()
    });

    let update_resolved = {
        let system_theme = system_theme.clone();
        let resolved_theme = resolved_theme.clone();
        let html_element = html_element.clone();
        let forced_theme = forced_theme.clone();
        let preview_theme = preview_theme.clone();
        Callback::from(move |new_theme: Theme| {
            let window = window().unwrap();
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
                    Theme::System => sys_theme.clone(),
                    other => other.clone(),
                }
            };

            resolved_theme.set(final_theme.clone());
            let _ = html_element.set_attribute("data-theme", &final_theme.as_str());
            let _ = html_element.set_attribute("class", &final_theme.as_str());
            let _ = html_element
                .set_attribute("style", &format!("color-scheme: {};", final_theme.as_str()));
        })
    };

    {
        let update_resolved = update_resolved.clone();
        let theme_for_mq = theme.clone();
        let theme_for_storage = theme.clone();
        let theme_for_interval = theme.clone();

        use_effect_with((), move |_| {
            update_resolved.emit((*theme_for_mq).clone());

            let window = window().unwrap();
            let media_query = window.match_media(SYSTEM_THEME_QUERY).unwrap().unwrap();
            let mq_listener = EventListener::new(&media_query, "change", {
                let update_resolved = update_resolved.clone();
                move |_| {
                    update_resolved.emit((*theme_for_mq).clone());
                }
            });

            let storage_listener = EventListener::new(&window, "storage", {
                let update_resolved = update_resolved.clone();
                let theme = theme_for_storage.clone();
                move |_| {
                    let val = match storage_type {
                        StorageType::LocalStorage => LocalStorage::get(storage_name).ok().flatten(),
                        StorageType::SessionStorage => {
                            SessionStorage::get(storage_name).ok().flatten()
                        }
                    };
                    if let Some(t) = val {
                        theme.set(t);
                    }
                    update_resolved.emit((*theme).clone());
                }
            });

            let set_theme = theme_for_interval.clone();
            let interval = Interval::new(60_000, move || {
                let hour = Date::new_0().get_hours();
                let theme = if (7..19).contains(&hour) {
                    Theme::Light
                } else {
                    Theme::Dark
                };
                set_theme.set(theme.clone());
                update_resolved.emit(theme.clone());
            });

            move || {
                drop(mq_listener);
                drop(storage_listener);
                drop(interval);
            }
        });
    }

    let set_theme = {
        let theme = theme.clone();
        let update_resolved = update_resolved.clone();
        Callback::from(move |new_theme: Theme| {
            match storage_type {
                StorageType::LocalStorage => {
                    let _ = LocalStorage::set(storage_name, &new_theme);
                }
                StorageType::SessionStorage => {
                    let _ = SessionStorage::set(storage_name, &new_theme);
                }
            }
            theme.set(new_theme.clone());
            update_resolved.emit(new_theme.clone());
        })
    };

    let set_custom_theme = {
        let custom_themes_state = custom_themes_state.clone();
        Callback::from(move |new_custom_theme: Rc<CustomTheme>| {
            if let Err(error) = new_custom_theme.validate() {
                gloo::console::error!(format!("Theme validation error: {}", error));
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
        let html_element = html_element.clone();
        Callback::from(move |theme: Theme| {
            preview_theme.set(Some(theme.clone()));
            let _ = html_element.set_attribute("data-theme", &theme.as_str());
            let _ = html_element.set_attribute("class", &theme.as_str());
            let _ =
                html_element.set_attribute("style", &format!("color-scheme: {};", theme.as_str()));
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
