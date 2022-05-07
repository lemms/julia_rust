pub fn julia(x: f32, y: f32,
         w: f32, h: f32,
         move_x: f32, move_y: f32,
         zoom: f32,
         cx: f32, cy: f32,
         max_iter: u32) -> u32 {

    let mut zx = 1.5 * (x - w / 2.0) / (0.5 * zoom * w) + move_x;
    let mut zy = 1.0 * (y - h / 2.0) / (0.5 * zoom * h) + move_y;
    
    for i in 0..max_iter {
        if zx * zx + zy * zy >= 4.0 {
            return max_iter - i;
        }

        let temp = zx * zx - zy * zy + cx;
        zy = 2.0 * zx * zy + cy;
        zx = temp;
    }

    max_iter
}
