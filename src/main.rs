use image::ImageBuffer;

mod julia;

fn write_image(filename: &str, w: u32, h: u32) {
    let move_x = 0.0;
    let move_y = 0.0;
    let zoom = 1.0;
    let cx = -0.7;
    let cy = 0.27015;
    let max_iter = 255;

    let image = ImageBuffer::from_fn(w, h, |x, y| {
        let i = julia::julia(x as f32, y as f32, w as f32, h as f32, move_x, move_y, zoom, cx, cy, max_iter);
        image::Rgb([(i << 21) as u8, (i << 10) as u8, (i << 3) as u8])
    });

    image.save(filename).unwrap();
}

fn main() {
    let filename = String::from("test.png");

    let w = 1920;
    let h = 1080;

    write_image(&filename, w, h);
}
