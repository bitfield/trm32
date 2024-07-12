use trm32::Machine;

fn main() {
    let m = Machine::new();
    println!("{}", m.state());
}
