struct MyHashMap {
    // hash 桶, 内容存的是 某个 key 对应的 key, value offset.
    // 也就是拿到这个 offset 去 key 里内找id, 再去 value 里找 value.
    // 默认值是 -1, 表示这个桶还没有被 key 占用.
    hash_bucket: Vec<i32>,
    // 当我们遇到一个 hash_bucket 被占用, 我们用 next 来记录我们的 key chain.
    // next[i] = v 中, i 表示的是 key, value 的 offset, 要用 i 去查
    // 当查到 key 不相等, v 就是下一轮迭代的 i, 也就是 chain 内的下一个 offset.
    // 所以要区分开, hash bucket 内的值指的是 offset, 而 next 的下标指的是 offset.
    // 而一个巧妙复用的地方是, hash bucket 和 next 关联是, hash bucket 当 key 匹配不上的时候,
    // 也用这个 offset 来关联第一次进入 next 查找的 offset. 具体的参考 put 的实现.
    next: Vec<i32>,
    // 存的是给这个 key 分配的 id, 要拿着这个 id 作为 offset 去 value 内取值即可.
    keys: Vec<i32>,
    // 所有的 id 以 pool 的形式来管理, 初始化所有的 id, 如果一个 id
    // 使用的方式是循环队列.
    id_pool: Vec<i32>,
    // 记录当前循环队列的开始位置.
    pool_start: usize,
    // 记录当前 id_pool 内还有多少可用的 id.
    pool_id_num: usize,

    // mask 用于筛除多余的 bit 数, 起到类似 hash 取余的作用, 但是速度会快很多.
    hash_key_mask: usize,
    pool_mask: usize,

    // 用拿到的 id 来 value 内取值.
    values: Vec<i32>,
}


/// https://leetcode.com/problems/design-hashmap/
/// 实现一个 hash map, 这里没有直接使用冲突探测, 而是采用了静态 id 分配的实现方法.
/// 这里的实现为了性能, 类型扩容, 空间密集, 并希望尽量的避免引入复杂的数据类型以提高复用率,
/// 采用了 key, value 分开用数组密集存储的实现方式.
/// API 太简单, 暂时不考虑 capacity 扩容问题, 我们选择在一开始开一个大内存块, 并以 2 的次幂来定义长度.
/// 题目说操作不超过 1w, 那么空间开 8192(2 ^ 13) 应该就够了.
/// 单写多读安全.
impl MyHashMap {
    /** Initialize your data structure here. */
    #[allow(dead_code)]
    pub(self) fn new() -> Self {
        let capacity = 1 << 13;
        let mut ret = MyHashMap {
            hash_bucket: vec![],
            next: vec![],
            keys: vec![],
            id_pool: vec![],
            pool_start: 0,
            pool_id_num: capacity,
            hash_key_mask: capacity - 1,
            pool_mask: capacity - 1,
            values: vec![],
        };
        ret.hash_bucket.resize(capacity, -1);
        ret.next.resize(capacity, -1);
        ret.keys.resize(capacity, 0);
        ret.id_pool.resize(capacity, 0);
        // TODO 简化写法?
        for i in 0..capacity {
            ret.id_pool[i] = i as i32;
        }
        ret.values.resize(capacity, 0);
        ret
    }

    // 更新规则: 不存在 key 则插入, 存在则更新.
    #[allow(dead_code)]
    pub(self) fn put(&mut self, key: i32, value: i32) {
        let term_id = self.insert_id(key);
        assert!(term_id >= 0);
        self.values[term_id as usize] = value;
    }

