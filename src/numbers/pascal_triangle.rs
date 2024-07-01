
pub struct PascalsTriangle {
    row_count: u32
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        Self {
            row_count
        }
    }
// essayer de revoir l' insertion avec map et fold
    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows = Vec::<Vec<u32>>::new();
        match self.row_count {
            0 => rows,
            1 => {
                rows.push(vec![1]);
                rows
            },
            _ => {
                rows.push(vec![1]);
                let mut prec_row = &vec![1, 1];
                for i in 2..=self.row_count as usize {
                    let mut row = vec![1, 1];
                    row.reserve(i);
                    let mid = i.div_euclid(2);
                    for j in 1..mid as usize {
                        row.insert(j, prec_row[j - 1] + prec_row[j]);
                        row.insert(row.len() - j, row[j]);
                    }
                    if i % 2 != 0 {
                        row.insert(mid, prec_row[mid - 1] + prec_row[mid]);
                    }
                    rows.push(row);
                    prec_row = &rows[rows.len() - 1];
                }

                rows
            }
        }
    }
}

#[test]
fn zero_rows() {
    let pt = PascalsTriangle::new(0);
    let expected: Vec<Vec<u32>> = vec![];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn single_row() {
    let pt = PascalsTriangle::new(1);
    let expected: Vec<Vec<u32>> = vec![vec![1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn two_rows() {
    let pt = PascalsTriangle::new(2);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn three_rows() {
    let pt = PascalsTriangle::new(3);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn four_rows() {
    let pt = PascalsTriangle::new(4);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1]];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn five_rows() {
    let pt = PascalsTriangle::new(5);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
    ];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn six_rows() {
    let pt = PascalsTriangle::new(6);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
    ];
    assert_eq!(pt.rows(), expected);
}

#[test]
fn ten_rows() {
    let pt = PascalsTriangle::new(10);
    let expected: Vec<Vec<u32>> = vec![
        vec![1],
        vec![1, 1],
        vec![1, 2, 1],
        vec![1, 3, 3, 1],
        vec![1, 4, 6, 4, 1],
        vec![1, 5, 10, 10, 5, 1],
        vec![1, 6, 15, 20, 15, 6, 1],
        vec![1, 7, 21, 35, 35, 21, 7, 1],
        vec![1, 8, 28, 56, 70, 56, 28, 8, 1],
        vec![1, 9, 36, 84, 126, 126, 84, 36, 9, 1],
    ];
    assert_eq!(pt.rows(), expected);
}