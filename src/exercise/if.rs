// if1.rs
//
// Execute `rustlings hint if1` or use the `hint` watch subcommand for a hint.
// I AM NOT DONE
pub fn bigger(a: i32, b: i32) -> i32 {
    // Complete this function to return the bigger number!
    // Do not use:
    // - another function call
    // - additional variables
// return a>b ? a : b;
if a > b{
    return a;
}else{
    return b;
}
}
// Don't mind this for now :)
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ten_is_bigger_than_eight() {
        assert_eq!(10, bigger(10, 8));
    }
    #[test]
    fn fortytwo_is_bigger_than_thirtytwo() {
        assert_eq!(42, bigger(32, 42));
    }
}
// if2.rs
//
// Step 1: Make me compile!
// Step 2: Get the bar_for_fuzz and default_to_baz tests passing!
//
// Execute `rustlings hint if2` or use the `hint` watch subcommand for a hint.
// I AM NOT DONE
pub fn foo_if_fizz(fizzish: &str) -> &str {
    if fizzish == "fizz" {
        "foo"
    } else if fizzish == "fuzz" {
        "bar"
    }else{
        "baz"
    }
}
// No test changes needed!
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn foo_for_fizz() {
        assert_eq!(foo_if_fizz("fizz"), "foo")
    }
    #[test]
    fn bar_for_fuzz() {
        assert_eq!(foo_if_fizz("fuzz"), "bar")
    }
    #[test]
    fn default_to_baz() {
        assert_eq!(foo_if_fizz("literally anything"), "baz")
    }
}
