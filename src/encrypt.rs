use std::collections::HashMap;

fn num_to_char(num: &str) -> &str {
    let hash = vec![
        ("01".to_owned(), "A"),
        ("02".to_owned(), "B"),
        ("03".to_owned(), "C"),
        ("04".to_owned(), "D"),
        ("05".to_owned(), "E"),
        ("06".to_owned(), "F"),
        ("07".to_owned(), "G"),
        ("08".to_owned(), "H"),
        ("09".to_owned(), "I"),
        ("10".to_owned(), "J"),
        ("11".to_owned(), "K"),
        ("12".to_owned(), "L"),
        ("13".to_owned(), "M"),
        ("14".to_owned(), "N"),
        ("15".to_owned(), "O"),
        ("16".to_owned(), "P"),
        ("17".to_owned(), "Q"),
        ("18".to_owned(), "R"),
        ("19".to_owned(), "S"),
        ("20".to_owned(), "T"),
        ("21".to_owned(), "U"),
        ("22".to_owned(), "V"),
        ("23".to_owned(), "W"),
        ("24".to_owned(), "X"),
        ("25".to_owned(), "Y"),
        ("26".to_owned(), "Z"),
    ].into_iter().collect::<HashMap<_,_>>();

    return hash.get(num).unwrap();
}


pub fn convert(input: &str) -> String {
    let mut s = input.to_string();
    if input.len() % 2 != 0 {
        s = "0".to_string() + input;
    }

    let mut res = String::new();
    for i in (0..s.len()).step_by(2) {
        let num = &s[i..i+2];
        res = res + num_to_char(num);
    }

    return res;
}
