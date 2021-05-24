mod bounding_box;
mod graphics;

use graphics::screenshot;
use std::io::BufReader;

fn main() {
    let path = "res/rounded_box.stl";
    let file = std::fs::File::open(&path).unwrap();
    let mut reader = BufReader::new(&file);
    let mesh = nom_stl::parse_stl(&mut reader).unwrap();

    let descrip = screenshot::ScreenshotDescriptor {
        mesh: &mesh,
        dst_path: "output.png",
        width: 512,
        height: 512,
        model_translation: cgmath::Vector3 {
            x: 0.0,
            y: -5.0,
            z: 0.0,
        },
        point_light_position: cgmath::Point3 {
            x: 0.0,
            y: 25.0,
            z: 25.0,
        },
        camera_position: cgmath::Point3 {
            x: 0.0,
            y: 0.0,
            z: 100.0,
        },
        camera_fovy: cgmath::Deg(45.0),
    };
    futures::executor::block_on(screenshot::run(descrip));
}
