use clap::Parser as ClapParser;

#[derive(ClapParser, Debug)]
#[command(name = "terbc", version, about = "Terbium bytecode compiler")]
pub struct Cli {
    pub input: String,

    #[arg(short = 'o', long, default_value = "program.tbc")]
    pub output: String,
}
