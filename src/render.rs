use image::{
    error::{EncodingError, ImageFormatHint},
    DynamicImage, ImageBuffer, ImageError, ImageFormat, Rgba,
};

#[derive(Debug)]
pub struct ImageDimensions(pub u32, pub u32);

#[derive(Debug)]
pub struct Image {
    pub dimensions: ImageDimensions,
    pub colors: Vec<Rgba<u8>>,
}

pub fn output_image(colors: Vec<Rgba<u8>>, dim: ImageDimensions) -> Result<(), ImageError> {
    // Ensure that the colors vector is the correct length.
    if colors.len() != (dim.0 * dim.1) as usize {
        return Err(ImageError::Encoding(EncodingError::from_format_hint(
            ImageFormatHint::Exact(ImageFormat::Png),
        )));
    }

    let image = ImageBuffer::from_fn(dim.0, dim.1, |x, y| {
        let index = (y * dim.0 + x) as usize;
        dbg!(index);
        colors[index]
    });

    image.save_with_format("output/image.png", ImageFormat::Png)?;
    dbg!("test");

    Ok(())
}

pub fn read_image(path: &str) -> Result<Image, ImageError> {
    let image = image::open(path)?;

    let image = match image {
        DynamicImage::ImageRgba8(image) => image,
        _ => {
            return Err(ImageError::Encoding(EncodingError::from_format_hint(
                ImageFormatHint::Exact(ImageFormat::Png),
            )))
        }
    };

    let dimensions = ImageDimensions(image.width(), image.height());
    let mut colors = Vec::new();

    for pixel in image.pixels() {
        colors.push(Rgba::from(pixel.0));
    }

    Ok(Image { dimensions, colors })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use image::Pixel;

    use super::*;

    #[test]
    fn test_read_image() {
        let path = "input/1.png";
        let res = read_image(path);

        assert!(res.is_ok());
    }

    #[test]
    fn test_write_image() {
        let colors = vec![*Rgba::from_slice(&[0u8, 0u8, 0u8, 255u8])];
        let dim = ImageDimensions(1, 1);

        let res = output_image(colors, dim);
        assert!(res.is_ok());

        // cleanup image file
        fs::remove_file("output/image.png").expect("failed to delete output image");
    }
}
