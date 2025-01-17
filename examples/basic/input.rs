use nyanko_engine::input::PlatformInput;

fn main() {
    let mut input = PlatformInput::new();
    input.update();

    println!("Basic input example initialized");
}
