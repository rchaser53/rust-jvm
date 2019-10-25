use r_jvm;

fn main() {
    r_jvm::execute(String::from("tests/class/HelloWorld"), false);
    r_jvm::execute(String::from("tests/class/FizzBuzz"), false);
    r_jvm::execute(String::from("tests/class/FizzBuzz2"), false);
}
