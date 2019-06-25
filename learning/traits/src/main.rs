
mod app;
use app::defaultlogin;
use crate::app::userlogin::UserLogin;
fn main() {
  let login =  defaultlogin::DefaultUserLogin{
      name: "dalong".into(),
      password: "dalong".into(),
  };
 // println!("{:?}",login);
  let logintoken = login.login("dalong".into(),"dalong".into());
  println!("{:?}",logintoken);
  let userid = 222i32;
  let logintoken2 = userid.login("dalong".into(),"dalong".into());
  println!("{:?}",logintoken2);
}
