pub const VERSION: &str = env!("CARGO_PKG_VERSION");

use std::fs::File;
use std::fs::OpenOptions;
use std::io::BufWriter;
use std::io::Read;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;

use bfbfe_lang::instructionize;
use bfbfe_lang::lexer;
use bfbfe_transpile::backend;
use bfbfe_transpile::backend::CompilerBackend;
use clap::value_parser;
use clap::Arg;
use clap::ArgAction;
use clap::ArgMatches;
use clap::Command;
use color_eyre::eyre::Result;
use color_eyre::Report;
use const_format::formatcp;
use itertools::Itertools;

fn parse_arguments() -> ArgMatches
{
    Command::new("BFBFE")
        .version(VERSION)
        .author("Reperak")
        .about("All-in-one optimizing Brainfuck transpiler and runtime")
        .long_about(formatcp!(
            "All-in-one optimizing Brainfuck transpiler and runtime

Components:
- bfbfe-lang:\t\t{}
- bfbfe-transpile:\t{}
- bfbfe-ir:\t\t{}",
            bfbfe_lang::VERSION,
            bfbfe_transpile::VERSION,
            bfbfe_ir::VERSION
        ))
        .arg(
            Arg::new("quiet")
                .long("quiet")
                .short('q')
                .required(false)
                .help("Don't display compiler performance metrics")
                .action(ArgAction::SetTrue),
        )
        .arg(
            Arg::new("target")
                .long("target")
                .short('t')
                .required(true)
                .help("Set the target format to compile to")
                .action(ArgAction::Set)
                .value_parser(value_parser!(backend::CompilerBackend)),
        )
        .arg(
            Arg::new("output")
                .short('o')
                .required(true)
                .long_help(
                    "The location to output the transpiled program to\n\nSet this to a single hyphen (-) to set the \
                     output to stdout",
                )
                .action(ArgAction::Set),
        )
        .arg(
            Arg::new("input")
                .required(true)
                .long_help(
                    "The location to read the Brainfuck program from\n\nLike to the output flag, this may be set to a \
                     single hyphen (-) to read from stdin",
                )
                .action(ArgAction::Set),
        )
        .get_matches()
}

fn transpile(arg_quiet: bool, arg_target: &CompilerBackend, arg_output: String, arg_input: String)
    -> Result<(), Report>
{
    // Load input into String
    let mut input = String::new();
    if arg_input == "-" {
        std::io::stdin().read_to_string(&mut input)?;
    } else {
        File::open(arg_input)?.read_to_string(&mut input)?;
    }

    // Load output into BufWriter
    let mut output: BufWriter<Box<dyn Write>> = if arg_output == "-" {
        BufWriter::new(Box::new(std::io::stdout()))
    } else {
        BufWriter::new(Box::new(
            OpenOptions::new()
                .create(true)
                .read(false)
                .append(false)
                .write(true)
                .open(PathBuf::from(arg_output))?,
        ))
    };

    // Tokenizing
    let (tokens, tokenizing_time) = {
        let inst = Instant::now();
        let tokens = lexer::tokenize_whole_program(&input.chars().collect_vec());
        let elapsed = inst.elapsed();
        let tokenizing_time = usize::try_from(elapsed.as_micros())?;
        (tokens, tokenizing_time)
    };

    // Instructionizing
    let (mut block, instructionizing_time) = {
        let inst = Instant::now();
        let block = instructionize::instructionize(&tokens)?;
        let elapsed = inst.elapsed();
        let instructionizing_time = usize::try_from(elapsed.as_micros())?;
        (block, instructionizing_time)
    };

    // Optimizing
    let optimizing_time = {
        let inst = Instant::now();
        block = block.optimize();
        let elapsed = inst.elapsed();
        usize::try_from(elapsed.as_micros())?
    };

    // Compiling
    let (program, compilation_time) = {
        let backend_func = backend::get_compiler_fn(arg_target);
        let inst = Instant::now();
        let program = backend_func(&block);
        let elapsed = inst.elapsed();
        let compilation_time = usize::try_from(elapsed.as_micros())?;
        (program, compilation_time)
    };

    if !arg_quiet {
        use owo_colors::OwoColorize;

        let total_time = tokenizing_time + instructionizing_time + optimizing_time + compilation_time;
        eprintln!("{} in {total_time} \u{3bc}s", "Build complete".bright_green().bold());
        eprintln!("{}\t\t{tokenizing_time} \u{3bc}s", "Tokenizing".underline());
        eprintln!("{}\t{instructionizing_time} \u{3bc}s", "Instructionizing".underline());
        eprintln!("{}\t\t{optimizing_time} \u{3bc}s", "Optimizing".underline());
        eprintln!("{}\t\t{compilation_time} \u{3bc}s", "Compiling".underline());
    }

    output.write_all(program.as_bytes())?;

    Ok(())
}

pub fn main() -> Result<()>
{
    color_eyre::install()?;

    let args = parse_arguments();

    let arg_quiet = *args.get_one::<bool>("quiet").unwrap();
    let arg_target = args.get_one::<CompilerBackend>("target").unwrap();
    let arg_output = args.get_one::<String>("output").unwrap().clone();
    let arg_input = args.get_one::<String>("input").unwrap().clone();

    transpile(arg_quiet, arg_target, arg_output, arg_input)?;

    Ok(())
}
