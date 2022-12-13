use super::*;
use std::collections::VecDeque;

struct Grid {
    arr: Vec<Vec<u8>>,
}

impl Grid {
    fn bfs(&self, start: (usize, usize), end: (usize, usize)) -> u32 {
        let mut steps = 0;
        let mut visited = vec![false; self.arr.len() * self.arr[0].len()];
        visited[start.0 * self.arr[0].len() + start.1] = true;
        let mut queue = Vec::new();
        queue.push(start);
        loop {
            let mut depth_queue = Vec::new();
            while !queue.is_empty() {
                let item = queue.pop().unwrap();
                if item == end {
                    return steps;
                }
                let current_val = self.arr[item.0][item.1];
                let check = [
                    Some((item.0 + 1, item.1)),
                    Some((item.0, item.1 + 1)),
                    item.0.checked_sub(1).map(|r| (r, item.1)),
                    item.1.checked_sub(1).map(|c| (item.0, c)),
                ];
                for (r, c) in check.into_iter().filter_map(|x| x) {
                    if let Some(val) = self.arr.get(r).and_then(|vec| vec.get(c)) {
                        let visited_idx = r * self.arr[0].len() + c;
                        if *val <= current_val + 1 && !visited[visited_idx] {
                            visited[visited_idx] = true;
                            if (r, c) == end {
                                return steps + 1;
                            }
                            depth_queue.push((r, c))
                        }
                    }
                }
            }
            queue.extend(depth_queue.iter());
            steps += 1;
        }
    }
    fn bfs2(&self, start: (usize, usize)) -> u32 {
        let mut steps = 0;
        let mut visited = vec![false; self.arr.len() * self.arr[0].len()];
        visited[start.0 * self.arr[0].len() + start.1] = true;
        let mut queue = Vec::new();
        queue.push(start);
        loop {
            let mut depth_queue = Vec::new();
            while !queue.is_empty() {
                let item = queue.pop().unwrap();

                let current_val = self.arr[item.0][item.1];
                if current_val == 0 {
                    return steps;
                }
                let check = [
                    Some((item.0 + 1, item.1)),
                    Some((item.0, item.1 + 1)),
                    item.0.checked_sub(1).map(|r| (r, item.1)),
                    item.1.checked_sub(1).map(|c| (item.0, c)),
                ];
                for (r, c) in check.into_iter().filter_map(|x| x) {
                    if let Some(val) = self.arr.get(r).and_then(|vec| vec.get(c)) {
                        let visited_idx = r * self.arr[0].len() + c;
                        // Walk backwards
                        if (*val >= current_val - 1) && !visited[visited_idx] {
                            visited[visited_idx] = true;
                            depth_queue.push((r, c))
                        }
                    }
                }
            }
            queue.extend(depth_queue.iter());
            steps += 1;
        }
    }
}

pub fn solve(input: &str) -> Answer<u32, u32> {
    let mut vec = Vec::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (row, line) in input.lines().enumerate() {
        let mut arr = Vec::new();
        let line = line.as_bytes();
        for (col, &b) in line.into_iter().enumerate() {
            if b == b'S' {
                start = (row, col);
                arr.push(0);
            } else if b == b'E' {
                end = (row, col);
                arr.push(b'z' - b'a')
            } else {
                arr.push(b - b'a');
            }
        }
        vec.push(arr);
    }
    let grid = Grid { arr: vec };
    Answer::new(grid.bfs(start, end), grid.bfs2(end))
}
