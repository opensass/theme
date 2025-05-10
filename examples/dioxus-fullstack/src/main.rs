use dioxus::prelude::*;
use theme::dioxus::ThemeProvider;
use theme::dioxus::use_theme;
use theme::Theme;

#[derive(Debug, Clone, Routable, PartialEq)]
#[rustfmt::skip]
enum Route {
    #[layout(Navbar)]
    #[route("/")]
    Home {},
    #[route("/blog/:id")]
    Blog { id: i32 },
}

const FAVICON: Asset = asset!("/assets/favicon.ico");
const MAIN_CSS: Asset = asset!("/assets/main.css");
const HEADER_SVG: Asset = asset!("/assets/header.svg");
const TAILWIND_CSS: Asset = asset!("/assets/tailwind.css");

fn main() {
    dioxus::launch(App);
}

#[component]
fn App() -> Element {
    rsx! {
        document::Link { rel: "icon", href: FAVICON }
        document::Link { rel: "stylesheet", href: MAIN_CSS } document::Link { rel: "stylesheet", href: TAILWIND_CSS }
        ThemeProvider {
            Router::<Route> {}
        }
    }
}


#[component]
fn ThemeToggle() -> Element {
    let theme_ctx = use_theme();

    let onclick = {
        move |_| {
            let new_theme = match (theme_ctx.theme)() {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
                _ => Theme::Light,
            };
            theme_ctx.set_theme.call(new_theme);
        }
    };

    rsx! {
        div { class: "flex items-center justify-center",
            button {
                onclick: onclick,
                class: "relative w-[50px] h-[26px] rounded-full bg-gray-300 dark:bg-gray-800 p-1 flex items-center justify-between transition-colors duration-300",
                span {
                    class: "absolute top-[2px] left-[2px] w-[22px] h-[22px] rounded-full bg-white transition-transform duration-300 transform translate-x-0 dark:translate-x-[24px]"
                }
                span {
                    class: "absolute inset-0 flex items-center justify-between px-2 text-xs z-0",
                    i {
                        class: "fas fa-moon text-yellow-400 dark:opacity-100 opacity-0 transition-opacity duration-300"
                    }
                    i {
                        class: "fas fa-sun text-yellow-600 dark:opacity-0 opacity-100 transition-opacity duration-300"
                    }
                }
            }
        }
    }
}

#[component]
fn Product() -> Element {
    let theme_ctx = use_theme();
    let colors = (theme_ctx.resolved_theme)().colors(Some(&(theme_ctx.custom_themes)()));

    let mut products = vec![];

    for i in 0..2 {
        products.push(rsx! {
            div {
                class: "border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden shadow-md transform transition-transform hover:scale-[1.02] duration-300 ease-in-out",
                style: "background: {colors.background}",
                img {
                    src: "https://placehold.co/300",
                    alt: "Product",
                    class: "w-full h-40 object-cover"
                }
                div {
                    class: "p-4",
                    h4 {
                        class: "text-lg font-medium mb-1",
                        style: "color: {colors.text}",
                        "Product {i + 1}"
                    }
                    p {
                        class: "text-sm mb-2",
                        style: "color: {colors.secondary}",
                        "$19.99"
                    }
                    button {
                        class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                        "Buy Now"
                    }
                }
            }
        });
    }

    rsx! {
        ThemeToggle {}
        div {
            class: "mt-4 grid grid-cols-1 md:grid-cols-2 gap-6 p-6",
            style: "background: {colors.background}",
            {products.into_iter()}
        }
    }
}

/// Home page
#[component]
fn Home() -> Element {
    rsx! {
        Product {}
        Echo {}
    }
}

/// Blog page
#[component]
pub fn Blog(id: i32) -> Element {
    rsx! {
        div {
            id: "blog",

            // Content
            h1 { "This is blog #{id}!" }
            p { "In blog #{id}, we show how the Dioxus router works and how URL parameters can be passed as props to our route components." }

            // Navigation links
            Link {
                to: Route::Blog { id: id - 1 },
                "Previous"
            }
            span { " <---> " }
            Link {
                to: Route::Blog { id: id + 1 },
                "Next"
            }
        }
    }
}

/// Shared navbar component.
#[component]
fn Navbar() -> Element {
    rsx! {
        div {
            id: "navbar",
            Link {
                to: Route::Home {},
                "Home"
            }
            Link {
                to: Route::Blog { id: 1 },
                "Blog"
            }
        }

        Outlet::<Route> {}
    }
}

/// Echo component that demonstrates fullstack server functions.
#[component]
fn Echo() -> Element {
    let mut response = use_signal(|| String::new());

    rsx! {
        div {
            id: "echo",
            h4 { "ServerFn Echo" }
            input {
                placeholder: "Type here to echo...",
                oninput:  move |event| async move {
                    let data = echo_server(event.value()).await.unwrap();
                    response.set(data);
                },
            }

            if !response().is_empty() {
                p {
                    "Server echoed: "
                    i { "{response}" }
                }
            }
        }
    }
}

/// Echo the user input on the server.
#[server(EchoServer)]
async fn echo_server(input: String) -> Result<String, ServerFnError> {
    Ok(input)
}
