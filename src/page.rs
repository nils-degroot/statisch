//! Generator code for the site

use std::path::PathBuf;
use crate::Config;
use horrorshow::{html, helper::doctype};

/// Structure for generating the page
pub struct Page {
    /// [Config] used when generating the page
    config: Config
}

impl Page {
    /// Creates a new instance of [Page]
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    /// Generate the page and return it as html
    pub fn render(&self) -> String {
        format!("{}", html! {
            : doctype::HTML;
            html {
                head {
                    title: self.config.title();
                    meta(charset="utf-8");
                    meta(name="viewport", content="width=device-width, initial-scale=1, shrink-to-fit=no");
                    link(rel="stylesheet", href=self.stylesheet_name());
                }
                body {
                    div(class="container") {
                        // Applications
                        @ if self.config.applications().len() > 0 {
                            h4: "Applications";
                            div(class="grid") {
                                @ for app in self.config.applications() {
                                    div(class="row") {
                                        span(class="iconify", data-icon=app.icon, data-width="28");
                                        a(href=app.link, target=app.target, class="app-name"): app.name;
                                    }
                                }
                            }
                        }
                        // Bookmarks
                        @ if self.config.bookmarks().len() > 0 {
                            h4(class="mb-0"): "Bookmarks";
                            div(class="grid") {
                                @ for section in self.config.bookmarks() {
                                    div(class="col bookmark-section-header") {
                                        h5(class="mb-1"): section.name();
                                        @ for bookmark in section.marks() {
                                            div(class="bookmark-item") {
                                                a(href=bookmark.link, target=bookmark.target): bookmark.name;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    script(src="https://code.iconify.design/2/2.1.0/iconify.min.js");
                }
            }
        })
    }

    /// Generates the `@font-face` header for css.
    ///
    /// # Returns
    ///
    /// A empty string if no font is specified or the css header
    fn font_face_style(&self) -> String {
        match self.config.font() {
            Some(f) => {
                let extension = f.extension()
                    .expect("Font did not have a file extension");

                format!(r#"@font-face {{
    font-family: "custom-font";
    src: url("custom-font.{}") format("{}");
}}

body {{
    font-family: "custom-font", Fallback, sans-serif;
}}"#, extension.to_str().unwrap(), self.config.font_format().unwrap())
            },
            None => "".to_string(),
        }
    }
    
    /// Get the file location for the copied font
    pub fn font_name(&self) -> Option<String> { 
        match self.config.font() {
            Some(f) => {
                Some(format!("custom-font.{}", f.extension()?.to_str()?))
            },
            None => None
        }
    }

    /// Returns the path to the font according to the config file
    pub fn font(&self) -> Option<PathBuf> {
        self.config.font()
    }

    /// Return the entire stylesheet used in the page
    pub fn stylesheet(&self) -> String {
        format!(
            "{}\n{}\n{}",
            self.config.theme().style_header(),
            self.font_face_style(),
            include_str!(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/resources/style.css"
            ))
        )
    }

    /// Returns the default name for the stylesheet
    pub fn stylesheet_name(&self) -> &'static str { "style.css" }

    /// Path to the favicon
    pub fn favicon(&self) -> Option<&PathBuf> { self.config.favicon.as_ref() }

    /// Export name for the favicon
    pub fn favicon_name(&self) -> &'static str { "favicon.ico" }
}
