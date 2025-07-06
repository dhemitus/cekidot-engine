use cekidot::run;

fn main() {
    pollster::block_on(run(120, 640, 480, "glfw loop"))
}
