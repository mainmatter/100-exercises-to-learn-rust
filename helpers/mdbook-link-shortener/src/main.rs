use clap::Parser;
use mdbook_link_shortener::LinkShortener;
use mdbook_preprocessor::{Preprocessor, MDBOOK_VERSION};
use semver::{Version, VersionReq};
use std::io;
use std::process;

#[derive(clap::Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    sub: Option<SubCommand>,
}

#[derive(clap::Parser, Debug)]
pub enum SubCommand {
    #[clap(name = "supports")]
    Supports(Supports),
}

#[derive(clap::Parser, Debug)]
pub struct Supports {
    #[arg(long)]
    renderer: String,
}

fn main() -> Result<(), anyhow::Error> {
    let cli = Cli::parse();
    let preprocessor = LinkShortener::new();

    if let Some(SubCommand::Supports(Supports { renderer })) = cli.sub {
        let code = if preprocessor
            .supports_renderer(&renderer)
            .expect("Failed to check renderer support")
        {
            0
        } else {
            1
        };
        process::exit(code);
    }

    handle_preprocessing(&preprocessor)?;

    Ok(())
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> anyhow::Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    let book_version = Version::parse(&ctx.mdbook_version)?;
    let version_req = VersionReq::parse(MDBOOK_VERSION)?;

    if !version_req.matches(&book_version) {
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}
