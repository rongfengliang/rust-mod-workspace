
extern crate users;

extern crate libmods;

extern crate moddemo;

// short call for moddemo mod
use moddemo::app::userlogin::{UserLogin};

use libmods::appdemos::MyAppdemo;
fn main() {
    let appmodinfo =MyAppdemo{appname:"dlaongdemo".into(),appversion: "v1.0".into()};
    let moddemo =UserLogin{name:"dlaongdemo".into()};
    print!("{:?}",appmodinfo);
}
