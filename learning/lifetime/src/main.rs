mod rong;
use rong::Rong;
fn main() {
   // Rong::demo();
   let mut i = 0;
   let p = & mut i;
   *p=1;
   println!("{}",p);
   println!("{}",i);
}
