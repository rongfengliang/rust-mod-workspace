#[derive(Debug)]
pub struct Add{
  pub first: i32,
  pub second: i32,
  pub third: i32,
}

// impl for default 
impl Default for Add {
   fn default() -> Add {
      Add {
         first:0,
         second:0,
         third:0,
      }
   }
}
impl Add {
   pub fn add(first: u32, second: u32) -> u32 {
       first+second
   }
   pub fn add2(&self,first: u32, second: u32) -> u32 {
      first+second
   }
}