use std::fs::write;
use std::path::Path;
use std::process::Command;
use clap::Parser;
use nanoid::nanoid;

#[derive(Parser, Debug)]
#[clap(version, about = "Quickly compile LaTeX to beautiful PNGs")]
struct Args {
    #[clap(value_parser)]
    code: String,

    #[clap(short = 'f', long = "color")]
    color: Option<String>,

    #[clap(short = 'm', long = "margin")]
    margin: Option<u8>,

    #[clap(short = 'v', long = "verbose")]
    verbose: bool
}

fn main()  {
    let args = Args::parse();
    let id = &format!("fastex-{}", nanoid!(7));
    let filename = id.to_string() + ".tex";

    let verbose = args.verbose;
    let color = &args.color.unwrap_or("black".to_string());
    let margin = &args.margin.unwrap_or(3);
    
    print_verbose("Creating document...", verbose);
    let template = include_str!("../template.tex");
    let code = template
        .replace("#CONTENT#", &args.code)
        .replace("#COLOR#", color)
        .replace("#MARGIN#", &margin.to_string());

    write(&filename, code).expect("Something went wrong while trying to create a file.");

    print_verbose("Compiling document...", verbose);
    Command::new("lualatex").args(["-shell-escape", &filename]).output().expect("Something went wrong while trying to run LuaLaTeX");

    let extensions = vec!["tex", "pdf", "log", "aux"];
    let mut to_remove: Vec<String> = extensions.iter().map(|ext| format!("{}.{}", &id, &ext)).collect();
    if Path::new("texput.log").exists() { to_remove.push("texput.log".to_string()); }

    print_verbose("Deleting auxilary files...", verbose);
    Command::new("rm").args(to_remove).spawn().expect("Something went wrong while deleting file");

    println!("✔ Saved to {}.png!", &id);
}

fn print_verbose(str: &str, verbose: bool) {
    if verbose { println!("{}", str); }
}