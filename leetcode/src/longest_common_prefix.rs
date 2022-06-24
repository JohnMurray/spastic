use std::cmp::min;

fn longest_common_prefix(strs: Vec<String>) -> String {
    let mut prefix = strs[0].as_str();
    for s in strs.iter().skip(1) {
        if prefix.len() <= s.len() && &s[0..prefix.len()] == prefix {
            continue;
        }
        
        for i in 0..(min(prefix.len(), s.len())) {
            if prefix.chars().nth(i).unwrap() != s.chars().nth(i).unwrap() {
                prefix = &prefix[..i];
                break;
            }
        }
        if prefix.len() > s.len() {
            prefix = &s[..];
        }

        if prefix.len() == 0 {
            break;
        }
    }

    prefix.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_common_prefix() {
        // assert_eq!("fl".to_string(), longest_common_prefix(vec!["flower".to_string(), "flow".to_string(), "flight".to_string()]));
        // assert_eq!("".to_string(),
        //     longest_common_prefix(vec![
        //         "biscuit".to_string(),
        //         "bichone".to_string(),
        //         "cracker".to_string(),
        //         "darn'it".to_string(),
        //     ]),
        // );

        // assert_eq!("biscuit".to_string(), longest_common_prefix(vec!["biscuit".to_string()]));
        assert_eq!("a".to_string(), longest_common_prefix(vec!["ab".to_string(), "a".to_string()]));
    }
}