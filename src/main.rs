extern crate users;
extern crate app_modmetrics;
use serde::{Serialize, Deserialize};
use serde_json;
use users::{user,aaa};
use app_modmetrics::metrics::{usermetrics};

#[macro_use]
extern crate macros_lib;

#[derive(Serialize, Deserialize, Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn demo(){
let point = Point { x: 1, y: 2 };

    // Convert the Point to a JSON string.
    let serialized = serde_json::to_string(&point).unwrap();

    // Prints serialized = {"x":1,"y":2}
    println!("serialized = {}", serialized);

    // Convert the JSON string back to a Point.
    let deserialized: Point = serde_json::from_str(&serialized).unwrap();

    let user = user::User{name: "dalong".into(),age:33u32};
    // Prints deserialized = Point { x: 1, y: 2 }
    println!("deserialized = {:?}", deserialized);
    let usermetrics = usermetrics::UserMetrics{metrics_name:"dalongdemo".into(),metrics_counts:43};
    println!("{:?},{:?}",user,usermetrics);
}
fn main() {
    dalong_demo!();
}
