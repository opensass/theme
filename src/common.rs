use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;

pub(crate) const SYSTEM_THEME_QUERY: &str = "(prefers-color-scheme: dark)";
pub(crate) const DEFAULT_STORAGE_KEY: &str = "theme";

/// Enum representing browser storage options for persisting the selected theme.
#[derive(Debug, Clone, PartialEq, Default, Copy)]
pub enum StorageType {
    /// Use the browser's `LocalStorage` for persisting data.
    #[default]
    LocalStorage,
    /// Use the browser's `SessionStorage` for persisting data.
    SessionStorage,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ColorTokens {
    pub primary: String,
    pub secondary: String,
    pub background: String,
    pub text: String,
    pub error: Option<String>,
    pub warning: Option<String>,
    pub success: Option<String>,
}

impl ColorTokens {
    pub fn merge_with(&self, other: &ColorTokens) -> ColorTokens {
        ColorTokens {
            primary: other.primary.clone(),
            secondary: other.secondary.clone(),
            background: other.background.clone(),
            text: other.text.clone(),
            error: other.error.clone().or_else(|| self.error.clone()),
            warning: other.warning.clone().or_else(|| self.warning.clone()),
            success: other.success.clone().or_else(|| self.success.clone()),
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        fn is_valid_hex(color: &str) -> bool {
            let color = color.trim_start_matches('#');
            color.len() == 6 || color.len() == 3 && u32::from_str_radix(color, 16).is_ok()
        }

        for (field_name, value) in [
            ("primary", &self.primary),
            ("secondary", &self.secondary),
            ("background", &self.background),
            ("text", &self.text),
        ] {
            if !is_valid_hex(value) {
                return Err(format!("Invalid hex color for '{}': {}", field_name, value));
            }
        }

        Ok(())
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CustomTheme {
    pub name: String,
    pub tokens: ColorTokens,
    pub base: Option<String>,
}

impl CustomTheme {
    pub fn validate(&self) -> Result<(), String> {
        if self.name.trim().is_empty() {
            return Err("Theme name cannot be empty.".to_string());
        }

        self.tokens.validate()?;
        Ok(())
    }

    /// Compose this theme with a base theme if `base` is provided.
    pub fn compose_with_base(
        &self,
        available_themes: &HashMap<String, Rc<CustomTheme>>,
    ) -> Result<ColorTokens, String> {
        if let Some(ref base_name) = self.base {
            if let Some(base_theme) = available_themes.get(base_name) {
                Ok(base_theme.tokens.merge_with(&self.tokens))
            } else {
                Err(format!("Base theme '{}' not found.", base_name))
            }
        } else {
            Ok(self.tokens.clone())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub enum Theme {
    Light,
    Dark,
    #[default]
    System,
    Custom(Rc<CustomTheme>),
}
impl std::str::FromStr for Theme {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "light" | "Light" => Ok(Theme::Light),
            "dark" | "Dark" => Ok(Theme::Dark),
            "system" | "System" => Ok(Theme::System),
            _ => Err(()),
        }
    }
}

impl Theme {
    pub fn as_str(&self) -> String {
        match self {
            Theme::Light => "light".to_string(),
            Theme::Dark => "dark".to_string(),
            Theme::System => "system".to_string(),
            Theme::Custom(custom) => custom.name.clone(),
        }
    }

    pub fn is_dark(&self, system_fallback: Option<bool>) -> bool {
        match self {
            Theme::Dark => true,
            Theme::Light => false,
            Theme::System => system_fallback.unwrap_or(false),
            Theme::Custom(custom) => custom.tokens.background.to_lowercase() != "#ffffff",
        }
    }

    pub fn colors(
        &self,
        available_themes: Option<&HashMap<String, Rc<CustomTheme>>>,
    ) -> ColorTokens {
        match self {
            Theme::Light => ColorTokens {
                primary: "#ffffff".into(),
                secondary: "#f0f0f0".into(),
                background: "#ffffff".into(),
                text: "#000000".into(),
                error: None,
                warning: None,
                success: None,
            },
            Theme::Dark => ColorTokens {
                primary: "#000000".into(),
                secondary: "#1a1a1a".into(),
                background: "#000000".into(),
                text: "#ffffff".into(),
                error: None,
                warning: None,
                success: None,
            },
            Theme::System => ColorTokens {
                primary: "#ffffff".into(),
                secondary: "#f0f0f0".into(),
                background: "#ffffff".into(),
                text: "#000000".into(),
                error: None,
                warning: None,
                success: None,
            },
            Theme::Custom(custom) => {
                if let Some(themes) = available_themes {
                    match custom.compose_with_base(themes) {
                        Ok(tokens) => tokens,
                        Err(_) => custom.tokens.clone(),
                    }
                } else {
                    custom.tokens.clone()
                }
            }
        }
    }
}
