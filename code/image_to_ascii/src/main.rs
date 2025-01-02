use clap::Parser;
use image::imageops::FilterType;
use image::{DynamicImage, GenericImageView};
use std::path::PathBuf;

#[derive(Parser)]
struct Args {
    // The flag names are decided by the field name's first letter

    // The input image path
    #[arg(short, long)]
    input: PathBuf,

    // The output width in characters
    #[arg(short, long, default_value_t = 100)]
    width: u32,

    // The output height in characters
    #[arg(short = 'H', long)] // Changed from 'h' to 'H' to avoid conflict with help
    height: Option<u32>, // This is an optional argument
}

const ASCII_CHARS: &str = "@%#*+=-:. "; // Each character represents a different intensity level in the image, with '@' being the darkest and space being the lightest.

fn main() {
    let args = Args::parse(); // Parse the command line arguments

    println!("\n\n"); // Add some space before the output

    match process_image(&args) {
        // Process the image and handle any errors
        Ok(ascii_art) => {
            println!("{}", ascii_art);
            println!("\n\n");
        } // Add some space before the output},  // Print the ASCII art if successful
        Err(e) => eprintln!("Error: {}", e), // Print the error message if an error occurred
    }
}

fn process_image(args: &Args) -> Result<String, Box<dyn std::error::Error>> {
    // load the image
    let img = image::open(&args.input)?;

    let height = args.height.unwrap_or_else(|| {
        let aspect_ratio = img.height() as f32 / img.width() as f32;
        (args.width as f32 * aspect_ratio * 0.5) as u32
    });

    let resized = img.resize_exact(args.width, height, FilterType::Lanczos3);
    let ascii_art = image_to_ascii(&resized);

    Ok(ascii_art)
}

fn image_to_ascii(img: &DynamicImage) -> String {
    let (width, height) = img.dimensions();
    // pre-allocate the string with the exact size we need
    let mut result = String::with_capacity((width * height) as usize);

    for y in 0..height {
        result.push('\t');
        for x in 0..width {
            let pixel = img.get_pixel(x, y);

            // Convert the pixel to grayscale using the luminance formula

            // The luminance formula is: 0.299 * R + 0.587 * G + 0.114 * B
            // Pixel[0] is the red channel, pixel[1] is the green channel, and pixel[2] is the blue channel

            let intensity =
                (0.299 * pixel[0] as f32 + 0.587 * pixel[1] as f32 + 0.114 * pixel[2] as f32) as u8;

            let char_index = (intensity as f32 / 255.0 * (ASCII_CHARS.len() - 1) as f32) as usize;
            result.push(ASCII_CHARS.chars().nth(char_index).unwrap());
        }
        result.push('\n');
    }
    result
}
