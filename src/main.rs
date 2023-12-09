use std::fs::read_to_string;

fn get_contents(filename: &str) -> Vec<Vec<i64>> {
    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()).collect()
}

fn not_terminal(last_sequence: &Vec<i64>) -> bool{
    last_sequence.len() > 1 && !last_sequence.iter().all(|n| *n == 0)
}

fn solution_1(filename: &str) -> i64 { 
    let histories = get_contents(filename);
    let mut sum = 0;

    for history in histories {
        let mut sequences: Vec<Vec<i64>> = Vec::new();
        sequences.push(history.clone());
        while not_terminal(sequences.last().unwrap()) {
            // get diffs between each two adjacent members of last sequence
            let mut diffs: Vec<i64> = Vec::new();
            let last = sequences.last().unwrap();
            for i in 0..last.len() - 1 {
                diffs.push(last[i+1] - last[i]);
            }
            sequences.push(diffs);
        }
        let mut k = 0;
        for seq in sequences.iter().rev() {
            k += seq.last().unwrap();
        }
        sum += k;
    }

    sum
}

fn main() {
    assert_eq!(solution_1("example.txt"), 114);
    assert_eq!(solution_1("input.txt"), 1938800261);
    // assert_eq!(solution_2("example.txt"), 0);
    // assert_eq!(solution_2("input.txt"), 0);
}
