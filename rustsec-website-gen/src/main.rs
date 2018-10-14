extern crate handlebars;
extern crate rustsec;
extern crate serde;
#[macro_use]
extern crate serde_derive;

use handlebars::Handlebars;
use rustsec::{Advisory, AdvisoryDatabase};
use std::{fs::File, io::{Read, Write}, path::PathBuf};

/// Handlebars template for advisory markdown files
pub const ADVISORY_TEMPLATE: &str = "advisory.md.hbs";

/// Parameters to pass to the `advisory.md.hbs` Handlebars template
#[derive(Debug, Serialize)]
pub struct AdvisoryParams {
    /// Advisory ID (i.e. `RUSTSEC-20YY-NNNN`)
    pub id: String,

    /// Package name (i.e. crate name)
    pub package: String,

    /// Title of advisory
    pub title: String,

    /// One-liner summary of the advisory
    pub summary: String,

    /// Full description
    pub description: String,

    /// Advisory date
    pub date: String,

    /// Tags to associate with this advisory
    pub tags: String,

    /// URL for more information
    pub url: Option<String>,

    /// Patched versions
    pub patched_versions: Vec<String>,

    /// Unaffected versions
    pub unaffected_versions: Option<Vec<String>>,
}

impl<'a> From<&'a Advisory> for AdvisoryParams {
    fn from(advisory: &Advisory) -> AdvisoryParams {
        let patched_versions = advisory
            .patched_versions
            .iter()
            .map(|req| req.to_string())
            .collect();

        let unaffected_versions = if advisory.unaffected_versions.is_empty() {
            None
        } else {
            Some(
                advisory
                    .unaffected_versions
                    .iter()
                    .map(|req| req.to_string())
                    .collect(),
            )
        };

        let mut summary = advisory.description.replace('\n', " ").replace("  ", " ");
        summary.retain(|c| match c {
            'A'...'Z' | 'a'...'z' | '0'...'9' | ' ' | ',' | '.' => true,
            _ => false,
        });

        let mut tags = vec![advisory.package.to_string()];
        tags.extend(advisory.keywords.iter().map(|kw| kw.as_str().to_owned()));

        AdvisoryParams {
            id: advisory.id.to_string(),
            package: advisory.package.to_string(),
            title: advisory.title.clone(),
            summary: summary.trim().to_owned(),
            description: advisory.description.trim().to_owned(),
            date: advisory.date.as_str().to_owned(),
            tags: tags.join(", "),
            url: advisory.url.clone(),
            patched_versions,
            unaffected_versions,
        }
    }
}

fn main() {
    let mut template_file = File::open(ADVISORY_TEMPLATE).unwrap();
    let mut template = String::new();
    template_file.read_to_string(&mut template).unwrap();

    let mut handlebars = Handlebars::new();
    handlebars
        .register_template_string(ADVISORY_TEMPLATE, template)
        .unwrap();

    let advisories: Vec<AdvisoryParams> = AdvisoryDatabase::fetch()
        .unwrap()
        .advisories()
        .map(AdvisoryParams::from)
        .collect();

    for advisory in &advisories {
        let output_path = PathBuf::from("_posts").join(format!("{}-{}.md", advisory.date, advisory.id));
        println!("*** Rendering {}", output_path.display());

        let rendered = handlebars.render(ADVISORY_TEMPLATE, advisory).unwrap();
        let mut output_file = File::create(output_path).unwrap();
        output_file.write_all(rendered.as_bytes()).unwrap();
    }

    println!("Done!");
}
