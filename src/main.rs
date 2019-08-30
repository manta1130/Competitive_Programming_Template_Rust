//提出時は下の二行を削除する。
extern crate cp_template;
use cp_template::*;

//提出用
#[allow(unused_imports)]
use graph::*;
#[allow(unused_imports)]
use input::*;
#[allow(unused_imports)]
use math::*;
#[allow(unused_imports)]
use modint::*;
#[allow(unused_imports)]
use ordfloat::*;
#[allow(unused_imports)]
use prime_number::*;
#[allow(unused_imports)]
use vectools::*;

fn main() {
    let a = OrdFloat(2.0);
    let b = OrdFloat(5.0);

    p(*std::cmp::max(a, b));
}
