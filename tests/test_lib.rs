#[cfg(test)]
mod tests {
    use easy_yaml::add;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