    // 如果 key 已经存在于词典中, 返回其被分配的 id, 否则将 key 加入字典, 并返回其 id.
    // TODO 这个可以用可变引用来统一修改 hash_bucket 和 next, 但是实现会遇到 borrow mut twice 的问题.
    // 要么用 unsafe, 要么把 next 和 hash_bucket 代码分开处理, 这里选后者.
    fn insert_id(&mut self, key: i32) -> i32 {
        // 先查询是否 key 已经存在了.
        let mut key_offset = self.hash_bucket[key as usize & self.hash_key_mask];
        // 查询 bucket 内是否存在.

        // key 在 hash 桶没有被占用.
        if key_offset < 0 {
            let new_id = allocate_new_id(self);
            self.keys[new_id as usize] = key;
            self.hash_bucket[key as usize & self.hash_key_mask] = new_id;
            return new_id;
        }

        // key 在 hash 桶内被占用且 key 命中.
        if self.keys[key_offset as usize] == key {
            return key_offset;
        }

        // key 不在 hash bucket 内, 处理在 next 内的情况.
        while self.next[key_offset as usize] >= 0 {
            if self.keys[self.next[key_offset as usize] as usize] == key {
                return self.next[key_offset as usize];
            }
            key_offset = self.next[key_offset as usize];
        }

        // 没找到这个 key, 需要新分配一个 id.
        // 到了最大的范围则返回 -1, 强行把分配给越界 assert 杀死.
        fn allocate_new_id(v: &mut MyHashMap) -> i32 {
            if v.pool_id_num == 0 { return -1; }
            let new_id = v.id_pool[v.pool_start];
            // 这个操作等同于取余数.
            v.pool_start = (v.pool_start + 1) & v.pool_mask;
            v.pool_id_num -= 1;
            new_id
        }

        let new_id = allocate_new_id(self);
        self.keys[new_id as usize] = key;
        self.next[key_offset as usize] = new_id;
        new_id
    }


    // 查询流程: 先在 hash_bucket 里面查, key 对不上则去 next 里拿 key chain(因为 hash 冲突) 来查.
    // 查不到 return -1
    #[allow(dead_code)]
    pub(self) fn get(&self, key: i32) -> i32 {
        let term_id = self.get_id(key);
        if term_id >= 0 { self.values[term_id as usize] } else { -1 }
    }

    // 和 get 不同, get_id 给定一个 key, 返回给这个 key 自动分配的 id 值.
    // 如果没有 id, 返回 -1.
    fn get_id(&self, key: i32) -> i32 {
        // term_id 代表被自动分配给 key 的 id 值, -1 代表结束条件.
        let mut term_id = self.hash_bucket[key as usize & self.hash_key_mask];
        while term_id >= 0 {
            if self.keys[term_id as usize] == key {
                return term_id;
            }
            term_id = self.next[term_id as usize];
        }
        return -1;
    }

    // 删除流程, 如果找不到 key, 那么就算了
    // 如果 key 已经存在, 那么就把这个 id 回收到 id_pool 内, 并且把 next chain 删除掉这一节.
    // 这里和 insert 一样, 需要对 hash 和 next 的两种情况做分别处理.
    #[allow(dead_code)]
    pub(self) fn remove(&mut self, key: i32) {
        let mut key_offset = self.hash_bucket[key as usize & self.hash_key_mask];
        // 找不到 key.
        if key_offset < 0 { return; }

        // key 在 hash_bucket 内
        if self.keys[key_offset as usize] == key {
            recycle_id(self, key_offset);
            // 把该 id 从 next chain 中删除.
            // 一定要记得把这里的 next 置为 -1, 这样下次 allocate 出来这个节点, next 自动挂载到尾部.
            self.hash_bucket[key as usize & self.hash_key_mask] = self.next[key_offset as usize];
            self.next[key_offset as usize] = -1;
            return;
        }

        // next 内进行 key 查找.
        while self.next[key_offset as usize] >= 0 {
            if self.keys[self.next[key_offset as usize] as usize] == key {
                let recy_id = self.next[key_offset as usize];
                recycle_id(self, recy_id);
                self.next[key_offset as usize] = self.next[self.next[key_offset as usize] as usize];
                self.next[recy_id as usize] = -1;
                return;
            }
            key_offset = self.next[key_offset as usize];
        }

        // 要把 id 给回收掉.
        fn recycle_id(v: &mut MyHashMap, id: i32) {
            let pool_index = (v.pool_start + v.pool_id_num) & v.pool_mask;
            v.id_pool[pool_index] = id;
            v.pool_id_num += 1;
        }
    }
}


