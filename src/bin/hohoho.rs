use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

use hohoho::Program;

use anyhow::anyhow;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Don't run the program - just convert it to Brainfuck
    #[arg(long = "brainfuck")]
    brainfuck: bool,

    #[arg()]
    file: PathBuf,
}

fn main() -> Result<(), anyhow::Error> {
    let args = Args::parse();

    let mut file = File::open(args.file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let prog = Program::parse(&contents)?.to_brainfuck().map_err(|err| anyhow!(err))?;

    if args.brainfuck {
        print!("{}", prog);
    } else {
        let mut stdin = std::io::stdin();
        let mut stdout = std::io::stdout();
        let mut interpreter = brainfuck::Interpreter::<brainfuck::tape::ArrayTape>::new(prog, &mut stdin, &mut stdout);

        interpreter.run().map_err(|err| anyhow!(err))?;
    }

    Ok(())
}
