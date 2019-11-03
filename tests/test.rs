use r_jvm;

fn test_helper(file_name: String) {
    println!("** {} **", &file_name);
    r_jvm::execute(file_name, 0);
    println!("");
}

fn main() {
    test_helper(String::from("tests/class/HelloWorld"));
    test_helper(String::from("tests/class/FizzBuzz"));
    test_helper(String::from("tests/class/FizzBuzz2"));
    test_helper(String::from("tests/class/NewAndCallInstanceMethod"));
    test_helper(String::from("tests/class/InitializeStatic"));
    test_helper(String::from("tests/class/Switch"));
    test_helper(String::from("tests/class/InstanceField"));
    test_helper(String::from("tests/class/PrimitiveArray"));
    test_helper(String::from("tests/class/CustomArray"));
}
