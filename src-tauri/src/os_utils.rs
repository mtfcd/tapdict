use screenshots::{DisplayInfo, Image, Screen};
// use std::{fs, time::Instant};

#[cfg(target_os = "windows")]
pub fn get_mouse_position() -> (Image, [i32; 2]) {
    use winapi::shared::windef::POINT;
    use winapi::um::winuser::GetCursorPos;

    let mut point: POINT = POINT { x: 0, y: 0 };
    unsafe {
        GetCursorPos(&mut point);
    }
    (get_img(point.x, point.y), [320, 240])
}

#[cfg(target_os = "linux")]
pub fn get_mouse_position() {
    println!("not surport");
}

fn get_img(x: i32, y: i32) -> Image {
    // let start = Instant::now();

    let display = DisplayInfo::from_point(x, y).unwrap();

    let screen = Screen::new(&display);
    let image = screen.capture_area(x - 320, y - 240, 640, 480).unwrap();
    // let buffer = image.buffer();
    // fs::write("target/capture_display_with_point.png", buffer).unwrap();

    // println!("运行耗时: {:?}", start.elapsed());
    return image;
}
