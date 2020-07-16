extern crate serial;
extern crate structopt;
extern crate xmodem;
#[macro_use]
extern crate structopt_derive;

use std::path::PathBuf;
use std::time::Duration;

use serial::core::{
    BaudRate, CharSize, FlowControl, Parity, PortSettings, SerialDevice, SerialPortSettings,
    StopBits,
};

use structopt::StructOpt;
use xmodem::{Progress, Xmodem};

mod parsers;

use parsers::{parse_baud_rate, parse_flow_control, parse_stop_bits, parse_width};

use std::fs::File;
use std::io::{self, stdin};

#[derive(StructOpt, Debug)]
#[structopt(about = "Write to TTY using the XMODEM protocol by default.")]
struct Opt {
    #[structopt(
        short = "i",
        help = "Input file (defaults to stdin if not set)",
        parse(from_os_str)
    )]
    input: Option<PathBuf>,

    #[structopt(
        short = "b",
        long = "baud",
        parse(try_from_str = "parse_baud_rate"),
        help = "Set baud rate",
        default_value = "115200"
    )]
    baud_rate: BaudRate,

    #[structopt(
        short = "t",
        long = "timeout",
        parse(try_from_str),
        help = "Set timeout in seconds",
        default_value = "10"
    )]
    timeout: u64,

    #[structopt(
        short = "w",
        long = "width",
        parse(try_from_str = "parse_width"),
        help = "Set data character width in bits",
        default_value = "8"
    )]
    char_width: CharSize,

    #[structopt(help = "Path to TTY device", parse(from_os_str))]
    tty_path: PathBuf,

    #[structopt(
        short = "f",
        long = "flow-control",
        parse(try_from_str = "parse_flow_control"),
        help = "Enable flow control ('hardware' or 'software')",
        default_value = "none"
    )]
    flow_control: FlowControl,

    #[structopt(
        short = "s",
        long = "stop-bits",
        parse(try_from_str = "parse_stop_bits"),
        help = "Set number of stop bits",
        default_value = "1"
    )]
    stop_bits: StopBits,

    #[structopt(short = "r", long = "raw", help = "Disable XMODEM")]
    raw: bool,
}

fn main() {
    use std::fs::File;
    use std::io::{self, BufRead, BufReader};

    let opt = Opt::from_args();
    let mut serial = serial::open(&opt.tty_path).expect("path points to invalid TTY");

    serial
        .set_timeout(Duration::from_secs(opt.timeout))
        .expect("failed to set timeout");

    let mut settings = serial
        .read_settings()
        .expect("unable to read settings of serial port");

    settings
        .set_baud_rate(opt.baud_rate)
        .expect("failed to set baud_rate");

    settings.set_stop_bits(opt.stop_bits);

    settings.set_char_size(opt.char_width);

    settings.set_flow_control(opt.flow_control);

    serial
        .write_settings(&settings)
        .expect("unable to write settings");

    let mut input: Box<io::Read> = match opt.input {
        None => Box::new(stdin()),
        Some(path) => Box::new(File::open(path).expect("failed to open input")),
    };

    if opt.raw {
        io::copy(&mut input, &mut serial).expect("failed to copy");
    } else {
        Xmodem::transmit_with_progress(&mut input, &mut serial, progressFn)
            .expect("failed to transmit via Xmodem");
    }
}

fn progressFn(progress: xmodem::Progress) {
    println!("progress: {:?}", progress);
}
