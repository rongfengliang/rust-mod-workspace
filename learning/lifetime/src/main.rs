mod rong;
use rong::Rong;
fn main() {
    let rong = Rong{first_name:"dalong",second_name:"demoapp",third_name:"demoapp"};
    println!("{:?}",rong);
    rong.printname();
}
