// use cargo_and_crates::kinds::PrimaryColor;
use cargo_and_crates::PrimaryColor;
use cargo_and_crates::utils::mix;
fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
