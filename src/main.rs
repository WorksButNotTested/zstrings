use {
    crate::arg::{Opt, Radix},
    anyhow::Result,
    clap::Parser,
    indicatif::{ProgressBar, ProgressStyle},
    log::{debug, info},
    std::fs::OpenOptions,
    zstrings::ZStrings,
};

const NUM_CHUNKS: usize = 4096;

mod arg;
mod zstrings;

fn main() -> Result<()> {
    let opt = Opt::parse();

    env_logger::builder()
        .filter_level((&opt.log_level).into())
        .format_timestamp(None)
        .format_target(false)
        .init();

    debug!("OPT: {:#?}", opt);

    let f = OpenOptions::new().read(true).open(&opt.input)?;
    let data = unsafe { memmap::MmapOptions::new().map(&f)? };
    let len = f.metadata()?.len() as usize;

    let progress_bar = ProgressBar::new(0);
    progress_bar.set_style(ProgressStyle::default_bar()
    .template(
        "{spinner:.green} [{elapsed_precise:.green}] [{eta_precise:.cyan}] {msg:.magenta} ({percent:.bold}%) [{bar:80.cyan/blue}]",
    )?);

    let zstrings =
        ZStrings::new_parallel(&data[..len], opt.length, NUM_CHUNKS, progress_bar)?.results(opt.alignment);
    debug!("{zstrings:#?}");

    let fname = match opt.file_name {
        true => format!("{}: ", opt.input),
        false => String::default(),
    };

    if let Some(radix) = opt.radix {
        match radix {
            Radix::D => {
                for r in &zstrings {
                    println!("{fname:}{:} {:}", r.offset(), r.string());
                }
            }
            Radix::X => {
                for r in &zstrings {
                    println!("{fname:}0x{:08x} {:}", r.offset(), r.string());
                }
            }
        }
    } else {
        for r in &zstrings {
            println!("{fname:}{:}", r.string());
        }
    }

    info!("DONE");
    Ok(())
}
