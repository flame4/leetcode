use super::Solution;
use std::cmp::Ordering;
use std::collections::binary_heap::BinaryHeap;

#[derive(Ord, Eq, PartialOrd, Debug)]
struct Point {
    distance: i32,  // 平方, 没开方.
    position: Vec<i32>,
}

//impl Ord for Point {
//    // 距离大的排前面(Greater)
//    fn cmp(&self, other: &Point) -> Ordering {
//        other.partial_cmp(self).unwrap()
//    }
//}
//
//impl PartialOrd for Point {
//    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
//        self.distance.partial_cmp(&other.distance)
//    }
//}
//
impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.distance == other.distance
    }
}

impl Point {
    pub fn new(position: Vec<i32>) -> Point {
        assert_eq!(position.len(), 2);
        Point {
            distance: position[0] * position[0] + position[1] * position[1],
            position
        }
    }
}

impl Solution {
    // 练习使用rust的优先级队列 BinaryHeap.
    // 暂时不包括强制控制capacity的功能, 只能自己手动控制保证复杂度为 O(Nlog(k)), 40ms.
    pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
        // TODO 优化写法.
        if k == 0 { return Vec::new() }
        let mut heap = BinaryHeap::new();
        for point in points {
            let p = Point::new(point);
            if (heap.len() as i32) < k {
                heap.push(p);
                continue
            }
            if p.distance.cmp(&heap.peek().unwrap().distance) == Ordering::Less {
                heap.pop();
                heap.push(p);
            }
        }

        let tmp = heap.into_sorted_vec();
        let mut ret = vec![];
        for i in tmp {
            ret.push(i.position);
        }
        ret
    }

    // TODO nth_element 算法, 平均复杂度O(N).
    // pub fn k_closest_1(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {}
}



#[cfg(test)]
mod tests {
    use super::Solution;

    #[test]
    pub fn k_closest_1() {
        let input = vec![
            // 18, 26, 20
            vec![3,3], vec![5,-1], vec![-2,4]
        ];
        assert_eq!(Solution::k_closest(input, 2), vec![vec![3, 3], vec![-2, 4]]);


        let input = vec![
            // 18, 26, 20, 45, 45, 26, 20
            vec![3,3], vec![5,-1], vec![-2,4], vec![3, 6], vec![6,3], vec![1,5], vec![2,4]
        ];
        assert_eq!(Solution::k_closest(input, 3), vec![vec![3, 3], vec![-2, 4], vec![2,4]]);
    }


    #[test]
    pub fn k_closest_2() {
        let input = vec![
            // 18, 26, 20
            vec![3,3], vec![5,-1], vec![-2,4]
        ];
        assert_eq!(Solution::k_closest(input, 1), vec![vec![3, 3]]);
    }
}
