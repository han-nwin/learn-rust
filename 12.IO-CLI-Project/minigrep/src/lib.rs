pub fn search<'a>(query: &str, file_path: &'a str) -> Vec<&'a str> {
    unimplemented!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = search("han", "poem.txt");
    }
}
