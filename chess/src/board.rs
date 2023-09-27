const COLUMNS: Vec<char> = vec!['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
const ROWS: Vec<i32> = vec![1,2,3,4,5,6,7,8];

pub struct Board {
    rows: [mut [mut i32, ..8], ..8]
}