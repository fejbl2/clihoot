fn main() {
    let x = common::hello::Hello::World;
    let _y = common::questions::Questions { hello: x };
    println!("Hello, world!");
}
