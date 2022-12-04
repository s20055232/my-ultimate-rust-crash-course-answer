// Part 2
const STARTING_MISSILES: i32 = 8;
const READY_AMOUNT: i32 = 2;

fn main() {
    // Part 1
    let missiles = 8;
    let ready = 2;
    println!("Firing {} of my {} missiles...", ready, missiles);
    // Part 2
    // missiles = missiles - ready; // this should cause error.
    let mut missiles = missiles;
    missiles = missiles - ready;
    println!("{} missiles left", missiles);
    let _missiles = STARTING_MISSILES;
    let _ready = READY_AMOUNT;
    // Extra
    let missiles: i32 = 8;
    let ready = 2 as i32;
    // warning: variable does not need to be mutable
    // let (mut missiles, mut ready) = (missiles, ready);
    let (missiles, ready) = (missiles, ready);
    println!("{}", missiles - ready);
    let _extra_var = 123;
    // error: constant cannot assign here
    // let READY_AMOUNT = 123;
}
