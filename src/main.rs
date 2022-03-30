use std::ffi::OsStr;

use clap::Parser;

mod qbl_xml;
use qbl_xml::*;

/// Convert between .qbl and .xml files
#[derive(Parser)]
#[clap(version)]
struct Cli {
    /// The file to convert
    #[clap(parse(from_os_str))]
    input: std::path::PathBuf,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FileType {
    Qbl,
    Xml,
}
impl FileType {
    fn other(self) -> Self {
        match self {
            Self::Qbl => Self::Xml,
            Self::Xml => Self::Qbl,
        }
    }
    fn extension(self) -> &'static str {
        match self {
            Self::Qbl => "qbl",
            Self::Xml => "xml",
        }
    }
    fn from_extension(extension: &str) -> Option<Self> {
        match extension {
            "xml" => Some(Self::Xml),
            "qbl" => Some(Self::Qbl),
            _ => None,
        }
    }
}
impl std::fmt::Display for FileType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.extension())
    }
}

fn main() {
    let args = Cli::parse();
    let input_file_name = &args.input;
    let input_file_type = input_file_name
        .extension()
        .and_then(OsStr::to_str)
        .and_then(FileType::from_extension)
        .expect("unknown input file type");

    let mut output_file_name = input_file_name.clone();
    output_file_name.set_extension(input_file_type.other().extension());

    println!("Reading data from {}", input_file_name.display());

    let input_data = std::fs::read(input_file_name).expect("could not read input file");

    println!(
        "Converting from {} to {}",
        input_file_type,
        input_file_type.other()
    );

    let output_data = match input_file_type {
        FileType::Qbl => qbl_to_xml(&input_data),
        FileType::Xml => xml_to_qbl(&input_data),
    };

    println!("Writing to {}", output_file_name.display());

    std::fs::write(output_file_name, output_data).expect("could not write to output file");

    println!("Done")
}
