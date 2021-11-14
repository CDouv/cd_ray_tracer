
mod vec3;

use vec3::Vec3;

// Construct a ppm file for image data, e.g.
// P3
// 3 2
// 255 0 0  0 255 0  0 0 255

fn write_ppm(w: i32, h: i32, max_value: i32) {
    println!("P3\n{} {}\n{}", w, h, max_value);

    for j in (0..h).rev() {
        for i in 0..w {

            let r: f32 = i as f32 / w as f32;
            let g: f32 = j as f32 / h as f32;
            let b: f32 = 0.2;

            let ir: i32 = (255.99 * r) as i32;
            let ig: i32 = (255.99 * g) as i32;
            let ib: i32 = (255.99 * b) as i32;

            println!("{} {} {}", ir, ig, ib);
            
        }
    }
}

fn main() {
    
    let width: i32 = 200;
    let height: i32 = 100;
    let max_value: i32 = 255;

    // we use a plain txt ppm to start building images
    write_ppm(width, height, max_value);

    let v1: Vec3 = Vec3::new(1.0,2.0,6.0);
    let v2: Vec3 = Vec3::new(3.0,5.0,6.0);
    let v3= v1 + v2;

    println!("{:?}",&v3);

}
