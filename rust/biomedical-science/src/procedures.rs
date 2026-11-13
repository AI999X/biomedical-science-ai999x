//! biomedical_science: procedures translated from the package's Python by tools/py2rust.py (do not edit; regenerate).
//! Source: models/biomedical_science/capabilities/procedures.py. Each keeps Python's number rules through
//! ax_engine::py and is checked against the package by tools/rust_accuracy.py before the build is listed.
#![allow(clippy::all, non_snake_case, unused_parens, unused_braces, unused_mut, unused_variables, unreachable_code)]

use ax_engine::py::{self, Py, PyErr, R};
use ax_engine::Val;
use serde_json::Value;
use std::collections::HashMap;

/// The procedures this crate implements (tools/export_rust_pack.py reads this list).
pub const NATIVE: &[&str] = &[
    "bmed_bmi",
    "bmed_bmr",
    "bmed_cable_steady_state",
    "bmed_cohens_kappa",
    "bmed_cox_hazard_ratio",
    "bmed_dose_liquid_volume",
    "bmed_dose_tablets",
    "bmed_dose_weight_based",
    "bmed_dx_predictive_values",
    "bmed_dx_roc_auc",
    "bmed_dx_sensitivity_specificity_ci",
    "bmed_dx_threshold_sensitivity_specificity",
    "bmed_dx_youden_cutoff",
    "bmed_elbow_lever_muscle_force",
    "bmed_epi_basic_rates",
    "bmed_epi_herd_immunity",
    "bmed_epi_r_from_growth_rate",
    "bmed_epi_reed_frost_final_size",
    "bmed_epi_seir_final_size",
    "bmed_epi_seir_growth_rate",
    "bmed_epi_seir_peak",
    "bmed_epi_sir_final_size",
    "bmed_epi_sir_peak",
    "bmed_epi_sis_prevalence",
    "bmed_exponential_hazard_ratio",
    "bmed_exponential_survival_fit",
    "bmed_fitzhugh_nagumo_fixed_point",
    "bmed_gaussian_psf_mtf",
    "bmed_ghk_current_density",
    "bmed_ghk_voltage",
    "bmed_hh_gating_steady_state",
    "bmed_hh_potassium_conductance_clamp",
    "bmed_hh_resting_potential",
    "bmed_hounsfield_units",
    "bmed_incidence_rate_ratio_ci",
    "bmed_indicator_dilution_cardiac_output",
    "bmed_iv_flow_rate",
    "bmed_iv_rate_from_dose_rate",
    "bmed_kelvin_voigt_creep",
    "bmed_km_confidence_interval",
    "bmed_km_survival_median",
    "bmed_larmor_frequency",
    "bmed_lif_firing_rate",
    "bmed_lif_subthreshold_voltage",
    "bmed_logrank_test",
    "bmed_mantel_haenszel_odds_ratio",
    "bmed_mri_adc_fit",
    "bmed_mri_ernst_angle",
    "bmed_mri_inversion_recovery",
    "bmed_mri_spin_echo_signal",
    "bmed_mri_spoiled_gradient_echo",
    "bmed_mri_t2_fit",
    "bmed_nelson_aalen_hazard",
    "bmed_nnt_ci",
    "bmed_odds_ratio_ci",
    "bmed_pk_auc_iv_bolus",
    "bmed_pk_auc_trapezoid",
    "bmed_pk_bioavailability_from_profiles",
    "bmed_pk_effect_site_concentration",
    "bmed_pk_hepatic_clearance_parallel_tube",
    "bmed_pk_hepatic_clearance_well_stirred",
    "bmed_pk_infusion_concentration",
    "bmed_pk_intermittent_infusion_steady_state",
    "bmed_pk_iv_bolus_concentration",
    "bmed_pk_loading_maintenance",
    "bmed_pk_michaelis_menten_concentration",
    "bmed_pk_michaelis_menten_steady_state",
    "bmed_pk_michaelis_menten_time",
    "bmed_pk_nca_iv_bolus",
    "bmed_pk_nth_dose_iv",
    "bmed_pk_oral_concentration",
    "bmed_pk_oral_peak",
    "bmed_pk_short_infusion",
    "bmed_pk_steady_state_iv",
    "bmed_pk_steady_state_oral",
    "bmed_pk_terminal_half_life_fit",
    "bmed_pk_time_to_steady_state_fraction",
    "bmed_pk_two_compartment_bolus",
    "bmed_pk_two_compartment_micro_constants",
    "bmed_poiseuille_vessel_flow",
    "bmed_postsynaptic_potential",
    "bmed_reconstitution_concentration",
    "bmed_relative_risk_ci",
    "bmed_respiratory_mechanics_fit",
    "bmed_restricted_mean_survival_time",
    "bmed_sls_creep",
    "bmed_sls_stress_relaxation",
    "bmed_smr_exact_ci",
    "bmed_snr_averaging",
    "bmed_ultrasound_interface",
    "bmed_ultrasound_matching_layer",
    "bmed_ultrasound_wavelength",
    "bmed_vaccine_efficacy",
    "bmed_windkessel_diastolic_decay",
    "bmed_windkessel_input_impedance",
    "bmed_xray_attenuation_layers",
];

type Out = Result<(Vec<(String, Value)>, f64), String>;

/// The run's result as the engine's answer: a dict of outputs and the residual of the package's own check; a
/// ValueError is the refusal's reason; any other error is an internal error (None).
fn finish(r: R, residual: impl Fn(&Py, &Py) -> f64, v: &HashMap<String, Val>) -> Option<Out> {
    let o = match r {
        Ok(o) => o,
        Err(PyErr::Value(m)) => return Some(Err(m)),
        Err(e) => {
            if std::env::var_os("AX_DEBUG").is_some() {        // the error behind an internal error, for diagnosis
                eprintln!("internal error: {e:?}");
            }
            return None;
        }
    };
    let Py::Dict(d) = &o else { return None };
    let outs: Vec<(String, Value)> = d.borrow().iter().map(|(k, x)| (py::to_str(k), py::to_json(x))).collect();
    let i = Py::dict(v.iter().map(|(k, x)| (Py::str(k), py::from_val(x))).collect());
    let res = residual(&i, &o);
    if std::env::var_os("AX_DEBUG").is_some() && !(res <= 1e-9) {
        eprintln!("check failed: residual {res}");
    }
    Some(Ok((outs, res)))
}

/// The engine's entry point (ax_engine::pack::NativeFn): None for a key that is not one of these procedures.
pub fn native(key: &str, v: &HashMap<String, Val>) -> Option<Out> {
    match key {
        "bmed_bmi" => {
            let r = f_bmi(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_26(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_bmr" => {
            let r = f_bmr(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("h") { Some(x) => py::from_val(x), None => { return None } }, match v.get("age") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sex") { Some(x) => py::from_val(x), None => { return None } }, match v.get("equation") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_108(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_cable_steady_state" => {
            let r = f_ne_cable(match v.get("Rm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Ri") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("end") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_189(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_cohens_kappa" => {
            let r = f_dx_kappa(match v.get("counts") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_264(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_cox_hazard_ratio" => {
            let r = f_sv_cox(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_391(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dose_liquid_volume" => {
            let r = f_dose_liquid_volume(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_450(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dose_tablets" => {
            let r = f_dose_tablets(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("H") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_509(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dose_weight_based" => {
            let r = f_dose_weight_based(match v.get("dose_mg_per_kg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("basis") { Some(x) => py::from_val(x), None => { return None } }, match v.get("doses_per_day") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_598(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_predictive_values" => {
            let r = f_dx_predictive_values(match v.get("sensitivity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("specificity") { Some(x) => py::from_val(x), None => { return None } }, match v.get("prevalence") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_663(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_roc_auc" => {
            let r = f_dx_roc_auc(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_731(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_sensitivity_specificity_ci" => {
            let r = f_dx_sens_spec_ci(match v.get("TP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_threshold_sensitivity_specificity" => {
            let r = f_dx_threshold(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("threshold") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_872(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_youden_cutoff" => {
            let r = f_dx_youden(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_951(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_elbow_lever_muscle_force" => {
            let r = f_bm_lever(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W_arm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_arm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_muscle") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1025(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_basic_rates" => {
            let r = f_epi_basic_rates(match v.get("cases") { Some(x) => py::from_val(x), None => { return None } }, match v.get("population") { Some(x) => py::from_val(x), None => { return None } }, match v.get("prevalent") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("deaths") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("period") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1105(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_herd_immunity" => {
            let r = f_epi_herd_immunity(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("efficacy") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1185(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_r_from_growth_rate" => {
            let r = f_epi_r_from_growth(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tg_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tg_sd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1249(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_reed_frost_final_size" => {
            let r = f_epi_reed_frost(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1335(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_final_size" => {
            let r = f_epi_seir_final_size(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1436(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_growth_rate" => {
            let r = f_epi_seir_growth(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1497(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_peak" => {
            let r = f_epi_seir_peak(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1612(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sir_final_size" => {
            let r = f_epi_sir_final_size(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1711(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sir_peak" => {
            let r = f_epi_sir_peak(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1790(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sis_prevalence" => {
            let r = f_epi_sis(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1869(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_exponential_hazard_ratio" => {
            let r = f_sv_exp_hr(match v.get("times1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1950(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_exponential_survival_fit" => {
            let r = f_sv_exponential(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2023(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_fitzhugh_nagumo_fixed_point" => {
            let r = f_ne_fhn(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2121(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_gaussian_psf_mtf" => {
            let r = f_im_psf_mtf(match v.get("fwhm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ghk_current_density" => {
            let r = f_ne_ghk_current(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2262(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ghk_voltage" => {
            let r = f_ne_ghk_voltage(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PNa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PCl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K_i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na_i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cl_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cl_i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2328(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_gating_steady_state" => {
            let r = f_ne_hh_gating(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2396(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_potassium_conductance_clamp" => {
            let r = f_ne_hh_gk_clamp(match v.get("gK_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_hold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_step") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2482(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_resting_potential" => {
            let r = f_ne_hh_rest(match v.get("gNa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ENa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2650(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hounsfield_units" => {
            let r = f_hounsfield(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_water") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_air") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2714(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_incidence_rate_ratio_ci" => {
            let r = f_dx_irr(match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2783(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_indicator_dilution_cardiac_output" => {
            let r = f_bm_indicator_dilution(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2858(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_iv_flow_rate" => {
            let r = f_iv_flow_rate(match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("drop_factor") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2934(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_iv_rate_from_dose_rate" => {
            let r = f_iv_rate_from_dose_rate(match v.get("drug") { Some(x) => py::from_val(x), None => { return None } }, match v.get("bag_volume") { Some(x) => py::from_val(x), None => { return None } }, match v.get("dose_rate") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("dose_ug_per_kg_min") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("W") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("drop_factor") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3019(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_kelvin_voigt_creep" => {
            let r = f_bm_kelvin_voigt(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3084(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_km_confidence_interval" => {
            let r = f_sv_km_ci(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3186(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_km_survival_median" => {
            let r = f_sv_km(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3287(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_larmor_frequency" => {
            let r = f_larmor(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_bar") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3354(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_lif_firing_rate" => {
            let r = f_ne_lif_rate(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_th") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_reset") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_ref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_lif_subthreshold_voltage" => {
            let r = f_ne_lif_voltage(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3494(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_logrank_test" => {
            let r = f_sv_logrank(match v.get("times1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mantel_haenszel_odds_ratio" => {
            let r = f_dx_mantel_haenszel(match v.get("tables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3683(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_adc_fit" => {
            let r = f_im_adc(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3759(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_ernst_angle" => {
            let r = f_ernst_angle(match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3818(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_inversion_recovery" => {
            let r = f_im_inversion_recovery(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TI") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3879(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_spin_echo_signal" => {
            let r = f_im_spin_echo(match v.get("M0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3940(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_spoiled_gradient_echo" => {
            let r = f_im_spoiled_gre(match v.get("M0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2star") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4004(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_t2_fit" => {
            let r = f_im_t2_fit(match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4080(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_nelson_aalen_hazard" => {
            let r = f_sv_nelson_aalen(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4173(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_nnt_ci" => {
            let r = f_dx_nnt(match v.get("events_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4245(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_odds_ratio_ci" => {
            let r = f_dx_odds_ratio(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4310(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_auc_iv_bolus" => {
            let r = f_pk_auc_iv_bolus(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4375(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_auc_trapezoid" => {
            let r = f_pk_auc_trapezoid(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4452(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_bioavailability_from_profiles" => {
            let r = f_pk_bioavailability(match v.get("D_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_oral") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_oral") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_oral") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4528(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_effect_site_concentration" => {
            let r = f_pk_effect_site(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ke0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4595(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_hepatic_clearance_parallel_tube" => {
            let r = f_pk_parallel_tube(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL_int") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4666(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_hepatic_clearance_well_stirred" => {
            let r = f_pk_well_stirred(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL_int") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4737(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_infusion_concentration" => {
            let r = f_pk_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4802(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_intermittent_infusion_steady_state" => {
            let r = f_pk_intermittent_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4863(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_iv_bolus_concentration" => {
            let r = f_pk_iv_bolus(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4928(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_loading_maintenance" => {
            let r = f_pk_loading_maintenance(match v.get("C_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4987(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_concentration" => {
            let r = f_pk_mm_concentration(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_steady_state" => {
            let r = f_pk_mm_steady_state(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5126(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_time" => {
            let r = f_pk_mm_time(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5187(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_nca_iv_bolus" => {
            let r = f_pk_nca_iv(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_terminal") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5284(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_nth_dose_iv" => {
            let r = f_pk_nth_dose(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5355(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_oral_concentration" => {
            let r = f_pk_oral(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5434(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_oral_peak" => {
            let r = f_pk_oral_peak(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5507(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_short_infusion" => {
            let r = f_pk_short_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5574(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_steady_state_iv" => {
            let r = f_pk_ss_iv(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5633(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_steady_state_oral" => {
            let r = f_pk_ss_oral(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5710(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_terminal_half_life_fit" => {
            let r = f_pk_half_life_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5785(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_time_to_steady_state_fraction" => {
            let r = f_pk_time_to_fraction_ss(match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5856(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_two_compartment_bolus" => {
            let r = f_pk_two_comp(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k12") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k21") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5924(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_two_compartment_micro_constants" => {
            let r = f_pk_micro_from_macro(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5985(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_poiseuille_vessel_flow" => {
            let r = f_bm_poiseuille(match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6044(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_postsynaptic_potential" => {
            let r = f_ne_psp(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6116(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_reconstitution_concentration" => {
            let r = f_reconstitution(match v.get("powder") { Some(x) => py::from_val(x), None => { return None } }, match v.get("diluent") { Some(x) => py::from_val(x), None => { return None } }, match v.get("displacement") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("D") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6188(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_relative_risk_ci" => {
            let r = f_dx_relative_risk(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6253(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_respiratory_mechanics_fit" => {
            let r = f_bm_respiratory_fit(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6343(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_restricted_mean_survival_time" => {
            let r = f_sv_rmst(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6437(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_sls_creep" => {
            let r = f_bm_sls_creep(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6503(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_sls_stress_relaxation" => {
            let r = f_bm_sls_relaxation(match v.get("eps0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6568(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_smr_exact_ci" => {
            let r = f_dx_smr(match v.get("observed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("expected") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6671(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_snr_averaging" => {
            let r = f_snr_averaging(match v.get("SNR1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("SNR_target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6744(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_interface" => {
            let r = f_im_us_interface(match v.get("Z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6803(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_matching_layer" => {
            let r = f_im_us_matching(match v.get("Z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6863(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_wavelength" => {
            let r = f_us_wavelength(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1540.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6922(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_vaccine_efficacy" => {
            let r = f_vaccine_efficacy(match v.get("cases_v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_v") { Some(x) => py::from_val(x), None => { return None } }, match v.get("cases_u") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_u") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6987(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_windkessel_diastolic_decay" => {
            let r = f_bm_windkessel_decay(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7054(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_windkessel_input_impedance" => {
            let r = f_bm_windkessel_impedance(match v.get("model") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7122(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_xray_attenuation_layers" => {
            let r = f_im_xray(match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_7197(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        _ => None,
    }
}

#[allow(non_snake_case)]
fn c_F_FARADAY() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(96485.33212331001)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_GAMMA_P_MHZ_T() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(42.577478461)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_KCAL_DAY_W() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.04842592592592593)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_LN2() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(0.6931471805599453)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_R_GAS() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(8.31446261815324)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c_YEAR_S() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(31557600.0)) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__BMR_EQ() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::tuple(vec![Py::str("mifflin_st_jeor"), Py::str("harris_benedict_revised")])) })());
    V.with(|v| v.clone())
}

#[allow(non_snake_case)]
fn c__FWHM_SIGMA() -> R {
    thread_local!(static V: R = (|| -> R { Ok(Py::Float(2.3548200450309493)) })());
    V.with(|v| v.clone())
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_Fraction_half() -> R {
    return Ok(py::fraction(&[Py::Int(1i64), Py::Int(2i64)])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__bisect(mut v_f: Py, mut v_a: Py, mut v_b: Py, mut v_n: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_fa: Py = Py::None;
    let mut v_fb: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_m: Py = Py::None;
    let t6629 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t6629.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t6629[0].clone();
    v_fb = t6629[1].clone();
    if ({ let t6630 = v_fa.clone(); Py::Bool({ let t6631 = Py::Int(0i64); if py::eq(&t6630, &t6631) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t6632 = v_fb.clone(); Py::Bool({ let t6633 = Py::Int(0i64); if py::eq(&t6632, &t6633) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t6634 = { let t6636 = v_fa.clone(); Py::Bool({ let t6637 = Py::Int(0i64); if py::gt(&t6636, &t6637)? { true } else { false } }) }; Py::Bool({ let t6635 = { let t6638 = v_fb.clone(); Py::Bool({ let t6639 = Py::Int(0i64); if py::gt(&t6638, &t6639)? { true } else { false } }) }; if py::ne(&t6634, &t6635) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t6640 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t6640;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t6645 = { let t6641 = v_m.clone(); Py::Bool({ let t6642 = v_a.clone(); if py::eq(&t6641, &t6642) { true } else { false } }) }; if t6645.truthy() { t6645 } else { { let t6643 = v_m.clone(); Py::Bool({ let t6644 = v_b.clone(); if py::eq(&t6643, &t6644) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t6646 = v_fm.clone(); Py::Bool({ let t6647 = Py::Int(0i64); if py::eq(&t6646, &t6647) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t6648 = { let t6650 = v_fm.clone(); Py::Bool({ let t6651 = Py::Int(0i64); if py::gt(&t6650, &t6651)? { true } else { false } }) }; Py::Bool({ let t6649 = { let t6652 = v_fa.clone(); Py::Bool({ let t6653 = Py::Int(0i64); if py::gt(&t6652, &t6653)? { true } else { false } }) }; if py::eq(&t6648, &t6649) { true } else { false } }) }).truthy() {
            let t6654 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t6654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t6654[0].clone();
            v_fa = t6654[1].clone();
        } else {
            v_b = v_m.clone();
        }
    }
    return Ok(py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__count(mut v_name: Py, mut v_v: Py, mut v_lo: Py) -> R {
    let _ = f__need({ let t6966 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6966.truthy() { t6966 } else { { let t6965 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t6965.truthy() { t6965 } else { { let t6963 = v_v.clone(); Py::Bool({ let t6964 = v_lo.clone(); if py::ge(&t6963, &t6964)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
    return Ok(v_v.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__cox_terms(mut v_T: Py, mut v_E: Py, mut v_x: Py, mut v_beta: Py) -> R {
    let mut v_D: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_a2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_l: Py = Py::None;
    let mut v_ll: Py = Py::None;
    let mut v_s0d: Py = Py::None;
    let mut v_s0r: Py = Py::None;
    let mut v_s1d: Py = Py::None;
    let mut v_s1r: Py = Py::None;
    let mut v_s2d: Py = Py::None;
    let mut v_s2r: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_w: Py = Py::None;
    let t334 = Py::Float(0.0);
    v_ll = t334.clone();
    v_U = t334.clone();
    v_I = t334.clone();
    v_w = { let mut out = vec![]; for t335 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t335;
out.push(py::math1("exp", &py::mul(&v_beta.clone(), &v_v.clone())?)?); } Py::list(out) };
    for t340 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t336 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t337 = (t336).items()?;
if t337.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t337[0].clone();
v_e = t337[1].clone();
if ({ let t338 = v_e.clone(); Py::Bool({ let t339 = Py::Int(1i64); if py::eq(&t338, &t339) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t340;
        v_R = { let mut out = vec![]; for t341 in py::iter(&(py::enumerate(&v_T.clone(), 0)?))? { let mut v_j: Py; let mut v_t: Py; 
let t342 = (t341).items()?;
if t342.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t342[0].clone();
v_t = t342[1].clone();
if ({ let t343 = v_t.clone(); Py::Bool({ let t344 = v_u.clone(); if py::ge(&t343, &t344)? { true } else { false } }) }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        v_D = { let mut out = vec![]; for t345 in py::iter(&(py::enumerate(&py::zip(&[v_T.clone(), v_E.clone()])?, 0)?))? { let mut v_e: Py; let mut v_j: Py; let mut v_t: Py; 
let t346 = (t345).items()?;
if t346.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_j = t346[0].clone();
let t347 = (t346[1].clone()).items()?;
if t347.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t347[0].clone();
v_e = t347[1].clone();
if ({ let t352 = { let t348 = v_t.clone(); Py::Bool({ let t349 = v_u.clone(); if py::eq(&t348, &t349) { true } else { false } }) }; if !t352.truthy() { t352 } else { { let t350 = v_e.clone(); Py::Bool({ let t351 = Py::Int(1i64); if py::eq(&t350, &t351) { true } else { false } }) } } }).truthy() { out.push(v_j.clone()); }} Py::list(out) };
        v_d = py::len(&v_D.clone())?;
        v_s0r = py::fsum(&{ let mut out = vec![]; for t353 in py::iter(&(v_R.clone()))? { let mut v_j: Py; 
v_j = t353;
out.push(py::getitem(&v_w.clone(), &v_j.clone())?); } Py::list(out) })?;
        v_s1r = py::fsum(&{ let mut out = vec![]; for t354 in py::iter(&(v_R.clone()))? { let mut v_j: Py; 
v_j = t354;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        v_s2r = py::fsum(&{ let mut out = vec![]; for t355 in py::iter(&(v_R.clone()))? { let mut v_j: Py; 
v_j = t355;
out.push(py::mul(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        v_s0d = py::fsum(&{ let mut out = vec![]; for t356 in py::iter(&(v_D.clone()))? { let mut v_j: Py; 
v_j = t356;
out.push(py::getitem(&v_w.clone(), &v_j.clone())?); } Py::list(out) })?;
        v_s1d = py::fsum(&{ let mut out = vec![]; for t357 in py::iter(&(v_D.clone()))? { let mut v_j: Py; 
v_j = t357;
out.push(py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        v_s2d = py::fsum(&{ let mut out = vec![]; for t358 in py::iter(&(v_D.clone()))? { let mut v_j: Py; 
v_j = t358;
out.push(py::mul(&py::mul(&py::getitem(&v_w.clone(), &v_j.clone())?, &py::getitem(&v_x.clone(), &v_j.clone())?)?, &py::getitem(&v_x.clone(), &v_j.clone())?)?); } Py::list(out) })?;
        v_ll = py::iadd(&v_ll, &py::mul(&v_beta.clone(), &py::fsum(&{ let mut out = vec![]; for t359 in py::iter(&(v_D.clone()))? { let mut v_j: Py; 
v_j = t359;
out.push(py::getitem(&v_x.clone(), &v_j.clone())?); } Py::list(out) })?)?)?;
        v_U = py::iadd(&v_U, &py::fsum(&{ let mut out = vec![]; for t360 in py::iter(&(v_D.clone()))? { let mut v_j: Py; 
v_j = t360;
out.push(py::getitem(&v_x.clone(), &v_j.clone())?); } Py::list(out) })?)?;
        for t361 in py::range_iter(&[(v_d.clone()).i()?]) {
            v_l = t361;
            v_f = py::truediv(&v_l.clone(), &v_d.clone())?;
            let t362 = (Py::tuple(vec![py::sub(&v_s0r.clone(), &py::mul(&v_f.clone(), &v_s0d.clone())?)?, py::sub(&v_s1r.clone(), &py::mul(&v_f.clone(), &v_s1d.clone())?)?, py::sub(&v_s2r.clone(), &py::mul(&v_f.clone(), &v_s2d.clone())?)?])).items()?;
            if t362.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_a0 = t362[0].clone();
            v_a1 = t362[1].clone();
            v_a2 = t362[2].clone();
            v_ll = py::sub(&v_ll, &py::math1("log", &v_a0.clone())?)?;
            v_U = py::sub(&v_U, &py::truediv(&v_a1.clone(), &v_a0.clone())?)?;
            v_I = py::iadd(&v_I, &py::sub(&py::truediv(&v_a2.clone(), &v_a0.clone())?, &py::pow(&py::truediv(&v_a1.clone(), &v_a0.clone())?, &Py::Int(2i64))?)?)?;
        }
    }
    return Ok(Py::tuple(vec![v_ll.clone(), v_U.clone(), v_I.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__exp_rate(mut v_T: Py, mut v_E: Py) -> R {
    let _ = f__need({ let t2006 = py::sum(&v_E.clone(), None)?; Py::Bool({ let t2007 = Py::Int(1i64); if py::ge(&t2006, &t2007)? { true } else { false } }) }, Py::str("at least one event is needed"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t2008 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t2008;
out.push({ let t2009 = v_t.clone(); Py::Bool({ let t2010 = Py::Int(0i64); if py::gt(&t2009, &t2010)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must be positive"))?;
    return Ok(py::truediv(&py::sum(&v_E.clone(), None)?, &py::fsum(&v_T.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t6621 in py::iter(&(v_vals.clone()))? {
        v_v = t6621;
        let _ = f__need({ let t6623 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t6623.truthy() { t6623 } else { { let t6622 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t6622.truthy() { t6622 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__final_size(mut v_R0: Py, mut v_s0: Py, mut v_n0: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    v_f = { let v_R0 = v_R0.clone(); let v_n0 = v_n0.clone(); let v_s0 = v_s0.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::math1("log", &py::truediv(&v_s0.clone(), &v_s.clone())?)?, &py::mul(&v_R0.clone(), &py::sub(&py::add(&v_s0.clone(), &v_n0.clone())?, &v_s.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_lo = py::mul(&py::mul(&Py::Float(0.5), &v_s0.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_R0.clone())?, &py::add(&v_s0.clone(), &v_n0.clone())?)?)?)?;
    v_hi = py::minmax(&[v_s0.clone(), py::truediv(&Py::Float(1.0), &v_R0.clone())?], false)?;
    return Ok(f__bisect(v_f.clone(), v_lo.clone(), v_hi.clone(), Py::Int(400i64))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__frac(mut v_name: Py, mut v_v: Py, mut v_lo_open: Py, mut v_hi_open: Py) -> R {
    let _ = f__fin(v_name.clone(), Py::tuple(vec![v_v.clone()]))?;
    let _ = f__need({ let t5843 = (if (v_lo_open.clone()).truthy() { { let t5835 = v_v.clone(); Py::Bool({ let t5836 = Py::Int(0i64); if py::gt(&t5835, &t5836)? { true } else { false } }) } } else { { let t5837 = v_v.clone(); Py::Bool({ let t5838 = Py::Int(0i64); if py::ge(&t5837, &t5838)? { true } else { false } }) } }); if !t5843.truthy() { t5843 } else { (if (v_hi_open.clone()).truthy() { { let t5839 = v_v.clone(); Py::Bool({ let t5840 = Py::Int(1i64); if py::lt(&t5839, &t5840)? { true } else { false } }) } } else { { let t5841 = v_v.clone(); Py::Bool({ let t5842 = Py::Int(1i64); if py::le(&t5841, &t5842)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__g(mut v_u: Py) -> R {
    return Ok((if ({ let t2535 = v_u.clone(); Py::Bool({ let t2536 = Py::Int(0i64); if py::eq(&t2535, &t2536) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_u.clone(), &py::neg(&py::math1("expm1", &py::neg(&v_u.clone())?)?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ghk_flux_g(mut v_u: Py) -> R {
    return Ok((if ({ let t2260 = v_u.clone(); Py::Bool({ let t2261 = Py::Int(0i64); if py::eq(&t2260, &t2261) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_u.clone(), &py::neg(&py::math1("expm1", &py::neg(&v_u.clone())?)?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ghk_terms(mut v_PK: Py, mut v_PNa: Py, mut v_PCl: Py, mut v_K_o: Py, mut v_K_i: Py, mut v_Na_o: Py, mut v_Na_i: Py, mut v_Cl_o: Py, mut v_Cl_i: Py) -> R {
    return Ok(Py::tuple(vec![py::add(&py::add(&py::mul(&v_PK.clone(), &v_K_o.clone())?, &py::mul(&v_PNa.clone(), &v_Na_o.clone())?)?, &py::mul(&v_PCl.clone(), &v_Cl_i.clone())?)?, py::add(&py::add(&py::mul(&v_PK.clone(), &v_K_i.clone())?, &py::mul(&v_PNa.clone(), &v_Na_i.clone())?)?, &py::mul(&v_PCl.clone(), &v_Cl_o.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hep_inputs(mut v_Q: Py, mut v_fu: Py, mut v_CL_int: Py) -> R {
    let _ = f__pos(Py::str("the blood flow Q and the intrinsic clearance CL_int"), Py::tuple(vec![v_Q.clone(), v_CL_int.clone()]))?;
    let _ = f__frac(Py::str("the unbound fraction fu"), v_fu.clone(), Py::Bool(true), Py::Bool(false))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hh_iv(mut v_v: Py, mut v_gna: Py, mut v_gk: Py, mut v_gl: Py, mut v_ena: Py, mut v_ek: Py, mut v_el: Py, mut v_mpow: Py) -> R {
    let mut v_ah: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_bh: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_bn: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_n: Py = Py::None;
    let t2540 = (f__hh_rates(v_v.clone())?).items()?;
    if t2540.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2541 = (t2540[0].clone()).items()?;
    if t2541.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2541[0].clone();
    v_bm = t2541[1].clone();
    let t2542 = (t2540[1].clone()).items()?;
    if t2542.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2542[0].clone();
    v_bh = t2542[1].clone();
    let t2543 = (t2540[2].clone()).items()?;
    if t2543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2543[0].clone();
    v_bn = t2543[1].clone();
    let t2544 = (Py::tuple(vec![py::truediv(&v_am.clone(), &py::add(&v_am.clone(), &v_bm.clone())?)?, py::truediv(&v_ah.clone(), &py::add(&v_ah.clone(), &v_bh.clone())?)?, py::truediv(&v_an.clone(), &py::add(&v_an.clone(), &v_bn.clone())?)?])).items()?;
    if t2544.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2544[0].clone();
    v_h = t2544[1].clone();
    v_n = t2544[2].clone();
    return Ok(py::add(&py::add(&py::mul(&py::mul(&py::mul(&v_gna.clone(), &py::pow(&v_m.clone(), &v_mpow.clone())?)?, &v_h.clone())?, &py::sub(&v_v.clone(), &v_ena.clone())?)?, &py::mul(&py::mul(&v_gk.clone(), &py::pow(&v_n.clone(), &Py::Int(4i64))?)?, &py::sub(&v_v.clone(), &v_ek.clone())?)?)?, &py::mul(&v_gl.clone(), &py::sub(&v_v.clone(), &v_el.clone())?)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hh_rates(mut v_v: Py) -> R {
    let mut v_ah: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_bh: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_bn: Py = Py::None;
    let t2537 = (Py::tuple(vec![py::mul(&Py::Float(0.1), &f__g(py::truediv(&py::add(&v_v.clone(), &Py::Float(55.0))?, &Py::Float(10.0))?)?)?, py::mul(&Py::Float(0.125), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(80.0))?)?)?])).items()?;
    if t2537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2537[0].clone();
    v_bn = t2537[1].clone();
    let t2538 = (Py::tuple(vec![py::mul(&Py::Float(1.0), &f__g(py::truediv(&py::add(&v_v.clone(), &Py::Float(40.0))?, &Py::Float(10.0))?)?)?, py::mul(&Py::Float(4.0), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(18.0))?)?)?])).items()?;
    if t2538.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2538[0].clone();
    v_bm = t2538[1].clone();
    let t2539 = (Py::tuple(vec![py::mul(&Py::Float(0.07), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(20.0))?)?)?, py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(35.0))?)?, &Py::Float(10.0))?)?)?)?])).items()?;
    if t2539.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2539[0].clone();
    v_bh = t2539[1].clone();
    return Ok(Py::tuple(vec![Py::tuple(vec![v_am.clone(), v_bm.clone()]), Py::tuple(vec![v_ah.clone(), v_bh.clone()]), Py::tuple(vec![v_an.clone(), v_bn.clone()])]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hh_rest(mut v_gNa: Py, mut v_gK: Py, mut v_gL: Py, mut v_ENa: Py, mut v_EK: Py, mut v_EL: Py, mut v_mpow: Py) -> R {
    let mut v_br: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_grid: Py = Py::None;
    let mut v_vals: Py = Py::None;
    v_g = Py::tuple(vec![py::truediv(&v_gNa.clone(), &Py::Float(10.0))?, py::truediv(&v_gK.clone(), &Py::Float(10.0))?, py::truediv(&v_gL.clone(), &Py::Float(10.0))?]);
    v_e = Py::tuple(vec![py::mul(&v_ENa.clone(), &Py::Float(1000.0))?, py::mul(&v_EK.clone(), &Py::Float(1000.0))?, py::mul(&v_EL.clone(), &Py::Float(1000.0))?]);
    v_f = { let v_EK = v_EK.clone(); let v_EL = v_EL.clone(); let v_ENa = v_ENa.clone(); let v_e = v_e.clone(); let v_g = v_g.clone(); let v_gK = v_gK.clone(); let v_gL = v_gL.clone(); let v_gNa = v_gNa.clone(); let v_mpow = v_mpow.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok({ let mut v: Vec<Py> = vec![]; v.push(v_v.clone()); v.extend((v_g.clone()).items()?); v.extend((v_e.clone()).items()?); let mut d: Vec<(Py, Py)> = vec![]; d.push((Py::str("mpow"), v_mpow.clone())); let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "v") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument v".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "gna") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument gna".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "gk") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument gk".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "gl") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument gl".into())) } } }; let a4 = if 4 < v.len() { v[4].clone() } else { match py::kw_take(&mut d, "ena") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument ena".into())) } } }; let a5 = if 5 < v.len() { v[5].clone() } else { match py::kw_take(&mut d, "ek") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument ek".into())) } } }; let a6 = if 6 < v.len() { v[6].clone() } else { match py::kw_take(&mut d, "el") { Some(x) => x, None => { return Err(PyErr::Type("_hh_iv() missing argument el".into())) } } }; let a7 = if 7 < v.len() { v[7].clone() } else { match py::kw_take(&mut d, "mpow") { Some(x) => x, None => Py::Int(3i64) } }; if v.len() > 8 { return Err(PyErr::Type("_hh_iv() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_hh_iv() got an unexpected keyword argument".into())) } f__hh_iv(a0, a1, a2, a3, a4, a5, a6, a7)? });
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_grid = { let mut out = vec![]; for t2545 in py::range_iter(&[(Py::Int(721i64)).i()?]) { let mut v_i: Py; 
v_i = t2545;
out.push(py::add(&py::neg(&Py::Float(120.0))?, &py::mul(&Py::Float(0.25), &v_i.clone())?)?); } Py::list(out) };
    v_vals = { let mut out = vec![]; for t2546 in py::iter(&(v_grid.clone()))? { let mut v_v: Py; 
v_v = t2546;
out.push(py::call(&v_f.clone(), &[v_v.clone()])?); } Py::list(out) };
    v_br = { let mut out = vec![]; for t2547 in py::iter(&(py::zip(&[v_grid.clone(), py::slice(&v_grid.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_vals.clone(), py::slice(&v_vals.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_fa: Py; let mut v_fb: Py; 
let t2548 = (t2547).items()?;
if t2548.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a = t2548[0].clone();
v_b = t2548[1].clone();
v_fa = t2548[2].clone();
v_fb = t2548[3].clone();
if ({ let t2557 = { let t2549 = { let t2551 = v_fa.clone(); Py::Bool({ let t2552 = Py::Int(0i64); if py::gt(&t2551, &t2552)? { true } else { false } }) }; Py::Bool({ let t2550 = { let t2553 = v_fb.clone(); Py::Bool({ let t2554 = Py::Int(0i64); if py::gt(&t2553, &t2554)? { true } else { false } }) }; if py::ne(&t2549, &t2550) { true } else { false } }) }; if t2557.truthy() { t2557 } else { { let t2555 = v_fa.clone(); Py::Bool({ let t2556 = Py::Int(0i64); if py::eq(&t2555, &t2556) { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_a.clone(), v_b.clone()])); }} Py::list(out) };
    let _ = f__need({ let t2558 = py::len(&v_br.clone())?; Py::Bool({ let t2559 = Py::Int(1i64); if py::eq(&t2558, &t2559) { true } else { false } }) }, Py::str("the zero-current steady state is not unique between -120 and +60 mV"))?;
    return Ok(py::truediv(&{ let mut v: Vec<Py> = vec![]; v.push(v_f.clone()); v.extend((py::getitem(&v_br.clone(), &Py::Int(0i64))?).items()?); let mut d: Vec<(Py, Py)> = vec![];  let a0 = if 0 < v.len() { v[0].clone() } else { match py::kw_take(&mut d, "f") { Some(x) => x, None => { return Err(PyErr::Type("_bisect() missing argument f".into())) } } }; let a1 = if 1 < v.len() { v[1].clone() } else { match py::kw_take(&mut d, "a") { Some(x) => x, None => { return Err(PyErr::Type("_bisect() missing argument a".into())) } } }; let a2 = if 2 < v.len() { v[2].clone() } else { match py::kw_take(&mut d, "b") { Some(x) => x, None => { return Err(PyErr::Type("_bisect() missing argument b".into())) } } }; let a3 = if 3 < v.len() { v[3].clone() } else { match py::kw_take(&mut d, "n") { Some(x) => x, None => Py::Int(400i64) } }; if v.len() > 4 { return Err(PyErr::Type("_bisect() takes fewer positional arguments".into())) } if !d.is_empty() { return Err(PyErr::Type("_bisect() got an unexpected keyword argument".into())) } f__bisect(a0, a1, a2, a3)? }, &Py::Float(1000.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__hh_stable(mut v_v: Py, mut v_gNa: Py, mut v_gK: Py, mut v_gL: Py, mut v_ENa: Py, mut v_EK: Py, mut v_EL: Py) -> R {
    let mut v_AM: Py = Py::None;
    let mut v_F: Py = Py::None;
    let mut v_J: Py = Py::None;
    let mut v_M: Py = Py::None;
    let mut v_ah: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_bh: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_bn: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_c3: Py = Py::None;
    let mut v_c4: Py = Py::None;
    let mut v_ck: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_fm: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_hstep: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_y0: Py = Py::None;
    let mut v_ym: Py = Py::None;
    let mut v_yp: Py = Py::None;
    v_g = Py::tuple(vec![py::truediv(&v_gNa.clone(), &Py::Float(10.0))?, py::truediv(&v_gK.clone(), &Py::Float(10.0))?, py::truediv(&v_gL.clone(), &Py::Float(10.0))?]);
    v_e = Py::tuple(vec![py::mul(&v_ENa.clone(), &Py::Float(1000.0))?, py::mul(&v_EK.clone(), &Py::Float(1000.0))?, py::mul(&v_EL.clone(), &Py::Float(1000.0))?]);
    v_F = { let v_EK = v_EK.clone(); let v_EL = v_EL.clone(); let v_ENa = v_ENa.clone(); let v_F = v_F.clone(); let v_e = v_e.clone(); let v_g = v_g.clone(); let v_gK = v_gK.clone(); let v_gL = v_gL.clone(); let v_gNa = v_gNa.clone(); let v_v = v_v.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_y: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_ah: Py = Py::None;
            let mut v_am: Py = Py::None;
            let mut v_an: Py = Py::None;
            let mut v_bh: Py = Py::None;
            let mut v_bm: Py = Py::None;
            let mut v_bn: Py = Py::None;
            let mut v_h: Py = Py::None;
            let mut v_m: Py = Py::None;
            let mut v_n: Py = Py::None;
            let mut v_vv: Py = Py::None;
            let t2586 = (v_y.clone()).items()?;
            if t2586.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_vv = t2586[0].clone();
            v_m = t2586[1].clone();
            v_h = t2586[2].clone();
            v_n = t2586[3].clone();
            let t2587 = (f__hh_rates(v_vv.clone())?).items()?;
            if t2587.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            let t2588 = (t2587[0].clone()).items()?;
            if t2588.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_am = t2588[0].clone();
            v_bm = t2588[1].clone();
            let t2589 = (t2587[1].clone()).items()?;
            if t2589.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_ah = t2589[0].clone();
            v_bh = t2589[1].clone();
            let t2590 = (t2587[2].clone()).items()?;
            if t2590.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_an = t2590[0].clone();
            v_bn = t2590[1].clone();
            return Ok(Py::list(vec![py::neg(&py::add(&py::add(&py::mul(&py::mul(&py::mul(&py::getitem(&v_g.clone(), &Py::Int(0i64))?, &py::pow(&v_m.clone(), &Py::Int(3i64))?)?, &v_h.clone())?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::mul(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, &py::pow(&v_n.clone(), &Py::Int(4i64))?)?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&v_g.clone(), &Py::Int(2i64))?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(2i64))?)?)?)?)?, py::sub(&py::mul(&v_am.clone(), &py::sub(&Py::Int(1i64), &v_m.clone())?)?, &py::mul(&v_bm.clone(), &v_m.clone())?)?, py::sub(&py::mul(&v_ah.clone(), &py::sub(&Py::Int(1i64), &v_h.clone())?)?, &py::mul(&v_bh.clone(), &v_h.clone())?)?, py::sub(&py::mul(&v_an.clone(), &py::sub(&Py::Int(1i64), &v_n.clone())?)?, &py::mul(&v_bn.clone(), &v_n.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2591 = (f__hh_rates(v_v.clone())?).items()?;
    if t2591.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2592 = (t2591[0].clone()).items()?;
    if t2592.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2592[0].clone();
    v_bm = t2592[1].clone();
    let t2593 = (t2591[1].clone()).items()?;
    if t2593.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2593[0].clone();
    v_bh = t2593[1].clone();
    let t2594 = (t2591[2].clone()).items()?;
    if t2594.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2594[0].clone();
    v_bn = t2594[1].clone();
    v_y0 = Py::list(vec![v_v.clone(), py::truediv(&v_am.clone(), &py::add(&v_am.clone(), &v_bm.clone())?)?, py::truediv(&v_ah.clone(), &py::add(&v_ah.clone(), &v_bh.clone())?)?, py::truediv(&v_an.clone(), &py::add(&v_an.clone(), &v_bn.clone())?)?]);
    v_J = { let mut out = vec![]; for t2595 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v__: Py; 
v__ = t2595;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &Py::Int(4i64))?); } Py::list(out) };
    for t2596 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
        v_j = t2596;
        v_hstep = py::mul(&Py::Float(1e-06), &(if ({ let t2597 = v_j.clone(); Py::Bool({ let t2598 = Py::Int(0i64); if py::eq(&t2597, &t2598) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.01) }))?;
        let t2599 = (Py::tuple(vec![Py::list((v_y0.clone()).items()?), Py::list((v_y0.clone()).items()?)])).items()?;
        if t2599.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_yp = t2599[0].clone();
        v_ym = t2599[1].clone();
        let t2600 = v_yp.clone(); let t2601 = v_j.clone();
        py::setitem(&t2600, &t2601, py::iadd(&py::getitem(&t2600, &t2601)?, &v_hstep.clone())?)?;
        let t2602 = v_ym.clone(); let t2603 = v_j.clone();
        py::setitem(&t2602, &t2603, py::sub(&py::getitem(&t2602, &t2603)?, &v_hstep.clone())?)?;
        let t2604 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_yp.clone()])?, py::call(&v_F.clone(), &[v_ym.clone()])?])).items()?;
        if t2604.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fp = t2604[0].clone();
        v_fm = t2604[1].clone();
        for t2605 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v_i = t2605;
            py::setitem(&py::getitem(&v_J.clone(), &v_i.clone())?, &v_j.clone(), py::truediv(&py::sub(&py::getitem(&v_fp.clone(), &v_i.clone())?, &py::getitem(&v_fm.clone(), &v_i.clone())?)?, &py::mul(&Py::Int(2i64), &v_hstep.clone())?)?)?;
        }
    }
    let t2607 = (Py::tuple(vec![Py::Int(4i64), { let mut out = vec![]; for t2606 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v__: Py; 
v__ = t2606;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &Py::Int(4i64))?); } Py::list(out) }, Py::list(vec![Py::Float(1.0)])])).items()?;
    if t2607.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2607[0].clone();
    v_M = t2607[1].clone();
    v_c = t2607[2].clone();
    for t2608 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2608;
        v_AM = { let mut out = vec![]; for t2609 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2609;
out.push({ let mut out = vec![]; for t2610 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t2610;
out.push(py::sum(&{ let mut out = vec![]; for t2611 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_l: Py; 
v_l = t2611;
out.push(py::mul(&py::getitem(&py::getitem(&v_J.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        if ({ let t2612 = v_k.clone(); Py::Bool({ let t2613 = Py::Int(1i64); if py::eq(&t2612, &t2613) { true } else { false } }) }).truthy() {
            v_AM = { let mut out = vec![]; for t2614 in py::iter(&(v_J.clone()))? { let mut v_row: Py; 
v_row = t2614;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
        }
        v_ck = py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t2615 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2615;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?, &v_k.clone())?;
        let _ = py::method(&v_c.clone(), "append", &[v_ck.clone()])?;
        v_M = { let mut out = vec![]; for t2616 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2616;
out.push({ let mut out = vec![]; for t2617 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t2617;
out.push(py::add(&py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_j.clone())?, &(if ({ let t2618 = v_i.clone(); Py::Bool({ let t2619 = v_j.clone(); if py::eq(&t2618, &t2619) { true } else { false } }) }).truthy() { v_ck.clone() } else { Py::Float(0.0) }))?); } Py::list(out) }); } Py::list(out) };
    }
    let t2620 = (py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?).items()?;
    if t2620.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c1 = t2620[0].clone();
    v_c2 = t2620[1].clone();
    v_c3 = t2620[2].clone();
    v_c4 = t2620[3].clone();
    return Ok({ let t2637 = { let t2621 = v_c1.clone(); Py::Bool({ let t2622 = Py::Int(0i64); if py::gt(&t2621, &t2622)? { true } else { false } }) }; if !t2637.truthy() { t2637 } else { { let t2636 = { let t2623 = v_c2.clone(); Py::Bool({ let t2624 = Py::Int(0i64); if py::gt(&t2623, &t2624)? { true } else { false } }) }; if !t2636.truthy() { t2636 } else { { let t2635 = { let t2625 = v_c3.clone(); Py::Bool({ let t2626 = Py::Int(0i64); if py::gt(&t2625, &t2626)? { true } else { false } }) }; if !t2635.truthy() { t2635 } else { { let t2634 = { let t2627 = v_c4.clone(); Py::Bool({ let t2628 = Py::Int(0i64); if py::gt(&t2627, &t2628)? { true } else { false } }) }; if !t2634.truthy() { t2634 } else { { let t2633 = { let t2629 = py::mul(&v_c1.clone(), &v_c2.clone())?; Py::Bool({ let t2630 = v_c3.clone(); if py::gt(&t2629, &t2630)? { true } else { false } }) }; if !t2633.truthy() { t2633 } else { { let t2631 = py::mul(&py::mul(&v_c1.clone(), &v_c2.clone())?, &v_c3.clone())?; Py::Bool({ let t2632 = py::add(&py::mul(&v_c3.clone(), &v_c3.clone())?, &py::mul(&py::mul(&v_c1.clone(), &v_c1.clone())?, &v_c4.clone())?)?; if py::gt(&t2631, &t2632)? { true } else { false } }) } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t3337 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t3337.truthy() { t3337 } else { { let t3336 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t3336.truthy() { t3336 } else { py::math1("isfinite", &v_v.clone())? } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ka_k(mut v_ka: Py, mut v_k: Py) -> R {
    let _ = f__pos(Py::str("the rate constants ka and k"), Py::tuple(vec![v_ka.clone(), v_k.clone()]))?;
    let _ = f__need({ let t5692 = py::abs(&py::sub(&v_ka.clone(), &v_k.clone())?)?; Py::Bool({ let t5693 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_ka.clone(), v_k.clone()], true)?)?; if py::gt(&t5692, &t5693)? { true } else { false } }) }, Py::str("ka equals k: the Bateman form does not apply (ka = k is not covered)"))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__km_curve(mut v_T: Py, mut v_E: Py) -> R {
    let mut v_curve: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t3248 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, Py::list(vec![])])).items()?;
    if t3248.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3248[0].clone();
    v_curve = t3248[1].clone();
    for t3264 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t3265 = (t3264).items()?;
        if t3265.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3265[0].clone();
        v_n = t3265[1].clone();
        v_d = t3265[2].clone();
        v_s = py::mul(&v_s, &py::fraction(&[py::sub(&v_n.clone(), &v_d.clone())?, v_n.clone()])?)?;
        let _ = py::method(&v_curve.clone(), "append", &[Py::tuple(vec![v_u.clone(), v_s.clone(), v_n.clone(), v_d.clone()])])?;
    }
    return Ok(v_curve.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__km_steps(mut v_T: Py, mut v_E: Py) -> R {
    let mut v_out: Py = Py::None;
    let mut v_u: Py = Py::None;
    v_out = Py::list(vec![]);
    for t6410 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t6406 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t6407 = (t6406).items()?;
if t6407.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t6407[0].clone();
v_e = t6407[1].clone();
if ({ let t6408 = v_e.clone(); Py::Bool({ let t6409 = Py::Int(1i64); if py::eq(&t6408, &t6409) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t6410;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_u.clone(), py::sum(&{ let mut out = vec![]; for t6411 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t6411;
if ({ let t6412 = v_t.clone(); Py::Bool({ let t6413 = v_u.clone(); if py::ge(&t6412, &t6413)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t6414 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t6415 = (t6414).items()?;
if t6415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t6415[0].clone();
v_e = t6415[1].clone();
if ({ let t6420 = { let t6416 = v_t.clone(); Py::Bool({ let t6417 = v_u.clone(); if py::eq(&t6416, &t6417) { true } else { false } }) }; if !t6420.truthy() { t6420 } else { { let t6418 = v_e.clone(); Py::Bool({ let t6419 = Py::Int(1i64); if py::eq(&t6418, &t6419) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__level(mut v_level: Py) -> R {
    let _ = f__fin(Py::str("the confidence level"), Py::tuple(vec![v_level.clone()]))?;
    let _ = f__need({ let t6624 = Py::Float(0.5); Py::Bool({ let t6625 = v_level.clone(); if py::le(&t6624, &t6625)? { { let t6626 = Py::Int(1i64); if py::lt(&t6625, &t6626)? { true } else { false } } } else { false } }) }, Py::str("the confidence level must be a fraction such as 0.95 (or 95%)"))?;
    return Ok(py::normal_dist("inv_cdf", &Py::Float(0.0), &Py::Float(1.0), &[py::add(&Py::Float(0.5), &py::truediv(&v_level.clone(), &Py::Float(2.0))?)?])?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__loglin(mut v_t: Py, mut v_y: Py, mut v_name: Py) -> R {
    let mut v_ly: Py = Py::None;
    let mut v_mt: Py = Py::None;
    let mut v_my: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_sxx: Py = Py::None;
    let mut v_sxy: Py = Py::None;
    let _ = f__need({ let t5756 = py::len(&v_t.clone())?; Py::Bool({ let t5757 = py::len(&v_y.clone())?; if py::eq(&t5756, &t5757) { true } else { false } }) }, Py::str("give one value per time"))?;
    let _ = f__need({ let t5758 = py::len(&v_t.clone())?; Py::Bool({ let t5759 = Py::Int(2i64); if py::ge(&t5758, &t5759)? { true } else { false } }) }, Py::str("give at least two points"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5760 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t5760;
out.push({ let t5761 = v_v.clone(); Py::Bool({ let t5762 = Py::Int(0i64); if py::gt(&t5761, &t5762)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be positive (a logarithm is taken)".to_string()].concat()))?;
    let _ = f__need({ let t5763 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t5764 = Py::Int(2i64); if py::ge(&t5763, &t5764)? { true } else { false } }) }, Py::str("the times must not all be equal"))?;
    v_ly = { let mut out = vec![]; for t5765 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t5765;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_t.clone())?;
    let t5766 = (Py::tuple(vec![py::truediv(&py::fsum(&v_t.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_ly.clone())?, &v_n.clone())?])).items()?;
    if t5766.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mt = t5766[0].clone();
    v_my = t5766[1].clone();
    v_sxx = py::fsum(&{ let mut out = vec![]; for t5767 in py::iter(&(v_t.clone()))? { let mut v_a: Py; 
v_a = t5767;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mt.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_sxy = py::fsum(&{ let mut out = vec![]; for t5768 in py::iter(&(py::zip(&[v_t.clone(), v_ly.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5769 = (t5768).items()?;
if t5769.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5769[0].clone();
v_b = t5769[1].clone();
out.push(py::mul(&py::sub(&v_a.clone(), &v_mt.clone())?, &py::sub(&v_b.clone(), &v_my.clone())?)?); } Py::list(out) })?;
    v_slope = py::truediv(&v_sxy.clone(), &v_sxx.clone())?;
    return Ok(Py::tuple(vec![v_slope.clone(), py::sub(&v_my.clone(), &py::mul(&v_slope.clone(), &v_mt.clone())?)?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__loglin_fit_out(mut v_x: Py, mut v_S: Py, mut v_name: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let _ = f__need({ let t4049 = { let t4045 = py::len(&v_x.clone())?; Py::Bool({ let t4046 = py::len(&v_S.clone())?; if py::eq(&t4045, &t4046) { true } else { false } }) }; if !t4049.truthy() { t4049 } else { { let t4047 = py::len(&v_x.clone())?; Py::Bool({ let t4048 = Py::Int(2i64); if py::ge(&t4047, &t4048)? { true } else { false } }) } } }, Py::str("give at least two points and one signal per point"))?;
    let t4064 = (f__loglin(v_x.clone(), v_S.clone(), Py::str("the signals"))?).items()?;
    if t4064.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t4064[0].clone();
    v_b = t4064[1].clone();
    let _ = f__need({ let t4065 = v_slope.clone(); Py::Bool({ let t4066 = Py::Int(0i64); if py::lt(&t4065, &t4066)? { true } else { false } }) }, Py::str(&["the signal does not decay with ".to_string(), py::format(&v_name.clone(), "")?].concat()))?;
    return Ok(Py::tuple(vec![v_slope.clone(), v_b.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__macro(mut v_k10: Py, mut v_k12: Py, mut v_k21: Py) -> R {
    let mut v_alpha: Py = Py::None;
    let mut v_disc: Py = Py::None;
    let mut v_s: Py = Py::None;
    v_s = py::add(&py::add(&v_k10.clone(), &v_k12.clone())?, &v_k21.clone())?;
    v_disc = py::math1("sqrt", &py::minmax(&[Py::Float(0.0), py::sub(&py::mul(&v_s.clone(), &v_s.clone())?, &py::mul(&py::mul(&Py::Float(4.0), &v_k10.clone())?, &v_k21.clone())?)?], true)?)?;
    v_alpha = py::truediv(&py::add(&v_s.clone(), &v_disc.clone())?, &Py::Float(2.0))?;
    return Ok(Py::tuple(vec![v_alpha.clone(), py::truediv(&py::mul(&v_k10.clone(), &v_k21.clone())?, &v_alpha.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__need(mut v_ok: Py, mut v_msg: Py) -> R {
    if (py::not(&v_ok.clone())).truthy() {
        return Err(py::value_error(py::to_str(&v_msg.clone())));
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__nonneg(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t7034 in py::iter(&(v_vals.clone()))? {
        v_v = t7034;
        let _ = f__need({ let t7039 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t7039.truthy() { t7039 } else { { let t7038 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t7038.truthy() { t7038 } else { { let t7037 = py::math1("isfinite", &v_v.clone())?; if !t7037.truthy() { t7037 } else { { let t7035 = v_v.clone(); Py::Bool({ let t7036 = Py::Int(0i64); if py::ge(&t7035, &t7036)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ols(mut v_X: Py, mut v_y: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::len(&v_X.clone())?;
    v_A = { let mut out = vec![]; for t6306 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t6306;
out.push({ let mut out = vec![]; for t6307 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t6307;
out.push(py::fsum(&{ let mut out = vec![]; for t6308 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t6308;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_b = { let mut out = vec![]; for t6309 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t6309;
out.push(py::fsum(&{ let mut out = vec![]; for t6310 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t6310;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(f__solve_lin(v_A.clone(), v_b.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t7185 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t7186 = (t7185).items()?;
if t7186.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t7186[0].clone();
v_v = t7186[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_cdf(mut v_k: Py, mut v_lam: Py) -> R {
    let mut v_ll: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_terms: Py = Py::None;
    if ({ let t6655 = v_lam.clone(); Py::Bool({ let t6656 = Py::Int(0i64); if py::eq(&t6655, &t6656) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_ll = py::math1("log", &v_lam.clone())?;
    v_terms = { let mut out = vec![]; for t6657 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t6657;
out.push(py::sub(&py::add(&py::neg(&v_lam.clone())?, &py::mul(&v_j.clone(), &v_ll.clone())?)?, &py::math1("lgamma", &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    v_mx = py::minmax(&[v_terms.clone()], true)?;
    return Ok(py::minmax(&[Py::Float(1.0), py::mul(&py::math1("exp", &v_mx.clone())?, &py::fsum(&{ let mut out = vec![]; for t6658 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t6658;
out.push(py::math1("exp", &py::sub(&v_t.clone(), &v_mx.clone())?)?); } Py::list(out) })?)?], false)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t7163 in py::iter(&(v_vals.clone()))? {
        v_v = t7163;
        let _ = f__need({ let t7168 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t7168.truthy() { t7168 } else { { let t7167 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t7167.truthy() { t7167 } else { { let t7166 = py::math1("isfinite", &v_v.clone())?; if !t7166.truthy() { t7166 } else { { let t7164 = v_v.clone(); Py::Bool({ let t7165 = Py::Int(0i64); if py::gt(&t7164, &t7165)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t7190 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t7190.truthy() { t7190 } else { { let t7189 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t7189.truthy() { t7189 } else { { let t7187 = v_x.clone(); Py::Bool({ let t7188 = Py::None; if py::is(&t7187, &t7188) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t7191 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t7191;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t7194 = { let t7192 = v_x.clone(); Py::Bool({ let t7193 = Py::Int(0i64); if py::eq(&t7192, &t7193) { true } else { false } }) }; if t7194.truthy() { t7194 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t7195 = v_x.clone(); Py::Bool({ let t7196 = Py::Int(0i64); if py::eq(&t7195, &t7196) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t7209 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t7209.truthy() { t7209 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t7213 = py::not(&{ let t7210 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t7210.truthy() { t7210 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t7213.truthy() { t7213 } else { { let t7211 = py::len(&v_a.clone())?; Py::Bool({ let t7212 = py::len(&v_b.clone())?; if py::ne(&t7211, &t7212) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t7214 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t7215 = (t7214).items()?;
if t7215.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t7215[0].clone();
v_y = t7215[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t7222 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t7222.truthy() { t7222 } else { { let t7221 = Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))); if t7221.truthy() { t7221 } else { { let t7220 = { let t7216 = v_a.clone(); Py::Bool({ let t7217 = Py::None; if py::is(&t7216, &t7217) { true } else { false } }) }; if t7220.truthy() { t7220 } else { { let t7218 = v_b.clone(); Py::Bool({ let t7219 = Py::None; if py::is(&t7218, &t7219) { true } else { false } }) } } } } } } }).truthy() {
        return Ok((if ({ let t7223 = v_a.clone(); Py::Bool({ let t7224 = v_b.clone(); if py::eq(&t7223, &t7224) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t7226: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t7225 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t7225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t7225[0].clone();
            v_b = t7225[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t7226 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t7227 = py::math1("isfinite", &v_a.clone())?; if !t7227.truthy() { t7227 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok((if ({ let t7228 = v_a.clone(); Py::Bool({ let t7229 = v_b.clone(); if py::eq(&t7228, &t7229) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t7230 = v_d.clone(); Py::Bool({ let t7231 = Py::Int(0i64); if py::eq(&t7230, &t7231) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t7236 = { let t7232 = py::abs(&v_a.clone())?; Py::Bool({ let t7233 = Py::Float(1e-15); if py::lt(&t7232, &t7233)? { true } else { false } }) }; if !t7236.truthy() { t7236 } else { { let t7234 = py::abs(&v_b.clone())?; Py::Bool({ let t7235 = Py::Float(1e-15); if py::lt(&t7234, &t7235)? { true } else { false } }) } } }).truthy() {
        return Ok(Py::Float(0.0));
    }
    return Ok(py::truediv(&v_d.clone(), &py::minmax(&[py::abs(&v_a.clone())?, py::abs(&v_b.clone())?], true)?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__seir_taylor_peak(mut v_b: Py, mut v_c: Py, mut v_s0: Py, mut v_e0: Py, mut v_i0: Py, mut v_K: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_G: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_X: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_ev: Py = Py::None;
    let mut v_g_end: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_risen: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_th: Py = Py::None;
    let mut v_top: Py = Py::None;
    let t1564 = (Py::tuple(vec![v_s0.clone(), v_e0.clone(), v_i0.clone(), Py::Float(0.0)])).items()?;
    if t1564.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1564[0].clone();
    v_e = t1564[1].clone();
    v_i = t1564[2].clone();
    v_tau = t1564[3].clone();
    v_risen = { let t1565 = py::sub(&py::mul(&v_c.clone(), &v_e.clone())?, &v_i.clone())?; Py::Bool({ let t1566 = Py::Int(0i64); if py::gt(&t1565, &t1566)? { true } else { false } }) };
    for t1567 in py::range_iter(&[(Py::Int(200000i64)).i()?]) {
        v__ = t1567;
        let t1568 = (Py::tuple(vec![Py::list(vec![v_s.clone()]), Py::list(vec![v_e.clone()]), Py::list(vec![v_i.clone()])])).items()?;
        if t1568.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_S = t1568[0].clone();
        v_E = t1568[1].clone();
        v_I = t1568[2].clone();
        for t1569 in py::range_iter(&[(v_K.clone()).i()?]) {
            v_k = t1569;
            v_p = py::fsum(&{ let mut out = vec![]; for t1570 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t1570;
out.push(py::mul(&py::getitem(&v_S.clone(), &v_j.clone())?, &py::getitem(&v_I.clone(), &py::sub(&v_k.clone(), &v_j.clone())?)?)?); } Py::list(out) })?;
            let _ = py::method(&v_S.clone(), "append", &[py::truediv(&py::mul(&py::neg(&v_b.clone())?, &v_p.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
            let _ = py::method(&v_E.clone(), "append", &[py::truediv(&py::sub(&py::mul(&v_b.clone(), &v_p.clone())?, &py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
            let _ = py::method(&v_I.clone(), "append", &[py::truediv(&py::sub(&py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?, &py::getitem(&v_I.clone(), &v_k.clone())?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
        }
        v_h = Py::Float(2.0);
        for t1571 in py::iter(&(Py::tuple(vec![v_S.clone(), v_E.clone(), v_I.clone()])))? {
            v_X = t1571;
            v_top = py::minmax(&[py::abs(&py::getitem(&v_X.clone(), &v_K.clone())?)?, (if (py::getitem(&v_X.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?).truthy() { py::pow(&py::abs(&py::getitem(&v_X.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?)?, &py::truediv(&v_K.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(0.0) })], true)?;
            if ({ let t1576 = { let t1572 = v_top.clone(); Py::Bool({ let t1573 = Py::Int(0i64); if py::gt(&t1572, &t1573)? { true } else { false } }) }; if !t1576.truthy() { t1576 } else { { let t1574 = py::getitem(&v_X.clone(), &Py::Int(0i64))?; Py::Bool({ let t1575 = Py::Int(0i64); if py::gt(&t1574, &t1575)? { true } else { false } }) } } }).truthy() {
                v_h = py::minmax(&[v_h.clone(), py::pow(&py::truediv(&py::mul(&Py::Float(1e-17), &py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, &v_top.clone())?, &py::truediv(&Py::Float(1.0), &v_K.clone())?)?], false)?;
            }
        }
        v_G = { let mut out = vec![]; for t1577 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1577;
out.push(py::sub(&py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?, &py::getitem(&v_I.clone(), &v_k.clone())?)?); } Py::list(out) };
        v_ev = { let v_E = v_E.clone(); let v_G = v_G.clone(); let v_I = v_I.clone(); let v_K = v_K.clone(); let v_S = v_S.clone(); let v_X = v_X.clone(); let v__ = v__.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_e = v_e.clone(); let v_e0 = v_e0.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_i0 = v_i0.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); let v_risen = v_risen.clone(); let v_s = v_s.clone(); let v_s0 = v_s0.clone(); let v_tau = v_tau.clone(); let v_top = v_top.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_x: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t1578 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1578;
out.push(py::mul(&py::getitem(&v_P.clone(), &v_k.clone())?, &py::pow(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_g_end = py::call(&v_ev.clone(), &[v_G.clone(), v_h.clone()])?;
        if ({ let t1581 = py::not(&v_risen.clone()); if !t1581.truthy() { t1581 } else { { let t1579 = v_g_end.clone(); Py::Bool({ let t1580 = Py::Int(0i64); if py::gt(&t1579, &t1580)? { true } else { false } }) } } }).truthy() {
            v_risen = Py::Bool(true);
        } else {
            if ({ let t1584 = v_risen.clone(); if !t1584.truthy() { t1584 } else { { let t1582 = v_g_end.clone(); Py::Bool({ let t1583 = Py::Int(0i64); if py::le(&t1582, &t1583)? { true } else { false } }) } } }).truthy() {
                let t1585 = (Py::tuple(vec![Py::Float(0.0), v_h.clone()])).items()?;
                if t1585.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_lo = t1585[0].clone();
                v_hi = t1585[1].clone();
                for t1586 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
                    v__ = t1586;
                    v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
                    if ({ let t1587 = v_m.clone(); Py::Bool({ let t1588 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1588, &t1587)? { true } else { false } }) }).truthy() {
                        break;
                    }
                    if ({ let t1589 = py::call(&v_ev.clone(), &[v_G.clone(), v_m.clone()])?; Py::Bool({ let t1590 = Py::Int(0i64); if py::gt(&t1589, &t1590)? { true } else { false } }) }).truthy() {
                        v_lo = v_m.clone();
                    } else {
                        v_hi = v_m.clone();
                    }
                }
                v_th = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
                return Ok(Py::tuple(vec![py::add(&v_tau.clone(), &v_th.clone())?, py::call(&v_ev.clone(), &[v_S.clone(), v_th.clone()])?, py::call(&v_ev.clone(), &[v_E.clone(), v_th.clone()])?, py::call(&v_ev.clone(), &[v_I.clone(), v_th.clone()])?]));
            }
        }
        let t1591 = (Py::tuple(vec![py::call(&v_ev.clone(), &[v_S.clone(), v_h.clone()])?, py::call(&v_ev.clone(), &[v_E.clone(), v_h.clone()])?, py::call(&v_ev.clone(), &[v_I.clone(), v_h.clone()])?])).items()?;
        if t1591.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1591[0].clone();
        v_e = t1591[1].clone();
        v_i = t1591[2].clone();
        v_tau = py::iadd(&v_tau, &v_h.clone())?;
        if ({ let t1594 = py::not(&v_risen.clone()); if !t1594.truthy() { t1594 } else { { let t1592 = py::mul(&v_b.clone(), &v_s.clone())?; Py::Bool({ let t1593 = Py::Float(1.0); if py::le(&t1592, &t1593)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1595 = v_tau.clone(); Py::Bool({ let t1596 = Py::Float(100000.0); if py::gt(&t1595, &t1596)? { true } else { false } }) }).truthy() {
            break;
        }
    }
    return Err(py::value_error(py::to_str(&Py::str("no epidemic peak: the infectious fraction never rises"))));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__sir_peak_time(mut v_beta: Py, mut v_gamma: Py, mut v_s0: Py, mut v_i0: Py) -> R {
    let mut v_I0: Py = Py::None;
    let mut v_R0: Py = Py::None;
    let mut v_S0: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let mut v_pts: Py = Py::None;
    let mut v_sp: Py = Py::None;
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    {
        let _g0 = ax_engine::pympf::scope("workdps", &Py::Int(25i64))?;
        let t1786 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_beta.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_gamma.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_s0.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_i0.clone()], &[])?])).items()?;
        if t1786.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_b = t1786[0].clone();
        v_g = t1786[1].clone();
        v_S0 = t1786[2].clone();
        v_I0 = t1786[3].clone();
        v_sp = py::truediv(&v_g.clone(), &v_b.clone())?;
        v_f = { let v_I0 = v_I0.clone(); let v_R0 = v_R0.clone(); let v_S0 = v_S0.clone(); let v_b = v_b.clone(); let v_beta = v_beta.clone(); let v_g = v_g.clone(); let v_gamma = v_gamma.clone(); let v_i0 = v_i0.clone(); let v_s0 = v_s0.clone(); let v_sp = v_sp.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::truediv(&Py::Int(1i64), &py::mul(&py::mul(&v_b.clone(), &v_s.clone())?, &py::add(&py::sub(&py::add(&v_S0.clone(), &v_I0.clone())?, &v_s.clone())?, &py::truediv(&py::mul(&ax_engine::pymp::call("log", &[py::truediv(&v_s.clone(), &v_S0.clone())?], &[])?, &v_g.clone())?, &v_b.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_pts = py::add(&py::add(&Py::list(vec![v_sp.clone()]), &{ let mut out = vec![]; for t1787 in py::iter(&(Py::tuple(vec![Py::Int(1000i64), Py::Int(100i64), Py::Int(10i64), Py::Int(1i64)])))? { let mut v_m: Py; 
v_m = t1787;
if ({ let t1788 = py::sub(&v_S0.clone(), &py::mul(&v_I0.clone(), &v_m.clone())?)?; Py::Bool({ let t1789 = v_sp.clone(); if py::gt(&t1788, &t1789)? { true } else { false } }) }).truthy() { out.push(py::sub(&v_S0.clone(), &py::mul(&v_I0.clone(), &v_m.clone())?)?); }} Py::list(out) })?, &Py::list(vec![v_S0.clone()]))?;
        return Ok(py::float(&ax_engine::pymp::call("quad", &[v_f.clone(), v_pts.clone()], &[])?)?);
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__solve_lin(mut v_A: Py, mut v_b: Py) -> R {
    let mut v_M: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_f: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_p: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_scale: Py = Py::None;
    let mut v_x: Py = Py::None;
    v_n = py::len(&v_A.clone())?;
    v_M = { let mut out = vec![]; for t6311 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t6312 = (t6311).items()?;
if t6312.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t6312[0].clone();
v_row = t6312[1].clone();
out.push(py::add(&Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_row.clone())?).items()?), &Py::list(vec![py::float(&py::getitem(&v_b.clone(), &v_i.clone())?)?]))?); } Py::list(out) };
    v_scale = { let t6315 = { let t = { let mut out = vec![]; for t6313 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t6313;
for t6314 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t6314;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t6315.truthy() { t6315 } else { Py::Float(1.0) } };
    for t6316 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t6316;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t6317 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t6318 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t6317, &t6318)? { true } else { false } }) }, Py::str("the equations are singular (the data do not determine the parameters)"))?;
        let t6319 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t6319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t6319[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t6319[1].clone())?;
        for t6320 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t6320;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t6321 = v_f.clone(); Py::Bool({ let t6322 = Py::Int(0i64); if py::ne(&t6321, &t6322) { true } else { false } }) }).truthy() {
                for t6323 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t6323;
                    let t6324 = py::getitem(&v_M.clone(), &v_r.clone())?; let t6325 = v_k.clone();
                    py::setitem(&t6324, &t6325, py::sub(&py::getitem(&t6324, &t6325)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t6326 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t6326;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t6327 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t6327;
out.push(py::mul(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_k.clone())?, &py::getitem(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?)?, &py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_r.clone())?)?)?;
    }
    return Ok(v_x.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__surv(mut v_times: Py, mut v_events: Py, mut v_label: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t6388 = { let t6384 = py::len(&v_times.clone())?; Py::Bool({ let t6385 = py::len(&v_events.clone())?; if py::eq(&t6384, &t6385) { true } else { false } }) }; if !t6388.truthy() { t6388 } else { { let t6386 = py::len(&v_times.clone())?; Py::Bool({ let t6387 = Py::Int(1i64); if py::ge(&t6386, &t6387)? { true } else { false } }) } } }, Py::str(&["give one event indicator per time (".to_string(), py::format(&v_label.clone(), "")?, ")".to_string()].concat()))?;
    for t6389 in py::iter(&(v_times.clone()))? {
        v_v = t6389;
        let _ = f__need({ let t6392 = py::math1("isfinite", &v_v.clone())?; if !t6392.truthy() { t6392 } else { { let t6390 = v_v.clone(); Py::Bool({ let t6391 = Py::Int(0i64); if py::ge(&t6390, &t6391)? { true } else { false } }) } } }, Py::str("times must be finite and >= 0"))?;
    }
    for t6393 in py::iter(&(v_events.clone()))? {
        v_e = t6393;
        let _ = f__need({ let t6394 = v_e.clone(); Py::Bool({ let t6395 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if py::contains(&t6395, &t6394)? { true } else { false } }) }, Py::str("event indicators are 1 (event) or 0 (censored)"))?;
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t6396 in py::iter(&(v_times.clone()))? { let mut v_v: Py; 
v_v = t6396;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t6397 in py::iter(&(v_events.clone()))? { let mut v_e: Py; 
v_e = t6397;
out.push(py::int(&v_e.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__table(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t6229 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("a"), v_a.clone()]), Py::tuple(vec![Py::str("b"), v_b.clone()]), Py::tuple(vec![Py::str("c"), v_c.clone()]), Py::tuple(vec![Py::str("d"), v_d.clone()])])))? {
        let t6234 = (t6229).items()?;
        if t6234.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t6234[0].clone();
        v_v = t6234[1].clone();
        let _ = f__count(v_nm.clone(), v_v.clone(), Py::Int(1i64))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trap(mut v_t: Py, mut v_C: Py, mut v_label: Py) -> R {
    let _ = f__need({ let t4503 = { let t4499 = py::len(&v_t.clone())?; Py::Bool({ let t4500 = py::len(&v_C.clone())?; if py::eq(&t4499, &t4500) { true } else { false } }) }; if !t4503.truthy() { t4503 } else { { let t4501 = py::len(&v_t.clone())?; Py::Bool({ let t4502 = Py::Int(2i64); if py::ge(&t4501, &t4502)? { true } else { false } }) } } }, Py::str(&[py::format(&v_label.clone(), "")?, ": give at least two times and as many concentrations".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4504 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4505 = (t4504).items()?;
if t4505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4505[0].clone();
v_b = t4505[1].clone();
out.push({ let t4506 = v_b.clone(); Py::Bool({ let t4507 = v_a.clone(); if py::gt(&t4506, &t4507)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_label.clone(), "")?, ": the times must increase strictly".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4508 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4508;
out.push({ let t4509 = v_c.clone(); Py::Bool({ let t4510 = Py::Int(0i64); if py::ge(&t4509, &t4510)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_label.clone(), "")?, ": concentrations cannot be negative".to_string()].concat()))?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t4511 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_C.clone(), py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_c1: Py; let mut v_c2: Py; let mut v_t1: Py; let mut v_t2: Py; 
let t4512 = (t4511).items()?;
if t4512.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_t1 = t4512[0].clone();
v_t2 = t4512[1].clone();
v_c1 = t4512[2].clone();
v_c2 = t4512[3].clone();
out.push(py::truediv(&py::mul(&py::sub(&v_t2.clone(), &v_t1.clone())?, &py::add(&v_c1.clone(), &v_c2.clone())?)?, &Py::Float(2.0))?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_groups(mut v_pos: Py, mut v_neg: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t917 = { let t913 = py::len(&v_pos.clone())?; Py::Bool({ let t914 = Py::Int(1i64); if py::ge(&t913, &t914)? { true } else { false } }) }; if !t917.truthy() { t917 } else { { let t915 = py::len(&v_neg.clone())?; Py::Bool({ let t916 = Py::Int(1i64); if py::ge(&t915, &t916)? { true } else { false } }) } } }, Py::str("give at least one score in each group"))?;
    for t918 in py::iter(&(py::add(&Py::list((v_pos.clone()).items()?), &Py::list((v_neg.clone()).items()?))?))? {
        v_v = t918;
        let _ = f__fin(Py::str("every score"), Py::tuple(vec![v_v.clone()]))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__w_log(mut v_L: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_w_new: Py = Py::None;
    let t5052 = (Py::tuple(vec![Py::Float(1e-300), py::add(&py::minmax(&[Py::Float(1.0), v_L.clone()], true)?, &Py::Float(1.0))?])).items()?;
    if t5052.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t5052[0].clone();
    v_hi = t5052[1].clone();
    v_w = (if ({ let t5053 = v_L.clone(); Py::Bool({ let t5054 = Py::Int(1i64); if py::lt(&t5053, &t5054)? { true } else { false } }) }).truthy() { py::math1("exp", &v_L.clone())? } else { py::sub(&v_L.clone(), &py::math1("log", &v_L.clone())?)? });
    v_w = py::minmax(&[py::minmax(&[v_w.clone(), v_lo.clone()], true)?, v_hi.clone()], false)?;
    for t5055 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t5055;
        v_h = py::sub(&py::add(&v_w.clone(), &py::math1("log", &v_w.clone())?)?, &v_L.clone())?;
        if ({ let t5056 = v_h.clone(); Py::Bool({ let t5057 = Py::Int(0i64); if py::gt(&t5056, &t5057)? { true } else { false } }) }).truthy() {
            v_hi = v_w.clone();
        } else {
            v_lo = v_w.clone();
        }
        v_w_new = py::sub(&v_w.clone(), &py::truediv(&v_h.clone(), &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_w.clone())?)?)?)?;
        if (py::not(&{ let t5058 = v_lo.clone(); Py::Bool({ let t5059 = v_w_new.clone(); if py::lt(&t5058, &t5059)? { { let t5060 = v_hi.clone(); if py::lt(&t5059, &t5060)? { true } else { false } } } else { false } }) })).truthy() {
            v_w_new = (if ({ let t5061 = v_lo.clone(); Py::Bool({ let t5062 = Py::Int(0i64); if py::gt(&t5061, &t5062)? { true } else { false } }) }).truthy() { py::math1("sqrt", &py::mul(&v_lo.clone(), &v_hi.clone())?)? } else { py::mul(&Py::Float(0.5), &v_hi.clone())? });
        }
        if ({ let t5063 = py::abs(&py::sub(&v_w_new.clone(), &v_w.clone())?)?; Py::Bool({ let t5064 = py::mul(&Py::Float(1e-16), &v_w.clone())?; if py::le(&t5063, &t5064)? { true } else { false } }) }).truthy() {
            return Ok(v_w_new.clone());
        }
        v_w = v_w_new.clone();
    }
    return Ok(v_w.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__wilson(mut v_x: Py, mut v_n: Py, mut v_z: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_p: Py = Py::None;
    v_p = py::truediv(&v_x.clone(), &v_n.clone())?;
    v_den = py::add(&Py::Float(1.0), &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &v_n.clone())?)?;
    v_c = py::truediv(&py::add(&v_p.clone(), &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &py::mul(&Py::Int(2i64), &v_n.clone())?)?)?, &v_den.clone())?;
    v_h = py::truediv(&py::mul(&v_z.clone(), &py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_p.clone(), &py::sub(&Py::Int(1i64), &v_p.clone())?)?, &v_n.clone())?, &py::truediv(&py::mul(&v_z.clone(), &v_z.clone())?, &py::mul(&py::mul(&Py::Int(4i64), &v_n.clone())?, &v_n.clone())?)?)?)?)?, &v_den.clone())?;
    return Ok(Py::tuple(vec![py::sub(&v_c.clone(), &v_h.clone())?, py::add(&v_c.clone(), &v_h.clone())?]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__youden(mut v_scores_pos: Py, mut v_scores_neg: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_best: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_thr: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let t922 = (Py::tuple(vec![py::len(&v_scores_pos.clone())?, py::len(&v_scores_neg.clone())?])).items()?;
    if t922.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t922[0].clone();
    v_n0 = t922[1].clone();
    v_best = Py::None;
    for t923 in py::iter(&(py::sorted(&py::bitor(&py::set(&v_scores_pos.clone())?, &py::set(&v_scores_neg.clone())?)?, (Py::Bool(true)).truthy())?))? {
        v_thr = t923;
        v_tp = py::sum(&{ let mut out = vec![]; for t924 in py::iter(&(v_scores_pos.clone()))? { let mut v_v: Py; 
v_v = t924;
if ({ let t925 = v_v.clone(); Py::Bool({ let t926 = v_thr.clone(); if py::ge(&t925, &t926)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_fp = py::sum(&{ let mut out = vec![]; for t927 in py::iter(&(v_scores_neg.clone()))? { let mut v_v: Py; 
v_v = t927;
if ({ let t928 = v_v.clone(); Py::Bool({ let t929 = v_thr.clone(); if py::ge(&t928, &t929)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_J = py::sub(&py::fraction(&[v_tp.clone(), v_n1.clone()])?, &py::fraction(&[v_fp.clone(), v_n0.clone()])?)?;
        if ({ let t934 = { let t930 = v_best.clone(); Py::Bool({ let t931 = Py::None; if py::is(&t930, &t931) { true } else { false } }) }; if t934.truthy() { t934 } else { { let t932 = v_J.clone(); Py::Bool({ let t933 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t932, &t933)? { true } else { false } }) } } }).truthy() {
            v_best = Py::tuple(vec![v_J.clone(), v_thr.clone(), v_tp.clone(), v_fp.clone()]);
        }
    }
    return Ok(Py::tuple(vec![v_best.clone(), v_n1.clone(), v_n0.clone()]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_indicator_dilution(mut v_m: Py, mut v_t: Py, mut v_C: Py) -> R {
    let mut v_auc: Py = Py::None;
    let _ = f__pos(Py::str("the indicator mass m"), Py::tuple(vec![v_m.clone()]))?;
    v_auc = f__trap(v_t.clone(), v_C.clone(), Py::str("the indicator curve"))?;
    let _ = f__need({ let t2844 = v_auc.clone(); Py::Bool({ let t2845 = Py::Int(0i64); if py::gt(&t2844, &t2845)? { true } else { false } }) }, Py::str("the indicator curve has no area"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("AUC"), v_auc.clone()), (Py::str("CO"), py::truediv(&v_m.clone(), &py::mul(&v_auc.clone(), &Py::Float(0.001))?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_kelvin_voigt(mut v_sigma: Py, mut v_E: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("sigma, E and eta"), Py::tuple(vec![v_sigma.clone(), v_E.clone(), v_eta.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("strain"), py::mul(&py::truediv(&v_sigma.clone(), &v_E.clone())?, &py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_E.clone())?, &v_t.clone())?, &v_eta.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_lever(mut v_W: Py, mut v_d_load: Py, mut v_W_arm: Py, mut v_d_arm: Py, mut v_d_muscle: Py, mut v_theta: Py) -> R {
    let mut v_F: Py = Py::None;
    let mut v_Rx: Py = Py::None;
    let mut v_Ry: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__nonneg(Py::str("the weights W and W_arm"), Py::tuple(vec![v_W.clone(), v_W_arm.clone()]))?;
    let _ = f__pos(Py::str("the distances d_load, d_arm and d_muscle"), Py::tuple(vec![v_d_load.clone(), v_d_arm.clone(), v_d_muscle.clone()]))?;
    let _ = f__need({ let t1004 = py::add(&v_W.clone(), &v_W_arm.clone())?; Py::Bool({ let t1005 = Py::Int(0i64); if py::gt(&t1004, &t1005)? { true } else { false } }) }, Py::str("there is no load"))?;
    let _ = f__need({ let t1006 = Py::Int(0i64); Py::Bool({ let t1007 = v_theta.clone(); if py::lt(&t1006, &t1007)? { { let t1008 = Py::Float(std::f64::consts::PI); if py::lt(&t1007, &t1008)? { true } else { false } } } else { false } }) }, Py::str("the muscle angle must lie between 0 and 180 degrees"))?;
    let t1009 = (Py::tuple(vec![py::math1("sin", &v_theta.clone())?, py::math1("cos", &v_theta.clone())?])).items()?;
    if t1009.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1009[0].clone();
    v_c = t1009[1].clone();
    v_F = py::truediv(&py::add(&py::mul(&v_W.clone(), &v_d_load.clone())?, &py::mul(&v_W_arm.clone(), &v_d_arm.clone())?)?, &py::mul(&v_d_muscle.clone(), &v_s.clone())?)?;
    let t1010 = (Py::tuple(vec![py::mul(&v_F.clone(), &v_c.clone())?, py::sub(&py::add(&v_W.clone(), &v_W_arm.clone())?, &py::mul(&v_F.clone(), &v_s.clone())?)?])).items()?;
    if t1010.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rx = t1010[0].clone();
    v_Ry = t1010[1].clone();
    if ({ let t1011 = py::abs(&v_Rx.clone())?; Py::Bool({ let t1012 = py::mul(&Py::Float(1e-12), &v_F.clone())?; if py::le(&t1011, &t1012)? { true } else { false } }) }).truthy() {
        v_Rx = Py::Float(0.0);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("F_muscle"), v_F.clone()), (Py::str("R_x"), v_Rx.clone()), (Py::str("R_y"), v_Ry.clone()), (Py::str("R"), py::hypot(&[v_Rx.clone(), v_Ry.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_poiseuille(mut v_dP: Py, mut v_r: Py, mut v_L: Py, mut v_mu: Py) -> R {
    let mut v_Q: Py = Py::None;
    let _ = f__pos(Py::str("dP, r, L and mu"), Py::tuple(vec![v_dP.clone(), v_r.clone(), v_L.clone(), v_mu.clone()]))?;
    v_Q = py::truediv(&py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_r.clone(), &Py::Int(4i64))?)?, &v_dP.clone())?, &py::mul(&py::mul(&Py::Float(8.0), &v_mu.clone())?, &v_L.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Q"), v_Q.clone()), (Py::str("v_mean"), py::truediv(&v_Q.clone(), &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_r.clone())?, &v_r.clone())?)?), (Py::str("tau_wall"), py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_mu.clone())?, &v_Q.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_r.clone(), &Py::Int(3i64))?)?)?), (Py::str("R_hyd"), py::truediv(&py::mul(&py::mul(&Py::Float(8.0), &v_mu.clone())?, &v_L.clone())?, &py::mul(&Py::Float(std::f64::consts::PI), &py::pow(&v_r.clone(), &Py::Int(4i64))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_respiratory_fit(mut v_P: Py, mut v_Q: Py, mut v_V: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_P0: Py = Py::None;
    let mut v_Rr: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_n = py::len(&v_P.clone())?;
    let _ = f__need({ let t6301 = { let t6294 = py::len(&v_Q.clone())?; Py::Bool({ let t6295 = v_n.clone(); if py::eq(&t6294, &t6295) { true } else { false } }) }; if !t6301.truthy() { t6301 } else { { let t6300 = { let t6296 = py::len(&v_V.clone())?; Py::Bool({ let t6297 = v_n.clone(); if py::eq(&t6296, &t6297) { true } else { false } }) }; if !t6300.truthy() { t6300 } else { { let t6298 = v_n.clone(); Py::Bool({ let t6299 = Py::Int(4i64); if py::ge(&t6298, &t6299)? { true } else { false } }) } } } } }, Py::str("give at least four samples of P, Q and V"))?;
    for t6302 in py::iter(&(py::add(&py::add(&Py::list((v_P.clone()).items()?), &Py::list((v_Q.clone()).items()?))?, &Py::list((v_V.clone()).items()?))?))? {
        v_v = t6302;
        let _ = f__fin(Py::str("every sample"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t6328 = (f__ols(Py::list(vec![Py::list((v_Q.clone()).items()?), Py::list((v_V.clone()).items()?), py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?]), Py::list((v_P.clone()).items()?))?).items()?;
    if t6328.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rr = t6328[0].clone();
    v_E = t6328[1].clone();
    v_P0 = t6328[2].clone();
    let _ = f__need({ let t6329 = v_E.clone(); Py::Bool({ let t6330 = Py::Int(0i64); if py::gt(&t6329, &t6330)? { true } else { false } }) }, Py::str("the fitted elastance is not positive"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), v_Rr.clone()), (Py::str("C"), py::truediv(&Py::Float(1.0), &v_E.clone())?), (Py::str("P0"), v_P0.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_sls_creep(mut v_sigma: Py, mut v_E_inf: Py, mut v_E1: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let mut v_e0: Py = Py::None;
    let mut v_e_inf: Py = Py::None;
    let mut v_tc: Py = Py::None;
    let _ = f__pos(Py::str("sigma, E_inf, E1 and eta"), Py::tuple(vec![v_sigma.clone(), v_E_inf.clone(), v_E1.clone(), v_eta.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_tc = py::truediv(&py::mul(&v_eta.clone(), &py::add(&v_E_inf.clone(), &v_E1.clone())?)?, &py::mul(&v_E_inf.clone(), &v_E1.clone())?)?;
    let t6490 = (Py::tuple(vec![py::truediv(&v_sigma.clone(), &v_E_inf.clone())?, py::truediv(&v_sigma.clone(), &py::add(&v_E_inf.clone(), &v_E1.clone())?)?])).items()?;
    if t6490.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e_inf = t6490[0].clone();
    v_e0 = t6490[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("strain"), py::sub(&v_e_inf.clone(), &py::mul(&py::sub(&v_e_inf.clone(), &v_e0.clone())?, &py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &v_tc.clone())?)?)?)?), (Py::str("strain_inf"), v_e_inf.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_sls_relaxation(mut v_eps0: Py, mut v_E_inf: Py, mut v_E1: Py, mut v_eta: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("eps0, E_inf, E1 and eta"), Py::tuple(vec![v_eps0.clone(), v_E_inf.clone(), v_E1.clone(), v_eta.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("stress"), py::mul(&v_eps0.clone(), &py::add(&v_E_inf.clone(), &py::mul(&v_E1.clone(), &py::math1("exp", &py::truediv(&py::mul(&py::neg(&v_E1.clone())?, &v_t.clone())?, &v_eta.clone())?)?)?)?)?), (Py::str("stress_inf"), py::mul(&v_eps0.clone(), &v_E_inf.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_windkessel_decay(mut v_P0: Py, mut v_Pv: Py, mut v_R: Py, mut v_C: Py, mut v_t: Py) -> R {
    let _ = f__pos(Py::str("P0, R and C"), Py::tuple(vec![v_P0.clone(), v_R.clone(), v_C.clone()]))?;
    let _ = f__nonneg(Py::str("the venous pressure Pv and the time t"), Py::tuple(vec![v_Pv.clone(), v_t.clone()]))?;
    let _ = f__need({ let t7040 = v_Pv.clone(); Py::Bool({ let t7041 = v_P0.clone(); if py::lt(&t7040, &t7041)? { true } else { false } }) }, Py::str("the starting pressure must exceed the venous pressure"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("P"), py::add(&v_Pv.clone(), &py::mul(&py::sub(&v_P0.clone(), &v_Pv.clone())?, &py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &py::mul(&v_R.clone(), &v_C.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bm_windkessel_impedance(mut v_model: Py, mut v_R: Py, mut v_C: Py, mut v_f: Py, mut v_Zc: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_w: Py = Py::None;
    let mut v_zi: Py = Py::None;
    let mut v_zr: Py = Py::None;
    let _ = f__pos(Py::str("R, C and f"), Py::tuple(vec![v_R.clone(), v_C.clone(), v_f.clone()]))?;
    let _ = f__need({ let t7101 = v_model.clone(); Py::Bool({ let t7102 = Py::tuple(vec![Py::str("two_element"), Py::str("three_element")]); if py::contains(&t7102, &t7101)? { true } else { false } }) }, Py::str("model is two_element or three_element"))?;
    if ({ let t7103 = v_model.clone(); Py::Bool({ let t7104 = Py::str("three_element"); if py::eq(&t7103, &t7104) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t7105 = v_Zc.clone(); Py::Bool({ let t7106 = Py::None; if !py::is(&t7105, &t7106) { true } else { false } }) }, Py::str("the three-element model needs the characteristic impedance Zc"))?;
        let _ = f__pos(Py::str("Zc"), Py::tuple(vec![v_Zc.clone()]))?;
    } else {
        let _ = f__need({ let t7107 = v_Zc.clone(); Py::Bool({ let t7108 = Py::None; if py::is(&t7107, &t7108) { true } else { false } }) }, Py::str("Zc belongs to the three-element model, not the two-element model"))?;
    }
    v_w = py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_den = py::add(&Py::Float(1.0), &py::pow(&py::mul(&py::mul(&v_w.clone(), &v_R.clone())?, &v_C.clone())?, &Py::Int(2i64))?)?;
    v_zr = py::add(&py::truediv(&v_R.clone(), &v_den.clone())?, &{ let t7109 = v_Zc.clone(); if t7109.truthy() { t7109 } else { Py::Float(0.0) } })?;
    v_zi = py::truediv(&py::mul(&py::mul(&py::mul(&py::neg(&v_w.clone())?, &v_R.clone())?, &v_R.clone())?, &v_C.clone())?, &v_den.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("Z_mod"), py::hypot(&[v_zr.clone(), v_zi.clone()])?), (Py::str("phase_deg"), py::math1("degrees", &py::atan2(&v_zi.clone(), &v_zr.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmi(mut v_m: Py, mut v_h: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the mass and the height"), Py::tuple(vec![v_m.clone(), v_h.clone()]))?;
    let _ = f__need({ let t13 = { let t7 = Py::Float(0.3); Py::Bool({ let t8 = v_h.clone(); if py::le(&t7, &t8)? { { let t9 = Py::Float(2.8); if py::le(&t8, &t9)? { true } else { false } } } else { false } }) }; if !t13.truthy() { t13 } else { { let t10 = Py::Int(1i64); Py::Bool({ let t11 = v_m.clone(); if py::le(&t10, &t11)? { { let t12 = Py::Int(700i64); if py::le(&t11, &t12)? { true } else { false } } } else { false } }) } } }, Py::str("the height must lie between 0.3 and 2.8 m and the mass between 1 and 700 kg"))?;
    v_v = py::truediv(&v_m.clone(), &py::mul(&v_h.clone(), &v_h.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("BMI"), v_v.clone()), (Py::str("BMI_prime"), py::truediv(&v_v.clone(), &Py::Float(25.0))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_bmr(mut v_m: Py, mut v_h: Py, mut v_age: Py, mut v_sex: Py, mut v_equation: Py) -> R {
    let mut v_hc: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the mass and the height"), Py::tuple(vec![v_m.clone(), v_h.clone()]))?;
    let _ = f__need({ let t79 = { let t73 = Py::Float(0.5); Py::Bool({ let t74 = v_h.clone(); if py::le(&t73, &t74)? { { let t75 = Py::Float(2.8); if py::le(&t74, &t75)? { true } else { false } } } else { false } }) }; if !t79.truthy() { t79 } else { { let t76 = Py::Int(10i64); Py::Bool({ let t77 = v_m.clone(); if py::le(&t76, &t77)? { { let t78 = Py::Int(400i64); if py::le(&t77, &t78)? { true } else { false } } } else { false } }) } } }, Py::str("the height must lie between 0.5 and 2.8 m and the mass between 10 and 400 kg"))?;
    let _ = f__need({ let t85 = f__isnum(v_age.clone())?; if !t85.truthy() { t85 } else { { let t82 = Py::Int(18i64); Py::Bool({ let t83 = v_age.clone(); if py::le(&t82, &t83)? { { let t84 = Py::Int(120i64); if py::le(&t83, &t84)? { true } else { false } } } else { false } }) } } }, Py::str("the equations are stated for adults: age 18 to 120 years"))?;
    let _ = f__need({ let t86 = v_sex.clone(); Py::Bool({ let t87 = Py::tuple(vec![Py::str("male"), Py::str("female")]); if py::contains(&t87, &t86)? { true } else { false } }) }, Py::str("sex must be male or female (the equations' two forms)"))?;
    let _ = f__need({ let t88 = v_equation.clone(); Py::Bool({ let t89 = c__BMR_EQ()?; if py::contains(&t89, &t88)? { true } else { false } }) }, Py::str("equation must be mifflin_st_jeor or harris_benedict_revised"))?;
    v_hc = py::mul(&v_h.clone(), &Py::Float(100.0))?;
    if ({ let t90 = v_equation.clone(); Py::Bool({ let t91 = Py::str("mifflin_st_jeor"); if py::eq(&t90, &t91) { true } else { false } }) }).truthy() {
        v_k = py::add(&py::sub(&py::add(&py::mul(&Py::Float(10.0), &v_m.clone())?, &py::mul(&Py::Float(6.25), &v_hc.clone())?)?, &py::mul(&Py::Float(5.0), &v_age.clone())?)?, &(if ({ let t92 = v_sex.clone(); Py::Bool({ let t93 = Py::str("male"); if py::eq(&t92, &t93) { true } else { false } }) }).truthy() { Py::Float(5.0) } else { py::neg(&Py::Float(161.0))? }))?;
    } else {
        if ({ let t94 = v_sex.clone(); Py::Bool({ let t95 = Py::str("male"); if py::eq(&t94, &t95) { true } else { false } }) }).truthy() {
            v_k = py::sub(&py::add(&py::add(&Py::Float(88.362), &py::mul(&Py::Float(13.397), &v_m.clone())?)?, &py::mul(&Py::Float(4.799), &v_hc.clone())?)?, &py::mul(&Py::Float(5.677), &v_age.clone())?)?;
        } else {
            v_k = py::sub(&py::add(&py::add(&Py::Float(447.593), &py::mul(&Py::Float(9.247), &v_m.clone())?)?, &py::mul(&Py::Float(3.098), &v_hc.clone())?)?, &py::mul(&Py::Float(4.33), &v_age.clone())?)?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("BMR_kcal_day"), v_k.clone()), (Py::str("BMR_W"), py::mul(&v_k.clone(), &c_KCAL_DAY_W()?)?), (Py::str("BMR_MJ_day"), py::mul(&v_k.clone(), &Py::Float(0.004184))?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dose_liquid_volume(mut v_D: Py, mut v_H: Py, mut v_Q: Py) -> R {
    let mut v_V: Py = Py::None;
    let _ = f__pos(Py::str("the ordered dose D, the stock amount H and the stock volume Q"), Py::tuple(vec![v_D.clone(), v_H.clone(), v_Q.clone()]))?;
    v_V = py::mul(&py::truediv(&v_D.clone(), &v_H.clone())?, &v_Q.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), v_V.clone()), (Py::str("V_mL"), py::mul(&v_V.clone(), &Py::Float(1000000.0))?), (Py::str("C"), py::truediv(&v_H.clone(), &v_Q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dose_tablets(mut v_D: Py, mut v_H: Py) -> R {
    let _ = f__pos(Py::str("the ordered dose D and the tablet strength H"), Py::tuple(vec![v_D.clone(), v_H.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("tablets"), py::truediv(&v_D.clone(), &v_H.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dose_weight_based(mut v_dose_mg_per_kg: Py, mut v_W: Py, mut v_basis: Py, mut v_doses_per_day: Py) -> R {
    let mut v_mg: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the dose per kilogram"), Py::tuple(vec![v_dose_mg_per_kg.clone()]))?;
    let _ = f__need({ let t556 = v_dose_mg_per_kg.clone(); Py::Bool({ let t557 = Py::Float(1000.0); if py::le(&t556, &t557)? { true } else { false } }) }, Py::str("the dose per kilogram is a number in mg/kg and must be at most 1000"))?;
    let _ = f__pos(Py::str("the body weight"), Py::tuple(vec![v_W.clone()]))?;
    let _ = f__need({ let t558 = Py::Float(0.3); Py::Bool({ let t559 = v_W.clone(); if py::le(&t558, &t559)? { { let t560 = Py::Float(400.0); if py::le(&t559, &t560)? { true } else { false } } } else { false } }) }, Py::str("the body weight must lie between 0.3 and 400 kg"))?;
    let _ = f__need({ let t565 = { let t561 = v_basis.clone(); Py::Bool({ let t562 = Py::str("per_dose"); if py::eq(&t561, &t562) { true } else { false } }) }; if t565.truthy() { t565 } else { { let t563 = v_basis.clone(); Py::Bool({ let t564 = Py::str("per_day"); if py::eq(&t563, &t564) { true } else { false } }) } } }, Py::str("basis must be per_dose or per_day (what the mg/kg figure refers to)"))?;
    if ({ let t566 = v_doses_per_day.clone(); Py::Bool({ let t567 = Py::None; if !py::is(&t566, &t567) { true } else { false } }) }).truthy() {
        let _ = f__count(Py::str("the number of doses per day"), v_doses_per_day.clone(), Py::Int(1i64))?;
        let _ = f__need({ let t572 = v_doses_per_day.clone(); Py::Bool({ let t573 = Py::Int(24i64); if py::le(&t572, &t573)? { true } else { false } }) }, Py::str("at most 24 doses per day"))?;
    }
    let _ = f__need({ let t578 = { let t574 = v_basis.clone(); Py::Bool({ let t575 = Py::str("per_dose"); if py::eq(&t574, &t575) { true } else { false } }) }; if t578.truthy() { t578 } else { { let t576 = v_doses_per_day.clone(); Py::Bool({ let t577 = Py::None; if !py::is(&t576, &t577) { true } else { false } }) } } }, Py::str("a dose per kilogram per day needs the number of doses per day"))?;
    v_mg = py::mul(&v_dose_mg_per_kg.clone(), &v_W.clone())?;
    v_out = Py::dict(vec![]);
    if ({ let t579 = v_basis.clone(); Py::Bool({ let t580 = Py::str("per_day"); if py::eq(&t579, &t580) { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("dose_mg"), py::truediv(&v_mg.clone(), &v_doses_per_day.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("daily_dose_mg"), v_mg.clone())?;
    } else {
        py::setitem(&v_out.clone(), &Py::str("dose_mg"), v_mg.clone())?;
        if ({ let t581 = v_doses_per_day.clone(); Py::Bool({ let t582 = Py::None; if !py::is(&t581, &t582) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("daily_dose_mg"), py::mul(&v_mg.clone(), &v_doses_per_day.clone())?)?;
        }
    }
    py::setitem(&v_out.clone(), &Py::str("dose"), py::mul(&py::getitem(&v_out.clone(), &Py::str("dose_mg"))?, &Py::Float(1e-06))?)?;
    if ({ let t583 = Py::str("daily_dose_mg"); Py::Bool({ let t584 = v_out.clone(); if py::contains(&t584, &t583)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("daily_dose"), py::mul(&py::getitem(&v_out.clone(), &Py::str("daily_dose_mg"))?, &Py::Float(1e-06))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_irr(mut v_events1: Py, mut v_time1: Py, mut v_events2: Py, mut v_time2: Py, mut v_level: Py) -> R {
    let mut v_irr: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__count(Py::str("events1"), v_events1.clone(), Py::Int(1i64))?;
    let _ = f__count(Py::str("events2"), v_events2.clone(), Py::Int(1i64))?;
    let _ = f__pos(Py::str("the person-times"), Py::tuple(vec![v_time1.clone(), v_time2.clone()]))?;
    v_z = f__level(v_level.clone())?;
    v_irr = py::truediv(&py::truediv(&v_events1.clone(), &v_time1.clone())?, &py::truediv(&v_events2.clone(), &v_time2.clone())?)?;
    v_se = py::math1("sqrt", &py::add(&py::truediv(&Py::Int(1i64), &v_events1.clone())?, &py::truediv(&Py::Int(1i64), &v_events2.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("IRR"), v_irr.clone()), (Py::str("lower"), py::mul(&v_irr.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?), (Py::str("upper"), py::mul(&v_irr.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_kappa(mut v_counts: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_pe: Py = Py::None;
    let mut v_po: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_v: Py = Py::None;
    v_k = py::len(&v_counts.clone())?;
    let _ = f__need({ let t236 = { let t230 = Py::Int(2i64); Py::Bool({ let t231 = v_k.clone(); if py::le(&t230, &t231)? { { let t232 = Py::Int(8i64); if py::le(&t231, &t232)? { true } else { false } } } else { false } }) }; if !t236.truthy() { t236 } else { Py::Bool(({ let mut out = vec![]; for t233 in py::iter(&(v_counts.clone()))? { let mut v_r: Py; 
v_r = t233;
out.push({ let t234 = py::len(&v_r.clone())?; Py::Bool({ let t235 = v_k.clone(); if py::eq(&t234, &t235) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give a square k x k table of counts (2 to 8 categories)"))?;
    for t237 in py::iter(&(v_counts.clone()))? {
        v_r = t237;
        for t238 in py::iter(&(v_r.clone()))? {
            v_v = t238;
            let _ = f__need({ let t243 = { let t239 = v_v.clone(); Py::Bool({ let t240 = Py::Int(0i64); if py::ge(&t239, &t240)? { true } else { false } }) }; if !t243.truthy() { t243 } else { { let t241 = py::float(&v_v.clone())?; Py::Bool({ let t242 = py::int(&v_v.clone())?; if py::eq(&t241, &t242) { true } else { false } }) } } }, Py::str("counts are whole numbers >= 0"))?;
        }
    }
    v_n = py::sum(&{ let mut out = vec![]; for t244 in py::iter(&(v_counts.clone()))? { let mut v_r: Py; 
v_r = t244;
out.push(py::sum(&v_r.clone(), None)?); } Py::list(out) }, None)?;
    let _ = f__need({ let t245 = v_n.clone(); Py::Bool({ let t246 = Py::Int(0i64); if py::gt(&t245, &t246)? { true } else { false } }) }, Py::str("the table is empty"))?;
    v_po = py::truediv(&py::sum(&{ let mut out = vec![]; for t247 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t247;
out.push(py::getitem(&py::getitem(&v_counts.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?, &v_n.clone())?;
    v_pe = py::truediv(&py::fsum(&{ let mut out = vec![]; for t248 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_i: Py; 
v_i = t248;
out.push(py::mul(&py::sum(&py::getitem(&v_counts.clone(), &v_i.clone())?, None)?, &py::sum(&{ let mut out = vec![]; for t249 in py::range_iter(&[(v_k.clone()).i()?]) { let mut v_j: Py; 
v_j = t249;
out.push(py::getitem(&py::getitem(&v_counts.clone(), &v_j.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?); } Py::list(out) })?, &py::mul(&v_n.clone(), &v_n.clone())?)?;
    let _ = f__need({ let t250 = v_pe.clone(); Py::Bool({ let t251 = Py::Int(1i64); if py::lt(&t250, &t251)? { true } else { false } }) }, Py::str("chance agreement is 1: kappa is undefined"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("kappa"), py::truediv(&py::sub(&v_po.clone(), &v_pe.clone())?, &py::sub(&Py::Int(1i64), &v_pe.clone())?)?), (Py::str("p_observed"), v_po.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_mantel_haenszel(mut v_tables: Py, mut v_level: Py) -> R {
    let mut v_Pi: Py = Py::None;
    let mut v_Qi: Py = Py::None;
    let mut v_R: Py = Py::None;
    let mut v_Ri: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_Si: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_c: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_orr: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_sPR: Py = Py::None;
    let mut v_sPSQR: Py = Py::None;
    let mut v_sQS: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_var: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__need({ let t3640 = { let t3635 = py::len(&v_tables.clone())?; Py::Bool({ let t3636 = Py::Int(1i64); if py::ge(&t3635, &t3636)? { true } else { false } }) }; if !t3640.truthy() { t3640 } else { Py::Bool(({ let mut out = vec![]; for t3637 in py::iter(&(v_tables.clone()))? { let mut v_r: Py; 
v_r = t3637;
out.push({ let t3638 = py::len(&v_r.clone())?; Py::Bool({ let t3639 = Py::Int(4i64); if py::eq(&t3638, &t3639) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give one row [a, b, c, d] per stratum"))?;
    for t3641 in py::iter(&(v_tables.clone()))? {
        v_r = t3641;
        for t3642 in py::iter(&(v_r.clone()))? {
            v_v = t3642;
            let _ = f__need({ let t3647 = { let t3643 = v_v.clone(); Py::Bool({ let t3644 = Py::Int(0i64); if py::ge(&t3643, &t3644)? { true } else { false } }) }; if !t3647.truthy() { t3647 } else { { let t3645 = py::float(&v_v.clone())?; Py::Bool({ let t3646 = py::int(&v_v.clone())?; if py::eq(&t3645, &t3646) { true } else { false } }) } } }, Py::str("cell counts are whole numbers >= 0"))?;
        }
        let _ = f__need({ let t3648 = py::sum(&v_r.clone(), None)?; Py::Bool({ let t3649 = Py::Int(0i64); if py::gt(&t3648, &t3649)? { true } else { false } }) }, Py::str("a stratum is empty"))?;
    }
    v_z = f__level(v_level.clone())?;
    let t3656 = Py::Float(0.0);
    v_R = t3656.clone();
    v_S = t3656.clone();
    v_sPR = t3656.clone();
    v_sPSQR = t3656.clone();
    v_sQS = t3656.clone();
    v_parts = Py::list(vec![]);
    for t3657 in py::iter(&(v_tables.clone()))? {
        let t3660 = (t3657).items()?;
        if t3660.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3660[0].clone();
        v_b = t3660[1].clone();
        v_c = t3660[2].clone();
        v_d = t3660[3].clone();
        v_n = py::add(&py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &v_d.clone())?;
        let t3658 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_d.clone())?, &v_n.clone())?, py::truediv(&py::mul(&v_b.clone(), &v_c.clone())?, &v_n.clone())?])).items()?;
        if t3658.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Ri = t3658[0].clone();
        v_Si = t3658[1].clone();
        let t3659 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_d.clone())?, &v_n.clone())?, py::truediv(&py::add(&v_b.clone(), &v_c.clone())?, &v_n.clone())?])).items()?;
        if t3659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Pi = t3659[0].clone();
        v_Qi = t3659[1].clone();
        let _ = py::method(&v_parts.clone(), "append", &[Py::tuple(vec![v_Ri.clone(), v_Si.clone(), v_Pi.clone(), v_Qi.clone()])])?;
    }
    v_R = py::fsum(&{ let mut out = vec![]; for t3661 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3661;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) })?;
    v_S = py::fsum(&{ let mut out = vec![]; for t3662 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3662;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) })?;
    let _ = f__need({ let t3667 = { let t3663 = v_R.clone(); Py::Bool({ let t3664 = Py::Int(0i64); if py::gt(&t3663, &t3664)? { true } else { false } }) }; if !t3667.truthy() { t3667 } else { { let t3665 = v_S.clone(); Py::Bool({ let t3666 = Py::Int(0i64); if py::gt(&t3665, &t3666)? { true } else { false } }) } } }, Py::str("the pooled odds ratio is zero or infinite"))?;
    v_var = py::add(&py::add(&py::truediv(&py::fsum(&{ let mut out = vec![]; for t3668 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3668;
out.push(py::mul(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?); } Py::list(out) })?, &py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_R.clone())?)?, &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3669 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3669;
out.push(py::add(&py::mul(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_p.clone(), &Py::Int(3i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?); } Py::list(out) })?, &py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_S.clone())?)?)?, &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3670 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3670;
out.push(py::mul(&py::getitem(&v_p.clone(), &Py::Int(3i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?); } Py::list(out) })?, &py::mul(&py::mul(&Py::Int(2i64), &v_S.clone())?, &v_S.clone())?)?)?;
    v_orr = py::truediv(&v_R.clone(), &v_S.clone())?;
    v_se = py::math1("sqrt", &v_var.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("OR_MH"), v_orr.clone()), (Py::str("lower"), py::mul(&v_orr.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?), (Py::str("upper"), py::mul(&v_orr.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_nnt(mut v_events_t: Py, mut v_n_t: Py, mut v_events_c: Py, mut v_n_c: Py, mut v_level: Py) -> R {
    let mut v_arr: Py = Py::None;
    let mut v_hi: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_pc: Py = Py::None;
    let mut v_pt: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__count(Py::str("events_t"), v_events_t.clone(), Py::Int(0i64))?;
    let _ = f__count(Py::str("events_c"), v_events_c.clone(), Py::Int(0i64))?;
    let _ = f__count(Py::str("n_t"), v_n_t.clone(), Py::Int(1i64))?;
    let _ = f__count(Py::str("n_c"), v_n_c.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t4222 = { let t4218 = v_events_t.clone(); Py::Bool({ let t4219 = v_n_t.clone(); if py::le(&t4218, &t4219)? { true } else { false } }) }; if !t4222.truthy() { t4222 } else { { let t4220 = v_events_c.clone(); Py::Bool({ let t4221 = v_n_c.clone(); if py::le(&t4220, &t4221)? { true } else { false } }) } } }, Py::str("events cannot exceed the group size"))?;
    v_z = f__level(v_level.clone())?;
    let t4229 = (Py::tuple(vec![py::truediv(&v_events_t.clone(), &v_n_t.clone())?, py::truediv(&v_events_c.clone(), &v_n_c.clone())?])).items()?;
    if t4229.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pt = t4229[0].clone();
    v_pc = t4229[1].clone();
    v_arr = py::sub(&v_pc.clone(), &v_pt.clone())?;
    v_se = py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_pt.clone(), &py::sub(&Py::Int(1i64), &v_pt.clone())?)?, &v_n_t.clone())?, &py::truediv(&py::mul(&v_pc.clone(), &py::sub(&Py::Int(1i64), &v_pc.clone())?)?, &v_n_c.clone())?)?)?;
    let t4230 = (Py::tuple(vec![py::sub(&v_arr.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?, py::add(&v_arr.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?])).items()?;
    if t4230.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4230[0].clone();
    v_hi = t4230[1].clone();
    let _ = f__need({ let t4231 = v_lo.clone(); Py::Bool({ let t4232 = Py::Int(0i64); if py::gt(&t4231, &t4232)? { true } else { false } }) }, Py::str("the interval for the risk reduction does not exclude zero (or shows harm): the NNT interval is not finite"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("ARR"), v_arr.clone()), (Py::str("ARR_lower"), v_lo.clone()), (Py::str("ARR_upper"), v_hi.clone()), (Py::str("NNT"), py::truediv(&Py::Int(1i64), &v_arr.clone())?), (Py::str("NNT_lower"), py::truediv(&Py::Int(1i64), &v_hi.clone())?), (Py::str("NNT_upper"), py::truediv(&Py::Int(1i64), &v_lo.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_odds_ratio(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_level: Py) -> R {
    let mut v_orr: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__table(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone())?;
    v_z = f__level(v_level.clone())?;
    v_orr = py::truediv(&py::mul(&v_a.clone(), &v_d.clone())?, &py::mul(&v_b.clone(), &v_c.clone())?)?;
    v_se = py::math1("sqrt", &py::add(&py::add(&py::add(&py::truediv(&Py::Int(1i64), &v_a.clone())?, &py::truediv(&Py::Int(1i64), &v_b.clone())?)?, &py::truediv(&Py::Int(1i64), &v_c.clone())?)?, &py::truediv(&Py::Int(1i64), &v_d.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("OR"), v_orr.clone()), (Py::str("lower"), py::mul(&v_orr.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?), (Py::str("upper"), py::mul(&v_orr.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_predictive_values(mut v_sensitivity: Py, mut v_specificity: Py, mut v_prevalence: Py) -> R {
    let mut v_fn: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_lrn: Py = Py::None;
    let mut v_lrp: Py = Py::None;
    let mut v_tn: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__frac(Py::str("the sensitivity"), v_sensitivity.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__frac(Py::str("the specificity"), v_specificity.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__frac(Py::str("the prevalence"), v_prevalence.clone(), Py::Bool(true), Py::Bool(true))?;
    v_tp = py::mul(&v_sensitivity.clone(), &v_prevalence.clone())?;
    v_fp = py::mul(&py::sub(&Py::Float(1.0), &v_specificity.clone())?, &py::sub(&Py::Float(1.0), &v_prevalence.clone())?)?;
    v_tn = py::mul(&v_specificity.clone(), &py::sub(&Py::Float(1.0), &v_prevalence.clone())?)?;
    v_fn = py::mul(&py::sub(&Py::Float(1.0), &v_sensitivity.clone())?, &v_prevalence.clone())?;
    v_lrp = py::truediv(&v_sensitivity.clone(), &py::sub(&Py::Float(1.0), &v_specificity.clone())?)?;
    v_lrn = py::truediv(&py::sub(&Py::Float(1.0), &v_sensitivity.clone())?, &v_specificity.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("PPV"), py::truediv(&v_tp.clone(), &py::add(&v_tp.clone(), &v_fp.clone())?)?), (Py::str("NPV"), py::truediv(&v_tn.clone(), &py::add(&v_tn.clone(), &v_fn.clone())?)?), (Py::str("LR_pos"), v_lrp.clone()), (Py::str("LR_neg"), v_lrn.clone()), (Py::str("DOR"), py::truediv(&v_lrp.clone(), &v_lrn.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_relative_risk(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py, mut v_level: Py) -> R {
    let mut v_rr: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let _ = f__table(v_a.clone(), v_b.clone(), v_c.clone(), v_d.clone())?;
    v_z = f__level(v_level.clone())?;
    v_rr = py::truediv(&py::truediv(&v_a.clone(), &py::add(&v_a.clone(), &v_b.clone())?)?, &py::truediv(&v_c.clone(), &py::add(&v_c.clone(), &v_d.clone())?)?)?;
    v_se = py::math1("sqrt", &py::sub(&py::add(&py::sub(&py::truediv(&Py::Int(1i64), &v_a.clone())?, &py::truediv(&Py::Int(1i64), &py::add(&v_a.clone(), &v_b.clone())?)?)?, &py::truediv(&Py::Int(1i64), &v_c.clone())?)?, &py::truediv(&Py::Int(1i64), &py::add(&v_c.clone(), &v_d.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("RR"), v_rr.clone()), (Py::str("lower"), py::mul(&v_rr.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?), (Py::str("upper"), py::mul(&v_rr.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_roc_auc(mut v_scores_pos: Py, mut v_scores_neg: Py) -> R {
    let mut v_u: Py = Py::None;
    let _ = f__two_groups(v_scores_pos.clone(), v_scores_neg.clone())?;
    v_u = py::fsum(&{ let mut out = vec![]; for t713 in py::iter(&(v_scores_pos.clone()))? { let mut v_a: Py; 
v_a = t713;
for t714 in py::iter(&(v_scores_neg.clone()))? { let mut v_b: Py; 
v_b = t714;
out.push((if ({ let t715 = v_a.clone(); Py::Bool({ let t716 = v_b.clone(); if py::gt(&t715, &t716)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { (if ({ let t717 = v_a.clone(); Py::Bool({ let t718 = v_b.clone(); if py::eq(&t717, &t718) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { Py::Float(0.0) }) })); }} Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("AUC"), py::truediv(&v_u.clone(), &py::mul(&py::len(&v_scores_pos.clone())?, &py::len(&v_scores_neg.clone())?)?)?), (Py::str("U"), v_u.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_sens_spec_ci(mut v_TP: Py, mut v_FN: Py, mut v_TN: Py, mut v_FP: Py, mut v_level: Py) -> R {
    let mut v_nm: Py = Py::None;
    let mut v_pl: Py = Py::None;
    let mut v_pu: Py = Py::None;
    let mut v_sl: Py = Py::None;
    let mut v_su: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_z: Py = Py::None;
    for t772 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("TP"), v_TP.clone()]), Py::tuple(vec![Py::str("FN"), v_FN.clone()]), Py::tuple(vec![Py::str("TN"), v_TN.clone()]), Py::tuple(vec![Py::str("FP"), v_FP.clone()])])))? {
        let t777 = (t772).items()?;
        if t777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t777[0].clone();
        v_v = t777[1].clone();
        let _ = f__count(v_nm.clone(), v_v.clone(), Py::Int(0i64))?;
    }
    let _ = f__need({ let t782 = { let t778 = py::add(&v_TP.clone(), &v_FN.clone())?; Py::Bool({ let t779 = Py::Int(1i64); if py::ge(&t778, &t779)? { true } else { false } }) }; if !t782.truthy() { t782 } else { { let t780 = py::add(&v_TN.clone(), &v_FP.clone())?; Py::Bool({ let t781 = Py::Int(1i64); if py::ge(&t780, &t781)? { true } else { false } }) } } }, Py::str("each group (diseased, non-diseased) needs at least one subject"))?;
    v_z = f__level(v_level.clone())?;
    let t789 = (f__wilson(v_TP.clone(), py::add(&v_TP.clone(), &v_FN.clone())?, v_z.clone())?).items()?;
    if t789.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sl = t789[0].clone();
    v_su = t789[1].clone();
    let t790 = (f__wilson(v_TN.clone(), py::add(&v_TN.clone(), &v_FP.clone())?, v_z.clone())?).items()?;
    if t790.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pl = t790[0].clone();
    v_pu = t790[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("sensitivity"), py::truediv(&v_TP.clone(), &py::add(&v_TP.clone(), &v_FN.clone())?)?), (Py::str("sens_lower"), v_sl.clone()), (Py::str("sens_upper"), v_su.clone()), (Py::str("specificity"), py::truediv(&v_TN.clone(), &py::add(&v_TN.clone(), &v_FP.clone())?)?), (Py::str("spec_lower"), v_pl.clone()), (Py::str("spec_upper"), v_pu.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_smr(mut v_observed: Py, mut v_expected: Py, mut v_level: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_hi_b: Py = Py::None;
    let mut v_lo: Py = Py::None;
    let mut v_up: Py = Py::None;
    let _ = f__count(Py::str("observed"), v_observed.clone(), Py::Int(0i64))?;
    let _ = f__need({ let t6613 = v_observed.clone(); Py::Bool({ let t6614 = Py::Int(1000i64); if py::le(&t6613, &t6614)? { true } else { false } }) }, Py::str("observed counts above 1000 are not supported"))?;
    let _ = f__pos(Py::str("the expected count"), Py::tuple(vec![v_expected.clone()]))?;
    let _ = f__level(v_level.clone())?;
    v_a = py::truediv(&py::sub(&Py::Int(1i64), &v_level.clone())?, &Py::Int(2i64))?;
    v_hi_b = py::add(&py::add(&v_observed.clone(), &py::mul(&Py::Float(20.0), &py::math1("sqrt", &py::add(&v_observed.clone(), &Py::Float(1.0))?)?)?)?, &Py::Float(50.0))?;
    if ({ let t6627 = v_observed.clone(); Py::Bool({ let t6628 = Py::Int(0i64); if py::eq(&t6627, &t6628) { true } else { false } }) }).truthy() {
        v_lo = Py::Float(0.0);
    } else {
        v_lo = f__bisect({ let v_a = v_a.clone(); let v_expected = v_expected.clone(); let v_hi_b = v_hi_b.clone(); let v_level = v_level.clone(); let v_lo = v_lo.clone(); let v_observed = v_observed.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lam: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&py::sub(&Py::Float(1.0), &f__pois_cdf(py::sub(&v_observed.clone(), &Py::Int(1i64))?, v_lam.clone())?)?, &v_a.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_hi_b.clone(), Py::Int(200i64))?;
    }
    v_up = f__bisect({ let v_a = v_a.clone(); let v_expected = v_expected.clone(); let v_hi_b = v_hi_b.clone(); let v_level = v_level.clone(); let v_lo = v_lo.clone(); let v_observed = v_observed.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_lam: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::sub(&f__pois_cdf(v_observed.clone(), v_lam.clone())?, &v_a.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, Py::Float(0.0), v_hi_b.clone(), Py::Int(200i64))?;
    return Ok(f__out(Py::dict(vec![(Py::str("SMR"), py::truediv(&v_observed.clone(), &v_expected.clone())?), (Py::str("lower"), py::truediv(&v_lo.clone(), &v_expected.clone())?), (Py::str("upper"), py::truediv(&v_up.clone(), &v_expected.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_threshold(mut v_scores_pos: Py, mut v_scores_neg: Py, mut v_threshold: Py) -> R {
    let mut v_fn: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_tn: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__two_groups(v_scores_pos.clone(), v_scores_neg.clone())?;
    let _ = f__fin(Py::str("the threshold"), Py::tuple(vec![v_threshold.clone()]))?;
    v_tp = py::sum(&{ let mut out = vec![]; for t853 in py::iter(&(v_scores_pos.clone()))? { let mut v_v: Py; 
v_v = t853;
if ({ let t854 = v_v.clone(); Py::Bool({ let t855 = v_threshold.clone(); if py::ge(&t854, &t855)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_fp = py::sum(&{ let mut out = vec![]; for t856 in py::iter(&(v_scores_neg.clone()))? { let mut v_v: Py; 
v_v = t856;
if ({ let t857 = v_v.clone(); Py::Bool({ let t858 = v_threshold.clone(); if py::ge(&t857, &t858)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    let t859 = (Py::tuple(vec![py::sub(&py::len(&v_scores_pos.clone())?, &v_tp.clone())?, py::sub(&py::len(&v_scores_neg.clone())?, &v_fp.clone())?])).items()?;
    if t859.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fn = t859[0].clone();
    v_tn = t859[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("sensitivity"), py::truediv(&v_tp.clone(), &py::add(&v_tp.clone(), &v_fn.clone())?)?), (Py::str("specificity"), py::truediv(&v_tn.clone(), &py::add(&v_tn.clone(), &v_fp.clone())?)?), (Py::str("TP"), v_tp.clone()), (Py::str("FN"), v_fn.clone()), (Py::str("TN"), v_tn.clone()), (Py::str("FP"), v_fp.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_dx_youden(mut v_scores_pos: Py, mut v_scores_neg: Py) -> R {
    let mut v_J: Py = Py::None;
    let mut v_fp: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_thr: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__two_groups(v_scores_pos.clone(), v_scores_neg.clone())?;
    let t935 = (f__youden(v_scores_pos.clone(), v_scores_neg.clone())?).items()?;
    if t935.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t936 = (t935[0].clone()).items()?;
    if t936.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_J = t936[0].clone();
    v_thr = t936[1].clone();
    v_tp = t936[2].clone();
    v_fp = t936[3].clone();
    v_n1 = t935[1].clone();
    v_n0 = t935[2].clone();
    let _ = f__need({ let t937 = v_J.clone(); Py::Bool({ let t938 = Py::Int(0i64); if py::gt(&t937, &t938)? { true } else { false } }) }, Py::str("the scores do not separate the groups (no cut-off with positive Youden index)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("threshold"), py::float(&v_thr.clone())?), (Py::str("sensitivity"), py::truediv(&v_tp.clone(), &v_n1.clone())?), (Py::str("specificity"), py::sub(&Py::Int(1i64), &py::truediv(&v_fp.clone(), &v_n0.clone())?)?), (Py::str("J"), py::float(&v_J.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_basic_rates(mut v_cases: Py, mut v_population: Py, mut v_prevalent: Py, mut v_deaths: Py, mut v_period: Py) -> R {
    let mut v_ip: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__count(Py::str("the number of new cases"), v_cases.clone(), Py::Int(0i64))?;
    let _ = f__count(Py::str("the population"), v_population.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t1070 = v_cases.clone(); Py::Bool({ let t1071 = v_population.clone(); if py::le(&t1070, &t1071)? { true } else { false } }) }, Py::str("the new cases cannot exceed the population"))?;
    v_ip = py::truediv(&v_cases.clone(), &v_population.clone())?;
    v_out = Py::dict(vec![(Py::str("incidence_proportion"), v_ip.clone()), (Py::str("incidence_per_100000"), py::mul(&v_ip.clone(), &Py::Float(100000.0))?)]);
    if ({ let t1072 = v_period.clone(); Py::Bool({ let t1073 = Py::None; if !py::is(&t1072, &t1073) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the period"), Py::tuple(vec![v_period.clone()]))?;
        py::setitem(&v_out.clone(), &Py::str("incidence_rate"), py::truediv(&v_cases.clone(), &py::mul(&v_population.clone(), &v_period.clone())?)?)?;
        py::setitem(&v_out.clone(), &Py::str("incidence_per_100000_person_years"), py::mul(&py::truediv(&v_cases.clone(), &py::truediv(&py::mul(&v_population.clone(), &v_period.clone())?, &c_YEAR_S()?)?)?, &Py::Float(100000.0))?)?;
    }
    if ({ let t1080 = v_prevalent.clone(); Py::Bool({ let t1081 = Py::None; if !py::is(&t1080, &t1081) { true } else { false } }) }).truthy() {
        let _ = f__count(Py::str("the number of existing (prevalent) cases"), v_prevalent.clone(), Py::Int(0i64))?;
        let _ = f__need({ let t1082 = v_prevalent.clone(); Py::Bool({ let t1083 = v_population.clone(); if py::le(&t1082, &t1083)? { true } else { false } }) }, Py::str("the existing cases cannot exceed the population"))?;
        py::setitem(&v_out.clone(), &Py::str("prevalence"), py::truediv(&v_prevalent.clone(), &v_population.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("prevalence_per_100000"), py::mul(&py::truediv(&v_prevalent.clone(), &v_population.clone())?, &Py::Float(100000.0))?)?;
    }
    if ({ let t1084 = v_deaths.clone(); Py::Bool({ let t1085 = Py::None; if !py::is(&t1084, &t1085) { true } else { false } }) }).truthy() {
        let _ = f__count(Py::str("the number of deaths"), v_deaths.clone(), Py::Int(0i64))?;
        let _ = f__need({ let t1086 = v_cases.clone(); Py::Bool({ let t1087 = Py::Int(1i64); if py::ge(&t1086, &t1087)? { true } else { false } }) }, Py::str("the case-fatality rate needs at least one case"))?;
        let _ = f__need({ let t1088 = v_deaths.clone(); Py::Bool({ let t1089 = v_cases.clone(); if py::le(&t1088, &t1089)? { true } else { false } }) }, Py::str("the deaths from the disease cannot exceed its cases"))?;
        py::setitem(&v_out.clone(), &Py::str("case_fatality"), py::truediv(&v_deaths.clone(), &v_cases.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("mortality_per_100000"), py::mul(&py::truediv(&v_deaths.clone(), &v_population.clone())?, &Py::Float(100000.0))?)?;
        if ({ let t1090 = v_period.clone(); Py::Bool({ let t1091 = Py::None; if !py::is(&t1090, &t1091) { true } else { false } }) }).truthy() {
            py::setitem(&v_out.clone(), &Py::str("mortality_per_100000_person_years"), py::mul(&py::truediv(&v_deaths.clone(), &py::truediv(&py::mul(&v_population.clone(), &v_period.clone())?, &c_YEAR_S()?)?)?, &Py::Float(100000.0))?)?;
        }
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_herd_immunity(mut v_R0: Py, mut v_efficacy: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("R0"), Py::tuple(vec![v_R0.clone()]))?;
    let _ = f__need({ let t1152 = v_R0.clone(); Py::Bool({ let t1153 = Py::Float(1.0); if py::gt(&t1152, &t1153)? { true } else { false } }) }, Py::str("R0 must exceed 1 (below 1 an infection dies out without vaccination)"))?;
    let _ = f__need({ let t1154 = v_R0.clone(); Py::Bool({ let t1155 = Py::Float(100.0); if py::le(&t1154, &t1155)? { true } else { false } }) }, Py::str("R0 must be at most 100"))?;
    v_h = py::sub(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_R0.clone())?)?;
    v_out = Py::dict(vec![(Py::str("threshold"), v_h.clone()), (Py::str("threshold_percent"), py::mul(&v_h.clone(), &Py::Float(100.0))?)]);
    if ({ let t1156 = v_efficacy.clone(); Py::Bool({ let t1157 = Py::None; if !py::is(&t1156, &t1157) { true } else { false } }) }).truthy() {
        let _ = f__frac(Py::str("the vaccine efficacy"), v_efficacy.clone(), Py::Bool(true), Py::Bool(false))?;
        v_c = py::truediv(&v_h.clone(), &v_efficacy.clone())?;
        py::setitem(&v_out.clone(), &Py::str("coverage"), v_c.clone())?;
        py::setitem(&v_out.clone(), &Py::str("coverage_percent"), py::mul(&v_c.clone(), &Py::Float(100.0))?)?;
        py::setitem(&v_out.clone(), &Py::str("reachable"), (if ({ let t1170 = v_c.clone(); Py::Bool({ let t1171 = Py::Float(1.0); if py::le(&t1170, &t1171)? { true } else { false } }) }).truthy() { Py::str("reachable") } else { Py::str("not reachable with this vaccine") }))?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_r_from_growth(mut v_r: Py, mut v_Tg_mean: Py, mut v_Tg_sd: Py) -> R {
    let mut v_k: Py = Py::None;
    let mut v_th: Py = Py::None;
    let _ = f__fin(Py::str("the growth rate r"), Py::tuple(vec![v_r.clone()]))?;
    let _ = f__pos(Py::str("the mean and standard deviation of the generation interval"), Py::tuple(vec![v_Tg_mean.clone(), v_Tg_sd.clone()]))?;
    v_k = py::pow(&py::truediv(&v_Tg_mean.clone(), &v_Tg_sd.clone())?, &Py::Int(2i64))?;
    v_th = py::truediv(&py::pow(&v_Tg_sd.clone(), &Py::Int(2i64))?, &v_Tg_mean.clone())?;
    let _ = f__need({ let t1235 = py::add(&Py::Float(1.0), &py::mul(&v_r.clone(), &v_th.clone())?)?; Py::Bool({ let t1236 = Py::Int(0i64); if py::gt(&t1235, &t1236)? { true } else { false } }) }, Py::str("the decline is too fast for this generation interval (1 + r sd^2/mean must be positive)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::math1("exp", &py::mul(&v_k.clone(), &py::math1("log1p", &py::mul(&v_r.clone(), &v_th.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_reed_frost(mut v_n: Py, mut v_m: Py, mut v_p: Py) -> R {
    let mut v_final: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_j: Py = Py::None;
    let mut v_nxt: Py = Py::None;
    let mut v_pi: Py = Py::None;
    let mut v_pr: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_qi: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_states: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__need({ let t1293 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1293.truthy() { t1293 } else { { let t1290 = Py::Int(1i64); Py::Bool({ let t1291 = v_n.clone(); if py::le(&t1290, &t1291)? { { let t1292 = Py::Int(30i64); if py::le(&t1291, &t1292)? { true } else { false } } } else { false } }) } } }, Py::str("n (initial susceptibles) is a whole number from 1 to 30"))?;
    let _ = f__need({ let t1297 = Py::Bool(matches!(v_m.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1297.truthy() { t1297 } else { { let t1294 = Py::Int(1i64); Py::Bool({ let t1295 = v_m.clone(); if py::le(&t1294, &t1295)? { { let t1296 = Py::Int(30i64); if py::le(&t1295, &t1296)? { true } else { false } } } else { false } }) } } }, Py::str("m (initial infectives) is a whole number from 1 to 30"))?;
    let _ = f__frac(Py::str("p"), v_p.clone(), Py::Bool(true), Py::Bool(true))?;
    v_q = py::sub(&Py::Float(1.0), &v_p.clone())?;
    v_states = Py::dict(vec![(Py::tuple(vec![v_n.clone(), v_m.clone()]), Py::Float(1.0))]);
    v_final = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    while (v_states.clone()).truthy() {
        v_nxt = Py::dict(vec![]);
        for t1310 in py::iter(&(py::method(&v_states.clone(), "items", &[])?))? {
            let t1319 = (t1310).items()?;
            if t1319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            let t1320 = (t1319[0].clone()).items()?;
            if t1320.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s = t1320[0].clone();
            v_i = t1320[1].clone();
            v_pr = t1319[1].clone();
            if ({ let t1315 = { let t1311 = v_i.clone(); Py::Bool({ let t1312 = Py::Int(0i64); if py::eq(&t1311, &t1312) { true } else { false } }) }; if t1315.truthy() { t1315 } else { { let t1313 = v_s.clone(); Py::Bool({ let t1314 = Py::Int(0i64); if py::eq(&t1313, &t1314) { true } else { false } }) } } }).truthy() {
                let t1316 = v_final.clone(); let t1317 = py::sub(&v_n.clone(), &v_s.clone())?;
                py::setitem(&t1316, &t1317, py::iadd(&py::getitem(&t1316, &t1317)?, &v_pr.clone())?)?;
                continue;
            }
            v_qi = py::pow(&v_q.clone(), &v_i.clone())?;
            v_pi = py::sub(&Py::Float(1.0), &v_qi.clone())?;
            for t1318 in py::range_iter(&[(py::add(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t1318;
                v_w = py::mul(&py::mul(&py::mul(&v_pr.clone(), &py::comb(&v_s.clone(), &v_j.clone())?)?, &py::pow(&v_pi.clone(), &v_j.clone())?)?, &py::pow(&v_qi.clone(), &py::sub(&v_s.clone(), &v_j.clone())?)?)?;
                if (v_w.clone()).truthy() {
                    py::setitem(&v_nxt.clone(), &Py::tuple(vec![py::sub(&v_s.clone(), &v_j.clone())?, v_j.clone()]), py::add(&py::method(&v_nxt.clone(), "get", &[Py::tuple(vec![py::sub(&v_s.clone(), &v_j.clone())?, v_j.clone()]), Py::Float(0.0)])?, &v_w.clone())?)?;
                }
            }
        }
        v_states = v_nxt.clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mean_final"), py::fsum(&{ let mut out = vec![]; for t1333 in py::iter(&(py::enumerate(&v_final.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t1334 = (t1333).items()?;
if t1334.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t1334[0].clone();
v_v = t1334[1].clone();
out.push(py::mul(&v_k.clone(), &v_v.clone())?); } Py::list(out) })?), (Py::str("P_no_spread"), py::getitem(&v_final.clone(), &Py::Int(0i64))?), (Py::str("distribution"), v_final.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_seir_final_size(mut v_R0: Py, mut v_s0: Py, mut v_e0: Py, mut v_i0: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("R0"), Py::tuple(vec![v_R0.clone()]))?;
    let _ = f__frac(Py::str("s0"), v_s0.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("e0"), v_e0.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__frac(Py::str("i0"), v_i0.clone(), Py::Bool(false), Py::Bool(false))?;
    let _ = f__need({ let t1394 = py::add(&v_e0.clone(), &v_i0.clone())?; Py::Bool({ let t1395 = Py::Int(0i64); if py::gt(&t1394, &t1395)? { true } else { false } }) }, Py::str("some exposed or infected must be present at the start"))?;
    let _ = f__need({ let t1396 = py::add(&py::add(&v_s0.clone(), &v_e0.clone())?, &v_i0.clone())?; Py::Bool({ let t1397 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1396, &t1397)? { true } else { false } }) }, Py::str("s0 + e0 + i0 cannot exceed 1"))?;
    v_s = f__final_size(v_R0.clone(), v_s0.clone(), py::add(&v_e0.clone(), &v_i0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("s_inf"), v_s.clone()), (Py::str("attack"), py::sub(&v_s0.clone(), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_seir_growth(mut v_R0: Py, mut v_sigma: Py, mut v_gamma: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_r: Py = Py::None;
    let mut v_root: Py = Py::None;
    let _ = f__pos(Py::str("R0, sigma and gamma"), Py::tuple(vec![v_R0.clone(), v_sigma.clone(), v_gamma.clone()]))?;
    v_a = py::add(&v_sigma.clone(), &v_gamma.clone())?;
    v_d = py::mul(&py::mul(&py::mul(&Py::Float(4.0), &v_sigma.clone())?, &v_gamma.clone())?, &py::sub(&v_R0.clone(), &Py::Float(1.0))?)?;
    v_root = py::math1("sqrt", &py::add(&py::mul(&v_a.clone(), &v_a.clone())?, &v_d.clone())?)?;
    v_r = (if ({ let t1483 = v_d.clone(); Py::Bool({ let t1484 = Py::Int(0i64); if py::ge(&t1483, &t1484)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &py::mul(&Py::Float(2.0), &py::add(&v_a.clone(), &v_root.clone())?)?)? } else { py::truediv(&py::sub(&v_root.clone(), &v_a.clone())?, &Py::Float(2.0))? });
    return Ok(f__out(Py::dict(vec![(Py::str("r"), v_r.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_seir_peak(mut v_beta: Py, mut v_sigma: Py, mut v_gamma: Py, mut v_s0: Py, mut v_e0: Py, mut v_i0: Py) -> R {
    let mut v_R0: Py = Py::None;
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_nm: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_tau: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("beta, sigma and gamma"), Py::tuple(vec![v_beta.clone(), v_sigma.clone(), v_gamma.clone()]))?;
    let _ = f__frac(Py::str("s0"), v_s0.clone(), Py::Bool(true), Py::Bool(false))?;
    for t1556 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("e0"), v_e0.clone()]), Py::tuple(vec![Py::str("i0"), v_i0.clone()])])))? {
        let t1557 = (t1556).items()?;
        if t1557.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t1557[0].clone();
        v_v = t1557[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(false), Py::Bool(false))?;
    }
    let _ = f__need({ let t1558 = py::add(&v_e0.clone(), &v_i0.clone())?; Py::Bool({ let t1559 = Py::Int(0i64); if py::gt(&t1558, &t1559)? { true } else { false } }) }, Py::str("an epidemic needs e0 + i0 > 0"))?;
    let _ = f__need({ let t1560 = py::add(&py::add(&v_s0.clone(), &v_e0.clone())?, &v_i0.clone())?; Py::Bool({ let t1561 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1560, &t1561)? { true } else { false } }) }, Py::str("s0 + e0 + i0 cannot exceed 1"))?;
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    let _ = f__need({ let t1562 = py::mul(&v_R0.clone(), &v_s0.clone())?; Py::Bool({ let t1563 = py::add(&Py::Float(1.0), &Py::Float(1e-06))?; if py::gt(&t1562, &t1563)? { true } else { false } }) }, Py::str("no epidemic peak: R0 s0 must exceed 1"))?;
    let t1597 = (f__seir_taylor_peak(v_R0.clone(), py::truediv(&v_sigma.clone(), &v_gamma.clone())?, v_s0.clone(), v_e0.clone(), v_i0.clone(), Py::Int(26i64))?).items()?;
    if t1597.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_tau = t1597[0].clone();
    v_s = t1597[1].clone();
    v_e = t1597[2].clone();
    v_i = t1597[3].clone();
    let _ = f__need({ let t1598 = v_i.clone(); Py::Bool({ let t1599 = py::mul(&v_i0.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t1598, &t1599)? { true } else { false } }) }, Py::str("the infectious fraction never rises above its initial value"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_peak"), py::truediv(&v_tau.clone(), &v_gamma.clone())?), (Py::str("i_max"), v_i.clone()), (Py::str("s_at_peak"), v_s.clone()), (Py::str("e_at_peak"), v_e.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_sir_final_size(mut v_R0: Py, mut v_s0: Py, mut v_i0: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("R0"), Py::tuple(vec![v_R0.clone()]))?;
    let _ = f__frac(Py::str("s0"), v_s0.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("i0"), v_i0.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__need({ let t1671 = py::add(&v_s0.clone(), &v_i0.clone())?; Py::Bool({ let t1672 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1671, &t1672)? { true } else { false } }) }, Py::str("s0 + i0 cannot exceed 1"))?;
    v_s = f__final_size(v_R0.clone(), v_s0.clone(), v_i0.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("s_inf"), v_s.clone()), (Py::str("attack"), py::sub(&v_s0.clone(), &v_s.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_sir_peak(mut v_beta: Py, mut v_gamma: Py, mut v_s0: Py, mut v_i0: Py) -> R {
    let mut v_R0: Py = Py::None;
    let mut v_imax: Py = Py::None;
    let _ = f__pos(Py::str("beta and gamma"), Py::tuple(vec![v_beta.clone(), v_gamma.clone()]))?;
    let _ = f__frac(Py::str("s0"), v_s0.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__frac(Py::str("i0"), v_i0.clone(), Py::Bool(true), Py::Bool(true))?;
    let _ = f__need({ let t1770 = py::add(&v_s0.clone(), &v_i0.clone())?; Py::Bool({ let t1771 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1770, &t1771)? { true } else { false } }) }, Py::str("s0 + i0 cannot exceed 1"))?;
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    let _ = f__need({ let t1772 = py::mul(&v_R0.clone(), &v_s0.clone())?; Py::Bool({ let t1773 = py::add(&Py::Float(1.0), &Py::Float(1e-06))?; if py::gt(&t1772, &t1773)? { true } else { false } }) }, Py::str("no epidemic peak: R0 s0 must exceed 1 (prevalence only falls)"))?;
    v_imax = py::sub(&py::add(&v_s0.clone(), &v_i0.clone())?, &py::truediv(&py::add(&Py::Float(1.0), &py::math1("log", &py::mul(&v_R0.clone(), &v_s0.clone())?)?)?, &v_R0.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("i_max"), v_imax.clone()), (Py::str("t_peak"), f__sir_peak_time(v_beta.clone(), v_gamma.clone(), v_s0.clone(), v_i0.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_epi_sis(mut v_beta: Py, mut v_gamma: Py, mut v_i0: Py, mut v_t: Py) -> R {
    let mut v_e: Py = Py::None;
    let mut v_i: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("beta and gamma"), Py::tuple(vec![v_beta.clone(), v_gamma.clone()]))?;
    let _ = f__frac(Py::str("i0"), v_i0.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_r = py::sub(&v_beta.clone(), &v_gamma.clone())?;
    if ({ let t1855 = v_r.clone(); Py::Bool({ let t1856 = Py::Int(0i64); if py::eq(&t1855, &t1856) { true } else { false } }) }).truthy() {
        v_i = py::truediv(&v_i0.clone(), &py::add(&Py::Float(1.0), &py::mul(&py::mul(&v_beta.clone(), &v_i0.clone())?, &v_t.clone())?)?)?;
    } else {
        v_e = py::math1("exp", &py::mul(&v_r.clone(), &v_t.clone())?)?;
        v_i = py::truediv(&py::mul(&v_i0.clone(), &v_e.clone())?, &py::add(&Py::Float(1.0), &py::truediv(&py::mul(&py::mul(&v_i0.clone(), &v_beta.clone())?, &py::math1("expm1", &py::mul(&v_r.clone(), &v_t.clone())?)?)?, &v_r.clone())?)?)?;
    }
    return Ok(f__out(Py::dict(vec![(Py::str("i"), v_i.clone()), (Py::str("i_star"), py::minmax(&[Py::Float(0.0), py::sub(&Py::Float(1.0), &py::truediv(&v_gamma.clone(), &v_beta.clone())?)?], true)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ernst_angle(mut v_TR: Py, mut v_T1: Py) -> R {
    let mut v_E1: Py = Py::None;
    let mut v_S: Py = Py::None;
    let mut v_a: Py = Py::None;
    let _ = f__pos(Py::str("TR and T1"), Py::tuple(vec![v_TR.clone(), v_T1.clone()]))?;
    v_E1 = py::math1("exp", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?;
    v_a = py::math1("acos", &v_E1.clone())?;
    v_S = py::truediv(&py::mul(&py::math1("sin", &v_a.clone())?, &py::sub(&Py::Int(1i64), &v_E1.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&v_E1.clone(), &py::math1("cos", &v_a.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha"), v_a.clone()), (Py::str("alpha_deg"), py::math1("degrees", &v_a.clone())?), (Py::str("S_rel"), v_S.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_hounsfield(mut v_mu: Py, mut v_mu_water: Py, mut v_mu_air: Py) -> R {
    let mut v_HU: Py = Py::None;
    let _ = f__pos(Py::str("the attenuation coefficients"), Py::tuple(vec![v_mu.clone(), v_mu_water.clone()]))?;
    let _ = f__need({ let t2701 = { let t2697 = v_mu_air.clone(); Py::Bool({ let t2698 = Py::Int(0i64); if py::ge(&t2697, &t2698)? { true } else { false } }) }; if !t2701.truthy() { t2701 } else { { let t2699 = v_mu_air.clone(); Py::Bool({ let t2700 = v_mu_water.clone(); if py::lt(&t2699, &t2700)? { true } else { false } }) } } }, Py::str("the air coefficient must lie in [0, mu_water)"))?;
    v_HU = py::truediv(&py::mul(&Py::Float(1000.0), &py::sub(&v_mu.clone(), &v_mu_water.clone())?)?, &py::sub(&v_mu_water.clone(), &v_mu_air.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("HU"), v_HU.clone()), (Py::str("mu_rel"), py::truediv(&v_mu.clone(), &v_mu_water.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_adc(mut v_b: Py, mut v_S: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let t3746 = (f__loglin_fit_out(v_b.clone(), v_S.clone(), Py::str("b"))?).items()?;
    if t3746.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t3746[0].clone();
    v_c = t3746[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("ADC"), py::neg(&v_slope.clone())?), (Py::str("S0"), py::math1("exp", &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_inversion_recovery(mut v_T1: Py, mut v_TR: Py, mut v_TI: Py) -> R {
    let mut v_e: Py = Py::None;
    let _ = f__pos(Py::str("T1, TR and TI"), Py::tuple(vec![v_T1.clone(), v_TR.clone(), v_TI.clone()]))?;
    let _ = f__need({ let t3865 = v_TI.clone(); Py::Bool({ let t3866 = v_TR.clone(); if py::lt(&t3865, &t3866)? { true } else { false } }) }, Py::str("TI must be shorter than TR"))?;
    v_e = py::math1("exp", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Mz_ratio"), py::add(&py::sub(&Py::Float(1.0), &py::mul(&Py::Float(2.0), &py::math1("exp", &py::truediv(&py::neg(&v_TI.clone())?, &v_T1.clone())?)?)?)?, &v_e.clone())?), (Py::str("TI_null"), py::mul(&v_T1.clone(), &py::math1("log", &py::truediv(&Py::Float(2.0), &py::add(&Py::Float(1.0), &v_e.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_psf_mtf(mut v_fwhm: Py, mut v_f: Py) -> R {
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("the FWHM"), Py::tuple(vec![v_fwhm.clone()]))?;
    let _ = f__nonneg(Py::str("the spatial frequency f"), Py::tuple(vec![v_f.clone()]))?;
    v_s = py::truediv(&v_fwhm.clone(), &c__FWHM_SIGMA()?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("MTF"), py::math1("exp", &py::mul(&py::mul(&py::mul(&py::mul(&py::mul(&py::neg(&Py::Float(2.0))?, &py::pow(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?)?, &v_s.clone())?, &v_s.clone())?, &v_f.clone())?, &v_f.clone())?)?), (Py::str("f50"), py::truediv(&py::math1("sqrt", &py::truediv(&c_LN2()?, &Py::Float(2.0))?)?, &py::mul(&Py::Float(std::f64::consts::PI), &v_s.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_spin_echo(mut v_M0: Py, mut v_TR: Py, mut v_TE: Py, mut v_T1: Py, mut v_T2: Py) -> R {
    let _ = f__pos(Py::str("M0, TR, TE, T1 and T2"), Py::tuple(vec![v_M0.clone(), v_TR.clone(), v_TE.clone(), v_T1.clone(), v_T2.clone()]))?;
    let _ = f__need({ let t3926 = v_TE.clone(); Py::Bool({ let t3927 = v_TR.clone(); if py::lt(&t3926, &t3927)? { true } else { false } }) }, Py::str("TE must be shorter than TR"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), py::mul(&py::mul(&v_M0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_TE.clone())?, &v_T2.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_spoiled_gre(mut v_M0: Py, mut v_TR: Py, mut v_TE: Py, mut v_T1: Py, mut v_T2star: Py, mut v_alpha: Py) -> R {
    let mut v_E1: Py = Py::None;
    let _ = f__pos(Py::str("M0, TR, TE, T1 and T2star"), Py::tuple(vec![v_M0.clone(), v_TR.clone(), v_TE.clone(), v_T1.clone(), v_T2star.clone()]))?;
    let _ = f__need({ let t3987 = v_TE.clone(); Py::Bool({ let t3988 = v_TR.clone(); if py::lt(&t3987, &t3988)? { true } else { false } }) }, Py::str("TE must be shorter than TR"))?;
    let _ = f__need({ let t3989 = Py::Int(0i64); Py::Bool({ let t3990 = v_alpha.clone(); if py::lt(&t3989, &t3990)? { { let t3991 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t3990, &t3991)? { true } else { false } } } else { false } }) }, Py::str("the flip angle must lie between 0 and 90 degrees"))?;
    v_E1 = py::math1("exp", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), py::mul(&py::truediv(&py::mul(&py::mul(&v_M0.clone(), &py::math1("sin", &v_alpha.clone())?)?, &py::sub(&Py::Int(1i64), &v_E1.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&py::math1("cos", &v_alpha.clone())?, &v_E1.clone())?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_TE.clone())?, &v_T2star.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_t2_fit(mut v_TE: Py, mut v_S: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let t4067 = (f__loglin_fit_out(v_TE.clone(), v_S.clone(), Py::str("echo time"))?).items()?;
    if t4067.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t4067[0].clone();
    v_c = t4067[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("T2"), py::truediv(&py::neg(&Py::Float(1.0))?, &v_slope.clone())?), (Py::str("S0"), py::math1("exp", &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_us_interface(mut v_Z1: Py, mut v_Z2: Py) -> R {
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the impedances Z1 and Z2"), Py::tuple(vec![v_Z1.clone(), v_Z2.clone()]))?;
    v_r = py::truediv(&py::sub(&v_Z2.clone(), &v_Z1.clone())?, &py::add(&v_Z2.clone(), &v_Z1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("R"), py::mul(&v_r.clone(), &v_r.clone())?), (Py::str("T"), py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_Z1.clone())?, &v_Z2.clone())?, &py::pow(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &Py::Int(2i64))?)?), (Py::str("r_p"), v_r.clone()), (Py::str("t_p"), py::truediv(&py::mul(&Py::Float(2.0), &v_Z2.clone())?, &py::add(&v_Z1.clone(), &v_Z2.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_us_matching(mut v_Z1: Py, mut v_Zm: Py, mut v_Z2: Py, mut v_d: Py, mut v_c_m: Py, mut v_f: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_den: Py = Py::None;
    let mut v_kd: Py = Py::None;
    let mut v_s: Py = Py::None;
    let _ = f__pos(Py::str("Z1, Zm, Z2, d, c_m and f"), Py::tuple(vec![v_Z1.clone(), v_Zm.clone(), v_Z2.clone(), v_d.clone(), v_c_m.clone(), v_f.clone()]))?;
    v_kd = py::mul(&py::truediv(&py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?, &v_c_m.clone())?, &v_d.clone())?;
    let t6850 = (Py::tuple(vec![py::math1("cos", &v_kd.clone())?, py::math1("sin", &v_kd.clone())?])).items()?;
    if t6850.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t6850[0].clone();
    v_s = t6850[1].clone();
    v_den = py::add(&py::mul(&py::mul(&py::pow(&py::add(&v_Z1.clone(), &v_Z2.clone())?, &Py::Int(2i64))?, &v_c.clone())?, &v_c.clone())?, &py::mul(&py::mul(&py::pow(&py::add(&v_Zm.clone(), &py::truediv(&py::mul(&v_Z1.clone(), &v_Z2.clone())?, &v_Zm.clone())?)?, &Py::Int(2i64))?, &v_s.clone())?, &v_s.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), py::truediv(&py::mul(&py::mul(&Py::Float(4.0), &v_Z1.clone())?, &v_Z2.clone())?, &v_den.clone())?), (Py::str("kd"), v_kd.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_xray(mut v_I0: Py, mut v_mu_rho: Py, mut v_rho: Py, mut v_x: Py) -> R {
    let mut v_T: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("I0"), Py::tuple(vec![v_I0.clone()]))?;
    v_n = py::len(&v_mu_rho.clone())?;
    let _ = f__need({ let t7176 = { let t7169 = v_n.clone(); Py::Bool({ let t7170 = Py::Int(1i64); if py::ge(&t7169, &t7170)? { true } else { false } }) }; if !t7176.truthy() { t7176 } else { { let t7175 = { let t7171 = py::len(&v_rho.clone())?; Py::Bool({ let t7172 = v_n.clone(); if py::eq(&t7171, &t7172) { true } else { false } }) }; if !t7175.truthy() { t7175 } else { { let t7173 = py::len(&v_x.clone())?; Py::Bool({ let t7174 = v_n.clone(); if py::eq(&t7173, &t7174) { true } else { false } }) } } } } }, Py::str("give one mass attenuation coefficient, density and thickness per layer"))?;
    for t7177 in py::iter(&(py::add(&Py::list((v_mu_rho.clone()).items()?), &Py::list((v_rho.clone()).items()?))?))? {
        v_v = t7177;
        let _ = f__need({ let t7178 = v_v.clone(); Py::Bool({ let t7179 = Py::Int(0i64); if py::gt(&t7178, &t7179)? { true } else { false } }) }, Py::str("attenuation coefficients and densities must be positive"))?;
    }
    for t7180 in py::iter(&(v_x.clone()))? {
        v_v = t7180;
        let _ = f__need({ let t7181 = v_v.clone(); Py::Bool({ let t7182 = Py::Int(0i64); if py::ge(&t7181, &t7182)? { true } else { false } }) }, Py::str("thicknesses cannot be negative"))?;
    }
    v_T = py::math1("exp", &py::neg(&py::fsum(&{ let mut out = vec![]; for t7183 in py::iter(&(py::zip(&[v_mu_rho.clone(), v_rho.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t7184 = (t7183).items()?;
if t7184.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t7184[0].clone();
v_b = t7184[1].clone();
v_c = t7184[2].clone();
out.push(py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &v_c.clone())?); } Py::list(out) })?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("I"), py::mul(&v_I0.clone(), &v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iv_flow_rate(mut v_V: Py, mut v_T: Py, mut v_rate: Py, mut v_drop_factor: Py) -> R {
    let mut v_dur: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the volume to infuse"), Py::tuple(vec![v_V.clone()]))?;
    let _ = f__need({ let t2905 = { let t2907 = v_T.clone(); Py::Bool({ let t2908 = Py::None; if py::is(&t2907, &t2908) { true } else { false } }) }; Py::Bool({ let t2906 = { let t2909 = v_rate.clone(); Py::Bool({ let t2910 = Py::None; if py::is(&t2909, &t2910) { true } else { false } }) }; if py::ne(&t2905, &t2906) { true } else { false } }) }, Py::str("give either the infusion time T or the flow rate, not both"))?;
    if ({ let t2911 = v_T.clone(); Py::Bool({ let t2912 = Py::None; if !py::is(&t2911, &t2912) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the infusion time"), Py::tuple(vec![v_T.clone()]))?;
        v_q = py::truediv(&v_V.clone(), &v_T.clone())?;
        v_dur = v_T.clone();
    } else {
        let _ = f__pos(Py::str("the flow rate"), Py::tuple(vec![v_rate.clone()]))?;
        v_q = v_rate.clone();
        v_dur = py::truediv(&v_V.clone(), &v_rate.clone())?;
    }
    v_out = Py::dict(vec![(Py::str("flow"), v_q.clone()), (Py::str("flow_mL_h"), py::mul(&v_q.clone(), &Py::Float(3600000000.0))?), (Py::str("duration"), v_dur.clone()), (Py::str("duration_h"), py::truediv(&v_dur.clone(), &Py::Float(3600.0))?), (Py::str("duration_min"), py::truediv(&v_dur.clone(), &Py::Float(60.0))?)]);
    if ({ let t2913 = v_drop_factor.clone(); Py::Bool({ let t2914 = Py::None; if !py::is(&t2913, &t2914) { true } else { false } }) }).truthy() {
        let _ = f__count(Py::str("the drop factor (drops per mL)"), v_drop_factor.clone(), Py::Int(1i64))?;
        let _ = f__need({ let t2919 = v_drop_factor.clone(); Py::Bool({ let t2920 = Py::Int(100i64); if py::le(&t2919, &t2920)? { true } else { false } }) }, Py::str("the drop factor must be at most 100 drops per mL"))?;
        py::setitem(&v_out.clone(), &Py::str("drops_per_min"), py::mul(&py::mul(&v_q.clone(), &Py::Float(60000000.0))?, &v_drop_factor.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_iv_rate_from_dose_rate(mut v_drug: Py, mut v_bag_volume: Py, mut v_dose_rate: Py, mut v_dose_ug_per_kg_min: Py, mut v_W: Py, mut v_drop_factor: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_dr: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_q: Py = Py::None;
    let _ = f__pos(Py::str("the drug amount in the bag and the bag volume"), Py::tuple(vec![v_drug.clone(), v_bag_volume.clone()]))?;
    let _ = f__need({ let t2981 = { let t2983 = v_dose_rate.clone(); Py::Bool({ let t2984 = Py::None; if py::is(&t2983, &t2984) { true } else { false } }) }; Py::Bool({ let t2982 = { let t2985 = v_dose_ug_per_kg_min.clone(); Py::Bool({ let t2986 = Py::None; if py::is(&t2985, &t2986) { true } else { false } }) }; if py::ne(&t2981, &t2982) { true } else { false } }) }, Py::str("give either the dose rate or the dose per kilogram per minute, not both"))?;
    if ({ let t2987 = v_dose_ug_per_kg_min.clone(); Py::Bool({ let t2988 = Py::None; if !py::is(&t2987, &t2988) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the dose per kilogram per minute"), Py::tuple(vec![v_dose_ug_per_kg_min.clone()]))?;
        let _ = f__need({ let t2989 = v_dose_ug_per_kg_min.clone(); Py::Bool({ let t2990 = Py::Float(1000.0); if py::le(&t2989, &t2990)? { true } else { false } }) }, Py::str("the dose per kilogram per minute is a number in ug/kg/min and must be at most 1000"))?;
        let _ = f__need({ let t2991 = v_W.clone(); Py::Bool({ let t2992 = Py::None; if !py::is(&t2991, &t2992) { true } else { false } }) }, Py::str("a dose per kilogram needs the body weight W"))?;
        let _ = f__pos(Py::str("the body weight"), Py::tuple(vec![v_W.clone()]))?;
        let _ = f__need({ let t2993 = Py::Float(0.3); Py::Bool({ let t2994 = v_W.clone(); if py::le(&t2993, &t2994)? { { let t2995 = Py::Float(400.0); if py::le(&t2994, &t2995)? { true } else { false } } } else { false } }) }, Py::str("the body weight must lie between 0.3 and 400 kg"))?;
        v_dr = py::truediv(&py::mul(&py::mul(&v_dose_ug_per_kg_min.clone(), &Py::Float(1e-09))?, &v_W.clone())?, &Py::Float(60.0))?;
    } else {
        let _ = f__need({ let t2996 = v_W.clone(); Py::Bool({ let t2997 = Py::None; if py::is(&t2996, &t2997) { true } else { false } }) }, Py::str("the body weight is used only with a dose per kilogram"))?;
        let _ = f__pos(Py::str("the dose rate"), Py::tuple(vec![v_dose_rate.clone()]))?;
        v_dr = v_dose_rate.clone();
    }
    v_C = py::truediv(&v_drug.clone(), &v_bag_volume.clone())?;
    v_q = py::truediv(&v_dr.clone(), &v_C.clone())?;
    v_out = Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("dose_rate"), v_dr.clone()), (Py::str("dose_mg_h"), py::mul(&v_dr.clone(), &Py::Float(3600000000.0))?), (Py::str("flow"), v_q.clone()), (Py::str("flow_mL_h"), py::mul(&v_q.clone(), &Py::Float(3600000000.0))?)]);
    if ({ let t2998 = v_drop_factor.clone(); Py::Bool({ let t2999 = Py::None; if !py::is(&t2998, &t2999) { true } else { false } }) }).truthy() {
        let _ = f__count(Py::str("the drop factor (drops per mL)"), v_drop_factor.clone(), Py::Int(1i64))?;
        let _ = f__need({ let t3004 = v_drop_factor.clone(); Py::Bool({ let t3005 = Py::Int(100i64); if py::le(&t3004, &t3005)? { true } else { false } }) }, Py::str("the drop factor must be at most 100 drops per mL"))?;
        py::setitem(&v_out.clone(), &Py::str("drops_per_min"), py::mul(&py::mul(&v_q.clone(), &Py::Float(60000000.0))?, &v_drop_factor.clone())?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_larmor(mut v_B: Py, mut v_gamma_bar: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("the magnetic field"), Py::tuple(vec![v_B.clone()]))?;
    v_g = (if ({ let t3334 = v_gamma_bar.clone(); Py::Bool({ let t3335 = Py::None; if py::is(&t3334, &t3335) { true } else { false } }) }).truthy() { c_GAMMA_P_MHZ_T()? } else { v_gamma_bar.clone() });
    let _ = f__need({ let t3341 = f__isnum(v_g.clone())?; if !t3341.truthy() { t3341 } else { { let t3338 = Py::Int(0i64); Py::Bool({ let t3339 = py::abs(&v_g.clone())?; if py::lt(&t3338, &t3339)? { { let t3340 = Py::Int(100i64); if py::le(&t3339, &t3340)? { true } else { false } } } else { false } }) } } }, Py::str("the reduced gyromagnetic ratio must be given in MHz/T (|gamma/2 pi| <= 100)"))?;
    v_f = py::mul(&py::mul(&py::abs(&v_g.clone())?, &Py::Float(1000000.0))?, &v_B.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("f"), v_f.clone()), (Py::str("f_MHz"), py::truediv(&v_f.clone(), &Py::Float(1000000.0))?), (Py::str("omega"), py::mul(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_cable(mut v_Rm: Py, mut v_Ri: Py, mut v_d: Py, mut v_x: Py, mut v_V0: Py, mut v_end: Py, mut v_L: Py) -> R {
    let mut v_a: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_e2: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let mut v_q: Py = Py::None;
    let mut v_rinf: Py = Py::None;
    let _ = f__pos(Py::str("Rm, Ri and d"), Py::tuple(vec![v_Rm.clone(), v_Ri.clone(), v_d.clone()]))?;
    let _ = f__nonneg(Py::str("the position x"), Py::tuple(vec![v_x.clone()]))?;
    let _ = f__fin(Py::str("V0"), Py::tuple(vec![v_V0.clone()]))?;
    let _ = f__need({ let t164 = v_end.clone(); Py::Bool({ let t165 = Py::tuple(vec![Py::str("sealed"), Py::str("killed"), Py::str("infinite")]); if py::contains(&t165, &t164)? { true } else { false } }) }, Py::str("end is sealed, killed or infinite"))?;
    v_lam = py::math1("sqrt", &py::truediv(&py::mul(&v_Rm.clone(), &v_d.clone())?, &py::mul(&Py::Float(4.0), &v_Ri.clone())?)?)?;
    v_rinf = py::mul(&py::truediv(&py::mul(&Py::Float(4.0), &v_Ri.clone())?, &py::mul(&py::mul(&Py::Float(std::f64::consts::PI), &v_d.clone())?, &v_d.clone())?)?, &v_lam.clone())?;
    if ({ let t166 = v_end.clone(); Py::Bool({ let t167 = Py::str("infinite"); if py::eq(&t166, &t167) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t168 = v_L.clone(); Py::Bool({ let t169 = Py::None; if py::is(&t168, &t169) { true } else { false } }) }, Py::str("an infinite cable has no length L"))?;
        return Ok(f__out(Py::dict(vec![(Py::str("V_x"), py::mul(&v_V0.clone(), &py::math1("exp", &py::truediv(&py::neg(&v_x.clone())?, &v_lam.clone())?)?)?), (Py::str("R_in"), v_rinf.clone())]))?);
    }
    let _ = f__need({ let t182 = v_L.clone(); Py::Bool({ let t183 = Py::None; if !py::is(&t182, &t183) { true } else { false } }) }, Py::str("a finite cable needs its length L"))?;
    let _ = f__pos(Py::str("the length L"), Py::tuple(vec![v_L.clone()]))?;
    let _ = f__need({ let t184 = v_x.clone(); Py::Bool({ let t185 = v_L.clone(); if py::le(&t184, &t185)? { true } else { false } }) }, Py::str("x must lie on the cable (x <= L)"))?;
    let t186 = (Py::tuple(vec![py::truediv(&v_L.clone(), &v_lam.clone())?, py::math1("exp", &py::truediv(&py::neg(&v_x.clone())?, &v_lam.clone())?)?, py::math1("exp", &py::truediv(&py::neg(&py::sub(&py::mul(&Py::Float(2.0), &v_L.clone())?, &v_x.clone())?)?, &v_lam.clone())?)?])).items()?;
    if t186.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_a = t186[0].clone();
    v_e1 = t186[1].clone();
    v_e2 = t186[2].clone();
    v_q = py::math1("exp", &py::mul(&py::neg(&Py::Float(2.0))?, &v_a.clone())?)?;
    if ({ let t187 = v_end.clone(); Py::Bool({ let t188 = Py::str("sealed"); if py::eq(&t187, &t188) { true } else { false } }) }).truthy() {
        return Ok(f__out(Py::dict(vec![(Py::str("V_x"), py::truediv(&py::mul(&v_V0.clone(), &py::add(&v_e1.clone(), &v_e2.clone())?)?, &py::add(&Py::Float(1.0), &v_q.clone())?)?), (Py::str("R_in"), py::truediv(&py::mul(&v_rinf.clone(), &py::add(&Py::Float(1.0), &v_q.clone())?)?, &py::sub(&Py::Float(1.0), &v_q.clone())?)?)]))?);
    }
    return Ok(f__out(Py::dict(vec![(Py::str("V_x"), py::truediv(&py::mul(&v_V0.clone(), &py::sub(&v_e1.clone(), &v_e2.clone())?)?, &py::sub(&Py::Float(1.0), &v_q.clone())?)?), (Py::str("R_in"), py::truediv(&py::mul(&v_rinf.clone(), &py::sub(&Py::Float(1.0), &v_q.clone())?)?, &py::add(&Py::Float(1.0), &v_q.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_fhn(mut v_a: Py, mut v_b: Py, mut v_eps: Py, mut v_I: Py) -> R {
    let mut v_B: Py = Py::None;
    let mut v_det: Py = Py::None;
    let mut v_h: Py = Py::None;
    let mut v_tr: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_w: Py = Py::None;
    let _ = f__fin(Py::str("a, b, eps and I"), Py::tuple(vec![v_a.clone(), v_b.clone(), v_eps.clone(), v_I.clone()]))?;
    let _ = f__need({ let t2067 = Py::Int(0i64); Py::Bool({ let t2068 = v_b.clone(); if py::lt(&t2067, &t2068)? { { let t2069 = Py::Int(1i64); if py::lt(&t2068, &t2069)? { true } else { false } } } else { false } }) }, Py::str("this calculation needs 0 < b < 1 (then the fixed point is unique)"))?;
    let _ = f__pos(Py::str("eps"), Py::tuple(vec![v_eps.clone()]))?;
    v_h = { let v_I = v_I.clone(); let v_a = v_a.clone(); let v_b = v_b.clone(); let v_eps = v_eps.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_v: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::add(&py::sub(&py::sub(&v_v.clone(), &py::truediv(&py::pow(&v_v.clone(), &Py::Int(3i64))?, &Py::Float(3.0))?)?, &py::truediv(&py::add(&v_v.clone(), &v_a.clone())?, &v_b.clone())?)?, &v_I.clone())?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_B = Py::Float(1.0);
    while ({ let t2080 = { let t2076 = py::call(&v_h.clone(), &[py::neg(&v_B.clone())?])?; Py::Bool({ let t2077 = Py::Int(0i64); if py::le(&t2076, &t2077)? { true } else { false } }) }; if t2080.truthy() { t2080 } else { { let t2078 = py::call(&v_h.clone(), &[v_B.clone()])?; Py::Bool({ let t2079 = Py::Int(0i64); if py::ge(&t2078, &t2079)? { true } else { false } }) } } }).truthy() {
        v_B = py::mul(&v_B, &Py::Float(2.0))?;
    }
    v_v = f__bisect(v_h.clone(), py::neg(&v_B.clone())?, v_B.clone(), Py::Int(400i64))?;
    v_w = py::truediv(&py::add(&v_v.clone(), &v_a.clone())?, &v_b.clone())?;
    v_tr = py::sub(&py::sub(&Py::Float(1.0), &py::mul(&v_v.clone(), &v_v.clone())?)?, &py::mul(&v_eps.clone(), &v_b.clone())?)?;
    v_det = py::mul(&v_eps.clone(), &py::add(&py::sub(&Py::Float(1.0), &v_b.clone())?, &py::mul(&py::mul(&v_b.clone(), &v_v.clone())?, &v_v.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_star"), v_v.clone()), (Py::str("w_star"), v_w.clone()), (Py::str("trace"), v_tr.clone()), (Py::str("determinant"), v_det.clone()), (Py::str("stable"), Py::Bool(({ let t2119 = v_tr.clone(); Py::Bool({ let t2120 = Py::Int(0i64); if py::lt(&t2119, &t2120)? { true } else { false } }) }).truthy()))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_ghk_current(mut v_P: Py, mut v_z: Py, mut v_C_in: Py, mut v_C_out: Py, mut v_V: Py, mut v_T: Py) -> R {
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::str("the permeability P and the temperature T"), Py::tuple(vec![v_P.clone(), v_T.clone()]))?;
    let _ = f__need({ let t2238 = Py::Bool(matches!(v_z.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t2238.truthy() { t2238 } else { { let t2237 = { let t2233 = v_z.clone(); Py::Bool({ let t2234 = Py::Int(0i64); if py::ne(&t2233, &t2234) { true } else { false } }) }; if !t2237.truthy() { t2237 } else { { let t2235 = py::abs(&v_z.clone())?; Py::Bool({ let t2236 = Py::Int(3i64); if py::le(&t2235, &t2236)? { true } else { false } }) } } } } }, Py::str("z is a nonzero whole number from -3 to 3"))?;
    let _ = f__nonneg(Py::str("the concentrations"), Py::tuple(vec![v_C_in.clone(), v_C_out.clone()]))?;
    let _ = f__fin(Py::str("the voltage V"), Py::tuple(vec![v_V.clone()]))?;
    v_u = py::truediv(&py::mul(&py::mul(&v_z.clone(), &c_F_FARADAY()?)?, &v_V.clone())?, &py::mul(&c_R_GAS()?, &v_T.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("I"), py::mul(&py::mul(&py::mul(&v_P.clone(), &v_z.clone())?, &c_F_FARADAY()?)?, &py::sub(&py::mul(&v_C_in.clone(), &f__ghk_flux_g(v_u.clone())?)?, &py::mul(&v_C_out.clone(), &f__ghk_flux_g(py::neg(&v_u.clone())?)?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_ghk_voltage(mut v_T: Py, mut v_PK: Py, mut v_PNa: Py, mut v_PCl: Py, mut v_K_o: Py, mut v_K_i: Py, mut v_Na_o: Py, mut v_Na_i: Py, mut v_Cl_o: Py, mut v_Cl_i: Py) -> R {
    let mut v_den: Py = Py::None;
    let mut v_num: Py = Py::None;
    let _ = f__pos(Py::str("the temperature T and PK"), Py::tuple(vec![v_T.clone(), v_PK.clone()]))?;
    let _ = f__nonneg(Py::str("PNa and PCl"), Py::tuple(vec![v_PNa.clone(), v_PCl.clone()]))?;
    let _ = f__pos(Py::str("every concentration"), Py::tuple(vec![v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone()]))?;
    let t2315 = (f__ghk_terms(v_PK.clone(), v_PNa.clone(), v_PCl.clone(), v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone())?).items()?;
    if t2315.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t2315[0].clone();
    v_den = t2315[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("V_m"), py::mul(&py::truediv(&py::mul(&c_R_GAS()?, &v_T.clone())?, &c_F_FARADAY()?)?, &py::math1("log", &py::truediv(&v_num.clone(), &v_den.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_hh_gating(mut v_V: Py) -> R {
    let mut v_ah: Py = Py::None;
    let mut v_am: Py = Py::None;
    let mut v_an: Py = Py::None;
    let mut v_bh: Py = Py::None;
    let mut v_bm: Py = Py::None;
    let mut v_bn: Py = Py::None;
    let mut v_v: Py = Py::None;
    let _ = f__fin(Py::str("V"), Py::tuple(vec![v_V.clone()]))?;
    v_v = py::mul(&v_V.clone(), &Py::Float(1000.0))?;
    let _ = f__need({ let t2372 = py::neg(&Py::Float(150.0))?; Py::Bool({ let t2373 = v_v.clone(); if py::le(&t2372, &t2373)? { { let t2374 = Py::Float(100.0); if py::le(&t2373, &t2374)? { true } else { false } } } else { false } }) }, Py::str("V must lie between -150 mV and +100 mV"))?;
    let t2380 = (f__hh_rates(v_v.clone())?).items()?;
    if t2380.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2381 = (t2380[0].clone()).items()?;
    if t2381.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2381[0].clone();
    v_bm = t2381[1].clone();
    let t2382 = (t2380[1].clone()).items()?;
    if t2382.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2382[0].clone();
    v_bh = t2382[1].clone();
    let t2383 = (t2380[2].clone()).items()?;
    if t2383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2383[0].clone();
    v_bn = t2383[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("m_inf"), py::truediv(&v_am.clone(), &py::add(&v_am.clone(), &v_bm.clone())?)?), (Py::str("h_inf"), py::truediv(&v_ah.clone(), &py::add(&v_ah.clone(), &v_bh.clone())?)?), (Py::str("n_inf"), py::truediv(&v_an.clone(), &py::add(&v_an.clone(), &v_bn.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_hh_gk_clamp(mut v_gK_max: Py, mut v_V_hold: Py, mut v_V_step: Py, mut v_t: Py) -> R {
    let mut v__: Py = Py::None;
    let mut v_a0: Py = Py::None;
    let mut v_a1: Py = Py::None;
    let mut v_b0: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n0: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_vv: Py = Py::None;
    let _ = f__pos(Py::str("gK_max"), Py::tuple(vec![v_gK_max.clone()]))?;
    let _ = f__fin(Py::str("V_hold and V_step"), Py::tuple(vec![v_V_hold.clone(), v_V_step.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    for t2452 in py::iter(&(Py::tuple(vec![v_V_hold.clone(), v_V_step.clone()])))? {
        v_vv = t2452;
        let _ = f__need({ let t2453 = py::neg(&Py::Float(0.15))?; Py::Bool({ let t2454 = v_vv.clone(); if py::le(&t2453, &t2454)? { { let t2455 = Py::Float(0.1); if py::le(&t2454, &t2455)? { true } else { false } } } else { false } }) }, Py::str("potentials must lie between -150 mV and +100 mV"))?;
    }
    let t2461 = (f__hh_rates(py::mul(&v_V_hold.clone(), &Py::Float(1000.0))?)?).items()?;
    if t2461.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2462 = (t2461[0].clone()).items()?;
    if t2462.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2462[0].clone();
    v__ = t2462[1].clone();
    let t2463 = (t2461[1].clone()).items()?;
    if t2463.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2463[0].clone();
    v__ = t2463[1].clone();
    let t2464 = (t2461[2].clone()).items()?;
    if t2464.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t2464[0].clone();
    v_b0 = t2464[1].clone();
    let t2465 = (f__hh_rates(py::mul(&v_V_step.clone(), &Py::Float(1000.0))?)?).items()?;
    if t2465.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2466 = (t2465[0].clone()).items()?;
    if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2466[0].clone();
    v__ = t2466[1].clone();
    let t2467 = (t2465[1].clone()).items()?;
    if t2467.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2467[0].clone();
    v__ = t2467[1].clone();
    let t2468 = (t2465[2].clone()).items()?;
    if t2468.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2468[0].clone();
    v_b1 = t2468[1].clone();
    let t2469 = (Py::tuple(vec![py::truediv(&v_a0.clone(), &py::add(&v_a0.clone(), &v_b0.clone())?)?, py::truediv(&v_a1.clone(), &py::add(&v_a1.clone(), &v_b1.clone())?)?])).items()?;
    if t2469.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n0 = t2469[0].clone();
    v_n1 = t2469[1].clone();
    v_n = py::sub(&v_n1.clone(), &py::mul(&py::sub(&v_n1.clone(), &v_n0.clone())?, &py::math1("exp", &py::mul(&py::mul(&py::neg(&py::add(&v_a1.clone(), &v_b1.clone())?)?, &v_t.clone())?, &Py::Float(1000.0))?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("g_K"), py::mul(&v_gK_max.clone(), &py::pow(&v_n.clone(), &Py::Int(4i64))?)?), (Py::str("n"), v_n.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_hh_rest(mut v_gNa: Py, mut v_gK: Py, mut v_gL: Py, mut v_ENa: Py, mut v_EK: Py, mut v_EL: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__pos(Py::str("the conductances"), Py::tuple(vec![v_gNa.clone(), v_gK.clone(), v_gL.clone()]))?;
    let _ = f__fin(Py::str("the reversal potentials"), Py::tuple(vec![v_ENa.clone(), v_EK.clone(), v_EL.clone()]))?;
    let _ = f__need({ let t2532 = v_EK.clone(); Py::Bool({ let t2533 = v_EL.clone(); if py::lt(&t2532, &t2533)? { { let t2534 = v_ENa.clone(); if py::lt(&t2533, &t2534)? { true } else { false } } } else { false } }) }, Py::str("the reversal potentials must satisfy EK < EL < ENa"))?;
    v_v = f__hh_rest(v_gNa.clone(), v_gK.clone(), v_gL.clone(), v_ENa.clone(), v_EK.clone(), v_EL.clone(), Py::Int(3i64))?;
    let _ = f__need(f__hh_stable(py::mul(&v_v.clone(), &Py::Float(1000.0))?, v_gNa.clone(), v_gK.clone(), v_gL.clone(), v_ENa.clone(), v_EK.clone(), v_EL.clone())?, Py::str("the zero-current state is unstable (the model fires repetitively): there is no resting potential"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_rest"), v_v.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_lif_rate(mut v_tau_m: Py, mut v_R_m: Py, mut v_I: Py, mut v_V_rest: Py, mut v_V_th: Py, mut v_V_reset: Py, mut v_t_ref: Py) -> R {
    let mut v_Vinf: Py = Py::None;
    let mut v_isi: Py = Py::None;
    let _ = f__pos(Py::str("tau_m and R_m"), Py::tuple(vec![v_tau_m.clone(), v_R_m.clone()]))?;
    let _ = f__nonneg(Py::str("the refractory period t_ref"), Py::tuple(vec![v_t_ref.clone()]))?;
    let _ = f__fin(Py::str("I, V_rest, V_th and V_reset"), Py::tuple(vec![v_I.clone(), v_V_rest.clone(), v_V_th.clone(), v_V_reset.clone()]))?;
    let _ = f__need({ let t3410 = v_V_reset.clone(); Py::Bool({ let t3411 = v_V_th.clone(); if py::lt(&t3410, &t3411)? { true } else { false } }) }, Py::str("the reset potential must lie below the threshold"))?;
    v_Vinf = py::add(&v_V_rest.clone(), &py::mul(&v_R_m.clone(), &v_I.clone())?)?;
    let _ = f__need({ let t3412 = v_Vinf.clone(); Py::Bool({ let t3413 = v_V_th.clone(); if py::gt(&t3412, &t3413)? { true } else { false } }) }, Py::str("the input does not bring the membrane to threshold: the neuron does not fire"))?;
    v_isi = py::add(&v_t_ref.clone(), &py::mul(&v_tau_m.clone(), &py::math1("log", &py::truediv(&py::sub(&v_Vinf.clone(), &v_V_reset.clone())?, &py::sub(&v_Vinf.clone(), &v_V_th.clone())?)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("rate"), py::truediv(&Py::Float(1.0), &v_isi.clone())?), (Py::str("ISI"), v_isi.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_lif_voltage(mut v_tau_m: Py, mut v_R_m: Py, mut v_I: Py, mut v_V_rest: Py, mut v_V0: Py, mut v_t: Py) -> R {
    let mut v_vinf: Py = Py::None;
    let _ = f__pos(Py::str("tau_m and R_m"), Py::tuple(vec![v_tau_m.clone(), v_R_m.clone()]))?;
    let _ = f__fin(Py::str("I, V_rest and V0"), Py::tuple(vec![v_I.clone(), v_V_rest.clone(), v_V0.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_vinf = py::add(&v_V_rest.clone(), &py::mul(&v_R_m.clone(), &v_I.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V"), py::add(&v_vinf.clone(), &py::mul(&py::sub(&v_V0.clone(), &v_vinf.clone())?, &py::math1("exp", &py::truediv(&py::neg(&v_t.clone())?, &v_tau_m.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_psp(mut v_tau_m: Py, mut v_tau_s: Py, mut v_R_m: Py, mut v_I0: Py, mut v_t: Py) -> R {
    let mut v_V: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__pos(Py::str("tau_m, tau_s and R_m"), Py::tuple(vec![v_tau_m.clone(), v_tau_s.clone(), v_R_m.clone()]))?;
    let _ = f__fin(Py::str("I0"), Py::tuple(vec![v_I0.clone()]))?;
    let _ = f__need({ let t6094 = v_I0.clone(); Py::Bool({ let t6095 = Py::Int(0i64); if py::ne(&t6094, &t6095) { true } else { false } }) }, Py::str("the synaptic current amplitude I0 must not be zero"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t6102 = py::abs(&py::sub(&v_tau_s.clone(), &v_tau_m.clone())?)?; Py::Bool({ let t6103 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_tau_s.clone(), v_tau_m.clone()], true)?)?; if py::gt(&t6102, &t6103)? { true } else { false } }) }, Py::str("tau_s equals tau_m: the difference-of-exponentials form does not apply"))?;
    v_a = py::truediv(&py::mul(&py::mul(&v_R_m.clone(), &v_I0.clone())?, &v_tau_s.clone())?, &py::sub(&v_tau_s.clone(), &v_tau_m.clone())?)?;
    v_V = { let v_I0 = v_I0.clone(); let v_R_m = v_R_m.clone(); let v_a = v_a.clone(); let v_t = v_t.clone(); let v_tau_m = v_tau_m.clone(); let v_tau_s = v_tau_s.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&v_a.clone(), &py::sub(&py::math1("exp", &py::truediv(&py::neg(&v_s.clone())?, &v_tau_s.clone())?)?, &py::math1("exp", &py::truediv(&py::neg(&v_s.clone())?, &v_tau_m.clone())?)?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_tp = py::truediv(&py::mul(&py::mul(&v_tau_m.clone(), &v_tau_s.clone())?, &py::math1("log", &py::truediv(&v_tau_s.clone(), &v_tau_m.clone())?)?)?, &py::sub(&v_tau_s.clone(), &v_tau_m.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("V_t"), py::call(&v_V.clone(), &[v_t.clone()])?), (Py::str("t_peak"), v_tp.clone()), (Py::str("V_peak"), py::call(&v_V.clone(), &[v_tp.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_auc_iv_bolus(mut v_D: Py, mut v_CL: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the dose D, the clearance CL and the volume V"), Py::tuple(vec![v_D.clone(), v_CL.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_k = py::truediv(&v_CL.clone(), &v_V.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AUC_inf"), py::truediv(&v_D.clone(), &v_CL.clone())?), (Py::str("AUC_0_t"), py::mul(&py::truediv(&v_D.clone(), &v_CL.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_auc_trapezoid(mut v_t: Py, mut v_C: Py, mut v_method: Py) -> R {
    let mut v_c1: Py = Py::None;
    let mut v_c2: Py = Py::None;
    let mut v_dt: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_t1: Py = Py::None;
    let mut v_t2: Py = Py::None;
    let _ = f__need({ let t4420 = { let t4416 = py::len(&v_t.clone())?; Py::Bool({ let t4417 = py::len(&v_C.clone())?; if py::eq(&t4416, &t4417) { true } else { false } }) }; if !t4420.truthy() { t4420 } else { { let t4418 = py::len(&v_t.clone())?; Py::Bool({ let t4419 = Py::Int(2i64); if py::ge(&t4418, &t4419)? { true } else { false } }) } } }, Py::str("give at least two times and as many concentrations"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4421 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4422 = (t4421).items()?;
if t4422.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4422[0].clone();
v_b = t4422[1].clone();
out.push({ let t4423 = v_b.clone(); Py::Bool({ let t4424 = v_a.clone(); if py::gt(&t4423, &t4424)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the times must increase strictly"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4425 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4425;
out.push({ let t4426 = v_c.clone(); Py::Bool({ let t4427 = Py::Int(0i64); if py::ge(&t4426, &t4427)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("concentrations cannot be negative"))?;
    let _ = f__need({ let t4428 = v_method.clone(); Py::Bool({ let t4429 = Py::tuple(vec![Py::str("linear"), Py::str("linear_up_log_down")]); if py::contains(&t4429, &t4428)? { true } else { false } }) }, Py::str("method is linear or linear_up_log_down"))?;
    v_parts = Py::list(vec![]);
    for t4430 in py::iter(&(py::zip(&[py::zip(&[v_t.clone(), v_C.clone()])?, py::zip(&[py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?])?))? {
        let t4437 = (t4430).items()?;
        if t4437.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t4438 = (t4437[0].clone()).items()?;
        if t4438.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t1 = t4438[0].clone();
        v_c1 = t4438[1].clone();
        let t4439 = (t4437[1].clone()).items()?;
        if t4439.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t2 = t4439[0].clone();
        v_c2 = t4439[1].clone();
        v_dt = py::sub(&v_t2.clone(), &v_t1.clone())?;
        if ({ let t4436 = { let t4431 = v_method.clone(); Py::Bool({ let t4432 = Py::str("linear_up_log_down"); if py::eq(&t4431, &t4432) { true } else { false } }) }; if !t4436.truthy() { t4436 } else { { let t4433 = Py::Int(0i64); Py::Bool({ let t4434 = v_c2.clone(); if py::lt(&t4433, &t4434)? { { let t4435 = v_c1.clone(); if py::lt(&t4434, &t4435)? { true } else { false } } } else { false } }) } } }).truthy() {
            let _ = py::method(&v_parts.clone(), "append", &[py::truediv(&py::mul(&v_dt.clone(), &py::sub(&v_c1.clone(), &v_c2.clone())?)?, &py::math1("log", &py::truediv(&v_c1.clone(), &v_c2.clone())?)?)?])?;
        } else {
            let _ = py::method(&v_parts.clone(), "append", &[py::truediv(&py::mul(&v_dt.clone(), &py::add(&v_c1.clone(), &v_c2.clone())?)?, &Py::Float(2.0))?])?;
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("AUC"), py::fsum(&v_parts.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_bioavailability(mut v_D_iv: Py, mut v_t_iv: Py, mut v_C_iv: Py, mut v_D_oral: Py, mut v_t_oral: Py, mut v_C_oral: Py) -> R {
    let mut v_a_iv: Py = Py::None;
    let mut v_a_or: Py = Py::None;
    let _ = f__pos(Py::str("the doses"), Py::tuple(vec![v_D_iv.clone(), v_D_oral.clone()]))?;
    let t4513 = (Py::tuple(vec![f__trap(v_t_iv.clone(), v_C_iv.clone(), Py::str("IV profile"))?, f__trap(v_t_oral.clone(), v_C_oral.clone(), Py::str("oral profile"))?])).items()?;
    if t4513.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_iv = t4513[0].clone();
    v_a_or = t4513[1].clone();
    let _ = f__need({ let t4514 = v_a_iv.clone(); Py::Bool({ let t4515 = Py::Int(0i64); if py::gt(&t4514, &t4515)? { true } else { false } }) }, Py::str("the IV profile has no area"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("AUC_iv"), v_a_iv.clone()), (Py::str("AUC_oral"), v_a_or.clone()), (Py::str("F"), py::truediv(&py::truediv(&v_a_or.clone(), &v_D_oral.clone())?, &py::truediv(&v_a_iv.clone(), &v_D_iv.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_effect_site(mut v_D: Py, mut v_V: Py, mut v_k: Py, mut v_ke0: Py, mut v_t: Py) -> R {
    let mut v_ce: Py = Py::None;
    let mut v_tp: Py = Py::None;
    let _ = f__pos(Py::str("the dose D and the volume V"), Py::tuple(vec![v_D.clone(), v_V.clone()]))?;
    let _ = f__ka_k(v_ke0.clone(), v_k.clone())?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_ce = py::mul(&py::truediv(&py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &v_ke0.clone())?, &py::sub(&v_ke0.clone(), &v_k.clone())?)?, &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_ke0.clone())?, &v_t.clone())?)?)?)?;
    v_tp = py::truediv(&py::math1("log", &py::truediv(&v_ke0.clone(), &v_k.clone())?)?, &py::sub(&v_ke0.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("Ce"), v_ce.clone()), (Py::str("t_peak"), v_tp.clone()), (Py::str("Ce_peak"), py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_tp.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_half_life_fit(mut v_t: Py, mut v_C: Py) -> R {
    let mut v_b: Py = Py::None;
    let mut v_k: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let _ = f__need({ let t5755 = { let t5751 = py::len(&v_t.clone())?; Py::Bool({ let t5752 = py::len(&v_C.clone())?; if py::eq(&t5751, &t5752) { true } else { false } }) }; if !t5755.truthy() { t5755 } else { { let t5753 = py::len(&v_t.clone())?; Py::Bool({ let t5754 = Py::Int(3i64); if py::ge(&t5753, &t5754)? { true } else { false } }) } } }, Py::str("give at least three times and as many concentrations"))?;
    let t5770 = (f__loglin(v_t.clone(), v_C.clone(), Py::str("the concentrations"))?).items()?;
    if t5770.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t5770[0].clone();
    v_b = t5770[1].clone();
    let _ = f__need({ let t5771 = v_slope.clone(); Py::Bool({ let t5772 = Py::Int(0i64); if py::lt(&t5771, &t5772)? { true } else { false } }) }, Py::str("the concentrations do not decline (the fitted slope of ln C is not negative)"))?;
    v_k = py::neg(&v_slope.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("k"), v_k.clone()), (Py::str("t_half"), py::truediv(&c_LN2()?, &v_k.clone())?), (Py::str("C0"), py::math1("exp", &v_b.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_infusion(mut v_R0: Py, mut v_CL: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_css: Py = Py::None;
    let _ = f__pos(Py::str("the infusion rate R0, the clearance CL and the volume V"), Py::tuple(vec![v_R0.clone(), v_CL.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_css = py::truediv(&v_R0.clone(), &v_CL.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), py::mul(&v_css.clone(), &py::neg(&py::math1("expm1", &py::mul(&py::truediv(&py::neg(&v_CL.clone())?, &v_V.clone())?, &v_t.clone())?)?)?)?), (Py::str("C_ss"), v_css.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_intermittent_infusion(mut v_R0: Py, mut v_T_inf: Py, mut v_tau: Py, mut v_CL: Py, mut v_V: Py) -> R {
    let mut v_cmax: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("R0, T_inf, tau, CL and V"), Py::tuple(vec![v_R0.clone(), v_T_inf.clone(), v_tau.clone(), v_CL.clone(), v_V.clone()]))?;
    let _ = f__need({ let t4849 = v_T_inf.clone(); Py::Bool({ let t4850 = v_tau.clone(); if py::lt(&t4849, &t4850)? { true } else { false } }) }, Py::str("the infusion must be shorter than the dosing interval"))?;
    v_k = py::truediv(&v_CL.clone(), &v_V.clone())?;
    v_cmax = py::truediv(&py::mul(&py::truediv(&v_R0.clone(), &v_CL.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_T_inf.clone())?)?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_max_ss"), v_cmax.clone()), (Py::str("C_min_ss"), py::mul(&v_cmax.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_tau.clone(), &v_T_inf.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_iv_bolus(mut v_D: Py, mut v_V: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_f: Py = Py::None;
    let _ = f__pos(Py::str("the dose D, the volume V and the rate constant k"), Py::tuple(vec![v_D.clone(), v_V.clone(), v_k.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_f = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &v_f.clone())?), (Py::str("fraction_remaining"), v_f.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_loading_maintenance(mut v_C_target: Py, mut v_V: Py, mut v_CL: Py) -> R {
    let _ = f__pos(Py::str("the target concentration, the volume V and the clearance CL"), Py::tuple(vec![v_C_target.clone(), v_V.clone(), v_CL.clone()]))?;
    return Ok(f__out(Py::dict(vec![(Py::str("LD"), py::mul(&v_C_target.clone(), &v_V.clone())?), (Py::str("R0"), py::mul(&v_C_target.clone(), &v_CL.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_micro_from_macro(mut v_D: Py, mut v_A: Py, mut v_alpha: Py, mut v_B: Py, mut v_beta: Py) -> R {
    let mut v_V1: Py = Py::None;
    let mut v_k10: Py = Py::None;
    let mut v_k12: Py = Py::None;
    let mut v_k21: Py = Py::None;
    let _ = f__pos(Py::str("D, A, alpha, B and beta"), Py::tuple(vec![v_D.clone(), v_A.clone(), v_alpha.clone(), v_B.clone(), v_beta.clone()]))?;
    let _ = f__need({ let t5971 = v_alpha.clone(); Py::Bool({ let t5972 = py::mul(&v_beta.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t5971, &t5972)? { true } else { false } }) }, Py::str("alpha must exceed beta"))?;
    v_V1 = py::truediv(&v_D.clone(), &py::add(&v_A.clone(), &v_B.clone())?)?;
    v_k21 = py::truediv(&py::add(&py::mul(&v_A.clone(), &v_beta.clone())?, &py::mul(&v_B.clone(), &v_alpha.clone())?)?, &py::add(&v_A.clone(), &v_B.clone())?)?;
    v_k10 = py::truediv(&py::mul(&v_alpha.clone(), &v_beta.clone())?, &v_k21.clone())?;
    v_k12 = py::sub(&py::sub(&py::add(&v_alpha.clone(), &v_beta.clone())?, &v_k21.clone())?, &v_k10.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("V1"), v_V1.clone()), (Py::str("k10"), v_k10.clone()), (Py::str("k12"), v_k12.clone()), (Py::str("k21"), v_k21.clone()), (Py::str("CL"), py::mul(&v_k10.clone(), &v_V1.clone())?), (Py::str("Vss"), py::mul(&v_V1.clone(), &py::add(&Py::Float(1.0), &py::truediv(&v_k12.clone(), &v_k21.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_mm_concentration(mut v_C0: Py, mut v_Vmax: Py, mut v_Km: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_L: Py = Py::None;
    let mut v_vm: Py = Py::None;
    let _ = f__pos(Py::str("C0, Vmax, Km and V"), Py::tuple(vec![v_C0.clone(), v_Vmax.clone(), v_Km.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_vm = py::truediv(&v_Vmax.clone(), &v_V.clone())?;
    v_L = py::add(&py::math1("log", &py::truediv(&v_C0.clone(), &v_Km.clone())?)?, &py::truediv(&py::sub(&v_C0.clone(), &py::mul(&v_vm.clone(), &v_t.clone())?)?, &v_Km.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), py::mul(&v_Km.clone(), &f__w_log(v_L.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_mm_steady_state(mut v_R0: Py, mut v_Vmax: Py, mut v_Km: Py) -> R {
    let _ = f__pos(Py::str("R0, Vmax and Km"), Py::tuple(vec![v_R0.clone(), v_Vmax.clone(), v_Km.clone()]))?;
    let _ = f__need({ let t5112 = v_R0.clone(); Py::Bool({ let t5113 = v_Vmax.clone(); if py::lt(&t5112, &t5113)? { true } else { false } }) }, Py::str("no steady state: the dosing rate must be below Vmax"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_ss"), py::truediv(&py::mul(&v_Km.clone(), &v_R0.clone())?, &py::sub(&v_Vmax.clone(), &v_R0.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_mm_time(mut v_C0: Py, mut v_C: Py, mut v_Vmax: Py, mut v_Km: Py, mut v_V: Py) -> R {
    let _ = f__pos(Py::str("C0, C, Vmax, Km and V"), Py::tuple(vec![v_C0.clone(), v_C.clone(), v_Vmax.clone(), v_Km.clone(), v_V.clone()]))?;
    let _ = f__need({ let t5173 = v_C.clone(); Py::Bool({ let t5174 = v_C0.clone(); if py::lt(&t5173, &t5174)? { true } else { false } }) }, Py::str("the target concentration must be below C0 (the concentration only falls)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::truediv(&py::mul(&v_V.clone(), &py::add(&py::sub(&v_C0.clone(), &v_C.clone())?, &py::mul(&v_Km.clone(), &py::math1("log", &py::truediv(&v_C0.clone(), &v_C.clone())?)?)?)?)?, &v_Vmax.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_nca_iv(mut v_D: Py, mut v_t: Py, mut v_C: Py, mut v_n_terminal: Py) -> R {
    let mut v_auc: Py = Py::None;
    let mut v_auc_inf: Py = Py::None;
    let mut v_b: Py = Py::None;
    let mut v_cc: Py = Py::None;
    let mut v_cl: Py = Py::None;
    let mut v_dmg: Py = Py::None;
    let mut v_lz: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let mut v_tt: Py = Py::None;
    let _ = f__pos(Py::str("the dose D"), Py::tuple(vec![v_D.clone()]))?;
    v_n = py::len(&v_t.clone())?;
    let _ = f__need({ let t5238 = { let t5234 = py::len(&v_C.clone())?; Py::Bool({ let t5235 = v_n.clone(); if py::eq(&t5234, &t5235) { true } else { false } }) }; if !t5238.truthy() { t5238 } else { { let t5236 = v_n.clone(); Py::Bool({ let t5237 = Py::Int(4i64); if py::ge(&t5236, &t5237)? { true } else { false } }) } } }, Py::str("give at least four times and as many concentrations"))?;
    let _ = f__need({ let t5239 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t5240 = Py::Int(0i64); if py::eq(&t5239, &t5240) { true } else { false } }) }, Py::str("the first sample must be at t = 0 (for a bolus, the measured or back-extrapolated C0)"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5241 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5242 = (t5241).items()?;
if t5242.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5242[0].clone();
v_b = t5242[1].clone();
out.push({ let t5243 = v_b.clone(); Py::Bool({ let t5244 = v_a.clone(); if py::gt(&t5243, &t5244)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the times must increase strictly"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5245 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t5245;
out.push({ let t5246 = v_c.clone(); Py::Bool({ let t5247 = Py::Int(0i64); if py::ge(&t5246, &t5247)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("concentrations cannot be negative"))?;
    let _ = f__need({ let t5251 = Py::Bool(matches!(v_n_terminal.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5251.truthy() { t5251 } else { { let t5248 = Py::Int(3i64); Py::Bool({ let t5249 = v_n_terminal.clone(); if py::le(&t5248, &t5249)? { { let t5250 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::le(&t5249, &t5250)? { true } else { false } } } else { false } }) } } }, Py::str("n_terminal (points used for lambda_z) is 3 to n - 1"))?;
    let t5252 = (Py::tuple(vec![py::slice(&v_t.clone(), Some((py::neg(&v_n_terminal.clone())?).i()?), None, None)?, py::slice(&v_C.clone(), Some((py::neg(&v_n_terminal.clone())?).i()?), None, None)?])).items()?;
    if t5252.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tt = t5252[0].clone();
    v_cc = t5252[1].clone();
    let t5267 = (f__loglin(v_tt.clone(), v_cc.clone(), Py::str("the terminal concentrations"))?).items()?;
    if t5267.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t5267[0].clone();
    v_b = t5267[1].clone();
    let _ = f__need({ let t5268 = v_slope.clone(); Py::Bool({ let t5269 = Py::Int(0i64); if py::lt(&t5268, &t5269)? { true } else { false } }) }, Py::str("the terminal concentrations do not decline"))?;
    v_lz = py::neg(&v_slope.clone())?;
    v_auc = py::fsum(&{ let mut out = vec![]; for t5270 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_C.clone(), py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_c1: Py; let mut v_c2: Py; let mut v_t1: Py; let mut v_t2: Py; 
let t5271 = (t5270).items()?;
if t5271.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_t1 = t5271[0].clone();
v_t2 = t5271[1].clone();
v_c1 = t5271[2].clone();
v_c2 = t5271[3].clone();
out.push(py::truediv(&py::mul(&py::sub(&v_t2.clone(), &v_t1.clone())?, &py::add(&v_c1.clone(), &v_c2.clone())?)?, &Py::Float(2.0))?); } Py::list(out) })?;
    v_auc_inf = py::add(&v_auc.clone(), &py::truediv(&py::getitem(&v_C.clone(), &py::neg(&Py::Int(1i64))?)?, &v_lz.clone())?)?;
    v_dmg = py::mul(&v_D.clone(), &Py::Float(1000000.0))?;
    v_cl = py::truediv(&v_dmg.clone(), &v_auc_inf.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("AUC_last"), v_auc.clone()), (Py::str("lambda_z"), v_lz.clone()), (Py::str("t_half"), py::truediv(&c_LN2()?, &v_lz.clone())?), (Py::str("AUC_inf"), v_auc_inf.clone()), (Py::str("CL"), v_cl.clone()), (Py::str("Vz"), py::truediv(&v_cl.clone(), &v_lz.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_nth_dose(mut v_D: Py, mut v_V: Py, mut v_k: Py, mut v_tau: Py, mut v_n: Py, mut v_t: Py) -> R {
    let mut v_acc: Py = Py::None;
    let mut v_cmax: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the dose D, the volume V, the rate constant k and the interval tau"), Py::tuple(vec![v_D.clone(), v_V.clone(), v_k.clone(), v_tau.clone()]))?;
    let _ = f__need({ let t5334 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5334.truthy() { t5334 } else { { let t5331 = Py::Int(1i64); Py::Bool({ let t5332 = v_n.clone(); if py::le(&t5331, &t5332)? { { let t5333 = Py::Int(200i64); if py::le(&t5332, &t5333)? { true } else { false } } } else { false } }) } } }, Py::str("n (the dose number) is a whole number from 1 to 200"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t5341 = v_t.clone(); Py::Bool({ let t5342 = v_tau.clone(); if py::le(&t5341, &t5342)? { true } else { false } }) }, Py::str("t is the time after the nth dose and must lie within the dosing interval (t <= tau)"))?;
    v_r = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?;
    v_acc = py::truediv(&py::neg(&py::math1("expm1", &py::mul(&py::mul(&py::neg(&v_n.clone())?, &v_k.clone())?, &v_tau.clone())?)?)?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?)?)?;
    v_cmax = py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &v_acc.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_max_n"), v_cmax.clone()), (Py::str("C"), py::mul(&v_cmax.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_oral(mut v_F: Py, mut v_D: Py, mut v_V: Py, mut v_ka: Py, mut v_k: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_a: Py = Py::None;
    let mut v_auc: Py = Py::None;
    let _ = f__frac(Py::str("the bioavailability F"), v_F.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__pos(Py::str("the dose D and the volume V"), Py::tuple(vec![v_D.clone(), v_V.clone()]))?;
    let _ = f__ka_k(v_ka.clone(), v_k.clone())?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_a = py::truediv(&py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_D.clone())?, &v_V.clone())?, &v_ka.clone())?, &py::sub(&v_ka.clone(), &v_k.clone())?)?;
    v_C = py::mul(&v_a.clone(), &py::sub(&py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?, &py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?)?)?;
    v_auc = py::mul(&v_a.clone(), &py::add(&py::truediv(&py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?, &v_k.clone())?, &py::truediv(&py::math1("expm1", &py::mul(&py::neg(&v_ka.clone())?, &v_t.clone())?)?, &v_ka.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("AUC_0_t"), v_auc.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_oral_peak(mut v_F: Py, mut v_D: Py, mut v_V: Py, mut v_ka: Py, mut v_k: Py) -> R {
    let mut v_tm: Py = Py::None;
    let _ = f__frac(Py::str("the bioavailability F"), v_F.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__pos(Py::str("the dose D and the volume V"), Py::tuple(vec![v_D.clone(), v_V.clone()]))?;
    let _ = f__ka_k(v_ka.clone(), v_k.clone())?;
    v_tm = py::truediv(&py::math1("log", &py::truediv(&v_ka.clone(), &v_k.clone())?)?, &py::sub(&v_ka.clone(), &v_k.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("t_max"), v_tm.clone()), (Py::str("C_max"), py::mul(&py::truediv(&py::mul(&v_F.clone(), &v_D.clone())?, &v_V.clone())?, &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_tm.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_parallel_tube(mut v_Q: Py, mut v_fu: Py, mut v_CL_int: Py) -> R {
    let mut v_E: Py = Py::None;
    let _ = f__hep_inputs(v_Q.clone(), v_fu.clone(), v_CL_int.clone())?;
    v_E = py::neg(&py::math1("expm1", &py::truediv(&py::mul(&py::neg(&v_fu.clone())?, &v_CL_int.clone())?, &v_Q.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("CL_H"), py::mul(&v_Q.clone(), &v_E.clone())?), (Py::str("E"), v_E.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_short_infusion(mut v_R0: Py, mut v_T_inf: Py, mut v_CL: Py, mut v_V: Py, mut v_t: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_cend: Py = Py::None;
    let mut v_k: Py = Py::None;
    let _ = f__pos(Py::str("the infusion rate R0, the duration T_inf, the clearance CL and the volume V"), Py::tuple(vec![v_R0.clone(), v_T_inf.clone(), v_CL.clone(), v_V.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_k = py::truediv(&v_CL.clone(), &v_V.clone())?;
    v_cend = py::mul(&py::truediv(&v_R0.clone(), &v_CL.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_T_inf.clone())?)?)?)?;
    v_C = (if ({ let t5560 = v_t.clone(); Py::Bool({ let t5561 = v_T_inf.clone(); if py::le(&t5560, &t5561)? { true } else { false } }) }).truthy() { py::mul(&py::truediv(&v_R0.clone(), &v_CL.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)? } else { py::mul(&v_cend.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_t.clone(), &v_T_inf.clone())?)?)?)? });
    return Ok(f__out(Py::dict(vec![(Py::str("C"), v_C.clone()), (Py::str("C_end"), v_cend.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_ss_iv(mut v_D: Py, mut v_V: Py, mut v_k: Py, mut v_tau: Py) -> R {
    let mut v_R: Py = Py::None;
    let mut v_cmax: Py = Py::None;
    let mut v_r: Py = Py::None;
    let _ = f__pos(Py::str("the dose D, the volume V, the rate constant k and the interval tau"), Py::tuple(vec![v_D.clone(), v_V.clone(), v_k.clone(), v_tau.clone()]))?;
    v_r = py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?;
    v_R = py::truediv(&Py::Float(1.0), &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?)?)?;
    v_cmax = py::mul(&py::truediv(&v_D.clone(), &v_V.clone())?, &v_R.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_max_ss"), v_cmax.clone()), (Py::str("C_min_ss"), py::mul(&v_cmax.clone(), &v_r.clone())?), (Py::str("C_avg_ss"), py::truediv(&v_D.clone(), &py::mul(&py::mul(&v_V.clone(), &v_k.clone())?, &v_tau.clone())?)?), (Py::str("R_ac"), v_R.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_ss_oral(mut v_F: Py, mut v_D: Py, mut v_V: Py, mut v_ka: Py, mut v_k: Py, mut v_tau: Py) -> R {
    let mut v_a1: Py = Py::None;
    let mut v_b1: Py = Py::None;
    let mut v_css: Py = Py::None;
    let mut v_pre: Py = Py::None;
    let mut v_tm: Py = Py::None;
    let _ = f__frac(Py::str("the bioavailability F"), v_F.clone(), Py::Bool(true), Py::Bool(false))?;
    let _ = f__pos(Py::str("the dose D, the volume V and the interval tau"), Py::tuple(vec![v_D.clone(), v_V.clone(), v_tau.clone()]))?;
    let _ = f__ka_k(v_ka.clone(), v_k.clone())?;
    let t5694 = (Py::tuple(vec![py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?)?, py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_ka.clone())?, &v_tau.clone())?)?)?])).items()?;
    if t5694.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t5694[0].clone();
    v_b1 = t5694[1].clone();
    v_pre = py::truediv(&py::mul(&py::mul(&v_F.clone(), &v_D.clone())?, &v_ka.clone())?, &py::mul(&v_V.clone(), &py::sub(&v_ka.clone(), &v_k.clone())?)?)?;
    v_css = { let v_D = v_D.clone(); let v_F = v_F.clone(); let v_V = v_V.clone(); let v_a1 = v_a1.clone(); let v_b1 = v_b1.clone(); let v_k = v_k.clone(); let v_ka = v_ka.clone(); let v_pre = v_pre.clone(); let v_tau = v_tau.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_s: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::mul(&v_pre.clone(), &py::sub(&py::truediv(&py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &v_s.clone())?)?, &v_a1.clone())?, &py::truediv(&py::math1("exp", &py::mul(&py::neg(&v_ka.clone())?, &v_s.clone())?)?, &v_b1.clone())?)?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    v_tm = py::truediv(&py::math1("log", &py::truediv(&py::mul(&v_ka.clone(), &v_a1.clone())?, &py::mul(&v_k.clone(), &v_b1.clone())?)?)?, &py::sub(&v_ka.clone(), &v_k.clone())?)?;
    let _ = f__need({ let t5695 = Py::Int(0i64); Py::Bool({ let t5696 = v_tm.clone(); if py::lt(&t5695, &t5696)? { { let t5697 = v_tau.clone(); if py::lt(&t5696, &t5697)? { true } else { false } } } else { false } }) }, Py::str("the steady-state peak does not fall inside the dosing interval"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_max_ss"), py::call(&v_css.clone(), &[v_tm.clone()])?), (Py::str("t_max_ss"), v_tm.clone()), (Py::str("C_min_ss"), py::call(&v_css.clone(), &[v_tau.clone()])?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_time_to_fraction_ss(mut v_CL: Py, mut v_V: Py, mut v_f: Py) -> R {
    let _ = f__pos(Py::str("the clearance CL and the volume V"), Py::tuple(vec![v_CL.clone(), v_V.clone()]))?;
    let _ = f__frac(Py::str("the fraction f"), v_f.clone(), Py::Bool(true), Py::Bool(true))?;
    return Ok(f__out(Py::dict(vec![(Py::str("t"), py::truediv(&py::neg(&py::math1("log1p", &py::neg(&v_f.clone())?)?)?, &py::truediv(&v_CL.clone(), &v_V.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_two_comp(mut v_D: Py, mut v_V1: Py, mut v_k10: Py, mut v_k12: Py, mut v_k21: Py, mut v_t: Py) -> R {
    let mut v_A: Py = Py::None;
    let mut v_B: Py = Py::None;
    let mut v_al: Py = Py::None;
    let mut v_be: Py = Py::None;
    let mut v_c0: Py = Py::None;
    let _ = f__pos(Py::str("D, V1, k10, k12 and k21"), Py::tuple(vec![v_D.clone(), v_V1.clone(), v_k10.clone(), v_k12.clone(), v_k21.clone()]))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let t5909 = (f__macro(v_k10.clone(), v_k12.clone(), v_k21.clone())?).items()?;
    if t5909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_al = t5909[0].clone();
    v_be = t5909[1].clone();
    let _ = f__need({ let t5910 = py::sub(&v_al.clone(), &v_be.clone())?; Py::Bool({ let t5911 = py::mul(&Py::Float(1e-09), &v_al.clone())?; if py::gt(&t5910, &t5911)? { true } else { false } }) }, Py::str("the two exponents coincide (degenerate model)"))?;
    v_c0 = py::truediv(&v_D.clone(), &v_V1.clone())?;
    v_A = py::truediv(&py::mul(&v_c0.clone(), &py::sub(&v_al.clone(), &v_k21.clone())?)?, &py::sub(&v_al.clone(), &v_be.clone())?)?;
    v_B = py::truediv(&py::mul(&v_c0.clone(), &py::sub(&v_k21.clone(), &v_be.clone())?)?, &py::sub(&v_al.clone(), &v_be.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("alpha"), v_al.clone()), (Py::str("beta"), v_be.clone()), (Py::str("A"), v_A.clone()), (Py::str("B"), v_B.clone()), (Py::str("C"), py::add(&py::mul(&v_A.clone(), &py::math1("exp", &py::mul(&py::neg(&v_al.clone())?, &v_t.clone())?)?)?, &py::mul(&v_B.clone(), &py::math1("exp", &py::mul(&py::neg(&v_be.clone())?, &v_t.clone())?)?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_well_stirred(mut v_Q: Py, mut v_fu: Py, mut v_CL_int: Py) -> R {
    let mut v_cl: Py = Py::None;
    let _ = f__hep_inputs(v_Q.clone(), v_fu.clone(), v_CL_int.clone())?;
    v_cl = py::truediv(&py::mul(&py::mul(&v_Q.clone(), &v_fu.clone())?, &v_CL_int.clone())?, &py::add(&v_Q.clone(), &py::mul(&v_fu.clone(), &v_CL_int.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("CL_H"), v_cl.clone()), (Py::str("E"), py::truediv(&v_cl.clone(), &v_Q.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_reconstitution(mut v_powder: Py, mut v_diluent: Py, mut v_displacement: Py, mut v_D: Py) -> R {
    let mut v_C: Py = Py::None;
    let mut v_Vt: Py = Py::None;
    let mut v_disp: Py = Py::None;
    let mut v_out: Py = Py::None;
    let _ = f__pos(Py::str("the drug amount in the vial and the diluent volume"), Py::tuple(vec![v_powder.clone(), v_diluent.clone()]))?;
    v_disp = Py::Float(0.0);
    if ({ let t6163 = v_displacement.clone(); Py::Bool({ let t6164 = Py::None; if !py::is(&t6163, &t6164) { true } else { false } }) }).truthy() {
        let _ = f__nonneg(Py::str("the displacement volume"), Py::tuple(vec![v_displacement.clone()]))?;
        v_disp = v_displacement.clone();
    }
    v_Vt = py::add(&v_diluent.clone(), &v_disp.clone())?;
    v_C = py::truediv(&v_powder.clone(), &v_Vt.clone())?;
    v_out = Py::dict(vec![(Py::str("V_total"), v_Vt.clone()), (Py::str("V_total_mL"), py::mul(&v_Vt.clone(), &Py::Float(1000000.0))?), (Py::str("C"), v_C.clone()), (Py::str("C_mg_mL"), v_C.clone())]);
    if ({ let t6171 = v_D.clone(); Py::Bool({ let t6172 = Py::None; if !py::is(&t6171, &t6172) { true } else { false } }) }).truthy() {
        let _ = f__pos(Py::str("the ordered dose"), Py::tuple(vec![v_D.clone()]))?;
        let _ = f__need({ let t6173 = v_D.clone(); Py::Bool({ let t6174 = v_powder.clone(); if py::le(&t6173, &t6174)? { true } else { false } }) }, Py::str("the ordered dose cannot exceed the drug in the vial"))?;
        py::setitem(&v_out.clone(), &Py::str("V_give"), py::truediv(&v_D.clone(), &v_C.clone())?)?;
        py::setitem(&v_out.clone(), &Py::str("V_give_mL"), py::mul(&py::truediv(&v_D.clone(), &v_C.clone())?, &Py::Float(1000000.0))?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_snr_averaging(mut v_SNR1: Py, mut v_N: Py, mut v_SNR_target: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the single-acquisition SNR"), Py::tuple(vec![v_SNR1.clone()]))?;
    let _ = f__need({ let t6718 = { let t6720 = v_N.clone(); Py::Bool({ let t6721 = Py::None; if py::is(&t6720, &t6721) { true } else { false } }) }; Py::Bool({ let t6719 = { let t6722 = v_SNR_target.clone(); Py::Bool({ let t6723 = Py::None; if py::is(&t6722, &t6723) { true } else { false } }) }; if py::ne(&t6718, &t6719) { true } else { false } }) }, Py::str("give the number of averages N or a target SNR (one of them)"))?;
    if ({ let t6724 = v_N.clone(); Py::Bool({ let t6725 = Py::None; if !py::is(&t6724, &t6725) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6729 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6729.truthy() { t6729 } else { { let t6726 = Py::Int(1i64); Py::Bool({ let t6727 = v_N.clone(); if py::le(&t6726, &t6727)? { { let t6728 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::le(&t6727, &t6728)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number from 1 to 10 million"))?;
        return Ok(f__out(Py::dict(vec![(Py::str("SNR"), py::mul(&v_SNR1.clone(), &py::math1("sqrt", &v_N.clone())?)?), (Py::str("gain"), py::math1("sqrt", &v_N.clone())?), (Py::str("N"), py::float(&v_N.clone())?)]))?);
    }
    let _ = f__pos(Py::str("the target SNR"), Py::tuple(vec![v_SNR_target.clone()]))?;
    let _ = f__need({ let t6742 = py::truediv(&v_SNR_target.clone(), &v_SNR1.clone())?; Py::Bool({ let t6743 = Py::Int(3000i64); if py::le(&t6742, &t6743)? { true } else { false } }) }, Py::str("the target needs more than 10 million averages"))?;
    v_n = py::minmax(&[Py::Int(1i64), py::math1("ceil", &py::mul(&py::pow(&py::truediv(&v_SNR_target.clone(), &v_SNR1.clone())?, &Py::Int(2i64))?, &py::sub(&Py::Int(1i64), &Py::Float(1e-12))?)?)?], true)?;
    return Ok(f__out(Py::dict(vec![(Py::str("SNR"), py::mul(&v_SNR1.clone(), &py::math1("sqrt", &v_n.clone())?)?), (Py::str("gain"), py::math1("sqrt", &v_n.clone())?), (Py::str("N"), py::float(&v_n.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_cox(mut v_times: Py, mut v_events: Py, mut v_x: Py, mut v_level: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_I: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_U: Py = Py::None;
    let mut v__: Py = Py::None;
    let mut v_beta: Py = Py::None;
    let mut v_done: Py = Py::None;
    let mut v_halv: Py = Py::None;
    let mut v_ll: Py = Py::None;
    let mut v_m: Py = Py::None;
    let mut v_nI: Py = Py::None;
    let mut v_nU: Py = Py::None;
    let mut v_nb: Py = Py::None;
    let mut v_nll: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_step: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_xc: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t319 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t319[0].clone();
    v_E = t319[1].clone();
    let _ = f__need({ let t320 = py::len(&v_x.clone())?; Py::Bool({ let t321 = py::len(&v_T.clone())?; if py::eq(&t320, &t321) { true } else { false } }) }, Py::str("give one covariate value per subject"))?;
    for t322 in py::iter(&(v_x.clone()))? {
        v_v = t322;
        let _ = f__fin(Py::str("every covariate value"), Py::tuple(vec![v_v.clone()]))?;
    }
    let _ = f__need({ let t326 = py::sum(&v_E.clone(), None)?; Py::Bool({ let t327 = Py::Int(2i64); if py::ge(&t326, &t327)? { true } else { false } }) }, Py::str("at least two events are needed"))?;
    let _ = f__need({ let t328 = py::len(&py::set(&v_x.clone())?)?; Py::Bool({ let t329 = Py::Int(2i64); if py::ge(&t328, &t329)? { true } else { false } }) }, Py::str("the covariate does not vary"))?;
    v_z = f__level(v_level.clone())?;
    v_m = py::truediv(&py::fsum(&v_x.clone())?, &py::len(&v_x.clone())?)?;
    v_xc = { let mut out = vec![]; for t333 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t333;
out.push(py::sub(&v_v.clone(), &v_m.clone())?); } Py::list(out) };
    v_beta = Py::Float(0.0);
    let t363 = (f__cox_terms(v_T.clone(), v_E.clone(), v_xc.clone(), v_beta.clone())?).items()?;
    if t363.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_ll = t363[0].clone();
    v_U = t363[1].clone();
    v_I = t363[2].clone();
    for t364 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t364;
        let _ = f__need({ let t365 = v_I.clone(); Py::Bool({ let t366 = Py::Int(0i64); if py::gt(&t365, &t366)? { true } else { false } }) }, Py::str("the partial likelihood is flat: the hazard ratio is not estimable"))?;
        v_step = py::truediv(&v_U.clone(), &v_I.clone())?;
        v_nb = py::add(&v_beta.clone(), &v_step.clone())?;
        let t367 = (f__cox_terms(v_T.clone(), v_E.clone(), v_xc.clone(), v_nb.clone())?).items()?;
        if t367.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_nll = t367[0].clone();
        v_nU = t367[1].clone();
        v_nI = t367[2].clone();
        v_halv = Py::Int(0i64);
        while ({ let t373 = { let t369 = v_nll.clone(); Py::Bool({ let t370 = py::sub(&v_ll.clone(), &py::mul(&Py::Float(1e-12), &py::abs(&v_ll.clone())?)?)?; if py::lt(&t369, &t370)? { true } else { false } }) }; if !t373.truthy() { t373 } else { { let t371 = v_halv.clone(); Py::Bool({ let t372 = Py::Int(60i64); if py::lt(&t371, &t372)? { true } else { false } }) } } }).truthy() {
            v_step = py::truediv(&v_step, &Py::Float(2.0))?;
            v_nb = py::add(&v_beta.clone(), &v_step.clone())?;
            let t368 = (f__cox_terms(v_T.clone(), v_E.clone(), v_xc.clone(), v_nb.clone())?).items()?;
            if t368.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            v_nll = t368[0].clone();
            v_nU = t368[1].clone();
            v_nI = t368[2].clone();
            v_halv = py::iadd(&v_halv, &Py::Int(1i64))?;
        }
        let _ = f__need({ let t374 = py::abs(&v_nb.clone())?; Py::Bool({ let t375 = Py::Int(25i64); if py::lt(&t374, &t375)? { true } else { false } }) }, Py::str("the estimate diverges (monotone likelihood): the hazard ratio is not finite"))?;
        v_done = { let t376 = py::abs(&py::sub(&v_nb.clone(), &v_beta.clone())?)?; Py::Bool({ let t377 = py::mul(&Py::Float(1e-14), &py::minmax(&[Py::Float(1.0), py::abs(&v_nb.clone())?], true)?)?; if py::le(&t376, &t377)? { true } else { false } }) };
        let t378 = (Py::tuple(vec![v_nb.clone(), v_nll.clone(), v_nU.clone(), v_nI.clone()])).items()?;
        if t378.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_beta = t378[0].clone();
        v_ll = t378[1].clone();
        v_U = t378[2].clone();
        v_I = t378[3].clone();
        if (v_done.clone()).truthy() {
            break;
        }
    }
    v_se = py::truediv(&Py::Float(1.0), &py::math1("sqrt", &v_I.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("beta"), v_beta.clone()), (Py::str("HR"), py::math1("exp", &v_beta.clone())?), (Py::str("se"), v_se.clone()), (Py::str("HR_lower"), py::math1("exp", &py::sub(&v_beta.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?)?), (Py::str("HR_upper"), py::math1("exp", &py::add(&v_beta.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_exp_hr(mut v_times1: Py, mut v_events1: Py, mut v_times2: Py, mut v_events2: Py, mut v_level: Py) -> R {
    let mut v_E1: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_hr: Py = Py::None;
    let mut v_l1: Py = Py::None;
    let mut v_l2: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t1924 = (f__surv(v_times1.clone(), v_events1.clone(), Py::str("group 1"))?).items()?;
    if t1924.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t1924[0].clone();
    v_E1 = t1924[1].clone();
    let t1925 = (f__surv(v_times2.clone(), v_events2.clone(), Py::str("group 2"))?).items()?;
    if t1925.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T2 = t1925[0].clone();
    v_E2 = t1925[1].clone();
    let t1931 = (Py::tuple(vec![f__exp_rate(v_T1.clone(), v_E1.clone())?, f__exp_rate(v_T2.clone(), v_E2.clone())?])).items()?;
    if t1931.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t1931[0].clone();
    v_l2 = t1931[1].clone();
    v_z = f__level(v_level.clone())?;
    v_hr = py::truediv(&v_l1.clone(), &v_l2.clone())?;
    v_se = py::math1("sqrt", &py::add(&py::truediv(&Py::Float(1.0), &py::sum(&v_E1.clone(), None)?)?, &py::truediv(&Py::Float(1.0), &py::sum(&v_E2.clone(), None)?)?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("HR"), v_hr.clone()), (Py::str("lower"), py::mul(&v_hr.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?), (Py::str("upper"), py::mul(&v_hr.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_exponential(mut v_times: Py, mut v_events: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_lam: Py = Py::None;
    let t2005 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t2005.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2005[0].clone();
    v_E = t2005[1].clone();
    v_lam = f__exp_rate(v_T.clone(), v_E.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("hazard"), v_lam.clone()), (Py::str("mean"), py::truediv(&Py::Float(1.0), &v_lam.clone())?), (Py::str("median"), py::truediv(&c_LN2()?, &v_lam.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_km(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_curve: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_med: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_st: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t3241 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t3241.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3241[0].clone();
    v_E = t3241[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_curve = f__km_curve(v_T.clone(), v_E.clone())?;
    v_st = Py::Int(1i64);
    v_med = Py::None;
    for t3266 in py::iter(&(v_curve.clone()))? {
        let t3274 = (t3266).items()?;
        if t3274.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3274[0].clone();
        v_s = t3274[1].clone();
        v_n = t3274[2].clone();
        v_d = t3274[3].clone();
        if ({ let t3267 = v_u.clone(); Py::Bool({ let t3268 = v_t.clone(); if py::le(&t3267, &t3268)? { true } else { false } }) }).truthy() {
            v_st = v_s.clone();
        }
        if ({ let t3273 = { let t3269 = v_med.clone(); Py::Bool({ let t3270 = Py::None; if py::is(&t3269, &t3270) { true } else { false } }) }; if !t3273.truthy() { t3273 } else { { let t3271 = v_s.clone(); Py::Bool({ let t3272 = f_Fraction_half()?; if py::le(&t3271, &t3272)? { true } else { false } }) } } }).truthy() {
            v_med = v_u.clone();
        }
    }
    return Ok(f__out(Py::dict(vec![(Py::str("S_t"), py::float(&v_st.clone())?), (Py::str("median"), v_med.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_km_ci(mut v_times: Py, mut v_events: Py, mut v_t: Py, mut v_level: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_V: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_ls: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_se: Py = Py::None;
    let mut v_seen: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_z: Py = Py::None;
    let t3139 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t3139.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3139[0].clone();
    v_E = t3139[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_z = f__level(v_level.clone())?;
    let t3152 = (Py::tuple(vec![Py::Float(1.0), Py::Float(0.0), Py::Bool(false)])).items()?;
    if t3152.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t3152[0].clone();
    v_V = t3152[1].clone();
    v_seen = t3152[2].clone();
    for t3168 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t3173 = (t3168).items()?;
        if t3173.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t3173[0].clone();
        v_n = t3173[1].clone();
        v_d = t3173[2].clone();
        if ({ let t3169 = v_u.clone(); Py::Bool({ let t3170 = v_t.clone(); if py::gt(&t3169, &t3170)? { true } else { false } }) }).truthy() {
            break;
        }
        let _ = f__need({ let t3171 = v_d.clone(); Py::Bool({ let t3172 = v_n.clone(); if py::lt(&t3171, &t3172)? { true } else { false } }) }, Py::str("the survival estimate reaches 0 by t: the log(-log) interval is undefined"))?;
        v_s = py::mul(&v_s, &py::truediv(&py::sub(&v_n.clone(), &v_d.clone())?, &v_n.clone())?)?;
        v_V = py::iadd(&v_V, &py::truediv(&v_d.clone(), &py::mul(&v_n.clone(), &py::sub(&v_n.clone(), &v_d.clone())?)?)?)?;
        v_seen = Py::Bool(true);
    }
    let _ = f__need(v_seen.clone(), Py::str("no event at or before t: S(t) = 1 and the interval is undefined"))?;
    v_ls = py::math1("log", &v_s.clone())?;
    v_se = py::truediv(&py::math1("sqrt", &v_V.clone())?, &py::abs(&v_ls.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S_t"), v_s.clone()), (Py::str("lower"), py::pow(&v_s.clone(), &py::math1("exp", &py::mul(&v_z.clone(), &v_se.clone())?)?)?), (Py::str("upper"), py::pow(&v_s.clone(), &py::math1("exp", &py::mul(&py::neg(&v_z.clone())?, &v_se.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_logrank(mut v_times1: Py, mut v_events1: Py, mut v_times2: Py, mut v_events2: Py) -> R {
    let mut v_E1: Py = Py::None;
    let mut v_E2: Py = Py::None;
    let mut v_T1: Py = Py::None;
    let mut v_T2: Py = Py::None;
    let mut v_chi2: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_d1: Py = Py::None;
    let mut v_e1: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_n1: Py = Py::None;
    let mut v_n2: Py = Py::None;
    let mut v_o1: Py = Py::None;
    let mut v_u: Py = Py::None;
    let mut v_v: Py = Py::None;
    let t3549 = (f__surv(v_times1.clone(), v_events1.clone(), Py::str("group 1"))?).items()?;
    if t3549.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t3549[0].clone();
    v_E1 = t3549[1].clone();
    let t3550 = (f__surv(v_times2.clone(), v_events2.clone(), Py::str("group 2"))?).items()?;
    if t3550.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T2 = t3550[0].clone();
    v_E2 = t3550[1].clone();
    let t3551 = Py::Float(0.0);
    v_o1 = t3551.clone();
    v_e1 = t3551.clone();
    v_v = t3551.clone();
    for t3556 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t3552 in py::iter(&(py::zip(&[py::add(&v_T1.clone(), &v_T2.clone())?, py::add(&v_E1.clone(), &v_E2.clone())?])?))? { let mut v_e: Py; let mut v_t: Py; 
let t3553 = (t3552).items()?;
if t3553.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t3553[0].clone();
v_e = t3553[1].clone();
if ({ let t3554 = v_e.clone(); Py::Bool({ let t3555 = Py::Int(1i64); if py::eq(&t3554, &t3555) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t3556;
        let t3563 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t3557 in py::iter(&(v_T1.clone()))? { let mut v_t: Py; 
v_t = t3557;
if ({ let t3558 = v_t.clone(); Py::Bool({ let t3559 = v_u.clone(); if py::ge(&t3558, &t3559)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t3560 in py::iter(&(v_T2.clone()))? { let mut v_t: Py; 
v_t = t3560;
if ({ let t3561 = v_t.clone(); Py::Bool({ let t3562 = v_u.clone(); if py::ge(&t3561, &t3562)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])).items()?;
        if t3563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n1 = t3563[0].clone();
        v_n2 = t3563[1].clone();
        v_d1 = py::sum(&{ let mut out = vec![]; for t3564 in py::iter(&(py::zip(&[v_T1.clone(), v_E1.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t3565 = (t3564).items()?;
if t3565.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t3565[0].clone();
v_e = t3565[1].clone();
if ({ let t3570 = { let t3566 = v_t.clone(); Py::Bool({ let t3567 = v_u.clone(); if py::eq(&t3566, &t3567) { true } else { false } }) }; if !t3570.truthy() { t3570 } else { { let t3568 = v_e.clone(); Py::Bool({ let t3569 = Py::Int(1i64); if py::eq(&t3568, &t3569) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_d = py::add(&v_d1.clone(), &py::sum(&{ let mut out = vec![]; for t3571 in py::iter(&(py::zip(&[v_T2.clone(), v_E2.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t3572 = (t3571).items()?;
if t3572.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t3572[0].clone();
v_e = t3572[1].clone();
if ({ let t3577 = { let t3573 = v_t.clone(); Py::Bool({ let t3574 = v_u.clone(); if py::eq(&t3573, &t3574) { true } else { false } }) }; if !t3577.truthy() { t3577 } else { { let t3575 = v_e.clone(); Py::Bool({ let t3576 = Py::Int(1i64); if py::eq(&t3575, &t3576) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?;
        v_n = py::add(&v_n1.clone(), &v_n2.clone())?;
        v_o1 = py::iadd(&v_o1, &v_d1.clone())?;
        v_e1 = py::iadd(&v_e1, &py::truediv(&py::mul(&v_d.clone(), &v_n1.clone())?, &v_n.clone())?)?;
        if ({ let t3578 = v_n.clone(); Py::Bool({ let t3579 = Py::Int(1i64); if py::gt(&t3578, &t3579)? { true } else { false } }) }).truthy() {
            v_v = py::iadd(&v_v, &py::truediv(&py::mul(&py::mul(&py::mul(&v_d.clone(), &py::truediv(&v_n1.clone(), &v_n.clone())?)?, &py::truediv(&v_n2.clone(), &v_n.clone())?)?, &py::sub(&v_n.clone(), &v_d.clone())?)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
        }
    }
    let _ = f__need({ let t3580 = v_v.clone(); Py::Bool({ let t3581 = Py::Int(0i64); if py::gt(&t3580, &t3581)? { true } else { false } }) }, Py::str("the log-rank variance is zero (no informative event times)"))?;
    v_chi2 = py::truediv(&py::pow(&py::sub(&v_o1.clone(), &v_e1.clone())?, &Py::Int(2i64))?, &v_v.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("chi_square"), v_chi2.clone()), (Py::str("p_value"), py::math1("erfc", &py::math1("sqrt", &py::truediv(&v_chi2.clone(), &Py::Float(2.0))?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_nelson_aalen(mut v_times: Py, mut v_events: Py, mut v_t: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_H: Py = Py::None;
    let mut v_T: Py = Py::None;
    let t4135 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t4135.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t4135[0].clone();
    v_E = t4135[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_H = py::fsum(&{ let mut out = vec![]; for t4157 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? { let mut v_d: Py; let mut v_n: Py; let mut v_u: Py; 
let t4158 = (t4157).items()?;
if t4158.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t4158[0].clone();
v_n = t4158[1].clone();
v_d = t4158[2].clone();
if ({ let t4159 = v_u.clone(); Py::Bool({ let t4160 = v_t.clone(); if py::le(&t4159, &t4160)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_d.clone(), &v_n.clone())?); }} Py::list(out) })?;
    return Ok(f__out(Py::dict(vec![(Py::str("H_t"), v_H.clone())]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_sv_rmst(mut v_times: Py, mut v_events: Py, mut v_tau: Py) -> R {
    let mut v_E: Py = Py::None;
    let mut v_T: Py = Py::None;
    let mut v_area: Py = Py::None;
    let mut v_d: Py = Py::None;
    let mut v_last: Py = Py::None;
    let mut v_n: Py = Py::None;
    let mut v_parts: Py = Py::None;
    let mut v_s: Py = Py::None;
    let mut v_u: Py = Py::None;
    let t6398 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t6398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t6398[0].clone();
    v_E = t6398[1].clone();
    let _ = f__pos(Py::str("tau"), Py::tuple(vec![v_tau.clone()]))?;
    let t6405 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0), Py::Float(0.0)])).items()?;
    if t6405.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_area = t6405[0].clone();
    v_s = t6405[1].clone();
    v_last = t6405[2].clone();
    v_parts = Py::list(vec![]);
    for t6421 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t6424 = (t6421).items()?;
        if t6424.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t6424[0].clone();
        v_n = t6424[1].clone();
        v_d = t6424[2].clone();
        if ({ let t6422 = v_u.clone(); Py::Bool({ let t6423 = v_tau.clone(); if py::ge(&t6422, &t6423)? { true } else { false } }) }).truthy() {
            break;
        }
        let _ = py::method(&v_parts.clone(), "append", &[py::mul(&v_s.clone(), &py::sub(&v_u.clone(), &v_last.clone())?)?])?;
        v_s = py::mul(&v_s, &py::truediv(&py::sub(&v_n.clone(), &v_d.clone())?, &v_n.clone())?)?;
        v_last = v_u.clone();
    }
    let _ = py::method(&v_parts.clone(), "append", &[py::mul(&v_s.clone(), &py::sub(&v_tau.clone(), &v_last.clone())?)?])?;
    return Ok(f__out(Py::dict(vec![(Py::str("RMST"), py::fsum(&v_parts.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_us_wavelength(mut v_f: Py, mut v_c: Py) -> R {
    let mut v_lam: Py = Py::None;
    let _ = f__pos(Py::str("the frequency and the speed of sound"), Py::tuple(vec![v_f.clone(), v_c.clone()]))?;
    v_lam = py::truediv(&v_c.clone(), &v_f.clone())?;
    return Ok(f__out(Py::dict(vec![(Py::str("wavelength"), v_lam.clone()), (Py::str("wavelength_mm"), py::mul(&v_lam.clone(), &Py::Float(1000.0))?), (Py::str("period"), py::truediv(&Py::Float(1.0), &v_f.clone())?), (Py::str("k"), py::truediv(&py::mul(&Py::Int(2i64), &Py::Float(std::f64::consts::PI))?, &v_lam.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_vaccine_efficacy(mut v_cases_v: Py, mut v_n_v: Py, mut v_cases_u: Py, mut v_n_u: Py) -> R {
    let mut v_aru: Py = Py::None;
    let mut v_arv: Py = Py::None;
    let mut v_out: Py = Py::None;
    let mut v_rr: Py = Py::None;
    let _ = f__count(Py::str("the vaccinated cases"), v_cases_v.clone(), Py::Int(0i64))?;
    let _ = f__count(Py::str("the vaccinated group size"), v_n_v.clone(), Py::Int(1i64))?;
    let _ = f__count(Py::str("the unvaccinated cases"), v_cases_u.clone(), Py::Int(1i64))?;
    let _ = f__count(Py::str("the unvaccinated group size"), v_n_u.clone(), Py::Int(1i64))?;
    let _ = f__need({ let t6971 = { let t6967 = v_cases_v.clone(); Py::Bool({ let t6968 = v_n_v.clone(); if py::le(&t6967, &t6968)? { true } else { false } }) }; if !t6971.truthy() { t6971 } else { { let t6969 = v_cases_u.clone(); Py::Bool({ let t6970 = v_n_u.clone(); if py::le(&t6969, &t6970)? { true } else { false } }) } } }, Py::str("the cases cannot exceed the group size"))?;
    v_arv = py::truediv(&v_cases_v.clone(), &v_n_v.clone())?;
    v_aru = py::truediv(&v_cases_u.clone(), &v_n_u.clone())?;
    v_rr = py::truediv(&v_arv.clone(), &v_aru.clone())?;
    v_out = Py::dict(vec![(Py::str("ARV"), v_arv.clone()), (Py::str("ARU"), v_aru.clone()), (Py::str("RR"), v_rr.clone()), (Py::str("VE"), py::sub(&Py::Float(1.0), &v_rr.clone())?), (Py::str("VE_percent"), py::mul(&py::sub(&Py::Float(1.0), &v_rr.clone())?, &Py::Float(100.0))?), (Py::str("ARR"), py::sub(&v_aru.clone(), &v_arv.clone())?)]);
    if ({ let t6972 = v_aru.clone(); Py::Bool({ let t6973 = v_arv.clone(); if py::gt(&t6972, &t6973)? { true } else { false } }) }).truthy() {
        py::setitem(&v_out.clone(), &Py::str("NNV"), py::truediv(&Py::Float(1.0), &py::sub(&v_aru.clone(), &v_arv.clone())?)?)?;
    }
    return Ok(f__out(v_out.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1025(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1026 = v_key.clone(); Py::Bool({ let t1027 = v_memo.clone(); if !py::contains(&t1027, &t1026)? { true } else { false } }) }).truthy() {
        let t1029: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1028 = v_inputs.clone(); f_bm_lever(py::getitem(&t1028, &Py::str("W"))?, py::getitem(&t1028, &Py::str("d_load"))?, py::getitem(&t1028, &Py::str("W_arm"))?, py::getitem(&t1028, &Py::str("d_arm"))?, py::getitem(&t1028, &Py::str("d_muscle"))?, py::getitem(&t1028, &Py::str("theta"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1029 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1030 = py::len(&v_memo.clone())?; Py::Bool({ let t1031 = Py::Int(64i64); if py::gt(&t1030, &t1031)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1032 = v_want.clone(); Py::Bool({ let t1033 = Py::None; if py::is(&t1032, &t1033) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1034 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1065 = (t1034).items()?;
        if t1065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1065[0].clone();
        v_v = t1065[1].clone();
        if ({ let t1035 = v_k.clone(); Py::Bool({ let t1036 = v_outputs.clone(); if !py::contains(&t1036, &t1035)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_108(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t109 = v_key.clone(); Py::Bool({ let t110 = v_memo.clone(); if !py::contains(&t110, &t109)? { true } else { false } }) }).truthy() {
        let t112: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t111 = v_inputs.clone(); f_bmr(py::getitem(&t111, &Py::str("m"))?, py::getitem(&t111, &Py::str("h"))?, py::getitem(&t111, &Py::str("age"))?, py::getitem(&t111, &Py::str("sex"))?, py::getitem(&t111, &Py::str("equation"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t112 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t113 = py::len(&v_memo.clone())?; Py::Bool({ let t114 = Py::Int(64i64); if py::gt(&t113, &t114)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t115 = v_want.clone(); Py::Bool({ let t116 = Py::None; if py::is(&t115, &t116) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t117 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t148 = (t117).items()?;
        if t148.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t148[0].clone();
        v_v = t148[1].clone();
        if ({ let t118 = v_k.clone(); Py::Bool({ let t119 = v_outputs.clone(); if !py::contains(&t119, &t118)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1105(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1106 = v_key.clone(); Py::Bool({ let t1107 = v_memo.clone(); if !py::contains(&t1107, &t1106)? { true } else { false } }) }).truthy() {
        let t1109: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1108 = v_inputs.clone(); f_epi_basic_rates(py::getitem(&t1108, &Py::str("cases"))?, py::getitem(&t1108, &Py::str("population"))?, py::method(&t1108, "get", &[Py::str("prevalent"), Py::None])?, py::method(&t1108, "get", &[Py::str("deaths"), Py::None])?, py::method(&t1108, "get", &[Py::str("period"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1109 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1110 = py::len(&v_memo.clone())?; Py::Bool({ let t1111 = Py::Int(64i64); if py::gt(&t1110, &t1111)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1112 = v_want.clone(); Py::Bool({ let t1113 = Py::None; if py::is(&t1112, &t1113) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1114 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1145 = (t1114).items()?;
        if t1145.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1145[0].clone();
        v_v = t1145[1].clone();
        if ({ let t1115 = v_k.clone(); Py::Bool({ let t1116 = v_outputs.clone(); if !py::contains(&t1116, &t1115)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1185(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1186 = v_key.clone(); Py::Bool({ let t1187 = v_memo.clone(); if !py::contains(&t1187, &t1186)? { true } else { false } }) }).truthy() {
        let t1189: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1188 = v_inputs.clone(); f_epi_herd_immunity(py::getitem(&t1188, &Py::str("R0"))?, py::method(&t1188, "get", &[Py::str("efficacy"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1189 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1190 = py::len(&v_memo.clone())?; Py::Bool({ let t1191 = Py::Int(64i64); if py::gt(&t1190, &t1191)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1192 = v_want.clone(); Py::Bool({ let t1193 = Py::None; if py::is(&t1192, &t1193) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1194 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1225 = (t1194).items()?;
        if t1225.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1225[0].clone();
        v_v = t1225[1].clone();
        if ({ let t1195 = v_k.clone(); Py::Bool({ let t1196 = v_outputs.clone(); if !py::contains(&t1196, &t1195)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1249(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1250 = v_key.clone(); Py::Bool({ let t1251 = v_memo.clone(); if !py::contains(&t1251, &t1250)? { true } else { false } }) }).truthy() {
        let t1253: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1252 = v_inputs.clone(); f_epi_r_from_growth(py::getitem(&t1252, &Py::str("r"))?, py::getitem(&t1252, &Py::str("Tg_mean"))?, py::getitem(&t1252, &Py::str("Tg_sd"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1253 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1254 = py::len(&v_memo.clone())?; Py::Bool({ let t1255 = Py::Int(64i64); if py::gt(&t1254, &t1255)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1256 = v_want.clone(); Py::Bool({ let t1257 = Py::None; if py::is(&t1256, &t1257) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1258 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1289 = (t1258).items()?;
        if t1289.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1289[0].clone();
        v_v = t1289[1].clone();
        if ({ let t1259 = v_k.clone(); Py::Bool({ let t1260 = v_outputs.clone(); if !py::contains(&t1260, &t1259)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1335(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1336 = v_key.clone(); Py::Bool({ let t1337 = v_memo.clone(); if !py::contains(&t1337, &t1336)? { true } else { false } }) }).truthy() {
        let t1339: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1338 = v_inputs.clone(); f_epi_reed_frost(py::getitem(&t1338, &Py::str("n"))?, py::getitem(&t1338, &Py::str("m"))?, py::getitem(&t1338, &Py::str("p"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1339 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1340 = py::len(&v_memo.clone())?; Py::Bool({ let t1341 = Py::Int(64i64); if py::gt(&t1340, &t1341)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1342 = v_want.clone(); Py::Bool({ let t1343 = Py::None; if py::is(&t1342, &t1343) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1344 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1375 = (t1344).items()?;
        if t1375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1375[0].clone();
        v_v = t1375[1].clone();
        if ({ let t1345 = v_k.clone(); Py::Bool({ let t1346 = v_outputs.clone(); if !py::contains(&t1346, &t1345)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1436(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1437 = v_key.clone(); Py::Bool({ let t1438 = v_memo.clone(); if !py::contains(&t1438, &t1437)? { true } else { false } }) }).truthy() {
        let t1440: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1439 = v_inputs.clone(); f_epi_seir_final_size(py::getitem(&t1439, &Py::str("R0"))?, py::getitem(&t1439, &Py::str("s0"))?, py::getitem(&t1439, &Py::str("e0"))?, py::getitem(&t1439, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1440 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1441 = py::len(&v_memo.clone())?; Py::Bool({ let t1442 = Py::Int(64i64); if py::gt(&t1441, &t1442)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1443 = v_want.clone(); Py::Bool({ let t1444 = Py::None; if py::is(&t1443, &t1444) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1445 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1476 = (t1445).items()?;
        if t1476.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1476[0].clone();
        v_v = t1476[1].clone();
        if ({ let t1446 = v_k.clone(); Py::Bool({ let t1447 = v_outputs.clone(); if !py::contains(&t1447, &t1446)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1497(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1498 = v_key.clone(); Py::Bool({ let t1499 = v_memo.clone(); if !py::contains(&t1499, &t1498)? { true } else { false } }) }).truthy() {
        let t1501: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1500 = v_inputs.clone(); f_epi_seir_growth(py::getitem(&t1500, &Py::str("R0"))?, py::getitem(&t1500, &Py::str("sigma"))?, py::getitem(&t1500, &Py::str("gamma"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1501 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1502 = py::len(&v_memo.clone())?; Py::Bool({ let t1503 = Py::Int(64i64); if py::gt(&t1502, &t1503)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1504 = v_want.clone(); Py::Bool({ let t1505 = Py::None; if py::is(&t1504, &t1505) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1506 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1537 = (t1506).items()?;
        if t1537.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1537[0].clone();
        v_v = t1537[1].clone();
        if ({ let t1507 = v_k.clone(); Py::Bool({ let t1508 = v_outputs.clone(); if !py::contains(&t1508, &t1507)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1612(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1613 = v_key.clone(); Py::Bool({ let t1614 = v_memo.clone(); if !py::contains(&t1614, &t1613)? { true } else { false } }) }).truthy() {
        let t1616: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1615 = v_inputs.clone(); f_epi_seir_peak(py::getitem(&t1615, &Py::str("beta"))?, py::getitem(&t1615, &Py::str("sigma"))?, py::getitem(&t1615, &Py::str("gamma"))?, py::getitem(&t1615, &Py::str("s0"))?, py::getitem(&t1615, &Py::str("e0"))?, py::getitem(&t1615, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1616 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1617 = py::len(&v_memo.clone())?; Py::Bool({ let t1618 = Py::Int(64i64); if py::gt(&t1617, &t1618)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1619 = v_want.clone(); Py::Bool({ let t1620 = Py::None; if py::is(&t1619, &t1620) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1621 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1652 = (t1621).items()?;
        if t1652.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1652[0].clone();
        v_v = t1652[1].clone();
        if ({ let t1622 = v_k.clone(); Py::Bool({ let t1623 = v_outputs.clone(); if !py::contains(&t1623, &t1622)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1711(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1712 = v_key.clone(); Py::Bool({ let t1713 = v_memo.clone(); if !py::contains(&t1713, &t1712)? { true } else { false } }) }).truthy() {
        let t1715: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1714 = v_inputs.clone(); f_epi_sir_final_size(py::getitem(&t1714, &Py::str("R0"))?, py::getitem(&t1714, &Py::str("s0"))?, py::getitem(&t1714, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1715 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1716 = py::len(&v_memo.clone())?; Py::Bool({ let t1717 = Py::Int(64i64); if py::gt(&t1716, &t1717)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1718 = v_want.clone(); Py::Bool({ let t1719 = Py::None; if py::is(&t1718, &t1719) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1720 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1751 = (t1720).items()?;
        if t1751.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1751[0].clone();
        v_v = t1751[1].clone();
        if ({ let t1721 = v_k.clone(); Py::Bool({ let t1722 = v_outputs.clone(); if !py::contains(&t1722, &t1721)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1790(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1791 = v_key.clone(); Py::Bool({ let t1792 = v_memo.clone(); if !py::contains(&t1792, &t1791)? { true } else { false } }) }).truthy() {
        let t1794: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1793 = v_inputs.clone(); f_epi_sir_peak(py::getitem(&t1793, &Py::str("beta"))?, py::getitem(&t1793, &Py::str("gamma"))?, py::getitem(&t1793, &Py::str("s0"))?, py::getitem(&t1793, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1794 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1795 = py::len(&v_memo.clone())?; Py::Bool({ let t1796 = Py::Int(64i64); if py::gt(&t1795, &t1796)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1797 = v_want.clone(); Py::Bool({ let t1798 = Py::None; if py::is(&t1797, &t1798) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1799 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1830 = (t1799).items()?;
        if t1830.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1830[0].clone();
        v_v = t1830[1].clone();
        if ({ let t1800 = v_k.clone(); Py::Bool({ let t1801 = v_outputs.clone(); if !py::contains(&t1801, &t1800)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1869(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1870 = v_key.clone(); Py::Bool({ let t1871 = v_memo.clone(); if !py::contains(&t1871, &t1870)? { true } else { false } }) }).truthy() {
        let t1873: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1872 = v_inputs.clone(); f_epi_sis(py::getitem(&t1872, &Py::str("beta"))?, py::getitem(&t1872, &Py::str("gamma"))?, py::getitem(&t1872, &Py::str("i0"))?, py::getitem(&t1872, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1873 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1874 = py::len(&v_memo.clone())?; Py::Bool({ let t1875 = Py::Int(64i64); if py::gt(&t1874, &t1875)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1876 = v_want.clone(); Py::Bool({ let t1877 = Py::None; if py::is(&t1876, &t1877) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1878 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1909 = (t1878).items()?;
        if t1909.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1909[0].clone();
        v_v = t1909[1].clone();
        if ({ let t1879 = v_k.clone(); Py::Bool({ let t1880 = v_outputs.clone(); if !py::contains(&t1880, &t1879)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_189(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t190 = v_key.clone(); Py::Bool({ let t191 = v_memo.clone(); if !py::contains(&t191, &t190)? { true } else { false } }) }).truthy() {
        let t193: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t192 = v_inputs.clone(); f_ne_cable(py::getitem(&t192, &Py::str("Rm"))?, py::getitem(&t192, &Py::str("Ri"))?, py::getitem(&t192, &Py::str("d"))?, py::getitem(&t192, &Py::str("x"))?, py::getitem(&t192, &Py::str("V0"))?, py::getitem(&t192, &Py::str("end"))?, py::method(&t192, "get", &[Py::str("L"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t193 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t194 = py::len(&v_memo.clone())?; Py::Bool({ let t195 = Py::Int(64i64); if py::gt(&t194, &t195)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t196 = v_want.clone(); Py::Bool({ let t197 = Py::None; if py::is(&t196, &t197) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t198 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t229 = (t198).items()?;
        if t229.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t229[0].clone();
        v_v = t229[1].clone();
        if ({ let t199 = v_k.clone(); Py::Bool({ let t200 = v_outputs.clone(); if !py::contains(&t200, &t199)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1950(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1951 = v_key.clone(); Py::Bool({ let t1952 = v_memo.clone(); if !py::contains(&t1952, &t1951)? { true } else { false } }) }).truthy() {
        let t1954: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1953 = v_inputs.clone(); f_sv_exp_hr(py::getitem(&t1953, &Py::str("times1"))?, py::getitem(&t1953, &Py::str("events1"))?, py::getitem(&t1953, &Py::str("times2"))?, py::getitem(&t1953, &Py::str("events2"))?, py::getitem(&t1953, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1954 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1955 = py::len(&v_memo.clone())?; Py::Bool({ let t1956 = Py::Int(64i64); if py::gt(&t1955, &t1956)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1957 = v_want.clone(); Py::Bool({ let t1958 = Py::None; if py::is(&t1957, &t1958) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1959 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1990 = (t1959).items()?;
        if t1990.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1990[0].clone();
        v_v = t1990[1].clone();
        if ({ let t1960 = v_k.clone(); Py::Bool({ let t1961 = v_outputs.clone(); if !py::contains(&t1961, &t1960)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2023(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2024 = v_key.clone(); Py::Bool({ let t2025 = v_memo.clone(); if !py::contains(&t2025, &t2024)? { true } else { false } }) }).truthy() {
        let t2027: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2026 = v_inputs.clone(); f_sv_exponential(py::getitem(&t2026, &Py::str("times"))?, py::getitem(&t2026, &Py::str("events"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2027 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2028 = py::len(&v_memo.clone())?; Py::Bool({ let t2029 = Py::Int(64i64); if py::gt(&t2028, &t2029)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2030 = v_want.clone(); Py::Bool({ let t2031 = Py::None; if py::is(&t2030, &t2031) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2032 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2063 = (t2032).items()?;
        if t2063.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2063[0].clone();
        v_v = t2063[1].clone();
        if ({ let t2033 = v_k.clone(); Py::Bool({ let t2034 = v_outputs.clone(); if !py::contains(&t2034, &t2033)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2121(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2122 = v_key.clone(); Py::Bool({ let t2123 = v_memo.clone(); if !py::contains(&t2123, &t2122)? { true } else { false } }) }).truthy() {
        let t2125: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2124 = v_inputs.clone(); f_ne_fhn(py::getitem(&t2124, &Py::str("a"))?, py::getitem(&t2124, &Py::str("b"))?, py::getitem(&t2124, &Py::str("eps"))?, py::getitem(&t2124, &Py::str("I"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2125 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2126 = py::len(&v_memo.clone())?; Py::Bool({ let t2127 = Py::Int(64i64); if py::gt(&t2126, &t2127)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2128 = v_want.clone(); Py::Bool({ let t2129 = Py::None; if py::is(&t2128, &t2129) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2130 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2161 = (t2130).items()?;
        if t2161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2161[0].clone();
        v_v = t2161[1].clone();
        if ({ let t2131 = v_k.clone(); Py::Bool({ let t2132 = v_outputs.clone(); if !py::contains(&t2132, &t2131)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2187 = v_key.clone(); Py::Bool({ let t2188 = v_memo.clone(); if !py::contains(&t2188, &t2187)? { true } else { false } }) }).truthy() {
        let t2190: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2189 = v_inputs.clone(); f_im_psf_mtf(py::getitem(&t2189, &Py::str("fwhm"))?, py::getitem(&t2189, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2190 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2191 = py::len(&v_memo.clone())?; Py::Bool({ let t2192 = Py::Int(64i64); if py::gt(&t2191, &t2192)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2193 = v_want.clone(); Py::Bool({ let t2194 = Py::None; if py::is(&t2193, &t2194) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2195 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2226 = (t2195).items()?;
        if t2226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2226[0].clone();
        v_v = t2226[1].clone();
        if ({ let t2196 = v_k.clone(); Py::Bool({ let t2197 = v_outputs.clone(); if !py::contains(&t2197, &t2196)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2262(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2263 = v_key.clone(); Py::Bool({ let t2264 = v_memo.clone(); if !py::contains(&t2264, &t2263)? { true } else { false } }) }).truthy() {
        let t2266: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2265 = v_inputs.clone(); f_ne_ghk_current(py::getitem(&t2265, &Py::str("P"))?, py::getitem(&t2265, &Py::str("z"))?, py::getitem(&t2265, &Py::str("C_in"))?, py::getitem(&t2265, &Py::str("C_out"))?, py::getitem(&t2265, &Py::str("V"))?, py::getitem(&t2265, &Py::str("T"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2266 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2267 = py::len(&v_memo.clone())?; Py::Bool({ let t2268 = Py::Int(64i64); if py::gt(&t2267, &t2268)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2269 = v_want.clone(); Py::Bool({ let t2270 = Py::None; if py::is(&t2269, &t2270) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2271 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2302 = (t2271).items()?;
        if t2302.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2302[0].clone();
        v_v = t2302[1].clone();
        if ({ let t2272 = v_k.clone(); Py::Bool({ let t2273 = v_outputs.clone(); if !py::contains(&t2273, &t2272)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2328(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2329 = v_key.clone(); Py::Bool({ let t2330 = v_memo.clone(); if !py::contains(&t2330, &t2329)? { true } else { false } }) }).truthy() {
        let t2332: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2331 = v_inputs.clone(); f_ne_ghk_voltage(py::getitem(&t2331, &Py::str("T"))?, py::getitem(&t2331, &Py::str("PK"))?, py::getitem(&t2331, &Py::str("PNa"))?, py::getitem(&t2331, &Py::str("PCl"))?, py::getitem(&t2331, &Py::str("K_o"))?, py::getitem(&t2331, &Py::str("K_i"))?, py::getitem(&t2331, &Py::str("Na_o"))?, py::getitem(&t2331, &Py::str("Na_i"))?, py::getitem(&t2331, &Py::str("Cl_o"))?, py::getitem(&t2331, &Py::str("Cl_i"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2332 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2333 = py::len(&v_memo.clone())?; Py::Bool({ let t2334 = Py::Int(64i64); if py::gt(&t2333, &t2334)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2335 = v_want.clone(); Py::Bool({ let t2336 = Py::None; if py::is(&t2335, &t2336) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2337 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2368 = (t2337).items()?;
        if t2368.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2368[0].clone();
        v_v = t2368[1].clone();
        if ({ let t2338 = v_k.clone(); Py::Bool({ let t2339 = v_outputs.clone(); if !py::contains(&t2339, &t2338)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2396(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2397 = v_key.clone(); Py::Bool({ let t2398 = v_memo.clone(); if !py::contains(&t2398, &t2397)? { true } else { false } }) }).truthy() {
        let t2400: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2399 = v_inputs.clone(); f_ne_hh_gating(py::getitem(&t2399, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2400 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2401 = py::len(&v_memo.clone())?; Py::Bool({ let t2402 = Py::Int(64i64); if py::gt(&t2401, &t2402)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2403 = v_want.clone(); Py::Bool({ let t2404 = Py::None; if py::is(&t2403, &t2404) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2405 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2436 = (t2405).items()?;
        if t2436.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2436[0].clone();
        v_v = t2436[1].clone();
        if ({ let t2406 = v_k.clone(); Py::Bool({ let t2407 = v_outputs.clone(); if !py::contains(&t2407, &t2406)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2482(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2483 = v_key.clone(); Py::Bool({ let t2484 = v_memo.clone(); if !py::contains(&t2484, &t2483)? { true } else { false } }) }).truthy() {
        let t2486: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2485 = v_inputs.clone(); f_ne_hh_gk_clamp(py::getitem(&t2485, &Py::str("gK_max"))?, py::getitem(&t2485, &Py::str("V_hold"))?, py::getitem(&t2485, &Py::str("V_step"))?, py::getitem(&t2485, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2486 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2487 = py::len(&v_memo.clone())?; Py::Bool({ let t2488 = Py::Int(64i64); if py::gt(&t2487, &t2488)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2489 = v_want.clone(); Py::Bool({ let t2490 = Py::None; if py::is(&t2489, &t2490) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2491 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2522 = (t2491).items()?;
        if t2522.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2522[0].clone();
        v_v = t2522[1].clone();
        if ({ let t2492 = v_k.clone(); Py::Bool({ let t2493 = v_outputs.clone(); if !py::contains(&t2493, &t2492)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_26(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t27 = v_key.clone(); Py::Bool({ let t28 = v_memo.clone(); if !py::contains(&t28, &t27)? { true } else { false } }) }).truthy() {
        let t30: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t29 = v_inputs.clone(); f_bmi(py::getitem(&t29, &Py::str("m"))?, py::getitem(&t29, &Py::str("h"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t30 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t31 = py::len(&v_memo.clone())?; Py::Bool({ let t32 = Py::Int(64i64); if py::gt(&t31, &t32)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t33 = v_want.clone(); Py::Bool({ let t34 = Py::None; if py::is(&t33, &t34) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t35 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t66 = (t35).items()?;
        if t66.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t66[0].clone();
        v_v = t66[1].clone();
        if ({ let t36 = v_k.clone(); Py::Bool({ let t37 = v_outputs.clone(); if !py::contains(&t37, &t36)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_264(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t265 = v_key.clone(); Py::Bool({ let t266 = v_memo.clone(); if !py::contains(&t266, &t265)? { true } else { false } }) }).truthy() {
        let t268: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t267 = v_inputs.clone(); f_dx_kappa(py::getitem(&t267, &Py::str("counts"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t268 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t269 = py::len(&v_memo.clone())?; Py::Bool({ let t270 = Py::Int(64i64); if py::gt(&t269, &t270)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t271 = v_want.clone(); Py::Bool({ let t272 = Py::None; if py::is(&t271, &t272) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t273 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t304 = (t273).items()?;
        if t304.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t304[0].clone();
        v_v = t304[1].clone();
        if ({ let t274 = v_k.clone(); Py::Bool({ let t275 = v_outputs.clone(); if !py::contains(&t275, &t274)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2650(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2651 = v_key.clone(); Py::Bool({ let t2652 = v_memo.clone(); if !py::contains(&t2652, &t2651)? { true } else { false } }) }).truthy() {
        let t2654: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2653 = v_inputs.clone(); f_ne_hh_rest(py::getitem(&t2653, &Py::str("gNa"))?, py::getitem(&t2653, &Py::str("gK"))?, py::getitem(&t2653, &Py::str("gL"))?, py::getitem(&t2653, &Py::str("ENa"))?, py::getitem(&t2653, &Py::str("EK"))?, py::getitem(&t2653, &Py::str("EL"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2654 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2655 = py::len(&v_memo.clone())?; Py::Bool({ let t2656 = Py::Int(64i64); if py::gt(&t2655, &t2656)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2657 = v_want.clone(); Py::Bool({ let t2658 = Py::None; if py::is(&t2657, &t2658) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2659 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2690 = (t2659).items()?;
        if t2690.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2690[0].clone();
        v_v = t2690[1].clone();
        if ({ let t2660 = v_k.clone(); Py::Bool({ let t2661 = v_outputs.clone(); if !py::contains(&t2661, &t2660)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2714(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2715 = v_key.clone(); Py::Bool({ let t2716 = v_memo.clone(); if !py::contains(&t2716, &t2715)? { true } else { false } }) }).truthy() {
        let t2718: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2717 = v_inputs.clone(); f_hounsfield(py::getitem(&t2717, &Py::str("mu"))?, py::getitem(&t2717, &Py::str("mu_water"))?, py::method(&t2717, "get", &[Py::str("mu_air"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2718 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2719 = py::len(&v_memo.clone())?; Py::Bool({ let t2720 = Py::Int(64i64); if py::gt(&t2719, &t2720)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2721 = v_want.clone(); Py::Bool({ let t2722 = Py::None; if py::is(&t2721, &t2722) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2723 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2754 = (t2723).items()?;
        if t2754.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2754[0].clone();
        v_v = t2754[1].clone();
        if ({ let t2724 = v_k.clone(); Py::Bool({ let t2725 = v_outputs.clone(); if !py::contains(&t2725, &t2724)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2783(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2784 = v_key.clone(); Py::Bool({ let t2785 = v_memo.clone(); if !py::contains(&t2785, &t2784)? { true } else { false } }) }).truthy() {
        let t2787: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2786 = v_inputs.clone(); f_dx_irr(py::getitem(&t2786, &Py::str("events1"))?, py::getitem(&t2786, &Py::str("time1"))?, py::getitem(&t2786, &Py::str("events2"))?, py::getitem(&t2786, &Py::str("time2"))?, py::getitem(&t2786, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2787 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2788 = py::len(&v_memo.clone())?; Py::Bool({ let t2789 = Py::Int(64i64); if py::gt(&t2788, &t2789)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2790 = v_want.clone(); Py::Bool({ let t2791 = Py::None; if py::is(&t2790, &t2791) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2792 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2823 = (t2792).items()?;
        if t2823.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2823[0].clone();
        v_v = t2823[1].clone();
        if ({ let t2793 = v_k.clone(); Py::Bool({ let t2794 = v_outputs.clone(); if !py::contains(&t2794, &t2793)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2858(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2859 = v_key.clone(); Py::Bool({ let t2860 = v_memo.clone(); if !py::contains(&t2860, &t2859)? { true } else { false } }) }).truthy() {
        let t2862: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2861 = v_inputs.clone(); f_bm_indicator_dilution(py::getitem(&t2861, &Py::str("m"))?, py::getitem(&t2861, &Py::str("t"))?, py::getitem(&t2861, &Py::str("C"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2862 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2863 = py::len(&v_memo.clone())?; Py::Bool({ let t2864 = Py::Int(64i64); if py::gt(&t2863, &t2864)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2865 = v_want.clone(); Py::Bool({ let t2866 = Py::None; if py::is(&t2865, &t2866) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2867 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2898 = (t2867).items()?;
        if t2898.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2898[0].clone();
        v_v = t2898[1].clone();
        if ({ let t2868 = v_k.clone(); Py::Bool({ let t2869 = v_outputs.clone(); if !py::contains(&t2869, &t2868)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2934(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2935 = v_key.clone(); Py::Bool({ let t2936 = v_memo.clone(); if !py::contains(&t2936, &t2935)? { true } else { false } }) }).truthy() {
        let t2938: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2937 = v_inputs.clone(); f_iv_flow_rate(py::getitem(&t2937, &Py::str("V"))?, py::method(&t2937, "get", &[Py::str("T"), Py::None])?, py::method(&t2937, "get", &[Py::str("rate"), Py::None])?, py::method(&t2937, "get", &[Py::str("drop_factor"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2938 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2939 = py::len(&v_memo.clone())?; Py::Bool({ let t2940 = Py::Int(64i64); if py::gt(&t2939, &t2940)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2941 = v_want.clone(); Py::Bool({ let t2942 = Py::None; if py::is(&t2941, &t2942) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2943 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2974 = (t2943).items()?;
        if t2974.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2974[0].clone();
        v_v = t2974[1].clone();
        if ({ let t2944 = v_k.clone(); Py::Bool({ let t2945 = v_outputs.clone(); if !py::contains(&t2945, &t2944)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3019(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3020 = v_key.clone(); Py::Bool({ let t3021 = v_memo.clone(); if !py::contains(&t3021, &t3020)? { true } else { false } }) }).truthy() {
        let t3023: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3022 = v_inputs.clone(); f_iv_rate_from_dose_rate(py::getitem(&t3022, &Py::str("drug"))?, py::getitem(&t3022, &Py::str("bag_volume"))?, py::method(&t3022, "get", &[Py::str("dose_rate"), Py::None])?, py::method(&t3022, "get", &[Py::str("dose_ug_per_kg_min"), Py::None])?, py::method(&t3022, "get", &[Py::str("W"), Py::None])?, py::method(&t3022, "get", &[Py::str("drop_factor"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3023 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3024 = py::len(&v_memo.clone())?; Py::Bool({ let t3025 = Py::Int(64i64); if py::gt(&t3024, &t3025)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3026 = v_want.clone(); Py::Bool({ let t3027 = Py::None; if py::is(&t3026, &t3027) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3028 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3059 = (t3028).items()?;
        if t3059.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3059[0].clone();
        v_v = t3059[1].clone();
        if ({ let t3029 = v_k.clone(); Py::Bool({ let t3030 = v_outputs.clone(); if !py::contains(&t3030, &t3029)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3084(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3085 = v_key.clone(); Py::Bool({ let t3086 = v_memo.clone(); if !py::contains(&t3086, &t3085)? { true } else { false } }) }).truthy() {
        let t3088: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3087 = v_inputs.clone(); f_bm_kelvin_voigt(py::getitem(&t3087, &Py::str("sigma"))?, py::getitem(&t3087, &Py::str("E"))?, py::getitem(&t3087, &Py::str("eta"))?, py::getitem(&t3087, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3088 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3089 = py::len(&v_memo.clone())?; Py::Bool({ let t3090 = Py::Int(64i64); if py::gt(&t3089, &t3090)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3091 = v_want.clone(); Py::Bool({ let t3092 = Py::None; if py::is(&t3091, &t3092) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3093 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3124 = (t3093).items()?;
        if t3124.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3124[0].clone();
        v_v = t3124[1].clone();
        if ({ let t3094 = v_k.clone(); Py::Bool({ let t3095 = v_outputs.clone(); if !py::contains(&t3095, &t3094)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3186(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3187 = v_key.clone(); Py::Bool({ let t3188 = v_memo.clone(); if !py::contains(&t3188, &t3187)? { true } else { false } }) }).truthy() {
        let t3190: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3189 = v_inputs.clone(); f_sv_km_ci(py::getitem(&t3189, &Py::str("times"))?, py::getitem(&t3189, &Py::str("events"))?, py::getitem(&t3189, &Py::str("t"))?, py::getitem(&t3189, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3190 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3191 = py::len(&v_memo.clone())?; Py::Bool({ let t3192 = Py::Int(64i64); if py::gt(&t3191, &t3192)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3193 = v_want.clone(); Py::Bool({ let t3194 = Py::None; if py::is(&t3193, &t3194) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3195 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3226 = (t3195).items()?;
        if t3226.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3226[0].clone();
        v_v = t3226[1].clone();
        if ({ let t3196 = v_k.clone(); Py::Bool({ let t3197 = v_outputs.clone(); if !py::contains(&t3197, &t3196)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3287(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3288 = v_key.clone(); Py::Bool({ let t3289 = v_memo.clone(); if !py::contains(&t3289, &t3288)? { true } else { false } }) }).truthy() {
        let t3291: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3290 = v_inputs.clone(); f_sv_km(py::getitem(&t3290, &Py::str("times"))?, py::getitem(&t3290, &Py::str("events"))?, py::getitem(&t3290, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3291 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3292 = py::len(&v_memo.clone())?; Py::Bool({ let t3293 = Py::Int(64i64); if py::gt(&t3292, &t3293)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3294 = v_want.clone(); Py::Bool({ let t3295 = Py::None; if py::is(&t3294, &t3295) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3296 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3327 = (t3296).items()?;
        if t3327.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3327[0].clone();
        v_v = t3327[1].clone();
        if ({ let t3297 = v_k.clone(); Py::Bool({ let t3298 = v_outputs.clone(); if !py::contains(&t3298, &t3297)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3354(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3355 = v_key.clone(); Py::Bool({ let t3356 = v_memo.clone(); if !py::contains(&t3356, &t3355)? { true } else { false } }) }).truthy() {
        let t3358: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3357 = v_inputs.clone(); f_larmor(py::getitem(&t3357, &Py::str("B"))?, py::method(&t3357, "get", &[Py::str("gamma_bar"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3358 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3359 = py::len(&v_memo.clone())?; Py::Bool({ let t3360 = Py::Int(64i64); if py::gt(&t3359, &t3360)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3361 = v_want.clone(); Py::Bool({ let t3362 = Py::None; if py::is(&t3361, &t3362) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3363 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3394 = (t3363).items()?;
        if t3394.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3394[0].clone();
        v_v = t3394[1].clone();
        if ({ let t3364 = v_k.clone(); Py::Bool({ let t3365 = v_outputs.clone(); if !py::contains(&t3365, &t3364)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3427 = v_key.clone(); Py::Bool({ let t3428 = v_memo.clone(); if !py::contains(&t3428, &t3427)? { true } else { false } }) }).truthy() {
        let t3430: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3429 = v_inputs.clone(); f_ne_lif_rate(py::getitem(&t3429, &Py::str("tau_m"))?, py::getitem(&t3429, &Py::str("R_m"))?, py::getitem(&t3429, &Py::str("I"))?, py::getitem(&t3429, &Py::str("V_rest"))?, py::getitem(&t3429, &Py::str("V_th"))?, py::getitem(&t3429, &Py::str("V_reset"))?, py::getitem(&t3429, &Py::str("t_ref"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3430 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3431 = py::len(&v_memo.clone())?; Py::Bool({ let t3432 = Py::Int(64i64); if py::gt(&t3431, &t3432)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3433 = v_want.clone(); Py::Bool({ let t3434 = Py::None; if py::is(&t3433, &t3434) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3466 = (t3435).items()?;
        if t3466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3466[0].clone();
        v_v = t3466[1].clone();
        if ({ let t3436 = v_k.clone(); Py::Bool({ let t3437 = v_outputs.clone(); if !py::contains(&t3437, &t3436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3494(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3495 = v_key.clone(); Py::Bool({ let t3496 = v_memo.clone(); if !py::contains(&t3496, &t3495)? { true } else { false } }) }).truthy() {
        let t3498: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3497 = v_inputs.clone(); f_ne_lif_voltage(py::getitem(&t3497, &Py::str("tau_m"))?, py::getitem(&t3497, &Py::str("R_m"))?, py::getitem(&t3497, &Py::str("I"))?, py::getitem(&t3497, &Py::str("V_rest"))?, py::getitem(&t3497, &Py::str("V0"))?, py::getitem(&t3497, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3498 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3499 = py::len(&v_memo.clone())?; Py::Bool({ let t3500 = Py::Int(64i64); if py::gt(&t3499, &t3500)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3501 = v_want.clone(); Py::Bool({ let t3502 = Py::None; if py::is(&t3501, &t3502) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3503 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3534 = (t3503).items()?;
        if t3534.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3534[0].clone();
        v_v = t3534[1].clone();
        if ({ let t3504 = v_k.clone(); Py::Bool({ let t3505 = v_outputs.clone(); if !py::contains(&t3505, &t3504)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3595 = v_key.clone(); Py::Bool({ let t3596 = v_memo.clone(); if !py::contains(&t3596, &t3595)? { true } else { false } }) }).truthy() {
        let t3598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3597 = v_inputs.clone(); f_sv_logrank(py::getitem(&t3597, &Py::str("times1"))?, py::getitem(&t3597, &Py::str("events1"))?, py::getitem(&t3597, &Py::str("times2"))?, py::getitem(&t3597, &Py::str("events2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3598 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3599 = py::len(&v_memo.clone())?; Py::Bool({ let t3600 = Py::Int(64i64); if py::gt(&t3599, &t3600)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3601 = v_want.clone(); Py::Bool({ let t3602 = Py::None; if py::is(&t3601, &t3602) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3603 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3634 = (t3603).items()?;
        if t3634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3634[0].clone();
        v_v = t3634[1].clone();
        if ({ let t3604 = v_k.clone(); Py::Bool({ let t3605 = v_outputs.clone(); if !py::contains(&t3605, &t3604)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3683(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3684 = v_key.clone(); Py::Bool({ let t3685 = v_memo.clone(); if !py::contains(&t3685, &t3684)? { true } else { false } }) }).truthy() {
        let t3687: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3686 = v_inputs.clone(); f_dx_mantel_haenszel(py::getitem(&t3686, &Py::str("tables"))?, py::getitem(&t3686, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3687 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3688 = py::len(&v_memo.clone())?; Py::Bool({ let t3689 = Py::Int(64i64); if py::gt(&t3688, &t3689)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3690 = v_want.clone(); Py::Bool({ let t3691 = Py::None; if py::is(&t3690, &t3691) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3692 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3723 = (t3692).items()?;
        if t3723.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3723[0].clone();
        v_v = t3723[1].clone();
        if ({ let t3693 = v_k.clone(); Py::Bool({ let t3694 = v_outputs.clone(); if !py::contains(&t3694, &t3693)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3759(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3760 = v_key.clone(); Py::Bool({ let t3761 = v_memo.clone(); if !py::contains(&t3761, &t3760)? { true } else { false } }) }).truthy() {
        let t3763: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3762 = v_inputs.clone(); f_im_adc(py::getitem(&t3762, &Py::str("b"))?, py::getitem(&t3762, &Py::str("S"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3763 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3764 = py::len(&v_memo.clone())?; Py::Bool({ let t3765 = Py::Int(64i64); if py::gt(&t3764, &t3765)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3766 = v_want.clone(); Py::Bool({ let t3767 = Py::None; if py::is(&t3766, &t3767) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3768 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3799 = (t3768).items()?;
        if t3799.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3799[0].clone();
        v_v = t3799[1].clone();
        if ({ let t3769 = v_k.clone(); Py::Bool({ let t3770 = v_outputs.clone(); if !py::contains(&t3770, &t3769)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3818(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3819 = v_key.clone(); Py::Bool({ let t3820 = v_memo.clone(); if !py::contains(&t3820, &t3819)? { true } else { false } }) }).truthy() {
        let t3822: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3821 = v_inputs.clone(); f_ernst_angle(py::getitem(&t3821, &Py::str("TR"))?, py::getitem(&t3821, &Py::str("T1"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3822 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3823 = py::len(&v_memo.clone())?; Py::Bool({ let t3824 = Py::Int(64i64); if py::gt(&t3823, &t3824)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3825 = v_want.clone(); Py::Bool({ let t3826 = Py::None; if py::is(&t3825, &t3826) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3827 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3858 = (t3827).items()?;
        if t3858.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3858[0].clone();
        v_v = t3858[1].clone();
        if ({ let t3828 = v_k.clone(); Py::Bool({ let t3829 = v_outputs.clone(); if !py::contains(&t3829, &t3828)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3879(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3880 = v_key.clone(); Py::Bool({ let t3881 = v_memo.clone(); if !py::contains(&t3881, &t3880)? { true } else { false } }) }).truthy() {
        let t3883: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3882 = v_inputs.clone(); f_im_inversion_recovery(py::getitem(&t3882, &Py::str("T1"))?, py::getitem(&t3882, &Py::str("TR"))?, py::getitem(&t3882, &Py::str("TI"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3883 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3884 = py::len(&v_memo.clone())?; Py::Bool({ let t3885 = Py::Int(64i64); if py::gt(&t3884, &t3885)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3886 = v_want.clone(); Py::Bool({ let t3887 = Py::None; if py::is(&t3886, &t3887) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3888 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3919 = (t3888).items()?;
        if t3919.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3919[0].clone();
        v_v = t3919[1].clone();
        if ({ let t3889 = v_k.clone(); Py::Bool({ let t3890 = v_outputs.clone(); if !py::contains(&t3890, &t3889)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_391(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t392 = v_key.clone(); Py::Bool({ let t393 = v_memo.clone(); if !py::contains(&t393, &t392)? { true } else { false } }) }).truthy() {
        let t395: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t394 = v_inputs.clone(); f_sv_cox(py::getitem(&t394, &Py::str("times"))?, py::getitem(&t394, &Py::str("events"))?, py::getitem(&t394, &Py::str("x"))?, py::getitem(&t394, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t395 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t396 = py::len(&v_memo.clone())?; Py::Bool({ let t397 = Py::Int(64i64); if py::gt(&t396, &t397)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t398 = v_want.clone(); Py::Bool({ let t399 = Py::None; if py::is(&t398, &t399) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t400 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t431 = (t400).items()?;
        if t431.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t431[0].clone();
        v_v = t431[1].clone();
        if ({ let t401 = v_k.clone(); Py::Bool({ let t402 = v_outputs.clone(); if !py::contains(&t402, &t401)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3940(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3941 = v_key.clone(); Py::Bool({ let t3942 = v_memo.clone(); if !py::contains(&t3942, &t3941)? { true } else { false } }) }).truthy() {
        let t3944: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3943 = v_inputs.clone(); f_im_spin_echo(py::getitem(&t3943, &Py::str("M0"))?, py::getitem(&t3943, &Py::str("TR"))?, py::getitem(&t3943, &Py::str("TE"))?, py::getitem(&t3943, &Py::str("T1"))?, py::getitem(&t3943, &Py::str("T2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3944 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3945 = py::len(&v_memo.clone())?; Py::Bool({ let t3946 = Py::Int(64i64); if py::gt(&t3945, &t3946)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3947 = v_want.clone(); Py::Bool({ let t3948 = Py::None; if py::is(&t3947, &t3948) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3949 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3980 = (t3949).items()?;
        if t3980.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3980[0].clone();
        v_v = t3980[1].clone();
        if ({ let t3950 = v_k.clone(); Py::Bool({ let t3951 = v_outputs.clone(); if !py::contains(&t3951, &t3950)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4004(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4005 = v_key.clone(); Py::Bool({ let t4006 = v_memo.clone(); if !py::contains(&t4006, &t4005)? { true } else { false } }) }).truthy() {
        let t4008: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4007 = v_inputs.clone(); f_im_spoiled_gre(py::getitem(&t4007, &Py::str("M0"))?, py::getitem(&t4007, &Py::str("TR"))?, py::getitem(&t4007, &Py::str("TE"))?, py::getitem(&t4007, &Py::str("T1"))?, py::getitem(&t4007, &Py::str("T2star"))?, py::getitem(&t4007, &Py::str("alpha"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4008 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4009 = py::len(&v_memo.clone())?; Py::Bool({ let t4010 = Py::Int(64i64); if py::gt(&t4009, &t4010)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4011 = v_want.clone(); Py::Bool({ let t4012 = Py::None; if py::is(&t4011, &t4012) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4013 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4044 = (t4013).items()?;
        if t4044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4044[0].clone();
        v_v = t4044[1].clone();
        if ({ let t4014 = v_k.clone(); Py::Bool({ let t4015 = v_outputs.clone(); if !py::contains(&t4015, &t4014)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4080(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4081 = v_key.clone(); Py::Bool({ let t4082 = v_memo.clone(); if !py::contains(&t4082, &t4081)? { true } else { false } }) }).truthy() {
        let t4084: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4083 = v_inputs.clone(); f_im_t2_fit(py::getitem(&t4083, &Py::str("TE"))?, py::getitem(&t4083, &Py::str("S"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4084 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4085 = py::len(&v_memo.clone())?; Py::Bool({ let t4086 = Py::Int(64i64); if py::gt(&t4085, &t4086)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4087 = v_want.clone(); Py::Bool({ let t4088 = Py::None; if py::is(&t4087, &t4088) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4089 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4120 = (t4089).items()?;
        if t4120.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4120[0].clone();
        v_v = t4120[1].clone();
        if ({ let t4090 = v_k.clone(); Py::Bool({ let t4091 = v_outputs.clone(); if !py::contains(&t4091, &t4090)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4173(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4174 = v_key.clone(); Py::Bool({ let t4175 = v_memo.clone(); if !py::contains(&t4175, &t4174)? { true } else { false } }) }).truthy() {
        let t4177: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4176 = v_inputs.clone(); f_sv_nelson_aalen(py::getitem(&t4176, &Py::str("times"))?, py::getitem(&t4176, &Py::str("events"))?, py::getitem(&t4176, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4177 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4178 = py::len(&v_memo.clone())?; Py::Bool({ let t4179 = Py::Int(64i64); if py::gt(&t4178, &t4179)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4180 = v_want.clone(); Py::Bool({ let t4181 = Py::None; if py::is(&t4180, &t4181) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4182 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4213 = (t4182).items()?;
        if t4213.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4213[0].clone();
        v_v = t4213[1].clone();
        if ({ let t4183 = v_k.clone(); Py::Bool({ let t4184 = v_outputs.clone(); if !py::contains(&t4184, &t4183)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4245(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4246 = v_key.clone(); Py::Bool({ let t4247 = v_memo.clone(); if !py::contains(&t4247, &t4246)? { true } else { false } }) }).truthy() {
        let t4249: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4248 = v_inputs.clone(); f_dx_nnt(py::getitem(&t4248, &Py::str("events_t"))?, py::getitem(&t4248, &Py::str("n_t"))?, py::getitem(&t4248, &Py::str("events_c"))?, py::getitem(&t4248, &Py::str("n_c"))?, py::getitem(&t4248, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4249 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4250 = py::len(&v_memo.clone())?; Py::Bool({ let t4251 = Py::Int(64i64); if py::gt(&t4250, &t4251)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4252 = v_want.clone(); Py::Bool({ let t4253 = Py::None; if py::is(&t4252, &t4253) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4254 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4285 = (t4254).items()?;
        if t4285.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4285[0].clone();
        v_v = t4285[1].clone();
        if ({ let t4255 = v_k.clone(); Py::Bool({ let t4256 = v_outputs.clone(); if !py::contains(&t4256, &t4255)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4310(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4311 = v_key.clone(); Py::Bool({ let t4312 = v_memo.clone(); if !py::contains(&t4312, &t4311)? { true } else { false } }) }).truthy() {
        let t4314: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4313 = v_inputs.clone(); f_dx_odds_ratio(py::getitem(&t4313, &Py::str("a"))?, py::getitem(&t4313, &Py::str("b"))?, py::getitem(&t4313, &Py::str("c"))?, py::getitem(&t4313, &Py::str("d"))?, py::getitem(&t4313, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4314 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4315 = py::len(&v_memo.clone())?; Py::Bool({ let t4316 = Py::Int(64i64); if py::gt(&t4315, &t4316)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4317 = v_want.clone(); Py::Bool({ let t4318 = Py::None; if py::is(&t4317, &t4318) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4319 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4350 = (t4319).items()?;
        if t4350.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4350[0].clone();
        v_v = t4350[1].clone();
        if ({ let t4320 = v_k.clone(); Py::Bool({ let t4321 = v_outputs.clone(); if !py::contains(&t4321, &t4320)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4375(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4376 = v_key.clone(); Py::Bool({ let t4377 = v_memo.clone(); if !py::contains(&t4377, &t4376)? { true } else { false } }) }).truthy() {
        let t4379: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4378 = v_inputs.clone(); f_pk_auc_iv_bolus(py::getitem(&t4378, &Py::str("D"))?, py::getitem(&t4378, &Py::str("CL"))?, py::getitem(&t4378, &Py::str("V"))?, py::getitem(&t4378, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4379 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4380 = py::len(&v_memo.clone())?; Py::Bool({ let t4381 = Py::Int(64i64); if py::gt(&t4380, &t4381)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4382 = v_want.clone(); Py::Bool({ let t4383 = Py::None; if py::is(&t4382, &t4383) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4384 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4415 = (t4384).items()?;
        if t4415.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4415[0].clone();
        v_v = t4415[1].clone();
        if ({ let t4385 = v_k.clone(); Py::Bool({ let t4386 = v_outputs.clone(); if !py::contains(&t4386, &t4385)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4452(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4453 = v_key.clone(); Py::Bool({ let t4454 = v_memo.clone(); if !py::contains(&t4454, &t4453)? { true } else { false } }) }).truthy() {
        let t4456: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4455 = v_inputs.clone(); f_pk_auc_trapezoid(py::getitem(&t4455, &Py::str("t"))?, py::getitem(&t4455, &Py::str("C"))?, py::getitem(&t4455, &Py::str("method"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4456 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4457 = py::len(&v_memo.clone())?; Py::Bool({ let t4458 = Py::Int(64i64); if py::gt(&t4457, &t4458)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4459 = v_want.clone(); Py::Bool({ let t4460 = Py::None; if py::is(&t4459, &t4460) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4461 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4492 = (t4461).items()?;
        if t4492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4492[0].clone();
        v_v = t4492[1].clone();
        if ({ let t4462 = v_k.clone(); Py::Bool({ let t4463 = v_outputs.clone(); if !py::contains(&t4463, &t4462)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_450(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t451 = v_key.clone(); Py::Bool({ let t452 = v_memo.clone(); if !py::contains(&t452, &t451)? { true } else { false } }) }).truthy() {
        let t454: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t453 = v_inputs.clone(); f_dose_liquid_volume(py::getitem(&t453, &Py::str("D"))?, py::getitem(&t453, &Py::str("H"))?, py::getitem(&t453, &Py::str("Q"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t454 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t455 = py::len(&v_memo.clone())?; Py::Bool({ let t456 = Py::Int(64i64); if py::gt(&t455, &t456)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t457 = v_want.clone(); Py::Bool({ let t458 = Py::None; if py::is(&t457, &t458) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t459 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t490 = (t459).items()?;
        if t490.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t490[0].clone();
        v_v = t490[1].clone();
        if ({ let t460 = v_k.clone(); Py::Bool({ let t461 = v_outputs.clone(); if !py::contains(&t461, &t460)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4528(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4529 = v_key.clone(); Py::Bool({ let t4530 = v_memo.clone(); if !py::contains(&t4530, &t4529)? { true } else { false } }) }).truthy() {
        let t4532: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4531 = v_inputs.clone(); f_pk_bioavailability(py::getitem(&t4531, &Py::str("D_iv"))?, py::getitem(&t4531, &Py::str("t_iv"))?, py::getitem(&t4531, &Py::str("C_iv"))?, py::getitem(&t4531, &Py::str("D_oral"))?, py::getitem(&t4531, &Py::str("t_oral"))?, py::getitem(&t4531, &Py::str("C_oral"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4532 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4533 = py::len(&v_memo.clone())?; Py::Bool({ let t4534 = Py::Int(64i64); if py::gt(&t4533, &t4534)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4535 = v_want.clone(); Py::Bool({ let t4536 = Py::None; if py::is(&t4535, &t4536) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4537 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4568 = (t4537).items()?;
        if t4568.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4568[0].clone();
        v_v = t4568[1].clone();
        if ({ let t4538 = v_k.clone(); Py::Bool({ let t4539 = v_outputs.clone(); if !py::contains(&t4539, &t4538)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4595(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4596 = v_key.clone(); Py::Bool({ let t4597 = v_memo.clone(); if !py::contains(&t4597, &t4596)? { true } else { false } }) }).truthy() {
        let t4599: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4598 = v_inputs.clone(); f_pk_effect_site(py::getitem(&t4598, &Py::str("D"))?, py::getitem(&t4598, &Py::str("V"))?, py::getitem(&t4598, &Py::str("k"))?, py::getitem(&t4598, &Py::str("ke0"))?, py::getitem(&t4598, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4599 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4600 = py::len(&v_memo.clone())?; Py::Bool({ let t4601 = Py::Int(64i64); if py::gt(&t4600, &t4601)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4602 = v_want.clone(); Py::Bool({ let t4603 = Py::None; if py::is(&t4602, &t4603) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4604 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4635 = (t4604).items()?;
        if t4635.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4635[0].clone();
        v_v = t4635[1].clone();
        if ({ let t4605 = v_k.clone(); Py::Bool({ let t4606 = v_outputs.clone(); if !py::contains(&t4606, &t4605)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4666(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4667 = v_key.clone(); Py::Bool({ let t4668 = v_memo.clone(); if !py::contains(&t4668, &t4667)? { true } else { false } }) }).truthy() {
        let t4670: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4669 = v_inputs.clone(); f_pk_parallel_tube(py::getitem(&t4669, &Py::str("Q"))?, py::getitem(&t4669, &Py::str("fu"))?, py::getitem(&t4669, &Py::str("CL_int"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4670 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4671 = py::len(&v_memo.clone())?; Py::Bool({ let t4672 = Py::Int(64i64); if py::gt(&t4671, &t4672)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4673 = v_want.clone(); Py::Bool({ let t4674 = Py::None; if py::is(&t4673, &t4674) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4675 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4706 = (t4675).items()?;
        if t4706.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4706[0].clone();
        v_v = t4706[1].clone();
        if ({ let t4676 = v_k.clone(); Py::Bool({ let t4677 = v_outputs.clone(); if !py::contains(&t4677, &t4676)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4737(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4738 = v_key.clone(); Py::Bool({ let t4739 = v_memo.clone(); if !py::contains(&t4739, &t4738)? { true } else { false } }) }).truthy() {
        let t4741: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4740 = v_inputs.clone(); f_pk_well_stirred(py::getitem(&t4740, &Py::str("Q"))?, py::getitem(&t4740, &Py::str("fu"))?, py::getitem(&t4740, &Py::str("CL_int"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4741 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4742 = py::len(&v_memo.clone())?; Py::Bool({ let t4743 = Py::Int(64i64); if py::gt(&t4742, &t4743)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4744 = v_want.clone(); Py::Bool({ let t4745 = Py::None; if py::is(&t4744, &t4745) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4746 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4777 = (t4746).items()?;
        if t4777.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4777[0].clone();
        v_v = t4777[1].clone();
        if ({ let t4747 = v_k.clone(); Py::Bool({ let t4748 = v_outputs.clone(); if !py::contains(&t4748, &t4747)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4802(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4803 = v_key.clone(); Py::Bool({ let t4804 = v_memo.clone(); if !py::contains(&t4804, &t4803)? { true } else { false } }) }).truthy() {
        let t4806: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4805 = v_inputs.clone(); f_pk_infusion(py::getitem(&t4805, &Py::str("R0"))?, py::getitem(&t4805, &Py::str("CL"))?, py::getitem(&t4805, &Py::str("V"))?, py::getitem(&t4805, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4806 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4807 = py::len(&v_memo.clone())?; Py::Bool({ let t4808 = Py::Int(64i64); if py::gt(&t4807, &t4808)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4809 = v_want.clone(); Py::Bool({ let t4810 = Py::None; if py::is(&t4809, &t4810) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4811 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4842 = (t4811).items()?;
        if t4842.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4842[0].clone();
        v_v = t4842[1].clone();
        if ({ let t4812 = v_k.clone(); Py::Bool({ let t4813 = v_outputs.clone(); if !py::contains(&t4813, &t4812)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4863(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4864 = v_key.clone(); Py::Bool({ let t4865 = v_memo.clone(); if !py::contains(&t4865, &t4864)? { true } else { false } }) }).truthy() {
        let t4867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4866 = v_inputs.clone(); f_pk_intermittent_infusion(py::getitem(&t4866, &Py::str("R0"))?, py::getitem(&t4866, &Py::str("T_inf"))?, py::getitem(&t4866, &Py::str("tau"))?, py::getitem(&t4866, &Py::str("CL"))?, py::getitem(&t4866, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4867 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4868 = py::len(&v_memo.clone())?; Py::Bool({ let t4869 = Py::Int(64i64); if py::gt(&t4868, &t4869)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4870 = v_want.clone(); Py::Bool({ let t4871 = Py::None; if py::is(&t4870, &t4871) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4872 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4903 = (t4872).items()?;
        if t4903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4903[0].clone();
        v_v = t4903[1].clone();
        if ({ let t4873 = v_k.clone(); Py::Bool({ let t4874 = v_outputs.clone(); if !py::contains(&t4874, &t4873)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4928(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4929 = v_key.clone(); Py::Bool({ let t4930 = v_memo.clone(); if !py::contains(&t4930, &t4929)? { true } else { false } }) }).truthy() {
        let t4932: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4931 = v_inputs.clone(); f_pk_iv_bolus(py::getitem(&t4931, &Py::str("D"))?, py::getitem(&t4931, &Py::str("V"))?, py::getitem(&t4931, &Py::str("k"))?, py::getitem(&t4931, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4932 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4933 = py::len(&v_memo.clone())?; Py::Bool({ let t4934 = Py::Int(64i64); if py::gt(&t4933, &t4934)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4935 = v_want.clone(); Py::Bool({ let t4936 = Py::None; if py::is(&t4935, &t4936) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4937 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4968 = (t4937).items()?;
        if t4968.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4968[0].clone();
        v_v = t4968[1].clone();
        if ({ let t4938 = v_k.clone(); Py::Bool({ let t4939 = v_outputs.clone(); if !py::contains(&t4939, &t4938)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4987(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4988 = v_key.clone(); Py::Bool({ let t4989 = v_memo.clone(); if !py::contains(&t4989, &t4988)? { true } else { false } }) }).truthy() {
        let t4991: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4990 = v_inputs.clone(); f_pk_loading_maintenance(py::getitem(&t4990, &Py::str("C_target"))?, py::getitem(&t4990, &Py::str("V"))?, py::getitem(&t4990, &Py::str("CL"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4991 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4992 = py::len(&v_memo.clone())?; Py::Bool({ let t4993 = Py::Int(64i64); if py::gt(&t4992, &t4993)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4994 = v_want.clone(); Py::Bool({ let t4995 = Py::None; if py::is(&t4994, &t4995) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4996 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5027 = (t4996).items()?;
        if t5027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5027[0].clone();
        v_v = t5027[1].clone();
        if ({ let t4997 = v_k.clone(); Py::Bool({ let t4998 = v_outputs.clone(); if !py::contains(&t4998, &t4997)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5066 = v_key.clone(); Py::Bool({ let t5067 = v_memo.clone(); if !py::contains(&t5067, &t5066)? { true } else { false } }) }).truthy() {
        let t5069: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5068 = v_inputs.clone(); f_pk_mm_concentration(py::getitem(&t5068, &Py::str("C0"))?, py::getitem(&t5068, &Py::str("Vmax"))?, py::getitem(&t5068, &Py::str("Km"))?, py::getitem(&t5068, &Py::str("V"))?, py::getitem(&t5068, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5069 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5070 = py::len(&v_memo.clone())?; Py::Bool({ let t5071 = Py::Int(64i64); if py::gt(&t5070, &t5071)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5072 = v_want.clone(); Py::Bool({ let t5073 = Py::None; if py::is(&t5072, &t5073) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5074 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5105 = (t5074).items()?;
        if t5105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5105[0].clone();
        v_v = t5105[1].clone();
        if ({ let t5075 = v_k.clone(); Py::Bool({ let t5076 = v_outputs.clone(); if !py::contains(&t5076, &t5075)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_509(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t510 = v_key.clone(); Py::Bool({ let t511 = v_memo.clone(); if !py::contains(&t511, &t510)? { true } else { false } }) }).truthy() {
        let t513: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t512 = v_inputs.clone(); f_dose_tablets(py::getitem(&t512, &Py::str("D"))?, py::getitem(&t512, &Py::str("H"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t513 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t514 = py::len(&v_memo.clone())?; Py::Bool({ let t515 = Py::Int(64i64); if py::gt(&t514, &t515)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t516 = v_want.clone(); Py::Bool({ let t517 = Py::None; if py::is(&t516, &t517) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t518 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t549 = (t518).items()?;
        if t549.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t549[0].clone();
        v_v = t549[1].clone();
        if ({ let t519 = v_k.clone(); Py::Bool({ let t520 = v_outputs.clone(); if !py::contains(&t520, &t519)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5126(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5127 = v_key.clone(); Py::Bool({ let t5128 = v_memo.clone(); if !py::contains(&t5128, &t5127)? { true } else { false } }) }).truthy() {
        let t5130: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5129 = v_inputs.clone(); f_pk_mm_steady_state(py::getitem(&t5129, &Py::str("R0"))?, py::getitem(&t5129, &Py::str("Vmax"))?, py::getitem(&t5129, &Py::str("Km"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5130 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5131 = py::len(&v_memo.clone())?; Py::Bool({ let t5132 = Py::Int(64i64); if py::gt(&t5131, &t5132)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5133 = v_want.clone(); Py::Bool({ let t5134 = Py::None; if py::is(&t5133, &t5134) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5135 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5166 = (t5135).items()?;
        if t5166.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5166[0].clone();
        v_v = t5166[1].clone();
        if ({ let t5136 = v_k.clone(); Py::Bool({ let t5137 = v_outputs.clone(); if !py::contains(&t5137, &t5136)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5187(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5188 = v_key.clone(); Py::Bool({ let t5189 = v_memo.clone(); if !py::contains(&t5189, &t5188)? { true } else { false } }) }).truthy() {
        let t5191: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5190 = v_inputs.clone(); f_pk_mm_time(py::getitem(&t5190, &Py::str("C0"))?, py::getitem(&t5190, &Py::str("C"))?, py::getitem(&t5190, &Py::str("Vmax"))?, py::getitem(&t5190, &Py::str("Km"))?, py::getitem(&t5190, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5191 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5192 = py::len(&v_memo.clone())?; Py::Bool({ let t5193 = Py::Int(64i64); if py::gt(&t5192, &t5193)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5194 = v_want.clone(); Py::Bool({ let t5195 = Py::None; if py::is(&t5194, &t5195) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5196 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5227 = (t5196).items()?;
        if t5227.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5227[0].clone();
        v_v = t5227[1].clone();
        if ({ let t5197 = v_k.clone(); Py::Bool({ let t5198 = v_outputs.clone(); if !py::contains(&t5198, &t5197)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5284(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5285 = v_key.clone(); Py::Bool({ let t5286 = v_memo.clone(); if !py::contains(&t5286, &t5285)? { true } else { false } }) }).truthy() {
        let t5288: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5287 = v_inputs.clone(); f_pk_nca_iv(py::getitem(&t5287, &Py::str("D"))?, py::getitem(&t5287, &Py::str("t"))?, py::getitem(&t5287, &Py::str("C"))?, py::getitem(&t5287, &Py::str("n_terminal"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5288 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5289 = py::len(&v_memo.clone())?; Py::Bool({ let t5290 = Py::Int(64i64); if py::gt(&t5289, &t5290)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5291 = v_want.clone(); Py::Bool({ let t5292 = Py::None; if py::is(&t5291, &t5292) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5293 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5324 = (t5293).items()?;
        if t5324.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5324[0].clone();
        v_v = t5324[1].clone();
        if ({ let t5294 = v_k.clone(); Py::Bool({ let t5295 = v_outputs.clone(); if !py::contains(&t5295, &t5294)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5355(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5356 = v_key.clone(); Py::Bool({ let t5357 = v_memo.clone(); if !py::contains(&t5357, &t5356)? { true } else { false } }) }).truthy() {
        let t5359: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5358 = v_inputs.clone(); f_pk_nth_dose(py::getitem(&t5358, &Py::str("D"))?, py::getitem(&t5358, &Py::str("V"))?, py::getitem(&t5358, &Py::str("k"))?, py::getitem(&t5358, &Py::str("tau"))?, py::getitem(&t5358, &Py::str("n"))?, py::getitem(&t5358, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5359 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5360 = py::len(&v_memo.clone())?; Py::Bool({ let t5361 = Py::Int(64i64); if py::gt(&t5360, &t5361)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5362 = v_want.clone(); Py::Bool({ let t5363 = Py::None; if py::is(&t5362, &t5363) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5364 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5395 = (t5364).items()?;
        if t5395.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5395[0].clone();
        v_v = t5395[1].clone();
        if ({ let t5365 = v_k.clone(); Py::Bool({ let t5366 = v_outputs.clone(); if !py::contains(&t5366, &t5365)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5434(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5435 = v_key.clone(); Py::Bool({ let t5436 = v_memo.clone(); if !py::contains(&t5436, &t5435)? { true } else { false } }) }).truthy() {
        let t5438: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5437 = v_inputs.clone(); f_pk_oral(py::getitem(&t5437, &Py::str("F"))?, py::getitem(&t5437, &Py::str("D"))?, py::getitem(&t5437, &Py::str("V"))?, py::getitem(&t5437, &Py::str("ka"))?, py::getitem(&t5437, &Py::str("k"))?, py::getitem(&t5437, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5438 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5439 = py::len(&v_memo.clone())?; Py::Bool({ let t5440 = Py::Int(64i64); if py::gt(&t5439, &t5440)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5441 = v_want.clone(); Py::Bool({ let t5442 = Py::None; if py::is(&t5441, &t5442) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5443 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5474 = (t5443).items()?;
        if t5474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5474[0].clone();
        v_v = t5474[1].clone();
        if ({ let t5444 = v_k.clone(); Py::Bool({ let t5445 = v_outputs.clone(); if !py::contains(&t5445, &t5444)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5507(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5508 = v_key.clone(); Py::Bool({ let t5509 = v_memo.clone(); if !py::contains(&t5509, &t5508)? { true } else { false } }) }).truthy() {
        let t5511: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5510 = v_inputs.clone(); f_pk_oral_peak(py::getitem(&t5510, &Py::str("F"))?, py::getitem(&t5510, &Py::str("D"))?, py::getitem(&t5510, &Py::str("V"))?, py::getitem(&t5510, &Py::str("ka"))?, py::getitem(&t5510, &Py::str("k"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5511 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5512 = py::len(&v_memo.clone())?; Py::Bool({ let t5513 = Py::Int(64i64); if py::gt(&t5512, &t5513)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5514 = v_want.clone(); Py::Bool({ let t5515 = Py::None; if py::is(&t5514, &t5515) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5516 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5547 = (t5516).items()?;
        if t5547.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5547[0].clone();
        v_v = t5547[1].clone();
        if ({ let t5517 = v_k.clone(); Py::Bool({ let t5518 = v_outputs.clone(); if !py::contains(&t5518, &t5517)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5574(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5575 = v_key.clone(); Py::Bool({ let t5576 = v_memo.clone(); if !py::contains(&t5576, &t5575)? { true } else { false } }) }).truthy() {
        let t5578: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5577 = v_inputs.clone(); f_pk_short_infusion(py::getitem(&t5577, &Py::str("R0"))?, py::getitem(&t5577, &Py::str("T_inf"))?, py::getitem(&t5577, &Py::str("CL"))?, py::getitem(&t5577, &Py::str("V"))?, py::getitem(&t5577, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5578 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5579 = py::len(&v_memo.clone())?; Py::Bool({ let t5580 = Py::Int(64i64); if py::gt(&t5579, &t5580)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5581 = v_want.clone(); Py::Bool({ let t5582 = Py::None; if py::is(&t5581, &t5582) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5583 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5614 = (t5583).items()?;
        if t5614.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5614[0].clone();
        v_v = t5614[1].clone();
        if ({ let t5584 = v_k.clone(); Py::Bool({ let t5585 = v_outputs.clone(); if !py::contains(&t5585, &t5584)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5633(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5634 = v_key.clone(); Py::Bool({ let t5635 = v_memo.clone(); if !py::contains(&t5635, &t5634)? { true } else { false } }) }).truthy() {
        let t5637: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5636 = v_inputs.clone(); f_pk_ss_iv(py::getitem(&t5636, &Py::str("D"))?, py::getitem(&t5636, &Py::str("V"))?, py::getitem(&t5636, &Py::str("k"))?, py::getitem(&t5636, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5637 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5638 = py::len(&v_memo.clone())?; Py::Bool({ let t5639 = Py::Int(64i64); if py::gt(&t5638, &t5639)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5640 = v_want.clone(); Py::Bool({ let t5641 = Py::None; if py::is(&t5640, &t5641) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5642 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5673 = (t5642).items()?;
        if t5673.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5673[0].clone();
        v_v = t5673[1].clone();
        if ({ let t5643 = v_k.clone(); Py::Bool({ let t5644 = v_outputs.clone(); if !py::contains(&t5644, &t5643)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5710(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5711 = v_key.clone(); Py::Bool({ let t5712 = v_memo.clone(); if !py::contains(&t5712, &t5711)? { true } else { false } }) }).truthy() {
        let t5714: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5713 = v_inputs.clone(); f_pk_ss_oral(py::getitem(&t5713, &Py::str("F"))?, py::getitem(&t5713, &Py::str("D"))?, py::getitem(&t5713, &Py::str("V"))?, py::getitem(&t5713, &Py::str("ka"))?, py::getitem(&t5713, &Py::str("k"))?, py::getitem(&t5713, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5714 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5715 = py::len(&v_memo.clone())?; Py::Bool({ let t5716 = Py::Int(64i64); if py::gt(&t5715, &t5716)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5717 = v_want.clone(); Py::Bool({ let t5718 = Py::None; if py::is(&t5717, &t5718) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5719 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5750 = (t5719).items()?;
        if t5750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5750[0].clone();
        v_v = t5750[1].clone();
        if ({ let t5720 = v_k.clone(); Py::Bool({ let t5721 = v_outputs.clone(); if !py::contains(&t5721, &t5720)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5785(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5786 = v_key.clone(); Py::Bool({ let t5787 = v_memo.clone(); if !py::contains(&t5787, &t5786)? { true } else { false } }) }).truthy() {
        let t5789: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5788 = v_inputs.clone(); f_pk_half_life_fit(py::getitem(&t5788, &Py::str("t"))?, py::getitem(&t5788, &Py::str("C"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5789 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5790 = py::len(&v_memo.clone())?; Py::Bool({ let t5791 = Py::Int(64i64); if py::gt(&t5790, &t5791)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5792 = v_want.clone(); Py::Bool({ let t5793 = Py::None; if py::is(&t5792, &t5793) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5794 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5825 = (t5794).items()?;
        if t5825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5825[0].clone();
        v_v = t5825[1].clone();
        if ({ let t5795 = v_k.clone(); Py::Bool({ let t5796 = v_outputs.clone(); if !py::contains(&t5796, &t5795)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5856(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5857 = v_key.clone(); Py::Bool({ let t5858 = v_memo.clone(); if !py::contains(&t5858, &t5857)? { true } else { false } }) }).truthy() {
        let t5860: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5859 = v_inputs.clone(); f_pk_time_to_fraction_ss(py::getitem(&t5859, &Py::str("CL"))?, py::getitem(&t5859, &Py::str("V"))?, py::getitem(&t5859, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5860 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5861 = py::len(&v_memo.clone())?; Py::Bool({ let t5862 = Py::Int(64i64); if py::gt(&t5861, &t5862)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5863 = v_want.clone(); Py::Bool({ let t5864 = Py::None; if py::is(&t5863, &t5864) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5865 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5896 = (t5865).items()?;
        if t5896.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5896[0].clone();
        v_v = t5896[1].clone();
        if ({ let t5866 = v_k.clone(); Py::Bool({ let t5867 = v_outputs.clone(); if !py::contains(&t5867, &t5866)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5924(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5925 = v_key.clone(); Py::Bool({ let t5926 = v_memo.clone(); if !py::contains(&t5926, &t5925)? { true } else { false } }) }).truthy() {
        let t5928: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5927 = v_inputs.clone(); f_pk_two_comp(py::getitem(&t5927, &Py::str("D"))?, py::getitem(&t5927, &Py::str("V1"))?, py::getitem(&t5927, &Py::str("k10"))?, py::getitem(&t5927, &Py::str("k12"))?, py::getitem(&t5927, &Py::str("k21"))?, py::getitem(&t5927, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5928 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5929 = py::len(&v_memo.clone())?; Py::Bool({ let t5930 = Py::Int(64i64); if py::gt(&t5929, &t5930)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5931 = v_want.clone(); Py::Bool({ let t5932 = Py::None; if py::is(&t5931, &t5932) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5933 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5964 = (t5933).items()?;
        if t5964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5964[0].clone();
        v_v = t5964[1].clone();
        if ({ let t5934 = v_k.clone(); Py::Bool({ let t5935 = v_outputs.clone(); if !py::contains(&t5935, &t5934)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_598(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t599 = v_key.clone(); Py::Bool({ let t600 = v_memo.clone(); if !py::contains(&t600, &t599)? { true } else { false } }) }).truthy() {
        let t602: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t601 = v_inputs.clone(); f_dose_weight_based(py::getitem(&t601, &Py::str("dose_mg_per_kg"))?, py::getitem(&t601, &Py::str("W"))?, py::getitem(&t601, &Py::str("basis"))?, py::method(&t601, "get", &[Py::str("doses_per_day"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t602 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t603 = py::len(&v_memo.clone())?; Py::Bool({ let t604 = Py::Int(64i64); if py::gt(&t603, &t604)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t605 = v_want.clone(); Py::Bool({ let t606 = Py::None; if py::is(&t605, &t606) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t607 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t638 = (t607).items()?;
        if t638.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t638[0].clone();
        v_v = t638[1].clone();
        if ({ let t608 = v_k.clone(); Py::Bool({ let t609 = v_outputs.clone(); if !py::contains(&t609, &t608)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5985(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5986 = v_key.clone(); Py::Bool({ let t5987 = v_memo.clone(); if !py::contains(&t5987, &t5986)? { true } else { false } }) }).truthy() {
        let t5989: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5988 = v_inputs.clone(); f_pk_micro_from_macro(py::getitem(&t5988, &Py::str("D"))?, py::getitem(&t5988, &Py::str("A"))?, py::getitem(&t5988, &Py::str("alpha"))?, py::getitem(&t5988, &Py::str("B"))?, py::getitem(&t5988, &Py::str("beta"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5989 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5990 = py::len(&v_memo.clone())?; Py::Bool({ let t5991 = Py::Int(64i64); if py::gt(&t5990, &t5991)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5992 = v_want.clone(); Py::Bool({ let t5993 = Py::None; if py::is(&t5992, &t5993) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5994 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6025 = (t5994).items()?;
        if t6025.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6025[0].clone();
        v_v = t6025[1].clone();
        if ({ let t5995 = v_k.clone(); Py::Bool({ let t5996 = v_outputs.clone(); if !py::contains(&t5996, &t5995)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6044(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6045 = v_key.clone(); Py::Bool({ let t6046 = v_memo.clone(); if !py::contains(&t6046, &t6045)? { true } else { false } }) }).truthy() {
        let t6048: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6047 = v_inputs.clone(); f_bm_poiseuille(py::getitem(&t6047, &Py::str("dP"))?, py::getitem(&t6047, &Py::str("r"))?, py::getitem(&t6047, &Py::str("L"))?, py::getitem(&t6047, &Py::str("mu"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6048 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6049 = py::len(&v_memo.clone())?; Py::Bool({ let t6050 = Py::Int(64i64); if py::gt(&t6049, &t6050)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6051 = v_want.clone(); Py::Bool({ let t6052 = Py::None; if py::is(&t6051, &t6052) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6053 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6084 = (t6053).items()?;
        if t6084.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6084[0].clone();
        v_v = t6084[1].clone();
        if ({ let t6054 = v_k.clone(); Py::Bool({ let t6055 = v_outputs.clone(); if !py::contains(&t6055, &t6054)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6116(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6117 = v_key.clone(); Py::Bool({ let t6118 = v_memo.clone(); if !py::contains(&t6118, &t6117)? { true } else { false } }) }).truthy() {
        let t6120: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6119 = v_inputs.clone(); f_ne_psp(py::getitem(&t6119, &Py::str("tau_m"))?, py::getitem(&t6119, &Py::str("tau_s"))?, py::getitem(&t6119, &Py::str("R_m"))?, py::getitem(&t6119, &Py::str("I0"))?, py::getitem(&t6119, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6120 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6121 = py::len(&v_memo.clone())?; Py::Bool({ let t6122 = Py::Int(64i64); if py::gt(&t6121, &t6122)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6123 = v_want.clone(); Py::Bool({ let t6124 = Py::None; if py::is(&t6123, &t6124) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6125 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6156 = (t6125).items()?;
        if t6156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6156[0].clone();
        v_v = t6156[1].clone();
        if ({ let t6126 = v_k.clone(); Py::Bool({ let t6127 = v_outputs.clone(); if !py::contains(&t6127, &t6126)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6188(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6189 = v_key.clone(); Py::Bool({ let t6190 = v_memo.clone(); if !py::contains(&t6190, &t6189)? { true } else { false } }) }).truthy() {
        let t6192: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6191 = v_inputs.clone(); f_reconstitution(py::getitem(&t6191, &Py::str("powder"))?, py::getitem(&t6191, &Py::str("diluent"))?, py::method(&t6191, "get", &[Py::str("displacement"), Py::None])?, py::method(&t6191, "get", &[Py::str("D"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6192 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6193 = py::len(&v_memo.clone())?; Py::Bool({ let t6194 = Py::Int(64i64); if py::gt(&t6193, &t6194)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6195 = v_want.clone(); Py::Bool({ let t6196 = Py::None; if py::is(&t6195, &t6196) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6197 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6228 = (t6197).items()?;
        if t6228.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6228[0].clone();
        v_v = t6228[1].clone();
        if ({ let t6198 = v_k.clone(); Py::Bool({ let t6199 = v_outputs.clone(); if !py::contains(&t6199, &t6198)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6253(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6254 = v_key.clone(); Py::Bool({ let t6255 = v_memo.clone(); if !py::contains(&t6255, &t6254)? { true } else { false } }) }).truthy() {
        let t6257: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6256 = v_inputs.clone(); f_dx_relative_risk(py::getitem(&t6256, &Py::str("a"))?, py::getitem(&t6256, &Py::str("b"))?, py::getitem(&t6256, &Py::str("c"))?, py::getitem(&t6256, &Py::str("d"))?, py::getitem(&t6256, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6257 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6258 = py::len(&v_memo.clone())?; Py::Bool({ let t6259 = Py::Int(64i64); if py::gt(&t6258, &t6259)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6260 = v_want.clone(); Py::Bool({ let t6261 = Py::None; if py::is(&t6260, &t6261) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6262 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6293 = (t6262).items()?;
        if t6293.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6293[0].clone();
        v_v = t6293[1].clone();
        if ({ let t6263 = v_k.clone(); Py::Bool({ let t6264 = v_outputs.clone(); if !py::contains(&t6264, &t6263)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6343(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6344 = v_key.clone(); Py::Bool({ let t6345 = v_memo.clone(); if !py::contains(&t6345, &t6344)? { true } else { false } }) }).truthy() {
        let t6347: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6346 = v_inputs.clone(); f_bm_respiratory_fit(py::getitem(&t6346, &Py::str("P"))?, py::getitem(&t6346, &Py::str("Q"))?, py::getitem(&t6346, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6347 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6348 = py::len(&v_memo.clone())?; Py::Bool({ let t6349 = Py::Int(64i64); if py::gt(&t6348, &t6349)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6350 = v_want.clone(); Py::Bool({ let t6351 = Py::None; if py::is(&t6350, &t6351) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6352 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6383 = (t6352).items()?;
        if t6383.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6383[0].clone();
        v_v = t6383[1].clone();
        if ({ let t6353 = v_k.clone(); Py::Bool({ let t6354 = v_outputs.clone(); if !py::contains(&t6354, &t6353)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6437(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6438 = v_key.clone(); Py::Bool({ let t6439 = v_memo.clone(); if !py::contains(&t6439, &t6438)? { true } else { false } }) }).truthy() {
        let t6441: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6440 = v_inputs.clone(); f_sv_rmst(py::getitem(&t6440, &Py::str("times"))?, py::getitem(&t6440, &Py::str("events"))?, py::getitem(&t6440, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6441 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6442 = py::len(&v_memo.clone())?; Py::Bool({ let t6443 = Py::Int(64i64); if py::gt(&t6442, &t6443)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6444 = v_want.clone(); Py::Bool({ let t6445 = Py::None; if py::is(&t6444, &t6445) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6446 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6477 = (t6446).items()?;
        if t6477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6477[0].clone();
        v_v = t6477[1].clone();
        if ({ let t6447 = v_k.clone(); Py::Bool({ let t6448 = v_outputs.clone(); if !py::contains(&t6448, &t6447)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6503(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6504 = v_key.clone(); Py::Bool({ let t6505 = v_memo.clone(); if !py::contains(&t6505, &t6504)? { true } else { false } }) }).truthy() {
        let t6507: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6506 = v_inputs.clone(); f_bm_sls_creep(py::getitem(&t6506, &Py::str("sigma"))?, py::getitem(&t6506, &Py::str("E_inf"))?, py::getitem(&t6506, &Py::str("E1"))?, py::getitem(&t6506, &Py::str("eta"))?, py::getitem(&t6506, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6507 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6508 = py::len(&v_memo.clone())?; Py::Bool({ let t6509 = Py::Int(64i64); if py::gt(&t6508, &t6509)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6510 = v_want.clone(); Py::Bool({ let t6511 = Py::None; if py::is(&t6510, &t6511) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6512 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6543 = (t6512).items()?;
        if t6543.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6543[0].clone();
        v_v = t6543[1].clone();
        if ({ let t6513 = v_k.clone(); Py::Bool({ let t6514 = v_outputs.clone(); if !py::contains(&t6514, &t6513)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6568(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6569 = v_key.clone(); Py::Bool({ let t6570 = v_memo.clone(); if !py::contains(&t6570, &t6569)? { true } else { false } }) }).truthy() {
        let t6572: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6571 = v_inputs.clone(); f_bm_sls_relaxation(py::getitem(&t6571, &Py::str("eps0"))?, py::getitem(&t6571, &Py::str("E_inf"))?, py::getitem(&t6571, &Py::str("E1"))?, py::getitem(&t6571, &Py::str("eta"))?, py::getitem(&t6571, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6572 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6573 = py::len(&v_memo.clone())?; Py::Bool({ let t6574 = Py::Int(64i64); if py::gt(&t6573, &t6574)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6575 = v_want.clone(); Py::Bool({ let t6576 = Py::None; if py::is(&t6575, &t6576) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6577 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6608 = (t6577).items()?;
        if t6608.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6608[0].clone();
        v_v = t6608[1].clone();
        if ({ let t6578 = v_k.clone(); Py::Bool({ let t6579 = v_outputs.clone(); if !py::contains(&t6579, &t6578)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_663(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t664 = v_key.clone(); Py::Bool({ let t665 = v_memo.clone(); if !py::contains(&t665, &t664)? { true } else { false } }) }).truthy() {
        let t667: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t666 = v_inputs.clone(); f_dx_predictive_values(py::getitem(&t666, &Py::str("sensitivity"))?, py::getitem(&t666, &Py::str("specificity"))?, py::getitem(&t666, &Py::str("prevalence"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t667 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t668 = py::len(&v_memo.clone())?; Py::Bool({ let t669 = Py::Int(64i64); if py::gt(&t668, &t669)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t670 = v_want.clone(); Py::Bool({ let t671 = Py::None; if py::is(&t670, &t671) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t672 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t703 = (t672).items()?;
        if t703.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t703[0].clone();
        v_v = t703[1].clone();
        if ({ let t673 = v_k.clone(); Py::Bool({ let t674 = v_outputs.clone(); if !py::contains(&t674, &t673)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6671(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6672 = v_key.clone(); Py::Bool({ let t6673 = v_memo.clone(); if !py::contains(&t6673, &t6672)? { true } else { false } }) }).truthy() {
        let t6675: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6674 = v_inputs.clone(); f_dx_smr(py::getitem(&t6674, &Py::str("observed"))?, py::getitem(&t6674, &Py::str("expected"))?, py::getitem(&t6674, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6675 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6676 = py::len(&v_memo.clone())?; Py::Bool({ let t6677 = Py::Int(64i64); if py::gt(&t6676, &t6677)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6678 = v_want.clone(); Py::Bool({ let t6679 = Py::None; if py::is(&t6678, &t6679) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6680 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6711 = (t6680).items()?;
        if t6711.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6711[0].clone();
        v_v = t6711[1].clone();
        if ({ let t6681 = v_k.clone(); Py::Bool({ let t6682 = v_outputs.clone(); if !py::contains(&t6682, &t6681)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6744(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6745 = v_key.clone(); Py::Bool({ let t6746 = v_memo.clone(); if !py::contains(&t6746, &t6745)? { true } else { false } }) }).truthy() {
        let t6748: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6747 = v_inputs.clone(); f_snr_averaging(py::getitem(&t6747, &Py::str("SNR1"))?, py::method(&t6747, "get", &[Py::str("N"), Py::None])?, py::method(&t6747, "get", &[Py::str("SNR_target"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6748 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6749 = py::len(&v_memo.clone())?; Py::Bool({ let t6750 = Py::Int(64i64); if py::gt(&t6749, &t6750)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6751 = v_want.clone(); Py::Bool({ let t6752 = Py::None; if py::is(&t6751, &t6752) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6753 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6784 = (t6753).items()?;
        if t6784.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6784[0].clone();
        v_v = t6784[1].clone();
        if ({ let t6754 = v_k.clone(); Py::Bool({ let t6755 = v_outputs.clone(); if !py::contains(&t6755, &t6754)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6804 = v_key.clone(); Py::Bool({ let t6805 = v_memo.clone(); if !py::contains(&t6805, &t6804)? { true } else { false } }) }).truthy() {
        let t6807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6806 = v_inputs.clone(); f_im_us_interface(py::getitem(&t6806, &Py::str("Z1"))?, py::getitem(&t6806, &Py::str("Z2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6807 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6808 = py::len(&v_memo.clone())?; Py::Bool({ let t6809 = Py::Int(64i64); if py::gt(&t6808, &t6809)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6810 = v_want.clone(); Py::Bool({ let t6811 = Py::None; if py::is(&t6810, &t6811) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6812 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6843 = (t6812).items()?;
        if t6843.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6843[0].clone();
        v_v = t6843[1].clone();
        if ({ let t6813 = v_k.clone(); Py::Bool({ let t6814 = v_outputs.clone(); if !py::contains(&t6814, &t6813)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6863(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6864 = v_key.clone(); Py::Bool({ let t6865 = v_memo.clone(); if !py::contains(&t6865, &t6864)? { true } else { false } }) }).truthy() {
        let t6867: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6866 = v_inputs.clone(); f_im_us_matching(py::getitem(&t6866, &Py::str("Z1"))?, py::getitem(&t6866, &Py::str("Zm"))?, py::getitem(&t6866, &Py::str("Z2"))?, py::getitem(&t6866, &Py::str("d"))?, py::getitem(&t6866, &Py::str("c_m"))?, py::getitem(&t6866, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6867 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6868 = py::len(&v_memo.clone())?; Py::Bool({ let t6869 = Py::Int(64i64); if py::gt(&t6868, &t6869)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6870 = v_want.clone(); Py::Bool({ let t6871 = Py::None; if py::is(&t6870, &t6871) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6872 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6903 = (t6872).items()?;
        if t6903.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6903[0].clone();
        v_v = t6903[1].clone();
        if ({ let t6873 = v_k.clone(); Py::Bool({ let t6874 = v_outputs.clone(); if !py::contains(&t6874, &t6873)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6922(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6923 = v_key.clone(); Py::Bool({ let t6924 = v_memo.clone(); if !py::contains(&t6924, &t6923)? { true } else { false } }) }).truthy() {
        let t6926: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6925 = v_inputs.clone(); f_us_wavelength(py::getitem(&t6925, &Py::str("f"))?, py::method(&t6925, "get", &[Py::str("c"), Py::Float(1540.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6926 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6927 = py::len(&v_memo.clone())?; Py::Bool({ let t6928 = Py::Int(64i64); if py::gt(&t6927, &t6928)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6929 = v_want.clone(); Py::Bool({ let t6930 = Py::None; if py::is(&t6929, &t6930) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6931 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6962 = (t6931).items()?;
        if t6962.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6962[0].clone();
        v_v = t6962[1].clone();
        if ({ let t6932 = v_k.clone(); Py::Bool({ let t6933 = v_outputs.clone(); if !py::contains(&t6933, &t6932)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6987(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6988 = v_key.clone(); Py::Bool({ let t6989 = v_memo.clone(); if !py::contains(&t6989, &t6988)? { true } else { false } }) }).truthy() {
        let t6991: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6990 = v_inputs.clone(); f_vaccine_efficacy(py::getitem(&t6990, &Py::str("cases_v"))?, py::getitem(&t6990, &Py::str("n_v"))?, py::getitem(&t6990, &Py::str("cases_u"))?, py::getitem(&t6990, &Py::str("n_u"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6991 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6992 = py::len(&v_memo.clone())?; Py::Bool({ let t6993 = Py::Int(64i64); if py::gt(&t6992, &t6993)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6994 = v_want.clone(); Py::Bool({ let t6995 = Py::None; if py::is(&t6994, &t6995) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6996 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7027 = (t6996).items()?;
        if t7027.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7027[0].clone();
        v_v = t7027[1].clone();
        if ({ let t6997 = v_k.clone(); Py::Bool({ let t6998 = v_outputs.clone(); if !py::contains(&t6998, &t6997)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7054(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7055 = v_key.clone(); Py::Bool({ let t7056 = v_memo.clone(); if !py::contains(&t7056, &t7055)? { true } else { false } }) }).truthy() {
        let t7058: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7057 = v_inputs.clone(); f_bm_windkessel_decay(py::getitem(&t7057, &Py::str("P0"))?, py::getitem(&t7057, &Py::str("Pv"))?, py::getitem(&t7057, &Py::str("R"))?, py::getitem(&t7057, &Py::str("C"))?, py::getitem(&t7057, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7058 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7059 = py::len(&v_memo.clone())?; Py::Bool({ let t7060 = Py::Int(64i64); if py::gt(&t7059, &t7060)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7061 = v_want.clone(); Py::Bool({ let t7062 = Py::None; if py::is(&t7061, &t7062) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7063 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7094 = (t7063).items()?;
        if t7094.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7094[0].clone();
        v_v = t7094[1].clone();
        if ({ let t7064 = v_k.clone(); Py::Bool({ let t7065 = v_outputs.clone(); if !py::contains(&t7065, &t7064)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7122(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7123 = v_key.clone(); Py::Bool({ let t7124 = v_memo.clone(); if !py::contains(&t7124, &t7123)? { true } else { false } }) }).truthy() {
        let t7126: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7125 = v_inputs.clone(); f_bm_windkessel_impedance(py::getitem(&t7125, &Py::str("model"))?, py::getitem(&t7125, &Py::str("R"))?, py::getitem(&t7125, &Py::str("C"))?, py::getitem(&t7125, &Py::str("f"))?, py::method(&t7125, "get", &[Py::str("Zc"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7126 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7127 = py::len(&v_memo.clone())?; Py::Bool({ let t7128 = Py::Int(64i64); if py::gt(&t7127, &t7128)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7129 = v_want.clone(); Py::Bool({ let t7130 = Py::None; if py::is(&t7129, &t7130) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7131 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7162 = (t7131).items()?;
        if t7162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7162[0].clone();
        v_v = t7162[1].clone();
        if ({ let t7132 = v_k.clone(); Py::Bool({ let t7133 = v_outputs.clone(); if !py::contains(&t7133, &t7132)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_7197(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t7198 = v_key.clone(); Py::Bool({ let t7199 = v_memo.clone(); if !py::contains(&t7199, &t7198)? { true } else { false } }) }).truthy() {
        let t7201: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t7200 = v_inputs.clone(); f_im_xray(py::getitem(&t7200, &Py::str("I0"))?, py::getitem(&t7200, &Py::str("mu_rho"))?, py::getitem(&t7200, &Py::str("rho"))?, py::getitem(&t7200, &Py::str("x"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t7201 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t7202 = py::len(&v_memo.clone())?; Py::Bool({ let t7203 = Py::Int(64i64); if py::gt(&t7202, &t7203)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t7204 = v_want.clone(); Py::Bool({ let t7205 = Py::None; if py::is(&t7204, &t7205) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t7206 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t7237 = (t7206).items()?;
        if t7237.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t7237[0].clone();
        v_v = t7237[1].clone();
        if ({ let t7207 = v_k.clone(); Py::Bool({ let t7208 = v_outputs.clone(); if !py::contains(&t7208, &t7207)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_731(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t732 = v_key.clone(); Py::Bool({ let t733 = v_memo.clone(); if !py::contains(&t733, &t732)? { true } else { false } }) }).truthy() {
        let t735: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t734 = v_inputs.clone(); f_dx_roc_auc(py::getitem(&t734, &Py::str("scores_pos"))?, py::getitem(&t734, &Py::str("scores_neg"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t735 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t736 = py::len(&v_memo.clone())?; Py::Bool({ let t737 = Py::Int(64i64); if py::gt(&t736, &t737)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t738 = v_want.clone(); Py::Bool({ let t739 = Py::None; if py::is(&t738, &t739) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t740 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t771 = (t740).items()?;
        if t771.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t771[0].clone();
        v_v = t771[1].clone();
        if ({ let t741 = v_k.clone(); Py::Bool({ let t742 = v_outputs.clone(); if !py::contains(&t742, &t741)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_803(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t804 = v_key.clone(); Py::Bool({ let t805 = v_memo.clone(); if !py::contains(&t805, &t804)? { true } else { false } }) }).truthy() {
        let t807: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t806 = v_inputs.clone(); f_dx_sens_spec_ci(py::getitem(&t806, &Py::str("TP"))?, py::getitem(&t806, &Py::str("FN"))?, py::getitem(&t806, &Py::str("TN"))?, py::getitem(&t806, &Py::str("FP"))?, py::getitem(&t806, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t807 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t808 = py::len(&v_memo.clone())?; Py::Bool({ let t809 = Py::Int(64i64); if py::gt(&t808, &t809)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t810 = v_want.clone(); Py::Bool({ let t811 = Py::None; if py::is(&t810, &t811) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t812 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t843 = (t812).items()?;
        if t843.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t843[0].clone();
        v_v = t843[1].clone();
        if ({ let t813 = v_k.clone(); Py::Bool({ let t814 = v_outputs.clone(); if !py::contains(&t814, &t813)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_872(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t873 = v_key.clone(); Py::Bool({ let t874 = v_memo.clone(); if !py::contains(&t874, &t873)? { true } else { false } }) }).truthy() {
        let t876: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t875 = v_inputs.clone(); f_dx_threshold(py::getitem(&t875, &Py::str("scores_pos"))?, py::getitem(&t875, &Py::str("scores_neg"))?, py::getitem(&t875, &Py::str("threshold"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t876 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t877 = py::len(&v_memo.clone())?; Py::Bool({ let t878 = Py::Int(64i64); if py::gt(&t877, &t878)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t879 = v_want.clone(); Py::Bool({ let t880 = Py::None; if py::is(&t879, &t880) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t881 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t912 = (t881).items()?;
        if t912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t912[0].clone();
        v_v = t912[1].clone();
        if ({ let t882 = v_k.clone(); Py::Bool({ let t883 = v_outputs.clone(); if !py::contains(&t883, &t882)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_951(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t952 = v_key.clone(); Py::Bool({ let t953 = v_memo.clone(); if !py::contains(&t953, &t952)? { true } else { false } }) }).truthy() {
        let t955: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t954 = v_inputs.clone(); f_dx_youden(py::getitem(&t954, &Py::str("scores_pos"))?, py::getitem(&t954, &Py::str("scores_neg"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t955 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t956 = py::len(&v_memo.clone())?; Py::Bool({ let t957 = Py::Int(64i64); if py::gt(&t956, &t957)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t958 = v_want.clone(); Py::Bool({ let t959 = Py::None; if py::is(&t958, &t959) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t960 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t991 = (t960).items()?;
        if t991.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t991[0].clone();
        v_v = t991[1].clone();
        if ({ let t961 = v_k.clone(); Py::Bool({ let t962 = v_outputs.clone(); if !py::contains(&t962, &t961)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
