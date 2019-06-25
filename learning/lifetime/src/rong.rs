#[derive(Debug,Clone,Copy)]
pub struct Rong<'a> {
    pub first_name: &'a str,
    pub second_name: &'a str,
    pub third_name: &'a str,
}

impl<'a>  Rong<'a> {
   pub fn printname(&self) {
       println!("{}",self.first_name);
   }
   pub fn demo(){
    let mut rong = Rong{first_name:"dalong",second_name:"demoapp",third_name:"demoapp"};
    println!("{:?}",rong);
    let newrong = rong;
    rong.printname();
    newrong.printname();
    println!("{:?}",rong);
   }
}