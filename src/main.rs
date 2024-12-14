use librust::hosting;
use librust::{umaidashi::*, *};

fn main() {
    let result = add(2, 2);
    umaidashi();
    println!("result: {}", result);

    hosting::add_to_waitlist();
}
