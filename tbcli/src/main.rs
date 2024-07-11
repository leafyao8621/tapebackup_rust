mod app;

use std::path::PathBuf;

use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug, PartialEq, Eq)]
enum Mode {
    READ,
    WRITE
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(index=1)]
    mode: Mode,
    #[clap(index=2)]
    device: PathBuf,
    #[clap(short='p', long="path")]
    path: Option<PathBuf>,
    #[clap(short='w', long="write-protect", action)]
    write_protect: bool
}

fn main() {
    let args = Args::parse();
    dbg!(&args);
    match args.mode {
        Mode::WRITE =>
            app::writer::write_to_device(
                args.device.into_os_string().into_string().unwrap(),
                args.path.unwrap().into_os_string().into_string().unwrap(),
                args.write_protect
            ),
        Mode::READ => println!("Not Implemented")
    }
}
