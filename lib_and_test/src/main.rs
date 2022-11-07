use myadd;

fn main() {
    let x = myadd::add(10, 20);
    println!("Hello, world!, add:{x}");
    //panic!("test");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_add() {
        let result = myadd::add(4, 10);
        assert_eq!(result, 14);
    }
}
