// Book example
fn largest<T: PartialOrd + Copy>(list: &[T])-> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

// Implemented same example with references
fn largest_ref<T: PartialOrd>(list: &[T])-> &T {
    let mut largest = &list[0];
    for item in list {
        if *item > *largest {
            largest = item;
        }
    }
    largest
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    use super::{
         largest,
         largest_ref,
    };
    const INTEGERS:[i32;5] = [34,50,25,100,65];
    const CHARS:[char;4] = ['y', 'm', 'a', 'q'];

    #[test]
    fn test1() {
        assert_eq!(largest(&INTEGERS), 100);
    }

    #[test]
    fn test2() {
        assert_eq!(largest(&CHARS), 'y');
    }

    #[test]
    fn test_ref1() {
        assert_eq!(*largest_ref(&INTEGERS), 100);
    }

    #[test]
    fn test_ref2() {
        assert_eq!(*largest_ref(&CHARS), 'y');
    }
}