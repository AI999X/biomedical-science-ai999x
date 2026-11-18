//! Biomedical Science: 15 calculations through the Rust library (the same engine as the desktop app, the
//! command line and the browser). Run from the rust/ folder:
//!     cargo run --release --example readme -p biomedical-science-ai999x
//! Inputs are text, read like a form field ("2 m", "0.5", "x^3 - 3x"); a JSON list stays a list.

use serde_json::{json, Value};

/// One calculation: the request in, its outcome and every value with its unit out.
fn show(request: Value) {
    let answer: Value = serde_json::from_str(&biomedical_science_ai999x::solve_json(&request.to_string())).unwrap();
    println!("{}", answer["outcome"].as_str().unwrap_or(""));
    for v in answer["values"].as_array().into_iter().flatten() {
        let unit = v[2].as_str().unwrap_or("");
        let unit = if unit.is_empty() || unit == "1" { String::new() } else { format!(" {unit}") };
        println!("{} = {}{}", v[0].as_str().unwrap_or(""), v[1], unit);
    }
}

fn main() {
    show(json!({"key": "bmed_bmr", "inputs": {"m": "70 kg", "h": "175 cm", "age": "30", "sex": "male", "equation": "mifflin_st_jeor"}}));
    println!();
    show(json!({"key": "bmed_cohens_kappa", "inputs": {"counts": [[40, 10], [5, 45]]}}));
    println!();
    show(json!({"key": "bmed_iv_rate_from_dose_rate", "inputs": {"drug": "400 mg", "bag_volume": "250 mL", "dose_ug_per_kg_min": "5", "W": "80 kg"}}));
    println!();
    show(json!({"key": "bmed_epi_herd_immunity", "inputs": {"R0": "4", "efficacy": "0.9"}}));
    println!();
    show(json!({"key": "bmed_nnt_ci", "inputs": {"events_t": "15", "n_t": "100", "events_c": "30", "n_c": "100", "level": "0.95"}}));
    println!();
    show(json!({"key": "bmed_hounsfield_units", "inputs": {"mu": "0.38 1/cm", "mu_water": "0.19 1/cm"}}));
    println!();
    show(json!({"key": "bmed_fitzhugh_nagumo_fixed_point", "inputs": {"a": "0.7", "b": "0.8", "eps": "0.08", "I": "0"}}));
    println!();
    show(json!({"key": "bmed_pk_bioavailability_from_profiles", "inputs": {"D_iv": "100 mg", "t_iv": [0, 1, 2, 4, 8, 12], "C_iv": [10, 8.2, 6.7, 4.5, 2, 0.9], "D_oral": "200 mg", "t_oral": [0, 1, 2, 4, 8, 12], "C_oral": [0, 6.1, 8, 7.2, 3.9, 1.8]}}));
    println!();
    show(json!({"key": "bmed_pk_auc_iv_bolus", "inputs": {"D": "500 mg", "CL": "5 L/h", "V": "40 L", "t": "8 h"}}));
    println!();
    show(json!({"key": "bmed_cox_hazard_ratio", "inputs": {"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "x": [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], "level": "0.95"}}));
    println!();
    show(json!({"key": "bmed_bmi", "inputs": {"m": "70 kg", "h": "1.75 m"}}));
    println!();
    show(json!({"key": "bmed_dx_youden_cutoff", "inputs": {"scores_pos": [0.9, 0.8, 0.7, 0.55, 0.4], "scores_neg": [0.6, 0.45, 0.4, 0.3, 0.2, 0.1]}}));
    println!();
    show(json!({"key": "bmed_iv_flow_rate", "inputs": {"V": "1000 mL", "T": "8 h", "drop_factor": "15"}}));
    println!();
    show(json!({"key": "bmed_epi_reed_frost_final_size", "inputs": {"n": "4", "m": "1", "p": "0.2"}}));
    println!();
    show(json!({"key": "bmed_epi_basic_rates", "inputs": {"cases": "150", "population": "10000", "prevalent": "400", "deaths": "6", "period": "1 yr"}}));
}
