use dt::cli::Cli;

#[test]
fn test_parse() {
    let args = vec!["dt", "--sync", "--reload", "--open", "command"];
    let cli = Cli::parse(args.iter());

    assert_eq!(cli.sync, true);
    assert_eq!(cli.reload, true);
    assert_eq!(cli.open, true);
    assert_eq!(cli.command, Some("command".to_string()));
}
