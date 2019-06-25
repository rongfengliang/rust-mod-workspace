
use std::rc::Rc;
fn main() {
   let rc1 = Rc::new("dalongdemo");
   println!("print rc {}",Rc::strong_count(&rc1));
   let rc2 = rc1.clone();
   println!("print rc {}",Rc::strong_count(&rc2));
}
