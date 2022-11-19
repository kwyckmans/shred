use clap::Parser;

#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path);
    let content = match content {
        Ok(content) => content,
        Err(error) => {
            panic!("Could not read provided file {:?}", error)
        }
    };

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("{}", line)
        }
    }
}
