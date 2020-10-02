use image::Luma;
use log::{debug, info, LevelFilter};
use qrcode::QrCode;
use simple_logger::SimpleLogger;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::BufRead;
use std::path::{Path, PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "qr")]
struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    debug: bool,

    /// Tab delimited
    #[structopt(short, long)]
    tab: bool,

    /// Data file (comma-separated values in each line)
    #[structopt(short, long, parse(from_os_str))]
    input: PathBuf,
}

fn main() {
    let opt = Opt::from_args();

    let level = if opt.debug {
        LevelFilter::Debug
    } else {
        LevelFilter::Info
    };

    SimpleLogger::new().with_level(level).init().unwrap();

    if let Ok(lines) = read_lines(opt.input) {
        let map = parse_data_from_line(lines.filter_map(|l| l.ok()), opt.tab);

        for (key, val) in map.iter() {
            let code = QrCode::new(val.as_bytes()).unwrap();

            // Render the bits into an image.
            let image = code.render::<Luma<u8>>().build();

            // Save the image.
            image.save(&key).unwrap();
            info!("QR Code created for Invoice: {}", &key);
        }
    }
}

fn parse_data_from_line(lines: impl Iterator<Item = String>, tab: bool) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let sep = if tab { "\t" } else { " " };
    for line in lines {
        let data: Vec<String> = line
            .trim()
            .split(',')
            .filter(|&x| !x.is_empty())
            .map(|x| x.to_string())
            .collect();

        debug!("{:?}", data);

        if data.len() < 5 {
            continue;
        }

        map.insert(data[2].clone() + ".png", data.join(sep));
    }
    debug!("{:?}", map);
    map
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
