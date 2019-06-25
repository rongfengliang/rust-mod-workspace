#[derive(Debug)]
pub struct Add{
  pub first: i32,
  pub second: i32,
  pub third: i32,
}

#[derive(Debug)]
pub struct Rong<'a>{
  pub first_name: &'a str,
  pub second_name: &'a str,
  pub last_name: &'a str,
}

#[derive(Debug,Default)]
pub struct AppDefault {
    first: i32,
    second: i32,
}

impl AppDefault  {
   pub fn add(&self)->i32 {
     self.first+self.second
   }
}
impl Drop for AppDefault {
   fn drop(&mut self) {
      println!("desc  {:?} app  default struct",self);
   }
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