use anyhow::{Context, Error};
use bimap::BiHashMap;
use itertools::Itertools;
use mdbook::book::{Book, Chapter};
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;
use std::collections::{BTreeMap, BTreeSet};
use std::fs::File;
use std::path::PathBuf;
use std::str::FromStr;

pub struct LinkShortener;

struct AliasGenerator {
    cursors: [usize; 3],
}

impl AliasGenerator {
    const ALPHABET: &'static [u8] = b"f2z4x6v8bnm3q5w7e9rtyuplkshgjdca";

    fn new() -> AliasGenerator {
        AliasGenerator { cursors: [0, 0, 0] }
    }

    /// Generate a 4 alphanumeric long alias, starting from "aaaa" and incrementing by one each time
    /// until "9999", using only lowercase letters and numbers.
    /// We skip ambiguous characters like "0", "o", "1", "l".
    fn next(&mut self) -> String {
        let mut alias = String::with_capacity(4);
        for cursor in &mut self.cursors {
            alias.push(Self::ALPHABET[*cursor] as char);
        }

        for cursor in self.cursors.iter_mut().rev() {
            if *cursor == Self::ALPHABET.len() - 1 {
                *cursor = 0;
            } else {
                *cursor += 1;
                break;
            }
        }

        alias
    }

    /// Generate a unique alias that is not already used by the `link2alias` map.
    fn next_until_unique(&mut self, link2alias: &BiHashMap<String, String>) -> String {
        let mut alias = self.next();
        while link2alias.contains_right(&alias) {
            alias = self.next();
        }
        alias
    }
}

impl LinkShortener {
    pub fn new() -> LinkShortener {
        LinkShortener
    }
}

impl Preprocessor for LinkShortener {
    fn name(&self) -> &str {
        "link-shortener"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = ctx
            .config
            .get_preprocessor(self.name())
            .context("Failed to get preprocessor configuration")?;
        let root_url = {
            let root_url = config.get("base_url").context("Failed to get `base_url`")?;
            root_url
                .as_str()
                .context("`base_url` is not a string")?
                .to_owned()
        };
        let mapping = {
            let mapping = config.get("mapping").context("Failed to get `mapping`")?;
            let mapping = mapping
                .as_str()
                .context("`mapping` is not a string")?
                .to_owned();
            PathBuf::from_str(&mapping).context("Failed to parse `mapping` as a path")?
        };
        let mut link2alias = {
            match File::open(&mapping) {
                Ok(file) => {
                    serde_json::from_reader(file).context("Failed to parse existing mapping")?
                }
                Err(e) => {
                    if e.kind() == std::io::ErrorKind::NotFound {
                        BiHashMap::new()
                    } else {
                        return Err(e).context("Failed to open existing mapping");
                    }
                }
            }
        };
        let verify = config
            .get("verify")
            .context("Failed to get `verify`")?
            .as_bool()
            .context("`verify` is not a boolean")?;
        // Env var overrides config
        let verify = std::env::var("LINK_SHORTENER_VERIFY")
            .map(|v| v == "true")
            .unwrap_or(verify);

        let mut alias_gen = AliasGenerator::new();

        book.sections.iter_mut().for_each(|i| {
            if let BookItem::Chapter(c) = i {
                c.content = replace_anchors(c, &root_url, &mut alias_gen, &mut link2alias, verify)
                    .expect("Error converting links for chapter");
                for i in c.sub_items.iter_mut() {
                    if let BookItem::Chapter(sub_chapter) = i {
                        sub_chapter.content = replace_anchors(
                            sub_chapter,
                            &root_url,
                            &mut alias_gen,
                            &mut link2alias,
                            verify,
                        )
                        .expect("Error converting links for subchapter");
                    }
                }
            }
        });

        if !verify {
            std::fs::create_dir_all(mapping.parent().expect("Mapping file path has no parent"))?;
            let mut file = File::create(&mapping).context("Failed to upsert mapping file")?;
            let ordered = link2alias.iter().collect::<BTreeMap<_, _>>();
            serde_json::to_writer_pretty(&mut file, &ordered)?;
        }

        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

fn replace_anchors(
    chapter: &mut Chapter,
    root_url: &str,
    alias_gen: &mut AliasGenerator,
    link2alias: &mut BiHashMap<String, String>,
    verify: bool,
) -> Result<String, anyhow::Error> {
    use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag};
    use pulldown_cmark_to_cmark::cmark;

    let mut buf = String::with_capacity(chapter.content.len());

    let mut unshortened_links = BTreeSet::new();
    let events = Parser::new_ext(&chapter.content, Options::all())
        .map(|e| {
            let Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            }) = &e
            else {
                return e;
            };

            match link_type {
                LinkType::Autolink
                | LinkType::Shortcut
                | LinkType::Inline
                | LinkType::Reference
                | LinkType::Collapsed => {
                    if dest_url.starts_with("http") {
                        let alias = if let Some(alias) = link2alias.get_by_left(dest_url.as_ref()) {
                            alias.to_owned()
                        } else {
                            if verify {
                                unshortened_links.insert(dest_url.to_string());
                                return e;
                            }
                            let alias = alias_gen.next_until_unique(&link2alias);
                            alias
                        };
                        link2alias.insert(dest_url.to_string(), alias.clone());

                        Event::Start(Tag::Link {
                            link_type: link_type.to_owned(),
                            dest_url: CowStr::from(format!(
                                "{root_url}/{alias}",
                                root_url = root_url,
                                alias = alias
                            )),
                            title: title.clone(),
                            id: id.clone(),
                        })
                    } else {
                        e
                    }
                }
                LinkType::Email
                | LinkType::ReferenceUnknown
                | LinkType::CollapsedUnknown
                | LinkType::ShortcutUnknown => e,
            }
        })
        .collect_vec();

    if verify && !unshortened_links.is_empty() {
        let unshortened_links = unshortened_links.iter().join(", ");
        return Err(anyhow::anyhow!(
            "The following links are not shortened: {unshortened_links}\nRun again with `LINK_SHORTENER_VERIFY=false` to update the mapping \
            with the shortened links."
        ));
    }

    cmark(events.into_iter(), &mut buf)
        .map(|_| buf)
        .map_err(|err| anyhow::anyhow!("Markdown serialization failed: {err}"))
}
