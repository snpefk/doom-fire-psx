extern crate image;
extern crate piston_window;
 
use rand::Rng; 
use piston_window::*;
use std::{thread, time};


const COLORS: [[u8; 4]; 37] = [
    [0x07, 0x07, 0x07, 0xFF],
    [0x1F, 0x07, 0x07, 0xFF],
    [0x2F, 0x0F, 0x07, 0xFF],
    [0x47, 0x0F, 0x07, 0xFF],
    [0x57, 0x17, 0x07, 0xFF],
    [0x67, 0x1F, 0x07, 0xFF],
    [0x77, 0x1F, 0x07, 0xFF],
    [0x8F, 0x27, 0x07, 0xFF],
    [0x9F, 0x2F, 0x07, 0xFF],
    [0xAF, 0x3F, 0x07, 0xFF],
    [0xBF, 0x47, 0x07, 0xFF],
    [0xC7, 0x47, 0x07, 0xFF],
    [0xDF, 0x4F, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xDF, 0x57, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x5F, 0x07, 0xFF],
    [0xD7, 0x67, 0x0F, 0xFF],
    [0xCF, 0x6F, 0x0F, 0xFF],
    [0xCF, 0x77, 0x0F, 0xFF],
    [0xCF, 0x7F, 0x0F, 0xFF],
    [0xCF, 0x87, 0x17, 0xFF],
    [0xC7, 0x87, 0x17, 0xFF],
    [0xC7, 0x8F, 0x17, 0xFF],
    [0xC7, 0x97, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0x9F, 0x1F, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xA7, 0x27, 0xFF],
    [0xBF, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xAF, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x2F, 0xFF],
    [0xB7, 0xB7, 0x37, 0xFF],
    [0xCF, 0xCF, 0x6F, 0xFF],
    [0xDF, 0xDF, 0x9F, 0xFF],
    [0xEF, 0xEF, 0xC7, 0xFF],
    [0xFF, 0xFF, 0xFF, 0xFF],
];

fn main() {
    let (width, height): (usize, usize) = (300, 168);

    let mut window: PistonWindow =
        WindowSettings::new("Doom Fire PSX", (width as u32, height as u32))
            .exit_on_esc(true)
            .opengl(OpenGL::V3_2)
            .build()
            .expect("Failed to create window");

    let mut canvas = image::ImageBuffer::new(width as u32, height as u32);
    let mut texture: G2dTexture =
        Texture::from_image(&mut window.factory, &canvas, &TextureSettings::new()).unwrap();

    let mut fbuffer: [usize; 53_760] = [0; 53_760];

    for x in 0..width {
        fbuffer[(height - 1) * width + x] = 36
    }
    
    let delay = time::Duration::from_millis(30);
    
    while let Some(event) = window.next() {
        thread::sleep(delay);
        for x in 0..width {
            for y in 1..height {
                let index = y * width + x;

                if fbuffer[index] == 0 { 
                    fbuffer[index - width] = 0;
                } else {
                    let rand_idx = rand::thread_rng().gen_range(1, 4);
                    let color: isize = (fbuffer[index] - (rand_idx & 1)) as isize;
                    fbuffer[index] = (color % 36) as usize;
                    let dst: usize = index - rand_idx + 1;
                    fbuffer[dst - width] = fbuffer[index];
                }
            }
        }

        for x in 0..width {
            for y in 0..height {
                let color = COLORS[fbuffer[(y * width + x)]];
                canvas.put_pixel(x as u32, y as u32, image::Rgba(color));
            }
        }
        texture.update(&mut window.encoder, &canvas).unwrap();

        window.draw_2d(&event, |context, graphics| {
            clear([0.03; 4], graphics);
            image(&texture, context.transform, graphics)
        });
    }
}
