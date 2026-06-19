pub fn search<'a>(query: &str, contents: String) -> Vec<&'a str> {
    let n = vec!["2", "3", "4"];
    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = search("han", String::from("poem.txt"));
    }
}
