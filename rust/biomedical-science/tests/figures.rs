//! Every Biomedical Science capability's own live figures (src/figures*.rs): its worked example gives each
//! published figure kind, a changed input changes the drawing, and no drawing holds a non-finite number or text outside
//! its canvas.

use ax_engine::figures::{figures, Figure};
use ax_engine::Input;
use biomedical_science_ai999x::pack;

/// The published figure kinds of every capability (publish/website/models/biomedical-science-ai999x/figures,
/// except the generic "curve"), as tools/figure_parity.py lists them.
const KINDS: &[(&str, &[&str])] = &[
    ("bmed_bmi", &["field"]),
    ("bmed_bmr", &["curve-2"]),
    ("bmed_cable_steady_state", &["curve-2"]),
    ("bmed_cohens_kappa", &["table"]),
    ("bmed_cox_hazard_ratio", &["survival"]),
    ("bmed_dx_roc_auc", &["roc"]),
    ("bmed_dx_sensitivity_specificity_ci", &["intervals"]),
    ("bmed_dx_threshold_sensitivity_specificity", &["roc"]),
    ("bmed_elbow_lever_muscle_force", &["free-body"]),
    ("bmed_epi_reed_frost_final_size", &["distribution"]),
    ("bmed_exponential_hazard_ratio", &["survival"]),
    ("bmed_exponential_survival_fit", &["survival"]),
    ("bmed_fitzhugh_nagumo_fixed_point", &["decision", "stability"]),
    ("bmed_gaussian_psf_mtf", &["field"]),
    ("bmed_incidence_rate_ratio_ci", &["intervals"]),
    ("bmed_indicator_dilution_cardiac_output", &["fit"]),
    ("bmed_km_confidence_interval", &["survival"]),
    ("bmed_km_survival_median", &["follow-up", "survival"]),
    ("bmed_logrank_test", &["survival"]),
    ("bmed_mantel_haenszel_odds_ratio", &["forest"]),
    ("bmed_mri_adc_fit", &["fit"]),
    ("bmed_mri_ernst_angle", &["field"]),
    ("bmed_mri_t2_fit", &["fit"]),
    ("bmed_nelson_aalen_hazard", &["survival"]),
    ("bmed_nnt_ci", &["risk"]),
    ("bmed_odds_ratio_ci", &["forest", "table"]),
    ("bmed_pk_auc_iv_bolus", &["pharmacokinetics"]),
    ("bmed_pk_auc_trapezoid", &["trapezoids"]),
    ("bmed_pk_bioavailability_from_profiles", &["pharmacokinetics"]),
    ("bmed_pk_effect_site_concentration", &["pharmacokinetics"]),
    ("bmed_pk_infusion_concentration", &["pharmacokinetics"]),
    ("bmed_pk_iv_bolus_concentration", &["pharmacokinetics"]),
    ("bmed_pk_loading_maintenance", &["pharmacokinetics"]),
    ("bmed_pk_michaelis_menten_concentration", &["pharmacokinetics"]),
    ("bmed_pk_michaelis_menten_time", &["pharmacokinetics"]),
    ("bmed_pk_nca_iv_bolus", &["pharmacokinetics"]),
    ("bmed_pk_nth_dose_iv", &["pharmacokinetics"]),
    ("bmed_pk_oral_concentration", &["pharmacokinetics"]),
    ("bmed_pk_oral_peak", &["pharmacokinetics"]),
    ("bmed_pk_short_infusion", &["pharmacokinetics"]),
    ("bmed_pk_terminal_half_life_fit", &["fit"]),
    ("bmed_pk_two_compartment_bolus", &["pharmacokinetics"]),
    ("bmed_pk_two_compartment_micro_constants", &["schematic"]),
    ("bmed_poiseuille_vessel_flow", &["flow"]),
    ("bmed_relative_risk_ci", &["risk"]),
    ("bmed_respiratory_mechanics_fit", &["fit"]),
    ("bmed_restricted_mean_survival_time", &["survival"]),
    ("bmed_smr_exact_ci", &["intervals"]),
    ("bmed_snr_averaging", &["field"]),
    ("bmed_ultrasound_interface", &["bars", "field"]),
    ("bmed_windkessel_input_impedance", &["bode"]),
    ("bmed_xray_attenuation_layers", &["layers"]),
];

