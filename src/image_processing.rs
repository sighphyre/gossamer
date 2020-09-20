use image::{imageops::FilterType, ImageBuffer, Pixel};
use image::{GenericImageView, Luma};
use imageproc::rect::Rect;
use rand::Rng;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub struct Dimensions {
    pub required_height: Option<u32>,
    pub required_width: Option<u32>,
}

fn rescale(current_known_dim: u32, required_known_dim: u32, current_unknown_dim: u32) -> u32 {
    ((required_known_dim as f32 / current_known_dim as f32) * current_unknown_dim as f32) as u32
}

fn resize<I: GenericImageView>(
    image: &I,
    dim: Dimensions,
) -> ImageBuffer<I::Pixel, Vec<<I::Pixel as Pixel>::Subpixel>>
where
    I::Pixel: 'static,
    <I::Pixel as Pixel>::Subpixel: 'static,
{
    let (width, height) = match dim {
        Dimensions {
            required_height: Some(required_height),
            required_width: Some(required_width),
        } => (required_width, required_height),
        Dimensions {
            required_height: None,
            required_width: Some(required_width),
        } => (
            required_width,
            rescale(image.width(), required_width, image.height()),
        ),
        Dimensions {
            required_height: Some(required_height),
            required_width: None,
        } => (
            rescale(image.height(), required_height, image.width()),
            required_height,
        ),
        Dimensions {
            required_height: None,
            required_width: None,
        } => (image.width(), image.height()),
    };
    println!(
        "Processing output image at width: {} and height: {}",
        width, height
    );
    image::imageops::resize(image, width, height, FilterType::Gaussian)
}

type FitnessFunc = fn(
    img1: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
    img2: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> f32;

pub fn coverge_on_image(
    path: PathBuf,
    output_folder_path: PathBuf,
    dims: Dimensions,
    checkpoint_density: u32,
    fitness_func: FitnessFunc,
) {
    create_dir_all(&output_folder_path).expect("Could not ensure output directory");
    let output_folder_path = output_folder_path
        .into_os_string()
        .into_string()
        .expect("Could not convert output path into string");

    let mut rng = rand::thread_rng();
    let img = image::open(path).expect("Can't find image to load!");
    let resized_image = resize(&img, dims);
    println!(
        "Rescale image to {} {}",
        resized_image.width(),
        resized_image.height()
    );

    let gray_image = image::imageops::colorops::grayscale(&resized_image);

    let fill_rect = Rect::at(0, 0).of_size(gray_image.width(), gray_image.height());
    let mut blank_canvas =
        imageproc::drawing::draw_filled_rect(&gray_image, fill_rect, Luma([255u8]));

    let mut current_fitness = fitness_func(&gray_image, &blank_canvas);
    let (w, h) = (gray_image.width() as f32, gray_image.height() as f32);

    let mut counter = 0;
    loop {
        let new_image = imageproc::drawing::draw_line_segment(
            &blank_canvas,
            (rng.gen::<f32>() * w, rng.gen::<f32>() * h),
            (rng.gen::<f32>() * w, rng.gen::<f32>() * h),
            Luma([rng.gen::<u8>()]),
        );
        let fitness = fitness_func(&new_image, &gray_image);
        if fitness > current_fitness {
            counter += 1;
            blank_canvas = new_image;
            current_fitness = fitness;
            if counter % checkpoint_density == 0 {
                println!("Current fitness is {}", current_fitness);
                blank_canvas
                    .save(format!("{}/out{}.jpg", output_folder_path, counter))
                    .expect("failed to save image");
            }
        }
    }
}
