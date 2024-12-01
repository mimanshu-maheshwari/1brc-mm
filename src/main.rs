use obrc_mm::hash_naive;

fn main() {
    const FILE_NAME: &str = "res/measurements.txt";
    hash_naive::solve(FILE_NAME);
}