/// Inputs to change for a capability whose inputs are not plain quantities (a second, different example).
const OTHER: &[(&str, &str, &str)] = &[
    ("bmed_cohens_kappa", "counts", "[[30, 12], [8, 50]]"),
    ("bmed_cox_hazard_ratio", "times", "[5, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 36]"),
    ("bmed_dx_roc_auc", "scores_pos", "[0.95, 0.8, 0.65, 0.5, 0.45]"),
    ("bmed_dx_threshold_sensitivity_specificity", "threshold", "0.42"),
    ("bmed_exponential_hazard_ratio", "times1", "[5, 6, 6, 8, 10, 13, 16, 22, 26]"),
    ("bmed_exponential_survival_fit", "times", "[5, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 40]"),
    ("bmed_indicator_dilution_cardiac_output", "C", "[0, 0.7, 3.1, 4.4, 3.2, 2.0, 1.0, 0.4, 0.1]"),
    ("bmed_km_confidence_interval", "t", "20"),
    ("bmed_km_survival_median", "t", "18"),
    ("bmed_logrank_test", "times2", "[1, 2, 2, 3, 3, 4, 5, 6, 7, 9]"),
    ("bmed_mantel_haenszel_odds_ratio", "tables", "[[12, 18, 5, 30], [8, 12, 4, 20], [6, 9, 3, 15]]"),
    ("bmed_mri_adc_fit", "S", "[1200, 500]"),
    ("bmed_mri_t2_fit", "S", "[880, 700, 560, 400, 260]"),
    ("bmed_nelson_aalen_hazard", "t", "20"),
    ("bmed_pk_auc_trapezoid", "C", "[0, 3.2, 6.1, 8.4, 6.6, 4.1, 2.8, 1.2]"),
    ("bmed_pk_bioavailability_from_profiles", "C_oral", "[0, 4.1, 7.0, 6.8, 3.5, 1.5]"),
    ("bmed_pk_nca_iv_bolus", "n_terminal", "3"),
    ("bmed_pk_terminal_half_life_fit", "C", "[9.0, 7.1, 4.6, 3.0, 2.0, 0.85]"),
    ("bmed_respiratory_mechanics_fit", "P", "[5.6, 10.2, 13.5, 16.4, 19.0, 21.0]"),
    ("bmed_restricted_mean_survival_time", "tau", "25"),
    ("bmed_windkessel_input_impedance", "model", "three_element"),
    ("bmed_xray_attenuation_layers", "x", "[1, 2, 2]"),
    ("bmed_epi_reed_frost_final_size", "p", "0.3"),
    ("bmed_fitzhugh_nagumo_fixed_point", "I", "0.5"),
    ("bmed_snr_averaging", "N", "9"),
];

/// A capability refused by the model for these inputs gives no figure (capability, inputs).
const REFUSED: (&str, &[(&str, &str)]) = ("bmed_bmi", &[("m", "70 kg"), ("h", "0 m")]);


/// The arguments of a structured example: solve('key', a='1 V', b=8, c="(A·B)'").
fn example(key: &str) -> Vec<(String, Input)> {
    let p = pack();
    let ex = &p.procedure(key).expect(key).structured_example;
    let cs: Vec<char> = ex.chars().collect();
    let mut i = ex.find(',').map(|x| ex[..x].chars().count() + 1).unwrap_or(cs.len());
    let mut out = vec![];
    while i < cs.len() {
        while i < cs.len() && (cs[i] == ' ' || cs[i] == ',') {
            i += 1;
        }
        if i >= cs.len() || cs[i] == ')' {
            break;
        }
        let st = i;
        while cs[i] != '=' {
            i += 1;
        }
        let name: String = cs[st..i].iter().collect();
        i += 1;
        let value: String;
        if cs[i] == '\'' || cs[i] == '"' {
            let qch = cs[i];
            let s0 = i + 1;
            i += 1;
            while cs[i] != qch {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
            i += 1;
        } else if cs[i] == '[' {
            let s0 = i;
            let mut depth = 0;
            loop {
                if cs[i] == '[' {
                    depth += 1;
                } else if cs[i] == ']' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                i += 1;
            }
            i += 1;
            value = cs[s0..i].iter().collect();
        } else {
            let s0 = i;
            while i < cs.len() && cs[i] != ',' && cs[i] != ')' {
                i += 1;
            }
            value = cs[s0..i].iter().collect();
        }
        out.push((name.trim().to_string(), Input::Text(value.trim().to_string())));
    }
    out
}

fn check_texts(f: &Figure) {
    if let Err(e) = ax_engine::figkit::check_figure(f) {
        panic!("{e}");
    }
}

#[test]
fn every_capability_draws_its_published_kinds() {
    let p = pack();
    let mut missing = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        assert!(ax_engine::solve(p, key, &given, None).solved(), "{key}: the worked example is not solved");
        let figs = figures(p, key, &given, None, None);
        for kind in *kinds {
            match figs.iter().find(|f| f.kind == *kind) {
                Some(f) => {
                    assert_eq!(f.slug, format!("{}-{kind}", key.replace('_', "-")));
                    // "inf" alone also occurs in words (infusion, infected); check_figure looks for "=\"inf" and "inf\""
                    assert!(!f.svg.contains("NaN"), "{key} {kind}: a non-finite number");
                    check_texts(f);
                }
                None => missing.push(format!("{key}: {kind}")),
            }
        }
        if let Some(i) = figs.iter().position(|f| f.slug.contains("-live-")) {
            assert!(figs[i..].iter().all(|f| f.slug.contains("-live-")), "{key}: generic figures before own ones");
        }
    }
    assert!(missing.is_empty(), "missing figures: {missing:?}");
}

