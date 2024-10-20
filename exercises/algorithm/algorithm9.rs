use std::cmp::Ord;
use std::default::Default;

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()], // 创建一个容量为1的默认元素
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        self.count += 1; // 增加元素计数
        if self.count < self.items.len() {
            self.items[self.count] = value; // 直接放入现有空间
        } else {
            self.items.push(value); // 添加新元素
        }
        self.bubble_up(self.count); // 进行向上调整
    }

    fn parent_idx(&self, idx: usize) -> usize {
        idx / 2
    }

    fn children_present(&self, idx: usize) -> bool {
        self.left_child_idx(idx) <= self.count
    }

    fn left_child_idx(&self, idx: usize) -> usize {
        idx * 2
    }

    fn right_child_idx(&self, idx: usize) -> usize {
        self.left_child_idx(idx) + 1
    }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        let left_child = self.left_child_idx(idx);
        let right_child = self.right_child_idx(idx);
        
        if right_child <= self.count {
            if (self.comparator)(&self.items[left_child], &self.items[right_child]) {
                left_child
            } else {
                right_child
            }
        } else {
            left_child
        }
    }

    fn bubble_up(&mut self, idx: usize) {
        let mut idx = idx;
        while idx > 1 {
            let parent_index = self.parent_idx(idx); // 先计算父节点的索引
            if (self.comparator)(&self.items[idx], &self.items[parent_index]) {
                self.items.swap(idx, parent_index); // 交换
                idx = parent_index; // 更新当前索引
            } else {
                break; // 如果当前元素不小于父元素，则退出
            }
        }
    }

    fn bubble_down(&mut self, idx: usize) {
        let mut idx = idx;
        while self.children_present(idx) {
            let smaller_child_idx = self.smallest_child_idx(idx);
            if (self.comparator)(&self.items[idx], &self.items[smaller_child_idx]) {
                break;
            }
            self.items.swap(idx, smaller_child_idx);
            idx = smaller_child_idx;
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default + Clone, // 这里要求 T 实现 Clone
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        let root_value = self.items[1].clone(); // 使用 clone 而不是移动
        self.items.swap(1, self.count); // 将最后一个元素移到根节点
        self.count -= 1; // 减少计数
        self.bubble_down(1); // 进行向下调整

        Some(root_value) // 返回根节点的值
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}
