pub mod colour;
pub mod gradient;

pub use colour::*;
pub use gradient::*;

#[cfg(test)]
mod tests {
    use color_eyre::eyre::Result;
    use image::{
        DynamicImage, GenericImageView, ImageFormat, Rgba, RgbaImage
    };

    use crate::{BlendMode, Colour};

    #[test]
    pub fn blend_test() -> Result<()> {
        use image::ImageReader;

        let cat = ImageReader::open("test/cat.jpg")?.decode()?;
        let bricks = ImageReader::open("test/bricks.png")?.decode()?;
        let get_colours = |image: &DynamicImage| -> Vec<Colour> {
            image
                .pixels()
                .map(|(_, _, pixel)| {
                    Colour::from_u8_rgba(pixel.0[0], pixel.0[1], pixel.0[2], pixel.0[3])
                })
                .collect()
        };

        let brick_colours = get_colours(&bricks);
        let mut cat_colours = get_colours(&cat);
        cat_colours = cat_colours.iter().map(|f| f.with_alpha(1.0)).collect();
        let mut blended_image = RgbaImage::new(512, 512);

        for (index, (base, blend)) in brick_colours.iter().zip(cat_colours).enumerate() {
            let x = index as u32 % 512;
            let y = index as u32 / 512;
            blended_image.put_pixel(x, y, Rgba(base.blend(blend, BlendMode::Darken).into()));
        }
        blended_image.save_with_format("test/blended.png", ImageFormat::Png)?;
        Ok(())
    }
}
