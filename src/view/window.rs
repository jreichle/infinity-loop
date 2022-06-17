use pixels::{Pixels, SurfaceTexture};

use winit::event::{Event, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::model::tile::*;

// use https://docs.rs/buttons/latest/buttons/ for button -- has winit support

pub fn initiate_window() {
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Infinity Loop")
        .build(&event_loop)
        .unwrap();

    let window_size = window.inner_size();
    let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);

    let mut pixels;
    if let Ok(buffer) = Pixels::new(320, 240, surface_texture) {
        pixels = buffer;
    } else {
        panic!("Problem! TODO!!!"); // TODO: error handling
    }

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; // suspend thread until new event arrives
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                draw(pixels.get_frame());
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                window.request_redraw();
            }
            _ => (),
        }
    });
}

// serialize -> Haspmap<(x,y), tile>
// needed:
// - how big are tiles currently
// - how big is canvas -> padding

fn draw(frame: &mut [u8]) {
    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = (i % 320 as usize) as i16;
        let y = (i / 320 as usize) as i16;

        let inside = x >= 10 && x < 110 && y > 20 && y < 120;

        let rgba = if inside {
            [0x5e, 0x99, 0x39, 0xff]
        } else {
            [0x48, 0xb2, 0xe8, 0xff]
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn draw_line(frame: &mut [u8]) {}
