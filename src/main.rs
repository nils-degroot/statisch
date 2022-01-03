//! Binary entrypoint for `statisch`

mod config;
mod page;

use std::{
    path::Path,
    fs::{File, self}, io::Write
};
use clap::{App, Arg};
use crate::{config::Config, page::Page};

/// Entry function
fn main() {
    let matches = App::new("Statisch")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Nils de Groot <nils@peeko.nl>")
        .about("A staticly generated site for your server")
        .arg(Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Configuration to use")
            .takes_value(true))
        .arg(Arg::with_name("output-dir")
            .short("o")
            .long("output-dir")
            .value_name("DIR")
            .help("Directory to output the content to")
            .takes_value(true))
        .arg(Arg::with_name("dump-html")
            .long("dump-html")
            .help("Dump the html to stdout"))
        .get_matches();

    // Parse the config
    let config = matches.value_of("config")
        .map(|p| Config::from_file(p))
        .unwrap_or_else(|| Config::default());

    // Dump the page to stdout
    if matches.is_present("dump-html") {
        println!("{}", Page::new(config).render());
        return
    }

    // Write the content to the given output directory
    if let Some(path) = matches.value_of("output-dir") {
        let path = Path::new(path);

        if path.exists() && path.is_dir() {
            let page = Page::new(config);

            // Create the index page
            File::create(path.join("index.html")).unwrap()
                .write_all(page.render().as_bytes()).unwrap();

            // Create style sheet
            File::create(path.join(page.stylesheet_name())).unwrap()
                .write_all(page.stylesheet().as_bytes()).unwrap();

            if let Some(favicon) = page.favicon() {
                // Copy favicon
                fs::copy(favicon, path.join(page.favicon_name()))
                    .expect("Failed to copy favicon");
            }

            if let (Some(font), Some(font_name)) = (page.font(), page.font_name()) {
                // Copy font
                fs::copy(font, path.join(font_name))
                    .expect("Failed to copy font");
            }
        } else {
            panic!("Target directory is not a directory or does not exist");
        }

        return
    }

    panic!("No export mode selected");
}
