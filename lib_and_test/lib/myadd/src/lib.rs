pub fn add(left: usize, right: usize) -> usize {
    add_two(1);
    left + right
}

pub fn add_three(x: usize) -> usize {
    x + 3
}

fn add_two(x: usize) -> usize {
    x + 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test1() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn test2() {
        let result = add_two(4);
        assert_eq!(result, 6);
    }
}
