use rustamath::random::lottery::lottery;

fn main() {
    let list = lottery(1u8, 10u8);
    println!("Lottery {:?}", list);

    let list = lottery(1u8, 59u8);
    println!("Lottery {:?}", &list[..6]);
}
