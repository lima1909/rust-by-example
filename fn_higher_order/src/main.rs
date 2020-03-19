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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_my_fn() {
        assert_eq!(5, call_my_fn(4, plus_one));
    }

    #[test]
    fn test_call_my_fn_closure() {
        assert_eq!(5, call_my_fn(4, |x| x + 1));
    }
}
