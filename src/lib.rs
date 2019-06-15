pub mod sha256;

#[cfg(test)]
mod tests {

    use sha256::{format_hash, hash, padding, parse};

    #[test]
    fn padding_test_0() {

        let testStr = "abc";

        let mut m = padding(testStr);

        assert_eq!((m.len() * 32) % 512, 0);

        let mut pop = m.pop();
        assert_eq!(pop, Some((testStr.len() as u32) * 8));

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

    #[test]
    fn parse_test_0() {
        let m = padding("str");

        let p = parse(&m);

        assert_eq!(p.len(), 1);
        assert_eq!(p[0].len(), 16);
    }

    #[test]
    fn hash_test_abc() {
        let hashed = hash("yasushi");

        println!("hashed \n
                 {:?}\n
                 {}", &hashed, format_hash(&hashed));
        assert_eq!(format_hash(&hashed), "d793a1fbaccefa73d1acf6cadab12e9648ebdf475093eafea3b1cc4842337c4e");
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

        println!("{}", &result);
        result
    }

}
