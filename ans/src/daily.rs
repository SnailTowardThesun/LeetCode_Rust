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
