//! The question reader on this model's own capabilities (release 1.6.7, 2026-11-21): a rate written "0.1 1/h",
//! "0.1 h^-1" or "0.1 per hour" and a time "6 hours" are read as units; a question that is not read offers the
//! capability's documented template.

use ax_engine::ask::ask_value;
use biomedical_science_ai999x::pack;

fn concentration(q: &str) -> Option<f64> {
    let r = ask_value(pack(), q);
    assert_eq!(r["outcome"], "SOLVED", "{q}: {r}");
    assert_eq!(r["capability"], "bmed_pk_iv_bolus_concentration", "{q}");
    r["answer"]["value"].as_f64()
}

#[test]
fn rates_and_times_written_every_usual_way() {
    let want = 0.5 / 40.0 * (-0.6f64).exp(); // kg/m^3: 500 mg in 40 L, k t = 0.6
    for q in [
        "IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h",
        "IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 h^-1, t = 6 h",
        "IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 per hour, t = 6 h",
        "What is the concentration after 6 h of an IV bolus with D = 500 mg, V = 40 L, k = 0.1 1/h?",
        "What is the concentration of an IV bolus with D = 500 mg, V = 40 L, k = 0.1 per hour, at t = 6 hours?",
    ] {
        let got = concentration(q).unwrap();
        assert!((got / want - 1.0).abs() < 1e-9, "{q}: {got}");
    }
}

#[test]
fn a_question_not_read_offers_the_documented_template() {
    let r = ask_value(pack(), "How much drug is left in the blood of the patient?");
    let outcome = r["outcome"].as_str().unwrap();
    assert!(matches!(outcome, "NEEDS_INPUT" | "UNRECOGNIZED_FORMAT" | "OUT_OF_SCOPE"), "{r}");
    let t = r["try"].as_str().expect("a template is offered");
    assert!(r["message"].as_str().unwrap().contains(t));
    // the offered template is itself answered
    assert_eq!(ask_value(pack(), t)["outcome"], "SOLVED", "{t}");
}

#[test]
fn the_unit_of_a_value_says_which_input_it_is() {
    // the dimension of a value's unit says which input it is when exactly one input has it
    let r = ask_value(pack(), "What is the concentration of an IV bolus of 500 mg after 6 h with V = 40 L and k = 0.1 1/h?");
    assert_eq!(r["outcome"], "SOLVED", "{r}");
}
