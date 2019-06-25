
extern crate users;

extern crate libmods;

extern crate moddemo;

mod app;
// short call for moddemo mod
use moddemo::app::userlogin::{UserLogin};
use app::add::{Add};
use libmods::appdemos::MyAppdemo;
fn main() {
    let appmodinfo =MyAppdemo{appname:"dlaongdemo".into(),appversion: "v1.0".into()};
    let moddemo =UserLogin{name:"dlaongdemo".into()};
    // default demo
    let addop: Add = Add {third:444, ..Default::default()};
    print!("{:?}",addop);
    let result = Add::add(12,44);
    let result2 = addop.add2(33,444);
    print!("{:?},\r\n{},\r\n{}",appmodinfo,result,result2);
}
