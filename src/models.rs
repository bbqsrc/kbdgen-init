use serde::{Deserialize, Serialize};
use serde_yaml as yaml;
use std::collections;
// use serde_yaml as yaml;
use std::collections::HashMap;
use strum_macros::{Display, EnumIter, EnumString};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ProjectDesc {
    pub name: String,
    pub description: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Project {
    pub locales: HashMap<String, ProjectDesc>,
    pub author: String,
    pub email: String,
    pub copyright: String,
    pub organisation: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutStrings {
    pub space: String,

    #[serde(rename = "return")]
    pub return_: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct DeriveOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<bool>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize, Display, EnumString)]
pub enum IsoKey {
    E00,
    E01,
    E02,
    E03,
    E04,
    E05,
    E06,
    E07,
    E08,
    E09,
    E10,
    E11,
    E12,
    D01,
    D02,
    D03,
    D04,
    D05,
    D06,
    D07,
    D08,
    D09,
    D10,
    D11,
    D12,
    D13,
    C01,
    C02,
    C03,
    C04,
    C05,
    C06,
    C07,
    C08,
    C09,
    C10,
    C11,
    C12,
    B00,
    B01,
    B02,
    B03,
    B04,
    B05,
    B06,
    B07,
    B08,
    B09,
    B10,
    B11, // Only exists on Brazilian keyboards
    // Needed for Android support:
    A01,
    A02,
    A03,
    A04,
    A05
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
enum MobileModes {
    Default,
    Shift,
}

// #[strum(deserialize_all = "snake_case")]
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModeKey {
    Win,
    Mac,
    Ios,
    Android,
    Chrome,
    X11,
    Desktop,
    Mobile,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]

enum DesktopModes {
    #[serde(rename = "default")]
    Default,
    #[serde(rename = "shift")]
    Shift,
    #[serde(rename = "caps")]
    Caps,
    #[serde(rename = "caps+shift")]
    CapsShift,
    #[serde(rename = "alt")]
    Alt,
    #[serde(rename = "alt+shift")]
    AltShift,
    #[serde(rename = "caps+alt")]
    CapsAlt,
    #[serde(rename = "ctrl")]
    Ctrl,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
enum MacModes {
    #[serde(rename = "cmd")]
    Cmd,
    #[serde(rename = "cmd+shift")]
    CmdShift,
    #[serde(rename = "cmd+alt")]
    CmdAlt,
    #[serde(rename = "cmd+alt+shift")]
    CmdAltShift,
}

/// A layout is defined as a file by the name <locale>.yaml or <locale>.<target>.yaml, and lives in the
/// locales/ directory in the kbdgen project bundle.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Layout {
    /// The display names for the layout, keyed by locale.
    #[serde(rename = "displayNames")]
    pub display_names: HashMap<String, String>,

    /// The different modes.
    pub modes: HashMap<ModeKey, yaml::Value>,

    /// The decimal key. Nominally a '.' or ','.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decimal: Option<String>,

    /// An override for space keys on some OSes. Keyed by target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub space: Option<HashMap<String, yaml::Value>>,

    /// Dead keys present, keyed by layer code.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "deadKeys")]
    pub dead_keys: Option<HashMap<String, yaml::Value>>,

    /// The items to be shown when a key is long-pressed. Values are space separated in one string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub longpress: Option<HashMap<String, String>>,

    /// The chain of inputs necessary to provide an output after a deadkey is pressed. Keyed by each individual input.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transforms: Option<HashMap<String, yaml::Value>>,

    /// Strings to be shown on some OSes
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strings: Option<LayoutStrings>,

    /// Derives
    #[serde(skip_serializing_if = "Option::is_none")]
    pub derive: Option<DeriveOptions>,

    /// Targets...
    #[serde(skip_serializing_if = "Option::is_none")]
    pub targets: Option<HashMap<String, yaml::Value>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutTargetWindows {
    /// The actual locale within Windows, as per their broken ISO 639-3 scheme or secret hardcoded lists.
    pub locale: String,

    /// The language name to be cached, in order to try to mask the ugly ISO code name that often shows.
    #[serde(rename = "languageName")]
    pub language_name: String,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct LayoutTargetAndroid {
    /// Minimum SDK can be specified for a specific layout
    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimumSdk")]
    pub minimum_sdk: Option<u32>,

    /// Styles
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<HashMap<String, yaml::Value>>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetAndroid {
    pub version: String,

    pub build: u32,

    #[serde(rename = "packageId")]
    pub package_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sentryDsn")]
    pub sentry_dsn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "showNumberHints")]
    pub show_number_hints: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "minimumSdk")]
    pub minimum_sdk: Option<u32>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chfst: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyStore")]
    pub key_store: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "keyAlias")]
    pub key_alias: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetIOS {
    pub version: String,

    pub build: u32,

    #[serde(rename = "packageId")]
    pub package_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignId")]
    pub code_sign_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "sentryDsn")]
    pub sentry_dsn: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "aboutDir")]
    pub about_dir: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chfst: Option<bool>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetWindows {
    pub version: String,

    #[serde(rename = "appName")]
    pub app_name: String,

    pub url: String,

    pub uuid: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignPfx")]
    pub code_sign_pfx: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "customLocales")]
    pub custom_locales: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "licensePath")]
    pub license_path: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "readmePath")]
    pub readme_path: Option<String>,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TargetMacOS {
    pub version: String,

    pub build: u32,

    #[serde(rename = "packageId")]
    pub package_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,

    #[serde(rename = "bundleName")]
    pub bundle_name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "teamId")]
    pub team_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[serde(rename = "codeSignId")]
    pub code_sign_id: Option<String>,
}
