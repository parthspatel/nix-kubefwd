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
            // TODO: Implement remove command
            println!("Remove command: skill={}, force={}", skill, force);
            Ok(())
        }

        Commands::List {
            scope,
            enabled,
            disabled,
        } => csm::cli::commands::list::execute(&scope, enabled, disabled, cli.json).await,

        Commands::Show { skill, content } => {
            // TODO: Implement show command
            println!("Show command: skill={}, content={}", skill, content);
            Ok(())
        }

        Commands::Enable { skill } => {
            // TODO: Implement enable command
            println!("Enable command: skill={}", skill);
            Ok(())
        }

        Commands::Disable { skill } => {
            // TODO: Implement disable command
            println!("Disable command: skill={}", skill);
            Ok(())
        }

        Commands::Update {
            skill,
            check,
            dry_run,
        } => {
            // TODO: Implement update command
            println!(
                "Update command: skill={:?}, check={}, dry_run={}",
                skill, check, dry_run
            );
            Ok(())
        }

        Commands::Conflicts { resolve } => {
            // TODO: Implement conflicts command
            println!("Conflicts command: resolve={}", resolve);
            Ok(())
        }

        Commands::Search { query } => {
            // TODO: Implement search command
            println!("Search command: query={}", query);
            Ok(())
        }

        Commands::Config { action } => {
            match action {
                ConfigAction::Get { key } => {
                    // TODO: Implement config get
                    println!("Config get: key={}", key);
                }
                ConfigAction::Set { key, value } => {
                    // TODO: Implement config set
                    println!("Config set: key={}, value={}", key, value);
                }
                ConfigAction::List => {
                    // TODO: Implement config list
                    println!("Config list");
                }
                ConfigAction::Edit => {
                    // TODO: Implement config edit
                    println!("Config edit");
                }
                ConfigAction::Reset { force } => {
                    // TODO: Implement config reset
                    println!("Config reset: force={}", force);
                }
            }
            Ok(())
        }

        Commands::Sync { rebuild, verify } => {
            // TODO: Implement sync command
            println!("Sync command: rebuild={}, verify={}", rebuild, verify);
            Ok(())
        }

        Commands::Export {
            all,
            skill,
            format,
            output,
        } => {
            // TODO: Implement export command
            println!(
                "Export command: all={}, skill={:?}, format={}, output={:?}",
                all, skill, format, output
            );
            Ok(())
        }

        Commands::Import {
            source,
            merge,
            dry_run,
        } => {
            // TODO: Implement import command
            println!(
                "Import command: source={}, merge={}, dry_run={}",
                source, merge, dry_run
            );
            Ok(())
        }

        Commands::Create {
            name,
            from,
            scope,
            edit,
        } => {
            // TODO: Implement create command
            println!(
                "Create command: name={}, from={:?}, scope={}, edit={}",
                name, from, scope, edit
            );
            Ok(())
        }

        Commands::Edit { skill, editor } => {
            // TODO: Implement edit command
            println!("Edit command: skill={}, editor={:?}", skill, editor);
            Ok(())
        }

        Commands::Ui { section } => csm::tui::run(section.as_deref()).await,

        Commands::Doctor { fix } => {
            // TODO: Implement doctor command
            println!("Doctor command: fix={}", fix);
            Ok(())
        }

        Commands::Completions { shell } => {
            // TODO: Implement completions command
            println!("Completions command: shell={}", shell);
            Ok(())
        }

        Commands::Migrate { dry_run, force } => {
            csm::cli::commands::migrate::execute(dry_run, force).await
        }
    }
}
