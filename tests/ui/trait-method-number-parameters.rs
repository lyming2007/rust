trait Foo {
    fn foo(&mut self, x: i32, y: i32) -> i32;
}

impl Foo for i32 {
    fn foo(
        &mut self, //~ ERROR
        //~| HELP: modify the signature to match the trait definition
        x: i32,
    ) {
    }
}

fn main() {}
