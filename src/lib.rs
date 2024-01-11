use std::convert::TryFrom;

use splr::solver::Solver;
use splr::{Certificate, Config, SolveIF};

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(js_name = "solveSat")]
pub fn solve_sat(s: &[i32]) -> String {
    let mut v: Vec<Vec<i32>> = Vec::new();
    let mut index = 0;

    while index < s.len() {
        let l = s[index];
        let vector = &s[index + 1..index + 1 + l as usize];
        v.push(vector.to_vec());
        index += l as usize + 1;
    }

    match Solver::try_from((Config::default(), v.as_ref())) {
        Ok(mut solver) => match solver.solve() {
            Ok(Certificate::SAT(v)) => format!("{{\"success\": true, \"result\": {:?}}}", v),
            Ok(Certificate::UNSAT) => "{\"success\": false, \"details\": \"unsat\"}".to_string(),
            Err(e) => format!("{{\"success\": false, \"details\": \"{:?}\"}}", e),
        },
        Err(e) => {
            format!("{{\"success\": false, \"details\": \"{:?}\"}}", e)
        }
    }
}
