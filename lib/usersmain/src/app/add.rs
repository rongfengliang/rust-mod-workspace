#[derive(Debug)]
pub struct Add{

}
impl Add {
   pub fn add(first: u32, second: u32) -> u32 {
       first+second
   }
   pub fn add2(&self,first: u32, second: u32) -> u32 {
      first+second
   }
}