#[cfg(test)]
mod tests {
    use super::MyHashMap;

    #[test]
    pub fn my_hash_map_test() {
        let mut map = MyHashMap::new();
        map.put(3, 100);
        map.put(100, 1);
        for i in 0..30 {
            map.put(1 << i, 2);
        }

        assert_eq!(map.get(0), -1);
        assert_eq!(map.get(-1), -1);
        assert_eq!(map.get(3), 100);
        for i in 0..30 {
            assert_eq!(map.get(1 << i), 2);
        }

        map.remove(100);
        map.remove(1);
        assert_eq!(map.get(100), -1);
        assert_eq!(map.get(1), -1);
    }

    #[test]
    pub fn my_hash_map_test2() {
        let mut map = MyHashMap::new();
        map.put(908, 29);
        map.put(395, 865);
        map.put(250, 847);
        map.remove(836);
        map.put(233, 568);
        map.put(657, 790);
        map.put(595, 271);
        map.put(769, 219);
        map.put(55, 112);
        map.put(157, 493);
        assert_eq!(map.get(920), -1);
        map.put(632, 358);
        assert_eq!(map.get(669), -1);
        map.put(506, 228);
        map.remove(904);
        assert_eq!(map.get(473), -1);
        map.put(461, 40);
        map.put(748, 973);
        map.put(446, 544);
        map.put(766, 461);
        assert_eq!(map.get(395), 865);
        map.remove(211);
        assert_eq!(map.get(415), -1);
        map.remove(157);
        assert_eq!(map.get(252), -1);
        map.put(252, 22);
        map.put(942, 681);
        map.put(600, 988);
        map.put(424, 39);
        map.put(685, 482);
        map.put(561, 605);
        map.remove(632);
        map.remove(461);
        map.put(916, 329);
        map.put(739, 735);
        map.remove(769);
        assert_eq!(map.get(942), 681);
        map.put(460, 226);
        map.put(183, 411);
        map.put(224, 524);
        map.remove(769);
        map.put(508, 614);
        assert_eq!(map.get(632), -1);
        map.put(254, 825);
        assert_eq!(map.get(603), -1);
        map.put(115, 667);
        map.remove(460);
        map.put(940, 813);
        map.put(50, 629);
        assert_eq!(map.get(519), -1);
        map.remove(595);
        map.put(39, 913);
        map.put(742, 13);
        map.remove(765);
        map.put(163, 627);
        map.put(554, 130);
        map.put(255, 945);
        map.put(22, 78);
        map.put(912, 390);
        map.remove(632);
        map.put(609, 410);
        map.remove(956);
        map.put(515, 243);
        map.put(975, 871);
        map.put(520, 313);
        map.put(682, 538);
        map.remove(563);
        map.put(119, 902);
        assert_eq!(map.get(916), 329);
        map.remove(766);
        map.put(293, 885);
        map.put(657, 665);
        map.put(518, 832);
        map.put(897, 489);
        map.put(340, 972);
        map.put(790, 24);
        map.put(637, 445);
        map.put(544, 498);
        map.remove(115);
        map.remove(600);
        map.put(269, 209);
        map.put(734, 3);
        map.put(243, 108);
        assert_eq!(map.get(233), 568);
        map.put(679, 632);
        map.put(640, 55);
        map.put(288, 301);
        map.remove(682);
        map.remove(871);
        map.put(922, 755);
        map.put(624, 787);
        map.remove(776);
        map.remove(293);
        map.put(564, 902);
        map.put(32, 743);
        map.put(934, 278);
        map.remove(250);
        map.remove(389);
        map.put(620, 711);
        map.put(420, 623);
        map.put(346, 959);
        map.put(829, 832);
        map.put(776, 894);
        map.put(465, 769);
        map.remove(508);
        assert_eq!(map.get(358), -1);
        map.put(126, 481);
        assert_eq!(map.get(255), 945);
        assert_eq!(map.get(50), 629);
        map.put(477, 991);
        map.put(973, 337);
        map.remove(32);
        map.put(823, 283);
    }


}


