// options2.rs
//
// Execute `rustlings hint options2` or use the `hint` watch subcommand for a
// hint.


#[cfg(test)]
mod tests {
    #[test]
    fn simple_option() {
        let target = "rustlings";
        let optional_target = Some(target);
    
        // 使用 if let 语法解构 `Some` 类型的值
        if let Some(word) = optional_target {
            assert_eq!(word, target);
        }
    }
    
    #[test]
    fn layered_option() {
        let range = 10;
        let mut optional_integers: Vec<Option<i8>> = vec![None];
    
        for i in 1..(range + 1) {
            optional_integers.push(Some(i));
        }
    
        let mut cursor = range;
    
        // 使用 while let 解构 `Option<Option<i8>>`
        while let Some(Some(integer)) = optional_integers.pop() {
            assert_eq!(integer, cursor);
            cursor -= 1;
        }
    
        assert_eq!(cursor, 0);
    }
    

}
