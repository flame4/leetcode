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
    fn new() -> Self {
        MyHashMap {}
    }

    /** value will always be non-negative. */
    fn put(&self, key: i32, value: i32) {

    }

    /** Returns the value to which the specified key is mapped, or -1 if this map contains no mapping for the key */
    fn get(&self, key: i32) -> i32 {

    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    fn remove(&self, key: i32) {

    }
}