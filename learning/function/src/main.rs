
fn add(first:i32,second: i32)->i32 {
    first+second
}

fn add2(first:i32,second: i32)->i32 {
    first+second
}
fn main() {

    let mut myfn:fn(first:i32,second: i32)->i32= add;
    myfn = add2;
    print!("result: {:?}",myfn(11,33));

}
