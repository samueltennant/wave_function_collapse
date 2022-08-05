use image::{
    error::{EncodingError, ImageFormatHint},
    DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgb,
};

pub struct ImageDimensions(pub u32, pub u32);

pub fn output_image(colors: Vec<Rgb<u8>>, dim: ImageDimensions) -> Result<(), ImageError> {
    // Ensure that the colors vector is the correct length.
    if colors.len() != (dim.0 * dim.1) as usize {
        return Err(ImageError::Encoding(EncodingError::from_format_hint(
            ImageFormatHint::Exact(ImageFormat::Png),
        )));
    }

    let image = ImageBuffer::from_fn(dim.0, dim.1, |x, y| {
        let index = (y * dim.0 + x) as usize;
        colors[index]
    });

    image.save_with_format("output/image.png", ImageFormat::Png)?;

    Ok(())
}

// Honestly thought this would need to do more.
// Don't care, not going to move it.
pub fn read_image(path: &str) -> Result<DynamicImage, ImageError> {
    let image = image::open(path)?;

    Ok(image)
}
