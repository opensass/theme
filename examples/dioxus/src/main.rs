use dioxus::prelude::*;
use dioxus_logger::tracing;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{Column, TableClasses};
use theme::dioxus::{use_theme, ThemeProvider};
use theme::Theme;

const FAVICON: Asset = asset!("/assets/favicon.ico");
// const MAIN_CSS: Asset = asset!("/assets/styles.css");
const TAILWIND_CSS: Asset = asset!("/assets/output.css");

fn main() {
    dioxus_logger::init(tracing::Level::INFO).expect("failed to init logger");
    tracing::info!("starting app");
    launch(app);
}

fn app() -> Element {
    rsx! {
        document::Script { src: "https://kit.fontawesome.com/8f223ead6e.js" },
        // document::Stylesheet { href: "https://unpkg.com/tailwindcss@2.2.19/dist/tailwind.min.css" },
        document::Stylesheet { href: TAILWIND_CSS },
        document::Link { rel: "icon", href: FAVICON }
        // document::Link { rel: "stylesheet", href: MAIN_CSS }
        ThemeProvider {
            Examples {}
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

#[component]
fn AdminTable() -> Element {
    let data = (1..=5)
        .map(|i| {
            hashmap! {
                "id" => i.to_string(),
                "name" => format!("User {}", i),
                "status" => "Active".to_string(),
                "actions" => "Edit".to_string()
            }
        })
        .collect::<Vec<_>>();

    let columns = vec![
        Column {
            id: "id",
            header: "ID",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "name",
            header: "Name",
            sortable: true,
            ..Default::default()
        },
        Column {
            id: "status",
            header: "Status",
            sortable: false,
            ..Default::default()
        },
        Column {
            id: "actions",
            header: "Actions",
            sortable: false,
            ..Default::default()
        },
    ];

    let classes = TableClasses {
        container: "mt-4 overflow-auto p-6 rounded-lg shadow-lg bg-white text-black dark:bg-black dark:text-white transition-colors duration-300",
        table: "min-w-full text-left border border-gray-200 bg-white text-black dark:border-gray-700 dark:bg-black dark:text-white",
        thead: "bg-gray-100 text-black dark:bg-gray-800 dark:text-white",
        row: "hover:bg-gray-50 dark:hover:bg-gray-800 transition duration-200",
        header_cell: "p-4 border-b border-gray-300 text-sm font-semibold text-black uppercase tracking-wide dark:border-gray-600 dark:text-white",
        body_cell: "p-4 border-b border-gray-200 text-sm text-black dark:border-gray-700 dark:bg-gray-900 dark:text-white",
        ..Default::default()
    };

    rsx! {
        ThemeToggle {}
        Table {
            data: data,
            columns: columns,
            classes: classes,
            paginate: false,
            search: false
        }
    }
}

#[component]
fn Examples() -> Element {
    rsx! {
        div {
            class: "m-6 min-h-screen flex flex-col items-center justify-center",
            h1 {
                class: "text-3xl font-bold mb-8 text-white",
                "Theme Dioxus Examples"
            }
            div {
                class: "grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8",

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Cards" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use theme::dioxus::use_theme;
use theme::Theme;

#[component]
fn Product() -> Element {{
    let theme_ctx = use_theme();
    let colors = (theme_ctx.resolved_theme)().colors(Some(&(theme_ctx.custom_themes)()));

    let mut products = vec![];

    for i in 0..2 {{
        products.push(rsx! {{
            div {{
                class: "border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden shadow-md transform transition-transform hover:scale-[1.02] duration-300 ease-in-out",
                style: "background: {{colors.background}}",
                img {{
                    src: "https://placehold.co/300",
                    alt: "Product",
                    class: "w-full h-40 object-cover"
                }}
                div {{
                    class: "p-4",
                    h4 {{
                        class: "text-lg font-medium mb-1",
                        style: "color: {{colors.text}}",
                        "Product {{i + 1}}"
                    }}
                    p {{
                        class: "text-sm mb-2",
                        style: "color: {{colors.secondary}}",
                        "$19.99"
                    }}
                    button {{
                        class: "px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700",
                        "Buy Now"
                    }}
                }}
            }}
        }});
    }}

    rsx! {{
        ThemeToggle {{}}
        div {{
            class: "mt-4 grid grid-cols-1 md:grid-cols-2 gap-6 p-6",
            style: "background: {{colors.background}}",
            {{products.into_iter()}}
        }}
    }}
}}"#
                    }
                    Product {}
                }

                div {
                    class: "flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full",
                    h2 { class: "text-xl font-bold mb-2", "Table (Table RS)" }
                    pre {
                        class: "font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto",
                        r#"use dioxus::prelude::*;
use maplit::hashmap;
use table_rs::dioxus::table::Table;
use table_rs::dioxus::types::{{Column, TableClasses}};

#[component]
fn AdminTable() -> Element {{
    let data = (1..=5).map(|i| {{
        hashmap! {{
            "id" => i.to_string(),
            "name" => format!("User {{}}", i),
            "status" => "Active".to_string(),
            "actions" => "Edit".to_string()
        }}
    }}).collect::<Vec<_>>();

    let columns = vec![
        Column {{ id: "id", header: "ID", sortable: true, ..Default::default() }},
        Column {{ id: "name", header: "Name", sortable: true, ..Default::default() }},
        Column {{ id: "status", header: "Status", sortable: false, ..Default::default() }},
        Column {{ id: "actions", header: "Actions", sortable: false, ..Default::default() }},
    ];

    let classes = TableClasses {{
        container: "mt-4 overflow-auto p-6 rounded-lg shadow-lg bg-white text-black dark:bg-black dark:text-white transition-colors duration-300",
        table: "min-w-full text-left border border-gray-200 bg-white text-black dark:border-gray-700 dark:bg-black dark:text-white",
        thead: "bg-gray-100 text-black dark:bg-gray-800 dark:text-white",
        row: "hover:bg-gray-50 dark:hover:bg-gray-800 transition duration-200",
        header_cell: "p-4 border-b border-gray-300 text-sm font-semibold text-black uppercase tracking-wide dark:border-gray-600 dark:text-white",
        body_cell: "p-4 border-b border-gray-200 text-sm text-black dark:border-gray-700 dark:bg-gray-900 dark:text-white",
        ..Default::default()
    }};

    rsx! {{
        ThemeToggle {{}}
        Table {{
            data: data,
            columns: columns,
            classes: classes,
            paginate: false,
            search: false
        }}
    }}
}}"#
                    }
                    AdminTable {}
                }
            }
        }
    }
}
