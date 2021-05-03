use svt::SvtFile;
use structopt::StructOpt;

use std::path::PathBuf;
use std::fs;

#[derive(StructOpt)]
#[structopt(about = "A tool for converting between Smash Ultimate 'sound volume table' files and yaml")]
struct Args {
    in_file: PathBuf,
    out_file: PathBuf,
}

fn main() {
    let args = Args::from_args();

    match SvtFile::open(&args.in_file) {
        Ok(file) => {
            fs::write(
                &args.out_file,
                serde_yaml::to_string(&file).unwrap()
            ).unwrap();
        }
        Err(svt::Error::BadMagic { .. }) => {
            // Magic doesn't match, should be yaml file

            let contents = fs::read_to_string(&args.in_file).unwrap();
            let file: SvtFile = serde_yaml::from_str(&contents).unwrap();

            file.save(&args.out_file).unwrap();
        },
        Err(err) => {
            // Another error occurred, magic matches but failed to parse
            eprintln!("An error occurred: {}", err);
        }
    }
}
