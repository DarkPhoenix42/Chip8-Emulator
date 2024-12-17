use crate::emulator::consts;
use sdl2;
use sdl2::render;
use sdl2::video;

pub struct Display {
    window: render::Canvas<video::Window>,
}

impl Display {
    pub fn new() -> Display {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let window = video_subsystem
            .window(
                consts::DISPLAY_TITLE,
                (consts::DISPLAY_WIDTH * consts::DISPLAY_SCALE) as u32,
                (consts::DISPLAY_HEIGHT * consts::DISPLAY_SCALE) as u32,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();

        Display { window: canvas }
    }

    pub fn draw_texture(&mut self, pixels: &[u8; consts::DISPLAY_WIDTH * consts::DISPLAY_HEIGHT]) {
        let texture_creator = self.window.texture_creator();

        let mut texture = texture_creator
            .create_texture_streaming(
                sdl2::pixels::PixelFormatEnum::RGB24,
                consts::DISPLAY_WIDTH as u32 * consts::DISPLAY_SCALE as u32,
                consts::DISPLAY_HEIGHT as u32 * consts::DISPLAY_SCALE as u32,
            )
            .unwrap();

        texture
            .with_lock(None, |buffer: &mut [u8], _| {
                for i in 0..consts::DISPLAY_HEIGHT {
                    for j in 0..consts::DISPLAY_WIDTH {
                        let offset = i * consts::DISPLAY_WIDTH + j;
                        let color = if pixels[offset] == 0 { 0 } else { 255 };

                        for y in 0..consts::DISPLAY_SCALE {
                            for x in 0..consts::DISPLAY_SCALE {
                                let idx = ((i * consts::DISPLAY_SCALE + y)
                                    * (consts::DISPLAY_WIDTH * consts::DISPLAY_SCALE)
                                    + (j * consts::DISPLAY_SCALE + x))
                                    * 3;

                                buffer[idx..idx + 3].fill(color);
                            }
                        }
                    }
                }
            })
            .unwrap();

        self.window.clear();
        self.window.copy(&texture, None, None).unwrap();
        self.window.present();
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_display() {
        let pixels = [1; consts::DISPLAY_HEIGHT * consts::DISPLAY_WIDTH];
        let mut display = Display::new();
        assert_eq!(display.window.window().title(), consts::DISPLAY_TITLE);
        assert_eq!(
            display.window.window().size(),
            (
                (consts::DISPLAY_WIDTH * consts::DISPLAY_SCALE) as u32,
                (consts::DISPLAY_HEIGHT * consts::DISPLAY_SCALE) as u32
            )
        );

        display.draw_texture(&pixels);
        loop {}
    }
}
