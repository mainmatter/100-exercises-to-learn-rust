use anyhow::{Context, Error};
use mdbook::book::Book;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use mdbook::BookItem;

pub struct ExerciseLinker;

impl ExerciseLinker {
    pub fn new() -> ExerciseLinker {
        ExerciseLinker
    }
}

impl Preprocessor for ExerciseLinker {
    fn name(&self) -> &str {
        "exercise-linker"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let config = ctx
            .config
            .get_preprocessor(self.name())
            .context("Failed to get preprocessor configuration")?;
        let key = String::from("exercise_root_url");
        let root_url = config
            .get(&key)
            .context("Failed to get `exercise_root_url`")?;
        let root_url = root_url
            .as_str()
            .context("`exercise_root_url` is not a string")?
            .to_owned();

        book.sections
            .iter_mut()
            .for_each(|i| process_book_item(i, &ctx.renderer, &root_url));
        Ok(book)
    }

    fn supports_renderer(&self, _renderer: &str) -> bool {
        true
    }
}

fn process_book_item(item: &mut BookItem, renderer: &str, root_url: &str) {
    match item {
        BookItem::Chapter(chapter) => {
            chapter.sub_items.iter_mut().for_each(|item| {
                process_book_item(item, renderer, root_url);
            });

            let Some(source_path) = &chapter.source_path else {
                return;
            };
            let source_path = source_path.display().to_string();

            // Ignore non-exercise chapters
            if !source_path.chars().take(2).all(|c| c.is_digit(10)) {
                return;
            }

            let exercise_path = source_path.strip_suffix(".md").unwrap();
            let link_section = format!(
                    "\n## Exercise\n\nThe exercise for this section is located in [`{exercise_path}`]({})\n",
                    format!("{}/{}", root_url, exercise_path)
                );
            chapter.content.push_str(&link_section);

            if renderer == "pandoc" {
                chapter.content.push_str("`\\newpage`{=latex}\n");
            }
        }
        BookItem::Separator => {}
        BookItem::PartTitle(_) => {}
    }
}
