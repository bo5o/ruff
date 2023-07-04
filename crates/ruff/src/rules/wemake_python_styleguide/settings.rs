//! Settings for the `wemake-python-styleguide` plugin.

use serde::{Deserialize, Serialize};

use ruff_macros::{CacheKey, CombineOptions, ConfigurationOptions};

#[derive(
    Debug, PartialEq, Eq, Serialize, Deserialize, Default, ConfigurationOptions, CombineOptions,
)]
#[serde(
    deny_unknown_fields,
    rename_all = "kebab-case",
    rename = "WemakePythonStyleguideOptions"
)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
pub struct Options {
    #[option(
        default = r#"2"#,
        value_type = "int",
        example = r#"
            # Minimum number of chars to define a valid variable and module name.
            min-name-length = 3
        "#
    )]
    /// Minimum number of chars to define a valid variable and module name.
    pub min_name_length: Option<usize>,
}

#[derive(Debug, CacheKey)]
pub struct Settings {
    pub min_name_length: usize,
}

impl Default for Settings {
    fn default() -> Self {
        Self { min_name_length: 2 }
    }
}

impl From<Options> for Settings {
    fn from(options: Options) -> Self {
        Self {
            min_name_length: options.min_name_length.unwrap_or_default(),
        }
    }
}

impl From<Settings> for Options {
    fn from(settings: Settings) -> Self {
        Self {
            min_name_length: Some(settings.min_name_length),
        }
    }
}
