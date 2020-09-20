use engiffen::engiffen;
use engiffen::load_images;
use engiffen::Quantizer;
use std::fs::File;
use std::{fs::read_dir, path::PathBuf};

pub fn make_gif(input_folder: PathBuf, output_filename: PathBuf) {
    let mut folder_content = read_dir(input_folder)
        .expect("Cannot file read directory")
        .map(|res| res.map(|e| e.path()))
        .map(|x| {
            x.unwrap()
                .into_os_string()
                .into_string()
                .expect("Failed to process file while processing gif")
        })
        .collect::<Vec<String>>();

    alphanumeric_sort::sort_str_slice(&mut folder_content);

    let images = load_images(&folder_content);
    let gif = engiffen(&images, 10, Quantizer::NeuQuant(2)).expect("Failed to create gif");
    let mut output =
        File::create(output_filename).expect("Failed to create file while attemping to write gif");
    gif.write(&mut output).expect("Failed to write gif");
}
