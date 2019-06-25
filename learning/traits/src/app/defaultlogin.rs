
use super::userlogin::UserLogin;
#[derive(Debug)]
pub struct DefaultUserLogin {
  pub name: String,
  pub password: String,
}

// extend method demo
impl UserLogin for i32 {
  fn login(&self,username:String, userpassword:String) -> String{
       if (username=="dalong"&&userpassword=="dalong") {
         return "dalongdemo".into();
       }else {
          return "error".into();
       }
   }
}
impl UserLogin for DefaultUserLogin {
    fn login(&self,username:String, userpassword:String) -> String{
       if (self.name==username && self.password==userpassword){
          return "sometoken for validate".into();
       }
       else {
           return "error".into();
       }
   }
}