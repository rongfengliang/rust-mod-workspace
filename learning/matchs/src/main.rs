fn main() {
    let x = 5_i32;
    match x {
        ref x => println!("{}",x),
    }
    let mut  usertoken: Option<String> = Some("dalongdemo".into());
    match usertoken {
        Some(ref r) => println!("{}",r),
        _ => println!("{}","nothing")
    }
    println!("{:?}",usertoken);
    if let Some(result) = usertoken {
        println!("{:?}",result);
    }
}
