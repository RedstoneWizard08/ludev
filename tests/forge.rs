mod tests {
    use std::collections::HashMap;

    use ludic::get_forge_versions;

    #[test]
    pub fn test_forge() {
        let res: HashMap<String, Vec<String>> = futures::executor::block_on(get_forge_versions());
        println!("{:?}", res);
    }
}
