pub fn convertir_pascal_case(input: &str) -> String {
    input
        .split(' ')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => {
                    let first_upper = first.to_uppercase().collect::<String>();
                    let rest = chars.flat_map(|c| c.to_lowercase()).collect::<String>();
                    first_upper + &rest
                }
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ken_format() {
        assert_eq!(convertir_pascal_case("hEllo"), "Hello");
        assert_eq!(convertir_pascal_case("hEllo wOrld!"), "Hello World!");
        assert_eq!(convertir_pascal_case("á"), "Á");
        assert_eq!(convertir_pascal_case(""), "");
        assert_eq!(
            convertir_pascal_case("hÉllo wÓrld! hOw aRe yOu?"),
            "Héllo Wórld! How Are You?"
        );
        assert_eq!(convertir_pascal_case("123"), "123");
        assert_eq!(convertir_pascal_case("h|éÉ"), "H|éé");
        assert_eq!(convertir_pascal_case("hEllo   wOrld"), "Hello   World");
        assert_eq!(convertir_pascal_case("@À#$"), "@à#$");
    }
}
