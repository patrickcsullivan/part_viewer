mod graphics;

use graphics::screenshot;

fn main() {
    let descrip = screenshot::ScreenshotDescriptor {
        mesh_path: "res/utah_teapot.stl",
        dst_path: "triangle.png",
        width: 256,
        height: 256,
    };
    futures::executor::block_on(screenshot::run(descrip));
}
