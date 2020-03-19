type MyFn = fn(i: i32) -> i32;

fn call_my_fn(x: i32, f: MyFn) -> i32 {
    f(x)
}

fn plus_one(i: i32) -> i32 {
    i + 1
}

fn main() {
    call_my_fn(3, |x| {
        println!("result: {}", x);
        x
    });

    println!("plus one: {}", call_my_fn(5, plus_one));
}
