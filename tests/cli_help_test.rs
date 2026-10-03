//! Tests for CLI command definitions.
//!
//! Verifies that all commands have help text (about descriptions)
//! so `agentml --help` is useful.

use agentml::cli::Cli;
use clap::{CommandFactory, Parser};

#[test]
fn test_all_commands_have_about() {
    let cmd = Cli::command();
    let subcommands: Vec<_> = cmd.get_subcommands().collect();

    assert!(!subcommands.is_empty(), "CLI should have subcommands");

    for sub in subcommands {
        let name = sub.get_name();
        // Skip the auto-generated help subcommand
        if name == "help" {
            continue;
        }
        assert!(
            sub.get_about().is_some(),
            "Command '{}' is missing an about description",
            name
        );
    }
}

#[test]
fn test_expected_commands_exist() {
    // Verify the core commands documented in the README exist
    let expected = [
        "initialize",
        "validate",
        "run",
        "context",
        "inspect",
        "brief",
        "agents-md",
        "mcp",
        "skill",
        "close",
        "self-check",
        "diff",
        "doctor",
        "completions",
        "version",
        "convert",
    ];

    let cmd = Cli::command();
    let names: Vec<_> = cmd.get_subcommands().map(|s| s.get_name()).collect();

    for expected_name in expected {
        assert!(
            names.contains(&expected_name),
            "Expected command '{}' not found in CLI",
            expected_name
        );
    }
}

#[test]
fn test_init_alias_works() {
    // `init` should be an alias for `initialize`
    let cli = Cli::try_parse_from(["agentml", "init", "--help"]);
    // --help causes a display-help error, which means parsing succeeded
    assert!(cli.is_err());
    let err = cli.unwrap_err();
    assert_eq!(
        err.kind(),
        clap::error::ErrorKind::DisplayHelp,
        "init alias should parse successfully"
    );
}

#[test]
fn test_skill_subcommands_have_about() {
    let cmd = Cli::command();
    let skill_cmd = cmd
        .get_subcommands()
        .find(|s| s.get_name() == "skill")
        .expect("skill command should exist");

    for sub in skill_cmd.get_subcommands() {
        let name = sub.get_name();
        if name == "help" {
            continue;
        }
        assert!(
            sub.get_about().is_some(),
            "Skill subcommand '{}' is missing an about description",
            name
        );
    }
}
