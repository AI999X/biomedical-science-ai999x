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
    "bmed_dx_roc_auc",
    "bmed_dx_sensitivity_specificity_ci",
    "bmed_dx_threshold_sensitivity_specificity",
    "bmed_dx_youden_cutoff",
    "bmed_elbow_lever_muscle_force",
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
        "bmed_dx_roc_auc" => {
            let r = f_dx_roc_auc(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_459(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_sensitivity_specificity_ci" => {
            let r = f_dx_sens_spec_ci(match v.get("TP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TN") { Some(x) => py::from_val(x), None => { return None } }, match v.get("FP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_531(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_threshold_sensitivity_specificity" => {
            let r = f_dx_threshold(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } }, match v.get("threshold") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_600(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_dx_youden_cutoff" => {
            let r = f_dx_youden(match v.get("scores_pos") { Some(x) => py::from_val(x), None => { return None } }, match v.get("scores_neg") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_679(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_elbow_lever_muscle_force" => {
            let r = f_bm_lever(match v.get("W") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_load") { Some(x) => py::from_val(x), None => { return None } }, match v.get("W_arm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_arm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d_muscle") { Some(x) => py::from_val(x), None => { return None } }, match v.get("theta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_753(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_r_from_growth_rate" => {
            let r = f_epi_r_from_growth(match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tg_mean") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Tg_sd") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_817(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_reed_frost_final_size" => {
            let r = f_epi_reed_frost(match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("p") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_903(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_final_size" => {
            let r = f_epi_seir_final_size(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1004(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_growth_rate" => {
            let r = f_epi_seir_growth(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1065(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_seir_peak" => {
            let r = f_epi_seir_peak(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("e0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1180(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sir_final_size" => {
            let r = f_epi_sir_final_size(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1279(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sir_peak" => {
            let r = f_epi_sir_peak(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("s0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1358(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_epi_sis_prevalence" => {
            let r = f_epi_sis(match v.get("beta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("i0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1437(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_exponential_hazard_ratio" => {
            let r = f_sv_exp_hr(match v.get("times1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1518(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_exponential_survival_fit" => {
            let r = f_sv_exponential(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1591(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_fitzhugh_nagumo_fixed_point" => {
            let r = f_ne_fhn(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eps") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1689(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_gaussian_psf_mtf" => {
            let r = f_im_psf_mtf(match v.get("fwhm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1754(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ghk_current_density" => {
            let r = f_ne_ghk_current(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("z") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_in") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_out") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1830(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ghk_voltage" => {
            let r = f_ne_ghk_voltage(match v.get("T") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PNa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("PCl") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("K_i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Na_i") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cl_o") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Cl_i") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1896(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_gating_steady_state" => {
            let r = f_ne_hh_gating(match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_1964(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_potassium_conductance_clamp" => {
            let r = f_ne_hh_gk_clamp(match v.get("gK_max") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_hold") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_step") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2050(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hh_resting_potential" => {
            let r = f_ne_hh_rest(match v.get("gNa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ENa") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EK") { Some(x) => py::from_val(x), None => { return None } }, match v.get("EL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2218(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_hounsfield_units" => {
            let r = f_hounsfield(match v.get("mu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_water") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_air") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(0.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2282(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_incidence_rate_ratio_ci" => {
            let r = f_dx_irr(match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("time2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2351(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_indicator_dilution_cardiac_output" => {
            let r = f_bm_indicator_dilution(match v.get("m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2426(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_kelvin_voigt_creep" => {
            let r = f_bm_kelvin_voigt(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2491(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_km_confidence_interval" => {
            let r = f_sv_km_ci(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2593(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_km_survival_median" => {
            let r = f_sv_km(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2694(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_larmor_frequency" => {
            let r = f_larmor(match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("gamma_bar") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2761(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_lif_firing_rate" => {
            let r = f_ne_lif_rate(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_th") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_reset") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_ref") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2833(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_lif_subthreshold_voltage" => {
            let r = f_ne_lif_voltage(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V_rest") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_2901(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_logrank_test" => {
            let r = f_sv_logrank(match v.get("times1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("times2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3001(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mantel_haenszel_odds_ratio" => {
            let r = f_dx_mantel_haenszel(match v.get("tables") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3090(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_adc_fit" => {
            let r = f_im_adc(match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3166(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_ernst_angle" => {
            let r = f_ernst_angle(match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3225(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_inversion_recovery" => {
            let r = f_im_inversion_recovery(match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TI") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3286(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_spin_echo_signal" => {
            let r = f_im_spin_echo(match v.get("M0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3347(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_spoiled_gradient_echo" => {
            let r = f_im_spoiled_gre(match v.get("M0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TR") { Some(x) => py::from_val(x), None => { return None } }, match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T2star") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3411(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_mri_t2_fit" => {
            let r = f_im_t2_fit(match v.get("TE") { Some(x) => py::from_val(x), None => { return None } }, match v.get("S") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3487(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_nelson_aalen_hazard" => {
            let r = f_sv_nelson_aalen(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3580(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_nnt_ci" => {
            let r = f_dx_nnt(match v.get("events_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3652(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_odds_ratio_ci" => {
            let r = f_dx_odds_ratio(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3717(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_auc_iv_bolus" => {
            let r = f_pk_auc_iv_bolus(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3782(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_auc_trapezoid" => {
            let r = f_pk_auc_trapezoid(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("method") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3859(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_bioavailability_from_profiles" => {
            let r = f_pk_bioavailability(match v.get("D_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_iv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D_oral") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t_oral") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C_oral") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_3935(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_effect_site_concentration" => {
            let r = f_pk_effect_site(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ke0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4002(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_hepatic_clearance_parallel_tube" => {
            let r = f_pk_parallel_tube(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL_int") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4073(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_hepatic_clearance_well_stirred" => {
            let r = f_pk_well_stirred(match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("fu") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL_int") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4144(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_infusion_concentration" => {
            let r = f_pk_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4209(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_intermittent_infusion_steady_state" => {
            let r = f_pk_intermittent_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4270(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_iv_bolus_concentration" => {
            let r = f_pk_iv_bolus(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4335(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_loading_maintenance" => {
            let r = f_pk_loading_maintenance(match v.get("C_target") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4394(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_concentration" => {
            let r = f_pk_mm_concentration(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4472(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_steady_state" => {
            let r = f_pk_mm_steady_state(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4533(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_michaelis_menten_time" => {
            let r = f_pk_mm_time(match v.get("C0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Vmax") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Km") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4594(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_nca_iv_bolus" => {
            let r = f_pk_nca_iv(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n_terminal") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4691(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_nth_dose_iv" => {
            let r = f_pk_nth_dose(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } }, match v.get("n") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4762(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_oral_concentration" => {
            let r = f_pk_oral(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4841(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_oral_peak" => {
            let r = f_pk_oral_peak(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4914(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_short_infusion" => {
            let r = f_pk_short_infusion(match v.get("R0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("T_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_4981(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_steady_state_iv" => {
            let r = f_pk_ss_iv(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5040(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_steady_state_oral" => {
            let r = f_pk_ss_oral(match v.get("F") { Some(x) => py::from_val(x), None => { return None } }, match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("ka") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5117(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_terminal_half_life_fit" => {
            let r = f_pk_half_life_fit(match v.get("t") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5192(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_time_to_steady_state_fraction" => {
            let r = f_pk_time_to_fraction_ss(match v.get("CL") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5263(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_two_compartment_bolus" => {
            let r = f_pk_two_comp(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k10") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k12") { Some(x) => py::from_val(x), None => { return None } }, match v.get("k21") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5331(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_pk_two_compartment_micro_constants" => {
            let r = f_pk_micro_from_macro(match v.get("D") { Some(x) => py::from_val(x), None => { return None } }, match v.get("A") { Some(x) => py::from_val(x), None => { return None } }, match v.get("alpha") { Some(x) => py::from_val(x), None => { return None } }, match v.get("B") { Some(x) => py::from_val(x), None => { return None } }, match v.get("beta") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5392(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_poiseuille_vessel_flow" => {
            let r = f_bm_poiseuille(match v.get("dP") { Some(x) => py::from_val(x), None => { return None } }, match v.get("r") { Some(x) => py::from_val(x), None => { return None } }, match v.get("L") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5451(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_postsynaptic_potential" => {
            let r = f_ne_psp(match v.get("tau_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau_s") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5523(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_relative_risk_ci" => {
            let r = f_dx_relative_risk(match v.get("a") { Some(x) => py::from_val(x), None => { return None } }, match v.get("b") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5588(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_respiratory_mechanics_fit" => {
            let r = f_bm_respiratory_fit(match v.get("P") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Q") { Some(x) => py::from_val(x), None => { return None } }, match v.get("V") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5678(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_restricted_mean_survival_time" => {
            let r = f_sv_rmst(match v.get("times") { Some(x) => py::from_val(x), None => { return None } }, match v.get("events") { Some(x) => py::from_val(x), None => { return None } }, match v.get("tau") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5772(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_sls_creep" => {
            let r = f_bm_sls_creep(match v.get("sigma") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5838(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_sls_stress_relaxation" => {
            let r = f_bm_sls_relaxation(match v.get("eps0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E_inf") { Some(x) => py::from_val(x), None => { return None } }, match v.get("E1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("eta") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_5903(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_smr_exact_ci" => {
            let r = f_dx_smr(match v.get("observed") { Some(x) => py::from_val(x), None => { return None } }, match v.get("expected") { Some(x) => py::from_val(x), None => { return None } }, match v.get("level") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6006(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_snr_averaging" => {
            let r = f_snr_averaging(match v.get("SNR1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("N") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } }, match v.get("SNR_target") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6079(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_interface" => {
            let r = f_im_us_interface(match v.get("Z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z2") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6138(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_matching_layer" => {
            let r = f_im_us_matching(match v.get("Z1") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zm") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Z2") { Some(x) => py::from_val(x), None => { return None } }, match v.get("d") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c_m") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6198(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_ultrasound_wavelength" => {
            let r = f_us_wavelength(match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("c") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::Float(1540.0)) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6257(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_windkessel_diastolic_decay" => {
            let r = f_bm_windkessel_decay(match v.get("P0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Pv") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("t") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6324(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_windkessel_input_impedance" => {
            let r = f_bm_windkessel_impedance(match v.get("model") { Some(x) => py::from_val(x), None => { return None } }, match v.get("R") { Some(x) => py::from_val(x), None => { return None } }, match v.get("C") { Some(x) => py::from_val(x), None => { return None } }, match v.get("f") { Some(x) => py::from_val(x), None => { return None } }, match v.get("Zc") { Some(x) => py::from_val(x), None => { (|| -> R { Ok(Py::None) })().ok()? } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6392(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
        }
        "bmed_xray_attenuation_layers" => {
            let r = f_im_xray(match v.get("I0") { Some(x) => py::from_val(x), None => { return None } }, match v.get("mu_rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("rho") { Some(x) => py::from_val(x), None => { return None } }, match v.get("x") { Some(x) => py::from_val(x), None => { return None } });
            finish(r, |i: &Py, o: &Py| -> f64 { match s_6467(i.clone(), o.clone()).and_then(|r| r.f()) { Ok(x) => x, Err(e) => { if std::env::var_os("AX_DEBUG").is_some() { eprintln!("check error: {e:?}"); } f64::NAN } } }, v)
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
    let t5964 = (Py::tuple(vec![py::call(&v_f.clone(), &[v_a.clone()])?, py::call(&v_f.clone(), &[v_b.clone()])?])).items()?;
    if t5964.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fa = t5964[0].clone();
    v_fb = t5964[1].clone();
    if ({ let t5965 = v_fa.clone(); Py::Bool({ let t5966 = Py::Int(0i64); if py::eq(&t5965, &t5966) { true } else { false } }) }).truthy() {
        return Ok(v_a.clone());
    }
    if ({ let t5967 = v_fb.clone(); Py::Bool({ let t5968 = Py::Int(0i64); if py::eq(&t5967, &t5968) { true } else { false } }) }).truthy() {
        return Ok(v_b.clone());
    }
    let _ = f__need({ let t5969 = { let t5971 = v_fa.clone(); Py::Bool({ let t5972 = Py::Int(0i64); if py::gt(&t5971, &t5972)? { true } else { false } }) }; Py::Bool({ let t5970 = { let t5973 = v_fb.clone(); Py::Bool({ let t5974 = Py::Int(0i64); if py::gt(&t5973, &t5974)? { true } else { false } }) }; if py::ne(&t5969, &t5970) { true } else { false } }) }, Py::str("no root in the interval"))?;
    for t5975 in py::range_iter(&[(v_n.clone()).i()?]) {
        v__ = t5975;
        v_m = py::mul(&Py::Float(0.5), &py::add(&v_a.clone(), &v_b.clone())?)?;
        if ({ let t5980 = { let t5976 = v_m.clone(); Py::Bool({ let t5977 = v_a.clone(); if py::eq(&t5976, &t5977) { true } else { false } }) }; if t5980.truthy() { t5980 } else { { let t5978 = v_m.clone(); Py::Bool({ let t5979 = v_b.clone(); if py::eq(&t5978, &t5979) { true } else { false } }) } } }).truthy() {
            break;
        }
        v_fm = py::call(&v_f.clone(), &[v_m.clone()])?;
        if ({ let t5981 = v_fm.clone(); Py::Bool({ let t5982 = Py::Int(0i64); if py::eq(&t5981, &t5982) { true } else { false } }) }).truthy() {
            return Ok(v_m.clone());
        }
        if ({ let t5983 = { let t5985 = v_fm.clone(); Py::Bool({ let t5986 = Py::Int(0i64); if py::gt(&t5985, &t5986)? { true } else { false } }) }; Py::Bool({ let t5984 = { let t5987 = v_fa.clone(); Py::Bool({ let t5988 = Py::Int(0i64); if py::gt(&t5987, &t5988)? { true } else { false } }) }; if py::eq(&t5983, &t5984) { true } else { false } }) }).truthy() {
            let t5989 = (Py::tuple(vec![v_m.clone(), v_fm.clone()])).items()?;
            if t5989.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t5989[0].clone();
            v_fa = t5989[1].clone();
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
    let _ = f__need({ let t5947 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t5947.truthy() { t5947 } else { { let t5946 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5946.truthy() { t5946 } else { { let t5944 = v_v.clone(); Py::Bool({ let t5945 = v_lo.clone(); if py::ge(&t5944, &t5945)? { true } else { false } }) } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a whole number of at least ".to_string(), py::format(&v_lo.clone(), "")?].concat()))?;
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
    let _ = f__need({ let t1574 = py::sum(&v_E.clone(), None)?; Py::Bool({ let t1575 = Py::Int(1i64); if py::ge(&t1574, &t1575)? { true } else { false } }) }, Py::str("at least one event is needed"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t1576 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t1576;
out.push({ let t1577 = v_t.clone(); Py::Bool({ let t1578 = Py::Int(0i64); if py::gt(&t1577, &t1578)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("times must be positive"))?;
    return Ok(py::truediv(&py::sum(&v_E.clone(), None)?, &py::fsum(&v_T.clone())?)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__fin(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t5956 in py::iter(&(v_vals.clone()))? {
        v_v = t5956;
        let _ = f__need({ let t5958 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t5958.truthy() { t5958 } else { { let t5957 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t5957.truthy() { t5957 } else { py::math1("isfinite", &v_v.clone())? } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a finite number".to_string()].concat()))?;
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
    let _ = f__need({ let t5250 = (if (v_lo_open.clone()).truthy() { { let t5242 = v_v.clone(); Py::Bool({ let t5243 = Py::Int(0i64); if py::gt(&t5242, &t5243)? { true } else { false } }) } } else { { let t5244 = v_v.clone(); Py::Bool({ let t5245 = Py::Int(0i64); if py::ge(&t5244, &t5245)? { true } else { false } }) } }); if !t5250.truthy() { t5250 } else { (if (v_hi_open.clone()).truthy() { { let t5246 = v_v.clone(); Py::Bool({ let t5247 = Py::Int(1i64); if py::lt(&t5246, &t5247)? { true } else { false } }) } } else { { let t5248 = v_v.clone(); Py::Bool({ let t5249 = Py::Int(1i64); if py::le(&t5248, &t5249)? { true } else { false } }) } }) } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a fraction between 0 and 1".to_string()].concat()))?;
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__g(mut v_u: Py) -> R {
    return Ok((if ({ let t2103 = v_u.clone(); Py::Bool({ let t2104 = Py::Int(0i64); if py::eq(&t2103, &t2104) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_u.clone(), &py::neg(&py::math1("expm1", &py::neg(&v_u.clone())?)?)?)? }));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ghk_flux_g(mut v_u: Py) -> R {
    return Ok((if ({ let t1828 = v_u.clone(); Py::Bool({ let t1829 = Py::Int(0i64); if py::eq(&t1828, &t1829) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { py::truediv(&v_u.clone(), &py::neg(&py::math1("expm1", &py::neg(&v_u.clone())?)?)?)? }));
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
    let t2108 = (f__hh_rates(v_v.clone())?).items()?;
    if t2108.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2109 = (t2108[0].clone()).items()?;
    if t2109.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2109[0].clone();
    v_bm = t2109[1].clone();
    let t2110 = (t2108[1].clone()).items()?;
    if t2110.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2110[0].clone();
    v_bh = t2110[1].clone();
    let t2111 = (t2108[2].clone()).items()?;
    if t2111.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2111[0].clone();
    v_bn = t2111[1].clone();
    let t2112 = (Py::tuple(vec![py::truediv(&v_am.clone(), &py::add(&v_am.clone(), &v_bm.clone())?)?, py::truediv(&v_ah.clone(), &py::add(&v_ah.clone(), &v_bh.clone())?)?, py::truediv(&v_an.clone(), &py::add(&v_an.clone(), &v_bn.clone())?)?])).items()?;
    if t2112.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_m = t2112[0].clone();
    v_h = t2112[1].clone();
    v_n = t2112[2].clone();
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
    let t2105 = (Py::tuple(vec![py::mul(&Py::Float(0.1), &f__g(py::truediv(&py::add(&v_v.clone(), &Py::Float(55.0))?, &Py::Float(10.0))?)?)?, py::mul(&Py::Float(0.125), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(80.0))?)?)?])).items()?;
    if t2105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2105[0].clone();
    v_bn = t2105[1].clone();
    let t2106 = (Py::tuple(vec![py::mul(&Py::Float(1.0), &f__g(py::truediv(&py::add(&v_v.clone(), &Py::Float(40.0))?, &Py::Float(10.0))?)?)?, py::mul(&Py::Float(4.0), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(18.0))?)?)?])).items()?;
    if t2106.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2106[0].clone();
    v_bm = t2106[1].clone();
    let t2107 = (Py::tuple(vec![py::mul(&Py::Float(0.07), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(65.0))?)?, &Py::Float(20.0))?)?)?, py::truediv(&Py::Float(1.0), &py::add(&Py::Float(1.0), &py::math1("exp", &py::truediv(&py::neg(&py::add(&v_v.clone(), &Py::Float(35.0))?)?, &Py::Float(10.0))?)?)?)?])).items()?;
    if t2107.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2107[0].clone();
    v_bh = t2107[1].clone();
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
    v_grid = { let mut out = vec![]; for t2113 in py::range_iter(&[(Py::Int(721i64)).i()?]) { let mut v_i: Py; 
v_i = t2113;
out.push(py::add(&py::neg(&Py::Float(120.0))?, &py::mul(&Py::Float(0.25), &v_i.clone())?)?); } Py::list(out) };
    v_vals = { let mut out = vec![]; for t2114 in py::iter(&(v_grid.clone()))? { let mut v_v: Py; 
v_v = t2114;
out.push(py::call(&v_f.clone(), &[v_v.clone()])?); } Py::list(out) };
    v_br = { let mut out = vec![]; for t2115 in py::iter(&(py::zip(&[v_grid.clone(), py::slice(&v_grid.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_vals.clone(), py::slice(&v_vals.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_fa: Py; let mut v_fb: Py; 
let t2116 = (t2115).items()?;
if t2116.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_a = t2116[0].clone();
v_b = t2116[1].clone();
v_fa = t2116[2].clone();
v_fb = t2116[3].clone();
if ({ let t2125 = { let t2117 = { let t2119 = v_fa.clone(); Py::Bool({ let t2120 = Py::Int(0i64); if py::gt(&t2119, &t2120)? { true } else { false } }) }; Py::Bool({ let t2118 = { let t2121 = v_fb.clone(); Py::Bool({ let t2122 = Py::Int(0i64); if py::gt(&t2121, &t2122)? { true } else { false } }) }; if py::ne(&t2117, &t2118) { true } else { false } }) }; if t2125.truthy() { t2125 } else { { let t2123 = v_fa.clone(); Py::Bool({ let t2124 = Py::Int(0i64); if py::eq(&t2123, &t2124) { true } else { false } }) } } }).truthy() { out.push(Py::tuple(vec![v_a.clone(), v_b.clone()])); }} Py::list(out) };
    let _ = f__need({ let t2126 = py::len(&v_br.clone())?; Py::Bool({ let t2127 = Py::Int(1i64); if py::eq(&t2126, &t2127) { true } else { false } }) }, Py::str("the zero-current steady state is not unique between -120 and +60 mV"))?;
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
            let t2154 = (v_y.clone()).items()?;
            if t2154.len() != 4 { return Err(PyErr::Value("unpack".into())); }
            v_vv = t2154[0].clone();
            v_m = t2154[1].clone();
            v_h = t2154[2].clone();
            v_n = t2154[3].clone();
            let t2155 = (f__hh_rates(v_vv.clone())?).items()?;
            if t2155.len() != 3 { return Err(PyErr::Value("unpack".into())); }
            let t2156 = (t2155[0].clone()).items()?;
            if t2156.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_am = t2156[0].clone();
            v_bm = t2156[1].clone();
            let t2157 = (t2155[1].clone()).items()?;
            if t2157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_ah = t2157[0].clone();
            v_bh = t2157[1].clone();
            let t2158 = (t2155[2].clone()).items()?;
            if t2158.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_an = t2158[0].clone();
            v_bn = t2158[1].clone();
            return Ok(Py::list(vec![py::neg(&py::add(&py::add(&py::mul(&py::mul(&py::mul(&py::getitem(&v_g.clone(), &Py::Int(0i64))?, &py::pow(&v_m.clone(), &Py::Int(3i64))?)?, &v_h.clone())?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(0i64))?)?)?, &py::mul(&py::mul(&py::getitem(&v_g.clone(), &Py::Int(1i64))?, &py::pow(&v_n.clone(), &Py::Int(4i64))?)?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(1i64))?)?)?)?, &py::mul(&py::getitem(&v_g.clone(), &Py::Int(2i64))?, &py::sub(&v_vv.clone(), &py::getitem(&v_e.clone(), &Py::Int(2i64))?)?)?)?)?, py::sub(&py::mul(&v_am.clone(), &py::sub(&Py::Int(1i64), &v_m.clone())?)?, &py::mul(&v_bm.clone(), &v_m.clone())?)?, py::sub(&py::mul(&v_ah.clone(), &py::sub(&Py::Int(1i64), &v_h.clone())?)?, &py::mul(&v_bh.clone(), &v_h.clone())?)?, py::sub(&py::mul(&v_an.clone(), &py::sub(&Py::Int(1i64), &v_n.clone())?)?, &py::mul(&v_bn.clone(), &v_n.clone())?)?]));
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
    let t2159 = (f__hh_rates(v_v.clone())?).items()?;
    if t2159.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2160 = (t2159[0].clone()).items()?;
    if t2160.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t2160[0].clone();
    v_bm = t2160[1].clone();
    let t2161 = (t2159[1].clone()).items()?;
    if t2161.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t2161[0].clone();
    v_bh = t2161[1].clone();
    let t2162 = (t2159[2].clone()).items()?;
    if t2162.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t2162[0].clone();
    v_bn = t2162[1].clone();
    v_y0 = Py::list(vec![v_v.clone(), py::truediv(&v_am.clone(), &py::add(&v_am.clone(), &v_bm.clone())?)?, py::truediv(&v_ah.clone(), &py::add(&v_ah.clone(), &v_bh.clone())?)?, py::truediv(&v_an.clone(), &py::add(&v_an.clone(), &v_bn.clone())?)?]);
    v_J = { let mut out = vec![]; for t2163 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v__: Py; 
v__ = t2163;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &Py::Int(4i64))?); } Py::list(out) };
    for t2164 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
        v_j = t2164;
        v_hstep = py::mul(&Py::Float(1e-06), &(if ({ let t2165 = v_j.clone(); Py::Bool({ let t2166 = Py::Int(0i64); if py::eq(&t2165, &t2166) { true } else { false } }) }).truthy() { Py::Float(1.0) } else { Py::Float(0.01) }))?;
        let t2167 = (Py::tuple(vec![Py::list((v_y0.clone()).items()?), Py::list((v_y0.clone()).items()?)])).items()?;
        if t2167.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_yp = t2167[0].clone();
        v_ym = t2167[1].clone();
        let t2168 = v_yp.clone(); let t2169 = v_j.clone();
        py::setitem(&t2168, &t2169, py::iadd(&py::getitem(&t2168, &t2169)?, &v_hstep.clone())?)?;
        let t2170 = v_ym.clone(); let t2171 = v_j.clone();
        py::setitem(&t2170, &t2171, py::sub(&py::getitem(&t2170, &t2171)?, &v_hstep.clone())?)?;
        let t2172 = (Py::tuple(vec![py::call(&v_F.clone(), &[v_yp.clone()])?, py::call(&v_F.clone(), &[v_ym.clone()])?])).items()?;
        if t2172.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_fp = t2172[0].clone();
        v_fm = t2172[1].clone();
        for t2173 in py::range_iter(&[(Py::Int(4i64)).i()?]) {
            v_i = t2173;
            py::setitem(&py::getitem(&v_J.clone(), &v_i.clone())?, &v_j.clone(), py::truediv(&py::sub(&py::getitem(&v_fp.clone(), &v_i.clone())?, &py::getitem(&v_fm.clone(), &v_i.clone())?)?, &py::mul(&Py::Int(2i64), &v_hstep.clone())?)?)?;
        }
    }
    let t2175 = (Py::tuple(vec![Py::Int(4i64), { let mut out = vec![]; for t2174 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v__: Py; 
v__ = t2174;
out.push(py::mul(&Py::list(vec![Py::Float(0.0)]), &Py::Int(4i64))?); } Py::list(out) }, Py::list(vec![Py::Float(1.0)])])).items()?;
    if t2175.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_n = t2175[0].clone();
    v_M = t2175[1].clone();
    v_c = t2175[2].clone();
    for t2176 in py::range_iter(&[(Py::Int(1i64)).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
        v_k = t2176;
        v_AM = { let mut out = vec![]; for t2177 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2177;
out.push({ let mut out = vec![]; for t2178 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t2178;
out.push(py::sum(&{ let mut out = vec![]; for t2179 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_l: Py; 
v_l = t2179;
out.push(py::mul(&py::getitem(&py::getitem(&v_J.clone(), &v_i.clone())?, &v_l.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_l.clone())?, &v_j.clone())?)?); } Py::list(out) }, None)?); } Py::list(out) }); } Py::list(out) };
        if ({ let t2180 = v_k.clone(); Py::Bool({ let t2181 = Py::Int(1i64); if py::eq(&t2180, &t2181) { true } else { false } }) }).truthy() {
            v_AM = { let mut out = vec![]; for t2182 in py::iter(&(v_J.clone()))? { let mut v_row: Py; 
v_row = t2182;
out.push(py::slice(&v_row.clone(), None, None, None)?); } Py::list(out) };
        }
        v_ck = py::truediv(&py::neg(&py::sum(&{ let mut out = vec![]; for t2183 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2183;
out.push(py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_i.clone())?); } Py::list(out) }, None)?)?, &v_k.clone())?;
        let _ = py::method(&v_c.clone(), "append", &[v_ck.clone()])?;
        v_M = { let mut out = vec![]; for t2184 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_i: Py; 
v_i = t2184;
out.push({ let mut out = vec![]; for t2185 in py::range_iter(&[(Py::Int(4i64)).i()?]) { let mut v_j: Py; 
v_j = t2185;
out.push(py::add(&py::getitem(&py::getitem(&v_AM.clone(), &v_i.clone())?, &v_j.clone())?, &(if ({ let t2186 = v_i.clone(); Py::Bool({ let t2187 = v_j.clone(); if py::eq(&t2186, &t2187) { true } else { false } }) }).truthy() { v_ck.clone() } else { Py::Float(0.0) }))?); } Py::list(out) }); } Py::list(out) };
    }
    let t2188 = (py::slice(&v_c.clone(), Some((Py::Int(1i64)).i()?), None, None)?).items()?;
    if t2188.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_c1 = t2188[0].clone();
    v_c2 = t2188[1].clone();
    v_c3 = t2188[2].clone();
    v_c4 = t2188[3].clone();
    return Ok({ let t2205 = { let t2189 = v_c1.clone(); Py::Bool({ let t2190 = Py::Int(0i64); if py::gt(&t2189, &t2190)? { true } else { false } }) }; if !t2205.truthy() { t2205 } else { { let t2204 = { let t2191 = v_c2.clone(); Py::Bool({ let t2192 = Py::Int(0i64); if py::gt(&t2191, &t2192)? { true } else { false } }) }; if !t2204.truthy() { t2204 } else { { let t2203 = { let t2193 = v_c3.clone(); Py::Bool({ let t2194 = Py::Int(0i64); if py::gt(&t2193, &t2194)? { true } else { false } }) }; if !t2203.truthy() { t2203 } else { { let t2202 = { let t2195 = v_c4.clone(); Py::Bool({ let t2196 = Py::Int(0i64); if py::gt(&t2195, &t2196)? { true } else { false } }) }; if !t2202.truthy() { t2202 } else { { let t2201 = { let t2197 = py::mul(&v_c1.clone(), &v_c2.clone())?; Py::Bool({ let t2198 = v_c3.clone(); if py::gt(&t2197, &t2198)? { true } else { false } }) }; if !t2201.truthy() { t2201 } else { { let t2199 = py::mul(&py::mul(&v_c1.clone(), &v_c2.clone())?, &v_c3.clone())?; Py::Bool({ let t2200 = py::add(&py::mul(&v_c3.clone(), &v_c3.clone())?, &py::mul(&py::mul(&v_c1.clone(), &v_c1.clone())?, &v_c4.clone())?)?; if py::gt(&t2199, &t2200)? { true } else { false } }) } } } } } } } } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__isnum(mut v_v: Py) -> R {
    return Ok({ let t2744 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t2744.truthy() { t2744 } else { { let t2743 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t2743.truthy() { t2743 } else { py::math1("isfinite", &v_v.clone())? } } } });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__ka_k(mut v_ka: Py, mut v_k: Py) -> R {
    let _ = f__pos(Py::str("the rate constants ka and k"), Py::tuple(vec![v_ka.clone(), v_k.clone()]))?;
    let _ = f__need({ let t5099 = py::abs(&py::sub(&v_ka.clone(), &v_k.clone())?)?; Py::Bool({ let t5100 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_ka.clone(), v_k.clone()], true)?)?; if py::gt(&t5099, &t5100)? { true } else { false } }) }, Py::str("ka equals k: the Bateman form does not apply (ka = k is not covered)"))?;
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
    let t2655 = (Py::tuple(vec![py::fraction(&[Py::Int(1i64)])?, Py::list(vec![])])).items()?;
    if t2655.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2655[0].clone();
    v_curve = t2655[1].clone();
    for t2671 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t2672 = (t2671).items()?;
        if t2672.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2672[0].clone();
        v_n = t2672[1].clone();
        v_d = t2672[2].clone();
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
    for t5745 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t5741 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t5742 = (t5741).items()?;
if t5742.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t5742[0].clone();
v_e = t5742[1].clone();
if ({ let t5743 = v_e.clone(); Py::Bool({ let t5744 = Py::Int(1i64); if py::eq(&t5743, &t5744) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t5745;
        let _ = py::method(&v_out.clone(), "append", &[Py::tuple(vec![v_u.clone(), py::sum(&{ let mut out = vec![]; for t5746 in py::iter(&(v_T.clone()))? { let mut v_t: Py; 
v_t = t5746;
if ({ let t5747 = v_t.clone(); Py::Bool({ let t5748 = v_u.clone(); if py::ge(&t5747, &t5748)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t5749 in py::iter(&(py::zip(&[v_T.clone(), v_E.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t5750 = (t5749).items()?;
if t5750.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t5750[0].clone();
v_e = t5750[1].clone();
if ({ let t5755 = { let t5751 = v_t.clone(); Py::Bool({ let t5752 = v_u.clone(); if py::eq(&t5751, &t5752) { true } else { false } }) }; if !t5755.truthy() { t5755 } else { { let t5753 = v_e.clone(); Py::Bool({ let t5754 = Py::Int(1i64); if py::eq(&t5753, &t5754) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])])?;
    }
    return Ok(v_out.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__level(mut v_level: Py) -> R {
    let _ = f__fin(Py::str("the confidence level"), Py::tuple(vec![v_level.clone()]))?;
    let _ = f__need({ let t5959 = Py::Float(0.5); Py::Bool({ let t5960 = v_level.clone(); if py::le(&t5959, &t5960)? { { let t5961 = Py::Int(1i64); if py::lt(&t5960, &t5961)? { true } else { false } } } else { false } }) }, Py::str("the confidence level must be a fraction such as 0.95 (or 95%)"))?;
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
    let _ = f__need({ let t5163 = py::len(&v_t.clone())?; Py::Bool({ let t5164 = py::len(&v_y.clone())?; if py::eq(&t5163, &t5164) { true } else { false } }) }, Py::str("give one value per time"))?;
    let _ = f__need({ let t5165 = py::len(&v_t.clone())?; Py::Bool({ let t5166 = Py::Int(2i64); if py::ge(&t5165, &t5166)? { true } else { false } }) }, Py::str("give at least two points"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t5167 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t5167;
out.push({ let t5168 = v_v.clone(); Py::Bool({ let t5169 = Py::Int(0i64); if py::gt(&t5168, &t5169)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_name.clone(), "")?, " must be positive (a logarithm is taken)".to_string()].concat()))?;
    let _ = f__need({ let t5170 = py::len(&py::set(&v_t.clone())?)?; Py::Bool({ let t5171 = Py::Int(2i64); if py::ge(&t5170, &t5171)? { true } else { false } }) }, Py::str("the times must not all be equal"))?;
    v_ly = { let mut out = vec![]; for t5172 in py::iter(&(v_y.clone()))? { let mut v_v: Py; 
v_v = t5172;
out.push(py::math1("log", &v_v.clone())?); } Py::list(out) };
    v_n = py::len(&v_t.clone())?;
    let t5173 = (Py::tuple(vec![py::truediv(&py::fsum(&v_t.clone())?, &v_n.clone())?, py::truediv(&py::fsum(&v_ly.clone())?, &v_n.clone())?])).items()?;
    if t5173.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_mt = t5173[0].clone();
    v_my = t5173[1].clone();
    v_sxx = py::fsum(&{ let mut out = vec![]; for t5174 in py::iter(&(v_t.clone()))? { let mut v_a: Py; 
v_a = t5174;
out.push(py::pow(&py::sub(&v_a.clone(), &v_mt.clone())?, &Py::Int(2i64))?); } Py::list(out) })?;
    v_sxy = py::fsum(&{ let mut out = vec![]; for t5175 in py::iter(&(py::zip(&[v_t.clone(), v_ly.clone()])?))? { let mut v_a: Py; let mut v_b: Py; 
let t5176 = (t5175).items()?;
if t5176.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t5176[0].clone();
v_b = t5176[1].clone();
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
    let _ = f__need({ let t3456 = { let t3452 = py::len(&v_x.clone())?; Py::Bool({ let t3453 = py::len(&v_S.clone())?; if py::eq(&t3452, &t3453) { true } else { false } }) }; if !t3456.truthy() { t3456 } else { { let t3454 = py::len(&v_x.clone())?; Py::Bool({ let t3455 = Py::Int(2i64); if py::ge(&t3454, &t3455)? { true } else { false } }) } } }, Py::str("give at least two points and one signal per point"))?;
    let t3471 = (f__loglin(v_x.clone(), v_S.clone(), Py::str("the signals"))?).items()?;
    if t3471.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t3471[0].clone();
    v_b = t3471[1].clone();
    let _ = f__need({ let t3472 = v_slope.clone(); Py::Bool({ let t3473 = Py::Int(0i64); if py::lt(&t3472, &t3473)? { true } else { false } }) }, Py::str(&["the signal does not decay with ".to_string(), py::format(&v_name.clone(), "")?].concat()))?;
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
    for t6304 in py::iter(&(v_vals.clone()))? {
        v_v = t6304;
        let _ = f__need({ let t6309 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t6309.truthy() { t6309 } else { { let t6308 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t6308.truthy() { t6308 } else { { let t6307 = py::math1("isfinite", &v_v.clone())?; if !t6307.truthy() { t6307 } else { { let t6305 = v_v.clone(); Py::Bool({ let t6306 = Py::Int(0i64); if py::ge(&t6305, &t6306)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be zero or positive".to_string()].concat()))?;
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
    v_A = { let mut out = vec![]; for t5641 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t5641;
out.push({ let mut out = vec![]; for t5642 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_j: Py; 
v_j = t5642;
out.push(py::fsum(&{ let mut out = vec![]; for t5643 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t5643;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&py::getitem(&v_X.clone(), &v_j.clone())?, &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) }); } Py::list(out) };
    v_b = { let mut out = vec![]; for t5644 in py::range_iter(&[(v_p.clone()).i()?]) { let mut v_i: Py; 
v_i = t5644;
out.push(py::fsum(&{ let mut out = vec![]; for t5645 in py::range_iter(&[(py::len(&v_y.clone())?).i()?]) { let mut v_k: Py; 
v_k = t5645;
out.push(py::mul(&py::getitem(&py::getitem(&v_X.clone(), &v_i.clone())?, &v_k.clone())?, &py::getitem(&v_y.clone(), &v_k.clone())?)?); } Py::list(out) })?); } Py::list(out) };
    return Ok(f__solve_lin(v_A.clone(), v_b.clone())?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__out(mut v_kw: Py) -> R {
    return Ok({ let mut out = vec![]; for t6455 in py::iter(&(py::method(&v_kw.clone(), "items", &[])?))? { let mut v_k: Py; let mut v_v: Py; 
let t6456 = (t6455).items()?;
if t6456.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t6456[0].clone();
v_v = t6456[1].clone();
out.push((v_k.clone(), f__r(v_v.clone())?)); } Py::dict(out) });
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pois_cdf(mut v_k: Py, mut v_lam: Py) -> R {
    let mut v_ll: Py = Py::None;
    let mut v_mx: Py = Py::None;
    let mut v_terms: Py = Py::None;
    if ({ let t5990 = v_lam.clone(); Py::Bool({ let t5991 = Py::Int(0i64); if py::eq(&t5990, &t5991) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(1.0));
    }
    v_ll = py::math1("log", &v_lam.clone())?;
    v_terms = { let mut out = vec![]; for t5992 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t5992;
out.push(py::sub(&py::add(&py::neg(&v_lam.clone())?, &py::mul(&v_j.clone(), &v_ll.clone())?)?, &py::math1("lgamma", &py::add(&v_j.clone(), &Py::Int(1i64))?)?)?); } Py::list(out) };
    v_mx = py::minmax(&[v_terms.clone()], true)?;
    return Ok(py::minmax(&[Py::Float(1.0), py::mul(&py::math1("exp", &v_mx.clone())?, &py::fsum(&{ let mut out = vec![]; for t5993 in py::iter(&(v_terms.clone()))? { let mut v_t: Py; 
v_t = t5993;
out.push(py::math1("exp", &py::sub(&v_t.clone(), &v_mx.clone())?)?); } Py::list(out) })?)?], false)?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__pos(mut v_name: Py, mut v_vals: Py) -> R {
    let mut v_v: Py = Py::None;
    for t6433 in py::iter(&(v_vals.clone()))? {
        v_v = t6433;
        let _ = f__need({ let t6438 = Py::Bool(matches!(v_v.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_) | Py::Float(_))); if !t6438.truthy() { t6438 } else { { let t6437 = py::not(&Py::Bool(matches!(v_v.clone(), Py::Bool(_)))); if !t6437.truthy() { t6437 } else { { let t6436 = py::math1("isfinite", &v_v.clone())?; if !t6436.truthy() { t6436 } else { { let t6434 = v_v.clone(); Py::Bool({ let t6435 = Py::Int(0i64); if py::gt(&t6434, &t6435)? { true } else { false } }) } } } } } } }, Py::str(&[py::format(&v_name.clone(), "")?, " must be a positive number".to_string()].concat()))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__r(mut v_x: Py) -> R {
    if ({ let t6460 = Py::Bool(matches!(v_x.clone(), Py::Bool(_))); if t6460.truthy() { t6460 } else { { let t6459 = Py::Bool(matches!(v_x.clone(), Py::Str(_))); if t6459.truthy() { t6459 } else { { let t6457 = v_x.clone(); Py::Bool({ let t6458 = Py::None; if py::is(&t6457, &t6458) { true } else { false } }) } } } } }).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_)))).truthy() {
        return Ok(v_x.clone());
    }
    if (Py::Bool(matches!(v_x.clone(), Py::List(_) | Py::Tuple(_)))).truthy() {
        return Ok({ let mut out = vec![]; for t6461 in py::iter(&(v_x.clone()))? { let mut v_v: Py; 
v_v = t6461;
out.push(f__r(v_v.clone())?); } Py::list(out) });
    }
    if ({ let t6464 = { let t6462 = v_x.clone(); Py::Bool({ let t6463 = Py::Int(0i64); if py::eq(&t6462, &t6463) { true } else { false } }) }; if t6464.truthy() { t6464 } else { py::not(&py::math1("isfinite", &v_x.clone())?) } }).truthy() {
        return Ok((if ({ let t6465 = v_x.clone(); Py::Bool({ let t6466 = Py::Int(0i64); if py::eq(&t6465, &t6466) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { v_x.clone() }));
    }
    return Ok(py::add(&py::float(&Py::str(&[py::format(&v_x.clone(), ".12g")?].concat()))?, &Py::Float(0.0))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__rel(mut v_a: Py, mut v_b: Py) -> R {
    let mut v_d: Py = Py::None;
    if ({ let t6479 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if t6479.truthy() { t6479 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }).truthy() {
        if ({ let t6483 = py::not(&{ let t6480 = Py::Bool(matches!(v_a.clone(), Py::List(_) | Py::Tuple(_))); if !t6480.truthy() { t6480 } else { Py::Bool(matches!(v_b.clone(), Py::List(_) | Py::Tuple(_))) } }); if t6483.truthy() { t6483 } else { { let t6481 = py::len(&v_a.clone())?; Py::Bool({ let t6482 = py::len(&v_b.clone())?; if py::ne(&t6481, &t6482) { true } else { false } }) } } }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        return Ok(py::minmax(&[py::add(&{ let mut out = vec![]; for t6484 in py::iter(&(py::zip(&[v_a.clone(), v_b.clone()])?))? { let mut v_x: Py; let mut v_y: Py; 
let t6485 = (t6484).items()?;
if t6485.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_x = t6485[0].clone();
v_y = t6485[1].clone();
out.push(f__rel(v_x.clone(), v_y.clone())?); } Py::list(out) }, &Py::list(vec![Py::Float(0.0)]))?], true)?);
    }
    if ({ let t6492 = Py::Bool(matches!(v_a.clone(), Py::Str(_) | Py::Bool(_))); if t6492.truthy() { t6492 } else { { let t6491 = Py::Bool(matches!(v_b.clone(), Py::Str(_) | Py::Bool(_))); if t6491.truthy() { t6491 } else { { let t6490 = { let t6486 = v_a.clone(); Py::Bool({ let t6487 = Py::None; if py::is(&t6486, &t6487) { true } else { false } }) }; if t6490.truthy() { t6490 } else { { let t6488 = v_b.clone(); Py::Bool({ let t6489 = Py::None; if py::is(&t6488, &t6489) { true } else { false } }) } } } } } } }).truthy() {
        return Ok((if ({ let t6493 = v_a.clone(); Py::Bool({ let t6494 = v_b.clone(); if py::eq(&t6493, &t6494) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    let t6496: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
            let t6495 = (Py::tuple(vec![py::float(&v_a.clone())?, py::float(&v_b.clone())?])).items()?;
            if t6495.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_a = t6495[0].clone();
            v_b = t6495[1].clone();
            #[allow(unreachable_code)]
            Ok(None)
        })();
    match t6496 {
        Ok(Some(r)) => { return Ok(r); }
        Ok(None) => {}
        Err(e) if ((matches!(e, PyErr::Type(_)) || matches!(&e, PyErr::Other(c) if c.as_str() == "TypeError"))) || (matches!(e, PyErr::Value(_))) => {
            return Ok(Py::Float(f64::INFINITY));
        }
        Err(e) => return Err(e),
    }
    if (py::not(&{ let t6497 = py::math1("isfinite", &v_a.clone())?; if !t6497.truthy() { t6497 } else { py::math1("isfinite", &v_b.clone())? } })).truthy() {
        return Ok((if ({ let t6498 = v_a.clone(); Py::Bool({ let t6499 = v_b.clone(); if py::eq(&t6498, &t6499) { true } else { false } }) }).truthy() { Py::Float(0.0) } else { Py::Float(f64::INFINITY) }));
    }
    v_d = py::abs(&py::sub(&v_a.clone(), &v_b.clone())?)?;
    if ({ let t6500 = v_d.clone(); Py::Bool({ let t6501 = Py::Int(0i64); if py::eq(&t6500, &t6501) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(0.0));
    }
    if ({ let t6506 = { let t6502 = py::abs(&v_a.clone())?; Py::Bool({ let t6503 = Py::Float(1e-15); if py::lt(&t6502, &t6503)? { true } else { false } }) }; if !t6506.truthy() { t6506 } else { { let t6504 = py::abs(&v_b.clone())?; Py::Bool({ let t6505 = Py::Float(1e-15); if py::lt(&t6504, &t6505)? { true } else { false } }) } } }).truthy() {
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
    let t1132 = (Py::tuple(vec![v_s0.clone(), v_e0.clone(), v_i0.clone(), Py::Float(0.0)])).items()?;
    if t1132.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_s = t1132[0].clone();
    v_e = t1132[1].clone();
    v_i = t1132[2].clone();
    v_tau = t1132[3].clone();
    v_risen = { let t1133 = py::sub(&py::mul(&v_c.clone(), &v_e.clone())?, &v_i.clone())?; Py::Bool({ let t1134 = Py::Int(0i64); if py::gt(&t1133, &t1134)? { true } else { false } }) };
    for t1135 in py::range_iter(&[(Py::Int(200000i64)).i()?]) {
        v__ = t1135;
        let t1136 = (Py::tuple(vec![Py::list(vec![v_s.clone()]), Py::list(vec![v_e.clone()]), Py::list(vec![v_i.clone()])])).items()?;
        if t1136.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_S = t1136[0].clone();
        v_E = t1136[1].clone();
        v_I = t1136[2].clone();
        for t1137 in py::range_iter(&[(v_K.clone()).i()?]) {
            v_k = t1137;
            v_p = py::fsum(&{ let mut out = vec![]; for t1138 in py::range_iter(&[(py::add(&v_k.clone(), &Py::Int(1i64))?).i()?]) { let mut v_j: Py; 
v_j = t1138;
out.push(py::mul(&py::getitem(&v_S.clone(), &v_j.clone())?, &py::getitem(&v_I.clone(), &py::sub(&v_k.clone(), &v_j.clone())?)?)?); } Py::list(out) })?;
            let _ = py::method(&v_S.clone(), "append", &[py::truediv(&py::mul(&py::neg(&v_b.clone())?, &v_p.clone())?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
            let _ = py::method(&v_E.clone(), "append", &[py::truediv(&py::sub(&py::mul(&v_b.clone(), &v_p.clone())?, &py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
            let _ = py::method(&v_I.clone(), "append", &[py::truediv(&py::sub(&py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?, &py::getitem(&v_I.clone(), &v_k.clone())?)?, &py::add(&v_k.clone(), &Py::Int(1i64))?)?])?;
        }
        v_h = Py::Float(2.0);
        for t1139 in py::iter(&(Py::tuple(vec![v_S.clone(), v_E.clone(), v_I.clone()])))? {
            v_X = t1139;
            v_top = py::minmax(&[py::abs(&py::getitem(&v_X.clone(), &v_K.clone())?)?, (if (py::getitem(&v_X.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?).truthy() { py::pow(&py::abs(&py::getitem(&v_X.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?)?, &py::truediv(&v_K.clone(), &py::sub(&v_K.clone(), &Py::Int(1i64))?)?)? } else { Py::Float(0.0) })], true)?;
            if ({ let t1144 = { let t1140 = v_top.clone(); Py::Bool({ let t1141 = Py::Int(0i64); if py::gt(&t1140, &t1141)? { true } else { false } }) }; if !t1144.truthy() { t1144 } else { { let t1142 = py::getitem(&v_X.clone(), &Py::Int(0i64))?; Py::Bool({ let t1143 = Py::Int(0i64); if py::gt(&t1142, &t1143)? { true } else { false } }) } } }).truthy() {
                v_h = py::minmax(&[v_h.clone(), py::pow(&py::truediv(&py::mul(&Py::Float(1e-17), &py::getitem(&v_X.clone(), &Py::Int(0i64))?)?, &v_top.clone())?, &py::truediv(&Py::Float(1.0), &v_K.clone())?)?], false)?;
            }
        }
        v_G = { let mut out = vec![]; for t1145 in py::range_iter(&[(py::add(&v_K.clone(), &Py::Int(1i64))?).i()?]) { let mut v_k: Py; 
v_k = t1145;
out.push(py::sub(&py::mul(&v_c.clone(), &py::getitem(&v_E.clone(), &v_k.clone())?)?, &py::getitem(&v_I.clone(), &v_k.clone())?)?); } Py::list(out) };
        v_ev = { let v_E = v_E.clone(); let v_G = v_G.clone(); let v_I = v_I.clone(); let v_K = v_K.clone(); let v_S = v_S.clone(); let v_X = v_X.clone(); let v__ = v__.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_e = v_e.clone(); let v_e0 = v_e0.clone(); let v_h = v_h.clone(); let v_i = v_i.clone(); let v_i0 = v_i0.clone(); let v_k = v_k.clone(); let v_p = v_p.clone(); let v_risen = v_risen.clone(); let v_s = v_s.clone(); let v_s0 = v_s0.clone(); let v_tau = v_tau.clone(); let v_top = v_top.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_P: Py = a.get(0).cloned().unwrap_or(Py::None);
            let mut v_x: Py = a.get(1).cloned().unwrap_or(Py::None);
            return Ok(py::sum(&{ let mut out = vec![]; for t1146 in py::range_iter(&[(py::len(&v_P.clone())?).i()?]) { let mut v_k: Py; 
v_k = t1146;
out.push(py::mul(&py::getitem(&v_P.clone(), &v_k.clone())?, &py::pow(&v_x.clone(), &v_k.clone())?)?); } Py::list(out) }, None)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) };
        v_g_end = py::call(&v_ev.clone(), &[v_G.clone(), v_h.clone()])?;
        if ({ let t1149 = py::not(&v_risen.clone()); if !t1149.truthy() { t1149 } else { { let t1147 = v_g_end.clone(); Py::Bool({ let t1148 = Py::Int(0i64); if py::gt(&t1147, &t1148)? { true } else { false } }) } } }).truthy() {
            v_risen = Py::Bool(true);
        } else {
            if ({ let t1152 = v_risen.clone(); if !t1152.truthy() { t1152 } else { { let t1150 = v_g_end.clone(); Py::Bool({ let t1151 = Py::Int(0i64); if py::le(&t1150, &t1151)? { true } else { false } }) } } }).truthy() {
                let t1153 = (Py::tuple(vec![Py::Float(0.0), v_h.clone()])).items()?;
                if t1153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
                v_lo = t1153[0].clone();
                v_hi = t1153[1].clone();
                for t1154 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
                    v__ = t1154;
                    v_m = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
                    if ({ let t1155 = v_m.clone(); Py::Bool({ let t1156 = Py::tuple(vec![v_lo.clone(), v_hi.clone()]); if py::contains(&t1156, &t1155)? { true } else { false } }) }).truthy() {
                        break;
                    }
                    if ({ let t1157 = py::call(&v_ev.clone(), &[v_G.clone(), v_m.clone()])?; Py::Bool({ let t1158 = Py::Int(0i64); if py::gt(&t1157, &t1158)? { true } else { false } }) }).truthy() {
                        v_lo = v_m.clone();
                    } else {
                        v_hi = v_m.clone();
                    }
                }
                v_th = py::mul(&Py::Float(0.5), &py::add(&v_lo.clone(), &v_hi.clone())?)?;
                return Ok(Py::tuple(vec![py::add(&v_tau.clone(), &v_th.clone())?, py::call(&v_ev.clone(), &[v_S.clone(), v_th.clone()])?, py::call(&v_ev.clone(), &[v_E.clone(), v_th.clone()])?, py::call(&v_ev.clone(), &[v_I.clone(), v_th.clone()])?]));
            }
        }
        let t1159 = (Py::tuple(vec![py::call(&v_ev.clone(), &[v_S.clone(), v_h.clone()])?, py::call(&v_ev.clone(), &[v_E.clone(), v_h.clone()])?, py::call(&v_ev.clone(), &[v_I.clone(), v_h.clone()])?])).items()?;
        if t1159.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_s = t1159[0].clone();
        v_e = t1159[1].clone();
        v_i = t1159[2].clone();
        v_tau = py::iadd(&v_tau, &v_h.clone())?;
        if ({ let t1162 = py::not(&v_risen.clone()); if !t1162.truthy() { t1162 } else { { let t1160 = py::mul(&v_b.clone(), &v_s.clone())?; Py::Bool({ let t1161 = Py::Float(1.0); if py::le(&t1160, &t1161)? { true } else { false } }) } } }).truthy() {
            break;
        }
        if ({ let t1163 = v_tau.clone(); Py::Bool({ let t1164 = Py::Float(100000.0); if py::gt(&t1163, &t1164)? { true } else { false } }) }).truthy() {
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
        let t1354 = (Py::tuple(vec![ax_engine::pymp::call("mpf", &[v_beta.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_gamma.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_s0.clone()], &[])?, ax_engine::pymp::call("mpf", &[v_i0.clone()], &[])?])).items()?;
        if t1354.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_b = t1354[0].clone();
        v_g = t1354[1].clone();
        v_S0 = t1354[2].clone();
        v_I0 = t1354[3].clone();
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
        v_pts = py::add(&py::add(&Py::list(vec![v_sp.clone()]), &{ let mut out = vec![]; for t1355 in py::iter(&(Py::tuple(vec![Py::Int(1000i64), Py::Int(100i64), Py::Int(10i64), Py::Int(1i64)])))? { let mut v_m: Py; 
v_m = t1355;
if ({ let t1356 = py::sub(&v_S0.clone(), &py::mul(&v_I0.clone(), &v_m.clone())?)?; Py::Bool({ let t1357 = v_sp.clone(); if py::gt(&t1356, &t1357)? { true } else { false } }) }).truthy() { out.push(py::sub(&v_S0.clone(), &py::mul(&v_I0.clone(), &v_m.clone())?)?); }} Py::list(out) })?, &Py::list(vec![v_S0.clone()]))?;
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
    v_M = { let mut out = vec![]; for t5646 in py::iter(&(py::enumerate(&v_A.clone(), 0)?))? { let mut v_i: Py; let mut v_row: Py; 
let t5647 = (t5646).items()?;
if t5647.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_i = t5647[0].clone();
v_row = t5647[1].clone();
out.push(py::add(&Py::list((py::map(&py::func(|a: &[Py]| -> R { let x = a.first().cloned().ok_or_else(|| PyErr::Type("float() takes one argument".into()))?; py::float(&x) }), &v_row.clone())?).items()?), &Py::list(vec![py::float(&py::getitem(&v_b.clone(), &v_i.clone())?)?]))?); } Py::list(out) };
    v_scale = { let t5650 = { let t = { let mut out = vec![]; for t5648 in py::iter(&(v_A.clone()))? { let mut v_row: Py; 
v_row = t5648;
for t5649 in py::iter(&(v_row.clone()))? { let mut v_x: Py; 
v_x = t5649;
out.push(py::abs(&v_x.clone())?); }} Py::list(out) }; if t.items()?.is_empty() { Py::Float(1.0) } else { py::minmax(&[t], true)? } }; if t5650.truthy() { t5650 } else { Py::Float(1.0) } };
    for t5651 in py::range_iter(&[(v_n.clone()).i()?]) {
        v_c = t5651;
        v_p = py::minmax_key(&[Py::list((py::range_iter(&[(v_c.clone()).i()?, (v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>())], &{ let v_A = v_A.clone(); let v_M = v_M.clone(); let v_b = v_b.clone(); let v_c = v_c.clone(); let v_n = v_n.clone(); let v_scale = v_scale.clone(); py::func(move |a: &[Py]| -> R {
            #[allow(unused_mut, unused_variables, unused_assignments)]
{
            let mut v_r: Py = a.get(0).cloned().unwrap_or(Py::None);
            return Ok(py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?)?);
            #[allow(unreachable_code)]
            Ok(Py::None)
        }
        }) }, true)?;
        let _ = f__need({ let t5652 = py::abs(&py::getitem(&py::getitem(&v_M.clone(), &v_p.clone())?, &v_c.clone())?)?; Py::Bool({ let t5653 = py::mul(&Py::Float(1e-12), &v_scale.clone())?; if py::gt(&t5652, &t5653)? { true } else { false } }) }, Py::str("the equations are singular (the data do not determine the parameters)"))?;
        let t5654 = (Py::tuple(vec![py::getitem(&v_M.clone(), &v_p.clone())?, py::getitem(&v_M.clone(), &v_c.clone())?])).items()?;
        if t5654.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        py::setitem(&v_M.clone(), &v_c.clone(), t5654[0].clone())?;
        py::setitem(&v_M.clone(), &v_p.clone(), t5654[1].clone())?;
        for t5655 in py::range_iter(&[(py::add(&v_c.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) {
            v_r = t5655;
            v_f = py::truediv(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_c.clone())?, &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_c.clone())?)?;
            if ({ let t5656 = v_f.clone(); Py::Bool({ let t5657 = Py::Int(0i64); if py::ne(&t5656, &t5657) { true } else { false } }) }).truthy() {
                for t5658 in py::range_iter(&[(v_c.clone()).i()?, (py::add(&v_n.clone(), &Py::Int(1i64))?).i()?]) {
                    v_k = t5658;
                    let t5659 = py::getitem(&v_M.clone(), &v_r.clone())?; let t5660 = v_k.clone();
                    py::setitem(&t5659, &t5660, py::sub(&py::getitem(&t5659, &t5660)?, &py::mul(&v_f.clone(), &py::getitem(&py::getitem(&v_M.clone(), &v_c.clone())?, &v_k.clone())?)?)?)?;
                }
            }
        }
    }
    v_x = py::mul(&Py::list(vec![Py::Float(0.0)]), &v_n.clone())?;
    for t5661 in py::iter(&(py::reversed(&Py::list((py::range_iter(&[(v_n.clone()).i()?])).into_iter().collect::<Vec<Py>>()))?))? {
        v_r = t5661;
        py::setitem(&v_x.clone(), &v_r.clone(), py::truediv(&py::sub(&py::getitem(&py::getitem(&v_M.clone(), &v_r.clone())?, &v_n.clone())?, &py::sum(&{ let mut out = vec![]; for t5662 in py::range_iter(&[(py::add(&v_r.clone(), &Py::Int(1i64))?).i()?, (v_n.clone()).i()?]) { let mut v_k: Py; 
v_k = t5662;
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
    let _ = f__need({ let t5723 = { let t5719 = py::len(&v_times.clone())?; Py::Bool({ let t5720 = py::len(&v_events.clone())?; if py::eq(&t5719, &t5720) { true } else { false } }) }; if !t5723.truthy() { t5723 } else { { let t5721 = py::len(&v_times.clone())?; Py::Bool({ let t5722 = Py::Int(1i64); if py::ge(&t5721, &t5722)? { true } else { false } }) } } }, Py::str(&["give one event indicator per time (".to_string(), py::format(&v_label.clone(), "")?, ")".to_string()].concat()))?;
    for t5724 in py::iter(&(v_times.clone()))? {
        v_v = t5724;
        let _ = f__need({ let t5727 = py::math1("isfinite", &v_v.clone())?; if !t5727.truthy() { t5727 } else { { let t5725 = v_v.clone(); Py::Bool({ let t5726 = Py::Int(0i64); if py::ge(&t5725, &t5726)? { true } else { false } }) } } }, Py::str("times must be finite and >= 0"))?;
    }
    for t5728 in py::iter(&(v_events.clone()))? {
        v_e = t5728;
        let _ = f__need({ let t5729 = v_e.clone(); Py::Bool({ let t5730 = Py::tuple(vec![Py::Int(0i64), Py::Int(1i64)]); if py::contains(&t5730, &t5729)? { true } else { false } }) }, Py::str("event indicators are 1 (event) or 0 (censored)"))?;
    }
    return Ok(Py::tuple(vec![{ let mut out = vec![]; for t5731 in py::iter(&(v_times.clone()))? { let mut v_v: Py; 
v_v = t5731;
out.push(py::float(&v_v.clone())?); } Py::list(out) }, { let mut out = vec![]; for t5732 in py::iter(&(v_events.clone()))? { let mut v_e: Py; 
v_e = t5732;
out.push(py::int(&v_e.clone())?); } Py::list(out) }]));
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__table(mut v_a: Py, mut v_b: Py, mut v_c: Py, mut v_d: Py) -> R {
    let mut v_nm: Py = Py::None;
    let mut v_v: Py = Py::None;
    for t5564 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("a"), v_a.clone()]), Py::tuple(vec![Py::str("b"), v_b.clone()]), Py::tuple(vec![Py::str("c"), v_c.clone()]), Py::tuple(vec![Py::str("d"), v_d.clone()])])))? {
        let t5569 = (t5564).items()?;
        if t5569.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t5569[0].clone();
        v_v = t5569[1].clone();
        let _ = f__count(v_nm.clone(), v_v.clone(), Py::Int(1i64))?;
    }
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__trap(mut v_t: Py, mut v_C: Py, mut v_label: Py) -> R {
    let _ = f__need({ let t3910 = { let t3906 = py::len(&v_t.clone())?; Py::Bool({ let t3907 = py::len(&v_C.clone())?; if py::eq(&t3906, &t3907) { true } else { false } }) }; if !t3910.truthy() { t3910 } else { { let t3908 = py::len(&v_t.clone())?; Py::Bool({ let t3909 = Py::Int(2i64); if py::ge(&t3908, &t3909)? { true } else { false } }) } } }, Py::str(&[py::format(&v_label.clone(), "")?, ": give at least two times and as many concentrations".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3911 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3912 = (t3911).items()?;
if t3912.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3912[0].clone();
v_b = t3912[1].clone();
out.push({ let t3913 = v_b.clone(); Py::Bool({ let t3914 = v_a.clone(); if py::gt(&t3913, &t3914)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_label.clone(), "")?, ": the times must increase strictly".to_string()].concat()))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3915 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t3915;
out.push({ let t3916 = v_c.clone(); Py::Bool({ let t3917 = Py::Int(0i64); if py::ge(&t3916, &t3917)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str(&[py::format(&v_label.clone(), "")?, ": concentrations cannot be negative".to_string()].concat()))?;
    return Ok(py::fsum(&{ let mut out = vec![]; for t3918 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_C.clone(), py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_c1: Py; let mut v_c2: Py; let mut v_t1: Py; let mut v_t2: Py; 
let t3919 = (t3918).items()?;
if t3919.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_t1 = t3919[0].clone();
v_t2 = t3919[1].clone();
v_c1 = t3919[2].clone();
v_c2 = t3919[3].clone();
out.push(py::truediv(&py::mul(&py::sub(&v_t2.clone(), &v_t1.clone())?, &py::add(&v_c1.clone(), &v_c2.clone())?)?, &Py::Float(2.0))?); } Py::list(out) })?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f__two_groups(mut v_pos: Py, mut v_neg: Py) -> R {
    let mut v_v: Py = Py::None;
    let _ = f__need({ let t645 = { let t641 = py::len(&v_pos.clone())?; Py::Bool({ let t642 = Py::Int(1i64); if py::ge(&t641, &t642)? { true } else { false } }) }; if !t645.truthy() { t645 } else { { let t643 = py::len(&v_neg.clone())?; Py::Bool({ let t644 = Py::Int(1i64); if py::ge(&t643, &t644)? { true } else { false } }) } } }, Py::str("give at least one score in each group"))?;
    for t646 in py::iter(&(py::add(&Py::list((v_pos.clone()).items()?), &Py::list((v_neg.clone()).items()?))?))? {
        v_v = t646;
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
    let t4459 = (Py::tuple(vec![Py::Float(1e-300), py::add(&py::minmax(&[Py::Float(1.0), v_L.clone()], true)?, &Py::Float(1.0))?])).items()?;
    if t4459.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t4459[0].clone();
    v_hi = t4459[1].clone();
    v_w = (if ({ let t4460 = v_L.clone(); Py::Bool({ let t4461 = Py::Int(1i64); if py::lt(&t4460, &t4461)? { true } else { false } }) }).truthy() { py::math1("exp", &v_L.clone())? } else { py::sub(&v_L.clone(), &py::math1("log", &v_L.clone())?)? });
    v_w = py::minmax(&[py::minmax(&[v_w.clone(), v_lo.clone()], true)?, v_hi.clone()], false)?;
    for t4462 in py::range_iter(&[(Py::Int(200i64)).i()?]) {
        v__ = t4462;
        v_h = py::sub(&py::add(&v_w.clone(), &py::math1("log", &v_w.clone())?)?, &v_L.clone())?;
        if ({ let t4463 = v_h.clone(); Py::Bool({ let t4464 = Py::Int(0i64); if py::gt(&t4463, &t4464)? { true } else { false } }) }).truthy() {
            v_hi = v_w.clone();
        } else {
            v_lo = v_w.clone();
        }
        v_w_new = py::sub(&v_w.clone(), &py::truediv(&v_h.clone(), &py::add(&Py::Float(1.0), &py::truediv(&Py::Float(1.0), &v_w.clone())?)?)?)?;
        if (py::not(&{ let t4465 = v_lo.clone(); Py::Bool({ let t4466 = v_w_new.clone(); if py::lt(&t4465, &t4466)? { { let t4467 = v_hi.clone(); if py::lt(&t4466, &t4467)? { true } else { false } } } else { false } }) })).truthy() {
            v_w_new = (if ({ let t4468 = v_lo.clone(); Py::Bool({ let t4469 = Py::Int(0i64); if py::gt(&t4468, &t4469)? { true } else { false } }) }).truthy() { py::math1("sqrt", &py::mul(&v_lo.clone(), &v_hi.clone())?)? } else { py::mul(&Py::Float(0.5), &v_hi.clone())? });
        }
        if ({ let t4470 = py::abs(&py::sub(&v_w_new.clone(), &v_w.clone())?)?; Py::Bool({ let t4471 = py::mul(&Py::Float(1e-16), &v_w.clone())?; if py::le(&t4470, &t4471)? { true } else { false } }) }).truthy() {
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
    let t650 = (Py::tuple(vec![py::len(&v_scores_pos.clone())?, py::len(&v_scores_neg.clone())?])).items()?;
    if t650.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n1 = t650[0].clone();
    v_n0 = t650[1].clone();
    v_best = Py::None;
    for t651 in py::iter(&(py::sorted(&py::bitor(&py::set(&v_scores_pos.clone())?, &py::set(&v_scores_neg.clone())?)?, (Py::Bool(true)).truthy())?))? {
        v_thr = t651;
        v_tp = py::sum(&{ let mut out = vec![]; for t652 in py::iter(&(v_scores_pos.clone()))? { let mut v_v: Py; 
v_v = t652;
if ({ let t653 = v_v.clone(); Py::Bool({ let t654 = v_thr.clone(); if py::ge(&t653, &t654)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_fp = py::sum(&{ let mut out = vec![]; for t655 in py::iter(&(v_scores_neg.clone()))? { let mut v_v: Py; 
v_v = t655;
if ({ let t656 = v_v.clone(); Py::Bool({ let t657 = v_thr.clone(); if py::ge(&t656, &t657)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_J = py::sub(&py::fraction(&[v_tp.clone(), v_n1.clone()])?, &py::fraction(&[v_fp.clone(), v_n0.clone()])?)?;
        if ({ let t662 = { let t658 = v_best.clone(); Py::Bool({ let t659 = Py::None; if py::is(&t658, &t659) { true } else { false } }) }; if t662.truthy() { t662 } else { { let t660 = v_J.clone(); Py::Bool({ let t661 = py::getitem(&v_best.clone(), &Py::Int(0i64))?; if py::gt(&t660, &t661)? { true } else { false } }) } } }).truthy() {
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
    let _ = f__need({ let t2412 = v_auc.clone(); Py::Bool({ let t2413 = Py::Int(0i64); if py::gt(&t2412, &t2413)? { true } else { false } }) }, Py::str("the indicator curve has no area"))?;
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
    let _ = f__need({ let t732 = py::add(&v_W.clone(), &v_W_arm.clone())?; Py::Bool({ let t733 = Py::Int(0i64); if py::gt(&t732, &t733)? { true } else { false } }) }, Py::str("there is no load"))?;
    let _ = f__need({ let t734 = Py::Int(0i64); Py::Bool({ let t735 = v_theta.clone(); if py::lt(&t734, &t735)? { { let t736 = Py::Float(std::f64::consts::PI); if py::lt(&t735, &t736)? { true } else { false } } } else { false } }) }, Py::str("the muscle angle must lie between 0 and 180 degrees"))?;
    let t737 = (Py::tuple(vec![py::math1("sin", &v_theta.clone())?, py::math1("cos", &v_theta.clone())?])).items()?;
    if t737.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_s = t737[0].clone();
    v_c = t737[1].clone();
    v_F = py::truediv(&py::add(&py::mul(&v_W.clone(), &v_d_load.clone())?, &py::mul(&v_W_arm.clone(), &v_d_arm.clone())?)?, &py::mul(&v_d_muscle.clone(), &v_s.clone())?)?;
    let t738 = (Py::tuple(vec![py::mul(&v_F.clone(), &v_c.clone())?, py::sub(&py::add(&v_W.clone(), &v_W_arm.clone())?, &py::mul(&v_F.clone(), &v_s.clone())?)?])).items()?;
    if t738.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_Rx = t738[0].clone();
    v_Ry = t738[1].clone();
    if ({ let t739 = py::abs(&v_Rx.clone())?; Py::Bool({ let t740 = py::mul(&Py::Float(1e-12), &v_F.clone())?; if py::le(&t739, &t740)? { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t5636 = { let t5629 = py::len(&v_Q.clone())?; Py::Bool({ let t5630 = v_n.clone(); if py::eq(&t5629, &t5630) { true } else { false } }) }; if !t5636.truthy() { t5636 } else { { let t5635 = { let t5631 = py::len(&v_V.clone())?; Py::Bool({ let t5632 = v_n.clone(); if py::eq(&t5631, &t5632) { true } else { false } }) }; if !t5635.truthy() { t5635 } else { { let t5633 = v_n.clone(); Py::Bool({ let t5634 = Py::Int(4i64); if py::ge(&t5633, &t5634)? { true } else { false } }) } } } } }, Py::str("give at least four samples of P, Q and V"))?;
    for t5637 in py::iter(&(py::add(&py::add(&Py::list((v_P.clone()).items()?), &Py::list((v_Q.clone()).items()?))?, &Py::list((v_V.clone()).items()?))?))? {
        v_v = t5637;
        let _ = f__fin(Py::str("every sample"), Py::tuple(vec![v_v.clone()]))?;
    }
    let t5663 = (f__ols(Py::list(vec![Py::list((v_Q.clone()).items()?), Py::list((v_V.clone()).items()?), py::mul(&Py::list(vec![Py::Float(1.0)]), &v_n.clone())?]), Py::list((v_P.clone()).items()?))?).items()?;
    if t5663.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_Rr = t5663[0].clone();
    v_E = t5663[1].clone();
    v_P0 = t5663[2].clone();
    let _ = f__need({ let t5664 = v_E.clone(); Py::Bool({ let t5665 = Py::Int(0i64); if py::gt(&t5664, &t5665)? { true } else { false } }) }, Py::str("the fitted elastance is not positive"))?;
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
    let t5825 = (Py::tuple(vec![py::truediv(&v_sigma.clone(), &v_E_inf.clone())?, py::truediv(&v_sigma.clone(), &py::add(&v_E_inf.clone(), &v_E1.clone())?)?])).items()?;
    if t5825.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_e_inf = t5825[0].clone();
    v_e0 = t5825[1].clone();
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
    let _ = f__need({ let t6310 = v_Pv.clone(); Py::Bool({ let t6311 = v_P0.clone(); if py::lt(&t6310, &t6311)? { true } else { false } }) }, Py::str("the starting pressure must exceed the venous pressure"))?;
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
    let _ = f__need({ let t6371 = v_model.clone(); Py::Bool({ let t6372 = Py::tuple(vec![Py::str("two_element"), Py::str("three_element")]); if py::contains(&t6372, &t6371)? { true } else { false } }) }, Py::str("model is two_element or three_element"))?;
    if ({ let t6373 = v_model.clone(); Py::Bool({ let t6374 = Py::str("three_element"); if py::eq(&t6373, &t6374) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6375 = v_Zc.clone(); Py::Bool({ let t6376 = Py::None; if !py::is(&t6375, &t6376) { true } else { false } }) }, Py::str("the three-element model needs the characteristic impedance Zc"))?;
        let _ = f__pos(Py::str("Zc"), Py::tuple(vec![v_Zc.clone()]))?;
    } else {
        let _ = f__need({ let t6377 = v_Zc.clone(); Py::Bool({ let t6378 = Py::None; if py::is(&t6377, &t6378) { true } else { false } }) }, Py::str("Zc belongs to the three-element model, not the two-element model"))?;
    }
    v_w = py::mul(&py::mul(&Py::Float(2.0), &Py::Float(std::f64::consts::PI))?, &v_f.clone())?;
    v_den = py::add(&Py::Float(1.0), &py::pow(&py::mul(&py::mul(&v_w.clone(), &v_R.clone())?, &v_C.clone())?, &Py::Int(2i64))?)?;
    v_zr = py::add(&py::truediv(&v_R.clone(), &v_den.clone())?, &{ let t6379 = v_Zc.clone(); if t6379.truthy() { t6379 } else { Py::Float(0.0) } })?;
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
    let _ = f__need({ let t3047 = { let t3042 = py::len(&v_tables.clone())?; Py::Bool({ let t3043 = Py::Int(1i64); if py::ge(&t3042, &t3043)? { true } else { false } }) }; if !t3047.truthy() { t3047 } else { Py::Bool(({ let mut out = vec![]; for t3044 in py::iter(&(v_tables.clone()))? { let mut v_r: Py; 
v_r = t3044;
out.push({ let t3045 = py::len(&v_r.clone())?; Py::Bool({ let t3046 = Py::Int(4i64); if py::eq(&t3045, &t3046) { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())) } }, Py::str("give one row [a, b, c, d] per stratum"))?;
    for t3048 in py::iter(&(v_tables.clone()))? {
        v_r = t3048;
        for t3049 in py::iter(&(v_r.clone()))? {
            v_v = t3049;
            let _ = f__need({ let t3054 = { let t3050 = v_v.clone(); Py::Bool({ let t3051 = Py::Int(0i64); if py::ge(&t3050, &t3051)? { true } else { false } }) }; if !t3054.truthy() { t3054 } else { { let t3052 = py::float(&v_v.clone())?; Py::Bool({ let t3053 = py::int(&v_v.clone())?; if py::eq(&t3052, &t3053) { true } else { false } }) } } }, Py::str("cell counts are whole numbers >= 0"))?;
        }
        let _ = f__need({ let t3055 = py::sum(&v_r.clone(), None)?; Py::Bool({ let t3056 = Py::Int(0i64); if py::gt(&t3055, &t3056)? { true } else { false } }) }, Py::str("a stratum is empty"))?;
    }
    v_z = f__level(v_level.clone())?;
    let t3063 = Py::Float(0.0);
    v_R = t3063.clone();
    v_S = t3063.clone();
    v_sPR = t3063.clone();
    v_sPSQR = t3063.clone();
    v_sQS = t3063.clone();
    v_parts = Py::list(vec![]);
    for t3064 in py::iter(&(v_tables.clone()))? {
        let t3067 = (t3064).items()?;
        if t3067.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_a = t3067[0].clone();
        v_b = t3067[1].clone();
        v_c = t3067[2].clone();
        v_d = t3067[3].clone();
        v_n = py::add(&py::add(&py::add(&v_a.clone(), &v_b.clone())?, &v_c.clone())?, &v_d.clone())?;
        let t3065 = (Py::tuple(vec![py::truediv(&py::mul(&v_a.clone(), &v_d.clone())?, &v_n.clone())?, py::truediv(&py::mul(&v_b.clone(), &v_c.clone())?, &v_n.clone())?])).items()?;
        if t3065.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Ri = t3065[0].clone();
        v_Si = t3065[1].clone();
        let t3066 = (Py::tuple(vec![py::truediv(&py::add(&v_a.clone(), &v_d.clone())?, &v_n.clone())?, py::truediv(&py::add(&v_b.clone(), &v_c.clone())?, &v_n.clone())?])).items()?;
        if t3066.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_Pi = t3066[0].clone();
        v_Qi = t3066[1].clone();
        let _ = py::method(&v_parts.clone(), "append", &[Py::tuple(vec![v_Ri.clone(), v_Si.clone(), v_Pi.clone(), v_Qi.clone()])])?;
    }
    v_R = py::fsum(&{ let mut out = vec![]; for t3068 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3068;
out.push(py::getitem(&v_p.clone(), &Py::Int(0i64))?); } Py::list(out) })?;
    v_S = py::fsum(&{ let mut out = vec![]; for t3069 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3069;
out.push(py::getitem(&v_p.clone(), &Py::Int(1i64))?); } Py::list(out) })?;
    let _ = f__need({ let t3074 = { let t3070 = v_R.clone(); Py::Bool({ let t3071 = Py::Int(0i64); if py::gt(&t3070, &t3071)? { true } else { false } }) }; if !t3074.truthy() { t3074 } else { { let t3072 = v_S.clone(); Py::Bool({ let t3073 = Py::Int(0i64); if py::gt(&t3072, &t3073)? { true } else { false } }) } } }, Py::str("the pooled odds ratio is zero or infinite"))?;
    v_var = py::add(&py::add(&py::truediv(&py::fsum(&{ let mut out = vec![]; for t3075 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3075;
out.push(py::mul(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?); } Py::list(out) })?, &py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_R.clone())?)?, &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3076 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3076;
out.push(py::add(&py::mul(&py::getitem(&v_p.clone(), &Py::Int(2i64))?, &py::getitem(&v_p.clone(), &Py::Int(1i64))?)?, &py::mul(&py::getitem(&v_p.clone(), &Py::Int(3i64))?, &py::getitem(&v_p.clone(), &Py::Int(0i64))?)?)?); } Py::list(out) })?, &py::mul(&py::mul(&Py::Int(2i64), &v_R.clone())?, &v_S.clone())?)?)?, &py::truediv(&py::fsum(&{ let mut out = vec![]; for t3077 in py::iter(&(v_parts.clone()))? { let mut v_p: Py; 
v_p = t3077;
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
    let _ = f__need({ let t3629 = { let t3625 = v_events_t.clone(); Py::Bool({ let t3626 = v_n_t.clone(); if py::le(&t3625, &t3626)? { true } else { false } }) }; if !t3629.truthy() { t3629 } else { { let t3627 = v_events_c.clone(); Py::Bool({ let t3628 = v_n_c.clone(); if py::le(&t3627, &t3628)? { true } else { false } }) } } }, Py::str("events cannot exceed the group size"))?;
    v_z = f__level(v_level.clone())?;
    let t3636 = (Py::tuple(vec![py::truediv(&v_events_t.clone(), &v_n_t.clone())?, py::truediv(&v_events_c.clone(), &v_n_c.clone())?])).items()?;
    if t3636.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pt = t3636[0].clone();
    v_pc = t3636[1].clone();
    v_arr = py::sub(&v_pc.clone(), &v_pt.clone())?;
    v_se = py::math1("sqrt", &py::add(&py::truediv(&py::mul(&v_pt.clone(), &py::sub(&Py::Int(1i64), &v_pt.clone())?)?, &v_n_t.clone())?, &py::truediv(&py::mul(&v_pc.clone(), &py::sub(&Py::Int(1i64), &v_pc.clone())?)?, &v_n_c.clone())?)?)?;
    let t3637 = (Py::tuple(vec![py::sub(&v_arr.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?, py::add(&v_arr.clone(), &py::mul(&v_z.clone(), &v_se.clone())?)?])).items()?;
    if t3637.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_lo = t3637[0].clone();
    v_hi = t3637[1].clone();
    let _ = f__need({ let t3638 = v_lo.clone(); Py::Bool({ let t3639 = Py::Int(0i64); if py::gt(&t3638, &t3639)? { true } else { false } }) }, Py::str("the interval for the risk reduction does not exclude zero (or shows harm): the NNT interval is not finite"))?;
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
    v_u = py::fsum(&{ let mut out = vec![]; for t441 in py::iter(&(v_scores_pos.clone()))? { let mut v_a: Py; 
v_a = t441;
for t442 in py::iter(&(v_scores_neg.clone()))? { let mut v_b: Py; 
v_b = t442;
out.push((if ({ let t443 = v_a.clone(); Py::Bool({ let t444 = v_b.clone(); if py::gt(&t443, &t444)? { true } else { false } }) }).truthy() { Py::Float(1.0) } else { (if ({ let t445 = v_a.clone(); Py::Bool({ let t446 = v_b.clone(); if py::eq(&t445, &t446) { true } else { false } }) }).truthy() { Py::Float(0.5) } else { Py::Float(0.0) }) })); }} Py::list(out) })?;
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
    for t500 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("TP"), v_TP.clone()]), Py::tuple(vec![Py::str("FN"), v_FN.clone()]), Py::tuple(vec![Py::str("TN"), v_TN.clone()]), Py::tuple(vec![Py::str("FP"), v_FP.clone()])])))? {
        let t505 = (t500).items()?;
        if t505.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t505[0].clone();
        v_v = t505[1].clone();
        let _ = f__count(v_nm.clone(), v_v.clone(), Py::Int(0i64))?;
    }
    let _ = f__need({ let t510 = { let t506 = py::add(&v_TP.clone(), &v_FN.clone())?; Py::Bool({ let t507 = Py::Int(1i64); if py::ge(&t506, &t507)? { true } else { false } }) }; if !t510.truthy() { t510 } else { { let t508 = py::add(&v_TN.clone(), &v_FP.clone())?; Py::Bool({ let t509 = Py::Int(1i64); if py::ge(&t508, &t509)? { true } else { false } }) } } }, Py::str("each group (diseased, non-diseased) needs at least one subject"))?;
    v_z = f__level(v_level.clone())?;
    let t517 = (f__wilson(v_TP.clone(), py::add(&v_TP.clone(), &v_FN.clone())?, v_z.clone())?).items()?;
    if t517.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_sl = t517[0].clone();
    v_su = t517[1].clone();
    let t518 = (f__wilson(v_TN.clone(), py::add(&v_TN.clone(), &v_FP.clone())?, v_z.clone())?).items()?;
    if t518.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_pl = t518[0].clone();
    v_pu = t518[1].clone();
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
    let _ = f__need({ let t5948 = v_observed.clone(); Py::Bool({ let t5949 = Py::Int(1000i64); if py::le(&t5948, &t5949)? { true } else { false } }) }, Py::str("observed counts above 1000 are not supported"))?;
    let _ = f__pos(Py::str("the expected count"), Py::tuple(vec![v_expected.clone()]))?;
    let _ = f__level(v_level.clone())?;
    v_a = py::truediv(&py::sub(&Py::Int(1i64), &v_level.clone())?, &Py::Int(2i64))?;
    v_hi_b = py::add(&py::add(&v_observed.clone(), &py::mul(&Py::Float(20.0), &py::math1("sqrt", &py::add(&v_observed.clone(), &Py::Float(1.0))?)?)?)?, &Py::Float(50.0))?;
    if ({ let t5962 = v_observed.clone(); Py::Bool({ let t5963 = Py::Int(0i64); if py::eq(&t5962, &t5963) { true } else { false } }) }).truthy() {
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
    v_tp = py::sum(&{ let mut out = vec![]; for t581 in py::iter(&(v_scores_pos.clone()))? { let mut v_v: Py; 
v_v = t581;
if ({ let t582 = v_v.clone(); Py::Bool({ let t583 = v_threshold.clone(); if py::ge(&t582, &t583)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    v_fp = py::sum(&{ let mut out = vec![]; for t584 in py::iter(&(v_scores_neg.clone()))? { let mut v_v: Py; 
v_v = t584;
if ({ let t585 = v_v.clone(); Py::Bool({ let t586 = v_threshold.clone(); if py::ge(&t585, &t586)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
    let t587 = (Py::tuple(vec![py::sub(&py::len(&v_scores_pos.clone())?, &v_tp.clone())?, py::sub(&py::len(&v_scores_neg.clone())?, &v_fp.clone())?])).items()?;
    if t587.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_fn = t587[0].clone();
    v_tn = t587[1].clone();
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
    let t663 = (f__youden(v_scores_pos.clone(), v_scores_neg.clone())?).items()?;
    if t663.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t664 = (t663[0].clone()).items()?;
    if t664.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_J = t664[0].clone();
    v_thr = t664[1].clone();
    v_tp = t664[2].clone();
    v_fp = t664[3].clone();
    v_n1 = t663[1].clone();
    v_n0 = t663[2].clone();
    let _ = f__need({ let t665 = v_J.clone(); Py::Bool({ let t666 = Py::Int(0i64); if py::gt(&t665, &t666)? { true } else { false } }) }, Py::str("the scores do not separate the groups (no cut-off with positive Youden index)"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("threshold"), py::float(&v_thr.clone())?), (Py::str("sensitivity"), py::truediv(&v_tp.clone(), &v_n1.clone())?), (Py::str("specificity"), py::sub(&Py::Int(1i64), &py::truediv(&v_fp.clone(), &v_n0.clone())?)?), (Py::str("J"), py::float(&v_J.clone())?)]))?);
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
    let _ = f__need({ let t803 = py::add(&Py::Float(1.0), &py::mul(&v_r.clone(), &v_th.clone())?)?; Py::Bool({ let t804 = Py::Int(0i64); if py::gt(&t803, &t804)? { true } else { false } }) }, Py::str("the decline is too fast for this generation interval (1 + r sd^2/mean must be positive)"))?;
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
    let _ = f__need({ let t861 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t861.truthy() { t861 } else { { let t858 = Py::Int(1i64); Py::Bool({ let t859 = v_n.clone(); if py::le(&t858, &t859)? { { let t860 = Py::Int(30i64); if py::le(&t859, &t860)? { true } else { false } } } else { false } }) } } }, Py::str("n (initial susceptibles) is a whole number from 1 to 30"))?;
    let _ = f__need({ let t865 = Py::Bool(matches!(v_m.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t865.truthy() { t865 } else { { let t862 = Py::Int(1i64); Py::Bool({ let t863 = v_m.clone(); if py::le(&t862, &t863)? { { let t864 = Py::Int(30i64); if py::le(&t863, &t864)? { true } else { false } } } else { false } }) } } }, Py::str("m (initial infectives) is a whole number from 1 to 30"))?;
    let _ = f__frac(Py::str("p"), v_p.clone(), Py::Bool(true), Py::Bool(true))?;
    v_q = py::sub(&Py::Float(1.0), &v_p.clone())?;
    v_states = Py::dict(vec![(Py::tuple(vec![v_n.clone(), v_m.clone()]), Py::Float(1.0))]);
    v_final = py::mul(&Py::list(vec![Py::Float(0.0)]), &py::add(&v_n.clone(), &Py::Int(1i64))?)?;
    while (v_states.clone()).truthy() {
        v_nxt = Py::dict(vec![]);
        for t878 in py::iter(&(py::method(&v_states.clone(), "items", &[])?))? {
            let t887 = (t878).items()?;
            if t887.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            let t888 = (t887[0].clone()).items()?;
            if t888.len() != 2 { return Err(PyErr::Value("unpack".into())); }
            v_s = t888[0].clone();
            v_i = t888[1].clone();
            v_pr = t887[1].clone();
            if ({ let t883 = { let t879 = v_i.clone(); Py::Bool({ let t880 = Py::Int(0i64); if py::eq(&t879, &t880) { true } else { false } }) }; if t883.truthy() { t883 } else { { let t881 = v_s.clone(); Py::Bool({ let t882 = Py::Int(0i64); if py::eq(&t881, &t882) { true } else { false } }) } } }).truthy() {
                let t884 = v_final.clone(); let t885 = py::sub(&v_n.clone(), &v_s.clone())?;
                py::setitem(&t884, &t885, py::iadd(&py::getitem(&t884, &t885)?, &v_pr.clone())?)?;
                continue;
            }
            v_qi = py::pow(&v_q.clone(), &v_i.clone())?;
            v_pi = py::sub(&Py::Float(1.0), &v_qi.clone())?;
            for t886 in py::range_iter(&[(py::add(&v_s.clone(), &Py::Int(1i64))?).i()?]) {
                v_j = t886;
                v_w = py::mul(&py::mul(&py::mul(&v_pr.clone(), &py::comb(&v_s.clone(), &v_j.clone())?)?, &py::pow(&v_pi.clone(), &v_j.clone())?)?, &py::pow(&v_qi.clone(), &py::sub(&v_s.clone(), &v_j.clone())?)?)?;
                if (v_w.clone()).truthy() {
                    py::setitem(&v_nxt.clone(), &Py::tuple(vec![py::sub(&v_s.clone(), &v_j.clone())?, v_j.clone()]), py::add(&py::method(&v_nxt.clone(), "get", &[Py::tuple(vec![py::sub(&v_s.clone(), &v_j.clone())?, v_j.clone()]), Py::Float(0.0)])?, &v_w.clone())?)?;
                }
            }
        }
        v_states = v_nxt.clone();
    }
    return Ok(f__out(Py::dict(vec![(Py::str("mean_final"), py::fsum(&{ let mut out = vec![]; for t901 in py::iter(&(py::enumerate(&v_final.clone(), 0)?))? { let mut v_k: Py; let mut v_v: Py; 
let t902 = (t901).items()?;
if t902.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_k = t902[0].clone();
v_v = t902[1].clone();
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
    let _ = f__need({ let t962 = py::add(&v_e0.clone(), &v_i0.clone())?; Py::Bool({ let t963 = Py::Int(0i64); if py::gt(&t962, &t963)? { true } else { false } }) }, Py::str("some exposed or infected must be present at the start"))?;
    let _ = f__need({ let t964 = py::add(&py::add(&v_s0.clone(), &v_e0.clone())?, &v_i0.clone())?; Py::Bool({ let t965 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t964, &t965)? { true } else { false } }) }, Py::str("s0 + e0 + i0 cannot exceed 1"))?;
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
    v_r = (if ({ let t1051 = v_d.clone(); Py::Bool({ let t1052 = Py::Int(0i64); if py::ge(&t1051, &t1052)? { true } else { false } }) }).truthy() { py::truediv(&v_d.clone(), &py::mul(&Py::Float(2.0), &py::add(&v_a.clone(), &v_root.clone())?)?)? } else { py::truediv(&py::sub(&v_root.clone(), &v_a.clone())?, &Py::Float(2.0))? });
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
    for t1124 in py::iter(&(Py::tuple(vec![Py::tuple(vec![Py::str("e0"), v_e0.clone()]), Py::tuple(vec![Py::str("i0"), v_i0.clone()])])))? {
        let t1125 = (t1124).items()?;
        if t1125.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_nm = t1125[0].clone();
        v_v = t1125[1].clone();
        let _ = f__frac(v_nm.clone(), v_v.clone(), Py::Bool(false), Py::Bool(false))?;
    }
    let _ = f__need({ let t1126 = py::add(&v_e0.clone(), &v_i0.clone())?; Py::Bool({ let t1127 = Py::Int(0i64); if py::gt(&t1126, &t1127)? { true } else { false } }) }, Py::str("an epidemic needs e0 + i0 > 0"))?;
    let _ = f__need({ let t1128 = py::add(&py::add(&v_s0.clone(), &v_e0.clone())?, &v_i0.clone())?; Py::Bool({ let t1129 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1128, &t1129)? { true } else { false } }) }, Py::str("s0 + e0 + i0 cannot exceed 1"))?;
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    let _ = f__need({ let t1130 = py::mul(&v_R0.clone(), &v_s0.clone())?; Py::Bool({ let t1131 = py::add(&Py::Float(1.0), &Py::Float(1e-06))?; if py::gt(&t1130, &t1131)? { true } else { false } }) }, Py::str("no epidemic peak: R0 s0 must exceed 1"))?;
    let t1165 = (f__seir_taylor_peak(v_R0.clone(), py::truediv(&v_sigma.clone(), &v_gamma.clone())?, v_s0.clone(), v_e0.clone(), v_i0.clone(), Py::Int(26i64))?).items()?;
    if t1165.len() != 4 { return Err(PyErr::Value("unpack".into())); }
    v_tau = t1165[0].clone();
    v_s = t1165[1].clone();
    v_e = t1165[2].clone();
    v_i = t1165[3].clone();
    let _ = f__need({ let t1166 = v_i.clone(); Py::Bool({ let t1167 = py::mul(&v_i0.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t1166, &t1167)? { true } else { false } }) }, Py::str("the infectious fraction never rises above its initial value"))?;
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
    let _ = f__need({ let t1239 = py::add(&v_s0.clone(), &v_i0.clone())?; Py::Bool({ let t1240 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1239, &t1240)? { true } else { false } }) }, Py::str("s0 + i0 cannot exceed 1"))?;
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
    let _ = f__need({ let t1338 = py::add(&v_s0.clone(), &v_i0.clone())?; Py::Bool({ let t1339 = py::add(&Py::Int(1i64), &Py::Float(1e-12))?; if py::le(&t1338, &t1339)? { true } else { false } }) }, Py::str("s0 + i0 cannot exceed 1"))?;
    v_R0 = py::truediv(&v_beta.clone(), &v_gamma.clone())?;
    let _ = f__need({ let t1340 = py::mul(&v_R0.clone(), &v_s0.clone())?; Py::Bool({ let t1341 = py::add(&Py::Float(1.0), &Py::Float(1e-06))?; if py::gt(&t1340, &t1341)? { true } else { false } }) }, Py::str("no epidemic peak: R0 s0 must exceed 1 (prevalence only falls)"))?;
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
    if ({ let t1423 = v_r.clone(); Py::Bool({ let t1424 = Py::Int(0i64); if py::eq(&t1423, &t1424) { true } else { false } }) }).truthy() {
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
    let _ = f__need({ let t2269 = { let t2265 = v_mu_air.clone(); Py::Bool({ let t2266 = Py::Int(0i64); if py::ge(&t2265, &t2266)? { true } else { false } }) }; if !t2269.truthy() { t2269 } else { { let t2267 = v_mu_air.clone(); Py::Bool({ let t2268 = v_mu_water.clone(); if py::lt(&t2267, &t2268)? { true } else { false } }) } } }, Py::str("the air coefficient must lie in [0, mu_water)"))?;
    v_HU = py::truediv(&py::mul(&Py::Float(1000.0), &py::sub(&v_mu.clone(), &v_mu_water.clone())?)?, &py::sub(&v_mu_water.clone(), &v_mu_air.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("HU"), v_HU.clone()), (Py::str("mu_rel"), py::truediv(&v_mu.clone(), &v_mu_water.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_adc(mut v_b: Py, mut v_S: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let t3153 = (f__loglin_fit_out(v_b.clone(), v_S.clone(), Py::str("b"))?).items()?;
    if t3153.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t3153[0].clone();
    v_c = t3153[1].clone();
    return Ok(f__out(Py::dict(vec![(Py::str("ADC"), py::neg(&v_slope.clone())?), (Py::str("S0"), py::math1("exp", &v_c.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_inversion_recovery(mut v_T1: Py, mut v_TR: Py, mut v_TI: Py) -> R {
    let mut v_e: Py = Py::None;
    let _ = f__pos(Py::str("T1, TR and TI"), Py::tuple(vec![v_T1.clone(), v_TR.clone(), v_TI.clone()]))?;
    let _ = f__need({ let t3272 = v_TI.clone(); Py::Bool({ let t3273 = v_TR.clone(); if py::lt(&t3272, &t3273)? { true } else { false } }) }, Py::str("TI must be shorter than TR"))?;
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
    let _ = f__need({ let t3333 = v_TE.clone(); Py::Bool({ let t3334 = v_TR.clone(); if py::lt(&t3333, &t3334)? { true } else { false } }) }, Py::str("TE must be shorter than TR"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), py::mul(&py::mul(&v_M0.clone(), &py::neg(&py::math1("expm1", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_TE.clone())?, &v_T2.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_spoiled_gre(mut v_M0: Py, mut v_TR: Py, mut v_TE: Py, mut v_T1: Py, mut v_T2star: Py, mut v_alpha: Py) -> R {
    let mut v_E1: Py = Py::None;
    let _ = f__pos(Py::str("M0, TR, TE, T1 and T2star"), Py::tuple(vec![v_M0.clone(), v_TR.clone(), v_TE.clone(), v_T1.clone(), v_T2star.clone()]))?;
    let _ = f__need({ let t3394 = v_TE.clone(); Py::Bool({ let t3395 = v_TR.clone(); if py::lt(&t3394, &t3395)? { true } else { false } }) }, Py::str("TE must be shorter than TR"))?;
    let _ = f__need({ let t3396 = Py::Int(0i64); Py::Bool({ let t3397 = v_alpha.clone(); if py::lt(&t3396, &t3397)? { { let t3398 = py::truediv(&Py::Float(std::f64::consts::PI), &Py::Int(2i64))?; if py::le(&t3397, &t3398)? { true } else { false } } } else { false } }) }, Py::str("the flip angle must lie between 0 and 90 degrees"))?;
    v_E1 = py::math1("exp", &py::truediv(&py::neg(&v_TR.clone())?, &v_T1.clone())?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("S"), py::mul(&py::truediv(&py::mul(&py::mul(&v_M0.clone(), &py::math1("sin", &v_alpha.clone())?)?, &py::sub(&Py::Int(1i64), &v_E1.clone())?)?, &py::sub(&Py::Int(1i64), &py::mul(&py::math1("cos", &v_alpha.clone())?, &v_E1.clone())?)?)?, &py::math1("exp", &py::truediv(&py::neg(&v_TE.clone())?, &v_T2star.clone())?)?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_im_t2_fit(mut v_TE: Py, mut v_S: Py) -> R {
    let mut v_c: Py = Py::None;
    let mut v_slope: Py = Py::None;
    let t3474 = (f__loglin_fit_out(v_TE.clone(), v_S.clone(), Py::str("echo time"))?).items()?;
    if t3474.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t3474[0].clone();
    v_c = t3474[1].clone();
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
    let t6185 = (Py::tuple(vec![py::math1("cos", &v_kd.clone())?, py::math1("sin", &v_kd.clone())?])).items()?;
    if t6185.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_c = t6185[0].clone();
    v_s = t6185[1].clone();
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
    let _ = f__need({ let t6446 = { let t6439 = v_n.clone(); Py::Bool({ let t6440 = Py::Int(1i64); if py::ge(&t6439, &t6440)? { true } else { false } }) }; if !t6446.truthy() { t6446 } else { { let t6445 = { let t6441 = py::len(&v_rho.clone())?; Py::Bool({ let t6442 = v_n.clone(); if py::eq(&t6441, &t6442) { true } else { false } }) }; if !t6445.truthy() { t6445 } else { { let t6443 = py::len(&v_x.clone())?; Py::Bool({ let t6444 = v_n.clone(); if py::eq(&t6443, &t6444) { true } else { false } }) } } } } }, Py::str("give one mass attenuation coefficient, density and thickness per layer"))?;
    for t6447 in py::iter(&(py::add(&Py::list((v_mu_rho.clone()).items()?), &Py::list((v_rho.clone()).items()?))?))? {
        v_v = t6447;
        let _ = f__need({ let t6448 = v_v.clone(); Py::Bool({ let t6449 = Py::Int(0i64); if py::gt(&t6448, &t6449)? { true } else { false } }) }, Py::str("attenuation coefficients and densities must be positive"))?;
    }
    for t6450 in py::iter(&(v_x.clone()))? {
        v_v = t6450;
        let _ = f__need({ let t6451 = v_v.clone(); Py::Bool({ let t6452 = Py::Int(0i64); if py::ge(&t6451, &t6452)? { true } else { false } }) }, Py::str("thicknesses cannot be negative"))?;
    }
    v_T = py::math1("exp", &py::neg(&py::fsum(&{ let mut out = vec![]; for t6453 in py::iter(&(py::zip(&[v_mu_rho.clone(), v_rho.clone(), v_x.clone()])?))? { let mut v_a: Py; let mut v_b: Py; let mut v_c: Py; 
let t6454 = (t6453).items()?;
if t6454.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_a = t6454[0].clone();
v_b = t6454[1].clone();
v_c = t6454[2].clone();
out.push(py::mul(&py::mul(&v_a.clone(), &v_b.clone())?, &v_c.clone())?); } Py::list(out) })?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("T"), v_T.clone()), (Py::str("I"), py::mul(&v_I0.clone(), &v_T.clone())?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_larmor(mut v_B: Py, mut v_gamma_bar: Py) -> R {
    let mut v_f: Py = Py::None;
    let mut v_g: Py = Py::None;
    let _ = f__pos(Py::str("the magnetic field"), Py::tuple(vec![v_B.clone()]))?;
    v_g = (if ({ let t2741 = v_gamma_bar.clone(); Py::Bool({ let t2742 = Py::None; if py::is(&t2741, &t2742) { true } else { false } }) }).truthy() { c_GAMMA_P_MHZ_T()? } else { v_gamma_bar.clone() });
    let _ = f__need({ let t2748 = f__isnum(v_g.clone())?; if !t2748.truthy() { t2748 } else { { let t2745 = Py::Int(0i64); Py::Bool({ let t2746 = py::abs(&v_g.clone())?; if py::lt(&t2745, &t2746)? { { let t2747 = Py::Int(100i64); if py::le(&t2746, &t2747)? { true } else { false } } } else { false } }) } } }, Py::str("the reduced gyromagnetic ratio must be given in MHz/T (|gamma/2 pi| <= 100)"))?;
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
    let _ = f__need({ let t1635 = Py::Int(0i64); Py::Bool({ let t1636 = v_b.clone(); if py::lt(&t1635, &t1636)? { { let t1637 = Py::Int(1i64); if py::lt(&t1636, &t1637)? { true } else { false } } } else { false } }) }, Py::str("this calculation needs 0 < b < 1 (then the fixed point is unique)"))?;
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
    while ({ let t1648 = { let t1644 = py::call(&v_h.clone(), &[py::neg(&v_B.clone())?])?; Py::Bool({ let t1645 = Py::Int(0i64); if py::le(&t1644, &t1645)? { true } else { false } }) }; if t1648.truthy() { t1648 } else { { let t1646 = py::call(&v_h.clone(), &[v_B.clone()])?; Py::Bool({ let t1647 = Py::Int(0i64); if py::ge(&t1646, &t1647)? { true } else { false } }) } } }).truthy() {
        v_B = py::mul(&v_B, &Py::Float(2.0))?;
    }
    v_v = f__bisect(v_h.clone(), py::neg(&v_B.clone())?, v_B.clone(), Py::Int(400i64))?;
    v_w = py::truediv(&py::add(&v_v.clone(), &v_a.clone())?, &v_b.clone())?;
    v_tr = py::sub(&py::sub(&Py::Float(1.0), &py::mul(&v_v.clone(), &v_v.clone())?)?, &py::mul(&v_eps.clone(), &v_b.clone())?)?;
    v_det = py::mul(&v_eps.clone(), &py::add(&py::sub(&Py::Float(1.0), &v_b.clone())?, &py::mul(&py::mul(&v_b.clone(), &v_v.clone())?, &v_v.clone())?)?)?;
    return Ok(f__out(Py::dict(vec![(Py::str("v_star"), v_v.clone()), (Py::str("w_star"), v_w.clone()), (Py::str("trace"), v_tr.clone()), (Py::str("determinant"), v_det.clone()), (Py::str("stable"), Py::Bool(({ let t1687 = v_tr.clone(); Py::Bool({ let t1688 = Py::Int(0i64); if py::lt(&t1687, &t1688)? { true } else { false } }) }).truthy()))]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_ne_ghk_current(mut v_P: Py, mut v_z: Py, mut v_C_in: Py, mut v_C_out: Py, mut v_V: Py, mut v_T: Py) -> R {
    let mut v_u: Py = Py::None;
    let _ = f__pos(Py::str("the permeability P and the temperature T"), Py::tuple(vec![v_P.clone(), v_T.clone()]))?;
    let _ = f__need({ let t1806 = Py::Bool(matches!(v_z.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t1806.truthy() { t1806 } else { { let t1805 = { let t1801 = v_z.clone(); Py::Bool({ let t1802 = Py::Int(0i64); if py::ne(&t1801, &t1802) { true } else { false } }) }; if !t1805.truthy() { t1805 } else { { let t1803 = py::abs(&v_z.clone())?; Py::Bool({ let t1804 = Py::Int(3i64); if py::le(&t1803, &t1804)? { true } else { false } }) } } } } }, Py::str("z is a nonzero whole number from -3 to 3"))?;
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
    let t1883 = (f__ghk_terms(v_PK.clone(), v_PNa.clone(), v_PCl.clone(), v_K_o.clone(), v_K_i.clone(), v_Na_o.clone(), v_Na_i.clone(), v_Cl_o.clone(), v_Cl_i.clone())?).items()?;
    if t1883.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_num = t1883[0].clone();
    v_den = t1883[1].clone();
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
    let _ = f__need({ let t1940 = py::neg(&Py::Float(150.0))?; Py::Bool({ let t1941 = v_v.clone(); if py::le(&t1940, &t1941)? { { let t1942 = Py::Float(100.0); if py::le(&t1941, &t1942)? { true } else { false } } } else { false } }) }, Py::str("V must lie between -150 mV and +100 mV"))?;
    let t1948 = (f__hh_rates(v_v.clone())?).items()?;
    if t1948.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t1949 = (t1948[0].clone()).items()?;
    if t1949.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_am = t1949[0].clone();
    v_bm = t1949[1].clone();
    let t1950 = (t1948[1].clone()).items()?;
    if t1950.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_ah = t1950[0].clone();
    v_bh = t1950[1].clone();
    let t1951 = (t1948[2].clone()).items()?;
    if t1951.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_an = t1951[0].clone();
    v_bn = t1951[1].clone();
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
    for t2020 in py::iter(&(Py::tuple(vec![v_V_hold.clone(), v_V_step.clone()])))? {
        v_vv = t2020;
        let _ = f__need({ let t2021 = py::neg(&Py::Float(0.15))?; Py::Bool({ let t2022 = v_vv.clone(); if py::le(&t2021, &t2022)? { { let t2023 = Py::Float(0.1); if py::le(&t2022, &t2023)? { true } else { false } } } else { false } }) }, Py::str("potentials must lie between -150 mV and +100 mV"))?;
    }
    let t2029 = (f__hh_rates(py::mul(&v_V_hold.clone(), &Py::Float(1000.0))?)?).items()?;
    if t2029.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2030 = (t2029[0].clone()).items()?;
    if t2030.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2030[0].clone();
    v__ = t2030[1].clone();
    let t2031 = (t2029[1].clone()).items()?;
    if t2031.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2031[0].clone();
    v__ = t2031[1].clone();
    let t2032 = (t2029[2].clone()).items()?;
    if t2032.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a0 = t2032[0].clone();
    v_b0 = t2032[1].clone();
    let t2033 = (f__hh_rates(py::mul(&v_V_step.clone(), &Py::Float(1000.0))?)?).items()?;
    if t2033.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    let t2034 = (t2033[0].clone()).items()?;
    if t2034.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2034[0].clone();
    v__ = t2034[1].clone();
    let t2035 = (t2033[1].clone()).items()?;
    if t2035.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v__ = t2035[0].clone();
    v__ = t2035[1].clone();
    let t2036 = (t2033[2].clone()).items()?;
    if t2036.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t2036[0].clone();
    v_b1 = t2036[1].clone();
    let t2037 = (Py::tuple(vec![py::truediv(&v_a0.clone(), &py::add(&v_a0.clone(), &v_b0.clone())?)?, py::truediv(&v_a1.clone(), &py::add(&v_a1.clone(), &v_b1.clone())?)?])).items()?;
    if t2037.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_n0 = t2037[0].clone();
    v_n1 = t2037[1].clone();
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
    let _ = f__need({ let t2100 = v_EK.clone(); Py::Bool({ let t2101 = v_EL.clone(); if py::lt(&t2100, &t2101)? { { let t2102 = v_ENa.clone(); if py::lt(&t2101, &t2102)? { true } else { false } } } else { false } }) }, Py::str("the reversal potentials must satisfy EK < EL < ENa"))?;
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
    let _ = f__need({ let t2817 = v_V_reset.clone(); Py::Bool({ let t2818 = v_V_th.clone(); if py::lt(&t2817, &t2818)? { true } else { false } }) }, Py::str("the reset potential must lie below the threshold"))?;
    v_Vinf = py::add(&v_V_rest.clone(), &py::mul(&v_R_m.clone(), &v_I.clone())?)?;
    let _ = f__need({ let t2819 = v_Vinf.clone(); Py::Bool({ let t2820 = v_V_th.clone(); if py::gt(&t2819, &t2820)? { true } else { false } }) }, Py::str("the input does not bring the membrane to threshold: the neuron does not fire"))?;
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
    let _ = f__need({ let t5501 = v_I0.clone(); Py::Bool({ let t5502 = Py::Int(0i64); if py::ne(&t5501, &t5502) { true } else { false } }) }, Py::str("the synaptic current amplitude I0 must not be zero"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t5509 = py::abs(&py::sub(&v_tau_s.clone(), &v_tau_m.clone())?)?; Py::Bool({ let t5510 = py::mul(&Py::Float(1e-06), &py::minmax(&[v_tau_s.clone(), v_tau_m.clone()], true)?)?; if py::gt(&t5509, &t5510)? { true } else { false } }) }, Py::str("tau_s equals tau_m: the difference-of-exponentials form does not apply"))?;
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
    let _ = f__need({ let t3827 = { let t3823 = py::len(&v_t.clone())?; Py::Bool({ let t3824 = py::len(&v_C.clone())?; if py::eq(&t3823, &t3824) { true } else { false } }) }; if !t3827.truthy() { t3827 } else { { let t3825 = py::len(&v_t.clone())?; Py::Bool({ let t3826 = Py::Int(2i64); if py::ge(&t3825, &t3826)? { true } else { false } }) } } }, Py::str("give at least two times and as many concentrations"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3828 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t3829 = (t3828).items()?;
if t3829.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t3829[0].clone();
v_b = t3829[1].clone();
out.push({ let t3830 = v_b.clone(); Py::Bool({ let t3831 = v_a.clone(); if py::gt(&t3830, &t3831)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the times must increase strictly"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t3832 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t3832;
out.push({ let t3833 = v_c.clone(); Py::Bool({ let t3834 = Py::Int(0i64); if py::ge(&t3833, &t3834)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("concentrations cannot be negative"))?;
    let _ = f__need({ let t3835 = v_method.clone(); Py::Bool({ let t3836 = Py::tuple(vec![Py::str("linear"), Py::str("linear_up_log_down")]); if py::contains(&t3836, &t3835)? { true } else { false } }) }, Py::str("method is linear or linear_up_log_down"))?;
    v_parts = Py::list(vec![]);
    for t3837 in py::iter(&(py::zip(&[py::zip(&[v_t.clone(), v_C.clone()])?, py::zip(&[py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?])?))? {
        let t3844 = (t3837).items()?;
        if t3844.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        let t3845 = (t3844[0].clone()).items()?;
        if t3845.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t1 = t3845[0].clone();
        v_c1 = t3845[1].clone();
        let t3846 = (t3844[1].clone()).items()?;
        if t3846.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_t2 = t3846[0].clone();
        v_c2 = t3846[1].clone();
        v_dt = py::sub(&v_t2.clone(), &v_t1.clone())?;
        if ({ let t3843 = { let t3838 = v_method.clone(); Py::Bool({ let t3839 = Py::str("linear_up_log_down"); if py::eq(&t3838, &t3839) { true } else { false } }) }; if !t3843.truthy() { t3843 } else { { let t3840 = Py::Int(0i64); Py::Bool({ let t3841 = v_c2.clone(); if py::lt(&t3840, &t3841)? { { let t3842 = v_c1.clone(); if py::lt(&t3841, &t3842)? { true } else { false } } } else { false } }) } } }).truthy() {
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
    let t3920 = (Py::tuple(vec![f__trap(v_t_iv.clone(), v_C_iv.clone(), Py::str("IV profile"))?, f__trap(v_t_oral.clone(), v_C_oral.clone(), Py::str("oral profile"))?])).items()?;
    if t3920.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a_iv = t3920[0].clone();
    v_a_or = t3920[1].clone();
    let _ = f__need({ let t3921 = v_a_iv.clone(); Py::Bool({ let t3922 = Py::Int(0i64); if py::gt(&t3921, &t3922)? { true } else { false } }) }, Py::str("the IV profile has no area"))?;
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
    let _ = f__need({ let t5162 = { let t5158 = py::len(&v_t.clone())?; Py::Bool({ let t5159 = py::len(&v_C.clone())?; if py::eq(&t5158, &t5159) { true } else { false } }) }; if !t5162.truthy() { t5162 } else { { let t5160 = py::len(&v_t.clone())?; Py::Bool({ let t5161 = Py::Int(3i64); if py::ge(&t5160, &t5161)? { true } else { false } }) } } }, Py::str("give at least three times and as many concentrations"))?;
    let t5177 = (f__loglin(v_t.clone(), v_C.clone(), Py::str("the concentrations"))?).items()?;
    if t5177.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t5177[0].clone();
    v_b = t5177[1].clone();
    let _ = f__need({ let t5178 = v_slope.clone(); Py::Bool({ let t5179 = Py::Int(0i64); if py::lt(&t5178, &t5179)? { true } else { false } }) }, Py::str("the concentrations do not decline (the fitted slope of ln C is not negative)"))?;
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
    let _ = f__need({ let t4256 = v_T_inf.clone(); Py::Bool({ let t4257 = v_tau.clone(); if py::lt(&t4256, &t4257)? { true } else { false } }) }, Py::str("the infusion must be shorter than the dosing interval"))?;
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
    let _ = f__need({ let t5378 = v_alpha.clone(); Py::Bool({ let t5379 = py::mul(&v_beta.clone(), &py::add(&Py::Int(1i64), &Py::Float(1e-09))?)?; if py::gt(&t5378, &t5379)? { true } else { false } }) }, Py::str("alpha must exceed beta"))?;
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
    let _ = f__need({ let t4519 = v_R0.clone(); Py::Bool({ let t4520 = v_Vmax.clone(); if py::lt(&t4519, &t4520)? { true } else { false } }) }, Py::str("no steady state: the dosing rate must be below Vmax"))?;
    return Ok(f__out(Py::dict(vec![(Py::str("C_ss"), py::truediv(&py::mul(&v_Km.clone(), &v_R0.clone())?, &py::sub(&v_Vmax.clone(), &v_R0.clone())?)?)]))?);
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn f_pk_mm_time(mut v_C0: Py, mut v_C: Py, mut v_Vmax: Py, mut v_Km: Py, mut v_V: Py) -> R {
    let _ = f__pos(Py::str("C0, C, Vmax, Km and V"), Py::tuple(vec![v_C0.clone(), v_C.clone(), v_Vmax.clone(), v_Km.clone(), v_V.clone()]))?;
    let _ = f__need({ let t4580 = v_C.clone(); Py::Bool({ let t4581 = v_C0.clone(); if py::lt(&t4580, &t4581)? { true } else { false } }) }, Py::str("the target concentration must be below C0 (the concentration only falls)"))?;
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
    let _ = f__need({ let t4645 = { let t4641 = py::len(&v_C.clone())?; Py::Bool({ let t4642 = v_n.clone(); if py::eq(&t4641, &t4642) { true } else { false } }) }; if !t4645.truthy() { t4645 } else { { let t4643 = v_n.clone(); Py::Bool({ let t4644 = Py::Int(4i64); if py::ge(&t4643, &t4644)? { true } else { false } }) } } }, Py::str("give at least four times and as many concentrations"))?;
    let _ = f__need({ let t4646 = py::getitem(&v_t.clone(), &Py::Int(0i64))?; Py::Bool({ let t4647 = Py::Int(0i64); if py::eq(&t4646, &t4647) { true } else { false } }) }, Py::str("the first sample must be at t = 0 (for a bolus, the measured or back-extrapolated C0)"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4648 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_a: Py; let mut v_b: Py; 
let t4649 = (t4648).items()?;
if t4649.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_a = t4649[0].clone();
v_b = t4649[1].clone();
out.push({ let t4650 = v_b.clone(); Py::Bool({ let t4651 = v_a.clone(); if py::gt(&t4650, &t4651)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("the times must increase strictly"))?;
    let _ = f__need(Py::Bool(({ let mut out = vec![]; for t4652 in py::iter(&(v_C.clone()))? { let mut v_c: Py; 
v_c = t4652;
out.push({ let t4653 = v_c.clone(); Py::Bool({ let t4654 = Py::Int(0i64); if py::ge(&t4653, &t4654)? { true } else { false } }) }); } Py::list(out) }).items()?.iter().all(|x| x.truthy())), Py::str("concentrations cannot be negative"))?;
    let _ = f__need({ let t4658 = Py::Bool(matches!(v_n_terminal.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4658.truthy() { t4658 } else { { let t4655 = Py::Int(3i64); Py::Bool({ let t4656 = v_n_terminal.clone(); if py::le(&t4655, &t4656)? { { let t4657 = py::sub(&v_n.clone(), &Py::Int(1i64))?; if py::le(&t4656, &t4657)? { true } else { false } } } else { false } }) } } }, Py::str("n_terminal (points used for lambda_z) is 3 to n - 1"))?;
    let t4659 = (Py::tuple(vec![py::slice(&v_t.clone(), Some((py::neg(&v_n_terminal.clone())?).i()?), None, None)?, py::slice(&v_C.clone(), Some((py::neg(&v_n_terminal.clone())?).i()?), None, None)?])).items()?;
    if t4659.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_tt = t4659[0].clone();
    v_cc = t4659[1].clone();
    let t4674 = (f__loglin(v_tt.clone(), v_cc.clone(), Py::str("the terminal concentrations"))?).items()?;
    if t4674.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_slope = t4674[0].clone();
    v_b = t4674[1].clone();
    let _ = f__need({ let t4675 = v_slope.clone(); Py::Bool({ let t4676 = Py::Int(0i64); if py::lt(&t4675, &t4676)? { true } else { false } }) }, Py::str("the terminal concentrations do not decline"))?;
    v_lz = py::neg(&v_slope.clone())?;
    v_auc = py::fsum(&{ let mut out = vec![]; for t4677 in py::iter(&(py::zip(&[v_t.clone(), py::slice(&v_t.clone(), Some((Py::Int(1i64)).i()?), None, None)?, v_C.clone(), py::slice(&v_C.clone(), Some((Py::Int(1i64)).i()?), None, None)?])?))? { let mut v_c1: Py; let mut v_c2: Py; let mut v_t1: Py; let mut v_t2: Py; 
let t4678 = (t4677).items()?;
if t4678.len() != 4 { return Err(PyErr::Value("unpack".into())); }
v_t1 = t4678[0].clone();
v_t2 = t4678[1].clone();
v_c1 = t4678[2].clone();
v_c2 = t4678[3].clone();
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
    let _ = f__need({ let t4741 = Py::Bool(matches!(v_n.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t4741.truthy() { t4741 } else { { let t4738 = Py::Int(1i64); Py::Bool({ let t4739 = v_n.clone(); if py::le(&t4738, &t4739)? { { let t4740 = Py::Int(200i64); if py::le(&t4739, &t4740)? { true } else { false } } } else { false } }) } } }, Py::str("n (the dose number) is a whole number from 1 to 200"))?;
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    let _ = f__need({ let t4748 = v_t.clone(); Py::Bool({ let t4749 = v_tau.clone(); if py::le(&t4748, &t4749)? { true } else { false } }) }, Py::str("t is the time after the nth dose and must lie within the dosing interval (t <= tau)"))?;
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
    v_C = (if ({ let t4967 = v_t.clone(); Py::Bool({ let t4968 = v_T_inf.clone(); if py::le(&t4967, &t4968)? { true } else { false } }) }).truthy() { py::mul(&py::truediv(&v_R0.clone(), &v_CL.clone())?, &py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_t.clone())?)?)?)? } else { py::mul(&v_cend.clone(), &py::math1("exp", &py::mul(&py::neg(&v_k.clone())?, &py::sub(&v_t.clone(), &v_T_inf.clone())?)?)?)? });
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
    let t5101 = (Py::tuple(vec![py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_k.clone())?, &v_tau.clone())?)?)?, py::neg(&py::math1("expm1", &py::mul(&py::neg(&v_ka.clone())?, &v_tau.clone())?)?)?])).items()?;
    if t5101.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_a1 = t5101[0].clone();
    v_b1 = t5101[1].clone();
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
    let _ = f__need({ let t5102 = Py::Int(0i64); Py::Bool({ let t5103 = v_tm.clone(); if py::lt(&t5102, &t5103)? { { let t5104 = v_tau.clone(); if py::lt(&t5103, &t5104)? { true } else { false } } } else { false } }) }, Py::str("the steady-state peak does not fall inside the dosing interval"))?;
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
    let t5316 = (f__macro(v_k10.clone(), v_k12.clone(), v_k21.clone())?).items()?;
    if t5316.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_al = t5316[0].clone();
    v_be = t5316[1].clone();
    let _ = f__need({ let t5317 = py::sub(&v_al.clone(), &v_be.clone())?; Py::Bool({ let t5318 = py::mul(&Py::Float(1e-09), &v_al.clone())?; if py::gt(&t5317, &t5318)? { true } else { false } }) }, Py::str("the two exponents coincide (degenerate model)"))?;
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
fn f_snr_averaging(mut v_SNR1: Py, mut v_N: Py, mut v_SNR_target: Py) -> R {
    let mut v_n: Py = Py::None;
    let _ = f__pos(Py::str("the single-acquisition SNR"), Py::tuple(vec![v_SNR1.clone()]))?;
    let _ = f__need({ let t6053 = { let t6055 = v_N.clone(); Py::Bool({ let t6056 = Py::None; if py::is(&t6055, &t6056) { true } else { false } }) }; Py::Bool({ let t6054 = { let t6057 = v_SNR_target.clone(); Py::Bool({ let t6058 = Py::None; if py::is(&t6057, &t6058) { true } else { false } }) }; if py::ne(&t6053, &t6054) { true } else { false } }) }, Py::str("give the number of averages N or a target SNR (one of them)"))?;
    if ({ let t6059 = v_N.clone(); Py::Bool({ let t6060 = Py::None; if !py::is(&t6059, &t6060) { true } else { false } }) }).truthy() {
        let _ = f__need({ let t6064 = Py::Bool(matches!(v_N.clone(), Py::Int(_) | Py::Big(_) | Py::Bool(_))); if !t6064.truthy() { t6064 } else { { let t6061 = Py::Int(1i64); Py::Bool({ let t6062 = v_N.clone(); if py::le(&t6061, &t6062)? { { let t6063 = py::pow(&Py::Int(10i64), &Py::Int(7i64))?; if py::le(&t6062, &t6063)? { true } else { false } } } else { false } }) } } }, Py::str("N must be a whole number from 1 to 10 million"))?;
        return Ok(f__out(Py::dict(vec![(Py::str("SNR"), py::mul(&v_SNR1.clone(), &py::math1("sqrt", &v_N.clone())?)?), (Py::str("gain"), py::math1("sqrt", &v_N.clone())?), (Py::str("N"), py::float(&v_N.clone())?)]))?);
    }
    let _ = f__pos(Py::str("the target SNR"), Py::tuple(vec![v_SNR_target.clone()]))?;
    let _ = f__need({ let t6077 = py::truediv(&v_SNR_target.clone(), &v_SNR1.clone())?; Py::Bool({ let t6078 = Py::Int(3000i64); if py::le(&t6077, &t6078)? { true } else { false } }) }, Py::str("the target needs more than 10 million averages"))?;
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
    let t1492 = (f__surv(v_times1.clone(), v_events1.clone(), Py::str("group 1"))?).items()?;
    if t1492.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t1492[0].clone();
    v_E1 = t1492[1].clone();
    let t1493 = (f__surv(v_times2.clone(), v_events2.clone(), Py::str("group 2"))?).items()?;
    if t1493.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T2 = t1493[0].clone();
    v_E2 = t1493[1].clone();
    let t1499 = (Py::tuple(vec![f__exp_rate(v_T1.clone(), v_E1.clone())?, f__exp_rate(v_T2.clone(), v_E2.clone())?])).items()?;
    if t1499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_l1 = t1499[0].clone();
    v_l2 = t1499[1].clone();
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
    let t1573 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t1573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t1573[0].clone();
    v_E = t1573[1].clone();
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
    let t2648 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t2648.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2648[0].clone();
    v_E = t2648[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_curve = f__km_curve(v_T.clone(), v_E.clone())?;
    v_st = Py::Int(1i64);
    v_med = Py::None;
    for t2673 in py::iter(&(v_curve.clone()))? {
        let t2681 = (t2673).items()?;
        if t2681.len() != 4 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2681[0].clone();
        v_s = t2681[1].clone();
        v_n = t2681[2].clone();
        v_d = t2681[3].clone();
        if ({ let t2674 = v_u.clone(); Py::Bool({ let t2675 = v_t.clone(); if py::le(&t2674, &t2675)? { true } else { false } }) }).truthy() {
            v_st = v_s.clone();
        }
        if ({ let t2680 = { let t2676 = v_med.clone(); Py::Bool({ let t2677 = Py::None; if py::is(&t2676, &t2677) { true } else { false } }) }; if !t2680.truthy() { t2680 } else { { let t2678 = v_s.clone(); Py::Bool({ let t2679 = f_Fraction_half()?; if py::le(&t2678, &t2679)? { true } else { false } }) } } }).truthy() {
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
    let t2546 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t2546.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t2546[0].clone();
    v_E = t2546[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_z = f__level(v_level.clone())?;
    let t2559 = (Py::tuple(vec![Py::Float(1.0), Py::Float(0.0), Py::Bool(false)])).items()?;
    if t2559.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_s = t2559[0].clone();
    v_V = t2559[1].clone();
    v_seen = t2559[2].clone();
    for t2575 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t2580 = (t2575).items()?;
        if t2580.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t2580[0].clone();
        v_n = t2580[1].clone();
        v_d = t2580[2].clone();
        if ({ let t2576 = v_u.clone(); Py::Bool({ let t2577 = v_t.clone(); if py::gt(&t2576, &t2577)? { true } else { false } }) }).truthy() {
            break;
        }
        let _ = f__need({ let t2578 = v_d.clone(); Py::Bool({ let t2579 = v_n.clone(); if py::lt(&t2578, &t2579)? { true } else { false } }) }, Py::str("the survival estimate reaches 0 by t: the log(-log) interval is undefined"))?;
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
    let t2956 = (f__surv(v_times1.clone(), v_events1.clone(), Py::str("group 1"))?).items()?;
    if t2956.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T1 = t2956[0].clone();
    v_E1 = t2956[1].clone();
    let t2957 = (f__surv(v_times2.clone(), v_events2.clone(), Py::str("group 2"))?).items()?;
    if t2957.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T2 = t2957[0].clone();
    v_E2 = t2957[1].clone();
    let t2958 = Py::Float(0.0);
    v_o1 = t2958.clone();
    v_e1 = t2958.clone();
    v_v = t2958.clone();
    for t2963 in py::iter(&(py::sorted(&py::set(&{ let mut out = vec![]; for t2959 in py::iter(&(py::zip(&[py::add(&v_T1.clone(), &v_T2.clone())?, py::add(&v_E1.clone(), &v_E2.clone())?])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2960 = (t2959).items()?;
if t2960.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2960[0].clone();
v_e = t2960[1].clone();
if ({ let t2961 = v_e.clone(); Py::Bool({ let t2962 = Py::Int(1i64); if py::eq(&t2961, &t2962) { true } else { false } }) }).truthy() { out.push(v_t.clone()); }} Py::list(out) })?, (Py::Bool(false)).truthy())?))? {
        v_u = t2963;
        let t2970 = (Py::tuple(vec![py::sum(&{ let mut out = vec![]; for t2964 in py::iter(&(v_T1.clone()))? { let mut v_t: Py; 
v_t = t2964;
if ({ let t2965 = v_t.clone(); Py::Bool({ let t2966 = v_u.clone(); if py::ge(&t2965, &t2966)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?, py::sum(&{ let mut out = vec![]; for t2967 in py::iter(&(v_T2.clone()))? { let mut v_t: Py; 
v_t = t2967;
if ({ let t2968 = v_t.clone(); Py::Bool({ let t2969 = v_u.clone(); if py::ge(&t2968, &t2969)? { true } else { false } }) }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?])).items()?;
        if t2970.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_n1 = t2970[0].clone();
        v_n2 = t2970[1].clone();
        v_d1 = py::sum(&{ let mut out = vec![]; for t2971 in py::iter(&(py::zip(&[v_T1.clone(), v_E1.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2972 = (t2971).items()?;
if t2972.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2972[0].clone();
v_e = t2972[1].clone();
if ({ let t2977 = { let t2973 = v_t.clone(); Py::Bool({ let t2974 = v_u.clone(); if py::eq(&t2973, &t2974) { true } else { false } }) }; if !t2977.truthy() { t2977 } else { { let t2975 = v_e.clone(); Py::Bool({ let t2976 = Py::Int(1i64); if py::eq(&t2975, &t2976) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?;
        v_d = py::add(&v_d1.clone(), &py::sum(&{ let mut out = vec![]; for t2978 in py::iter(&(py::zip(&[v_T2.clone(), v_E2.clone()])?))? { let mut v_e: Py; let mut v_t: Py; 
let t2979 = (t2978).items()?;
if t2979.len() != 2 { return Err(PyErr::Value("unpack".into())); }
v_t = t2979[0].clone();
v_e = t2979[1].clone();
if ({ let t2984 = { let t2980 = v_t.clone(); Py::Bool({ let t2981 = v_u.clone(); if py::eq(&t2980, &t2981) { true } else { false } }) }; if !t2984.truthy() { t2984 } else { { let t2982 = v_e.clone(); Py::Bool({ let t2983 = Py::Int(1i64); if py::eq(&t2982, &t2983) { true } else { false } }) } } }).truthy() { out.push(Py::Int(1i64)); }} Py::list(out) }, None)?)?;
        v_n = py::add(&v_n1.clone(), &v_n2.clone())?;
        v_o1 = py::iadd(&v_o1, &v_d1.clone())?;
        v_e1 = py::iadd(&v_e1, &py::truediv(&py::mul(&v_d.clone(), &v_n1.clone())?, &v_n.clone())?)?;
        if ({ let t2985 = v_n.clone(); Py::Bool({ let t2986 = Py::Int(1i64); if py::gt(&t2985, &t2986)? { true } else { false } }) }).truthy() {
            v_v = py::iadd(&v_v, &py::truediv(&py::mul(&py::mul(&py::mul(&v_d.clone(), &py::truediv(&v_n1.clone(), &v_n.clone())?)?, &py::truediv(&v_n2.clone(), &v_n.clone())?)?, &py::sub(&v_n.clone(), &v_d.clone())?)?, &py::sub(&v_n.clone(), &Py::Int(1i64))?)?)?;
        }
    }
    let _ = f__need({ let t2987 = v_v.clone(); Py::Bool({ let t2988 = Py::Int(0i64); if py::gt(&t2987, &t2988)? { true } else { false } }) }, Py::str("the log-rank variance is zero (no informative event times)"))?;
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
    let t3542 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t3542.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t3542[0].clone();
    v_E = t3542[1].clone();
    let _ = f__nonneg(Py::str("the time t"), Py::tuple(vec![v_t.clone()]))?;
    v_H = py::fsum(&{ let mut out = vec![]; for t3564 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? { let mut v_d: Py; let mut v_n: Py; let mut v_u: Py; 
let t3565 = (t3564).items()?;
if t3565.len() != 3 { return Err(PyErr::Value("unpack".into())); }
v_u = t3565[0].clone();
v_n = t3565[1].clone();
v_d = t3565[2].clone();
if ({ let t3566 = v_u.clone(); Py::Bool({ let t3567 = v_t.clone(); if py::le(&t3566, &t3567)? { true } else { false } }) }).truthy() { out.push(py::truediv(&v_d.clone(), &v_n.clone())?); }} Py::list(out) })?;
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
    let t5733 = (f__surv(v_times.clone(), v_events.clone(), Py::str("times"))?).items()?;
    if t5733.len() != 2 { return Err(PyErr::Value("unpack".into())); }
    v_T = t5733[0].clone();
    v_E = t5733[1].clone();
    let _ = f__pos(Py::str("tau"), Py::tuple(vec![v_tau.clone()]))?;
    let t5740 = (Py::tuple(vec![Py::Float(0.0), Py::Float(1.0), Py::Float(0.0)])).items()?;
    if t5740.len() != 3 { return Err(PyErr::Value("unpack".into())); }
    v_area = t5740[0].clone();
    v_s = t5740[1].clone();
    v_last = t5740[2].clone();
    v_parts = Py::list(vec![]);
    for t5756 in py::iter(&(f__km_steps(v_T.clone(), v_E.clone())?))? {
        let t5759 = (t5756).items()?;
        if t5759.len() != 3 { return Err(PyErr::Value("unpack".into())); }
        v_u = t5759[0].clone();
        v_n = t5759[1].clone();
        v_d = t5759[2].clone();
        if ({ let t5757 = v_u.clone(); Py::Bool({ let t5758 = v_tau.clone(); if py::ge(&t5757, &t5758)? { true } else { false } }) }).truthy() {
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
fn s_1004(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1005 = v_key.clone(); Py::Bool({ let t1006 = v_memo.clone(); if !py::contains(&t1006, &t1005)? { true } else { false } }) }).truthy() {
        let t1008: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1007 = v_inputs.clone(); f_epi_seir_final_size(py::getitem(&t1007, &Py::str("R0"))?, py::getitem(&t1007, &Py::str("s0"))?, py::getitem(&t1007, &Py::str("e0"))?, py::getitem(&t1007, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1008 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1009 = py::len(&v_memo.clone())?; Py::Bool({ let t1010 = Py::Int(64i64); if py::gt(&t1009, &t1010)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1011 = v_want.clone(); Py::Bool({ let t1012 = Py::None; if py::is(&t1011, &t1012) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1013 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1044 = (t1013).items()?;
        if t1044.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1044[0].clone();
        v_v = t1044[1].clone();
        if ({ let t1014 = v_k.clone(); Py::Bool({ let t1015 = v_outputs.clone(); if !py::contains(&t1015, &t1014)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1065(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1066 = v_key.clone(); Py::Bool({ let t1067 = v_memo.clone(); if !py::contains(&t1067, &t1066)? { true } else { false } }) }).truthy() {
        let t1069: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1068 = v_inputs.clone(); f_epi_seir_growth(py::getitem(&t1068, &Py::str("R0"))?, py::getitem(&t1068, &Py::str("sigma"))?, py::getitem(&t1068, &Py::str("gamma"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1069 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1070 = py::len(&v_memo.clone())?; Py::Bool({ let t1071 = Py::Int(64i64); if py::gt(&t1070, &t1071)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1072 = v_want.clone(); Py::Bool({ let t1073 = Py::None; if py::is(&t1072, &t1073) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1074 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1105 = (t1074).items()?;
        if t1105.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1105[0].clone();
        v_v = t1105[1].clone();
        if ({ let t1075 = v_k.clone(); Py::Bool({ let t1076 = v_outputs.clone(); if !py::contains(&t1076, &t1075)? { true } else { false } }) }).truthy() {
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
fn s_1180(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1181 = v_key.clone(); Py::Bool({ let t1182 = v_memo.clone(); if !py::contains(&t1182, &t1181)? { true } else { false } }) }).truthy() {
        let t1184: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1183 = v_inputs.clone(); f_epi_seir_peak(py::getitem(&t1183, &Py::str("beta"))?, py::getitem(&t1183, &Py::str("sigma"))?, py::getitem(&t1183, &Py::str("gamma"))?, py::getitem(&t1183, &Py::str("s0"))?, py::getitem(&t1183, &Py::str("e0"))?, py::getitem(&t1183, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1184 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1185 = py::len(&v_memo.clone())?; Py::Bool({ let t1186 = Py::Int(64i64); if py::gt(&t1185, &t1186)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1187 = v_want.clone(); Py::Bool({ let t1188 = Py::None; if py::is(&t1187, &t1188) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1189 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1220 = (t1189).items()?;
        if t1220.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1220[0].clone();
        v_v = t1220[1].clone();
        if ({ let t1190 = v_k.clone(); Py::Bool({ let t1191 = v_outputs.clone(); if !py::contains(&t1191, &t1190)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1279(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1280 = v_key.clone(); Py::Bool({ let t1281 = v_memo.clone(); if !py::contains(&t1281, &t1280)? { true } else { false } }) }).truthy() {
        let t1283: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1282 = v_inputs.clone(); f_epi_sir_final_size(py::getitem(&t1282, &Py::str("R0"))?, py::getitem(&t1282, &Py::str("s0"))?, py::getitem(&t1282, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1283 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1284 = py::len(&v_memo.clone())?; Py::Bool({ let t1285 = Py::Int(64i64); if py::gt(&t1284, &t1285)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1286 = v_want.clone(); Py::Bool({ let t1287 = Py::None; if py::is(&t1286, &t1287) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1288 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1319 = (t1288).items()?;
        if t1319.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1319[0].clone();
        v_v = t1319[1].clone();
        if ({ let t1289 = v_k.clone(); Py::Bool({ let t1290 = v_outputs.clone(); if !py::contains(&t1290, &t1289)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1358(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1359 = v_key.clone(); Py::Bool({ let t1360 = v_memo.clone(); if !py::contains(&t1360, &t1359)? { true } else { false } }) }).truthy() {
        let t1362: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1361 = v_inputs.clone(); f_epi_sir_peak(py::getitem(&t1361, &Py::str("beta"))?, py::getitem(&t1361, &Py::str("gamma"))?, py::getitem(&t1361, &Py::str("s0"))?, py::getitem(&t1361, &Py::str("i0"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1362 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1363 = py::len(&v_memo.clone())?; Py::Bool({ let t1364 = Py::Int(64i64); if py::gt(&t1363, &t1364)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1365 = v_want.clone(); Py::Bool({ let t1366 = Py::None; if py::is(&t1365, &t1366) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1367 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1398 = (t1367).items()?;
        if t1398.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1398[0].clone();
        v_v = t1398[1].clone();
        if ({ let t1368 = v_k.clone(); Py::Bool({ let t1369 = v_outputs.clone(); if !py::contains(&t1369, &t1368)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1437(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1438 = v_key.clone(); Py::Bool({ let t1439 = v_memo.clone(); if !py::contains(&t1439, &t1438)? { true } else { false } }) }).truthy() {
        let t1441: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1440 = v_inputs.clone(); f_epi_sis(py::getitem(&t1440, &Py::str("beta"))?, py::getitem(&t1440, &Py::str("gamma"))?, py::getitem(&t1440, &Py::str("i0"))?, py::getitem(&t1440, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1441 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1442 = py::len(&v_memo.clone())?; Py::Bool({ let t1443 = Py::Int(64i64); if py::gt(&t1442, &t1443)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1444 = v_want.clone(); Py::Bool({ let t1445 = Py::None; if py::is(&t1444, &t1445) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1446 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1477 = (t1446).items()?;
        if t1477.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1477[0].clone();
        v_v = t1477[1].clone();
        if ({ let t1447 = v_k.clone(); Py::Bool({ let t1448 = v_outputs.clone(); if !py::contains(&t1448, &t1447)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1518(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1519 = v_key.clone(); Py::Bool({ let t1520 = v_memo.clone(); if !py::contains(&t1520, &t1519)? { true } else { false } }) }).truthy() {
        let t1522: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1521 = v_inputs.clone(); f_sv_exp_hr(py::getitem(&t1521, &Py::str("times1"))?, py::getitem(&t1521, &Py::str("events1"))?, py::getitem(&t1521, &Py::str("times2"))?, py::getitem(&t1521, &Py::str("events2"))?, py::getitem(&t1521, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1522 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1523 = py::len(&v_memo.clone())?; Py::Bool({ let t1524 = Py::Int(64i64); if py::gt(&t1523, &t1524)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1525 = v_want.clone(); Py::Bool({ let t1526 = Py::None; if py::is(&t1525, &t1526) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1527 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1558 = (t1527).items()?;
        if t1558.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1558[0].clone();
        v_v = t1558[1].clone();
        if ({ let t1528 = v_k.clone(); Py::Bool({ let t1529 = v_outputs.clone(); if !py::contains(&t1529, &t1528)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1591(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1592 = v_key.clone(); Py::Bool({ let t1593 = v_memo.clone(); if !py::contains(&t1593, &t1592)? { true } else { false } }) }).truthy() {
        let t1595: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1594 = v_inputs.clone(); f_sv_exponential(py::getitem(&t1594, &Py::str("times"))?, py::getitem(&t1594, &Py::str("events"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1595 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1596 = py::len(&v_memo.clone())?; Py::Bool({ let t1597 = Py::Int(64i64); if py::gt(&t1596, &t1597)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1598 = v_want.clone(); Py::Bool({ let t1599 = Py::None; if py::is(&t1598, &t1599) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1600 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1631 = (t1600).items()?;
        if t1631.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1631[0].clone();
        v_v = t1631[1].clone();
        if ({ let t1601 = v_k.clone(); Py::Bool({ let t1602 = v_outputs.clone(); if !py::contains(&t1602, &t1601)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1689(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1690 = v_key.clone(); Py::Bool({ let t1691 = v_memo.clone(); if !py::contains(&t1691, &t1690)? { true } else { false } }) }).truthy() {
        let t1693: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1692 = v_inputs.clone(); f_ne_fhn(py::getitem(&t1692, &Py::str("a"))?, py::getitem(&t1692, &Py::str("b"))?, py::getitem(&t1692, &Py::str("eps"))?, py::getitem(&t1692, &Py::str("I"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1693 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1694 = py::len(&v_memo.clone())?; Py::Bool({ let t1695 = Py::Int(64i64); if py::gt(&t1694, &t1695)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1696 = v_want.clone(); Py::Bool({ let t1697 = Py::None; if py::is(&t1696, &t1697) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1698 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1729 = (t1698).items()?;
        if t1729.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1729[0].clone();
        v_v = t1729[1].clone();
        if ({ let t1699 = v_k.clone(); Py::Bool({ let t1700 = v_outputs.clone(); if !py::contains(&t1700, &t1699)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1754(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1755 = v_key.clone(); Py::Bool({ let t1756 = v_memo.clone(); if !py::contains(&t1756, &t1755)? { true } else { false } }) }).truthy() {
        let t1758: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1757 = v_inputs.clone(); f_im_psf_mtf(py::getitem(&t1757, &Py::str("fwhm"))?, py::getitem(&t1757, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1758 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1759 = py::len(&v_memo.clone())?; Py::Bool({ let t1760 = Py::Int(64i64); if py::gt(&t1759, &t1760)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1761 = v_want.clone(); Py::Bool({ let t1762 = Py::None; if py::is(&t1761, &t1762) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1763 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1794 = (t1763).items()?;
        if t1794.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1794[0].clone();
        v_v = t1794[1].clone();
        if ({ let t1764 = v_k.clone(); Py::Bool({ let t1765 = v_outputs.clone(); if !py::contains(&t1765, &t1764)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1830(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1831 = v_key.clone(); Py::Bool({ let t1832 = v_memo.clone(); if !py::contains(&t1832, &t1831)? { true } else { false } }) }).truthy() {
        let t1834: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1833 = v_inputs.clone(); f_ne_ghk_current(py::getitem(&t1833, &Py::str("P"))?, py::getitem(&t1833, &Py::str("z"))?, py::getitem(&t1833, &Py::str("C_in"))?, py::getitem(&t1833, &Py::str("C_out"))?, py::getitem(&t1833, &Py::str("V"))?, py::getitem(&t1833, &Py::str("T"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1834 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1835 = py::len(&v_memo.clone())?; Py::Bool({ let t1836 = Py::Int(64i64); if py::gt(&t1835, &t1836)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1837 = v_want.clone(); Py::Bool({ let t1838 = Py::None; if py::is(&t1837, &t1838) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1839 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1870 = (t1839).items()?;
        if t1870.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1870[0].clone();
        v_v = t1870[1].clone();
        if ({ let t1840 = v_k.clone(); Py::Bool({ let t1841 = v_outputs.clone(); if !py::contains(&t1841, &t1840)? { true } else { false } }) }).truthy() {
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
fn s_1896(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1897 = v_key.clone(); Py::Bool({ let t1898 = v_memo.clone(); if !py::contains(&t1898, &t1897)? { true } else { false } }) }).truthy() {
        let t1900: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1899 = v_inputs.clone(); f_ne_ghk_voltage(py::getitem(&t1899, &Py::str("T"))?, py::getitem(&t1899, &Py::str("PK"))?, py::getitem(&t1899, &Py::str("PNa"))?, py::getitem(&t1899, &Py::str("PCl"))?, py::getitem(&t1899, &Py::str("K_o"))?, py::getitem(&t1899, &Py::str("K_i"))?, py::getitem(&t1899, &Py::str("Na_o"))?, py::getitem(&t1899, &Py::str("Na_i"))?, py::getitem(&t1899, &Py::str("Cl_o"))?, py::getitem(&t1899, &Py::str("Cl_i"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1900 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1901 = py::len(&v_memo.clone())?; Py::Bool({ let t1902 = Py::Int(64i64); if py::gt(&t1901, &t1902)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1903 = v_want.clone(); Py::Bool({ let t1904 = Py::None; if py::is(&t1903, &t1904) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1905 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t1936 = (t1905).items()?;
        if t1936.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t1936[0].clone();
        v_v = t1936[1].clone();
        if ({ let t1906 = v_k.clone(); Py::Bool({ let t1907 = v_outputs.clone(); if !py::contains(&t1907, &t1906)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_1964(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t1965 = v_key.clone(); Py::Bool({ let t1966 = v_memo.clone(); if !py::contains(&t1966, &t1965)? { true } else { false } }) }).truthy() {
        let t1968: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t1967 = v_inputs.clone(); f_ne_hh_gating(py::getitem(&t1967, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t1968 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t1969 = py::len(&v_memo.clone())?; Py::Bool({ let t1970 = Py::Int(64i64); if py::gt(&t1969, &t1970)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t1971 = v_want.clone(); Py::Bool({ let t1972 = Py::None; if py::is(&t1971, &t1972) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t1973 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2004 = (t1973).items()?;
        if t2004.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2004[0].clone();
        v_v = t2004[1].clone();
        if ({ let t1974 = v_k.clone(); Py::Bool({ let t1975 = v_outputs.clone(); if !py::contains(&t1975, &t1974)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2050(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2051 = v_key.clone(); Py::Bool({ let t2052 = v_memo.clone(); if !py::contains(&t2052, &t2051)? { true } else { false } }) }).truthy() {
        let t2054: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2053 = v_inputs.clone(); f_ne_hh_gk_clamp(py::getitem(&t2053, &Py::str("gK_max"))?, py::getitem(&t2053, &Py::str("V_hold"))?, py::getitem(&t2053, &Py::str("V_step"))?, py::getitem(&t2053, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2054 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2055 = py::len(&v_memo.clone())?; Py::Bool({ let t2056 = Py::Int(64i64); if py::gt(&t2055, &t2056)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2057 = v_want.clone(); Py::Bool({ let t2058 = Py::None; if py::is(&t2057, &t2058) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2059 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2090 = (t2059).items()?;
        if t2090.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2090[0].clone();
        v_v = t2090[1].clone();
        if ({ let t2060 = v_k.clone(); Py::Bool({ let t2061 = v_outputs.clone(); if !py::contains(&t2061, &t2060)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2218(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2219 = v_key.clone(); Py::Bool({ let t2220 = v_memo.clone(); if !py::contains(&t2220, &t2219)? { true } else { false } }) }).truthy() {
        let t2222: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2221 = v_inputs.clone(); f_ne_hh_rest(py::getitem(&t2221, &Py::str("gNa"))?, py::getitem(&t2221, &Py::str("gK"))?, py::getitem(&t2221, &Py::str("gL"))?, py::getitem(&t2221, &Py::str("ENa"))?, py::getitem(&t2221, &Py::str("EK"))?, py::getitem(&t2221, &Py::str("EL"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2222 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2223 = py::len(&v_memo.clone())?; Py::Bool({ let t2224 = Py::Int(64i64); if py::gt(&t2223, &t2224)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2225 = v_want.clone(); Py::Bool({ let t2226 = Py::None; if py::is(&t2225, &t2226) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2227 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2258 = (t2227).items()?;
        if t2258.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2258[0].clone();
        v_v = t2258[1].clone();
        if ({ let t2228 = v_k.clone(); Py::Bool({ let t2229 = v_outputs.clone(); if !py::contains(&t2229, &t2228)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2282(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2283 = v_key.clone(); Py::Bool({ let t2284 = v_memo.clone(); if !py::contains(&t2284, &t2283)? { true } else { false } }) }).truthy() {
        let t2286: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2285 = v_inputs.clone(); f_hounsfield(py::getitem(&t2285, &Py::str("mu"))?, py::getitem(&t2285, &Py::str("mu_water"))?, py::method(&t2285, "get", &[Py::str("mu_air"), Py::Float(0.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2286 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2287 = py::len(&v_memo.clone())?; Py::Bool({ let t2288 = Py::Int(64i64); if py::gt(&t2287, &t2288)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2289 = v_want.clone(); Py::Bool({ let t2290 = Py::None; if py::is(&t2289, &t2290) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2291 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2322 = (t2291).items()?;
        if t2322.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2322[0].clone();
        v_v = t2322[1].clone();
        if ({ let t2292 = v_k.clone(); Py::Bool({ let t2293 = v_outputs.clone(); if !py::contains(&t2293, &t2292)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2351(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2352 = v_key.clone(); Py::Bool({ let t2353 = v_memo.clone(); if !py::contains(&t2353, &t2352)? { true } else { false } }) }).truthy() {
        let t2355: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2354 = v_inputs.clone(); f_dx_irr(py::getitem(&t2354, &Py::str("events1"))?, py::getitem(&t2354, &Py::str("time1"))?, py::getitem(&t2354, &Py::str("events2"))?, py::getitem(&t2354, &Py::str("time2"))?, py::getitem(&t2354, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2355 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2356 = py::len(&v_memo.clone())?; Py::Bool({ let t2357 = Py::Int(64i64); if py::gt(&t2356, &t2357)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2358 = v_want.clone(); Py::Bool({ let t2359 = Py::None; if py::is(&t2358, &t2359) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2360 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2391 = (t2360).items()?;
        if t2391.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2391[0].clone();
        v_v = t2391[1].clone();
        if ({ let t2361 = v_k.clone(); Py::Bool({ let t2362 = v_outputs.clone(); if !py::contains(&t2362, &t2361)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2426(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2427 = v_key.clone(); Py::Bool({ let t2428 = v_memo.clone(); if !py::contains(&t2428, &t2427)? { true } else { false } }) }).truthy() {
        let t2430: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2429 = v_inputs.clone(); f_bm_indicator_dilution(py::getitem(&t2429, &Py::str("m"))?, py::getitem(&t2429, &Py::str("t"))?, py::getitem(&t2429, &Py::str("C"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2430 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2431 = py::len(&v_memo.clone())?; Py::Bool({ let t2432 = Py::Int(64i64); if py::gt(&t2431, &t2432)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2433 = v_want.clone(); Py::Bool({ let t2434 = Py::None; if py::is(&t2433, &t2434) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2435 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2466 = (t2435).items()?;
        if t2466.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2466[0].clone();
        v_v = t2466[1].clone();
        if ({ let t2436 = v_k.clone(); Py::Bool({ let t2437 = v_outputs.clone(); if !py::contains(&t2437, &t2436)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2491(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2492 = v_key.clone(); Py::Bool({ let t2493 = v_memo.clone(); if !py::contains(&t2493, &t2492)? { true } else { false } }) }).truthy() {
        let t2495: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2494 = v_inputs.clone(); f_bm_kelvin_voigt(py::getitem(&t2494, &Py::str("sigma"))?, py::getitem(&t2494, &Py::str("E"))?, py::getitem(&t2494, &Py::str("eta"))?, py::getitem(&t2494, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2495 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2496 = py::len(&v_memo.clone())?; Py::Bool({ let t2497 = Py::Int(64i64); if py::gt(&t2496, &t2497)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2498 = v_want.clone(); Py::Bool({ let t2499 = Py::None; if py::is(&t2498, &t2499) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2500 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2531 = (t2500).items()?;
        if t2531.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2531[0].clone();
        v_v = t2531[1].clone();
        if ({ let t2501 = v_k.clone(); Py::Bool({ let t2502 = v_outputs.clone(); if !py::contains(&t2502, &t2501)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2593(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2594 = v_key.clone(); Py::Bool({ let t2595 = v_memo.clone(); if !py::contains(&t2595, &t2594)? { true } else { false } }) }).truthy() {
        let t2597: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2596 = v_inputs.clone(); f_sv_km_ci(py::getitem(&t2596, &Py::str("times"))?, py::getitem(&t2596, &Py::str("events"))?, py::getitem(&t2596, &Py::str("t"))?, py::getitem(&t2596, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2597 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2598 = py::len(&v_memo.clone())?; Py::Bool({ let t2599 = Py::Int(64i64); if py::gt(&t2598, &t2599)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2600 = v_want.clone(); Py::Bool({ let t2601 = Py::None; if py::is(&t2600, &t2601) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2602 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2633 = (t2602).items()?;
        if t2633.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2633[0].clone();
        v_v = t2633[1].clone();
        if ({ let t2603 = v_k.clone(); Py::Bool({ let t2604 = v_outputs.clone(); if !py::contains(&t2604, &t2603)? { true } else { false } }) }).truthy() {
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
fn s_2694(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2695 = v_key.clone(); Py::Bool({ let t2696 = v_memo.clone(); if !py::contains(&t2696, &t2695)? { true } else { false } }) }).truthy() {
        let t2698: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2697 = v_inputs.clone(); f_sv_km(py::getitem(&t2697, &Py::str("times"))?, py::getitem(&t2697, &Py::str("events"))?, py::getitem(&t2697, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2698 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2699 = py::len(&v_memo.clone())?; Py::Bool({ let t2700 = Py::Int(64i64); if py::gt(&t2699, &t2700)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2701 = v_want.clone(); Py::Bool({ let t2702 = Py::None; if py::is(&t2701, &t2702) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2703 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2734 = (t2703).items()?;
        if t2734.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2734[0].clone();
        v_v = t2734[1].clone();
        if ({ let t2704 = v_k.clone(); Py::Bool({ let t2705 = v_outputs.clone(); if !py::contains(&t2705, &t2704)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2761(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2762 = v_key.clone(); Py::Bool({ let t2763 = v_memo.clone(); if !py::contains(&t2763, &t2762)? { true } else { false } }) }).truthy() {
        let t2765: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2764 = v_inputs.clone(); f_larmor(py::getitem(&t2764, &Py::str("B"))?, py::method(&t2764, "get", &[Py::str("gamma_bar"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2765 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2766 = py::len(&v_memo.clone())?; Py::Bool({ let t2767 = Py::Int(64i64); if py::gt(&t2766, &t2767)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2768 = v_want.clone(); Py::Bool({ let t2769 = Py::None; if py::is(&t2768, &t2769) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2770 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2801 = (t2770).items()?;
        if t2801.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2801[0].clone();
        v_v = t2801[1].clone();
        if ({ let t2771 = v_k.clone(); Py::Bool({ let t2772 = v_outputs.clone(); if !py::contains(&t2772, &t2771)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2833(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2834 = v_key.clone(); Py::Bool({ let t2835 = v_memo.clone(); if !py::contains(&t2835, &t2834)? { true } else { false } }) }).truthy() {
        let t2837: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2836 = v_inputs.clone(); f_ne_lif_rate(py::getitem(&t2836, &Py::str("tau_m"))?, py::getitem(&t2836, &Py::str("R_m"))?, py::getitem(&t2836, &Py::str("I"))?, py::getitem(&t2836, &Py::str("V_rest"))?, py::getitem(&t2836, &Py::str("V_th"))?, py::getitem(&t2836, &Py::str("V_reset"))?, py::getitem(&t2836, &Py::str("t_ref"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2837 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2838 = py::len(&v_memo.clone())?; Py::Bool({ let t2839 = Py::Int(64i64); if py::gt(&t2838, &t2839)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2840 = v_want.clone(); Py::Bool({ let t2841 = Py::None; if py::is(&t2840, &t2841) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2842 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2873 = (t2842).items()?;
        if t2873.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2873[0].clone();
        v_v = t2873[1].clone();
        if ({ let t2843 = v_k.clone(); Py::Bool({ let t2844 = v_outputs.clone(); if !py::contains(&t2844, &t2843)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_2901(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t2902 = v_key.clone(); Py::Bool({ let t2903 = v_memo.clone(); if !py::contains(&t2903, &t2902)? { true } else { false } }) }).truthy() {
        let t2905: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t2904 = v_inputs.clone(); f_ne_lif_voltage(py::getitem(&t2904, &Py::str("tau_m"))?, py::getitem(&t2904, &Py::str("R_m"))?, py::getitem(&t2904, &Py::str("I"))?, py::getitem(&t2904, &Py::str("V_rest"))?, py::getitem(&t2904, &Py::str("V0"))?, py::getitem(&t2904, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t2905 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t2906 = py::len(&v_memo.clone())?; Py::Bool({ let t2907 = Py::Int(64i64); if py::gt(&t2906, &t2907)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t2908 = v_want.clone(); Py::Bool({ let t2909 = Py::None; if py::is(&t2908, &t2909) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t2910 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t2941 = (t2910).items()?;
        if t2941.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t2941[0].clone();
        v_v = t2941[1].clone();
        if ({ let t2911 = v_k.clone(); Py::Bool({ let t2912 = v_outputs.clone(); if !py::contains(&t2912, &t2911)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3001(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3002 = v_key.clone(); Py::Bool({ let t3003 = v_memo.clone(); if !py::contains(&t3003, &t3002)? { true } else { false } }) }).truthy() {
        let t3005: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3004 = v_inputs.clone(); f_sv_logrank(py::getitem(&t3004, &Py::str("times1"))?, py::getitem(&t3004, &Py::str("events1"))?, py::getitem(&t3004, &Py::str("times2"))?, py::getitem(&t3004, &Py::str("events2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3005 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3006 = py::len(&v_memo.clone())?; Py::Bool({ let t3007 = Py::Int(64i64); if py::gt(&t3006, &t3007)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3008 = v_want.clone(); Py::Bool({ let t3009 = Py::None; if py::is(&t3008, &t3009) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3010 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3041 = (t3010).items()?;
        if t3041.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3041[0].clone();
        v_v = t3041[1].clone();
        if ({ let t3011 = v_k.clone(); Py::Bool({ let t3012 = v_outputs.clone(); if !py::contains(&t3012, &t3011)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3090(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3091 = v_key.clone(); Py::Bool({ let t3092 = v_memo.clone(); if !py::contains(&t3092, &t3091)? { true } else { false } }) }).truthy() {
        let t3094: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3093 = v_inputs.clone(); f_dx_mantel_haenszel(py::getitem(&t3093, &Py::str("tables"))?, py::getitem(&t3093, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3094 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3095 = py::len(&v_memo.clone())?; Py::Bool({ let t3096 = Py::Int(64i64); if py::gt(&t3095, &t3096)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3097 = v_want.clone(); Py::Bool({ let t3098 = Py::None; if py::is(&t3097, &t3098) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3099 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3130 = (t3099).items()?;
        if t3130.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3130[0].clone();
        v_v = t3130[1].clone();
        if ({ let t3100 = v_k.clone(); Py::Bool({ let t3101 = v_outputs.clone(); if !py::contains(&t3101, &t3100)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3166(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3167 = v_key.clone(); Py::Bool({ let t3168 = v_memo.clone(); if !py::contains(&t3168, &t3167)? { true } else { false } }) }).truthy() {
        let t3170: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3169 = v_inputs.clone(); f_im_adc(py::getitem(&t3169, &Py::str("b"))?, py::getitem(&t3169, &Py::str("S"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3170 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3171 = py::len(&v_memo.clone())?; Py::Bool({ let t3172 = Py::Int(64i64); if py::gt(&t3171, &t3172)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3173 = v_want.clone(); Py::Bool({ let t3174 = Py::None; if py::is(&t3173, &t3174) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3175 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3206 = (t3175).items()?;
        if t3206.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3206[0].clone();
        v_v = t3206[1].clone();
        if ({ let t3176 = v_k.clone(); Py::Bool({ let t3177 = v_outputs.clone(); if !py::contains(&t3177, &t3176)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3225(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3226 = v_key.clone(); Py::Bool({ let t3227 = v_memo.clone(); if !py::contains(&t3227, &t3226)? { true } else { false } }) }).truthy() {
        let t3229: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3228 = v_inputs.clone(); f_ernst_angle(py::getitem(&t3228, &Py::str("TR"))?, py::getitem(&t3228, &Py::str("T1"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3229 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3230 = py::len(&v_memo.clone())?; Py::Bool({ let t3231 = Py::Int(64i64); if py::gt(&t3230, &t3231)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3232 = v_want.clone(); Py::Bool({ let t3233 = Py::None; if py::is(&t3232, &t3233) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3234 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3265 = (t3234).items()?;
        if t3265.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3265[0].clone();
        v_v = t3265[1].clone();
        if ({ let t3235 = v_k.clone(); Py::Bool({ let t3236 = v_outputs.clone(); if !py::contains(&t3236, &t3235)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3286(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3287 = v_key.clone(); Py::Bool({ let t3288 = v_memo.clone(); if !py::contains(&t3288, &t3287)? { true } else { false } }) }).truthy() {
        let t3290: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3289 = v_inputs.clone(); f_im_inversion_recovery(py::getitem(&t3289, &Py::str("T1"))?, py::getitem(&t3289, &Py::str("TR"))?, py::getitem(&t3289, &Py::str("TI"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3290 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3291 = py::len(&v_memo.clone())?; Py::Bool({ let t3292 = Py::Int(64i64); if py::gt(&t3291, &t3292)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3293 = v_want.clone(); Py::Bool({ let t3294 = Py::None; if py::is(&t3293, &t3294) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3295 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3326 = (t3295).items()?;
        if t3326.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3326[0].clone();
        v_v = t3326[1].clone();
        if ({ let t3296 = v_k.clone(); Py::Bool({ let t3297 = v_outputs.clone(); if !py::contains(&t3297, &t3296)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3347(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3348 = v_key.clone(); Py::Bool({ let t3349 = v_memo.clone(); if !py::contains(&t3349, &t3348)? { true } else { false } }) }).truthy() {
        let t3351: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3350 = v_inputs.clone(); f_im_spin_echo(py::getitem(&t3350, &Py::str("M0"))?, py::getitem(&t3350, &Py::str("TR"))?, py::getitem(&t3350, &Py::str("TE"))?, py::getitem(&t3350, &Py::str("T1"))?, py::getitem(&t3350, &Py::str("T2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3351 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3352 = py::len(&v_memo.clone())?; Py::Bool({ let t3353 = Py::Int(64i64); if py::gt(&t3352, &t3353)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3354 = v_want.clone(); Py::Bool({ let t3355 = Py::None; if py::is(&t3354, &t3355) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3356 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3387 = (t3356).items()?;
        if t3387.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3387[0].clone();
        v_v = t3387[1].clone();
        if ({ let t3357 = v_k.clone(); Py::Bool({ let t3358 = v_outputs.clone(); if !py::contains(&t3358, &t3357)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3411(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3412 = v_key.clone(); Py::Bool({ let t3413 = v_memo.clone(); if !py::contains(&t3413, &t3412)? { true } else { false } }) }).truthy() {
        let t3415: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3414 = v_inputs.clone(); f_im_spoiled_gre(py::getitem(&t3414, &Py::str("M0"))?, py::getitem(&t3414, &Py::str("TR"))?, py::getitem(&t3414, &Py::str("TE"))?, py::getitem(&t3414, &Py::str("T1"))?, py::getitem(&t3414, &Py::str("T2star"))?, py::getitem(&t3414, &Py::str("alpha"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3415 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3416 = py::len(&v_memo.clone())?; Py::Bool({ let t3417 = Py::Int(64i64); if py::gt(&t3416, &t3417)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3418 = v_want.clone(); Py::Bool({ let t3419 = Py::None; if py::is(&t3418, &t3419) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3420 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3451 = (t3420).items()?;
        if t3451.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3451[0].clone();
        v_v = t3451[1].clone();
        if ({ let t3421 = v_k.clone(); Py::Bool({ let t3422 = v_outputs.clone(); if !py::contains(&t3422, &t3421)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3487(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3488 = v_key.clone(); Py::Bool({ let t3489 = v_memo.clone(); if !py::contains(&t3489, &t3488)? { true } else { false } }) }).truthy() {
        let t3491: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3490 = v_inputs.clone(); f_im_t2_fit(py::getitem(&t3490, &Py::str("TE"))?, py::getitem(&t3490, &Py::str("S"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3491 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3492 = py::len(&v_memo.clone())?; Py::Bool({ let t3493 = Py::Int(64i64); if py::gt(&t3492, &t3493)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3494 = v_want.clone(); Py::Bool({ let t3495 = Py::None; if py::is(&t3494, &t3495) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3496 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3527 = (t3496).items()?;
        if t3527.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3527[0].clone();
        v_v = t3527[1].clone();
        if ({ let t3497 = v_k.clone(); Py::Bool({ let t3498 = v_outputs.clone(); if !py::contains(&t3498, &t3497)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3580(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3581 = v_key.clone(); Py::Bool({ let t3582 = v_memo.clone(); if !py::contains(&t3582, &t3581)? { true } else { false } }) }).truthy() {
        let t3584: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3583 = v_inputs.clone(); f_sv_nelson_aalen(py::getitem(&t3583, &Py::str("times"))?, py::getitem(&t3583, &Py::str("events"))?, py::getitem(&t3583, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3584 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3585 = py::len(&v_memo.clone())?; Py::Bool({ let t3586 = Py::Int(64i64); if py::gt(&t3585, &t3586)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3587 = v_want.clone(); Py::Bool({ let t3588 = Py::None; if py::is(&t3587, &t3588) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3589 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3620 = (t3589).items()?;
        if t3620.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3620[0].clone();
        v_v = t3620[1].clone();
        if ({ let t3590 = v_k.clone(); Py::Bool({ let t3591 = v_outputs.clone(); if !py::contains(&t3591, &t3590)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3652(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3653 = v_key.clone(); Py::Bool({ let t3654 = v_memo.clone(); if !py::contains(&t3654, &t3653)? { true } else { false } }) }).truthy() {
        let t3656: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3655 = v_inputs.clone(); f_dx_nnt(py::getitem(&t3655, &Py::str("events_t"))?, py::getitem(&t3655, &Py::str("n_t"))?, py::getitem(&t3655, &Py::str("events_c"))?, py::getitem(&t3655, &Py::str("n_c"))?, py::getitem(&t3655, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3656 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3657 = py::len(&v_memo.clone())?; Py::Bool({ let t3658 = Py::Int(64i64); if py::gt(&t3657, &t3658)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3659 = v_want.clone(); Py::Bool({ let t3660 = Py::None; if py::is(&t3659, &t3660) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3661 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3692 = (t3661).items()?;
        if t3692.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3692[0].clone();
        v_v = t3692[1].clone();
        if ({ let t3662 = v_k.clone(); Py::Bool({ let t3663 = v_outputs.clone(); if !py::contains(&t3663, &t3662)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3717(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3718 = v_key.clone(); Py::Bool({ let t3719 = v_memo.clone(); if !py::contains(&t3719, &t3718)? { true } else { false } }) }).truthy() {
        let t3721: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3720 = v_inputs.clone(); f_dx_odds_ratio(py::getitem(&t3720, &Py::str("a"))?, py::getitem(&t3720, &Py::str("b"))?, py::getitem(&t3720, &Py::str("c"))?, py::getitem(&t3720, &Py::str("d"))?, py::getitem(&t3720, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3721 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3722 = py::len(&v_memo.clone())?; Py::Bool({ let t3723 = Py::Int(64i64); if py::gt(&t3722, &t3723)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3724 = v_want.clone(); Py::Bool({ let t3725 = Py::None; if py::is(&t3724, &t3725) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3726 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3757 = (t3726).items()?;
        if t3757.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3757[0].clone();
        v_v = t3757[1].clone();
        if ({ let t3727 = v_k.clone(); Py::Bool({ let t3728 = v_outputs.clone(); if !py::contains(&t3728, &t3727)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3782(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3783 = v_key.clone(); Py::Bool({ let t3784 = v_memo.clone(); if !py::contains(&t3784, &t3783)? { true } else { false } }) }).truthy() {
        let t3786: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3785 = v_inputs.clone(); f_pk_auc_iv_bolus(py::getitem(&t3785, &Py::str("D"))?, py::getitem(&t3785, &Py::str("CL"))?, py::getitem(&t3785, &Py::str("V"))?, py::getitem(&t3785, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3786 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3787 = py::len(&v_memo.clone())?; Py::Bool({ let t3788 = Py::Int(64i64); if py::gt(&t3787, &t3788)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3789 = v_want.clone(); Py::Bool({ let t3790 = Py::None; if py::is(&t3789, &t3790) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3791 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3822 = (t3791).items()?;
        if t3822.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3822[0].clone();
        v_v = t3822[1].clone();
        if ({ let t3792 = v_k.clone(); Py::Bool({ let t3793 = v_outputs.clone(); if !py::contains(&t3793, &t3792)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_3859(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3860 = v_key.clone(); Py::Bool({ let t3861 = v_memo.clone(); if !py::contains(&t3861, &t3860)? { true } else { false } }) }).truthy() {
        let t3863: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3862 = v_inputs.clone(); f_pk_auc_trapezoid(py::getitem(&t3862, &Py::str("t"))?, py::getitem(&t3862, &Py::str("C"))?, py::getitem(&t3862, &Py::str("method"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3863 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3864 = py::len(&v_memo.clone())?; Py::Bool({ let t3865 = Py::Int(64i64); if py::gt(&t3864, &t3865)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3866 = v_want.clone(); Py::Bool({ let t3867 = Py::None; if py::is(&t3866, &t3867) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3868 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3899 = (t3868).items()?;
        if t3899.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3899[0].clone();
        v_v = t3899[1].clone();
        if ({ let t3869 = v_k.clone(); Py::Bool({ let t3870 = v_outputs.clone(); if !py::contains(&t3870, &t3869)? { true } else { false } }) }).truthy() {
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
fn s_3935(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t3936 = v_key.clone(); Py::Bool({ let t3937 = v_memo.clone(); if !py::contains(&t3937, &t3936)? { true } else { false } }) }).truthy() {
        let t3939: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t3938 = v_inputs.clone(); f_pk_bioavailability(py::getitem(&t3938, &Py::str("D_iv"))?, py::getitem(&t3938, &Py::str("t_iv"))?, py::getitem(&t3938, &Py::str("C_iv"))?, py::getitem(&t3938, &Py::str("D_oral"))?, py::getitem(&t3938, &Py::str("t_oral"))?, py::getitem(&t3938, &Py::str("C_oral"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t3939 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t3940 = py::len(&v_memo.clone())?; Py::Bool({ let t3941 = Py::Int(64i64); if py::gt(&t3940, &t3941)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t3942 = v_want.clone(); Py::Bool({ let t3943 = Py::None; if py::is(&t3942, &t3943) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t3944 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t3975 = (t3944).items()?;
        if t3975.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t3975[0].clone();
        v_v = t3975[1].clone();
        if ({ let t3945 = v_k.clone(); Py::Bool({ let t3946 = v_outputs.clone(); if !py::contains(&t3946, &t3945)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4002(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4003 = v_key.clone(); Py::Bool({ let t4004 = v_memo.clone(); if !py::contains(&t4004, &t4003)? { true } else { false } }) }).truthy() {
        let t4006: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4005 = v_inputs.clone(); f_pk_effect_site(py::getitem(&t4005, &Py::str("D"))?, py::getitem(&t4005, &Py::str("V"))?, py::getitem(&t4005, &Py::str("k"))?, py::getitem(&t4005, &Py::str("ke0"))?, py::getitem(&t4005, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4006 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4007 = py::len(&v_memo.clone())?; Py::Bool({ let t4008 = Py::Int(64i64); if py::gt(&t4007, &t4008)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4009 = v_want.clone(); Py::Bool({ let t4010 = Py::None; if py::is(&t4009, &t4010) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4011 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4042 = (t4011).items()?;
        if t4042.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4042[0].clone();
        v_v = t4042[1].clone();
        if ({ let t4012 = v_k.clone(); Py::Bool({ let t4013 = v_outputs.clone(); if !py::contains(&t4013, &t4012)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4073(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4074 = v_key.clone(); Py::Bool({ let t4075 = v_memo.clone(); if !py::contains(&t4075, &t4074)? { true } else { false } }) }).truthy() {
        let t4077: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4076 = v_inputs.clone(); f_pk_parallel_tube(py::getitem(&t4076, &Py::str("Q"))?, py::getitem(&t4076, &Py::str("fu"))?, py::getitem(&t4076, &Py::str("CL_int"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4077 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4078 = py::len(&v_memo.clone())?; Py::Bool({ let t4079 = Py::Int(64i64); if py::gt(&t4078, &t4079)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4080 = v_want.clone(); Py::Bool({ let t4081 = Py::None; if py::is(&t4080, &t4081) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4082 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4113 = (t4082).items()?;
        if t4113.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4113[0].clone();
        v_v = t4113[1].clone();
        if ({ let t4083 = v_k.clone(); Py::Bool({ let t4084 = v_outputs.clone(); if !py::contains(&t4084, &t4083)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4144(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4145 = v_key.clone(); Py::Bool({ let t4146 = v_memo.clone(); if !py::contains(&t4146, &t4145)? { true } else { false } }) }).truthy() {
        let t4148: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4147 = v_inputs.clone(); f_pk_well_stirred(py::getitem(&t4147, &Py::str("Q"))?, py::getitem(&t4147, &Py::str("fu"))?, py::getitem(&t4147, &Py::str("CL_int"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4148 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4149 = py::len(&v_memo.clone())?; Py::Bool({ let t4150 = Py::Int(64i64); if py::gt(&t4149, &t4150)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4151 = v_want.clone(); Py::Bool({ let t4152 = Py::None; if py::is(&t4151, &t4152) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4153 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4184 = (t4153).items()?;
        if t4184.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4184[0].clone();
        v_v = t4184[1].clone();
        if ({ let t4154 = v_k.clone(); Py::Bool({ let t4155 = v_outputs.clone(); if !py::contains(&t4155, &t4154)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4209(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4210 = v_key.clone(); Py::Bool({ let t4211 = v_memo.clone(); if !py::contains(&t4211, &t4210)? { true } else { false } }) }).truthy() {
        let t4213: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4212 = v_inputs.clone(); f_pk_infusion(py::getitem(&t4212, &Py::str("R0"))?, py::getitem(&t4212, &Py::str("CL"))?, py::getitem(&t4212, &Py::str("V"))?, py::getitem(&t4212, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4213 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4214 = py::len(&v_memo.clone())?; Py::Bool({ let t4215 = Py::Int(64i64); if py::gt(&t4214, &t4215)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4216 = v_want.clone(); Py::Bool({ let t4217 = Py::None; if py::is(&t4216, &t4217) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4218 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4249 = (t4218).items()?;
        if t4249.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4249[0].clone();
        v_v = t4249[1].clone();
        if ({ let t4219 = v_k.clone(); Py::Bool({ let t4220 = v_outputs.clone(); if !py::contains(&t4220, &t4219)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4270(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4271 = v_key.clone(); Py::Bool({ let t4272 = v_memo.clone(); if !py::contains(&t4272, &t4271)? { true } else { false } }) }).truthy() {
        let t4274: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4273 = v_inputs.clone(); f_pk_intermittent_infusion(py::getitem(&t4273, &Py::str("R0"))?, py::getitem(&t4273, &Py::str("T_inf"))?, py::getitem(&t4273, &Py::str("tau"))?, py::getitem(&t4273, &Py::str("CL"))?, py::getitem(&t4273, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4274 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4275 = py::len(&v_memo.clone())?; Py::Bool({ let t4276 = Py::Int(64i64); if py::gt(&t4275, &t4276)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4277 = v_want.clone(); Py::Bool({ let t4278 = Py::None; if py::is(&t4277, &t4278) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4279 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4310 = (t4279).items()?;
        if t4310.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4310[0].clone();
        v_v = t4310[1].clone();
        if ({ let t4280 = v_k.clone(); Py::Bool({ let t4281 = v_outputs.clone(); if !py::contains(&t4281, &t4280)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4335(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4336 = v_key.clone(); Py::Bool({ let t4337 = v_memo.clone(); if !py::contains(&t4337, &t4336)? { true } else { false } }) }).truthy() {
        let t4339: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4338 = v_inputs.clone(); f_pk_iv_bolus(py::getitem(&t4338, &Py::str("D"))?, py::getitem(&t4338, &Py::str("V"))?, py::getitem(&t4338, &Py::str("k"))?, py::getitem(&t4338, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4339 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4340 = py::len(&v_memo.clone())?; Py::Bool({ let t4341 = Py::Int(64i64); if py::gt(&t4340, &t4341)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4342 = v_want.clone(); Py::Bool({ let t4343 = Py::None; if py::is(&t4342, &t4343) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4344 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4375 = (t4344).items()?;
        if t4375.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4375[0].clone();
        v_v = t4375[1].clone();
        if ({ let t4345 = v_k.clone(); Py::Bool({ let t4346 = v_outputs.clone(); if !py::contains(&t4346, &t4345)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4394(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4395 = v_key.clone(); Py::Bool({ let t4396 = v_memo.clone(); if !py::contains(&t4396, &t4395)? { true } else { false } }) }).truthy() {
        let t4398: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4397 = v_inputs.clone(); f_pk_loading_maintenance(py::getitem(&t4397, &Py::str("C_target"))?, py::getitem(&t4397, &Py::str("V"))?, py::getitem(&t4397, &Py::str("CL"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4398 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4399 = py::len(&v_memo.clone())?; Py::Bool({ let t4400 = Py::Int(64i64); if py::gt(&t4399, &t4400)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4401 = v_want.clone(); Py::Bool({ let t4402 = Py::None; if py::is(&t4401, &t4402) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4403 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4434 = (t4403).items()?;
        if t4434.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4434[0].clone();
        v_v = t4434[1].clone();
        if ({ let t4404 = v_k.clone(); Py::Bool({ let t4405 = v_outputs.clone(); if !py::contains(&t4405, &t4404)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4472(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4473 = v_key.clone(); Py::Bool({ let t4474 = v_memo.clone(); if !py::contains(&t4474, &t4473)? { true } else { false } }) }).truthy() {
        let t4476: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4475 = v_inputs.clone(); f_pk_mm_concentration(py::getitem(&t4475, &Py::str("C0"))?, py::getitem(&t4475, &Py::str("Vmax"))?, py::getitem(&t4475, &Py::str("Km"))?, py::getitem(&t4475, &Py::str("V"))?, py::getitem(&t4475, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4476 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4477 = py::len(&v_memo.clone())?; Py::Bool({ let t4478 = Py::Int(64i64); if py::gt(&t4477, &t4478)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4479 = v_want.clone(); Py::Bool({ let t4480 = Py::None; if py::is(&t4479, &t4480) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4481 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4512 = (t4481).items()?;
        if t4512.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4512[0].clone();
        v_v = t4512[1].clone();
        if ({ let t4482 = v_k.clone(); Py::Bool({ let t4483 = v_outputs.clone(); if !py::contains(&t4483, &t4482)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4533(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4534 = v_key.clone(); Py::Bool({ let t4535 = v_memo.clone(); if !py::contains(&t4535, &t4534)? { true } else { false } }) }).truthy() {
        let t4537: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4536 = v_inputs.clone(); f_pk_mm_steady_state(py::getitem(&t4536, &Py::str("R0"))?, py::getitem(&t4536, &Py::str("Vmax"))?, py::getitem(&t4536, &Py::str("Km"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4537 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4538 = py::len(&v_memo.clone())?; Py::Bool({ let t4539 = Py::Int(64i64); if py::gt(&t4538, &t4539)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4540 = v_want.clone(); Py::Bool({ let t4541 = Py::None; if py::is(&t4540, &t4541) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4542 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4573 = (t4542).items()?;
        if t4573.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4573[0].clone();
        v_v = t4573[1].clone();
        if ({ let t4543 = v_k.clone(); Py::Bool({ let t4544 = v_outputs.clone(); if !py::contains(&t4544, &t4543)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_459(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t460 = v_key.clone(); Py::Bool({ let t461 = v_memo.clone(); if !py::contains(&t461, &t460)? { true } else { false } }) }).truthy() {
        let t463: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t462 = v_inputs.clone(); f_dx_roc_auc(py::getitem(&t462, &Py::str("scores_pos"))?, py::getitem(&t462, &Py::str("scores_neg"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t463 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t464 = py::len(&v_memo.clone())?; Py::Bool({ let t465 = Py::Int(64i64); if py::gt(&t464, &t465)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t466 = v_want.clone(); Py::Bool({ let t467 = Py::None; if py::is(&t466, &t467) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t468 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t499 = (t468).items()?;
        if t499.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t499[0].clone();
        v_v = t499[1].clone();
        if ({ let t469 = v_k.clone(); Py::Bool({ let t470 = v_outputs.clone(); if !py::contains(&t470, &t469)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4594(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4595 = v_key.clone(); Py::Bool({ let t4596 = v_memo.clone(); if !py::contains(&t4596, &t4595)? { true } else { false } }) }).truthy() {
        let t4598: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4597 = v_inputs.clone(); f_pk_mm_time(py::getitem(&t4597, &Py::str("C0"))?, py::getitem(&t4597, &Py::str("C"))?, py::getitem(&t4597, &Py::str("Vmax"))?, py::getitem(&t4597, &Py::str("Km"))?, py::getitem(&t4597, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4598 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4599 = py::len(&v_memo.clone())?; Py::Bool({ let t4600 = Py::Int(64i64); if py::gt(&t4599, &t4600)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4601 = v_want.clone(); Py::Bool({ let t4602 = Py::None; if py::is(&t4601, &t4602) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4603 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4634 = (t4603).items()?;
        if t4634.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4634[0].clone();
        v_v = t4634[1].clone();
        if ({ let t4604 = v_k.clone(); Py::Bool({ let t4605 = v_outputs.clone(); if !py::contains(&t4605, &t4604)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4691(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4692 = v_key.clone(); Py::Bool({ let t4693 = v_memo.clone(); if !py::contains(&t4693, &t4692)? { true } else { false } }) }).truthy() {
        let t4695: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4694 = v_inputs.clone(); f_pk_nca_iv(py::getitem(&t4694, &Py::str("D"))?, py::getitem(&t4694, &Py::str("t"))?, py::getitem(&t4694, &Py::str("C"))?, py::getitem(&t4694, &Py::str("n_terminal"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4695 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4696 = py::len(&v_memo.clone())?; Py::Bool({ let t4697 = Py::Int(64i64); if py::gt(&t4696, &t4697)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4698 = v_want.clone(); Py::Bool({ let t4699 = Py::None; if py::is(&t4698, &t4699) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4700 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4731 = (t4700).items()?;
        if t4731.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4731[0].clone();
        v_v = t4731[1].clone();
        if ({ let t4701 = v_k.clone(); Py::Bool({ let t4702 = v_outputs.clone(); if !py::contains(&t4702, &t4701)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4762(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4763 = v_key.clone(); Py::Bool({ let t4764 = v_memo.clone(); if !py::contains(&t4764, &t4763)? { true } else { false } }) }).truthy() {
        let t4766: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4765 = v_inputs.clone(); f_pk_nth_dose(py::getitem(&t4765, &Py::str("D"))?, py::getitem(&t4765, &Py::str("V"))?, py::getitem(&t4765, &Py::str("k"))?, py::getitem(&t4765, &Py::str("tau"))?, py::getitem(&t4765, &Py::str("n"))?, py::getitem(&t4765, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4766 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4767 = py::len(&v_memo.clone())?; Py::Bool({ let t4768 = Py::Int(64i64); if py::gt(&t4767, &t4768)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4769 = v_want.clone(); Py::Bool({ let t4770 = Py::None; if py::is(&t4769, &t4770) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4771 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4802 = (t4771).items()?;
        if t4802.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4802[0].clone();
        v_v = t4802[1].clone();
        if ({ let t4772 = v_k.clone(); Py::Bool({ let t4773 = v_outputs.clone(); if !py::contains(&t4773, &t4772)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4841(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4842 = v_key.clone(); Py::Bool({ let t4843 = v_memo.clone(); if !py::contains(&t4843, &t4842)? { true } else { false } }) }).truthy() {
        let t4845: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4844 = v_inputs.clone(); f_pk_oral(py::getitem(&t4844, &Py::str("F"))?, py::getitem(&t4844, &Py::str("D"))?, py::getitem(&t4844, &Py::str("V"))?, py::getitem(&t4844, &Py::str("ka"))?, py::getitem(&t4844, &Py::str("k"))?, py::getitem(&t4844, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4845 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4846 = py::len(&v_memo.clone())?; Py::Bool({ let t4847 = Py::Int(64i64); if py::gt(&t4846, &t4847)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4848 = v_want.clone(); Py::Bool({ let t4849 = Py::None; if py::is(&t4848, &t4849) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4850 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4881 = (t4850).items()?;
        if t4881.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4881[0].clone();
        v_v = t4881[1].clone();
        if ({ let t4851 = v_k.clone(); Py::Bool({ let t4852 = v_outputs.clone(); if !py::contains(&t4852, &t4851)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4914(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4915 = v_key.clone(); Py::Bool({ let t4916 = v_memo.clone(); if !py::contains(&t4916, &t4915)? { true } else { false } }) }).truthy() {
        let t4918: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4917 = v_inputs.clone(); f_pk_oral_peak(py::getitem(&t4917, &Py::str("F"))?, py::getitem(&t4917, &Py::str("D"))?, py::getitem(&t4917, &Py::str("V"))?, py::getitem(&t4917, &Py::str("ka"))?, py::getitem(&t4917, &Py::str("k"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4918 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4919 = py::len(&v_memo.clone())?; Py::Bool({ let t4920 = Py::Int(64i64); if py::gt(&t4919, &t4920)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4921 = v_want.clone(); Py::Bool({ let t4922 = Py::None; if py::is(&t4921, &t4922) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4923 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t4954 = (t4923).items()?;
        if t4954.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t4954[0].clone();
        v_v = t4954[1].clone();
        if ({ let t4924 = v_k.clone(); Py::Bool({ let t4925 = v_outputs.clone(); if !py::contains(&t4925, &t4924)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_4981(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t4982 = v_key.clone(); Py::Bool({ let t4983 = v_memo.clone(); if !py::contains(&t4983, &t4982)? { true } else { false } }) }).truthy() {
        let t4985: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t4984 = v_inputs.clone(); f_pk_short_infusion(py::getitem(&t4984, &Py::str("R0"))?, py::getitem(&t4984, &Py::str("T_inf"))?, py::getitem(&t4984, &Py::str("CL"))?, py::getitem(&t4984, &Py::str("V"))?, py::getitem(&t4984, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t4985 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t4986 = py::len(&v_memo.clone())?; Py::Bool({ let t4987 = Py::Int(64i64); if py::gt(&t4986, &t4987)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t4988 = v_want.clone(); Py::Bool({ let t4989 = Py::None; if py::is(&t4988, &t4989) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t4990 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5021 = (t4990).items()?;
        if t5021.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5021[0].clone();
        v_v = t5021[1].clone();
        if ({ let t4991 = v_k.clone(); Py::Bool({ let t4992 = v_outputs.clone(); if !py::contains(&t4992, &t4991)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5040(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5041 = v_key.clone(); Py::Bool({ let t5042 = v_memo.clone(); if !py::contains(&t5042, &t5041)? { true } else { false } }) }).truthy() {
        let t5044: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5043 = v_inputs.clone(); f_pk_ss_iv(py::getitem(&t5043, &Py::str("D"))?, py::getitem(&t5043, &Py::str("V"))?, py::getitem(&t5043, &Py::str("k"))?, py::getitem(&t5043, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5044 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5045 = py::len(&v_memo.clone())?; Py::Bool({ let t5046 = Py::Int(64i64); if py::gt(&t5045, &t5046)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5047 = v_want.clone(); Py::Bool({ let t5048 = Py::None; if py::is(&t5047, &t5048) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5049 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5080 = (t5049).items()?;
        if t5080.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5080[0].clone();
        v_v = t5080[1].clone();
        if ({ let t5050 = v_k.clone(); Py::Bool({ let t5051 = v_outputs.clone(); if !py::contains(&t5051, &t5050)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5117(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5118 = v_key.clone(); Py::Bool({ let t5119 = v_memo.clone(); if !py::contains(&t5119, &t5118)? { true } else { false } }) }).truthy() {
        let t5121: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5120 = v_inputs.clone(); f_pk_ss_oral(py::getitem(&t5120, &Py::str("F"))?, py::getitem(&t5120, &Py::str("D"))?, py::getitem(&t5120, &Py::str("V"))?, py::getitem(&t5120, &Py::str("ka"))?, py::getitem(&t5120, &Py::str("k"))?, py::getitem(&t5120, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5121 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5122 = py::len(&v_memo.clone())?; Py::Bool({ let t5123 = Py::Int(64i64); if py::gt(&t5122, &t5123)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5124 = v_want.clone(); Py::Bool({ let t5125 = Py::None; if py::is(&t5124, &t5125) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5126 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5157 = (t5126).items()?;
        if t5157.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5157[0].clone();
        v_v = t5157[1].clone();
        if ({ let t5127 = v_k.clone(); Py::Bool({ let t5128 = v_outputs.clone(); if !py::contains(&t5128, &t5127)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5192(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5193 = v_key.clone(); Py::Bool({ let t5194 = v_memo.clone(); if !py::contains(&t5194, &t5193)? { true } else { false } }) }).truthy() {
        let t5196: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5195 = v_inputs.clone(); f_pk_half_life_fit(py::getitem(&t5195, &Py::str("t"))?, py::getitem(&t5195, &Py::str("C"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5196 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5197 = py::len(&v_memo.clone())?; Py::Bool({ let t5198 = Py::Int(64i64); if py::gt(&t5197, &t5198)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5199 = v_want.clone(); Py::Bool({ let t5200 = Py::None; if py::is(&t5199, &t5200) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5201 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5232 = (t5201).items()?;
        if t5232.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5232[0].clone();
        v_v = t5232[1].clone();
        if ({ let t5202 = v_k.clone(); Py::Bool({ let t5203 = v_outputs.clone(); if !py::contains(&t5203, &t5202)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5263(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5264 = v_key.clone(); Py::Bool({ let t5265 = v_memo.clone(); if !py::contains(&t5265, &t5264)? { true } else { false } }) }).truthy() {
        let t5267: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5266 = v_inputs.clone(); f_pk_time_to_fraction_ss(py::getitem(&t5266, &Py::str("CL"))?, py::getitem(&t5266, &Py::str("V"))?, py::getitem(&t5266, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5267 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5268 = py::len(&v_memo.clone())?; Py::Bool({ let t5269 = Py::Int(64i64); if py::gt(&t5268, &t5269)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5270 = v_want.clone(); Py::Bool({ let t5271 = Py::None; if py::is(&t5270, &t5271) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5272 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5303 = (t5272).items()?;
        if t5303.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5303[0].clone();
        v_v = t5303[1].clone();
        if ({ let t5273 = v_k.clone(); Py::Bool({ let t5274 = v_outputs.clone(); if !py::contains(&t5274, &t5273)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_531(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t532 = v_key.clone(); Py::Bool({ let t533 = v_memo.clone(); if !py::contains(&t533, &t532)? { true } else { false } }) }).truthy() {
        let t535: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t534 = v_inputs.clone(); f_dx_sens_spec_ci(py::getitem(&t534, &Py::str("TP"))?, py::getitem(&t534, &Py::str("FN"))?, py::getitem(&t534, &Py::str("TN"))?, py::getitem(&t534, &Py::str("FP"))?, py::getitem(&t534, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t535 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t536 = py::len(&v_memo.clone())?; Py::Bool({ let t537 = Py::Int(64i64); if py::gt(&t536, &t537)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t538 = v_want.clone(); Py::Bool({ let t539 = Py::None; if py::is(&t538, &t539) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t540 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t571 = (t540).items()?;
        if t571.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t571[0].clone();
        v_v = t571[1].clone();
        if ({ let t541 = v_k.clone(); Py::Bool({ let t542 = v_outputs.clone(); if !py::contains(&t542, &t541)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5331(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5332 = v_key.clone(); Py::Bool({ let t5333 = v_memo.clone(); if !py::contains(&t5333, &t5332)? { true } else { false } }) }).truthy() {
        let t5335: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5334 = v_inputs.clone(); f_pk_two_comp(py::getitem(&t5334, &Py::str("D"))?, py::getitem(&t5334, &Py::str("V1"))?, py::getitem(&t5334, &Py::str("k10"))?, py::getitem(&t5334, &Py::str("k12"))?, py::getitem(&t5334, &Py::str("k21"))?, py::getitem(&t5334, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5335 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5336 = py::len(&v_memo.clone())?; Py::Bool({ let t5337 = Py::Int(64i64); if py::gt(&t5336, &t5337)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5338 = v_want.clone(); Py::Bool({ let t5339 = Py::None; if py::is(&t5338, &t5339) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5340 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5371 = (t5340).items()?;
        if t5371.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5371[0].clone();
        v_v = t5371[1].clone();
        if ({ let t5341 = v_k.clone(); Py::Bool({ let t5342 = v_outputs.clone(); if !py::contains(&t5342, &t5341)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5392(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5393 = v_key.clone(); Py::Bool({ let t5394 = v_memo.clone(); if !py::contains(&t5394, &t5393)? { true } else { false } }) }).truthy() {
        let t5396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5395 = v_inputs.clone(); f_pk_micro_from_macro(py::getitem(&t5395, &Py::str("D"))?, py::getitem(&t5395, &Py::str("A"))?, py::getitem(&t5395, &Py::str("alpha"))?, py::getitem(&t5395, &Py::str("B"))?, py::getitem(&t5395, &Py::str("beta"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5396 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5397 = py::len(&v_memo.clone())?; Py::Bool({ let t5398 = Py::Int(64i64); if py::gt(&t5397, &t5398)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5399 = v_want.clone(); Py::Bool({ let t5400 = Py::None; if py::is(&t5399, &t5400) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5401 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5432 = (t5401).items()?;
        if t5432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5432[0].clone();
        v_v = t5432[1].clone();
        if ({ let t5402 = v_k.clone(); Py::Bool({ let t5403 = v_outputs.clone(); if !py::contains(&t5403, &t5402)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5451(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5452 = v_key.clone(); Py::Bool({ let t5453 = v_memo.clone(); if !py::contains(&t5453, &t5452)? { true } else { false } }) }).truthy() {
        let t5455: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5454 = v_inputs.clone(); f_bm_poiseuille(py::getitem(&t5454, &Py::str("dP"))?, py::getitem(&t5454, &Py::str("r"))?, py::getitem(&t5454, &Py::str("L"))?, py::getitem(&t5454, &Py::str("mu"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5455 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5456 = py::len(&v_memo.clone())?; Py::Bool({ let t5457 = Py::Int(64i64); if py::gt(&t5456, &t5457)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5458 = v_want.clone(); Py::Bool({ let t5459 = Py::None; if py::is(&t5458, &t5459) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5460 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5491 = (t5460).items()?;
        if t5491.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5491[0].clone();
        v_v = t5491[1].clone();
        if ({ let t5461 = v_k.clone(); Py::Bool({ let t5462 = v_outputs.clone(); if !py::contains(&t5462, &t5461)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5523(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5524 = v_key.clone(); Py::Bool({ let t5525 = v_memo.clone(); if !py::contains(&t5525, &t5524)? { true } else { false } }) }).truthy() {
        let t5527: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5526 = v_inputs.clone(); f_ne_psp(py::getitem(&t5526, &Py::str("tau_m"))?, py::getitem(&t5526, &Py::str("tau_s"))?, py::getitem(&t5526, &Py::str("R_m"))?, py::getitem(&t5526, &Py::str("I0"))?, py::getitem(&t5526, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5527 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5528 = py::len(&v_memo.clone())?; Py::Bool({ let t5529 = Py::Int(64i64); if py::gt(&t5528, &t5529)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5530 = v_want.clone(); Py::Bool({ let t5531 = Py::None; if py::is(&t5530, &t5531) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5532 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5563 = (t5532).items()?;
        if t5563.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5563[0].clone();
        v_v = t5563[1].clone();
        if ({ let t5533 = v_k.clone(); Py::Bool({ let t5534 = v_outputs.clone(); if !py::contains(&t5534, &t5533)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5588(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5589 = v_key.clone(); Py::Bool({ let t5590 = v_memo.clone(); if !py::contains(&t5590, &t5589)? { true } else { false } }) }).truthy() {
        let t5592: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5591 = v_inputs.clone(); f_dx_relative_risk(py::getitem(&t5591, &Py::str("a"))?, py::getitem(&t5591, &Py::str("b"))?, py::getitem(&t5591, &Py::str("c"))?, py::getitem(&t5591, &Py::str("d"))?, py::getitem(&t5591, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5592 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5593 = py::len(&v_memo.clone())?; Py::Bool({ let t5594 = Py::Int(64i64); if py::gt(&t5593, &t5594)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5595 = v_want.clone(); Py::Bool({ let t5596 = Py::None; if py::is(&t5595, &t5596) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5597 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5628 = (t5597).items()?;
        if t5628.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5628[0].clone();
        v_v = t5628[1].clone();
        if ({ let t5598 = v_k.clone(); Py::Bool({ let t5599 = v_outputs.clone(); if !py::contains(&t5599, &t5598)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5678(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5679 = v_key.clone(); Py::Bool({ let t5680 = v_memo.clone(); if !py::contains(&t5680, &t5679)? { true } else { false } }) }).truthy() {
        let t5682: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5681 = v_inputs.clone(); f_bm_respiratory_fit(py::getitem(&t5681, &Py::str("P"))?, py::getitem(&t5681, &Py::str("Q"))?, py::getitem(&t5681, &Py::str("V"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5682 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5683 = py::len(&v_memo.clone())?; Py::Bool({ let t5684 = Py::Int(64i64); if py::gt(&t5683, &t5684)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5685 = v_want.clone(); Py::Bool({ let t5686 = Py::None; if py::is(&t5685, &t5686) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5687 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5718 = (t5687).items()?;
        if t5718.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5718[0].clone();
        v_v = t5718[1].clone();
        if ({ let t5688 = v_k.clone(); Py::Bool({ let t5689 = v_outputs.clone(); if !py::contains(&t5689, &t5688)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5772(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5773 = v_key.clone(); Py::Bool({ let t5774 = v_memo.clone(); if !py::contains(&t5774, &t5773)? { true } else { false } }) }).truthy() {
        let t5776: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5775 = v_inputs.clone(); f_sv_rmst(py::getitem(&t5775, &Py::str("times"))?, py::getitem(&t5775, &Py::str("events"))?, py::getitem(&t5775, &Py::str("tau"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5776 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5777 = py::len(&v_memo.clone())?; Py::Bool({ let t5778 = Py::Int(64i64); if py::gt(&t5777, &t5778)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5779 = v_want.clone(); Py::Bool({ let t5780 = Py::None; if py::is(&t5779, &t5780) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5781 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5812 = (t5781).items()?;
        if t5812.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5812[0].clone();
        v_v = t5812[1].clone();
        if ({ let t5782 = v_k.clone(); Py::Bool({ let t5783 = v_outputs.clone(); if !py::contains(&t5783, &t5782)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5838(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5839 = v_key.clone(); Py::Bool({ let t5840 = v_memo.clone(); if !py::contains(&t5840, &t5839)? { true } else { false } }) }).truthy() {
        let t5842: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5841 = v_inputs.clone(); f_bm_sls_creep(py::getitem(&t5841, &Py::str("sigma"))?, py::getitem(&t5841, &Py::str("E_inf"))?, py::getitem(&t5841, &Py::str("E1"))?, py::getitem(&t5841, &Py::str("eta"))?, py::getitem(&t5841, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5842 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5843 = py::len(&v_memo.clone())?; Py::Bool({ let t5844 = Py::Int(64i64); if py::gt(&t5843, &t5844)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5845 = v_want.clone(); Py::Bool({ let t5846 = Py::None; if py::is(&t5845, &t5846) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5847 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5878 = (t5847).items()?;
        if t5878.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5878[0].clone();
        v_v = t5878[1].clone();
        if ({ let t5848 = v_k.clone(); Py::Bool({ let t5849 = v_outputs.clone(); if !py::contains(&t5849, &t5848)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_5903(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t5904 = v_key.clone(); Py::Bool({ let t5905 = v_memo.clone(); if !py::contains(&t5905, &t5904)? { true } else { false } }) }).truthy() {
        let t5907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t5906 = v_inputs.clone(); f_bm_sls_relaxation(py::getitem(&t5906, &Py::str("eps0"))?, py::getitem(&t5906, &Py::str("E_inf"))?, py::getitem(&t5906, &Py::str("E1"))?, py::getitem(&t5906, &Py::str("eta"))?, py::getitem(&t5906, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t5907 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t5908 = py::len(&v_memo.clone())?; Py::Bool({ let t5909 = Py::Int(64i64); if py::gt(&t5908, &t5909)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t5910 = v_want.clone(); Py::Bool({ let t5911 = Py::None; if py::is(&t5910, &t5911) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t5912 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t5943 = (t5912).items()?;
        if t5943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t5943[0].clone();
        v_v = t5943[1].clone();
        if ({ let t5913 = v_k.clone(); Py::Bool({ let t5914 = v_outputs.clone(); if !py::contains(&t5914, &t5913)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_600(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t601 = v_key.clone(); Py::Bool({ let t602 = v_memo.clone(); if !py::contains(&t602, &t601)? { true } else { false } }) }).truthy() {
        let t604: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t603 = v_inputs.clone(); f_dx_threshold(py::getitem(&t603, &Py::str("scores_pos"))?, py::getitem(&t603, &Py::str("scores_neg"))?, py::getitem(&t603, &Py::str("threshold"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t604 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t605 = py::len(&v_memo.clone())?; Py::Bool({ let t606 = Py::Int(64i64); if py::gt(&t605, &t606)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t607 = v_want.clone(); Py::Bool({ let t608 = Py::None; if py::is(&t607, &t608) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t609 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t640 = (t609).items()?;
        if t640.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t640[0].clone();
        v_v = t640[1].clone();
        if ({ let t610 = v_k.clone(); Py::Bool({ let t611 = v_outputs.clone(); if !py::contains(&t611, &t610)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6006(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6007 = v_key.clone(); Py::Bool({ let t6008 = v_memo.clone(); if !py::contains(&t6008, &t6007)? { true } else { false } }) }).truthy() {
        let t6010: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6009 = v_inputs.clone(); f_dx_smr(py::getitem(&t6009, &Py::str("observed"))?, py::getitem(&t6009, &Py::str("expected"))?, py::getitem(&t6009, &Py::str("level"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6010 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6011 = py::len(&v_memo.clone())?; Py::Bool({ let t6012 = Py::Int(64i64); if py::gt(&t6011, &t6012)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6013 = v_want.clone(); Py::Bool({ let t6014 = Py::None; if py::is(&t6013, &t6014) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6015 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6046 = (t6015).items()?;
        if t6046.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6046[0].clone();
        v_v = t6046[1].clone();
        if ({ let t6016 = v_k.clone(); Py::Bool({ let t6017 = v_outputs.clone(); if !py::contains(&t6017, &t6016)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6079(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6080 = v_key.clone(); Py::Bool({ let t6081 = v_memo.clone(); if !py::contains(&t6081, &t6080)? { true } else { false } }) }).truthy() {
        let t6083: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6082 = v_inputs.clone(); f_snr_averaging(py::getitem(&t6082, &Py::str("SNR1"))?, py::method(&t6082, "get", &[Py::str("N"), Py::None])?, py::method(&t6082, "get", &[Py::str("SNR_target"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6083 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6084 = py::len(&v_memo.clone())?; Py::Bool({ let t6085 = Py::Int(64i64); if py::gt(&t6084, &t6085)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6086 = v_want.clone(); Py::Bool({ let t6087 = Py::None; if py::is(&t6086, &t6087) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6088 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6119 = (t6088).items()?;
        if t6119.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6119[0].clone();
        v_v = t6119[1].clone();
        if ({ let t6089 = v_k.clone(); Py::Bool({ let t6090 = v_outputs.clone(); if !py::contains(&t6090, &t6089)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6138(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6139 = v_key.clone(); Py::Bool({ let t6140 = v_memo.clone(); if !py::contains(&t6140, &t6139)? { true } else { false } }) }).truthy() {
        let t6142: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6141 = v_inputs.clone(); f_im_us_interface(py::getitem(&t6141, &Py::str("Z1"))?, py::getitem(&t6141, &Py::str("Z2"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6142 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6143 = py::len(&v_memo.clone())?; Py::Bool({ let t6144 = Py::Int(64i64); if py::gt(&t6143, &t6144)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6145 = v_want.clone(); Py::Bool({ let t6146 = Py::None; if py::is(&t6145, &t6146) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6147 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6178 = (t6147).items()?;
        if t6178.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6178[0].clone();
        v_v = t6178[1].clone();
        if ({ let t6148 = v_k.clone(); Py::Bool({ let t6149 = v_outputs.clone(); if !py::contains(&t6149, &t6148)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6198(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6199 = v_key.clone(); Py::Bool({ let t6200 = v_memo.clone(); if !py::contains(&t6200, &t6199)? { true } else { false } }) }).truthy() {
        let t6202: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6201 = v_inputs.clone(); f_im_us_matching(py::getitem(&t6201, &Py::str("Z1"))?, py::getitem(&t6201, &Py::str("Zm"))?, py::getitem(&t6201, &Py::str("Z2"))?, py::getitem(&t6201, &Py::str("d"))?, py::getitem(&t6201, &Py::str("c_m"))?, py::getitem(&t6201, &Py::str("f"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6202 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6203 = py::len(&v_memo.clone())?; Py::Bool({ let t6204 = Py::Int(64i64); if py::gt(&t6203, &t6204)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6205 = v_want.clone(); Py::Bool({ let t6206 = Py::None; if py::is(&t6205, &t6206) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6207 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6238 = (t6207).items()?;
        if t6238.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6238[0].clone();
        v_v = t6238[1].clone();
        if ({ let t6208 = v_k.clone(); Py::Bool({ let t6209 = v_outputs.clone(); if !py::contains(&t6209, &t6208)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6257(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6258 = v_key.clone(); Py::Bool({ let t6259 = v_memo.clone(); if !py::contains(&t6259, &t6258)? { true } else { false } }) }).truthy() {
        let t6261: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6260 = v_inputs.clone(); f_us_wavelength(py::getitem(&t6260, &Py::str("f"))?, py::method(&t6260, "get", &[Py::str("c"), Py::Float(1540.0)])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6261 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6262 = py::len(&v_memo.clone())?; Py::Bool({ let t6263 = Py::Int(64i64); if py::gt(&t6262, &t6263)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6264 = v_want.clone(); Py::Bool({ let t6265 = Py::None; if py::is(&t6264, &t6265) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6266 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6297 = (t6266).items()?;
        if t6297.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6297[0].clone();
        v_v = t6297[1].clone();
        if ({ let t6267 = v_k.clone(); Py::Bool({ let t6268 = v_outputs.clone(); if !py::contains(&t6268, &t6267)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6324(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6325 = v_key.clone(); Py::Bool({ let t6326 = v_memo.clone(); if !py::contains(&t6326, &t6325)? { true } else { false } }) }).truthy() {
        let t6328: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6327 = v_inputs.clone(); f_bm_windkessel_decay(py::getitem(&t6327, &Py::str("P0"))?, py::getitem(&t6327, &Py::str("Pv"))?, py::getitem(&t6327, &Py::str("R"))?, py::getitem(&t6327, &Py::str("C"))?, py::getitem(&t6327, &Py::str("t"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6328 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6329 = py::len(&v_memo.clone())?; Py::Bool({ let t6330 = Py::Int(64i64); if py::gt(&t6329, &t6330)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6331 = v_want.clone(); Py::Bool({ let t6332 = Py::None; if py::is(&t6331, &t6332) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6333 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6364 = (t6333).items()?;
        if t6364.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6364[0].clone();
        v_v = t6364[1].clone();
        if ({ let t6334 = v_k.clone(); Py::Bool({ let t6335 = v_outputs.clone(); if !py::contains(&t6335, &t6334)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6392(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6393 = v_key.clone(); Py::Bool({ let t6394 = v_memo.clone(); if !py::contains(&t6394, &t6393)? { true } else { false } }) }).truthy() {
        let t6396: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6395 = v_inputs.clone(); f_bm_windkessel_impedance(py::getitem(&t6395, &Py::str("model"))?, py::getitem(&t6395, &Py::str("R"))?, py::getitem(&t6395, &Py::str("C"))?, py::getitem(&t6395, &Py::str("f"))?, py::method(&t6395, "get", &[Py::str("Zc"), Py::None])?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6396 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6397 = py::len(&v_memo.clone())?; Py::Bool({ let t6398 = Py::Int(64i64); if py::gt(&t6397, &t6398)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6399 = v_want.clone(); Py::Bool({ let t6400 = Py::None; if py::is(&t6399, &t6400) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6401 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6432 = (t6401).items()?;
        if t6432.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6432[0].clone();
        v_v = t6432[1].clone();
        if ({ let t6402 = v_k.clone(); Py::Bool({ let t6403 = v_outputs.clone(); if !py::contains(&t6403, &t6402)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_6467(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t6468 = v_key.clone(); Py::Bool({ let t6469 = v_memo.clone(); if !py::contains(&t6469, &t6468)? { true } else { false } }) }).truthy() {
        let t6471: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t6470 = v_inputs.clone(); f_im_xray(py::getitem(&t6470, &Py::str("I0"))?, py::getitem(&t6470, &Py::str("mu_rho"))?, py::getitem(&t6470, &Py::str("rho"))?, py::getitem(&t6470, &Py::str("x"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t6471 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t6472 = py::len(&v_memo.clone())?; Py::Bool({ let t6473 = Py::Int(64i64); if py::gt(&t6472, &t6473)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t6474 = v_want.clone(); Py::Bool({ let t6475 = Py::None; if py::is(&t6474, &t6475) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t6476 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t6507 = (t6476).items()?;
        if t6507.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t6507[0].clone();
        v_v = t6507[1].clone();
        if ({ let t6477 = v_k.clone(); Py::Bool({ let t6478 = v_outputs.clone(); if !py::contains(&t6478, &t6477)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_679(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t680 = v_key.clone(); Py::Bool({ let t681 = v_memo.clone(); if !py::contains(&t681, &t680)? { true } else { false } }) }).truthy() {
        let t683: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t682 = v_inputs.clone(); f_dx_youden(py::getitem(&t682, &Py::str("scores_pos"))?, py::getitem(&t682, &Py::str("scores_neg"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t683 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t684 = py::len(&v_memo.clone())?; Py::Bool({ let t685 = Py::Int(64i64); if py::gt(&t684, &t685)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t686 = v_want.clone(); Py::Bool({ let t687 = Py::None; if py::is(&t686, &t687) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t688 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t719 = (t688).items()?;
        if t719.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t719[0].clone();
        v_v = t719[1].clone();
        if ({ let t689 = v_k.clone(); Py::Bool({ let t690 = v_outputs.clone(); if !py::contains(&t690, &t689)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_753(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t754 = v_key.clone(); Py::Bool({ let t755 = v_memo.clone(); if !py::contains(&t755, &t754)? { true } else { false } }) }).truthy() {
        let t757: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t756 = v_inputs.clone(); f_bm_lever(py::getitem(&t756, &Py::str("W"))?, py::getitem(&t756, &Py::str("d_load"))?, py::getitem(&t756, &Py::str("W_arm"))?, py::getitem(&t756, &Py::str("d_arm"))?, py::getitem(&t756, &Py::str("d_muscle"))?, py::getitem(&t756, &Py::str("theta"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t757 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t758 = py::len(&v_memo.clone())?; Py::Bool({ let t759 = Py::Int(64i64); if py::gt(&t758, &t759)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t760 = v_want.clone(); Py::Bool({ let t761 = Py::None; if py::is(&t760, &t761) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t762 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t793 = (t762).items()?;
        if t793.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t793[0].clone();
        v_v = t793[1].clone();
        if ({ let t763 = v_k.clone(); Py::Bool({ let t764 = v_outputs.clone(); if !py::contains(&t764, &t763)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_817(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t818 = v_key.clone(); Py::Bool({ let t819 = v_memo.clone(); if !py::contains(&t819, &t818)? { true } else { false } }) }).truthy() {
        let t821: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t820 = v_inputs.clone(); f_epi_r_from_growth(py::getitem(&t820, &Py::str("r"))?, py::getitem(&t820, &Py::str("Tg_mean"))?, py::getitem(&t820, &Py::str("Tg_sd"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t821 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t822 = py::len(&v_memo.clone())?; Py::Bool({ let t823 = Py::Int(64i64); if py::gt(&t822, &t823)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t824 = v_want.clone(); Py::Bool({ let t825 = Py::None; if py::is(&t824, &t825) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t826 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t857 = (t826).items()?;
        if t857.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t857[0].clone();
        v_v = t857[1].clone();
        if ({ let t827 = v_k.clone(); Py::Bool({ let t828 = v_outputs.clone(); if !py::contains(&t828, &t827)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}

#[allow(unused_mut, unused_variables, unused_assignments, non_snake_case, unreachable_code)]
fn s_903(mut v_inputs: Py, mut v_outputs: Py) -> R {
    let mut v_memo: Py = Py::dict(vec![]);
    let mut v_k: Py = Py::None;
    let mut v_key: Py = Py::None;
    let mut v_v: Py = Py::None;
    let mut v_want: Py = Py::None;
    let mut v_worst: Py = Py::None;
    v_key = Py::str(&py::repr(&py::sorted(&py::method(&v_inputs.clone(), "items", &[])?, (Py::Bool(false)).truthy())?));
    if ({ let t904 = v_key.clone(); Py::Bool({ let t905 = v_memo.clone(); if !py::contains(&t905, &t904)? { true } else { false } }) }).truthy() {
        let t907: Result<Option<Py>, PyErr> = (|| -> Result<Option<Py>, PyErr> {
                py::setitem(&v_memo.clone(), &v_key.clone(), { let t906 = v_inputs.clone(); f_epi_reed_frost(py::getitem(&t906, &Py::str("n"))?, py::getitem(&t906, &Py::str("m"))?, py::getitem(&t906, &Py::str("p"))?)? })?;
                #[allow(unreachable_code)]
                Ok(None)
            })();
        match t907 {
            Ok(Some(r)) => { return Ok(r); }
            Ok(None) => {}
            Err(e) if (!matches!(e, PyErr::Unsupported(_))) => {
                py::setitem(&v_memo.clone(), &v_key.clone(), Py::None)?;
            }
            Err(e) => return Err(e),
        }
        if ({ let t908 = py::len(&v_memo.clone())?; Py::Bool({ let t909 = Py::Int(64i64); if py::gt(&t908, &t909)? { true } else { false } }) }).truthy() {
            let _ = py::method(&v_memo.clone(), "pop", &[(match (v_memo.clone()).items()?.into_iter().next() { Some(x) => x, None => return Err(PyErr::Other("StopIteration".into())) })])?;
        }
    }
    v_want = py::getitem(&v_memo.clone(), &v_key.clone())?;
    if ({ let t910 = v_want.clone(); Py::Bool({ let t911 = Py::None; if py::is(&t910, &t911) { true } else { false } }) }).truthy() {
        return Ok(Py::Float(f64::INFINITY));
    }
    v_worst = Py::Float(0.0);
    for t912 in py::iter(&(py::method(&v_want.clone(), "items", &[])?))? {
        let t943 = (t912).items()?;
        if t943.len() != 2 { return Err(PyErr::Value("unpack".into())); }
        v_k = t943[0].clone();
        v_v = t943[1].clone();
        if ({ let t913 = v_k.clone(); Py::Bool({ let t914 = v_outputs.clone(); if !py::contains(&t914, &t913)? { true } else { false } }) }).truthy() {
            return Ok(Py::Float(f64::INFINITY));
        }
        v_worst = py::minmax(&[v_worst.clone(), f__rel(py::getitem(&v_outputs.clone(), &v_k.clone())?, v_v.clone())?], true)?;
    }
    return Ok(v_worst.clone());
    #[allow(unreachable_code)]
    Ok(Py::None)
}
