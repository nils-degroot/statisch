//! Module for all configuration code for the site

use std::{fs, path::PathBuf};
use serde::{Serialize, Deserialize};

/// Default link target to use for a application and bookmark
fn default_target() -> String { "_blank".to_string() }

/// Default icon to use for a application
fn default_icon() -> String { "ic:baseline-person".to_string() }

/// Configuration for a application
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Application {
    /// Name of the application, shown right in the row
    pub name: String,
    /// Link of to application
    pub link: String,
    /// Icon used for the application, should conform a [iconify](https://iconify.design) icon name
    #[serde(default = "default_icon")]
    pub icon: String,
    /// Target of the link for the application, defaults to `_blank`
    #[serde(default = "default_target")]
    pub target: String
}

/// Configuration for a Section of a bookmark
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct BookmarkSection {
    /// Name of the section
    name: String,
    /// Bookmark of the section, empty by default
    #[serde(default = "Vec::new")]
    marks: Vec<Bookmark>
}

impl BookmarkSection {
    /// Returns the name of the section
    pub fn name(&self) -> String {
        self.name.clone()
    }

    /// Returns the bookmarks in the section
    pub fn marks(&self) -> Vec<Bookmark> {
        self.marks.clone()
    }
}

/// Configuration for a single bookmark
#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct Bookmark {
    /// Name of the bookmark
    pub name: String,
    /// Link of the bookmark
    pub link: String,
    /// Link target of the bookmark, defaults to `_blank`
    #[serde(default = "default_target")]
    pub target: String
}

/// Possible themes to use, sets the colorscheme for the page
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Theme {
    #[serde(rename = "gruvbox")]
    Gruvbox
}

impl Theme {
    /// Return the theme serialised to css
    pub fn style_header(&self) -> String {
        match self {
            Theme::Gruvbox => include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"), "/resources/themes/gruvbox.css"
            )),
        }.to_string()
    }
}

/// Default theme to use for the page
fn default_theme() -> Theme { Theme::Gruvbox }

/// Default title of the page
fn default_title() -> String { "Statisch".to_string() }

/// Global configuration of the page
#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Config {
    /// Title to use for the page, defaults to `Statisch`
    #[serde(default = "default_title")]
    title: String,
    /// Default theme to use, defaults to gruvbox
    #[serde(default = "default_theme")]
    theme: Theme,
    /// Custom font to use
    font: Option<PathBuf>,
    /// Application in the page
    #[serde(default = "Vec::new")]
    applications: Vec<Application>,
    /// Bookmarks in the page
    #[serde(default = "Vec::new")]
    bookmarks: Vec<BookmarkSection>,
    /// Favicon to copy
    pub favicon: Option<PathBuf>
}

impl Config {
    /// Read a file and page it as config
    ///
    /// # Panics
    ///
    /// Panics if the config file was not valid yaml or missed required keys
    pub fn from_file<S: ToString>(path: S) -> Self {
        let file = fs::read_to_string(path.to_string())
            .expect("Could not find or open the config file");

        serde_yaml::from_str::<Config>(&file)
            .expect("Failed to parse the config file")
    }

    /// Default configuration to use
    pub fn default() -> Self {
        serde_yaml::from_str::<Config>("
            empty: true
        ").unwrap()
    }

    /// Getter for title
    pub fn title(&self) -> String {
        self.title.clone()
    }

    /// Getter for application
    pub fn applications(&self) -> Vec<Application> {
        self.applications.clone()
    }

    /// Getter for bookmarks
    pub fn bookmarks(&self) -> Vec<BookmarkSection> {
        self.bookmarks.clone()
    }

    /// Getter for theme
    pub fn theme(&self) -> Theme {
        self.theme.clone()
    }

    /// Getter for font
    pub fn font(&self) -> Option<PathBuf> {
        self.font.clone()
    }

    /// Translate the font extension to a css legal format
    ///
    /// # Panics
    ///
    /// Panics when the font type is not valid
    pub fn font_format(&self) -> Option<String> {
        Some(match self.font.as_ref()?.extension()?.to_str()? {
            "ttf" => "truetype",
            "woff" => "woff",
            "woff2" => "woff2",
            "eot" => "embedded-opentype",
            _ => panic!("Invalid font format")
        }.to_string())
    }
}
