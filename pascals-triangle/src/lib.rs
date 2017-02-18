pub struct PascalsTriangle {
    number_of_rows: u32    
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle {number_of_rows: row_count}
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.number_of_rows).map(|row| PascalsTriangle::row(row)).collect()
    }

    pub fn row(number: u32) -> Vec<u32> {
        let mut row = vec![1];

        for column in 1..(number+1) {
            if let Some(&last) = row.last() {
                row.push((last * (number + 1 - column)) / column)
            }
        }
        row
    }
}
