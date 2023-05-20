#[link(name = "gl")]
extern "C" {
    fn glGetError() -> u32;
}

fn main() {
    println!("0x{:04x}", unsafe { glGetError() });
}
