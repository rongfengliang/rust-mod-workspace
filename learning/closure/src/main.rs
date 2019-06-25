
fn make_adder(x:i32) -> Box<Fn(i32) -> i32> {
  Box::new(move |y| x+y)
}

fn main() {

    // let f = make_adder(33);
    // println!("{}",f(1));
    let x = 1_i32;
    let add_x = |a| x+a;
    let result =add_x(x);
    println!("{}",result);
    println!("{}",x);

}
