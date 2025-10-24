//! CLI entry point
//!
//! Parses command-line arguments and dispatches to TUI or other components.

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about = "tmux + Claude Code integration manager", long_about = None)]
struct Cli {
    // Future: add subcommands here
}

fn main() -> anyhow::Result<()> {
    let _cli = Cli::parse();

    // Default behavior: launch TUI
    cctmux_tui::run()
}