/// The numbers of a text multiplied by f ("12 V" -> "16.44 V"; "[3, 6]" -> "[4.11, 8.22]"; "8e-05 m^3/s" ->
/// "0.0001096 m^3/s"): a number that follows a letter or "^" (a unit's power) is kept.
fn scale_text(t: &str, f: f64) -> Option<String> {
    let cs: Vec<char> = t.chars().collect();
    let mut out = String::new();
    let mut changed = false;
    let mut i = 0;
    while i < cs.len() {
        let prev = if i == 0 { ' ' } else { cs[i - 1] };
        let starts = cs[i].is_ascii_digit() || (cs[i] == '-' && i + 1 < cs.len() && cs[i + 1].is_ascii_digit());
        if starts && !(prev.is_alphanumeric() || prev == '^' || prev == '.' || prev == '_') {
            let s0 = i;
            i += 1;
            while i < cs.len() && (cs[i].is_ascii_digit() || cs[i] == '.') {
                i += 1;
            }
            if i + 1 < cs.len() && (cs[i] == 'e' || cs[i] == 'E')
                && (cs[i + 1].is_ascii_digit() || ((cs[i + 1] == '-' || cs[i + 1] == '+') && i + 2 < cs.len())) {
                i += 2;
                while i < cs.len() && cs[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let tok: String = cs[s0..i].iter().collect();
            match tok.parse::<f64>() {
                Ok(x) if x != 0.0 => {
                    out.push_str(&format!("{}", x * f));
                    changed = true;
                }
                _ => out.push_str(&tok),
            }
        } else {
            out.push(cs[i]);
            i += 1;
        }
    }
    changed.then_some(out)
}

#[test]
fn a_changed_input_changes_the_drawing() {
    let p = pack();
    let mut unchanged = vec![];
    for (key, kinds) in KINDS {
        let given = example(key);
        let base = figures(p, key, &given, None, None);
        let mut tries: Vec<Vec<(String, Input)>> = vec![];
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            tries.push(g);
        }
        for f in [1.37, 0.73, 2.0] {
            for i in 0..given.len() {
                if let Input::Text(t) = &given[i].1 {
                    if let Some(t2) = scale_text(t, f) {
                        let mut g = given.clone();
                        g[i].1 = Input::Text(t2);
                        tries.push(g);
                    }
                }
            }
        }
        let mut ok = false;
        for g in tries {
            let a = ax_engine::solve(p, key, &g, None);
            if !a.solved() {
                continue;
            }
            let figs = figures(p, key, &g, None, None);
            let differs = kinds.iter().all(|kind| {
                match (base.iter().find(|f| f.kind == *kind), figs.iter().find(|f| f.kind == *kind)) {
                    (Some(a), Some(b)) => a.svg != b.svg,
                    _ => false,
                }
            });
            if differs {
                ok = true;
                for f in figs.iter().filter(|f| !f.slug.contains("-live-")) {
                    assert!(!f.svg.contains("NaN"), "{key}: a non-finite number");
                    check_texts(f);
                }
                break;
            }
        }
        if !ok {
            unchanged.push(*key);
        }
    }
    assert!(unchanged.is_empty(), "drawing did not follow a changed input: {unchanged:?}");
}

#[test]
fn a_refused_input_draws_nothing() {
    let p = pack();
    let (key, inputs) = REFUSED;
    let given: Vec<(String, Input)> = inputs.iter().map(|(k, v)| (k.to_string(), Input::Text(v.to_string()))).collect();
    let a = ax_engine::solve(p, key, &given, None);
    assert!(!a.solved(), "{key}: expected a refusal");
    assert!(figures(p, key, &given, None, None).is_empty());
}

/// Writes every worked example's own figures (and those of the OTHER inputs, prefixed "x-") as SVG files into
/// $FIG_DUMP, for a contact sheet: cargo test --test figures -- --ignored dump
#[test]
#[ignore]
fn dump() {
    let Ok(dir) = std::env::var("FIG_DUMP") else { return };
    let p = pack();
    for (key, _) in KINDS {
        let given = example(key);
        for f in figures(p, key, &given, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
            std::fs::write(format!("{dir}/{}.svg", f.slug), &f.svg).unwrap();
        }
        for (_, sym, val) in OTHER.iter().filter(|o| o.0 == *key) {
            let mut g = given.clone();
            for e in g.iter_mut() {
                if e.0 == *sym {
                    e.1 = Input::Text(val.to_string());
                }
            }
            for f in figures(p, key, &g, None, None).iter().filter(|f| !f.slug.contains("-live-")) {
                std::fs::write(format!("{dir}/x-{}.svg", f.slug), &f.svg).unwrap();
            }
        }
    }
}
