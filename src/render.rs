use image::{
    error::{EncodingError, ImageFormatHint},
    DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgb,
};

#[derive(Debug)]
pub struct ImageDimensions(pub u32, pub u32);

#[derive(Debug)]
pub struct Image {
    pub dimensions: ImageDimensions,
    pub colors: Vec<Rgb<u8>>,
}

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

pub fn read_image(path: &str) -> Result<Image, ImageError> {
    let image = image::open(path)?;

    let image = match image {
        DynamicImage::ImageRgb8(image) => image,
        _ => {
            return Err(ImageError::Encoding(EncodingError::from_format_hint(
                ImageFormatHint::Exact(ImageFormat::Png),
            )))
        }
    };

    let dimensions = ImageDimensions(image.width(), image.height());
    let mut colors = Vec::new();

    for pixel in image.pixels() {
        colors.push(Rgb::from(pixel.0));
    }

    Ok(Image { dimensions, colors })
}
