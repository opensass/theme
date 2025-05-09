# Theme Yew Usage

Adding Theme to your project is simple:

1. Make sure your project is set up with **Yew**. Follow their [Getting Started Guide](https://yew.rs/docs/getting-started/introduction) for setup instructions.

1. Add the Theme component to your dependencies by including it in your `Cargo.toml` file:

   ```sh
   cargo add theme --features=yew
   ```

1. Import the `ThemeProvider` component into your Yew component and start using it in your app.

## 🛠️ Usage

Follow these steps to integrate `theme` into your Yew application:

### 1. Import the Required Components

Import the `ThemeProvider` and related types into your Yew project:

```rust
use yew::prelude::*;
use theme::yew::ThemeProvider;
use theme::{Theme, StorageType};
```

### 2. Define Custom Themes (Optional)

You can define and register custom themes using the `CustomTheme` type:

```rust
use std::collections::HashMap;
use std::rc::Rc;
use theme::{CustomTheme, ColorTokens};

let mut custom_themes = HashMap::new();

custom_themes.insert(
    "solarized".to_string(),
    Rc::new(CustomTheme {
        name: "solarized".to_string(),
        base: None, // or Some("light".to_string()) if you want to inherit
        tokens: ColorTokens {
            primary: "#268bd2".to_string(),
            secondary: "#2aa198".to_string(),
            background: "#fdf6e3".to_string(),
            text: "#657b83".to_string(),
            error: Some("#dc322f".to_string()),
            warning: Some("#cb4b16".to_string()),
            success: Some("#859900".to_string()),
        },
    }),
);
```

### 3. Wrap Your App with the `ThemeProvider`

Wrap your main app component inside the `ThemeProvider` to provide theme context and behavior to your app:

```rust
use std::rc::Rc;
use yew::prelude::*;
use theme::yew::ThemeProvider;
use std::collections::HashMap;
use theme::{Theme, StorageType, CustomTheme, ColorTokens};

#[function_component(App)]
pub fn app() -> Html {
    let mut custom_themes = HashMap::new();
    custom_themes.insert(
        "solarized".to_string(),
        Rc::new(CustomTheme {
            name: "solarized".to_string(),
            base: None,
            tokens: ColorTokens {
                primary: "#268bd2".to_string(),
                secondary: "#2aa198".to_string(),
                background: "#fdf6e3".to_string(),
                text: "#657b83".to_string(),
                error: Some("#dc322f".to_string()),
                warning: Some("#cb4b16".to_string()),
                success: Some("#859900".to_string()),
            },
        }),
    );

    html! {
        <ThemeProvider
            default_theme={Theme::System}
            storage_type={StorageType::LocalStorage}
            storage_name={"theme"}
            custom_themes={custom_themes}
            forced_theme={None}
        >
            <MainApp />
        </ThemeProvider>
    }
}

#[function_component(MainApp)]
pub fn main_app() -> Html {
    html! {
        <h1>{ "Welcome to the themed app!" }</h1>
    }
}

fn main() {
    // yew::Renderer::<App>::new().render();
}
```

### 4. Access the Theme Context with the `use_theme` Hook

Use the `use_theme` hook to access the current theme, resolved theme, and control functions within your components:

```rust
use yew::prelude::*;
use theme::yew::use_theme;
use theme::Theme;

#[function_component(MainApp)]
pub fn main_app() -> Html {
    let ctx = use_theme();

    let onclick = {
        let set_theme = ctx.set_theme.clone();
        Callback::from(move |_| set_theme.emit(Theme::Dark))
    };

    html! {
        <div>
            <h2>{ format!("Current Theme: {:?}", *ctx.resolved_theme) }</h2>
            <button onclick={onclick}>{ "Switch to Dark Theme" }</button>
        </div>
    }
}

fn main() {
    // yew::Renderer::<MainApp>::new().render();
}
```

## 🔧 Props

### `ThemeProviderProps` Props

#### Main Props

| Property        | Type                               | Description                                                         | Default         |
| --------------- | ---------------------------------- | ------------------------------------------------------------------- | --------------- |
| `default_theme` | `Theme`                            | The theme to use if nothing is stored or detected.                  | `Theme::System` |
| `storage_type`  | `StorageType`                      | Whether to persist the theme in `LocalStorage` or `SessionStorage`. | `LocalStorage`  |
| `storage_name`  | `&'static str`                     | Key name for storing the selected theme in browser storage.         | `"theme"`       |
| `forced_theme`  | `Option<Theme>`                    | Overrides all other theme logic if provided.                        | `None`          |
| `custom_themes` | `HashMap<String, Rc<CustomTheme>>` | Map of user-defined themes. Can be applied and previewed.           | `{}`            |
| `children`      | `Html`                             | Child components that will have access to the theme context.        | **Required**    |

#### Behavioral Props

| Property           | Type                        | Description                                                          | Default |
| ------------------ | --------------------------- | -------------------------------------------------------------------- | ------- |
| `reset_to_system`  | `Callback<()>`              | Reverts the theme to follow the system theme.                        | no-op   |
| `apply_preview`    | `Callback<Theme>`           | Applies a temporary theme preview (doesn't persist or update state). | no-op   |
| `set_custom_theme` | `Callback<Rc<CustomTheme>>` | Adds a new custom theme if it passes validation.                     | no-op   |

## 💡 Notes

1. **Auto System Theme Support**: When `Theme::System` is used, the component tracks `prefers-color-scheme` and switches between light and dark automatically based on system settings.

1. **Time-Based Theme Switching**: If no preference is stored, `Theme::System` will fall back to light mode during 7 AM - 6:59 PM and dark mode otherwise.

1. **Forced Theme**: When `forced_theme` is provided, it overrides all system, storage, or runtime theme choices, effectively locking the app to that theme.

1. **Custom Themes**: Add your own themes and styles dynamically. Each must implement the `validate()` method to ensure it's structured correctly.

1. **Tailwind Compatibility (v3 or lower)**: This provider works with Tailwind CSS's `data-theme=` and `class=` bindings, making it compatible with libraries like [DaisyUI](https://daisyui.com). It sets:

   - `data-theme`
   - `class`
   - `style="color-scheme:..."` on the root HTML element.

1. **Storage Syncing**: Theme changes are synced across tabs and windows using the `storage` event.

1. **Easy API**: Use `set_theme`, `reset_to_system`, or `apply_preview` to control appearance from any component.

1. **Hooks First**: Just use `use_theme()` to access all theme information and actions within your components.
