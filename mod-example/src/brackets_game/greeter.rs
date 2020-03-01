pub struct Greeter {}

impl Greeter {
    pub fn hello(self, who: String) {
        println!("Hello {}", who);
    }
}
