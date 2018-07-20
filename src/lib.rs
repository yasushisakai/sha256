pub mod sha256;

#[cfg(test)]
mod tests {

    use sha256::padding;

    #[test]
    fn padding_test_0() {

        let str = String::from("abc");

        let mut m = padding(str.clone());

        assert_eq!((m.len() * 32) % 512, 0);

        let mut pop = m.pop();
        assert_eq!(pop, Some((str.len() as u32) * 8));

        for _ in 0..14 {
            pop = m.pop();
            assert_eq!(pop, Some(0));
        }

        let a = u32::from('a') << (8 * 3);
        let b = u32::from('b') << (8 * 2);
        let c = u32::from('c') << (8 * 1);
        let one = 1 << 7;

        let sum = a + b + c + one;

        assert_eq!(m.pop().unwrap(), sum);

        println!("{}", show_binary(&sum));

    }

    fn show_binary(n: &u32) -> String {
        let raw_string = format!("{:032b}", n);
        let mut chars = raw_string.chars();
        let mut i = 0;
        let mut result = String::new();
        loop {
            match chars.next() {
                Some(c) => {
                    result.push(c);
                },
                None => break
            }

            if i % 8 == 7 {
                result.push(' ');
            }
                    i = i + 1;
        }

        println!("{}", result);
        String::from("hello")
    }

}
