// use cargo_cratesio::kinds::PrimaryColor;
// use cargo_cratesio::utils::mix;

use cargo_cratesio::PrimaryColor;
use cargo_cratesio::mix;

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}