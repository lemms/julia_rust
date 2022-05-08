use image::ImageBuffer;

//mod cividis;
//mod inferno;
mod julia;
//mod magma;
//mod plasma;
mod turbo;
//mod twilight;
//mod viridis;

fn write_image(filename: &str, w: u32, h: u32) {
    let move_x = 0.0;
    let move_y = 0.0;
    let zoom = 1.0;
    let cx = -0.7;
    let cy = 0.27015;
    let max_iter = 1024;

    let image = ImageBuffer::from_fn(w, h, |x, y| {
        let i = julia::julia(x as f32, y as f32, w as f32, h as f32, move_x, move_y, zoom, cx, cy, max_iter);
        let index = (i as f32 / max_iter as f32 * 255.0) as usize;
        image::Rgb([(turbo::TURBO[index][0] * 255.0) as u8,
                    (turbo::TURBO[index][1] * 255.0) as u8,
                    (turbo::TURBO[index][2] * 255.0) as u8])
    });

    image.save(filename).unwrap();
}

fn main() {
    let filename = String::from("test.png");

    let w = 2560;
    let h = 1440;

    write_image(&filename, w, h);
}
