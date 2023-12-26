use std::io::Cursor;

use screenshots::image::ImageFormat;
use screenshots::{display_info::DisplayInfo, Screen};

const IMG_WIDTH: i32 = 300;
const IMG_HEIGHT: i32 = 100;

pub fn get_img_pos() -> (Vec<u8>, (i32, i32)) {
    use mouse_position::mouse_position::Mouse;
    let position = Mouse::get_mouse_position();
    let (x, y) = match position {
        Mouse::Position { x, y } => (x, y),
        Mouse::Error => panic!("get mouse pos error!"),
    };
    get_img(x, y)
}

#[derive(Debug)]
struct Area {
    left: i32,
    top: i32,
    width: u32,
    height: u32,
    mouse_pos: (i32, i32),
}

pub fn get_img(x: i32, y: i32) -> (Vec<u8>, (i32, i32)) {
    dbg!(x, y);
    let screen = Screen::from_point(x, y).unwrap();
    let area = compute_img_area(screen.display_info, [x, y]);
    let image = screen
        .capture_area(area.left, area.top, area.width, area.height) // scale_factr
        .unwrap();

    #[cfg(debug_assertions)]
    {
        println!("display: {:?}", screen);
        // let image = screen.capture().unwrap();
        image.save("screen.png").unwrap();
    }
    let vec = Vec::new();
    let mut buf = Cursor::new(vec);
    image.write_to(&mut buf, ImageFormat::Png).unwrap();
    return (buf.into_inner(), area.mouse_pos);
}

fn compute_img_area(display: DisplayInfo, pos: [i32; 2]) -> Area {
    let logical_w = IMG_WIDTH as f32 / display.scale_factor;
    let logical_h = IMG_HEIGHT as f32 / display.scale_factor;

    let (left, x) = top_left(pos[0] as f32, logical_w, display.width as f32);
    let (top, y) = top_left(pos[1] as f32, logical_h, display.height as f32);
    let phic_x = x * display.scale_factor;
    let phic_y = y * display.scale_factor;
    Area {
        left: left as i32,
        top: top as i32,
        width: logical_w as u32,
        height: logical_h as u32,
        mouse_pos: (phic_x as i32, phic_y as i32),
    }
}

// return (img left or top ax, mouse point in img ax, img right or bottom)
fn top_left(mouse_point: f32, img_size: f32, max_size: f32) -> (f32, f32) {
    let img_point = img_size / 2.0;
    if mouse_point < img_point {
        return (0.0, mouse_point);
    }
    if mouse_point < (max_size - img_point) {
        return (mouse_point - img_point, img_point);
    }
    return (max_size - img_size, img_size - (max_size - mouse_point));
}
