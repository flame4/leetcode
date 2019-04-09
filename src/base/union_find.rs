#[derive(Debug)]
#[allow(dead_code)]
struct UnionFind {
    // TODO 可以用HashMap扩展为任意元素的集合.
    // 初始化为 -1. 正数表示父亲是其他节点, 负数表示为秩的**上界**的负数.
    content: Vec<i32>
}


impl UnionFind {
    /// union, 在查询过程中路径已经不断被压缩.
    /// 新的合并分支需要把**上界**矮的合并到高的.
    #[allow(dead_code)]
    pub fn union(&mut self, x: usize, y: usize) {
        if x >= self.content.len() || y >= self.content.len() { return; }
        let x_parent = self.find(x);
        let y_parent = self.find(y);
        if x_parent == y_parent { return; }
        let xp_height = self.content[x_parent];
        let yp_height = self.content[y_parent];
        if xp_height < yp_height {
            // x is higher.
            self.content[y_parent] = x_parent as i32;
        } else {
            self.content[x_parent] = y_parent as i32;
            if yp_height == xp_height {
                self.content[y_parent] -= 1;
            }
        }
    }

    /// 2. 路径压缩.
    #[allow(dead_code)]
    pub fn find(&mut self, x: usize) -> usize {
        if self.content[x] < 0 {
            return x;
        }
        self.content[x] = self.find(self.content[x] as usize) as i32;
        self.content[x] as usize
    }

    /// 初始化一个长度为num的并查集合.
    #[allow(dead_code)]
    pub fn new(num: usize) -> Self {
        let mut content = Vec::new();
        content.resize(num + 1, -1);
        UnionFind { content }
    }
}

#[cfg(test)]
mod tests {
    use super::UnionFind;

    #[test]
    pub fn union_find_test() {
        let mut uf = UnionFind::new(10);
        assert_eq!(uf.find(8), 8);
        uf.union(3, 4);
        assert_eq!(uf.find(3), 4);
        uf.union(5, 6);
        uf.union(7, 8);
        uf.union(8, 9);
        assert_eq!(uf.find(7), uf.find(9));
        assert_ne!(uf.find(7), uf.find(4));
        uf.union(6, 7);
        assert_ne!(uf.find(7), uf.find(4));
        assert_eq!(uf.find(5), uf.find(9));
        uf.union(4, 5);
        assert_eq!(uf.find(7), uf.find(4));
        assert_ne!(uf.find(7), uf.find(1));
    }
}



