// Author: hankun1991@outlook.com

/**
* 20230315
* No.1615
*/
fn maximal_network_rank(n: i32, roads: Vec<Vec<i32>>) -> i32 {
    let mut ans = 0;

    let mut degree = vec![0; n as usize];
    let mut graph = vec![vec![false; n as usize]; n as usize];

    for i in roads {
        degree[i[0] as usize] += 1;
        degree[i[1] as usize] += 1;
        graph[i[0] as usize][i[1] as usize] = true;
        graph[i[1] as usize][i[0] as usize] = true;
    }

    for i in 0..n {
        for j in i+1..n {
            ans = std::cmp::max(ans, degree[i as usize] + degree[j as usize] - graph[i as usize][j as usize] as i32);
        }
    }

    return ans;
}

#[test]
fn test_maximal_network_rank() {
    let eg = vec![vec![0,1], vec![0,3], vec![1,2], vec![1,3]];
    let ret = maximal_network_rank(4, eg);
    assert_eq!(ret, 4);
}

/**
* 20230317
* No.2389
* 给你一个长度为 n的整数数组 nums ，和一个长度为 m 的整数数组 queries 。

* 返回一个长度为 m 的数组 answer ，其中 answer[i] 是 nums 中 元素之和小于等于 queries[i] 的 子序列 的 最大 长度 。
*
* 子序列 是由一个数组删除某些元素（也可以不删除）但不改变剩余元素顺序得到的一个数组。
*
*
* 示例 1：
*
* 输入：nums = [4,5,2,1], queries = [3,10,21]
* 输出：[2,3,4]
* 解释：queries 对应的 answer 如下：
* - 子序列 [2,1] 的和小于或等于 3 。可以证明满足题目要求的子序列的最大长度是 2 ，所以 answer[0] = 2 。
* - 子序列 [4,5,1] 的和小于或等于 10 。可以证明满足题目要求的子序列的最大长度是 3 ，所以 answer[1] = 3 。
* - 子序列 [4,5,2,1] 的和小于或等于 21 。可以证明满足题目要求的子序列的最大长度是 4 ，所以 answer[2] = 4 。
* 示例 2：
*
* 输入：nums = [2,3,4,5], queries = [1]
* 输出：[0]
* 解释：空子序列是唯一一个满足元素和小于或等于 1 的子序列，所以 answer[0] = 0 。
*
*/

fn answer_queries(nums: Vec<i32>, queries: Vec<i32>) -> Vec<i32> {
    let mut nums = nums;
    nums.sort();

    let mut container = vec![];
    container.push(nums[0]);
    for i in 1..nums.len() {
       container.push(nums[i] + container[i-1]);
    }

    let mut queries = queries;
    queries.iter_mut().for_each(|x|*x=container.partition_point(|y|y<=x) as i32);
    return queries;
}

#[test]
fn test_answer_quries() {
    let nums = vec![4,5,2,1];
    let queries = vec![3,10,21];
    let ret = answer_queries(nums, queries);
    for i in ret {
        print!("{}\t", i);
    }
    // assert_eq!(ret.len(), 3);
}