use std::fs::read_to_string;

fn get_contents(filename: &str) -> Vec<Vec<i64>> {
    let contents = read_to_string(filename).expect("Something went wrong reading the file");
    contents.lines().map(|s| s.to_string().split_whitespace().map(|n| n.parse::<i64>().unwrap()).collect()).collect()
}

fn not_terminal(last_sequence: &Vec<i64>) -> bool{
    last_sequence.len() > 1 && !last_sequence.iter().all(|n| *n == 0)
}

fn build_sequences(history: Vec<i64>) -> Vec<Vec<i64>> {
    let mut sequences = Vec::new();
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
    sequences
}

fn solution(filename: &str, backwards: bool) -> i64 { 
    let histories = get_contents(filename);
    let mut sum = 0;
    for history in histories {
        let sequences: Vec<Vec<i64>> = build_sequences(history);
        let mut k = 0;
        for seq in sequences.iter().rev() {
            if backwards {
                k = seq.first().unwrap() - k;
            } else {
                k += seq.last().unwrap();
            }
        }
        sum += k;
    }
    sum
}

fn main() {
    println!("{}", solution("example.txt", false));
    println!("{}", solution("input.txt", false));
    println!("{}", solution("example.txt", true));
    println!("{}", solution("input.txt", true));
}
