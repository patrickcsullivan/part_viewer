mod graphics;

use graphics::screenshot;

fn main() {
    let descrip = screenshot::ScreenshotDescriptor {
        mesh_path: "res/sphere.stl",
        dst_path: "output.png",
        width: 512,
        height: 512,
    };
    futures::executor::block_on(screenshot::run(descrip));
}
