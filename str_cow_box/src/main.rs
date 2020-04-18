use std::borrow::Cow;

#[derive(Debug)]
struct StrBoxStruct<'a> {
    b: &'a str,
    u: i32,
}

impl<'a> StrBoxStruct<'a> {
    fn ret_str_by_b(&self) -> &str {
        match &self.b {
            &"0" => "ten",
            _ => "invalid",
        }
    }
}

// https://hermanradtke.com/2015/05/29/creating-a-rust-function-that-returns-string-or-str.html
fn add_str_to_str<'b>(input: &'b str) -> Cow<'b, str> {
    let mut s = String::from(input);
    s.push_str("-");
    s.push_str(&input);
    s.into()
}

fn main() {
    let s = add_str_to_str("yeh");
    println!("{}", s);

    let boxed_int = Box::<i32>::new(5);
    let sbs = StrBoxStruct {
        b: &s,
        u: *boxed_int,
    };
    println!("{:?}", sbs);

    println!("{}", sbs.ret_str_by_b());
}
