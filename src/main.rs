mod graphics;

use graphics::screenshot;

fn main() {
    use futures::executor::block_on;
    block_on(screenshot::run());
}
