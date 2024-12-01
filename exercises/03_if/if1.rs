fn bigger(a: i32, b: i32) -> i32 {
    // TODO: Complete this function to return the bigger number!
    // If both numbers are equal, any of them can be returned.
    // Do not use:
    // - another function call
    // - additional variables
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    // You can optionally experiment here.
    println!("bigger 2 or 5? {}", bigger(2, 5));
    println!("bigger 6 or 2? {}", bigger(6, 2));
    println!("bigger 200 or 99? {}", bigger(200,99));
    println!("bigger 9 or 9? {}", bigger(9, 9));
    
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

    #[test]
    fn equal_numbers() {
        assert_eq!(42, bigger(42, 42));
    }
}
