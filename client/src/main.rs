fn main() {
    let x = common::hello::Hello::World;
    let y = common::questions::Questions { hello: x };
    println!("Hello, world!");
}
