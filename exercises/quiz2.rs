// quiz2.rs
// This is a quiz for the following sections:
// - Strings

// Ok, here are a bunch of values-- some are `String`s, some are `&str`s. Your
// task is to call one of these two functions on each value depending on what
// you think each value is. That is, add either `string_slice` or `string`
// before the parentheses on each line. If you're right, it will compile!

fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue"); // &str literal
    string("red".to_string()); // &str method to convert to String
    string(String::from("hi")); // String static method to convert to String
    string("rust is fun!".to_owned()); // &str method to create an owned version, which happens to be a String
    string_slice("nice weather".into()); string("nice weather".into()); // does both: .into() allows an implied conversion
    string(format!("Interpolation {}", "Station")); // format!() produces a value which cannot be determined from slicing a single string, so a new String must be created
    string_slice(&String::from("abc")[0..1]); // &str (literal) -> String (String::from()) -> &String (&) -> &str ([0..1])
    string_slice("  hello there ".trim()); // .trim() can be determined from slicing a &str, so it produces a &str
    string("Happy Monday!".to_string().replace("Mon", "Tues")); // .replace() produces a value which cannot be determined from slicing the String, so a new String must be created
    string("mY sHiFt KeY iS sTiCkY".to_lowercase()); // .to_lowercase() produces a value which cannot be determined from slicing the &str, so a new String must be created
}
