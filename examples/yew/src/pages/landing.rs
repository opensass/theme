use maplit::hashmap;
use sidebar::yew::item::MenuItem;
use sidebar::yew::menu::Menu;
use sidebar::yew::sidebar::Sidebar;
use table_rs::yew::table::Table;
use table_rs::yew::types::{Column, TableClasses};
use theme::yew::use_theme;
use theme::Theme;
use yew::prelude::*;

#[function_component(ThemeToggle)]
pub fn theme_toggle() -> Html {
    let theme_ctx = use_theme();

    let onclick = {
        let theme_ctx = theme_ctx.clone();
        Callback::from(move |_| {
            let new_theme = match *theme_ctx.theme {
                Theme::Light => Theme::Dark,
                Theme::Dark => Theme::Light,
                _ => Theme::Light,
            };
            theme_ctx.set_theme.emit(new_theme);
        })
    };

    html! {
        <div class="flex items-center justify-center">
            <button
                {onclick}
                class="relative w-[50px] h-[26px] rounded-full bg-gray-300 dark:bg-gray-800 p-1 flex items-center justify-between transition-colors duration-300"
            >
                <span
                    class="absolute top-[2px] left-[2px] w-[22px] h-[22px] rounded-full bg-white transition-transform duration-300 transform translate-x-0 dark:translate-x-[24px]"
                />
                <span class="absolute inset-0 flex items-center justify-between px-2 text-xs z-0">
                    <i
                        class="fas fa-moon text-yellow-400 dark:opacity-100 opacity-0 transition-opacity duration-300"
                    />
                    <i
                        class="fas fa-sun text-yellow-600 dark:opacity-0 opacity-100 transition-opacity duration-300"
                    />
                </span>
            </button>
        </div>
    }
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let theme_ctx = use_theme();
    let selected = use_state(|| "Dashboard".to_string());

    let colors = theme_ctx
        .resolved_theme
        .colors(Some(&*theme_ctx.custom_themes));

    html! {
        <>
            <ThemeToggle />
            <div
                class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-4 p-4 bg-gray-100 dark:bg-gray-900 transition-colors duration-300"
                style={format!("background: {}", colors.background)}
            >
                <Sidebar
                    user_name="Ferris"
                    designation="Admin"
                    user_img="/assets/logo.webp"
                    logo_img_url="/assets/logo.webp"
                    on_logout={Callback::from(|_| log::info!("Logout!"))}
                    style="width: 200px; height: 400px;"
                    class="md:col-span-1 bg-gray-100 dark:bg-gray-900 text-black dark:text-white rounded-lg shadow-lg transition-colors duration-300"
                >
                    <Menu
                        sub_heading="Main"
                        class="space-y-2 text-gray-800 dark:text-gray-100"
                        style=""
                    >
                        <MenuItem
                            label="Dashboard"
                            href="/dashboard"
                            icon_html={html! {
                                <span class="text-blue-500">
                                    <i class="fas fa-chart-bar"></i>
                                </span>
                            }}
                            selected={selected.clone()}
                            item_class="hover:bg-gray-200 dark:hover:bg-gray-700 p-2 rounded-md transition-all text-black dark:text-white"
                        />
                        <MenuItem
                            label="Settings"
                            href="/settings"
                            icon_html={html! {
                                <span class="text-gray-600 dark:text-gray-400">
                                    <i class="fas fa-cog"></i>
                                </span>
                            }}
                            selected={selected.clone()}
                            item_class="hover:bg-gray-200 dark:hover:bg-gray-700 p-2 rounded-md transition-all text-black dark:text-white"
                        />
                    </Menu>
                </Sidebar>
                <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                    { for (0..4).map(|i| html! {
                        <div
                            class="p-4 rounded-lg shadow-lg hover:shadow-xl transition-shadow"
                            style={format!("background: {}; transition: background-color 0.3s;", colors.secondary)}
                        >
                            <h3
                                class="text-lg font-semibold mb-2"
                                style={format!("color: {}; transition: color 0.3s;", colors.text)}
                            >
                                { format!("Card {}", i + 1) }
                            </h3>
                            <p
                                class="text-sm"
                                style={format!("color: {}; transition: color 0.3s;", colors.text)}
                            >
                                { "content..." }
                            </p>
                        </div>
                    }) }
                </div>
            </div>
        </>
    }
}

#[function_component(Product)]
pub fn product() -> Html {
    let theme_ctx = use_theme();
    let colors = theme_ctx
        .resolved_theme
        .colors(Some(&*theme_ctx.custom_themes));
    html! {
        <>
            <ThemeToggle />
            <div
                class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-6 p-6"
                style={format!("background: {}", colors.background)}
            >
                { for (0..2).map(|i| html! {
                <div class="border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden shadow-md rounded-lg transform transition-transform hover:scale-[1.02] duration-300 ease-in-out" style={format!("background: {}", colors.background)}>
                    <img src="https://placehold.co/300" alt="Product" class="w-full h-40 object-cover"/>
                    <div class="p-4">
                        <h4 class="text-lg font-medium mb-1" style={format!("color: {}", colors.text)}>{ format!("Product {}", i + 1) }</h4>
                        <p class="text-sm mb-2" style={format!("color: {}", colors.secondary)}>{ "$19.99" }</p>
                        <button class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">{ "Buy Now" }</button>
                    </div>
                </div>
            }) }
            </div>
        </>
    }
}

#[function_component(AdminTable)]
pub fn admin_table() -> Html {
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

    html! {
        <>
            <ThemeToggle />
            <Table data={data} columns={columns} classes={classes} paginate=false search=false />
        </>
    }
}

