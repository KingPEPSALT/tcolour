pub mod colour;
pub mod gradient;

pub use colour::*;
pub use gradient::*;

#[cfg(test)]
mod tests {

    #[cfg(feature = "image-tests")]
    use color_eyre::eyre::Result;

    #[cfg(feature = "image-tests")]
    #[test]
    pub fn with_noise_test() -> Result<()> {
        use crate::{BlendMode, Colour, Gradient};
        use fastnoise_lite::{FastNoiseLite, FractalType, NoiseType};
        use image::{ImageFormat, Rgba, RgbaImage};

        let mut galaxy_noise = FastNoiseLite::with_seed(0xbeefddd);
        galaxy_noise.noise_type = NoiseType::OpenSimplex2S;
        galaxy_noise.fractal_type = FractalType::FBm;
        galaxy_noise.octaves = 6;
        galaxy_noise.gain = 0.7f32;
        galaxy_noise.lacunarity = 2f32;
        galaxy_noise.frequency = 0.7f32;

        let galaxy_gradient = Gradient(vec![
            (-1f64, Colour::solid(0f64, 0f64, 0.02f64)), // black
            (-0.1f64, Colour::solid(0.04f64, 0.04f64, 0.08f64)), // Near-black blue
            (0.3f64, Colour::solid(0.1f64, 0.08f64, 0.24f64)), // Deep indigo
            (0.6f64, Colour::solid(0.20f64, 0.08f64, 0.45f64)), // Dark purple-blue
            (0.75f64, Colour::solid(0.40f64, 0.12f64, 0.55f64)), // Soft electric purple
            (0.7f64, Colour::solid(0.55f64, 0.20f64, 0.65f64)), // Muted magenta-blue
            (0.9f64, Colour::solid(0.65f64, 0.30f64, 0.75f64)), // Soft pinkish-purple
            (1f64, Colour::solid(0.65f64, 0.40f64, 0.80f64)), // Dim glowing violet
        ]);

        let mut star_noise = FastNoiseLite::with_seed(0xbeefaaa);
        star_noise.noise_type = NoiseType::ValueCubic;
        star_noise.fractal_type = FractalType::FBm;
        star_noise.octaves = 4;
        star_noise.gain = 1.0f32;
        star_noise.lacunarity = 4f32;
        star_noise.frequency = 2f32;

        let star_gradient = Gradient(vec![
            (0.95f64, Colour::transparent()),
            // (0.95f64, Colour::solid(0.0f64, 0.0f64, 0.0f64)), // Below 0.8: no star (dark)
            (0.975f64, Colour::solid(0.9f64, 0.9f64, 1.0f64)), // Brighter bluish-white
            (0.9875f64, Colour::solid(1.0f64, 0.95f64, 0.9f64)), // Slight warm tint (subtle yellowish)
            (1.0f64, Colour::solid(1.0f64, 1.0f64, 1.0f64)),     // Full white highlight
        ]);

        let mut image = RgbaImage::new(512, 512);
        for y in 0..image.height() {
            for x in 0..image.width() {
                image.put_pixel(
                    x,
                    y,
                    Rgba(
                        galaxy_gradient
                            .sample(
                                galaxy_noise.get_noise_2d(x as f32 / 512f32, y as f32 / 512f32)
                                    as f64,
                            )
                            .blend(
                                star_gradient.sample(
                                    star_noise.get_noise_2d(x as f32 / 512f32, y as f32 / 512f32)
                                        as f64,
                                ),
                                BlendMode::HardLight,
                            )
                            .into(),
                    ),
                );
            }
        }
        image.save_with_format("test/noise.png", ImageFormat::Png)?;
        Ok(())
    }
}
