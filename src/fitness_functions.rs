use image::ImageBuffer;

fn mse(
    img1: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
    img2: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> f32 {
    let iter1 = img1.enumerate_pixels().map(|x| x.2).map(|x| x.0[0]);
    let iter2 = img2.enumerate_pixels().map(|x| x.2).map(|x| x.0[0]);
    let iterator = iter1.zip(iter2);

    let length = (img1.height() * img1.width()) as f32;

    let mut mse: f32 = 0.0;
    for (pixel1, pixel2) in iterator {
        let value = if pixel1 > pixel2 {
            pixel1 - pixel2
        } else {
            pixel2 - pixel1
        };
        mse += (value as u16 * value as u16) as f32 / length;
    }
    mse
}

//For MSE less is better, which is the inversely proportional
// to fitness so we're going to make our mse calc negative
pub fn calc_mse(
    img1: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
    img2: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> f32 {
    mse(img1, img2) * -1_f32
}

pub fn calc_psnr(
    img1: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
    img2: &ImageBuffer<image::Luma<u8>, Vec<u8>>,
) -> f32 {
    (65536_f32 / mse(img1, img2)).log10()
}
