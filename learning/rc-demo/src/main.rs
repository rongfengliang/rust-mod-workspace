
use std::rc::Rc;
fn compare_option<T>(first:Option<T>,second:Option<T>) -> bool {
    match (first,second) {
      (Some(..),Some(..)) => true,
       _ => false,
    }
}
fn main() {
   let rc1 = Rc::new("dalongdemo");
   println!("print rc {}",Rc::strong_count(&rc1));
   let rc2 = rc1.clone();
   println!("print rc {}",Rc::strong_count(&rc2));
   let dalong : Option<String> = Some("dalong".into());
   let dalong2 : Option<String> = Some("dalong2".into());
   let result = compare_option(dalong,dalong2);
   println!("{}",result);

}
