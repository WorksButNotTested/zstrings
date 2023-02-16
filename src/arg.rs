use {
    clap::{Parser, ValueEnum},
    log::LevelFilter,
};

#[derive(ValueEnum, Clone, Debug)]
pub enum Level {
    Off,
    Error,
    Warn,
    Info,
    Debug,
    Trace,
}

impl From<&Level> for LevelFilter {
    fn from(value: &Level) -> LevelFilter {
        match value {
            Level::Off => LevelFilter::Off,
            Level::Error => LevelFilter::Error,
            Level::Warn => LevelFilter::Warn,
            Level::Info => LevelFilter::Info,
            Level::Debug => LevelFilter::Debug,
            Level::Trace => LevelFilter::Trace,
        }
    }
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Radix {
    D,
    X,
}

#[derive(Parser, Debug)]
#[command(
    version = "1.3",
    about = "zstrings",
    long_about = "Tool for zero terminated strings in binary files"
)]
pub struct Opt {
    #[arg(help = "Input file")]
    pub input: String,

    #[arg(
        help = "Log level",
        required = false,
        default_value = "info",
        short,
        long,
        value_enum
    )]
    pub log_level: Level,

    #[arg(
        help = "String Length",
        required = false,
        default_value = "4",
        short = 'n',
        long = "min-len"
    )]
    pub length: usize,

    #[arg(
        help = "Offset Radix",
        required = false,
        short = 't',
        long = "radix",
        value_enum
    )]
    pub radix: Option<Radix>,

    #[arg(
        help = "File Name",
        required = false,
        short = 'f',
        long = "print-file-name"
    )]
    pub file_name: bool,

    #[arg(help = "Alignment", required = false, short = 'a', long = "alignment", value_parser = valid_alignment)]
    pub alignment: Option<usize>,
}

fn valid_alignment(s: &str) -> Result<usize, String> {
    let alignment: usize = s.parse().map_err(|_| format!("`{s}` isn't a number"))?;

    if alignment == 0 {
        Err(format!("Alignment '{alignment:}' must be a power of two"))?;
    }

    if (alignment & (alignment - 1)) == 0 {
        Ok(alignment)
    } else {
        Err(format!("Alignment '{alignment:}' must be a power of two"))
    }
}
