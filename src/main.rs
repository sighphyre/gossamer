mod fitness_functions;
mod gif_processing;
mod image_processing;

use core::fmt::Display;
use core::str::FromStr;
use fitness_functions::{calc_mse, calc_psnr};
use gif_processing::make_gif;
use image_processing::{coverge_on_image, Dimensions};
use std::{fmt, fmt::Debug, path::PathBuf};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "Goassamer",
    about = "Making images. The worst way possible. Because reasons."
)]
enum Mode {
    #[structopt(name = "new", about = "Starts processing a new image")]
    New {
        #[structopt(
            parse(from_os_str),
            name = "imagepath",
            long,
            short,
            help = "Path to the image you want to process."
        )]
        image_path: PathBuf,
        #[structopt(
            name = "height",
            long,
            short,
            help = "Height to rescale the image to. If this is set and no width is passed then the image will be rescaled while maintaining the aspect ratio."
        )]
        height: Option<u32>,
        #[structopt(
            name = "width",
            long,
            short,
            help = "Width to rescale the image to. If this is set and no height is passed then the image will be rescaled while maintaining the aspect ratio."
        )]
        width: Option<u32>,
        #[structopt(
            parse(from_os_str),
            name = "outputpath",
            long,
            short,
            help = "Output path for checkpoint images."
        )]
        outputpath: PathBuf,
        #[structopt(
            name = "checkpointdensity",
            long,
            short,
            help = "How often checkpoint images are outputted.",
            default_value = "100"
        )]
        checkpointdensity: u32,
        #[structopt(
            name = "fitnessfunction",
            long,
            short,
            help = "What fitness function to use, defaults to psnr.",
            default_value = "psnr"
        )]
        fitness_function: FitnessFunc,
    },
    #[structopt(
        name = "rollup",
        about = "Turns a processed image folder into a pretty gif!"
    )]
    Rollup {
        #[structopt(
            parse(from_os_str),
            name = "inputfolder",
            long,
            short,
            help = "Path to folder that you want to process."
        )]
        input_folder: PathBuf,
        #[structopt(
            parse(from_os_str),
            name = "outputfile",
            long,
            short,
            help = "Path to the gif you want to write. The folder must exist if it's not adjacted to the execution path.",
            default_value = "output.gif"
        )]
        output_file: PathBuf,
    },
}

#[derive(Debug)]
struct CliError(String);

impl Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        std::fmt::Display::fmt(&self, f)
    }
}

enum FitnessFunc {
    Mse,
    Psnr,
}

impl FromStr for FitnessFunc {
    type Err = CliError;

    fn from_str(func_name: &str) -> Result<Self, Self::Err> {
        match func_name {
            "mse" => Ok(FitnessFunc::Mse),
            "psnr" => Ok(FitnessFunc::Psnr),
            _ => Err(CliError("Unknown fitness function".to_string())),
        }
    }
}

fn main() {
    let opt = Mode::from_args();
    match opt {
        Mode::New {
            image_path,
            height,
            width,
            outputpath,
            checkpointdensity,
            fitness_function,
        } => {
            let dimensions = Dimensions {
                required_width: width,
                required_height: height,
            };
            let fitness_func = match fitness_function {
                FitnessFunc::Mse => {
                    println!("Starting a new run with mse");
                    calc_mse
                }
                FitnessFunc::Psnr => {
                    println!("Starting a new run with psnr");
                    calc_psnr
                }
            };
            coverge_on_image(
                image_path,
                outputpath,
                dimensions,
                checkpointdensity,
                fitness_func,
            );
        }
        Mode::Rollup {
            input_folder,
            output_file,
        } => {
            make_gif(input_folder, output_file);
        }
    }
}
