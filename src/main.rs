const STARTING_MISSILES: i32 = 10;
const READY_AMOUNT: i32 = 6;

fn main() {
    println!("Rust 101 - Demo application");
    let mut missiles = STARTING_MISSILES;
    let ready: i32 = READY_AMOUNT;
    println!("Firing {} of my {} missiles...", ready, missiles);
    missiles = missiles - ready;
    println!("{} Missiles left", missiles);
}
