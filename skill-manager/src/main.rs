//! Claude Skill Manager CLI Entry Point
//!
//! This is the main entry point for the `csm` command-line tool.

use csm::cli::{Cli, Commands, ConfigAction};
use csm::utils::error::Error;

use clap::Parser;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    // Initialize logging
    init_logging();

    // Parse CLI arguments
    let cli = Cli::parse();

    // Set verbosity
    if cli.verbose > 0 {
        tracing::info!("Verbosity level: {}", cli.verbose);
    }

    // Execute command
    let result = execute_command(cli).await;

    // Handle result
    match result {
        Ok(()) => std::process::exit(0),
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(e.exit_code());
        }
    }
}

/// Initialize logging based on environment and verbosity
fn init_logging() {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("warn"));

    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .without_time()
        .init();
}

/// Execute the parsed command
async fn execute_command(cli: Cli) -> Result<(), Error> {
    match cli.command {
        Commands::Init {
            global,
            local,
            force,
            import_existing,
        } => csm::cli::commands::init::execute(global, local, force, import_existing).await,

        Commands::Add {
            source,
            name,
            scope,
            update_mode,
        } => csm::cli::commands::add::execute(&source, name.as_deref(), &scope, &update_mode).await,

        Commands::Remove { skill, force } => {
            csm::cli::commands::remove::execute(&skill, force || cli.yes).await
        }

        Commands::List {
            scope,
            enabled,
            disabled,
        } => csm::cli::commands::list::execute(&scope, enabled, disabled, cli.json).await,

        Commands::Show { skill, content } => {
            csm::cli::commands::show::execute(&skill, content, cli.json).await
        }

        Commands::Enable { skill } => csm::cli::commands::enable::execute_enable(&skill).await,

        Commands::Disable { skill } => csm::cli::commands::enable::execute_disable(&skill).await,

        Commands::Update {
            skill,
            check,
            dry_run,
        } => csm::cli::commands::update::execute(skill.as_deref(), check, dry_run).await,

        Commands::Conflicts { resolve } => {
            csm::cli::commands::conflicts::execute(resolve, cli.json).await
        }

        Commands::Search { query } => csm::cli::commands::search::execute(&query, cli.json).await,

        Commands::Config { action } => match action {
            ConfigAction::Get { key } => csm::cli::commands::config::execute_get(&key).await,
            ConfigAction::Set { key, value } => {
                csm::cli::commands::config::execute_set(&key, &value).await
            }
            ConfigAction::List => csm::cli::commands::config::execute_list(cli.json).await,
            ConfigAction::Edit => csm::cli::commands::config::execute_edit().await,
            ConfigAction::Reset { force } => {
                csm::cli::commands::config::execute_reset(force || cli.yes).await
            }
        },

        Commands::Sync { rebuild, verify } => {
            csm::cli::commands::sync::execute(rebuild, verify).await
        }

        Commands::Export {
            all,
            skill,
            format,
            output,
        } => {
            csm::cli::commands::export::execute(all, skill.as_deref(), &format, output.as_deref())
                .await
        }

        Commands::Import {
            source,
            merge,
            dry_run,
        } => csm::cli::commands::import::execute(&source, merge, dry_run).await,

        Commands::Create {
            name,
            from,
            scope,
            edit,
        } => csm::cli::commands::create::execute(&name, from.as_deref(), &scope, edit).await,

        Commands::Edit { skill, editor } => {
            csm::cli::commands::edit::execute(&skill, editor.as_deref()).await
        }

        Commands::Ui { section } => csm::tui::run(section.as_deref()).await,

        Commands::Doctor { fix } => csm::cli::commands::doctor::execute(fix).await,

        Commands::Completions { shell } => csm::cli::commands::completions::execute(&shell).await,

        Commands::Migrate { dry_run, force } => {
            csm::cli::commands::migrate::execute(dry_run, force).await
        }
    }
}
