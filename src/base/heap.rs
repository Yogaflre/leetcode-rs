// <大顶堆>
#[derive(PartialEq, Debug)]
struct Heap {
    nodes: Vec<i32>, //堆是完全二叉树，所以用数组存储
    count: usize,    //已经存在的元素个数
}

impl Heap {
    // 对指定节点进行堆化
    fn heapify(nums: &mut Vec<i32>, mut index: usize) {
        while index < nums.len() {
            let mut next = index;
            // 如果左孩子比根节点大，则将next设置为左孩子
            if index * 2 + 1 <= nums.len() - 1 && nums[index] <= nums[index * 2 + 1] {
                next = index * 2 + 1;
            }
            // 如果右孩子比根节点和左孩子都大，则将next设置为右孩子
            if index * 2 + 2 <= nums.len() - 1
                && nums[next] < nums[index * 2 + 2]
                && nums[index] < nums[index * 2 + 2]
            {
                next = index * 2 + 2;
            }
            if next == index {
                break;
            }

            let tmp = nums[next];
            nums[next] = nums[index];
            nums[index] = tmp;
            index = next;
        }
    }

    // 建堆，从下往上建堆，寻找第一个非叶子节点的位置(n/2)
    fn create(mut nums: Vec<i32>) -> Heap {
        if nums.len() > 1 {
            // NOTE 这里选取起始点(n/2)很重要！根据完全二叉树特性选取第一个非叶子节点
            for i in (0..=((nums.len() / 2) - 1)).rev() {
                Heap::heapify(&mut nums, i);
            }
        }
        return Heap {
            count: nums.len(),
            nodes: nums,
        };
    }

    // 先插入到数组末尾，然后堆化(比较末尾元素与父节点大小)
    fn insert(&mut self, num: i32) {
        let mut index = self.nodes.len();
        self.nodes.push(num);
        self.count += 1;
        while index != 0 {
            if self.nodes[index] > self.nodes[index / 2] {
                let tmp = self.nodes[index];
                self.nodes[index] = self.nodes[index / 2];
                self.nodes[index / 2] = tmp;
            }
            index = index / 2;
        }
    }

    // 删除最大值（堆顶节点）。将最后一个元素放置在堆顶，然后再堆化
    fn remove_max(&mut self) {
        if self.count >= 1 {
            self.count -= 1;
            if self.count == 1 {
                self.nodes.remove(0);
            } else {
                self.nodes[0] = self.nodes.pop().unwrap();
                Heap::heapify(&mut self.nodes, 0);
            }
        }
    }

    // 建好堆后排序
    fn sort(&mut self) {
        let mut len = self.count;
        while len > 1 {
            len -= 1;

            let tmp = self.nodes[0];
            self.nodes[0] = self.nodes[len];
            self.nodes[len] = tmp;

            // 堆化
            let mut index = 0;
            let mut next = index;
            while index < len {
                if index * 2 + 1 <= len - 1 && self.nodes[index] < self.nodes[index * 2 + 1] {
                    next = next * 2 + 1;
                }
                if index * 2 + 2 <= len - 1
                    && self.nodes[next] < self.nodes[index * 2 + 2]
                    && self.nodes[index] < self.nodes[index * 2 + 2]
                {
                    next = index * 2 + 2;
                }
                if index == next {
                    break;
                }
                let tmp = self.nodes[next];
                self.nodes[next] = self.nodes[index];
                self.nodes[index] = tmp;
                index = next;
            }
        }
    }
}

#[test]
fn run() {
    let mut heap: Heap = Heap::create(vec![1, 8, 5, 4, 7]);
    assert_eq!(
        heap,
        Heap {
            nodes: vec![8, 7, 5, 4, 1],
            count: 5,
        }
    );

    heap.insert(2);
    assert_eq!(
        heap,
        Heap {
            nodes: vec![8, 7, 5, 4, 1, 2],
            count: 6,
        }
    );

    heap.remove_max();
    assert_eq!(
        heap,
        Heap {
            nodes: vec![7, 4, 5, 2, 1],
            count: 5,
        }
    );

    heap.sort();
    assert_eq!(heap.nodes, vec![1, 2, 4, 5, 7]);
}