#[function_component(LandingPage)]
pub fn landing_page() -> Html {
    html! {
        <div class="m-6 min-h-screen flex flex-col items-center justify-center">
            <h1 class="text-3xl font-bold mb-8 text-white">{ "Theme Yew Examples" }</h1>
            <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                <div
                    class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full"
                >
                    <h2 class="text-xl font-bold mb-2">{ "Dashboard (Sidebar RS)" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use sidebar::yew::item::MenuItem;
use sidebar::yew::menu::Menu;
use sidebar::yew::sidebar::Sidebar;
use theme::yew::use_theme;


#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    let theme_ctx = use_theme();
    let selected = use_state(|| "Dashboard".to_string());

    let colors = theme_ctx
        .resolved_theme
        .colors(Some(&*theme_ctx.custom_themes));

    html! {
        <div
            class="mt-4 grid grid-cols-1 md:grid-cols-2 gap-4 p-4 bg-gray-100 dark:bg-gray-900 transition-colors duration-300"
            style={format!("background: {}", colors.background)}
        >
            <Sidebar
                user_name="Ferris"
                designation="Admin"
                user_img="/assets/logo.webp"
                logo_img_url="/assets/logo.webp"
                on_logout={Callback::from(|_| log::info!("Logout!"))}
                style="width: 200px; height: 400px;"
                class="md:col-span-1 bg-gray-100 dark:bg-gray-900 text-black dark:text-white rounded-lg shadow-lg transition-colors duration-300"
            >
                <Menu sub_heading="Main" class="space-y-2 text-gray-800 dark:text-gray-100" style="">
                    <MenuItem
                        label="Dashboard"
                        href="/dashboard"
                        icon_html={html! {
                            <span class="text-blue-500">
                                <i class="fas fa-chart-bar"></i>
                            </span>
                        }}
                        selected={selected.clone()}
                        item_class="hover:bg-gray-200 dark:hover:bg-gray-700 p-2 rounded-md transition-all text-black dark:text-white"
                    />
                    <MenuItem
                        label="Settings"
                        href="/settings"
                        icon_html={html! {
                            <span class="text-gray-600 dark:text-gray-400">
                                <i class="fas fa-cog"></i>
                            </span>
                        }}
                        selected={selected.clone()}
                        item_class="hover:bg-gray-200 dark:hover:bg-gray-700 p-2 rounded-md transition-all text-black dark:text-white"
                    />
                </Menu>
            </Sidebar>
            <div class="grid grid-cols-1 md:grid-cols-2 gap-4">
                { for (0..4).map(|i| html! {
                    <div
                        class="p-4 rounded-lg shadow-lg hover:shadow-xl transition-shadow"
                        style={format!("background: {}; transition: background-color 0.3s;", colors.secondary)}
                    >
                        <h3
                            class="text-lg font-semibold mb-2"
                            style={format!("color: {}; transition: color 0.3s;", colors.text)}
                        >
                            { format!("Card {}", i + 1) }
                        </h3>
                        <p
                            class="text-sm"
                            style={format!("color: {}; transition: color 0.3s;", colors.text)}
                        >
                            { "content..." }
                        </p>
                    </div>
                }) }
            </div>
        </div>
    }
}"# }
                    </pre>
                    <Dashboard />
                </div>
                <div
                    class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full"
                >
                    <h2 class="text-xl font-bold mb-2">{ "Cards" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use theme::yew::use_theme;
use theme::Theme;


#[function_component(Product)]
pub fn product() -> Html {
    let theme_ctx = use_theme();
    let colors = theme_ctx
        .resolved_theme
        .colors(Some(&*theme_ctx.custom_themes));
    html! {
        <div
            class="grid grid-cols-1 md:grid-cols-2 gap-6 p-6"
            style={format!("background: {}", colors.background)}
        >
            { for (0..2).map(|i| html! {
                <div class="border border-gray-300 dark:border-gray-600 rounded-lg overflow-hidden shadow-md rounded-lg transform transition-transform hover:scale-[1.02] duration-300 ease-in-out" style={format!("background: {}", colors.background)}>
                    <img src="https://placehold.co/300" alt="Product" class="w-full h-40 object-cover"/>
                    <div class="p-4">
                        <h4 class="text-lg font-medium mb-1" style={format!("color: {}", colors.text)}>{ format!("Product {}", i + 1) }</h4>
                        <p class="text-sm mb-2" style={format!("color: {}", colors.secondary)}>{ "$19.99" }</p>
                        <button class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700">{ "Buy Now" }</button>
                    </div>
                </div>
            }) }
        </div>
    }
}"# }
                    </pre>
                    <Product />
                </div>
                <div
                    class="flex flex-col items-center bg-gray-200 p-4 rounded-lg shadow-md w-full"
                >
                    <h2 class="text-xl font-bold mb-2">{ "Table (Table RS)" }</h2>
                    <pre
                        class="font-mono text-xs text-white p-4 bg-gray-800 mb-8 rounded-md w-full overflow-x-auto"
                    >
                        { r#"use yew::prelude::*;
use maplit::hashmap;
use table_rs::yew::table::Table;
use table_rs::yew::types::{Column, TableClasses};


#[function_component(AdminTable)]
pub fn admin_table() -> Html {
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

    html! {
        <Table
            data={data}
            columns={columns}
            classes={classes}
            paginate=false
            search=false
        />
    }
}"# }
                    </pre>
                    <AdminTable />
                </div>
            </div>
        </div>
    }
}
