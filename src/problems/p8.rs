use super::Answer;

const GRID_SIZE: usize = 99;

pub fn solve(input: &str) -> Answer<usize, u32> {
    let mut vec = Vec::with_capacity(GRID_SIZE * GRID_SIZE);
    for line in input.lines() {
        for c in line.chars() {
            vec.push(c.to_digit(10).unwrap() as u8);
        }
    }
    assert_eq!(vec.len(), GRID_SIZE * GRID_SIZE);
    let grid = Grid::new(vec);
    let mut count_grid = Grid::new(vec![0; grid.data.len()]);
    // Was it worth it to make this generic? No, but I did it anyway
    line_fold(
        &grid,
        IndexIterator::<Left>::new(Index::new(0, 0)),
        &mut count_grid,
    );
    line_fold(
        &grid,
        IndexIterator::<Right>::new(Index::new(0, GRID_SIZE - 1)),
        &mut count_grid,
    );
    line_fold(
        &grid,
        IndexIterator::<Up>::new(Index::new(GRID_SIZE - 1, 0)),
        &mut count_grid,
    );
    line_fold(
        &grid,
        IndexIterator::<Down>::new(Index::new(0, 0)),
        &mut count_grid,
    );
    Answer::new(
        count_grid.data.iter().filter(|&&x| x != 0).count(),
        part2(&grid),
    )
}

#[derive(Debug)]
struct Grid {
    data: Vec<u8>,
}

impl Grid {
    fn new(data: Vec<u8>) -> Self {
        Self { data }
    }
}

impl std::ops::Index<Index> for Grid {
    type Output = u8;

    fn index(&self, index: Index) -> &Self::Output {
        &self.data[index.row * GRID_SIZE + index.col]
    }
}

impl std::ops::IndexMut<Index> for Grid {
    fn index_mut(&mut self, index: Index) -> &mut Self::Output {
        &mut self.data[index.row * GRID_SIZE + index.col]
    }
}

#[derive(Copy, Clone)]
struct Index {
    row: usize,
    col: usize,
}

impl Index {
    fn new(row: usize, col: usize) -> Self {
        Self { row, col }
    }
    fn is_edge(&self) -> bool {
        self.row == 0 || self.row == GRID_SIZE - 1 || self.col == 0 || self.col == GRID_SIZE - 1
    }
}

// Traits to advance indices

trait Walker {
    fn advance_minor(index: Index) -> Index;
    fn advance_major(index: Index) -> Index;
}

#[derive(Clone)]
struct Left {}
#[derive(Clone)]
struct Right {}
#[derive(Clone)]
struct Up {}
#[derive(Clone)]
struct Down {}
impl Walker for Left {
    fn advance_minor(index: Index) -> Index {
        Index::new(index.row, index.col + 1)
    }

    fn advance_major(index: Index) -> Index {
        Index::new(index.row + 1, 0)
    }
}
impl Walker for Right {
    fn advance_minor(index: Index) -> Index {
        Index::new(index.row, index.col - 1)
    }

    fn advance_major(index: Index) -> Index {
        Index::new(index.row + 1, GRID_SIZE - 1)
    }
}
impl Walker for Up {
    fn advance_minor(index: Index) -> Index {
        Index::new(index.row - 1, index.col)
    }

    fn advance_major(index: Index) -> Index {
        Index::new(GRID_SIZE - 1, index.col + 1)
    }
}
impl Walker for Down {
    fn advance_minor(index: Index) -> Index {
        Index::new(index.row + 1, index.col)
    }

    fn advance_major(index: Index) -> Index {
        Index::new(0, index.col + 1)
    }
}

#[derive(Clone)]
struct IndexIterator<F> {
    count: usize,
    index: Index,
    f: std::marker::PhantomData<F>,
}

impl<F: Walker> IndexIterator<F> {
    fn new(index: Index) -> Self {
        Self {
            count: GRID_SIZE,
            index,
            f: std::marker::PhantomData,
        }
    }
    fn next_iter(mut self) -> Self {
        self.index = F::advance_major(self.index);
        self
    }
}

impl<F> Iterator for IndexIterator<F>
where
    F: Walker,
{
    type Item = Index;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count == 1 {
            None
        } else {
            let index = self.index;
            self.count -= 1;
            self.index = (F::advance_minor)(index);
            Some(index)
        }
    }
}

fn line_fold<F: Walker + Clone>(grid: &Grid, begin: IndexIterator<F>, out: &mut Grid) {
    let mut iter = begin.clone();
    for _ in 0..GRID_SIZE {
        out[iter.index] += 1; // Edge
        iter.clone().fold(0, |acc, idx| {
            let x = grid[idx];
            if x > acc {
                out[idx] += 1;
                x
            } else {
                acc
            }
        });
        iter = iter.next_iter();
    }
}

fn part2(grid: &Grid) -> u32 {
    let mut best_score = 0;
    // Edge values will always be 0 so skip them
    for row in 1..GRID_SIZE - 1 {
        for col in 1..GRID_SIZE - 1 {
            let init_index = Index::new(row, col);
            let mut score = 1;
            let height = grid[init_index];
            for update in [
                Left::advance_minor,
                Right::advance_minor,
                Up::advance_minor,
                Down::advance_minor,
            ] {
                let mut dir_count = 1; // Must see at least one neighbor if not on edge
                let mut index = update(init_index);
                while !index.is_edge() {
                    if grid[index] >= height {
                        break;
                    }
                    index = update(index);
                    dir_count += 1;
                }
                score *= dir_count;
            }
            if score > best_score {
                best_score = score;
            }
        }
    }
    best_score
}
