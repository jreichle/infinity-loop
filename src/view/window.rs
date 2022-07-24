use pixels::{Pixels, SurfaceTexture};

use winit::dpi::PhysicalSize;
use winit::event::{ElementState, Event, MouseButton, WindowEvent};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;

use crate::model::coordinate::Coordinate;
use crate::model::gameboard::*;
use crate::model::grid::*;
use crate::model::testlevel::*;
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

    let mut index = 0;

    let mut pixels;
    if let Ok(buffer) = Pixels::new(window_size.width, window_size.height, surface_texture) {
        pixels = buffer;
    } else {
        panic!("Problem! TODO!!!"); // TODO: error handling
    }

    let levels = TEST_LEVELS
        .map(|l| parse_level(l, char_to_tile).unwrap())
        .to_vec();

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait; // suspend thread until new event arrives
        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            Event::RedrawRequested(_) => {
                // On window load
                draw(pixels.get_frame(), window_size, &levels[index]);
                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                window.request_redraw();
            }
            Event::WindowEvent {
                window_id: _,
                event:
                    WindowEvent::MouseInput {
                        device_id: _,
                        state: ElementState::Pressed,
                        button: MouseButton::Left,
                        modifiers: _,
                    },
            } => {
                // On mouse click
                index = (index + 1) % 20;
                draw(pixels.get_frame(), window_size, &levels[index]);
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

fn draw(frame: &mut [u8], window_size: PhysicalSize<u32>, board: &Grid<Tile<Square>>) {
    let tile_set = board.serialize_board();

    let mut tile_width = (window_size.width as usize) / (board.columns() as usize);
    if tile_width % 2 == 0 {
        tile_width -= 1;
    }

    let mut tile_height = (window_size.height as usize) / (board.rows() as usize);
    if tile_height % 2 == 0 {
        tile_height -= 1;
    }

    let tile_center_x = tile_width / 2 + 1;
    let tile_center_y = tile_height / 2 + 1;

    for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {
        let x = i % window_size.width as usize;
        let y = i / window_size.width as usize;

        let tile_coord_x = x / tile_width;
        let tile_coord_y = y / tile_height;

        let color1 = [0xff, 0xff, 0xff, 0xff];
        let color2 = [0x00, 0x00, 0x00, 0xff];
        let color3 = [0xff, 0xff, 0xff, 0x03];

        let tile;
        if let Some(result) = tile_set.get(&Coordinate {
            column: tile_coord_y as isize,
            row: tile_coord_x as isize,
        }) {
            tile = result.get_value();
        } else {
            pixel.copy_from_slice(&color1);
            continue;
        }

        let norm_x = std::cmp::max(0, (x as i16) - ((tile_coord_x * tile_width) as i16)) as usize;
        let norm_y = std::cmp::max(0, (y as i16) - ((tile_coord_y * tile_height) as i16)) as usize;
        let center_square_size = 3;
        let helper_grid_distance = 6;

        let rgba = if norm_x >= tile_center_x - center_square_size
            && norm_x <= tile_center_x + center_square_size
            && norm_y >= tile_center_y - center_square_size
            && norm_y <= tile_center_y + center_square_size
            && tile != ' '
        {
            // Centered box
            color2
        } else if norm_x == tile_center_x && norm_y < tile_center_y && "╹┗┃┣┛┻┫╋".contains(tile)
        {
            // Upper arm
            color2
        } else if norm_x == tile_center_x && norm_y > tile_center_y && "╻┃┏┣┓┫┳╋".contains(tile)
        {
            // Lower arm
            color2
        } else if norm_x < tile_center_x && norm_y == tile_center_y && "╸┛━┻┓┫┳╋".contains(tile)
        {
            // Left arm
            color2
        } else if norm_x > tile_center_x && norm_y == tile_center_y && "╺┗┏┣━┻┳╋".contains(tile)
        {
            // Right arm
            color2
        } else if (norm_x == 0 && tile_coord_x != 0 && norm_y % helper_grid_distance == 0)
            || (norm_y == 0 && tile_coord_y != 0 && norm_x % helper_grid_distance == 0)
        {
            // Grid lines
            color3
        } else {
            // default
            color1
        };

        pixel.copy_from_slice(&rgba);
    }
}

fn draw_line(frame: &mut [u8]) {}
