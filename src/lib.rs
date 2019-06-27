#[macro_use]
extern crate serde_derive;

use std::path::{Path, PathBuf};
use std::process::Command;

pub mod ir;
mod models;

fn cldr_dir() -> PathBuf {
    directories::ProjectDirs::from("", "", "kbdgen")
        .expect("project dir")
        .cache_dir()
        .join("cldr")
}

#[derive(Debug, Deserialize)]
pub struct Name {
    pub value: String,
}

#[derive(Debug, Deserialize)]
pub struct Map {
    pub iso: String,
    pub to: String,
    pub transform: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct KeyMap {
    #[serde(rename = "map")]
    pub keys: Vec<Map>,
    pub modifiers: Option<String>,
    #[serde(rename = "longPress")]
    pub long_press: Option<String>,
}

// impl KeyMap {
//     pub fn is_mobile(&self) -> bool {
//         if self.maps.len() == 0 {
//             return false;
//         }

//         let map = &self.maps[0];
//         if map.len() == 0 {
//             return false;
//         }

//     }
// }

#[derive(Debug, Deserialize)]
pub struct Version {
    pub platform: String,
    pub number: String,
}

#[derive(Debug, Deserialize)]
pub struct Transform {
    pub from: String,
    pub to: String,
}

#[derive(Debug, Deserialize)]
pub struct Transforms {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "transform")]
    pub values: Vec<Transform>,
}

#[derive(Debug, Deserialize)]
pub struct Names {
    #[serde(rename = "name")]
    pub values: Vec<Name>,
}

#[derive(Debug, Deserialize)]
pub struct Keyboard {
    pub locale: String,
    pub names: Vec<Names>,
    pub version: Version,
    #[serde(rename = "keyMap")]
    pub key_maps: Vec<KeyMap>,
    pub transforms: Option<Vec<Transforms>>,
    // <settings transformFailure="omit" transformPartial="hide"/>
}

impl Keyboard {
    pub fn is_mobile() -> bool {
        unimplemented!()
    }
}

pub fn update_cldr_repo() {
    if !cldr_dir().exists() {
        println!("Downloading CLDR repo…");
        let mut command = Command::new("git")
            .args(&[
                "clone",
                "--depth",
                "1",
                "https://github.com/unicode-org/cldr",
                &*cldr_dir().to_string_lossy(),
            ])
            .spawn()
            .expect("git clone failed");
        command.wait().unwrap();
    } else {
        println!("Updating CLDR repo…");
        let mut command = Command::new("git")
            .current_dir(cldr_dir())
            .args(&["pull"])
            .spawn()
            .expect("git pull failed");
        command.wait().unwrap();
    }
}

use std::collections::BTreeMap;

pub fn select_base_locale() -> Option<(String, BTreeMap<String, Vec<String>>)> {
    let kbd_path = cldr_dir().join("keyboards");
    let set: BTreeMap<String, BTreeMap<String, Vec<String>>> = BTreeMap::new();
    let mut locale_map = globwalk::GlobWalkerBuilder::new(kbd_path, "*.xml")
        .build()
        .unwrap()
        .into_iter()
        .filter_map(Result::ok)
        .filter(|entry| {
            !entry
                .path()
                .file_stem()
                .unwrap()
                .to_string_lossy()
                .starts_with("_")
        })
        .fold(set, |mut acc, cur| {
            let tag = (&*cur.path().file_stem().unwrap().to_string_lossy())
                .split("-t")
                .next()
                .unwrap()
                .to_string();
            let kbd_os = (&*cur
                .path()
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_string_lossy())
                .to_string();
            let entry = acc
                .entry(tag)
                .or_insert(BTreeMap::new())
                .entry(kbd_os)
                .or_insert(vec![]);
            (*entry).push(
                cur.path()
                    .file_name()
                    .unwrap()
                    .to_string_lossy()
                    .to_string(),
            );
            acc
        });
    let mut locales = locale_map.iter().collect::<Vec<_>>();
    locales.sort();

    let options = skim::SkimOptionsBuilder::default()
        .prompt(Some("Which locale to use as base? "))
        .exact(true)
        .ansi(true)
        .build()
        .unwrap();

    let cyan = console::Style::new().cyan().dim();
    let text = locales
        .iter()
        .map(|(locale, items)| {
            let x = items.keys().map(|x| &**x).collect::<Vec<_>>();
            format!("{}   {}", locale, cyan.apply_to(x.join(", ")))
        })
        .collect::<Vec<_>>()
        .join("\n")
        .as_bytes()
        .to_owned();
    let cur = std::io::Cursor::new(text);

    let result = skim::Skim::run_with(&options, Some(Box::new(cur)))?;
    let result = result
        .selected_items
        .first()?
        .get_text()
        .split("   ")
        .next()
        .unwrap();

    Some((result.to_string(), locale_map.remove(result).unwrap()))
}

pub fn parse_path(os: &str, file: &str) -> Keyboard {
    let fn_ = cldr_dir().join("keyboards").join(os).join(file);
    println!("{:?}", &fn_);
    let f = std::fs::File::open(fn_).unwrap();
    let kbd: Keyboard = serde_xml_rs::from_reader(f).unwrap();
    kbd
}
