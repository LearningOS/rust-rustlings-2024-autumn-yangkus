#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>), // 使用 Box 来包裹 List，以解决递归类型的问题
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    List::Nil // 返回空的 cons 列表
}

pub fn create_non_empty_list() -> List {
    List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil)))) // 创建一个包含两个元素的非空 cons 列表
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list());
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
