fn sort<T: Ord + Clone>(array: &mut [T]) {
    let len = array.len();
    for i in 1..len {
        let key = array[i].clone(); // 获取当前元素
        let mut j = i;
        // 将比 key 大的元素向右移动一位
        while j > 0 && array[j - 1] > key {
            array[j] = array[j - 1].clone(); // 使用 clone() 复制元素
            j -= 1;
        }
        array[j] = key; // 将 key 插入到正确位置
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }

    #[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }

    #[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}
