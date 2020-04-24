use  std::any::type_name;

struct Foo {}

fn print_type_name_of<T>(_: T) {
    println!("{}", type_name::<T>() )
}

fn main() {
    let x = &false;
    print_type_name_of(x);

    let &x = &false;
    print_type_name_of(x);

    let ref x = &false;
    print_type_name_of(x);

    let foo = Foo{};
    print_type_name_of(foo);

    let foo = &Foo{};
    print_type_name_of(foo);

}

