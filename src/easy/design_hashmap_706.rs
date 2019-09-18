struct MyHashMap {
    // hash 桶, 内容存的是 key 和 value 对应的 offset. 也就是拿到这个 offset 去 key 和 value 内找.
    // 默认值是 -1, 表示这个桶还没有被 key 占用.
    hash_bucket: Vec<i32>,
    // 当我们遇到一个 hash_bucket 被占用, 我们用 next 来记录我们的 key chain.
    // 内容同样是 key 和 value 对应的 offset, 在 next 也被复用为 next pos.
    // 用 size 的递增来保证不会覆盖了已经写好的数据, 新数据只往后追加.
    next: Vec<i32>,
    keys: Vec<i32>,
    values: Vec<i32>,
    // 实际内容的 size, 也用于分配下一个新的 value pos 的 id 分配.
    size: usize,
    // 最大容量.
    capacity: usize,
}


/// https://leetcode.com/problems/design-hashmap/
/// 实现一个 hash map, 这里没有直接使用冲突探测, 而是采用了静态 id 分配的实现方法.
/// 这里的实现为了性能, 类型扩容, 空间考虑, 并希望尽量的避免引入复杂的数据类型以提高复用率,
/// 采用了 key, value 分开用数组密集存储 的 实现方式.
/// API 太简单, 暂时不考虑 capacity 扩容问题, 我们选择在一开始开一个大内存块.
/// 题目说操作不超过 1w, 那么空间开 1w 自然就够了.
impl MyHashMap {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    pub(self) fn new() -> Self {
        let mut ret = MyHashMap {
            hash_bucket: vec![],
            next: vec![],
            keys: vec![],
            values: vec![],
            size: 0,
            capacity: 10000,
        };
        ret.hash_bucket.resize(ret.capacity, -1);
        ret.next.resize(ret.capacity, -1);
        ret.keys.resize(ret.capacity, -1);
        ret.values.resize(ret.capacity, -1);
        ret
    }

    // 更新规则: 不存在 key 则插入, 存在则更新.
    // 注意如果不存在 key, 改动的可能是 hash_bucket 或者 next 的一个.
    // hash 函数设计的好, 可以使得大部分 key 落在 hash_bucket 内, 只需要一次查询即可.
    // next 部分极限情况下, 会退化到 O(n) 的查找和插入复杂度, 但是在实际数据分布下会呈现类似跳表的查询复杂度.
    // next 插入部分, 的确是有可能破坏别的 hash_bucket 的 key chain, 但是因为 key 不一样, 所以最后
    // 总是会插入新的位置(类似 chain 复用的感觉, 多个 key 的 chain 会串到一起)
    // 查询不存在的 key 的时候会造成一些无用查询.
    // TODO 这个可以用可变引用来统一修改 hash_bucket 和 next, 但是实现会遇到 borrow mut twice 的问题.
    #[allow(dead_code)]
    pub(self) fn put(&mut self, key: i32, value: i32) {
        let hash_key = self.hash(key);
        // 先查询 hash_bucket.
        let hash_offset = self.hash_bucket[hash_key as usize];



    }

    // 查询流程: 先在 hash_bucket 里面查, key 对不上则去 next 里拿 key chain(因为 hash 冲突) 来查.
    // 查不到 return -1
    #[allow(dead_code)]
    pub(self) fn get(&self, key: i32) -> i32 {
        let hash_key = self.hash(key);
        let mut kv_offset = self.hash_bucket[hash_key as usize];

        while kv_offset >= 0 && self.keys[kv_offset as usize] != key {
            kv_offset = self.next[kv_offset as usize]
        }
        if kv_offset >= 0 { self.values[kv_offset as usize] } else { -1 }
    }

    /** Removes the mapping of the specified value key if this map contains a mapping for the key */
    #[allow(dead_code)]
    pub(self) fn remove(&mut self, key: i32) {}


    // TODO hash 可以配置为外部闭包.
    fn hash(&self, key: i32) -> i32 {
        key % self.capacity as i32
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


