// goog youtube link: https://www.youtube.com/watch?v=nvur2Ast8hE
//
trait Draw {
    fn draw(&self);
}

#[derive(Debug)]
struct Screen<'a> {
    s: &'a str,
}

impl<'a> Draw for Screen<'a> {
    fn draw(&self) {
        println!("draw: {}", self.s);
    }
}

#[derive(Debug)]
struct DrawUser<T>
where
    T: Draw,
{
    pub draw: T,
}

impl<T> DrawUser<T>
where
    T: Draw,
{
    #[allow(dead_code)]
    fn new(&mut self, d: T) {
        self.draw = d;
    }
}

fn main() {
    let s = DrawUser {
        draw: Screen { s: "s" },
    };
    println!("{:?}", s);

    let mut sx = DrawUser {
        draw: Screen { s: "_s" },
    };
    sx.new(Screen { s: "sx" });
    println!("{:?}", sx);
}
