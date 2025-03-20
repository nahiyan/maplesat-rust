use crate::models::lit::Lit;
use crate::models::solver::Solver;
use std::io::BufRead;

impl<'a> Solver<'a> {
    pub fn load_dimacs<T: BufRead>(&mut self, reader: T) {
        let mut num_vars: usize = 0;
        let mut num_clauses: usize = 0;
        let mut num_found_clauses: usize = 0;
        for line in reader.lines() {
            let line = line.expect("Failed to read line from file.");
            let line = line.trim();
            if line.starts_with("c") {
                // Skip the ecomments
                continue;
            } else if line.starts_with("p cnf") {
                // Parse the header
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() != 4 {
                    panic!("Parse error! Unexpected header: {}", line)
                }
                num_vars = parts[2]
                    .parse()
                    .expect("Failed to parse number of variables.");
                num_clauses = parts[3]
                    .parse()
                    .expect("Failed to parse number of clauses.");
            } else {
                // Parse a clause
                let mut clause: Vec<Lit> = vec![];
                for lit_str in line.split_whitespace() {
                    let word: i32 = lit_str.parse().expect("Failed to parse literal");
                    if word == 0 {
                        break;
                    }
                    clause.push(Lit::from(word));
                }
                self.add_clause(clause);
                num_found_clauses += 1;
            }
        }

        if self.num_vars() != num_vars {
            panic!(
                "Error! DIMACS header mismatch: wrong number of variables. {} {}",
                self.num_vars(),
                num_vars
            );
        }
        if num_found_clauses != num_clauses {
            panic!("Error! DIMACS header mismatch: wrong number of clauses.");
        }
    }
}

#[cfg(test)]
mod tests {
    use std::io::{BufReader, Cursor};

    use super::*;

    #[test]
    fn test_parse_valid_dimacs() {
        let dimacs_data = "\
        c This is a comment
        p cnf 3 2
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);

        assert_eq!(solver.num_vars(), 3);
    }

    #[test]
    #[should_panic(expected = "Parse error! Unexpected header: p cnf 3")]
    fn test_parse_invalid_header() {
        let dimacs_data = "\
        c This is a comment
        p cnf 3
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);
    }

    #[test]
    #[should_panic(expected = "Failed to parse number of variables.")]
    fn test_parse_invalid_num_vars() {
        let dimacs_data = "\
        c This is a comment
        p cnf x 2
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);
    }

    #[test]
    #[should_panic(expected = "Failed to parse number of clauses.")]
    fn test_parse_invalid_num_clauses() {
        let dimacs_data = "\
        c This is a comment
        p cnf 3 x
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);
    }

    #[test]
    #[should_panic(expected = "Error! DIMACS header mismatch: wrong number of variables.")]
    fn test_parse_mismatched_num_vars() {
        let dimacs_data = "\
        c This is a comment
        p cnf 2 2
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);
    }

    #[test]
    #[should_panic(expected = "Error! DIMACS header mismatch: wrong number of clauses.")]
    fn test_parse_mismatched_num_clauses() {
        let dimacs_data = "\
        c This is a comment
        p cnf 3 1
        1 -3 0
        2 3 -1 0
        ";
        let reader = BufReader::new(Cursor::new(dimacs_data));
        let mut solver = Solver::new();
        solver.load_dimacs(reader);
    }
}
