mod bounding_box;
mod graphics;

use bounding_box::BoundingBox;
use graphics::screenshot;
use std::io::BufReader;

fn main() {
    let width = 512;
    let height = 512;
    let aspect = width as f32 / height as f32;
    let camera_fovy = cgmath::Deg(45.0);

    let path = "res/sphere.stl";
    let file = std::fs::File::open(&path).unwrap();
    let mut reader = BufReader::new(&file);
    let mesh = nom_stl::parse_stl(&mut reader).unwrap();
    let mut bounding_box = BoundingBox::new(&mesh);

    // Shift the model and its bounding box so that the bounding box is centered
    // on the origin.
    let model_translation = bounding_box.center_to_origin();
    bounding_box.shift(model_translation);

    let look_down_axis = bounding_box.largest_cross_section_axis();
    let camera_position = bounding_box.pick_camera_position(aspect, camera_fovy, &look_down_axis);
    let point_light_position = bounding_box.pick_light_position(&look_down_axis);

    let descrip = screenshot::ScreenshotDescriptor {
        mesh: &mesh,
        dst_path: "output.png",
        width,
        height,
        model_translation,
        point_light_position,
        camera_position,
        camera_fovy,
    };
    futures::executor::block_on(screenshot::run(descrip));
}
