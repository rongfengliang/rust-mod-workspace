#[derive(Debug)]
pub struct Rong<'a> {
    pub first_name: &'a str,
    pub second_name: &'a str,
    pub third_name: &'a str,
}