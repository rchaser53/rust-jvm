use r_jvm;

fn main() {
    r_jvm::execute(String::from("tests/class/HelloWorld"), 0);
    r_jvm::execute(String::from("tests/class/FizzBuzz"), 0);
    r_jvm::execute(String::from("tests/class/FizzBuzz2"), 0);
    r_jvm::execute(String::from("tests/class/NewAndCallInstanceMethod"), 0);
    r_jvm::execute(String::from("tests/class/InitializeStatic"), 0);
    r_jvm::execute(String::from("tests/class/Switch"), 0);
}
