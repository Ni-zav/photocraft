#![deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable)]

fn main() {
    let args = match std::env::args_os().skip(1).map(|arg| arg.into_string()).collect::<Result<Vec<_>, _>>() {
        Ok(args) => args,
        Err(_) => {
            eprintln!("photocraft-cli: command-line arguments must be valid UTF-8");
            std::process::exit(2);
        }
    };
    let code = photocraft_cli::run(&args, &mut std::io::stdout(), &mut std::io::stderr());
    std::process::exit(code);
}
