//! Generator for the https://rustsec.org web site
//!
//! Creates markdown versions of each advisory, which are rendered into a final
//! template using markdown.

#![warn(missing_docs, rust_2018_idioms, unused_qualifications)]

use handlebars::Handlebars;
use serde::Serialize;
use std::{fs::File, io::Write, path::PathBuf};

/// Filename of the advisory template
pub const ADVISORY_TEMPLATE_NAME: &str = "advisory.md.hbs";

/// Handlebars template for advisory markdown files (from `advisory.md.hbs`)
pub const ADVISORY_TEMPLATE_STRING: &str = include_str!("../templates/advisory.md.hbs");

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

impl<'a> From<&'a rustsec::Advisory> for AdvisoryParams {
    fn from(advisory: &rustsec::Advisory) -> AdvisoryParams {
        let patched_versions = advisory
            .versions
            .patched
            .iter()
            .map(|req| req.to_string())
            .collect();

        let unaffected_versions = if advisory.versions.unaffected.is_empty() {
            None
        } else {
            Some(
                advisory
                    .versions
                    .unaffected
                    .iter()
                    .map(|req| req.to_string())
                    .collect(),
            )
        };

        let mut summary = advisory
            .metadata
            .description
            .replace('\n', " ")
            .replace("  ", " ");
        summary.retain(|c| match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | ' ' | ',' | '.' => true,
            _ => false,
        });

        let mut tags = vec![advisory.metadata.package.to_string()];
        tags.extend(
            advisory
                .metadata
                .keywords
                .iter()
                .map(|kw| kw.as_str().to_owned()),
        );

        AdvisoryParams {
            id: advisory.metadata.id.to_string(),
            package: advisory.metadata.package.to_string(),
            title: advisory.metadata.title.clone(),
            summary: summary.trim().to_owned(),
            description: advisory.metadata.description.trim().to_owned(),
            date: advisory.metadata.date.as_str().to_owned(),
            tags: tags.join(" "),
            url: advisory.metadata.url.clone(),
            patched_versions,
            unaffected_versions,
        }
    }
}

fn main() {
    let mut handlebars = Handlebars::new();
    handlebars.set_strict_mode(true);
    handlebars.register_escape_fn(handlebars::no_escape);

    handlebars
        .register_template_string(ADVISORY_TEMPLATE_NAME, ADVISORY_TEMPLATE_STRING)
        .unwrap();

    let advisories: Vec<AdvisoryParams> = rustsec::Database::fetch()
        .unwrap()
        .iter()
        .map(AdvisoryParams::from)
        .collect();

    for advisory in &advisories {
        let output_path =
            PathBuf::from("_posts").join(format!("{}-{}.md", advisory.date, advisory.id));

        println!("*** Rendering {}", output_path.display());

        let mut rendered = handlebars.render(ADVISORY_TEMPLATE_NAME, advisory).unwrap();

        // TODO: escaping bug? Find a better solution for (not) escaping these
        // These are getting escaped by handlebars and are double-escaped in the HTML
        // unless removed using the hax below
        rendered = rendered.replace("&lt;", "<").replace("&gt;", ">");

        let mut output_file = File::create(output_path).unwrap();
        output_file.write_all(rendered.as_bytes()).unwrap();
    }

    println!("Done!");
}
