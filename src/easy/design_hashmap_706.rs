struct MyHashMap {

}


/// https://leetcode.com/problems/design-hashmap/
/// 实现一个 hash map, 这里实现为线性冲突探测算法.
/// 这里的实现为了性能和空间考虑, 并希望尽量的避免引入复杂的数据类型以提高复用率, 采用了
/// id 存储 + value 数组密集存储 的 二次探测查找.
/// API 太简单, 暂时不考虑 capacity 扩容问题, 我们选择在一开始开一个大内存块.
/// 题目说操作不超过 1w, 那么空间开 1w 自然就够了.
impl MyHashMap {

    /** Initialize your data structure here. */
    #[allow(dead_code)]
    pub(self) fn new() -> Self {
        MyHashMap {}
    }

    /** value will always be non-negative. */
    #[allow(dead_code)]
    pub(self) fn put(&mut self, key: i32, value: i32) {

    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    #[allow(dead_code)]
    pub(self) fn get(&mut self, key: i32) -> i32 {
        0
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    #[allow(dead_code)]
    pub(self) fn remove(&mut self, key: i32) {

    }
}


#[cfg(test)]
mod tests {
    use super::MyHashMap;

    #[test]
    pub fn my_hash_map_test() {
        let mut map = MyHashMap::new();
        map.put(1, 100);
        map.put(100, 1);
        for i in 0..30 {
            map.put(1 << i, 0);
        }

        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(-1), -1);
        assert_eq!(map.get(1), 100);
        for i in 0..30 {
            assert_eq!(map.get(1 << i), 0);
        }

        map.remove(100);
        map.remove(1);
        assert_eq!(map.get(100), -1);
        assert_eq!(map.get(1), -1);
    }
}


