use std::collections::LinkedList;

pub fn merge<T: Ord + Clone>(list_a: LinkedList<T>, list_b: LinkedList<T>) -> LinkedList<T> {
    let mut merged_list = LinkedList::new();
    
    let mut current_a = list_a.clone(); // 克隆原始链表以保持不变
    let mut current_b = list_b.clone();

    // Merging the two lists
    while let (Some(a), Some(b)) = (current_a.front(), current_b.front()) {
        if a <= b {
            merged_list.push_back(a.clone()); // 添加元素
            current_a.pop_front(); // 移除前端元素
        } else {
            merged_list.push_back(b.clone());
            current_b.pop_front();
        }
    }

    // Add remaining elements from list_a
    while let Some(a) = current_a.pop_front() {
        merged_list.push_back(a); // 直接添加值
    }

    // Add remaining elements from list_b
    while let Some(b) = current_b.pop_front() {
        merged_list.push_back(b); // 直接添加值
    }

    merged_list
}
