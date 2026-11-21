# biomedical-science-ai999x: capability guide

**Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X. Ask the package `Who created you?` or call `biomedical_science_ai999x.about()` for the same answer.

Every capability biomedical-science-ai999x supports, generated from its certified capability contracts. Every example below was executed when this guide was built. The structured call `solve("<capability id>", ...)` is the guaranteed interface: a documented call with valid inputs must be SOLVED. Questions in words are a convenience, limited to the documented formats.

## Quick start

```python
import biomedical_science_ai999x

result = biomedical_science_ai999x.solve('bmed_pk_iv_bolus_concentration', D='0.0005 kg', V='0.04 m^3', k='2.777777777777778e-05 1/s', t='21600 s')
result.show()
# C = 0.00686014 kg/m^3, fraction_remaining = 0.548811
```

## By topic

| topic | capabilities |
|---|---|
| Biomechanics | [Static lever model of the forearm: muscle force and joint reaction](#bmed_elbow_lever_muscle_force), [Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance](#bmed_poiseuille_vessel_flow), [Two-element Windkessel: diastolic pressure decay](#bmed_windkessel_diastolic_decay), [Windkessel models (two- and three-element): input impedance at a frequency](#bmed_windkessel_input_impedance), [Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress](#bmed_kelvin_voigt_creep), [Standard linear solid (Zener) tissue model: creep under constant stress](#bmed_sls_creep), [Standard linear solid (Zener) tissue model: stress relaxation at constant strain](#bmed_sls_stress_relaxation), [Cardiac output by indicator dilution (Stewart-Hamilton)](#bmed_indicator_dilution_cardiac_output), [Single-compartment respiratory mechanics: resistance and compliance by least squares](#bmed_respiratory_mechanics_fit), [Body mass index (a calculation, not medical advice)](#bmed_bmi), [Basal metabolic rate by a stated regression (a calculation, not medical advice)](#bmed_bmr) |
| Diagnostic test statistics | [ROC curve area of a diagnostic score (Mann-Whitney form)](#bmed_dx_roc_auc), [Optimal diagnostic cut-off by the Youden index](#bmed_dx_youden_cutoff), [Sensitivity and specificity of a diagnostic score at a cut-off](#bmed_dx_threshold_sensitivity_specificity), [Sensitivity and specificity with Wilson score intervals](#bmed_dx_sensitivity_specificity_ci), [Cohen's kappa for agreement between two raters or two tests](#bmed_cohens_kappa), [Predictive values and likelihood ratios of a diagnostic test](#bmed_dx_predictive_values) |
| Dosage calculations | [Weight-based dose (dosage arithmetic, not medical advice)](#bmed_dose_weight_based), [Tablets to give: desired over have (dosage arithmetic, not medical advice)](#bmed_dose_tablets), [Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice)](#bmed_dose_liquid_volume), [Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice)](#bmed_reconstitution_concentration), [IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice)](#bmed_iv_flow_rate), [Infusion pump rate from a dose rate (arithmetic, not medical advice)](#bmed_iv_rate_from_dose_rate) |
| Epidemic models | [SIR epidemic model: final size (final susceptible fraction and attack fraction)](#bmed_epi_sir_final_size), [SEIR epidemic model: final size](#bmed_epi_seir_final_size), [SIR epidemic model: peak prevalence and the time of the peak](#bmed_epi_sir_peak), [SEIR epidemic model: initial exponential growth rate from R0](#bmed_epi_seir_growth_rate), [Reproduction number from an epidemic growth rate (gamma-distributed generation interval)](#bmed_epi_r_from_growth_rate), [SIS epidemic model: prevalence at a time and the endemic level](#bmed_epi_sis_prevalence), [Reed-Frost chain-binomial epidemic: distribution and mean of the final size](#bmed_epi_reed_frost_final_size), [SEIR epidemic model: peak prevalence and the time of the peak](#bmed_epi_seir_peak), [Herd-immunity threshold and critical vaccination coverage](#bmed_epi_herd_immunity) |
| Epidemiological measures of association | [Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval](#bmed_odds_ratio_ci), [Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval](#bmed_relative_risk_ci), [Absolute risk reduction and number needed to treat with Wald intervals](#bmed_nnt_ci), [Incidence rate ratio of two person-time groups with log-Wald interval](#bmed_incidence_rate_ratio_ci), [Standardized mortality (incidence) ratio with exact Poisson interval](#bmed_smr_exact_ci), [Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval](#bmed_mantel_haenszel_odds_ratio), [Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality](#bmed_epi_basic_rates), [Vaccine efficacy from attack rates](#bmed_vaccine_efficacy) |
| Medical-imaging physics | [X-ray attenuation through tissue layers (narrow monoenergetic beam)](#bmed_xray_attenuation_layers), [MRI spin-echo signal from T1, T2, TR and TE (simplified model)](#bmed_mri_spin_echo_signal), [MRI inversion recovery: longitudinal magnetisation at TI and the null point](#bmed_mri_inversion_recovery), [MRI spoiled gradient-echo signal (Ernst equation)](#bmed_mri_spoiled_gradient_echo), [Ultrasound reflection and transmission at a tissue interface (normal incidence)](#bmed_ultrasound_interface), [Ultrasound transmission through a matching layer (normal incidence)](#bmed_ultrasound_matching_layer), [Imaging resolution: MTF of a Gaussian point-spread function](#bmed_gaussian_psf_mtf), [Diffusion MRI: apparent diffusion coefficient from signals at several b-values](#bmed_mri_adc_fit), [MRI multi-echo T2 estimate (log-linear fit)](#bmed_mri_t2_fit), [Signal-to-noise ratio gained by signal averaging](#bmed_snr_averaging), [MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal)](#bmed_mri_ernst_angle), [CT number in Hounsfield units](#bmed_hounsfield_units), [Larmor (precession) frequency in a magnetic field](#bmed_larmor_frequency), [Ultrasound wavelength in tissue](#bmed_ultrasound_wavelength) |
| Neuron models | [Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl](#bmed_ghk_voltage), [Goldman-Hodgkin-Katz current equation: ionic current density through a membrane](#bmed_ghk_current_density), [Leaky integrate-and-fire neuron: firing rate under constant current](#bmed_lif_firing_rate), [Passive membrane (leaky integrator): voltage at a time under constant current](#bmed_lif_subthreshold_voltage), [Passive cable (dendrite or axon): steady-state voltage along it and input resistance](#bmed_cable_steady_state), [Hodgkin-Huxley model: steady-state gating variables at a membrane potential](#bmed_hh_gating_steady_state), [Hodgkin-Huxley model: zero-current steady (resting) membrane potential](#bmed_hh_resting_potential), [Hodgkin-Huxley model: potassium conductance after a voltage-clamp step](#bmed_hh_potassium_conductance_clamp), [Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current](#bmed_postsynaptic_potential), [FitzHugh-Nagumo neuron model: fixed point and its stability](#bmed_fitzhugh_nagumo_fixed_point) |
| Pharmacokinetic data analysis | [Elimination rate constant and half-life from concentration data (log-linear fit)](#bmed_pk_terminal_half_life_fit), [AUC from concentration-time data by the trapezoidal rule](#bmed_pk_auc_trapezoid), [Non-compartmental analysis of an IV bolus profile (AUC, lambda z, half-life, clearance, Vz)](#bmed_pk_nca_iv_bolus), [Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice)](#bmed_pk_bioavailability_from_profiles) |
| Pharmacokinetic models | [One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)](#bmed_pk_iv_bolus_concentration), [Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice)](#bmed_pk_auc_iv_bolus), [Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice)](#bmed_pk_oral_concentration), [Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice)](#bmed_pk_oral_peak), [Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice)](#bmed_pk_nth_dose_iv), [Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice)](#bmed_pk_steady_state_iv), [Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice)](#bmed_pk_steady_state_oral), [Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice)](#bmed_pk_infusion_concentration), [Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice)](#bmed_pk_short_infusion), [Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation)](#bmed_pk_loading_maintenance), [Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice)](#bmed_pk_intermittent_infusion_steady_state), [Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice)](#bmed_pk_time_to_steady_state_fraction), [Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice)](#bmed_pk_michaelis_menten_concentration), [Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice)](#bmed_pk_michaelis_menten_time), [Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice)](#bmed_pk_michaelis_menten_steady_state), [Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice)](#bmed_pk_two_compartment_bolus), [Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice)](#bmed_pk_two_compartment_micro_constants), [Hepatic clearance: well-stirred liver model (a model calculation, not medical advice)](#bmed_pk_hepatic_clearance_well_stirred), [Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice)](#bmed_pk_hepatic_clearance_parallel_tube), [Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice)](#bmed_pk_effect_site_concentration) |
| Survival analysis | [Kaplan-Meier survival estimate at a time and median survival](#bmed_km_survival_median), [Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval](#bmed_km_confidence_interval), [Nelson-Aalen cumulative hazard at a time](#bmed_nelson_aalen_hazard), [Log-rank test comparing two survival curves](#bmed_logrank_test), [Cox proportional-hazards model with one covariate: hazard ratio (Efron ties)](#bmed_cox_hazard_ratio), [Exponential survival model: hazard rate, mean and median survival (maximum likelihood)](#bmed_exponential_survival_fit), [Hazard ratio of two exponential survival groups with Wald interval](#bmed_exponential_hazard_ratio), [Restricted mean survival time from the Kaplan-Meier curve](#bmed_restricted_mean_survival_time) |

## Outcomes

| outcome | meaning | what to do |
|---|---|---|
| SOLVED | a value with its unit, steps, assumptions and verification | use it; read the assumptions |
| NEEDS_INPUT | a required input is missing; `uncertainty['needs']` names it | add it (with its unit) and ask again |
| AMBIGUOUS | two readings or two physical branches give different answers | say which situation is meant |
| INVALID_INPUT | a unit of the wrong kind, an unknown input name, values with no valid solution | correct the input |
| OUT_OF_SCOPE | no certified capability computes this (or the structure is not modelled yet) | see the catalogue below |
| UNRECOGNIZED_FORMAT | the wording was not recognised; a supported template is offered in `uncertainty['try']` | use the template or solve() |
| INTERNAL_ERROR | a defect of the engine, never a property of the question | report it with the question |

## Result fields

| field | meaning |
|---|---|
| value, unit | the result in SI units |
| values | every output of a procedure (x1, x2, discriminant ...) |
| outcome | one of the seven outcomes above |
| steps | each equation used, its inputs (with where each came from) and its result |
| assumptions | conditions and declared defaults the answer relies on |
| verification | residual of every equation, dimensional consistency, independent checks |
| latex() | the derivation as LaTeX |
| show() | renders in a notebook, prints in a terminal |

## Common mistakes

- Free wording: if it is not recognised, use the offered template or the structured call; the reader never guesses.
- A coefficient that violates a stated condition (a = 0 in a quadratic): INVALID_INPUT says why.
- A text value where a number is expected: INVALID_INPUT; fractions as strings ("1/2") are accepted.
- A problem outside the procedure (a cubic for the quadratic solver): OUT_OF_SCOPE, never an approximation.

## Procedures: algorithms with several outputs

Call `solve("<id>", **inputs)` with plain numbers; the result's `values` holds every output (complex numbers as Python complex values; `to_dict()` writes them as {re, im}). Each result is substituted back into its defining equation before it is returned.

| id | operation | inputs | outputs |
|---|---|---|---|
| bmed_pk_iv_bolus_concentration | One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) | D, V, k, t | C, fraction_remaining |
| bmed_pk_terminal_half_life_fit | Elimination rate constant and half-life from concentration data (log-linear fit) | t, C | k, t_half, C0 |
| bmed_pk_auc_iv_bolus | Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice) | D, CL, V, t | AUC_inf, AUC_0_t |
| bmed_pk_auc_trapezoid | AUC from concentration-time data by the trapezoidal rule | t, C, method | AUC |
| bmed_pk_nca_iv_bolus | Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz) | D, t, C, n_terminal | AUC_last, lambda_z, t_half, AUC_inf, CL, Vz |
| bmed_pk_oral_concentration | Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice) | F, D, V, ka, k, t | C, AUC_0_t |
| bmed_pk_oral_peak | Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice) | F, D, V, ka, k | t_max, C_max |
| bmed_pk_nth_dose_iv | Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice) | D, V, k, tau, n, t | C_max_n, C |
| bmed_pk_steady_state_iv | Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice) | D, V, k, tau | C_max_ss, C_min_ss, C_avg_ss, R_ac |
| bmed_pk_steady_state_oral | Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice) | F, D, V, ka, k, tau | C_max_ss, t_max_ss, C_min_ss |
| bmed_pk_infusion_concentration | Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice) | R0, CL, V, t | C, C_ss |
| bmed_pk_short_infusion | Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice) | R0, T_inf, CL, V, t | C, C_end |
| bmed_pk_loading_maintenance | Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation) | C_target, V, CL | LD, R0 |
| bmed_pk_intermittent_infusion_steady_state | Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice) | R0, T_inf, tau, CL, V | C_max_ss, C_min_ss |
| bmed_pk_time_to_steady_state_fraction | Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice) | CL, V, f | t |
| bmed_pk_michaelis_menten_concentration | Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice) | C0, Vmax, Km, V, t | C |
| bmed_pk_michaelis_menten_time | Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice) | C0, C, Vmax, Km, V | t |
| bmed_pk_michaelis_menten_steady_state | Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice) | R0, Vmax, Km | C_ss |
| bmed_pk_two_compartment_bolus | Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice) | D, V1, k10, k12, k21, t | alpha, beta, A, B, C |
| bmed_pk_two_compartment_micro_constants | Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice) | D, A, alpha, B, beta | V1, k10, k12, k21, CL, Vss |
| bmed_pk_bioavailability_from_profiles | Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice) | D_iv, t_iv, C_iv, D_oral, t_oral, C_oral | AUC_iv, AUC_oral, F |
| bmed_pk_hepatic_clearance_well_stirred | Hepatic clearance: well-stirred liver model (a model calculation, not medical advice) | Q, fu, CL_int | CL_H, E |
| bmed_pk_hepatic_clearance_parallel_tube | Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice) | Q, fu, CL_int | CL_H, E |
| bmed_pk_effect_site_concentration | Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice) | D, V, k, ke0, t | Ce, t_peak, Ce_peak |
| bmed_epi_sir_final_size | SIR epidemic model: final size (final susceptible fraction and attack fraction) | R0, s0, i0 | s_inf, attack |
| bmed_epi_seir_final_size | SEIR epidemic model: final size | R0, s0, e0, i0 | s_inf, attack |
| bmed_epi_sir_peak | SIR epidemic model: peak prevalence and the time of the peak | beta, gamma, s0, i0 | i_max, t_peak |
| bmed_epi_seir_growth_rate | SEIR epidemic model: initial exponential growth rate from R0 | R0, sigma, gamma | r |
| bmed_epi_r_from_growth_rate | Reproduction number from an epidemic growth rate (gamma-distributed generation interval) | r, Tg_mean, Tg_sd | R |
| bmed_epi_sis_prevalence | SIS epidemic model: prevalence at a time and the endemic level | beta, gamma, i0, t | i, i_star |
| bmed_epi_reed_frost_final_size | Reed-Frost chain-binomial epidemic: distribution and mean of the final size | n, m, p | mean_final, P_no_spread, distribution |
| bmed_dx_roc_auc | ROC curve area of a diagnostic score (Mann-Whitney form) | scores_pos, scores_neg | AUC, U |
| bmed_dx_youden_cutoff | Optimal diagnostic cut-off by the Youden index | scores_pos, scores_neg | threshold, sensitivity, specificity, J |
| bmed_dx_threshold_sensitivity_specificity | Sensitivity and specificity of a diagnostic score at a cut-off | scores_pos, scores_neg, threshold | sensitivity, specificity, TP, FN, TN, FP |
| bmed_dx_sensitivity_specificity_ci | Sensitivity and specificity with Wilson score intervals | TP, FN, TN, FP, level | sensitivity, sens_lower, sens_upper, specificity, spec_lower, spec_upper |
| bmed_odds_ratio_ci | Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval | a, b, c, d, level | OR, lower, upper |
| bmed_relative_risk_ci | Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval | a, b, c, d, level | RR, lower, upper |
| bmed_nnt_ci | Absolute risk reduction and number needed to treat with Wald intervals | events_t, n_t, events_c, n_c, level | ARR, ARR_lower, ARR_upper, NNT, NNT_lower, NNT_upper |
| bmed_incidence_rate_ratio_ci | Incidence rate ratio of two person-time groups with log-Wald interval | events1, time1, events2, time2, level | IRR, lower, upper |
| bmed_smr_exact_ci | Standardized mortality (incidence) ratio with exact Poisson interval | observed, expected, level | SMR, lower, upper |
| bmed_mantel_haenszel_odds_ratio | Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval | tables, level | OR_MH, lower, upper |
| bmed_cohens_kappa | Cohen's kappa for agreement between two raters or two tests | counts | kappa, p_observed |
| bmed_km_survival_median | Kaplan-Meier survival estimate at a time and median survival | times, events, t | S_t, median |
| bmed_km_confidence_interval | Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval | times, events, t, level | S_t, lower, upper |
| bmed_nelson_aalen_hazard | Nelson-Aalen cumulative hazard at a time | times, events, t | H_t |
| bmed_logrank_test | Log-rank test comparing two survival curves | times1, events1, times2, events2 | chi_square, p_value |
| bmed_cox_hazard_ratio | Cox proportional-hazards model with one covariate: hazard ratio (Efron ties) | times, events, x, level | beta, HR, se, HR_lower, HR_upper |
| bmed_exponential_survival_fit | Exponential survival model: hazard rate, mean and median survival (maximum likelihood) | times, events | hazard, mean, median |
| bmed_exponential_hazard_ratio | Hazard ratio of two exponential survival groups with Wald interval | times1, events1, times2, events2, level | HR, lower, upper |
| bmed_restricted_mean_survival_time | Restricted mean survival time from the Kaplan-Meier curve | times, events, tau | RMST |
| bmed_elbow_lever_muscle_force | Static lever model of the forearm: muscle force and joint reaction | W, d_load, W_arm, d_arm, d_muscle, theta | F_muscle, R_x, R_y, R |
| bmed_poiseuille_vessel_flow | Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance | dP, r, L, mu | Q, v_mean, tau_wall, R_hyd |
| bmed_windkessel_diastolic_decay | Two-element Windkessel: diastolic pressure decay | P0, Pv, R, C, t | P |
| bmed_windkessel_input_impedance | Windkessel models (two- and three-element): input impedance at a frequency | model, R, C, f, Zc | Z_mod, phase_deg |
| bmed_kelvin_voigt_creep | Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress | sigma, E, eta, t | strain |
| bmed_sls_creep | Standard linear solid (Zener) tissue model: creep under constant stress | sigma, E_inf, E1, eta, t | strain, strain_inf |
| bmed_sls_stress_relaxation | Standard linear solid (Zener) tissue model: stress relaxation at constant strain | eps0, E_inf, E1, eta, t | stress, stress_inf |
| bmed_indicator_dilution_cardiac_output | Cardiac output by indicator dilution (Stewart-Hamilton) | m, t, C | AUC, CO |
| bmed_respiratory_mechanics_fit | Single-compartment respiratory mechanics: resistance and compliance by least squares | P, Q, V | R, C, P0 |
| bmed_ghk_voltage | Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl | T, PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i | V_m |
| bmed_ghk_current_density | Goldman-Hodgkin-Katz current equation: ionic current density through a membrane | P, z, C_in, C_out, V, T | I |
| bmed_lif_firing_rate | Leaky integrate-and-fire neuron: firing rate under constant current | tau_m, R_m, I, V_rest, V_th, V_reset, t_ref | rate, ISI |
| bmed_lif_subthreshold_voltage | Passive membrane (leaky integrator): voltage at a time under constant current | tau_m, R_m, I, V_rest, V0, t | V |
| bmed_cable_steady_state | Passive cable (dendrite or axon): steady-state voltage along it and input resistance | Rm, Ri, d, x, V0, end, L | V_x, R_in |
| bmed_hh_gating_steady_state | Hodgkin-Huxley model: steady-state gating variables at a membrane potential | V | m_inf, h_inf, n_inf |
| bmed_hh_resting_potential | Hodgkin-Huxley model: zero-current steady (resting) membrane potential | gNa, gK, gL, ENa, EK, EL | V_rest |
| bmed_hh_potassium_conductance_clamp | Hodgkin-Huxley model: potassium conductance after a voltage-clamp step | gK_max, V_hold, V_step, t | g_K, n |
| bmed_postsynaptic_potential | Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current | tau_m, tau_s, R_m, I0, t | V_t, t_peak, V_peak |
| bmed_fitzhugh_nagumo_fixed_point | FitzHugh-Nagumo neuron model: fixed point and its stability | a, b, eps, I | v_star, w_star, trace, determinant, stable |
| bmed_xray_attenuation_layers | X-ray attenuation through tissue layers (narrow monoenergetic beam) | I0, mu_rho, rho, x | T, I |
| bmed_mri_spin_echo_signal | MRI spin-echo signal from T1, T2, TR and TE (simplified model) | M0, TR, TE, T1, T2 | S |
| bmed_mri_inversion_recovery | MRI inversion recovery: longitudinal magnetisation at TI and the null point | T1, TR, TI | Mz_ratio, TI_null |
| bmed_mri_spoiled_gradient_echo | MRI spoiled gradient-echo signal (Ernst equation) | M0, TR, TE, T1, T2star, alpha | S |
| bmed_ultrasound_interface | Ultrasound reflection and transmission at a tissue interface (normal incidence) | Z1, Z2 | R, T, r_p, t_p |
| bmed_ultrasound_matching_layer | Ultrasound transmission through a matching layer (normal incidence) | Z1, Zm, Z2, d, c_m, f | T, kd |
| bmed_gaussian_psf_mtf | Imaging resolution: MTF of a Gaussian point-spread function | fwhm, f | MTF, f50 |
| bmed_mri_adc_fit | Diffusion MRI: apparent diffusion coefficient from signals at several b-values | b, S | ADC, S0 |
| bmed_mri_t2_fit | MRI multi-echo T2 estimate (log-linear fit) | TE, S | T2, S0 |
| bmed_snr_averaging | Signal-to-noise ratio gained by signal averaging | SNR1, N, SNR_target | SNR, gain, N |
| bmed_mri_ernst_angle | MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal) | TR, T1 | alpha, alpha_deg, S_rel |
| bmed_epi_seir_peak | SEIR epidemic model: peak prevalence and the time of the peak | beta, sigma, gamma, s0, e0, i0 | t_peak, i_max, s_at_peak, e_at_peak |
| bmed_bmi | Body mass index (a calculation, not medical advice) | m, h | BMI, BMI_prime |
| bmed_bmr | Basal metabolic rate by a stated regression (a calculation, not medical advice) | m, h, age, sex, equation | BMR_kcal_day, BMR_W, BMR_MJ_day |
| bmed_hounsfield_units | CT number in Hounsfield units | mu, mu_water, mu_air | HU, mu_rel |
| bmed_larmor_frequency | Larmor (precession) frequency in a magnetic field | B, gamma_bar | f, f_MHz, omega |
| bmed_ultrasound_wavelength | Ultrasound wavelength in tissue | f, c | wavelength, wavelength_mm, period, k |
| bmed_epi_basic_rates | Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality | cases, population, prevalent, deaths, period | incidence_proportion, incidence_per_100000, incidence_rate, incidence_per_100000_person_years, prevalence, prevalence_per_100000, case_fatality, mortality_per_100000, mortality_per_100000_person_years |
| bmed_dx_predictive_values | Predictive values and likelihood ratios of a diagnostic test | sensitivity, specificity, prevalence | PPV, NPV, LR_pos, LR_neg, DOR |
| bmed_epi_herd_immunity | Herd-immunity threshold and critical vaccination coverage | R0, efficacy | threshold, threshold_percent, coverage, coverage_percent, reachable |
| bmed_vaccine_efficacy | Vaccine efficacy from attack rates | cases_v, n_v, cases_u, n_u | ARV, ARU, RR, VE, VE_percent, ARR, NNV |
| bmed_dose_weight_based | Weight-based dose (dosage arithmetic, not medical advice) | dose_mg_per_kg, W, basis, doses_per_day | dose, dose_mg, daily_dose, daily_dose_mg |
| bmed_dose_tablets | Tablets to give: desired over have (dosage arithmetic, not medical advice) | D, H | tablets |
| bmed_dose_liquid_volume | Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice) | D, H, Q | V, V_mL, C |
| bmed_reconstitution_concentration | Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice) | powder, diluent, displacement, D | V_total, V_total_mL, C, C_mg_mL, V_give, V_give_mL |
| bmed_iv_flow_rate | IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice) | V, T, rate, drop_factor | flow, flow_mL_h, duration, duration_h, duration_min, drops_per_min |
| bmed_iv_rate_from_dose_rate | Infusion pump rate from a dose rate (arithmetic, not medical advice) | drug, bag_volume, dose_rate, dose_ug_per_kg_min, W, drop_factor | C, dose_rate, dose_mg_h, flow, flow_mL_h, drops_per_min |

<a id="bmed_pk_iv_bolus_concentration"></a>

### One-compartment IV bolus: concentration at a time (a model calculation, not medical advice), `bmed_pk_iv_bolus_concentration`

The plasma concentration of a one-compartment model a time t after an intravenous bolus dose, with first-order elimination; also the fraction of the dose remaining.

$$
C(t) = \frac{D}{V}\,e^{-k t}
$$

| input | meaning | condition |
|---|---|---|
| `D` | bolus dose |  |
| `V` | volume of distribution |  |
| `k` | elimination rate constant |  |
| `t` | time after the dose |  |

| output | meaning |
|---|---|
| `C` | concentration at time t |
| `fraction_remaining` | fraction of the dose still in the body |

- **Method:** C(t) = (D/V) exp(-k t)
- **Assumes:** one compartment, instantaneous mixing; first-order elimination; $t  \geq  0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![One-compartment intravenous bolus: plasma concentration over 48 h (log axis, so each line is straight) for D = 500 mg, V = 40 L and three elimination rate constants (the given k = 0](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-pk-iv-bolus-concentration-pharmacokinetics.svg)

*One-compartment intravenous bolus: plasma concentration over 48 h (log axis, so each line is straight) for D = 500 mg, V = 40 L and three elimination rate constants (the given k = 0.1 1/h, half and double), 97 runs each. Marked: t = 6 h, C = 6.86 mg/L, fraction of the dose remaining 0.5488. In SI units: C = 0.00686 kg/m^3. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h.*

![Concentration at time t as V runs from 24 to 56 L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-iv-bolus-concentration-live-1)

*Concentration at time t as V runs from 24 to 56 L, every other input as you entered it (41 solutions). Marked: your V = 40 L gives C = 6.86 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h.*

![Concentration at time t as D runs from 300 to 700 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-iv-bolus-concentration-live-2)

*Concentration at time t as D runs from 300 to 700 mg, every other input as you entered it (29 solutions). Marked: your D = 500 mg gives C = 6.86 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_iv_bolus_concentration', D=0.0005, V=0.04, k=2.777777777777778e-05, t=21600.0)
# -> C = 0.00686015 kg/m^3, fraction_remaining = 0.548812   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_iv_bolus_concentration', D=0.001, V=0.05, k=6.944444444444444e-05, t=5400.0)
# -> C = 0.0137458 kg/m^3, fraction_remaining = 0.687289   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h | SOLVED | C = 0.00686015 kg/m^3, fraction_remaining = 0.548812 |
| IV bolus concentration: D = 1 g, V = 50 L, k = 0.25 1/h, t = 90 min | SOLVED | C = 0.0137458 kg/m^3, fraction_remaining = 0.687289 |
| IV bolus concentration: V = 40 L, k = 0.1 1/h, t = 6 h | NEEDS_INPUT | bmed_pk_iv_bolus_concentration needs D (bolus dose). Example: solve('bmed_pk_iv_bolus_concentration', D='0.0005 kg', V=' |
| IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| IV bolus concentration: D = 0.5 g, V = 40000 mL, k = 0.1 1/h, t = 360 min | SOLVED | C = 0.00686015 kg/m^3, fraction_remaining = 0.548812 |
| IV bolus concentration: D = 500 mg, V = 40 L, k = -0.1 1/h, t = 6 h | INVALID_INPUT | One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): the dose D, the volume V an |
| IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_terminal_half_life_fit"></a>

### Elimination rate constant and half-life from concentration data (log-linear fit), `bmed_pk_terminal_half_life_fit`

The first-order elimination rate constant, half-life and back-extrapolated concentration from a straight-line least-squares fit of ln C against time (mono-exponential decline).

$$
\ln C = \ln C_0 - k t,\quad t_{1/2} = \frac{\ln 2}{k}
$$

| input | meaning | condition |
|---|---|---|
| `t` | sampling times, in one time unit (results are per that unit) |  |
| `C` | concentrations at those times, in one concentration unit (C0 is in that unit) |  |

| output | meaning |
|---|---|
| `k` | elimination rate constant, per time unit of t |
| `t_half` | half-life, in the time unit of t |
| `C0` | back-extrapolated concentration at $t = 0$, in the unit of C |

- **Method:** least-squares line ln $C = ln$ $C_{0} - k$ t; t_half = ln $2 / k$
- **Assumes:** mono-exponential decline over the sampled times; all concentrations positive; unweighted least squares on ln C; at least three points
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Log-linear fit of the 6 concentration samples given (log axis)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-terminal-half-life-fit-fit)

*Log-linear fit of the 6 concentration samples given (log axis). The dashed line is C0 exp(-k t) drawn from the returned k = 0.1748 per time unit and C0 = 9.735 (derived). Marked: the half-life t1/2 = 3.965 where the line reaches C0/2; dot: the back-extrapolated C0. Your inputs: t = 1, 2, 4, 6, 8, 12, C = 8.2, 6.9, 4.8, 3.4, 2.4, 1.2.*

```python
biomedical_science_ai999x.solve('bmed_pk_terminal_half_life_fit', t=[1, 2, 4, 6, 8, 12], C=[8.2, 6.9, 4.8, 3.4, 2.4, 1.2])
# -> k = 0.174805, t_half = 3.96526, C0 = 9.73505   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_terminal_half_life_fit', t=[0.5, 1, 3, 5], C=[95, 80, 41, 22])
# -> k = 0.325426, t_half = 2.12997, C0 = 110.832   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Terminal half-life fit: t = [1, 2, 4, 6, 8, 12], C = [8.2, 6.9, 4.8, 3.4, 2.4, 1.2] | SOLVED | k = 0.174805, t_half = 3.96526, C0 = 9.73505 |
| Terminal half-life fit: t = [0.5, 1, 3, 5], C = [95, 80, 41, 22] | SOLVED | k = 0.325426, t_half = 2.12997, C0 = 110.832 |
| Terminal half-life fit: C = [8.2, 6.9, 4.8, 3.4, 2.4, 1.2] | NEEDS_INPUT | bmed_pk_terminal_half_life_fit needs t (sampling times, in one time unit (results are per that unit)). Example: solve('b |
| Terminal half-life fit: t = [1, 2, 4, 6, 8, 12], C = [8.2, 6.9, 4.8, 3.4, 2.4, 1.2], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Terminal half-life fit: t = [1, 2, 4], C = [8.2, 9.1, 10.5] | INVALID_INPUT | Elimination rate constant and half-life from concentration data (log-linear fit): the concentrations do not decline (the |
| Terminal half-life fit: t = [1, 2, 4, 6], C = [8.2, 6.9, 4.8, 3.4] mg/L | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_auc_iv_bolus"></a>

### Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice), `bmed_pk_auc_iv_bolus`

The area under the plasma concentration-time curve from 0 to infinity and from 0 to t after an IV bolus in a one-compartment model.

$$
AUC_{0-\infty} = \frac{D}{CL},\quad AUC_{0-t} = \frac{D}{CL}\left(1 - e^{-CL\,t/V}\right)
$$

| input | meaning | condition |
|---|---|---|
| `D` | bolus dose |  |
| `CL` | clearance |  |
| `V` | volume of distribution |  |
| `t` | end of the partial area |  |

| output | meaning |
|---|---|
| `AUC_inf` | area from 0 to infinity |
| `AUC_0_t` | area from 0 to t |

- **Method:** AUC(0-inf) = D/CL; AUC(0-t) = (D/CL)(1 - exp(-CL t / V))
- **Assumes:** one compartment; first-order elimination $k = CL/V$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Concentration after an intravenous bolus (D = 500 mg, CL = 5 L/h, V = 40 L): each point is the model's bolus concentration with k = CL/V = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-auc-iv-bolus-pharmacokinetics)

*Concentration after an intravenous bolus (D = 500 mg, CL = 5 L/h, V = 40 L): each point is the model's bolus concentration with k = CL/V = 0.125 1/h, 91 runs over 24 h. Shaded: the area from 0 to t = 8 h, AUC = 63.21 mg h/L, of the area to infinity 100 mg h/L. In SI units: AUC_inf = 360 kg*s/m^3. Your inputs: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h.*

![Area from 0 to infinity and area from 0 to t as D runs from 300 to 700 mg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-auc-iv-bolus-live-1)

*Area from 0 to infinity and area from 0 to t as D runs from 300 to 700 mg, every other input as you entered it (41 solutions). Marked: your D = 500 mg gives AUC_inf = 360. Your inputs: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h.*

![Area from 0 to infinity and area from 0 to t as CL runs from 3 to 7 L/h, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-auc-iv-bolus-live-2)

*Area from 0 to infinity and area from 0 to t as CL runs from 3 to 7 L/h, every other input as you entered it (29 solutions). Marked: your CL = 5 L/h gives AUC_inf = 360. Your inputs: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_auc_iv_bolus', D=0.0005, CL=1.3888888888888887e-06, V=0.04, t=28800.0)
# -> AUC_inf = 360 kg*s/m^3, AUC_0_t = 227.563 kg*s/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_auc_iv_bolus', D=9.999999999999999e-05, CL=2e-06, V=0.03, t=86400.0)
# -> AUC_inf = 50 kg*s/m^3, AUC_0_t = 49.8424 kg*s/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| AUC of an IV bolus: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h | SOLVED | AUC_inf = 360 kg*s/m^3, AUC_0_t = 227.563 kg*s/m^3 |
| AUC of an IV bolus: D = 100 mg, CL = 120 mL/min, V = 30 L, t = 1 day | SOLVED | AUC_inf = 50 kg*s/m^3, AUC_0_t = 49.8424 kg*s/m^3 |
| AUC of an IV bolus: CL = 5 L/h, V = 40 L, t = 8 h | NEEDS_INPUT | bmed_pk_auc_iv_bolus needs D (bolus dose). Example: solve('bmed_pk_auc_iv_bolus', D='0.0005 kg', CL='1.3888888888888887e |
| AUC of an IV bolus: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| AUC of an IV bolus: D = 500 mg, CL = 0 L/h, V = 40 L, t = 8 h | INVALID_INPUT | Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice): the dose D,  |

<a id="bmed_pk_auc_trapezoid"></a>

### AUC from concentration-time data by the trapezoidal rule, `bmed_pk_auc_trapezoid`

The area under a sampled concentration-time curve from the first to the last sample, by the linear trapezoidal rule or the linear-up/log-down rule (stated).

$$
AUC = \sum_i \frac{(C_i + C_{i+1})\,\Delta t_i}{2}\quad\text{or}\quad \frac{(C_i - C_{i+1})\,\Delta t_i}{\ln(C_i/C_{i+1})}
$$

| input | meaning | condition |
|---|---|---|
| `t` | sampling times, strictly increasing, in one time unit |  |
| `C` | concentrations at those times, in one concentration unit |  |
| `method` | linear trapezoids, or linear while rising and logarithmic while falling |  |

| output | meaning |
|---|---|
| `AUC` | area from the first to the last sample, in (unit of C) x (unit of t) |

- **Method:** sum of segment areas: linear $(C_{1} + C_{2})$ $dt / 2$; log-down $(C_{1} - C_{2})$ dt / ln(C1/C2)
- **Assumes:** times strictly increasing; concentrations >= 0; log-down segments need both concentrations positive (otherwise linear)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The 8 concentration samples given and the 7 trapezoids between them; the model's AUC by the linear method is 51](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-auc-trapezoid-trapezoids)

*The 8 concentration samples given and the 7 trapezoids between them; the model's AUC by the linear method is 51.85 (with the method linear up log down the model gives AUC = 51.25). Your inputs: t = 0, 0.5, 1, 2, 4, 6, 8, 12, C = 0, 4.2, 6.8, 7.9, 6.1..., method = linear.*

![Area from the first to the last sample, in](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-auc-trapezoid-live-1)

*Area from the first to the last sample, in... for every choice of method, every other input as you entered it (highlighted: your choice). linear: 51.85; linear_up_log_down: 51.25. Your inputs: t = 0, 0.5, 1, 2, 4, 6, 8, 12, C = 0, 4.2, 6.8, 7.9, 6.1..., method = linear.*

```python
biomedical_science_ai999x.solve('bmed_pk_auc_trapezoid', t=[0, 0.5, 1, 2, 4, 6, 8, 12], C=[0, 4.2, 6.8, 7.9, 6.1, 4.3, 3.0, 1.5], method='linear')
# -> AUC = 51.85   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_auc_trapezoid', t=[0, 0.5, 1, 2, 4, 6, 8, 12], C=[0, 4.2, 6.8, 7.9, 6.1, 4.3, 3.0, 1.5], method='linear_up_log_down')
# -> AUC = 51.2462   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| AUC by the trapezoidal rule: t = [0, 0.5, 1, 2, 4, 6, 8, 12], C = [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5], method = linear | SOLVED | AUC = 51.85 |
| AUC by the trapezoidal rule: t = [0, 0.5, 1, 2, 4, 6, 8, 12], C = [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5], method = linear_up_log_down | SOLVED | AUC = 51.2462 |
| AUC by the trapezoidal rule: C = [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5], method = linear | NEEDS_INPUT | bmed_pk_auc_trapezoid needs t (sampling times, strictly increasing, in one time unit). Example: solve('bmed_pk_auc_trape |
| AUC by the trapezoidal rule: t = [0, 0.5, 1, 2, 4, 6, 8, 12], C = [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5], method = linear, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| AUC by the trapezoidal rule: t = [0, 1, 2], C = [0, 4, 3] | NEEDS_INPUT | bmed_pk_auc_trapezoid needs method (linear trapezoids, or linear while rising and logarithmic while falling). Example: s |
| AUC by the trapezoidal rule: t = [0, 2, 1], C = [0, 4, 3], method = linear | INVALID_INPUT | AUC from concentration-time data by the trapezoidal rule: the times must increase strictly. |

<a id="bmed_pk_nca_iv_bolus"></a>

### Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz), `bmed_pk_nca_iv_bolus`

Non-compartmental parameters from a concentration-time profile after an IV bolus: AUC to the last sample (linear trapezoids), terminal rate constant from the last n points, half-life, AUC to infinity, clearance and terminal volume.

$$
AUC_{0-\infty} = AUC_{0-t_{last}} + \frac{C_{last}}{\lambda_z},\quad CL = \frac{D}{AUC_{0-\infty}},\quad V_z = \frac{CL}{\lambda_z}
$$

| input | meaning | condition |
|---|---|---|
| `D` | IV bolus dose |  |
| `t` | sampling times in h, the first at 0 (measured or back-extrapolated C0) |  |
| `C` | concentrations in mg/L at those times |  |
| `n_terminal` | number of last points used for the terminal slope (3 to n - 1) |  |

| output | meaning |
|---|---|
| `AUC_last` | AUC from 0 to the last sample, $mg h/L$ |
| `lambda_z` | terminal rate constant, $1/h$ |
| `t_half` | terminal half-life, h |
| `AUC_inf` | AUC from 0 to infinity (observed C_last / lambda_z added), $mg h/L$ |
| `CL` | clearance D / AUC_inf, L/h |
| `Vz` | terminal volume CL / lambda_z, L |

- **Method:** AUC by trapezoids; ln $C = a - lambda_z$ t on the last points; AUC_inf = AUC_last + C_last/lambda_z; CL = D/AUC_inf; $Vz = CL/lambda_z$
- **Assumes:** linear trapezoidal rule; lambda_z by unweighted least squares on ln C of the last n points; extrapolation with the observed last concentration; times in h, concentrations in mg/L
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Non-compartmental analysis of the 10 samples given after an intravenous bolus (log axis)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nca-iv-bolus-pharmacokinetics)

*Non-compartmental analysis of the 10 samples given after an intravenous bolus (log axis). The dashed line has the model's terminal slope lambda_z = 0.1202 from the last 4 samples, drawn through the last sample (derived). Returned: t1/2 = 5.768, AUC_last = 18.48, AUC_inf = 19.41, CL = 5.151, Vz = 42.86. Your inputs: D = 9.999999999999999e-05 kg, t = 0, 0.25, 0.5, 1, 2, 4..., C = 5, 4, 3.3, 2.44, 1.72..., n_terminal = 4.*

![CL as D runs from 6e-05 to 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nca-iv-bolus-live-1)

*CL as D runs from 6e-05 to 0.00014 kg, every other input as you entered it (41 solutions). Marked: your D = 0.0001 kg gives CL = 5.151. Your inputs: D = 9.999999999999999e-05 kg, t = 0, 0.25, 0.5, 1, 2, 4..., C = 5, 4, 3.3, 2.44, 1.72..., n_terminal = 4.*

![CL as n_terminal runs from 3 to 8, every other input as you entered it (6 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nca-iv-bolus-live-2)

*CL as n_terminal runs from 3 to 8, every other input as you entered it (6 solutions). Marked: your n_terminal = 4 gives CL = 5.151. Your inputs: D = 9.999999999999999e-05 kg, t = 0, 0.25, 0.5, 1, 2, 4..., C = 5, 4, 3.3, 2.44, 1.72..., n_terminal = 4.*

```python
biomedical_science_ai999x.solve('bmed_pk_nca_iv_bolus', D=9.999999999999999e-05, t=[0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24], C=[5.0, 4.0, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112], n_terminal=4)
# -> AUC_last = 18.4825, lambda_z = 0.120166, t_half = 5.76825, AUC_inf = 19.4145, CL = 5.15078, Vz = 42.8639   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_nca_iv_bolus', D=0.00025, t=[0, 1, 2, 4, 8, 12], C=[12.5, 9.8, 7.9, 5.1, 2.2, 0.95], n_terminal=3)
# -> AUC_last = 53.9, lambda_z = 0.210067, t_half = 3.29965, AUC_inf = 58.4224, CL = 4.27918, Vz = 20.3706   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Non-compartmental analysis of an IV bolus: D = 100 mg, t = [0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24], C = [5, 4, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112], n_terminal = 4 | SOLVED | AUC_last = 18.4825, lambda_z = 0.120166, t_half = 5.76825, AUC_inf = 19.4145, CL = 5.15078, Vz = 42.8639 |
| Non-compartmental analysis of an IV bolus: D = 250 mg, t = [0, 1, 2, 4, 8, 12], C = [12.5, 9.8, 7.9, 5.1, 2.2, 0.95], n_terminal = 3 | SOLVED | AUC_last = 53.9, lambda_z = 0.210067, t_half = 3.29965, AUC_inf = 58.4224, CL = 4.27918, Vz = 20.3706 |
| Non-compartmental analysis of an IV bolus: t = [0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24], C = [5, 4, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112], n_terminal = 4 | NEEDS_INPUT | bmed_pk_nca_iv_bolus needs D (IV bolus dose). Example: solve('bmed_pk_nca_iv_bolus', D='9.999999999999999e-05 kg', t=[0, |
| Non-compartmental analysis of an IV bolus: D = 100 mg, t = [0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24], C = [5, 4, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112], n_terminal = 4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Non-compartmental analysis of an IV bolus: D = 100 mg, t = [1, 2, 4, 8, 12], C = [4, 3, 2, 1, 0.5], n_terminal = 3 | INVALID_INPUT | Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz): the first sample must be at |

<a id="bmed_pk_oral_concentration"></a>

### Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice), `bmed_pk_oral_concentration`

The plasma concentration and partial AUC of a one-compartment model with first-order absorption (rate ka) and elimination (rate k) after a single oral dose with bioavailability F.

$$
C(t) = \frac{F D k_a}{V (k_a - k)}\left(e^{-k t} - e^{-k_a t}\right)
$$

| input | meaning | condition |
|---|---|---|
| `F` | bioavailable fraction (0 < F <= 1; 80% or 0.8) |  |
| `D` | oral dose |  |
| `V` | volume of distribution |  |
| `ka` | absorption rate constant |  |
| `k` | elimination rate constant |  |
| `t` | time after the dose |  |

| output | meaning |
|---|---|
| `C` | concentration at time t |
| `AUC_0_t` | area under the curve from 0 to t |

- **Method:** Bateman function $C = F$ D ka / (V (ka - k)) (exp(-k t) - exp(-ka t)) and its integral
- **Assumes:** one compartment; first-order absorption and elimination; $ka  \neq  k$; no lag time
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Plasma concentration after one oral dose (D = 250 mg, F = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-concentration-pharmacokinetics)

*Plasma concentration after one oral dose (D = 250 mg, F = 0.8, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h), computed by the model at 120 times over 24 h; dot: the peak the model returns, t_max = 1.98 h, C_max = 4.246 mg/L. Marked: t = 3 h, C = 3.986 mg/L. In SI units: C = 0.003986 kg/m^3. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h.*

![Concentration at time t as t runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-concentration-live-1)

*Concentration at time t as t runs from 1.8 to 4.2 h, every other input as you entered it (41 solutions). Marked: your t = 3 h gives C = 3.986 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h.*

![Concentration at time t as F runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-concentration-live-2)

*Concentration at time t as F runs from 0.2 to 0.9714, every other input as you entered it (19 solutions). Marked: your F = 0.8 gives C = 3.986 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_oral_concentration', F=0.8, D=0.00025, V=0.035, ka=0.0003333333333333333, k=4.1666666666666665e-05, t=10800.0)
# -> C = 0.00398566 kg/m^3, AUC_0_t = 37.7397 kg*s/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_oral_concentration', F=0.6, D=4.9999999999999996e-05, V=0.1, ka=1.388888888888889e-05, k=5.555555555555556e-05, t=43200.0)
# -> C = 4.58094e-05 kg/m^3, AUC_0_t = 1.61185 kg*s/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Oral dose concentration: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h | SOLVED | C = 0.00398566 kg/m^3, AUC_0_t = 37.7397 kg*s/m^3 |
| Oral dose concentration: F = 60%, D = 50 mg, V = 100 L, ka = 0.05 1/h, k = 0.2 1/h, t = 12 h | SOLVED | C = 4.58094e-05 kg/m^3, AUC_0_t = 1.61185 kg*s/m^3 |
| Oral dose concentration: D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h | NEEDS_INPUT | bmed_pk_oral_concentration needs F (bioavailable fraction (0 < F <= 1; 80% or 0.8)). Example: solve('bmed_pk_oral_concen |
| Oral dose concentration: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Oral dose concentration: F = 80, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Oral dose concentration: F = 0.8, D = 250 mg, V = 35 L, ka = 0.15 1/h, k = 0.15 1/h, t = 3 h | INVALID_INPUT | Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice): |

<a id="bmed_pk_oral_peak"></a>

### Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice), `bmed_pk_oral_peak`

The time of the peak plasma concentration and the peak concentration of a one-compartment model with first-order absorption after a single oral dose.

$$
t_{max} = \frac{\ln(k_a/k)}{k_a - k},\quad C_{max} = \frac{F D}{V} e^{-k t_{max}}
$$

| input | meaning | condition |
|---|---|---|
| `F` | bioavailable fraction $(0 < F  \leq  1)$ |  |
| `D` | oral dose |  |
| `V` | volume of distribution |  |
| `ka` | absorption rate constant |  |
| `k` | elimination rate constant |  |

| output | meaning |
|---|---|
| `t_max` | time of the peak |
| `C_max` | peak concentration |

- **Method:** $dC/dt = 0$: t_max = ln(ka/k)/(ka - k); C_max = (F D / V) exp(-k t_max)
- **Assumes:** one compartment; first-order absorption and elimination; $ka  \neq  k$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Oral dose (F = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-peak-pharmacokinetics)

*Oral dose (F = 0.8, D = 250 mg, V = 35 L, k = 0.15 1/h) for three absorption rate constants (the given ka, three times and a third of it): concentration curves (81 runs each of the model's oral concentration) with the peak the model returns on each: ka = 1.2 1/h: t_max 1.98 h, C_max 4.246 mg/L; ka = 3.6 1/h: t_max 0.9212 h, C_max 4.977 mg/L; ka = 0.4 1/h: t_max 3.923 h, C_max 3.172 mg/L. Faster absorption gives an earlier, higher peak. In SI units: t_max = 7130 s. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h.*

![Peak concentration as V runs from 21 to 49 L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-peak-live-1)

*Peak concentration as V runs from 21 to 49 L, every other input as you entered it (41 solutions). Marked: your V = 35 L gives C_max = 4.246 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h.*

![Peak concentration as F runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-oral-peak-live-2)

*Peak concentration as F runs from 0.2 to 0.9714, every other input as you entered it (19 solutions). Marked: your F = 0.8 gives C_max = 4.246 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h.*

```python
biomedical_science_ai999x.solve('bmed_pk_oral_peak', F=0.8, D=0.00025, V=0.035, ka=0.0003333333333333333, k=4.1666666666666665e-05)
# -> t_max = 7129.51 s, C_max = 0.0042457 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_oral_peak', F=1, D=0.0005, V=0.06, ka=0.00025, k=2.4166666666666663e-05)
# -> t_max = 10346.1 s, C_max = 0.00648981 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Oral dose peak: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h | SOLVED | t_max = 7129.51 s, C_max = 0.0042457 kg/m^3 |
| Oral dose peak: F = 1, D = 500 mg, V = 60 L, ka = 0.9 1/h, k = 0.087 1/h | SOLVED | t_max = 10346.1 s, C_max = 0.00648981 kg/m^3 |
| Oral dose peak: D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h | NEEDS_INPUT | bmed_pk_oral_peak needs F (bioavailable fraction (0 < F <= 1)). Example: solve('bmed_pk_oral_peak', F=0.8, D='0.00025 kg |
| Oral dose peak: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_nth_dose_iv"></a>

### Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice), `bmed_pk_nth_dose_iv`

The peak concentration just after the nth of equal IV bolus doses given every tau, and the concentration a time t later, in a one-compartment model (superposition).

$$
C_n(t) = \frac{D}{V}\,\frac{1 - e^{-n k \tau}}{1 - e^{-k \tau}}\,e^{-k t}
$$

| input | meaning | condition |
|---|---|---|
| `D` | each bolus dose |  |
| `V` | volume of distribution |  |
| `k` | elimination rate constant |  |
| `tau` | dosing interval |  |
| `n` | number of the dose (1 = first) |  |
| `t` | time after the nth dose (0 to tau) |  |

| output | meaning |
|---|---|
| `C_max_n` | concentration just after the nth dose |
| `C` | concentration a time t after the nth dose |

- **Method:** superposition: $C = (D/V)$ $(1 - r^{n})/(1 - r)$ exp(-k t), $r = exp$(-k tau)
- **Assumes:** equal doses at equal intervals; linear one-compartment model; $0  \leq  t  \leq  \tau$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![5 intravenous doses of 500 mg every 8 h (V = 40 L, k = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nth-dose-iv-pharmacokinetics)

*5 intravenous doses of 500 mg every 8 h (V = 40 L, k = 0.1 1/h): the concentration the model computes at 21 times in each interval rises towards its steady state. Marked: 2 h after dose 3, C = 16.9 mg/L. In SI units: C = 0.0169 kg/m^3. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h.*

![Concentration a time t after the nth dose as D runs from 300 to 700 mg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nth-dose-iv-live-1)

*Concentration a time t after the nth dose as D runs from 300 to 700 mg, every other input as you entered it (41 solutions). Marked: your D = 500 mg gives C = 16.9 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h.*

![Concentration a time t after the nth dose as V runs from 24 to 56 L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-nth-dose-iv-live-2)

*Concentration a time t after the nth dose as V runs from 24 to 56 L, every other input as you entered it (29 solutions). Marked: your V = 40 L gives C = 16.9 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_nth_dose_iv', D=0.0005, V=0.04, k=2.777777777777778e-05, tau=28800.0, n=3, t=7200.0)
# -> C_max_n = 0.0206403 kg/m^3, C = 0.0168989 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_nth_dose_iv', D=0.001, V=0.06, k=5.555555555555556e-05, tau=43200.0, n=1, t=0.0)
# -> C_max_n = 0.0166667 kg/m^3, C = 0.0166667 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h | SOLVED | C_max_n = 0.0206403 kg/m^3, C = 0.0168989 kg/m^3 |
| Concentration after the nth IV dose: D = 1 g, V = 60 L, k = 0.2 1/h, tau = 12 h, n = 1, t = 0 h | SOLVED | C_max_n = 0.0166667 kg/m^3, C = 0.0166667 kg/m^3 |
| Concentration after the nth IV dose: V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h | NEEDS_INPUT | bmed_pk_nth_dose_iv needs D (each bolus dose). Example: solve('bmed_pk_nth_dose_iv', D='0.0005 kg', V='0.04 m^3', k='2.7 |
| Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 2 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 9 h | INVALID_INPUT | Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice): t is the time after |
| Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 2.5, t = 2 h | INVALID_INPUT | n: expected a whole number written exactly, got '2.5'. |

<a id="bmed_pk_steady_state_iv"></a>

### Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice), `bmed_pk_steady_state_iv`

The steady-state peak, trough and average concentrations and the accumulation factor of a one-compartment model given equal IV bolus doses every tau.

$$
C_{max,ss} = \frac{D/V}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k\tau},\quad \bar C_{ss} = \frac{D}{V k \tau}
$$

| input | meaning | condition |
|---|---|---|
| `D` | each bolus dose |  |
| `V` | volume of distribution |  |
| `k` | elimination rate constant |  |
| `tau` | dosing interval |  |

| output | meaning |
|---|---|
| `C_max_ss` | steady-state peak (just after a dose) |
| `C_min_ss` | steady-state trough (just before a dose) |
| `C_avg_ss` | average concentration over an interval at steady state |
| `R_ac` | accumulation factor: steady-state peak over first-dose peak |

- **Method:** geometric series: C_max,ss = (D/V)/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k tau); C_avg = D/(V k tau)
- **Assumes:** equal doses at equal intervals; linear one-compartment model
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Steady-state peak (just after a dose) as k runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-steady-state-iv-live-1)

*Steady-state peak (just after a dose) as k runs from 0.06 to 0.14 1/h, every other input as you entered it (41 solutions). Marked: your k = 0.1 1/h gives C_max_ss = 22.7 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h.*

![Steady-state peak (just after a dose) as τ runs from 4](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-steady-state-iv-live-2)

*Steady-state peak (just after a dose) as τ runs from 4.8 to 11.2 h, every other input as you entered it (29 solutions). Marked: your τ = 8 h gives C_max_ss = 22.7 mg/L. Your inputs: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_steady_state_iv', D=0.0005, V=0.04, k=2.777777777777778e-05, tau=28800.0)
# -> C_max_ss = 0.0226996 kg/m^3, C_min_ss = 0.0101996 kg/m^3, C_avg_ss = 0.015625 kg/m^3, R_ac = 1.81597   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_steady_state_iv', D=0.00025, V=0.025, k=1.6055555555555554e-05, tau=43200.0)
# -> C_max_ss = 0.0199909 kg/m^3, C_min_ss = 0.00999095 kg/m^3, C_avg_ss = 0.0144175 kg/m^3, R_ac = 1.99909   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Steady state of repeated IV doses: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h | SOLVED | C_max_ss = 0.0226996 kg/m^3, C_min_ss = 0.0101996 kg/m^3, C_avg_ss = 0.015625 kg/m^3, R_ac = 1.81597 |
| Steady state of repeated IV doses: D = 250 mg, V = 25 L, k = 0.0578 1/h, tau = 12 h | SOLVED | C_max_ss = 0.0199909 kg/m^3, C_min_ss = 0.00999095 kg/m^3, C_avg_ss = 0.0144175 kg/m^3, R_ac = 1.99909 |
| Steady state of repeated IV doses: V = 40 L, k = 0.1 1/h, tau = 8 h | NEEDS_INPUT | bmed_pk_steady_state_iv needs D (each bolus dose). Example: solve('bmed_pk_steady_state_iv', D='0.0005 kg', V='0.04 m^3' |
| Steady state of repeated IV doses: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_steady_state_oral"></a>

### Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice), `bmed_pk_steady_state_oral`

The steady-state peak concentration, the time of the peak after a dose and the trough of a one-compartment model with first-order absorption given equal oral doses every tau.

$$
C_{ss}(t) = \frac{F D k_a}{V (k_a - k)}\left[\frac{e^{-k t}}{1 - e^{-k\tau}} - \frac{e^{-k_a t}}{1 - e^{-k_a \tau}}\right]
$$

| input | meaning | condition |
|---|---|---|
| `F` | bioavailable fraction $(0 < F  \leq  1)$ |  |
| `D` | each oral dose |  |
| `V` | volume of distribution |  |
| `ka` | absorption rate constant |  |
| `k` | elimination rate constant |  |
| `tau` | dosing interval |  |

| output | meaning |
|---|---|
| `C_max_ss` | steady-state peak |
| `t_max_ss` | time of the peak after a dose |
| `C_min_ss` | steady-state trough (at the dosing times) |

- **Method:** superposition to steady state: C_ss(t) = F D ka/(V(ka - k)) [exp(-k $t)/(1 - exp$(-k tau)) - exp(-ka $t)/(1 - exp$(-ka tau))]; $dC/dt = 0$ for the peak
- **Assumes:** equal doses at equal intervals; linear one-compartment model, first-order absorption; $ka  \neq  k$; the steady-state peak lies inside the interval
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Steady-state peak and steady-state trough as τ runs from 4](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-steady-state-oral-live-1)

*Steady-state peak and steady-state trough as τ runs from 4.8 to 11.2 h, every other input as you entered it (41 solutions). Marked: your τ = 8 h gives C_max_ss = 6.395 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, tau = 8 h.*

![Steady-state peak and steady-state trough as k runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-steady-state-oral-live-2)

*Steady-state peak and steady-state trough as k runs from 0.09 to 0.21 1/h, every other input as you entered it (29 solutions). Marked: your k = 0.15 1/h gives C_max_ss = 6.395 mg/L. Your inputs: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, tau = 8 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_steady_state_oral', F=0.8, D=0.00025, V=0.035, ka=0.0003333333333333333, k=4.1666666666666665e-05, tau=28800.0)
# -> C_max_ss = 0.00639474 kg/m^3, t_max_ss = 5901.01 s, C_min_ss = 0.00281433 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_steady_state_oral', F=0.5, D=9.999999999999999e-05, V=0.07, ka=0.0005555555555555556, k=2.777777777777778e-05, tau=86400.0)
# -> C_max_ss = 0.000674329 kg/m^3, t_max_ss = 5495.93 s, C_min_ss = 7.50141e-05 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Steady state of repeated oral doses: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, tau = 8 h | SOLVED | C_max_ss = 0.00639474 kg/m^3, t_max_ss = 5901.01 s, C_min_ss = 0.00281433 kg/m^3 |
| Steady state of repeated oral doses: F = 0.5, D = 100 mg, V = 70 L, ka = 2 1/h, k = 0.1 1/h, tau = 24 h | SOLVED | C_max_ss = 0.000674329 kg/m^3, t_max_ss = 5495.93 s, C_min_ss = 7.50141e-05 kg/m^3 |
| Steady state of repeated oral doses: D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, tau = 8 h | NEEDS_INPUT | bmed_pk_steady_state_oral needs F (bioavailable fraction (0 < F <= 1)). Example: solve('bmed_pk_steady_state_oral', F=0. |
| Steady state of repeated oral doses: F = 0.8, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, tau = 8 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_infusion_concentration"></a>

### Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice), `bmed_pk_infusion_concentration`

The plasma concentration of a one-compartment model during a constant-rate intravenous infusion started at $t = 0$, and the steady-state concentration it approaches.

$$
C(t) = \frac{R_0}{CL}\left(1 - e^{-CL\,t/V}\right),\quad C_{ss} = \frac{R_0}{CL}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | infusion rate |  |
| `CL` | clearance |  |
| `V` | volume of distribution |  |
| `t` | time since the start of the infusion |  |

| output | meaning |
|---|---|
| `C` | concentration at time t |
| `C_ss` | steady-state concentration $R_{0}/CL$ |

- **Method:** $C = (R_{0}/CL)$(1 - exp(-CL t / V))
- **Assumes:** one compartment, first-order elimination; infusion still running at t; no drug at $t = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Constant-rate infusion (R0 = 50 mg/h, CL = 5 L/h): the concentration rises towards the steady state R0/CL = 10 mg/L, sooner for a smaller volume of distribution (half, the given and twice V, 81 runs each)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-infusion-concentration-pharmacokinetics)

*Constant-rate infusion (R0 = 50 mg/h, CL = 5 L/h): the concentration rises towards the steady state R0/CL = 10 mg/L, sooner for a smaller volume of distribution (half, the given and twice V, 81 runs each). Marked: t = 6 h, C = 5.276 mg/L. In SI units: C = 0.005276 kg/m^3. Your inputs: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 h.*

![Concentration at time t and steady-state concentration R0/CL as CL runs from 3 to 7 L/h, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-infusion-concentration-live-1)

*Concentration at time t and steady-state concentration R0/CL as CL runs from 3 to 7 L/h, every other input as you entered it (41 solutions). Marked: your CL = 5 L/h gives C = 5.276 mg/L. Your inputs: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 h.*

![Concentration at time t and steady-state concentration R0/CL as R0 runs from 30 to 70 mg/h, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-infusion-concentration-live-2)

*Concentration at time t and steady-state concentration R0/CL as R0 runs from 30 to 70 mg/h, every other input as you entered it (29 solutions). Marked: your R0 = 50 mg/h gives C = 5.276 mg/L. Your inputs: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_infusion_concentration', R0=1.3888888888888889e-08, CL=1.3888888888888887e-06, V=0.04, t=21600.0)
# -> C = 0.00527633 kg/m^3, C_ss = 0.01 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_infusion_concentration', R0=3.3333333333333334e-08, CL=1.6666666666666667e-06, V=0.02, t=86400.0)
# -> C = 0.0199851 kg/m^3, C_ss = 0.02 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Infusion concentration: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 h | SOLVED | C = 0.00527633 kg/m^3, C_ss = 0.01 kg/m^3 |
| Infusion concentration: R0 = 2 mg/min, CL = 100 mL/min, V = 20 L, t = 1 day | SOLVED | C = 0.0199851 kg/m^3, C_ss = 0.02 kg/m^3 |
| Infusion concentration: CL = 5 L/h, V = 40 L, t = 6 h | NEEDS_INPUT | bmed_pk_infusion_concentration needs R0 (infusion rate). Example: solve('bmed_pk_infusion_concentration', R0='1.38888888 |
| Infusion concentration: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_short_infusion"></a>

### Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice), `bmed_pk_short_infusion`

The plasma concentration of a one-compartment model at time t after the start of a constant-rate infusion lasting T_inf (during or after the infusion), and the concentration at the end of the infusion.

$$
C(t) = \begin{cases}\frac{R_0}{CL}(1 - e^{-kt}) & t \le T\\ \frac{R_0}{CL}(1 - e^{-kT})\,e^{-k(t - T)} & t > T\end{cases}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | infusion rate |  |
| `T_inf` | duration of the infusion |  |
| `CL` | clearance |  |
| `V` | volume of distribution |  |
| `t` | time since the start of the infusion |  |

| output | meaning |
|---|---|
| `C` | concentration at time t |
| `C_end` | concentration at the end of the infusion |

- **Method:** during: $(R_{0}/CL)$(1 - exp(-k t)); after: C_end exp(-k (t - T_inf)), $k = CL/V$
- **Assumes:** one compartment, first-order elimination; single infusion, no drug before it
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![A short infusion of R0 = 200 mg/h for T_inf = 1 h (CL = 6 L/h, V = 30 L, shaded: the infusion): concentration over 12 h, 97 runs](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-short-infusion-pharmacokinetics)

*A short infusion of R0 = 200 mg/h for T_inf = 1 h (CL = 6 L/h, V = 30 L, shaded: the infusion): concentration over 12 h, 97 runs. Dot: at the end of the infusion C_end = 6.042 mg/L; marked: t = 4 h, C = 3.316 mg/L. In SI units: C = 0.003316 kg/m^3. Your inputs: R0 = 200 mg/h, T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h.*

![Concentration at time t as T_inf runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-short-infusion-live-1)

*Concentration at time t as T_inf runs from 0.6 to 1.4 h, every other input as you entered it (41 solutions). Marked: your T_inf = 1 h gives C = 3.316 mg/L. Your inputs: R0 = 200 mg/h, T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h.*

![Concentration at time t as R0 runs from 120 to 280 mg/h, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-short-infusion-live-2)

*Concentration at time t as R0 runs from 120 to 280 mg/h, every other input as you entered it (29 solutions). Marked: your R0 = 200 mg/h gives C = 3.316 mg/L. Your inputs: R0 = 200 mg/h, T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_short_infusion', R0=5.5555555555555555e-08, T_inf=3600.0, CL=1.6666666666666667e-06, V=0.03, t=14400.0)
# -> C = 0.00331609 kg/m^3, C_end = 0.00604231 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_short_infusion', R0=2.7777777777777777e-08, T_inf=7200.0, CL=1.111111111111111e-06, V=0.05, t=5400.0)
# -> C = 0.00282699 kg/m^3, C_end = 0.00369641 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Short infusion concentration: R0 = 200 mg/h, T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h | SOLVED | C = 0.00331609 kg/m^3, C_end = 0.00604231 kg/m^3 |
| Short infusion concentration: R0 = 100 mg/h, T_inf = 2 h, CL = 4 L/h, V = 50 L, t = 1.5 h | SOLVED | C = 0.00282699 kg/m^3, C_end = 0.00369641 kg/m^3 |
| Short infusion concentration: T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h | NEEDS_INPUT | bmed_pk_short_infusion needs R0 (infusion rate). Example: solve('bmed_pk_short_infusion', R0='5.5555555555555555e-08 kg/ |
| Short infusion concentration: R0 = 200 mg/h, T_inf = 1 h, CL = 6 L/h, V = 30 L, t = 4 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_loading_maintenance"></a>

### Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation), `bmed_pk_loading_maintenance`

For a one-compartment model, the bolus and the constant infusion rate that together hold the model concentration at a target value from the start (model calculation only, not a dosing recommendation).

$$
LD = C_{target} V,\quad R_0 = C_{target}\,CL
$$

| input | meaning | condition |
|---|---|---|
| `C_target` | target model concentration |  |
| `V` | volume of distribution |  |
| `CL` | clearance |  |

| output | meaning |
|---|---|
| `LD` | loading bolus of the model |
| `R0` | maintenance infusion rate of the model |

- **Method:** $LD = C$ V fills the volume; $R_{0} = C$ CL replaces what is cleared
- **Assumes:** one compartment, first-order elimination; bolus and infusion start together
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Loading dose and maintenance infusion for a target of 15 mg/L (V = 50 L, CL = 4 L/h): the model returns LD = 750 mg and R0 = 60 mg/h](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-loading-maintenance-pharmacokinetics)

*Loading dose and maintenance infusion for a target of 15 mg/L (V = 50 L, CL = 4 L/h): the model returns LD = 750 mg and R0 = 60 mg/h. Drawn: the model's bolus concentration of LD (k = CL/V), its infusion concentration at R0, 61 runs each, and their sum (superposition, derived). In SI units: LD = 0.00075 kg. Your inputs: C_target = 15 mg/L, V = 50 L, CL = 4 L/h.*

![Loading bolus of the model as C_target runs from 9 to 21 mg/L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-loading-maintenance-live-1)

*Loading bolus of the model as C_target runs from 9 to 21 mg/L, every other input as you entered it (41 solutions). Marked: your C_target = 15 mg/L gives LD = 0.75 g. Your inputs: C_target = 15 mg/L, V = 50 L, CL = 4 L/h.*

![Maintenance infusion rate of the model as CL runs from 2](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-loading-maintenance-live-2)

*Maintenance infusion rate of the model as CL runs from 2.4 to 5.6 L/h, every other input as you entered it (29 solutions). Marked: your CL = 4 L/h gives R0 = 1.667e-08. Your inputs: C_target = 15 mg/L, V = 50 L, CL = 4 L/h.*

```python
biomedical_science_ai999x.solve('bmed_pk_loading_maintenance', C_target=0.015, V=0.05, CL=1.111111111111111e-06)
# -> LD = 0.00075 kg, R0 = 1.66667e-08 kg/s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_loading_maintenance', C_target=0.002, V=0.35000000000000003, CL=8.333333333333334e-06)
# -> LD = 0.0007 kg, R0 = 1.66667e-08 kg/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Loading dose and maintenance rate model: C_target = 15 mg/L, V = 50 L, CL = 4 L/h | SOLVED | LD = 0.00075 kg, R0 = 1.66667e-08 kg/s |
| Loading dose and maintenance rate model: C_target = 2 mg/L, V = 350 L, CL = 0.5 L/min | SOLVED | LD = 0.0007 kg, R0 = 1.66667e-08 kg/s |
| Loading dose and maintenance rate model: V = 50 L, CL = 4 L/h | NEEDS_INPUT | bmed_pk_loading_maintenance needs C_target (target model concentration). Example: solve('bmed_pk_loading_maintenance', C |
| Loading dose and maintenance rate model: C_target = 15 mg/L, V = 50 L, CL = 4 L/h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_intermittent_infusion_steady_state"></a>

### Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice), `bmed_pk_intermittent_infusion_steady_state`

The steady-state peak (end of infusion) and trough (end of interval) of a one-compartment model given infusions of duration T_inf at rate R0 every tau.

$$
C_{max,ss} = \frac{R_0}{CL}\,\frac{1 - e^{-kT}}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k(\tau - T)}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | infusion rate |  |
| `T_inf` | duration of each infusion |  |
| `tau` | dosing interval |  |
| `CL` | clearance |  |
| `V` | volume of distribution |  |

| output | meaning |
|---|---|
| `C_max_ss` | steady-state concentration at the end of an infusion |
| `C_min_ss` | steady-state concentration at the end of an interval |

- **Method:** C_max,$ss = (R_{0}/CL)$(1 - exp(-k T))/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k(tau - T)), $k = CL/V$
- **Assumes:** one compartment, first-order elimination; T_inf < tau; identical infusions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Steady-state concentration at the end of an](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-intermittent-infusion-steady-state-live-1)

*Steady-state concentration at the end of an... as τ runs from 4.8 to 11.2 h, every other input as you entered it (41 solutions). Marked: your τ = 8 h gives C_max_ss = 37.85 mg/L. Your inputs: R0 = 1000 mg/h, T_inf = 1 h, tau = 8 h, CL = 6 L/h, V = 30 L.*

![Steady-state concentration at the end of an](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-intermittent-infusion-steady-state-live-2)

*Steady-state concentration at the end of an... as CL runs from 3.6 to 8.4 L/h, every other input as you entered it (29 solutions). Marked: your CL = 6 L/h gives C_max_ss = 37.85 mg/L. Your inputs: R0 = 1000 mg/h, T_inf = 1 h, tau = 8 h, CL = 6 L/h, V = 30 L.*

```python
biomedical_science_ai999x.solve('bmed_pk_intermittent_infusion_steady_state', R0=2.7777777777777776e-07, T_inf=3600.0, tau=28800.0, CL=1.6666666666666667e-06, V=0.03)
# -> C_max_ss = 0.0378542 kg/m^3, C_min_ss = 0.00933472 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_intermittent_infusion_steady_state', R0=1.3888888888888888e-07, T_inf=1800.0, tau=43200.0, CL=1.3888888888888887e-06, V=0.02)
# -> C_max_ss = 0.012366 kg/m^3, C_min_ss = 0.000697641 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Intermittent infusion steady state: R0 = 1000 mg/h, T_inf = 1 h, tau = 8 h, CL = 6 L/h, V = 30 L | SOLVED | C_max_ss = 0.0378542 kg/m^3, C_min_ss = 0.00933472 kg/m^3 |
| Intermittent infusion steady state: R0 = 500 mg/h, T_inf = 30 min, tau = 12 h, CL = 5 L/h, V = 20 L | SOLVED | C_max_ss = 0.012366 kg/m^3, C_min_ss = 0.000697641 kg/m^3 |
| Intermittent infusion steady state: T_inf = 1 h, tau = 8 h, CL = 6 L/h, V = 30 L | NEEDS_INPUT | bmed_pk_intermittent_infusion_steady_state needs R0 (infusion rate). Example: solve('bmed_pk_intermittent_infusion_stead |
| Intermittent infusion steady state: R0 = 1000 mg/h, T_inf = 1 h, tau = 8 h, CL = 6 L/h, V = 30 L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Intermittent infusion steady state: R0 = 1000 mg/h, T_inf = 9 h, tau = 8 h, CL = 6 L/h, V = 30 L | INVALID_INPUT | Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice): the infusion must be s |

<a id="bmed_pk_time_to_steady_state_fraction"></a>

### Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice), `bmed_pk_time_to_steady_state_fraction`

The time after the start of a constant-rate infusion at which a one-compartment model reaches a stated fraction of its steady-state concentration.

$$
t_f = -\frac{\ln(1 - f)}{CL/V}
$$

| input | meaning | condition |
|---|---|---|
| `CL` | clearance |  |
| `V` | volume of distribution |  |
| `f` | fraction of steady state (90% or 0.9) |  |

| output | meaning |
|---|---|
| `t` | time to reach f of steady state |

- **Method:** $1 - exp$(-k t) = f: $t = -ln$(1 - f)/k, $k = CL/V$
- **Assumes:** one compartment, first-order elimination; no drug at the start; $0 < f < 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Time to reach f of steady state as f runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-time-to-steady-state-fraction-live-1)

*Time to reach f of steady state as f runs from 0.125 to 0.875, every other input as you entered it (41 solutions). Marked: your f = 0.5 gives t = 1.444 day. Your inputs: CL = 2 L/h, V = 100 L, f = 0.5.*

![Time to reach f of steady state as V runs from 60 to 140 L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-time-to-steady-state-fraction-live-2)

*Time to reach f of steady state as V runs from 60 to 140 L, every other input as you entered it (29 solutions). Marked: your V = 100 L gives t = 1.444 day. Your inputs: CL = 2 L/h, V = 100 L, f = 0.5.*

```python
biomedical_science_ai999x.solve('bmed_pk_time_to_steady_state_fraction', CL=1.3888888888888887e-06, V=0.04, f=0.9)
# -> t = 66314.5 s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_time_to_steady_state_fraction', CL=5.555555555555555e-07, V=0.1, f=0.5)
# -> t = 124766 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Time to fraction of steady state: CL = 5 L/h, V = 40 L, f = 90% | SOLVED | t = 66314.5 s |
| Time to fraction of steady state: CL = 2 L/h, V = 100 L, f = 0.5 | SOLVED | t = 124766 s |
| Time to fraction of steady state: V = 40 L, f = 90% | NEEDS_INPUT | bmed_pk_time_to_steady_state_fraction needs CL (clearance). Example: solve('bmed_pk_time_to_steady_state_fraction', CL=' |
| Time to fraction of steady state: CL = 5 L/h, V = 40 L, f = 90%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Time to fraction of steady state: CL = 5 L/h, V = 40 L, f = 100% | INVALID_INPUT | Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice): the fraction f mu |

<a id="bmed_pk_michaelis_menten_concentration"></a>

### Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice), `bmed_pk_michaelis_menten_concentration`

The concentration a time t after an IV bolus in a one-compartment model with saturable Michaelis-Menten elimination, from the closed form with the Lambert W function.

$$
C(t) = K_m\,W\!\left(\frac{C_0}{K_m}\exp\frac{C_0 - V_{max} t / V}{K_m}\right)
$$

| input | meaning | condition |
|---|---|---|
| `C0` | initial concentration |  |
| `Vmax` | maximum elimination rate (amount per time) |  |
| `Km` | Michaelis constant |  |
| `V` | volume of distribution |  |
| `t` | time after the dose |  |

| output | meaning |
|---|---|
| `C` | concentration at time t |

- **Method:** V dC/dt = -Vmax C/(Km + C) integrates to $C = Km$ W($(C_{0}/Km)$ exp((C0 - Vmax t/V)/Km)); W found from w + ln $w = L$ by Newton steps
- **Assumes:** one compartment; elimination rate Vmax C/(Km + C); no other elimination
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Saturable (Michaelis-Menten) elimination: concentration over 144 h for Vmax = 20](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-concentration-pharmacokinetics)

*Saturable (Michaelis-Menten) elimination: concentration over 144 h for Vmax = 20.83 mg/h, Km = 4 mg/L, V = 50 L and three starting concentrations (half, the given and twice C0), 73 runs each. Far above Km the fall is nearly linear, below Km it becomes exponential. Marked: t = 48 h, C = 5.307 mg/L. In SI units: C = 0.005307 kg/m^3. Your inputs: C0 = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day.*

![Concentration at time t as V runs from 30 to 70 L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-concentration-live-1)

*Concentration at time t as V runs from 30 to 70 L, every other input as you entered it (41 solutions). Marked: your V = 50 L gives C = 5.307 mg/L. Your inputs: C0 = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day.*

![Concentration at time t as C0 runs from 12 to 28 mg/L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-concentration-live-2)

*Concentration at time t as C0 runs from 12 to 28 mg/L, every other input as you entered it (29 solutions). Marked: your C0 = 20 mg/L gives C = 5.307 mg/L. Your inputs: C0 = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day.*

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_concentration', C0=0.02, Vmax=5.787037037037037e-09, Km=0.004, V=0.05, t=172800.0)
# -> C = 0.0053069 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_concentration', C0=0.005, Vmax=2.7777777777777776e-09, Km=0.01, V=0.02, t=28800.0)
# -> C = 0.0037847 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Michaelis-Menten concentration: C0 = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day | SOLVED | C = 0.0053069 kg/m^3 |
| Michaelis-Menten concentration: C0 = 5 mg/L, Vmax = 10 mg/h, Km = 10 mg/L, V = 20 L, t = 8 h | SOLVED | C = 0.0037847 kg/m^3 |
| Michaelis-Menten concentration: Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day | NEEDS_INPUT | bmed_pk_michaelis_menten_concentration needs C0 (initial concentration). Example: solve('bmed_pk_michaelis_menten_concen |
| Michaelis-Menten concentration: C0 = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, t = 2 day, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_michaelis_menten_time"></a>

### Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice), `bmed_pk_michaelis_menten_time`

The time for the concentration of a one-compartment model with Michaelis-Menten elimination to fall from C0 to C.

$$
t = \frac{V}{V_{max}}\left[(C_0 - C) + K_m \ln\frac{C_0}{C}\right]
$$

| input | meaning | condition |
|---|---|---|
| `C0` | initial concentration |  |
| `C` | target concentration (< C0) |  |
| `Vmax` | maximum elimination rate |  |
| `Km` | Michaelis constant |  |
| `V` | volume of distribution |  |

| output | meaning |
|---|---|
| `t` | time to fall from C0 to C |

- **Method:** $t = V$ [$(C_{0} - C) + Km$ ln(C0/C)] / Vmax
- **Assumes:** one compartment; Michaelis-Menten elimination only; $C < C_{0}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Time for the concentration to fall from C0 = 20 mg/L to each lower value under saturable elimination (Vmax = 20](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-time-pharmacokinetics)

*Time for the concentration to fall from C0 = 20 mg/L to each lower value under saturable elimination (Vmax = 20.83 mg/h, Km = 4 mg/L, V = 50 L), 60 runs of the model, drawn as concentration against time. Marked: the target 10 mg/L is reached after t = 30.65 h. In SI units: t = 1.104e+05 s. Your inputs: C0 = 20 mg/L, C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L.*

![Time to fall from C0 to C as C0 runs from 12 to 28 mg/L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-time-live-1)

*Time to fall from C0 to C as C0 runs from 12 to 28 mg/L, every other input as you entered it (41 solutions). Marked: your C0 = 20 mg/L gives t = 1.277 day. Your inputs: C0 = 20 mg/L, C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L.*

![Time to fall from C0 to C as C runs from 6 to 14 mg/L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-time-live-2)

*Time to fall from C0 to C as C runs from 6 to 14 mg/L, every other input as you entered it (29 solutions). Marked: your C = 10 mg/L gives t = 1.277 day. Your inputs: C0 = 20 mg/L, C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L.*

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_time', C0=0.02, C=0.01, Vmax=5.787037037037037e-09, Km=0.004, V=0.05)
# -> t = 110355 s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_time', C0=0.03, C=0.003, Vmax=5.555555555555555e-09, Km=0.005, V=0.04)
# -> t = 277293 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Michaelis-Menten time to concentration: C0 = 20 mg/L, C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L | SOLVED | t = 110355 s |
| Michaelis-Menten time to concentration: C0 = 30 mg/L, C = 3 mg/L, Vmax = 20 mg/h, Km = 5 mg/L, V = 40 L | SOLVED | t = 277293 s |
| Michaelis-Menten time to concentration: C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L | NEEDS_INPUT | bmed_pk_michaelis_menten_time needs C0 (initial concentration). Example: solve('bmed_pk_michaelis_menten_time', C0='0.02 |
| Michaelis-Menten time to concentration: C0 = 20 mg/L, C = 10 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Michaelis-Menten time to concentration: C0 = 10 mg/L, C = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L | INVALID_INPUT | Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice): the target conc |

<a id="bmed_pk_michaelis_menten_steady_state"></a>

### Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice), `bmed_pk_michaelis_menten_steady_state`

The steady-state concentration of a one-compartment model with Michaelis-Menten elimination given a constant dosing rate below Vmax.

$$
C_{ss} = \frac{K_m R_0}{V_{max} - R_0}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | constant dosing rate (amount per time) |  |
| `Vmax` | maximum elimination rate |  |
| `Km` | Michaelis constant |  |

| output | meaning |
|---|---|
| `C_ss` | steady-state concentration |

- **Method:** R0 = Vmax C/(Km + C): $C_ss = Km$ R0/(Vmax - R0)
- **Assumes:** Michaelis-Menten elimination only; R0 < Vmax (otherwise no steady state)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Steady-state concentration as R0 runs from 180 to 420 mg/day, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-steady-state-live-1)

*Steady-state concentration as R0 runs from 180 to 420 mg/day, every other input as you entered it (41 solutions). Marked: your R0 = 300 mg/day gives C_ss = 6 mg/L. Your inputs: R0 = 300 mg/day, Vmax = 500 mg/day, Km = 4 mg/L.*

![Steady-state concentration as Vmax runs from 314](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-michaelis-menten-steady-state-live-2)

*Steady-state concentration as Vmax runs from 314.3 to 700 mg/day, every other input as you entered it (28 solutions). Marked: your Vmax = 500 mg/day gives C_ss = 6 mg/L. Your inputs: R0 = 300 mg/day, Vmax = 500 mg/day, Km = 4 mg/L.*

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_steady_state', R0=3.472222222222222e-09, Vmax=5.787037037037037e-09, Km=0.004)
# -> C_ss = 0.006 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_michaelis_menten_steady_state', R0=4.166666666666667e-09, Vmax=5.555555555555555e-09, Km=0.005)
# -> C_ss = 0.015 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Michaelis-Menten steady state: R0 = 300 mg/day, Vmax = 500 mg/day, Km = 4 mg/L | SOLVED | C_ss = 0.006 kg/m^3 |
| Michaelis-Menten steady state: R0 = 15 mg/h, Vmax = 20 mg/h, Km = 5 mg/L | SOLVED | C_ss = 0.015 kg/m^3 |
| Michaelis-Menten steady state: Vmax = 500 mg/day, Km = 4 mg/L | NEEDS_INPUT | bmed_pk_michaelis_menten_steady_state needs R0 (constant dosing rate (amount per time)). Example: solve('bmed_pk_michael |
| Michaelis-Menten steady state: R0 = 300 mg/day, Vmax = 500 mg/day, Km = 4 mg/L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Michaelis-Menten steady state: R0 = 600 mg/day, Vmax = 500 mg/day, Km = 4 mg/L | INVALID_INPUT | Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical  |

<a id="bmed_pk_two_compartment_bolus"></a>

### Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice), `bmed_pk_two_compartment_bolus`

The hybrid rate constants alpha and beta, the coefficients A and B and the central concentration at time t of a two-compartment model after an IV bolus, from the volume V1 and the micro-constants k10, k12, k21.

$$
C(t) = A e^{-\alpha t} + B e^{-\beta t},\quad \alpha\beta = k_{10}k_{21},\ \alpha + \beta = k_{10} + k_{12} + k_{21}
$$

| input | meaning | condition |
|---|---|---|
| `D` | bolus dose |  |
| `V1` | central volume |  |
| `k10` | elimination rate constant from the central compartment |  |
| `k12` | transfer rate constant central to peripheral |  |
| `k21` | transfer rate constant peripheral to central |  |
| `t` | time after the dose |  |

| output | meaning |
|---|---|
| `alpha` | fast (distribution) rate constant |
| `beta` | slow (terminal) rate constant |
| `A` | coefficient of exp(-alpha t) |
| `B` | coefficient of exp(-beta t) |
| `C` | central concentration at time t |

- **Method:** alpha, beta roots of $x^{2} - (k10 + k12 + k21)$ $x + k10$ $k21 = 0$; $A = (D/V_{1})$$(\alpha - k21)/(\alpha - \beta)$, $B = (D/V_{1})$$(k21 - \beta)/(\alpha - \beta)$
- **Assumes:** linear two-compartment model, elimination from the central compartment; $\alpha > \beta$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Two-compartment intravenous bolus (D = 100 mg, V1 = 10 L, k10 = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-bolus-pharmacokinetics)

*Two-compartment intravenous bolus (D = 100 mg, V1 = 10 L, k10 = 0.2, k12 = 0.5, k21 = 0.3 1/h): central concentration over 24 h (log axis, 97 runs) and its two exponential phases drawn from the returned A = 7.294 mg/L, alpha = 0.9359 1/h and B = 2.706 mg/L, beta = 0.06411 1/h (derived). Marked: t = 4 h, C = 2.266 mg/L. In SI units: C = 0.002266 kg/m^3. Your inputs: D = 100 mg, V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h.*

![Central concentration at time t as V1 runs from 6 to 14 L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-bolus-live-1)

*Central concentration at time t as V1 runs from 6 to 14 L, every other input as you entered it (41 solutions). Marked: your V1 = 10 L gives C = 2.266 mg/L. Your inputs: D = 100 mg, V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h.*

![Central concentration at time t as D runs from 60 to 140 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-bolus-live-2)

*Central concentration at time t as D runs from 60 to 140 mg, every other input as you entered it (29 solutions). Marked: your D = 100 mg gives C = 2.266 mg/L. Your inputs: D = 100 mg, V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h.*

```python
biomedical_science_ai999x.solve('bmed_pk_two_compartment_bolus', D=9.999999999999999e-05, V1=0.01, k10=5.555555555555556e-05, k12=0.0001388888888888889, k21=8.333333333333333e-05, t=14400.0)
# -> alpha = 0.000259969 1/s, beta = 1.78084e-05 1/s, A = 0.00729416 kg/m^3, B = 0.00270584 kg/m^3, C = 0.00226643 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_two_compartment_bolus', D=0.0005, V1=0.02, k10=2.777777777777778e-05, k12=0.0003333333333333333, k21=0.00011111111111111112, t=1800.0)
# -> alpha = 0.000465593 1/s, beta = 6.629e-06 1/s, A = 0.0193088 kg/m^3, B = 0.00569119 kg/m^3, C = 0.0139756 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Two-compartment bolus: D = 100 mg, V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h | SOLVED | alpha = 0.000259969 1/s, beta = 1.78084e-05 1/s, A = 0.00729416 kg/m^3, B = 0.00270584 kg/m^3, C = 0.00226643 kg/m^3 |
| Two-compartment bolus: D = 500 mg, V1 = 20 L, k10 = 0.1 1/h, k12 = 1.2 1/h, k21 = 0.4 1/h, t = 30 min | SOLVED | alpha = 0.000465593 1/s, beta = 6.629e-06 1/s, A = 0.0193088 kg/m^3, B = 0.00569119 kg/m^3, C = 0.0139756 kg/m^3 |
| Two-compartment bolus: V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h | NEEDS_INPUT | bmed_pk_two_compartment_bolus needs D (bolus dose). Example: solve('bmed_pk_two_compartment_bolus', D='9.999999999999999 |
| Two-compartment bolus: D = 100 mg, V1 = 10 L, k10 = 0.2 1/h, k12 = 0.5 1/h, k21 = 0.3 1/h, t = 4 h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_two_compartment_micro_constants"></a>

### Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice), `bmed_pk_two_compartment_micro_constants`

The central volume, the micro rate constants k10, k12, k21, the clearance and the steady-state volume of a two-compartment model from the biexponential fit $C = A$ exp(-alpha t) + B exp(-beta t) of an IV bolus.

$$
k_{21} = \frac{A\beta + B\alpha}{A + B},\quad k_{10} = \frac{\alpha\beta}{k_{21}},\quad k_{12} = \alpha + \beta - k_{21} - k_{10}
$$

| input | meaning | condition |
|---|---|---|
| `D` | bolus dose |  |
| `A` | coefficient of the fast exponential |  |
| `alpha` | fast rate constant |  |
| `B` | coefficient of the slow exponential |  |
| `beta` | slow rate constant |  |

| output | meaning |
|---|---|
| `V1` | central volume |
| `k10` | elimination rate constant |
| `k12` | central to peripheral |
| `k21` | peripheral to central |
| `CL` | clearance |
| `Vss` | volume of distribution at steady state |

- **Method:** $V_{1} = D/(A + B)$; k21 = (A beta + B alpha)/(A + B); $k10 = \alpha$ $\beta / k21$; $k12 = \alpha + \beta - k21 - k10$; $CL = k10$ V1; Vss = V1 $(1 + k12/k21)$
- **Assumes:** IV bolus, linear two-compartment model, elimination from the central compartment; $\alpha > \beta > 0$, $A > 0$, $B > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The two-compartment model recovered from the bi-exponential coefficients given (D = 100 mg, A = 6 mg/L, alpha = 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-micro-constants-schematic)

*The two-compartment model recovered from the bi-exponential coefficients given (D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h). Marked: V1 = 12.5 L, k10 = 0.4615 1/h, k12 = 0.701 1/h, k21 = 0.4875 1/h; also returned CL = 5.769 L/h and Vss = 30.47 L. In SI units: k21 = 0.0001354 1/s. Your inputs: D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h.*

![Peripheral to central as α runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-micro-constants-live-1)

*Peripheral to central as α runs from 0.9 to 2.1 1/h, every other input as you entered it (41 solutions). Marked: your α = 1.5 1/h gives k21 = 0.4875 1/h. Your inputs: D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h.*

![Volume of distribution at steady state as D runs from 60 to 140 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-two-compartment-micro-constants-live-2)

*Volume of distribution at steady state as D runs from 60 to 140 mg, every other input as you entered it (29 solutions). Marked: your D = 100 mg gives Vss = 30.47 L. Your inputs: D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h.*

```python
biomedical_science_ai999x.solve('bmed_pk_two_compartment_micro_constants', D=9.999999999999999e-05, A=0.006, alpha=0.00041666666666666664, B=0.002, beta=4.1666666666666665e-05)
# -> V1 = 0.0125 m^3, k10 = 0.000128205 1/s, k12 = 0.000194712 1/s, k21 = 0.000135417 1/s, CL = 1.60256e-06 m^3/s, Vss = 0.0304734 m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_two_compartment_micro_constants', D=0.0005, A=0.02, alpha=0.0008333333333333333, B=0.005, beta=5.555555555555556e-05)
# -> V1 = 0.02 m^3, k10 = 0.000219298 1/s, k12 = 0.00045848 1/s, k21 = 0.000211111 1/s, CL = 4.38596e-06 m^3/s, Vss = 0.0634349 m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Two-compartment micro-constants: D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h | SOLVED | V1 = 0.0125 m^3, k10 = 0.000128205 1/s, k12 = 0.000194712 1/s, k21 = 0.000135417 1/s, CL = 1.60256e-06 m^3/s, Vss = 0.0304734 m^3 |
| Two-compartment micro-constants: D = 500 mg, A = 20 mg/L, alpha = 3 1/h, B = 5 mg/L, beta = 0.2 1/h | SOLVED | V1 = 0.02 m^3, k10 = 0.000219298 1/s, k12 = 0.00045848 1/s, k21 = 0.000211111 1/s, CL = 4.38596e-06 m^3/s, Vss = 0.0634349 m^3 |
| Two-compartment micro-constants: A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h | NEEDS_INPUT | bmed_pk_two_compartment_micro_constants needs D (bolus dose). Example: solve('bmed_pk_two_compartment_micro_constants',  |
| Two-compartment micro-constants: D = 100 mg, A = 6 mg/L, alpha = 1.5 1/h, B = 2 mg/L, beta = 0.15 1/h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Two-compartment micro-constants: D = 100 mg, A = 6 mg/L, alpha = 0.1 1/h, B = 2 mg/L, beta = 0.15 1/h | INVALID_INPUT | Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical a |

<a id="bmed_pk_bioavailability_from_profiles"></a>

### Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice), `bmed_pk_bioavailability_from_profiles`

The absolute bioavailability F from the dose-normalised areas under an oral and an IV concentration-time profile (linear trapezoids from the first to the last sample).

$$
F = \frac{AUC_{oral}\,D_{iv}}{AUC_{iv}\,D_{oral}}
$$

| input | meaning | condition |
|---|---|---|
| `D_iv` | IV dose |  |
| `t_iv` | IV sampling times (one time unit for both profiles) |  |
| `C_iv` | IV concentrations (one concentration unit for both profiles) |  |
| `D_oral` | oral dose |  |
| `t_oral` | oral sampling times |  |
| `C_oral` | oral concentrations |  |

| output | meaning |
|---|---|
| `AUC_iv` | IV area (units of the lists) |
| `AUC_oral` | oral area (units of the lists) |
| `F` | absolute bioavailability (AUC_oral/D_oral)/(AUC_iv/D_iv) |

- **Method:** F = (AUC_oral D_iv)/(AUC_iv D_oral) with linear-trapezoid areas
- **Assumes:** linear pharmacokinetics (dose-proportional areas); both profiles sampled to a comparable end; areas to the last sample, no extrapolation; the same units in both profiles
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The intravenous and oral concentration profiles as given, with the areas the model computes: AUC_iv = 46](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-bioavailability-from-profiles-pharmacokinetics)

*The intravenous and oral concentration profiles as given, with the areas the model computes: AUC_iv = 46.55, AUC_oral = 58.9. Dose-normalised, their ratio is the absolute bioavailability F = 0.6327. Your inputs: D_iv = 9.999999999999999e-05 kg, t_iv = 0, 1, 2, 4, 8, 12, C_iv = 10, 8.2, 6.7, 4.5, 2, 0.9, D_oral = 0.00019999999999999998 kg, t_oral = 0, 1, 2, 4, 8, 12, C_oral = 0, 6.1, 8, 7.2, 3.9, 1.8.*

![Absolute bioavailability (AUC_oral/D_oral)/(A](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-bioavailability-from-profiles-live-1)

*Absolute bioavailability (AUC_oral/D_oral)/(A... as D_oral runs from 0.00012 to 0.00028 kg, every other input as you entered it (41 solutions). Marked: your D_oral = 0.0002 kg gives F = 0.6327. Your inputs: D_iv = 9.999999999999999e-05 kg, t_iv = 0, 1, 2, 4, 8, 12, C_iv = 10, 8.2, 6.7, 4.5, 2, 0.9, D_oral = 0.00019999999999999998 kg, t_oral = 0, 1, 2, 4, 8, 12, C_oral = 0, 6.1, 8, 7.2, 3.9, 1.8.*

![Absolute bioavailability (AUC_oral/D_oral)/(A](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-bioavailability-from-profiles-live-2)

*Absolute bioavailability (AUC_oral/D_oral)/(A... as D_iv runs from 6e-05 to 0.00014 kg, every other input as you entered it (29 solutions). Marked: your D_iv = 0.0001 kg gives F = 0.6327. Your inputs: D_iv = 9.999999999999999e-05 kg, t_iv = 0, 1, 2, 4, 8, 12, C_iv = 10, 8.2, 6.7, 4.5, 2, 0.9, D_oral = 0.00019999999999999998 kg, t_oral = 0, 1, 2, 4, 8, 12, C_oral = 0, 6.1, 8, 7.2, 3.9, 1.8.*

```python
biomedical_science_ai999x.solve('bmed_pk_bioavailability_from_profiles', D_iv=9.999999999999999e-05, t_iv=[0, 1, 2, 4, 8, 12], C_iv=[10, 8.2, 6.7, 4.5, 2.0, 0.9], D_oral=0.00019999999999999998, t_oral=[0, 1, 2, 4, 8, 12], C_oral=[0, 6.1, 8.0, 7.2, 3.9, 1.8])
# -> AUC_iv = 46.55, AUC_oral = 58.9, F = 0.632653   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_bioavailability_from_profiles', D_iv=4.9999999999999996e-05, t_iv=[0, 2, 4, 8], C_iv=[5, 3.5, 2.4, 1.2], D_oral=4.9999999999999996e-05, t_oral=[0, 2, 4, 8], C_oral=[0, 2.2, 2.0, 1.1])
# -> AUC_iv = 21.6, AUC_oral = 12.6, F = 0.583333   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Bioavailability from profiles: D_iv = 100 mg, t_iv = [0, 1, 2, 4, 8, 12], C_iv = [10, 8.2, 6.7, 4.5, 2, 0.9], D_oral = 200 mg, t_oral = [0, 1, 2, 4, 8, 12], C_oral = [0, 6.1, 8, 7.2, 3.9, 1.8] | SOLVED | AUC_iv = 46.55, AUC_oral = 58.9, F = 0.632653 |
| Bioavailability from profiles: D_iv = 50 mg, t_iv = [0, 2, 4, 8], C_iv = [5, 3.5, 2.4, 1.2], D_oral = 50 mg, t_oral = [0, 2, 4, 8], C_oral = [0, 2.2, 2, 1.1] | SOLVED | AUC_iv = 21.6, AUC_oral = 12.6, F = 0.583333 |
| Bioavailability from profiles: t_iv = [0, 1, 2, 4, 8, 12], C_iv = [10, 8.2, 6.7, 4.5, 2, 0.9], D_oral = 200 mg, t_oral = [0, 1, 2, 4, 8, 12], C_oral = [0, 6.1, 8, 7.2, 3.9, 1.8] | NEEDS_INPUT | bmed_pk_bioavailability_from_profiles needs D_iv (IV dose). Example: solve('bmed_pk_bioavailability_from_profiles', D_iv |
| Bioavailability from profiles: D_iv = 100 mg, t_iv = [0, 1, 2, 4, 8, 12], C_iv = [10, 8.2, 6.7, 4.5, 2, 0.9], D_oral = 200 mg, t_oral = [0, 1, 2, 4, 8, 12], C_oral = [0, 6.1, 8, 7.2, 3.9, 1.8], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_hepatic_clearance_well_stirred"></a>

### Hepatic clearance: well-stirred liver model (a model calculation, not medical advice), `bmed_pk_hepatic_clearance_well_stirred`

The hepatic clearance and extraction ratio of the well-stirred (venous equilibrium) liver model from hepatic blood flow, unbound fraction and intrinsic clearance.

$$
CL_H = \frac{Q\,f_u\,CL_{int}}{Q + f_u\,CL_{int}}
$$

| input | meaning | condition |
|---|---|---|
| `Q` | hepatic blood flow |  |
| `fu` | unbound fraction in blood $(0 < fu  \leq  1)$ |  |
| `CL_int` | intrinsic clearance of unbound drug |  |

| output | meaning |
|---|---|
| `CL_H` | hepatic clearance |
| `E` | extraction ratio |

- **Method:** $CL_H = Q$ fu CL_int / (Q + fu CL_int); $E = CL_H / Q$
- **Assumes:** well-stirred liver: outflow concentration equals the liver concentration; linear elimination
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Hepatic clearance as fu runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-hepatic-clearance-well-stirred-live-1)

*Hepatic clearance as fu runs from 0.025 to 0.175, every other input as you entered it (41 solutions). Marked: your fu = 0.1 gives CL_H = 0.5357 L/min. Your inputs: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h.*

![Hepatic clearance as CL_int runs from 300 to 700 L/h, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-hepatic-clearance-well-stirred-live-2)

*Hepatic clearance as CL_int runs from 300 to 700 L/h, every other input as you entered it (29 solutions). Marked: your CL_int = 500 L/h gives CL_H = 0.5357 L/min. Your inputs: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h.*

```python
biomedical_science_ai999x.solve('bmed_pk_hepatic_clearance_well_stirred', Q=2.4999999999999998e-05, fu=0.1, CL_int=0.0001388888888888889)
# -> CL_H = 8.92857e-06 m^3/s, E = 0.357143   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_hepatic_clearance_well_stirred', Q=2.5e-05, fu=0.5, CL_int=1e-05)
# -> CL_H = 4.16667e-06 m^3/s, E = 0.166667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Well-stirred hepatic clearance: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h | SOLVED | CL_H = 8.92857e-06 m^3/s, E = 0.357143 |
| Well-stirred hepatic clearance: Q = 1.5 L/min, fu = 50%, CL_int = 0.6 L/min | SOLVED | CL_H = 4.16667e-06 m^3/s, E = 0.166667 |
| Well-stirred hepatic clearance: fu = 0.1, CL_int = 500 L/h | NEEDS_INPUT | bmed_pk_hepatic_clearance_well_stirred needs Q (hepatic blood flow). Example: solve('bmed_pk_hepatic_clearance_well_stir |
| Well-stirred hepatic clearance: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_hepatic_clearance_parallel_tube"></a>

### Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice), `bmed_pk_hepatic_clearance_parallel_tube`

The hepatic clearance and extraction ratio of the parallel-tube liver model, in which the concentration falls along the sinusoids, from blood flow, unbound fraction and intrinsic clearance.

$$
CL_H = Q\left(1 - e^{-f_u CL_{int}/Q}\right)
$$

| input | meaning | condition |
|---|---|---|
| `Q` | hepatic blood flow |  |
| `fu` | unbound fraction in blood $(0 < fu  \leq  1)$ |  |
| `CL_int` | intrinsic clearance of unbound drug |  |

| output | meaning |
|---|---|
| `CL_H` | hepatic clearance |
| `E` | extraction ratio |

- **Method:** $E = 1 - exp$(-fu CL_int / Q); $CL_H = Q$ E
- **Assumes:** plug flow along identical sinusoids, enzymes evenly distributed; linear elimination
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Hepatic clearance as fu runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-hepatic-clearance-parallel-tube-live-1)

*Hepatic clearance as fu runs from 0.025 to 0.175, every other input as you entered it (41 solutions). Marked: your fu = 0.1 gives CL_H = 0.6394 L/min. Your inputs: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h.*

![Hepatic clearance as CL_int runs from 300 to 700 L/h, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-hepatic-clearance-parallel-tube-live-2)

*Hepatic clearance as CL_int runs from 300 to 700 L/h, every other input as you entered it (29 solutions). Marked: your CL_int = 500 L/h gives CL_H = 0.6394 L/min. Your inputs: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h.*

```python
biomedical_science_ai999x.solve('bmed_pk_hepatic_clearance_parallel_tube', Q=2.4999999999999998e-05, fu=0.1, CL_int=0.0001388888888888889)
# -> CL_H = 1.06562e-05 m^3/s, E = 0.426247   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_hepatic_clearance_parallel_tube', Q=2.5e-05, fu=0.5, CL_int=1e-05)
# -> CL_H = 4.53173e-06 m^3/s, E = 0.181269   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Parallel-tube hepatic clearance: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h | SOLVED | CL_H = 1.06562e-05 m^3/s, E = 0.426247 |
| Parallel-tube hepatic clearance: Q = 1.5 L/min, fu = 50%, CL_int = 0.6 L/min | SOLVED | CL_H = 4.53173e-06 m^3/s, E = 0.181269 |
| Parallel-tube hepatic clearance: fu = 0.1, CL_int = 500 L/h | NEEDS_INPUT | bmed_pk_hepatic_clearance_parallel_tube needs Q (hepatic blood flow). Example: solve('bmed_pk_hepatic_clearance_parallel |
| Parallel-tube hepatic clearance: Q = 90 L/h, fu = 0.1, CL_int = 500 L/h, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_pk_effect_site_concentration"></a>

### Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice), `bmed_pk_effect_site_concentration`

The effect-site concentration of a one-compartment model linked to an effect compartment (rate ke0) after an IV bolus, the time of its peak and the peak value.

$$
C_e(t) = \frac{D}{V}\,\frac{k_{e0}}{k_{e0} - k}\left(e^{-k t} - e^{-k_{e0} t}\right),\quad t_{peak} = \frac{\ln(k_{e0}/k)}{k_{e0} - k}
$$

| input | meaning | condition |
|---|---|---|
| `D` | bolus dose |  |
| `V` | volume of distribution |  |
| `k` | elimination rate constant |  |
| `ke0` | effect-site equilibration rate constant |  |
| `t` | time after the dose |  |

| output | meaning |
|---|---|
| `Ce` | effect-site concentration at time t |
| `t_peak` | time of the effect-site peak |
| `Ce_peak` | peak effect-site concentration |

- **Method:** dCe/dt = ke0 (C - Ce): $Ce = (D/V)$ $ke0/(ke0 - k)$ (exp(-k t) - exp(-ke0 t)); peak where $Ce = C$
- **Assumes:** one compartment plus an effect compartment of negligible volume; $ke0  \neq  k$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Effect-site concentration after an intravenous bolus (D = 10 mg, V = 20 L, k = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-effect-site-concentration-pharmacokinetics)

*Effect-site concentration after an intravenous bolus (D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h), 81 runs over 4.464 h, beside the plasma concentration of the same bolus (the model's one-compartment bolus). Dot: the peak t_peak = 1.116 h, Ce_peak = 0.3577 mg/L; marked: t = 0.5 h, Ce = 0.2899 mg/L. In SI units: Ce = 0.0002899 kg/m^3. Your inputs: D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min.*

![Effect-site concentration at time t as V runs from 12 to 28 L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-effect-site-concentration-live-1)

*Effect-site concentration at time t as V runs from 12 to 28 L, every other input as you entered it (41 solutions). Marked: your V = 20 L gives Ce = 0.2899 mg/L. Your inputs: D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min.*

![Effect-site concentration at time t as D runs from 6 to 14 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-pk-effect-site-concentration-live-2)

*Effect-site concentration at time t as D runs from 6 to 14 mg, every other input as you entered it (29 solutions). Marked: your D = 10 mg gives Ce = 0.2899 mg/L. Your inputs: D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min.*

```python
biomedical_science_ai999x.solve('bmed_pk_effect_site_concentration', D=9.999999999999999e-06, V=0.02, k=8.333333333333333e-05, ke0=0.0005555555555555556, t=1800.0)
# -> Ce = 0.000289899 kg/m^3, t_peak = 4017.43 s, Ce_peak = 0.000357746 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_pk_effect_site_concentration', D=9.999999999999999e-05, V=0.05, k=2.777777777777778e-05, ke0=0.0001388888888888889, t=14400.0)
# -> Ce = 0.00133746 kg/m^3, t_peak = 14484.9 s, Ce_peak = 0.00133748 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Effect-site concentration: D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min | SOLVED | Ce = 0.000289899 kg/m^3, t_peak = 4017.43 s, Ce_peak = 0.000357746 kg/m^3 |
| Effect-site concentration: D = 100 mg, V = 50 L, k = 0.1 1/h, ke0 = 0.5 1/h, t = 4 h | SOLVED | Ce = 0.00133746 kg/m^3, t_peak = 14484.9 s, Ce_peak = 0.00133748 kg/m^3 |
| Effect-site concentration: V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min | NEEDS_INPUT | bmed_pk_effect_site_concentration needs D (bolus dose). Example: solve('bmed_pk_effect_site_concentration', D='9.9999999 |
| Effect-site concentration: D = 10 mg, V = 20 L, k = 0.3 1/h, ke0 = 2 1/h, t = 30 min, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_epi_sir_final_size"></a>

### SIR epidemic model: final size (final susceptible fraction and attack fraction), `bmed_epi_sir_final_size`

The fraction still susceptible when an SIR epidemic ends and the fraction infected during it, from R0 and the initial susceptible and infected fractions (the final-size relation).

$$
\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + i_0 - s_\infty)
$$

| input | meaning | condition |
|---|---|---|
| `R0` | basic reproduction number beta/gamma |  |
| `s0` | initial susceptible fraction |  |
| `i0` | initial infected fraction (> 0) |  |

| output | meaning |
|---|---|
| `s_inf` | susceptible fraction at the end |
| `attack` | fraction infected during the epidemic, s0 - s_inf |

- **Method:** final-size relation ln(s0/s_inf) = R0 (s0 + i0 - s_inf), solved by bisection on (0, min(s0, 1/R0)]
- **Assumes:** closed population, no births or deaths, homogeneous mixing; fractions of the population; the rest of the population $(1 - s_{0} - i_{0})$ is immune
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Susceptible fraction at the end as R0 runs from 0](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-epi-sir-final-size-live-1.svg)

*Susceptible fraction at the end as R0 runs from 0.625 to 4.375, every other input as you entered it (41 solutions). Marked: your R0 = 2.5 gives s_inf = 0.1059. Your inputs: R0 = 2.5, s0 = 0.99, i0 = 0.01.*

![Susceptible fraction at the end as s0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-sir-final-size-live-2)

*Susceptible fraction at the end as s0 runs from 0.2475 to 0.99, every other input as you entered it (15 solutions). Marked: your s0 = 0.99 gives s_inf = 0.1059. Your inputs: R0 = 2.5, s0 = 0.99, i0 = 0.01.*

```python
biomedical_science_ai999x.solve('bmed_epi_sir_final_size', R0=2.5, s0=0.99, i0=0.01)
# -> s_inf = 0.105894, attack = 0.884106   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_sir_final_size', R0=1.5, s0=0.7, i0=0.001)
# -> s_inf = 0.618785, attack = 0.0812154   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SIR final size: R0 = 2.5, s0 = 0.99, i0 = 0.01 | SOLVED | s_inf = 0.105894, attack = 0.884106 |
| SIR final size: R0 = 1.5, s0 = 0.7, i0 = 0.001 | SOLVED | s_inf = 0.618785, attack = 0.0812154 |
| SIR final size: s0 = 0.99, i0 = 0.01 | NEEDS_INPUT | bmed_epi_sir_final_size needs R0 (basic reproduction number beta/gamma). Example: solve('bmed_epi_sir_final_size', R0=2. |
| SIR final size: R0 = 2.5, s0 = 0.99, i0 = 0.01, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| SIR final size: R0 = 2.5, s0 = 99%, i0 = 1% | SOLVED | s_inf = 0.105894, attack = 0.884106 |
| SIR final size: R0 = 2.5, s0 = 0.99, i0 = 0 | INVALID_INPUT | SIR epidemic model: final size (final susceptible fraction and attack fraction): i0 must be a fraction between 0 and 1. |

<a id="bmed_epi_seir_final_size"></a>

### SEIR epidemic model: final size, `bmed_epi_seir_final_size`

The final susceptible fraction and the attack fraction of an SEIR epidemic (the latent stage does not change the final size) from R0 and the initial susceptible, exposed and infectious fractions.

$$
\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + e_0 + i_0 - s_\infty)
$$

| input | meaning | condition |
|---|---|---|
| `R0` | basic reproduction number beta/gamma |  |
| `s0` | initial susceptible fraction |  |
| `e0` | initial exposed (latent) fraction |  |
| `i0` | initial infectious fraction |  |

| output | meaning |
|---|---|
| `s_inf` | susceptible fraction at the end |
| `attack` | fraction infected during the epidemic, s0 - s_inf |

- **Method:** final-size relation ln(s0/s_inf) = R0 (s0 + e0 + i0 - s_inf), solved by bisection
- **Assumes:** closed population, homogeneous mixing; exponentially distributed latent and infectious periods; $e_{0} + i_{0} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Susceptible fraction at the end as R0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-final-size-live-1)

*Susceptible fraction at the end as R0 runs from 0.75 to 5.25, every other input as you entered it (41 solutions). Marked: your R0 = 3 gives s_inf = 0.05808. Your inputs: R0 = 3, s0 = 0.98, e0 = 0.01, i0 = 0.01.*

![Susceptible fraction at the end as s0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-final-size-live-2)

*Susceptible fraction at the end as s0 runs from 0.245 to 0.98, every other input as you entered it (15 solutions). Marked: your s0 = 0.98 gives s_inf = 0.05808. Your inputs: R0 = 3, s0 = 0.98, e0 = 0.01, i0 = 0.01.*

```python
biomedical_science_ai999x.solve('bmed_epi_seir_final_size', R0=3, s0=0.98, e0=0.01, i0=0.01)
# -> s_inf = 0.058078, attack = 0.921922   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_seir_final_size', R0=1.8, s0=0.6, e0=0.002, i0=0)
# -> s_inf = 0.493974, attack = 0.106026   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SEIR final size: R0 = 3, s0 = 0.98, e0 = 0.01, i0 = 0.01 | SOLVED | s_inf = 0.058078, attack = 0.921922 |
| SEIR final size: R0 = 1.8, s0 = 0.6, e0 = 0.002, i0 = 0 | SOLVED | s_inf = 0.493974, attack = 0.106026 |
| SEIR final size: s0 = 0.98, e0 = 0.01, i0 = 0.01 | NEEDS_INPUT | bmed_epi_seir_final_size needs R0 (basic reproduction number beta/gamma). Example: solve('bmed_epi_seir_final_size', R0= |
| SEIR final size: R0 = 3, s0 = 0.98, e0 = 0.01, i0 = 0.01, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_epi_sir_peak"></a>

### SIR epidemic model: peak prevalence and the time of the peak, `bmed_epi_sir_peak`

The largest infected fraction of an SIR epidemic and the time at which it occurs, from the transmission and recovery rates and the initial fractions.

$$
i_{max} = s_0 + i_0 - \frac{1 + \ln(R_0 s_0)}{R_0},\quad t_{peak} = \int_{1/R_0}^{s_0}\frac{ds}{\beta s\,i(s)}
$$

| input | meaning | condition |
|---|---|---|
| `beta` | transmission rate |  |
| `gamma` | recovery rate |  |
| `s0` | initial susceptible fraction |  |
| `i0` | initial infected fraction (> 0) |  |

| output | meaning |
|---|---|
| `i_max` | peak infected fraction |
| `t_peak` | time of the peak |

- **Method:** i_max = s0 + i0 - (1 + ln(R0 s0))/R0 from the first integral of the SIR system; t_peak = integral over s from $1/R_{0}$ to s0 of ds / (beta s i(s)) (mpmath quadrature)
- **Assumes:** closed population, homogeneous mixing, frequency-dependent transmission; R0 $s_{0} > 1$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Peak infected fraction as s0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-sir-peak-live-1)

*Peak infected fraction as s0 runs from 0.3531 to 0.999, every other input as you entered it (41 solutions). Marked: your s0 = 0.999 gives i_max = 0.3008. Your inputs: beta = 0.3 1/day, gamma = 0.1 1/day, s0 = 0.999, i0 = 0.001.*

![Peak infected fraction as β runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-sir-peak-live-2)

*Peak infected fraction as β runs from 0.18 to 0.42 1/day, every other input as you entered it (25 solutions). Marked: your β = 0.3 1/day gives i_max = 0.3008. Your inputs: beta = 0.3 1/day, gamma = 0.1 1/day, s0 = 0.999, i0 = 0.001.*

```python
biomedical_science_ai999x.solve('bmed_epi_sir_peak', beta=3.472222222222222e-06, gamma=1.1574074074074074e-06, s0=0.999, i0=0.001)
# -> i_max = 0.300796, t_peak = 3.3139e+06 s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_sir_peak', beta=5.787037037037037e-06, gamma=2.3148148148148148e-06, s0=0.9, i0=0.01)
# -> i_max = 0.185628, t_peak = 1.56867e+06 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SIR peak: beta = 0.3 1/day, gamma = 0.1 1/day, s0 = 0.999, i0 = 0.001 | SOLVED | i_max = 0.300796, t_peak = 3.3139e+06 s |
| SIR peak: beta = 0.5 1/day, gamma = 0.2 1/day, s0 = 0.9, i0 = 0.01 | SOLVED | i_max = 0.185628, t_peak = 1.56867e+06 s |
| SIR peak: gamma = 0.1 1/day, s0 = 0.999, i0 = 0.001 | NEEDS_INPUT | bmed_epi_sir_peak needs beta (transmission rate). Example: solve('bmed_epi_sir_peak', beta='3.472222222222222e-06 1/s',  |
| SIR peak: beta = 0.3 1/day, gamma = 0.1 1/day, s0 = 0.999, i0 = 0.001, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| SIR peak: beta = 0.1 1/day, gamma = 0.2 1/day, s0 = 0.999, i0 = 0.001 | INVALID_INPUT | SIR epidemic model: peak prevalence and the time of the peak: no epidemic peak: R0 s0 must exceed 1 (prevalence only fal |

<a id="bmed_epi_seir_growth_rate"></a>

### SEIR epidemic model: initial exponential growth rate from R0, `bmed_epi_seir_growth_rate`

The early exponential growth (or decline) rate of an SEIR epidemic in a fully susceptible population, from R0, the rate of leaving the latent stage and the recovery rate.

$$
r = \frac{-(\sigma + \gamma) + \sqrt{(\sigma + \gamma)^2 + 4\sigma\gamma(R_0 - 1)}}{2}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | basic reproduction number beta/gamma |  |
| `sigma` | rate of leaving the latent stage (1/latent period) |  |
| `gamma` | recovery rate (1/infectious period) |  |

| output | meaning |
|---|---|
| `r` | initial growth rate (negative when R0 < 1) |

- **Method:** largest root of $r^{2} + (\sigma + \gamma)$ r - sigma gamma $(R_{0} - 1) = 0$
- **Assumes:** fully susceptible population at the start; exponential latent and infectious periods
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Initial growth rate (negative when R0 < 1) as R0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-growth-rate-live-1)

*Initial growth rate (negative when R0 < 1) as R0 runs from 0.625 to 4.375, every other input as you entered it (41 solutions). Marked: your R0 = 2.5 gives r = 9.158e-07. Your inputs: R0 = 2.5, sigma = 0.2 1/day, gamma = 0.1 1/day.*

![Initial growth rate (negative when R0 < 1) as γ runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-growth-rate-live-2)

*Initial growth rate (negative when R0 < 1) as γ runs from 0.06 to 0.14 1/day, every other input as you entered it (29 solutions). Marked: your γ = 0.1 1/day gives r = 9.158e-07. Your inputs: R0 = 2.5, sigma = 0.2 1/day, gamma = 0.1 1/day.*

```python
biomedical_science_ai999x.solve('bmed_epi_seir_growth_rate', R0=2.5, sigma=2.3148148148148148e-06, gamma=1.1574074074074074e-06)
# -> r = 9.15842e-07 1/s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_seir_growth_rate', R0=0.8, sigma=5.787037037037037e-06, gamma=2.8935185185185184e-06)
# -> r = -4.04667e-07 1/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SEIR growth rate: R0 = 2.5, sigma = 0.2 1/day, gamma = 0.1 1/day | SOLVED | r = 9.15842e-07 1/s |
| SEIR growth rate: R0 = 0.8, sigma = 0.5 1/day, gamma = 0.25 1/day | SOLVED | r = -4.04667e-07 1/s |
| SEIR growth rate: sigma = 0.2 1/day, gamma = 0.1 1/day | NEEDS_INPUT | bmed_epi_seir_growth_rate needs R0 (basic reproduction number beta/gamma). Example: solve('bmed_epi_seir_growth_rate', R |
| SEIR growth rate: R0 = 2.5, sigma = 0.2 1/day, gamma = 0.1 1/day, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_epi_r_from_growth_rate"></a>

### Reproduction number from an epidemic growth rate (gamma-distributed generation interval), `bmed_epi_r_from_growth_rate`

The reproduction number implied by an observed exponential growth rate when the generation interval follows a gamma distribution of stated mean and standard deviation (Wallinga-Lipsitch).

$$
R = \left(1 + r\,\frac{s^2}{\mu}\right)^{\mu^2/s^2}
$$

| input | meaning | condition |
|---|---|---|
| `r` | exponential growth rate (negative for a decline) |  |
| `Tg_mean` | mean generation interval |  |
| `Tg_sd` | standard deviation of the generation interval |  |

| output | meaning |
|---|---|
| `R` | reproduction number |

- **Method:** $R = 1 / M$(-r), M the moment generating function of the gamma distribution: R = (1 + r theta)^k, k = mean^2/sd^2, theta = sd^2/mean
- **Assumes:** exponential growth or decline; gamma-distributed generation interval
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Reproduction number as Tg_mean runs from 3 to 7 day, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-r-from-growth-rate-live-1)

*Reproduction number as Tg_mean runs from 3 to 7 day, every other input as you entered it (41 solutions). Marked: your Tg_mean = 5 day gives R = 1.618. Your inputs: r = 0.1 1/day, Tg_mean = 5 day, Tg_sd = 2 day.*

![Reproduction number as r runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-r-from-growth-rate-live-2)

*Reproduction number as r runs from 0.06 to 0.14 1/day, every other input as you entered it (29 solutions). Marked: your r = 0.1 1/day gives R = 1.618. Your inputs: r = 0.1 1/day, Tg_mean = 5 day, Tg_sd = 2 day.*

```python
biomedical_science_ai999x.solve('bmed_epi_r_from_growth_rate', r=1.1574074074074074e-06, Tg_mean=432000.0, Tg_sd=172800.0)
# -> R = 1.6177   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_r_from_growth_rate', r=-5.787037037037037e-07, Tg_mean=561600.0, Tg_sd=345600.0)
# -> R = 0.70694   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Reproduction number from growth rate: r = 0.1 1/day, Tg_mean = 5 day, Tg_sd = 2 day | SOLVED | R = 1.6177 |
| Reproduction number from growth rate: r = -0.05 1/day, Tg_mean = 6.5 day, Tg_sd = 4 day | SOLVED | R = 0.70694 |
| Reproduction number from growth rate: Tg_mean = 5 day, Tg_sd = 2 day | NEEDS_INPUT | bmed_epi_r_from_growth_rate needs r (exponential growth rate (negative for a decline)). Example: solve('bmed_epi_r_from_ |
| Reproduction number from growth rate: r = 0.1 1/day, Tg_mean = 5 day, Tg_sd = 2 day, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_epi_sis_prevalence"></a>

### SIS epidemic model: prevalence at a time and the endemic level, `bmed_epi_sis_prevalence`

The infected fraction of an SIS model (no immunity after recovery) at time t and the endemic equilibrium it approaches.

$$
i(t) = \frac{i_0\,e^{rt}}{1 + i_0\,\beta\,(e^{rt} - 1)/r},\quad r = \beta - \gamma,\quad i^* = 1 - \gamma/\beta
$$

| input | meaning | condition |
|---|---|---|
| `beta` | transmission rate |  |
| `gamma` | recovery rate |  |
| `i0` | initial infected fraction |  |
| `t` | time |  |

| output | meaning |
|---|---|
| `i` | infected fraction at time t |
| `i_star` | endemic infected fraction (0 when beta <= gamma) |

- **Method:** logistic solution of $di/dt = \beta$ i $(1 - i) - \gamma$ i: $i = i_{0}$ e^{rt} / (1 + i0 beta (e^{rt} - 1)/r), $r = \beta - \gamma$
- **Assumes:** closed population, homogeneous mixing; recovered individuals are susceptible again
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Infected fraction at time t as β runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-sis-prevalence-live-1)

*Infected fraction at time t as β runs from 0.24 to 0.56 1/day, every other input as you entered it (41 solutions). Marked: your β = 0.4 1/day gives i = 0.1601. Your inputs: beta = 0.4 1/day, gamma = 0.1 1/day, i0 = 0.01, t = 10 day.*

![Infected fraction at time t as t runs from 6 to 14 day, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-sis-prevalence-live-2)

*Infected fraction at time t as t runs from 6 to 14 day, every other input as you entered it (29 solutions). Marked: your t = 10 day gives i = 0.1601. Your inputs: beta = 0.4 1/day, gamma = 0.1 1/day, i0 = 0.01, t = 10 day.*

```python
biomedical_science_ai999x.solve('bmed_epi_sis_prevalence', beta=4.6296296296296296e-06, gamma=1.1574074074074074e-06, i0=0.01, t=864000.0)
# -> i = 0.160111, i_star = 0.75   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_sis_prevalence', beta=1.1574074074074074e-06, gamma=2.3148148148148148e-06, i0=0.3, t=432000.0)
# -> i = 0.162748, i_star = 0   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SIS prevalence: beta = 0.4 1/day, gamma = 0.1 1/day, i0 = 0.01, t = 10 day | SOLVED | i = 0.160111, i_star = 0.75 |
| SIS prevalence: beta = 0.1 1/day, gamma = 0.2 1/day, i0 = 0.3, t = 5 day | SOLVED | i = 0.162748, i_star = 0 |
| SIS prevalence: gamma = 0.1 1/day, i0 = 0.01, t = 10 day | NEEDS_INPUT | bmed_epi_sis_prevalence needs beta (transmission rate). Example: solve('bmed_epi_sis_prevalence', beta='4.62962962962962 |
| SIS prevalence: beta = 0.4 1/day, gamma = 0.1 1/day, i0 = 0.01, t = 10 day, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_epi_reed_frost_final_size"></a>

### Reed-Frost chain-binomial epidemic: distribution and mean of the final size, `bmed_epi_reed_frost_final_size`

The probability distribution of the number of susceptibles eventually infected in a Reed-Frost chain-binomial epidemic in a small closed group, its mean and the chance of no spread.

$$
S_{t+1} \sim \mathrm{Bin}\!\left(S_t, (1 - p)^{I_t}\right),\quad I_{t+1} = S_t - S_{t+1}
$$

| input | meaning | condition |
|---|---|---|
| `n` | initial number of susceptibles (1 to 30) |  |
| `m` | initial number of infectives (1 to 30) |  |
| `p` | probability that one infective infects one given susceptible in a generation $(0 < p < 1)$ |  |

| output | meaning |
|---|---|
| `mean_final` | expected number of susceptibles infected |
| `P_no_spread` | probability that no susceptible is infected |
| `distribution` | probabilities of 0, 1, ..., n susceptibles infected |

- **Method:** Markov chain over (susceptibles, infectives): a susceptible escapes i infectives with probability $(1 - p)^{i}$; new infections are binomial; probabilities summed over all paths
- **Assumes:** discrete generations, each infective infectious for one generation; independent contacts
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Reed-Frost chain-binomial final size: probability that 0, 1, ](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-reed-frost-final-size-distribution)

*Reed-Frost chain-binomial final size: probability that 0, 1, ..., 4 of 4 susceptibles are infected, starting from 1 infective(s), for three per-contact infection probabilities (half, the given and twice p, one run each): p = 0.1: mean 0.5254, no spread 0.6561; p = 0.2: mean 1.27, no spread 0.4096; p = 0.4: mean 2.853, no spread 0.1296. Your inputs: n = 4, m = 1, p = 0.2.*

![Expected number of susceptibles infected as n runs from 1 to 8, every other input as you entered it (8 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-reed-frost-final-size-live-1)

*Expected number of susceptibles infected as n runs from 1 to 8, every other input as you entered it (8 solutions). Marked: your n = 4 gives mean_final = 1.27. Your inputs: n = 4, m = 1, p = 0.2.*

![Every entry of distribution as computed for your inputs (5 entries: 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-reed-frost-final-size-live-2)

*Every entry of distribution as computed for your inputs (5 entries: 0.4096, 0.2097, 0.1636, 0.1352, 0.08195). Your inputs: n = 4, m = 1, p = 0.2.*

![Expected number of susceptibles infected as m runs from 1 to 5, every other input as you entered it (5 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-reed-frost-final-size-live-3)

*Expected number of susceptibles infected as m runs from 1 to 5, every other input as you entered it (5 solutions). Marked: your m = 1 gives mean_final = 1.27. Your inputs: n = 4, m = 1, p = 0.2.*

```python
biomedical_science_ai999x.solve('bmed_epi_reed_frost_final_size', n=4, m=1, p=0.2)
# -> mean_final = 1.27014, P_no_spread = 0.4096, distribution = [0.4096, 0.2097152, 0.163577856, 0.1351614464, 0.0819454976]   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_reed_frost_final_size', n=10, m=2, p=0.05)
# -> mean_final = 1.53511, P_no_spread = 0.358486, distribution = [0.358485922409, 0.244085487499, 0.159570440671, 0.10291044883, 0.0640931869702, 0.0374640511926, 0.0198552532867, 0.00911594235476, 0.00338709781558, 0.000902915019467, 0.000129253952282]   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Reed-Frost final size: n = 4, m = 1, p = 0.2 | SOLVED | mean_final = 1.27014, P_no_spread = 0.4096, distribution = [0.4096, 0.2097152, 0.163577856, 0.1351614464, 0.0819454976] |
| Reed-Frost final size: n = 10, m = 2, p = 0.05 | SOLVED | mean_final = 1.53511, P_no_spread = 0.358486, distribution = [0.358485922409, 0.244085487499, 0.159570440671, 0.10291044883, 0.0640931869702, 0.0374640511926, 0.0198552532867, 0.00911594235476, 0.00338709781558, 0.000902915019467, 0.000129253952282] |
| Reed-Frost final size: m = 1, p = 0.2 | NEEDS_INPUT | bmed_epi_reed_frost_final_size needs n (initial number of susceptibles (1 to 30)). Example: solve('bmed_epi_reed_frost_f |
| Reed-Frost final size: n = 4, m = 1, p = 0.2, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Reed-Frost final size: n = 4.5, m = 1, p = 0.2 | INVALID_INPUT | n: expected a whole number written exactly, got '4.5'. |
| Reed-Frost final size: n = 4, m = 1, p = 1.2 | INVALID_INPUT | Reed-Frost chain-binomial epidemic: distribution and mean of the final size: p must be a fraction between 0 and 1. |

<a id="bmed_dx_roc_auc"></a>

### ROC curve area of a diagnostic score (Mann-Whitney form), `bmed_dx_roc_auc`

The area under the ROC curve of a continuous diagnostic score from the scores of diseased and non-diseased subjects: the probability that a diseased subject scores higher (ties count one half), with the Mann-Whitney U.

$$
AUC = \frac{1}{n_1 n_0}\sum_{i,j}\left[\mathbb{1}(x_i > y_j) + \tfrac12\mathbb{1}(x_i = y_j)\right]
$$

| input | meaning | condition |
|---|---|---|
| `scores_pos` | scores of the diseased (condition-positive) subjects |  |
| `scores_neg` | scores of the non-diseased subjects |  |

| output | meaning |
|---|---|
| `AUC` | area under the ROC curve |
| `U` | Mann-Whitney U of the diseased group |

- **Method:** AUC = U/(n1 n0), U = number of (diseased, non-diseased) pairs with the diseased score higher + half the ties
- **Assumes:** higher scores indicate disease; ties count one half
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Scores of the 5 diseased and 6 healthy subjects given (bottom) and the ROC curve they give (top; sensitivity and specificity from the model at each of 11 thresholds)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-roc-auc-roc)

*Scores of the 5 diseased and 6 healthy subjects given (bottom) and the ROC curve they give (top; sensitivity and specificity from the model at each of 11 thresholds). The model's area under the curve AUC = 0.8833, equal to the Mann-Whitney U = 26.5 divided by 30 pairs. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3....*

![Area under the ROC curve as scores_pos[3] runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-roc-auc-live-1)

*Area under the ROC curve as scores_pos[3] runs from 0.175 to 1.225, every other input as you entered it (41 solutions). Marked: your scores_pos[3] = 0.7 gives AUC = 0.8833. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3....*

![Area under the ROC curve as scores_pos[4] runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-roc-auc-live-2)

*Area under the ROC curve as scores_pos[4] runs from 0.1375 to 0.9625, every other input as you entered it (29 solutions). Marked: your scores_pos[4] = 0.55 gives AUC = 0.8833. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3....*

```python
biomedical_science_ai999x.solve('bmed_dx_roc_auc', scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4], scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1])
# -> AUC = 0.883333, U = 26.5   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dx_roc_auc', scores_pos=[3, 4, 5, 5], scores_neg=[1, 2, 3, 5])
# -> AUC = 0.78125, U = 12.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ROC AUC: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1] | SOLVED | AUC = 0.883333, U = 26.5 |
| ROC AUC: scores_pos = [3, 4, 5, 5], scores_neg = [1, 2, 3, 5] | SOLVED | AUC = 0.78125, U = 12.5 |
| ROC AUC: scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1] | NEEDS_INPUT | bmed_dx_roc_auc needs scores_pos (scores of the diseased (condition-positive) subjects). Example: solve('bmed_dx_roc_auc |
| ROC AUC: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_dx_youden_cutoff"></a>

### Optimal diagnostic cut-off by the Youden index, `bmed_dx_youden_cutoff`

The score threshold that maximises the Youden index J = sensitivity + specificity - 1 over all observed cut-offs, with the sensitivity and specificity there.

$$
J(c) = \mathrm{Se}(c) + \mathrm{Sp}(c) - 1,\quad c^* = \arg\max_c J(c)
$$

| input | meaning | condition |
|---|---|---|
| `scores_pos` | scores of the diseased subjects |  |
| `scores_neg` | scores of the non-diseased subjects |  |

| output | meaning |
|---|---|
| `threshold` | optimal cut-off (score >= threshold is test-positive) |
| `sensitivity` | sensitivity at the cut-off |
| `specificity` | specificity at the cut-off |
| `J` | Youden index at the cut-off |

- **Method:** for every observed score as threshold: $J = TP/n_{1} - FP/n_{0}$ (exact fractions); the maximum, highest threshold first
- **Assumes:** higher scores indicate disease; cut-offs at the observed scores; ties in J: the highest threshold (most specific); some cut-off has $J > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Optimal cut-off (score >= threshold is](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-youden-cutoff-live-1)

*Optimal cut-off (score >= threshold is... as scores_pos[3] runs from 0.175 to 1.225, every other input as you entered it (41 solutions). Marked: your scores_pos[3] = 0.7 gives threshold = 0.55. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3....*

![Optimal cut-off (score >= threshold is](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-youden-cutoff-live-2)

*Optimal cut-off (score >= threshold is... as scores_neg[2] runs from 0.1125 to 0.7875, every other input as you entered it (29 solutions). Marked: your scores_neg[2] = 0.45 gives threshold = 0.55. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3....*

```python
biomedical_science_ai999x.solve('bmed_dx_youden_cutoff', scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4], scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1])
# -> threshold = 0.55, sensitivity = 0.8, specificity = 0.833333, J = 0.633333   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dx_youden_cutoff', scores_pos=[12, 15, 18, 22, 25, 30], scores_neg=[8, 10, 11, 14, 16, 19, 21])
# -> threshold = 22, sensitivity = 0.5, specificity = 1, J = 0.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Youden cut-off: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1] | SOLVED | threshold = 0.55, sensitivity = 0.8, specificity = 0.833333, J = 0.633333 |
| Youden cut-off: scores_pos = [12, 15, 18, 22, 25, 30], scores_neg = [8, 10, 11, 14, 16, 19, 21] | SOLVED | threshold = 22, sensitivity = 0.5, specificity = 1, J = 0.5 |
| Youden cut-off: scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1] | NEEDS_INPUT | bmed_dx_youden_cutoff needs scores_pos (scores of the diseased subjects). Example: solve('bmed_dx_youden_cutoff', scores |
| Youden cut-off: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_dx_threshold_sensitivity_specificity"></a>

### Sensitivity and specificity of a diagnostic score at a cut-off, `bmed_dx_threshold_sensitivity_specificity`

The confusion-matrix counts, sensitivity and specificity of a continuous diagnostic score when scores at or above a threshold are called positive.

$$
\mathrm{Se} = \frac{TP}{TP + FN},\quad \mathrm{Sp} = \frac{TN}{TN + FP}
$$

| input | meaning | condition |
|---|---|---|
| `scores_pos` | scores of the diseased subjects |  |
| `scores_neg` | scores of the non-diseased subjects |  |
| `threshold` | cut-off: a score >= threshold is test-positive |  |

| output | meaning |
|---|---|
| `sensitivity` | TP/(TP + FN) |
| `specificity` | TN/(TN + FP) |
| `TP` | true positives |
| `FN` | false negatives |
| `TN` | true negatives |
| `FP` | false positives |

- **Method:** count the 2x2 table at the cut-off; sensitivity = TP/(TP + FN), specificity = TN/(TN + FP)
- **Assumes:** a score at or above the threshold is positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![ROC curve of a test with 5 diseased and 6 healthy scores (dots below): sensitivity and specificity computed by the model at 11 thresholds; area under the curve 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-threshold-sensitivity-specificity-roc)

*ROC curve of a test with 5 diseased and 6 healthy scores (dots below): sensitivity and specificity computed by the model at 11 thresholds; area under the curve 0.8833. Marked: the cut-off given, 0.5, sensitivity 0.8, specificity 0.8333. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3..., threshold = 0.5.*

![TP/(TP + FN) as threshold runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-threshold-sensitivity-specificity-live-1)

*TP/(TP + FN) as threshold runs from 0.125 to 0.875, every other input as you entered it (39 solutions). Marked: your threshold = 0.5 gives sensitivity = 0.8. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3..., threshold = 0.5.*

![TP/(TP + FN) as scores_pos[5] runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-threshold-sensitivity-specificity-live-2)

*TP/(TP + FN) as scores_pos[5] runs from 0.1 to 0.7, every other input as you entered it (29 solutions). Marked: your scores_pos[5] = 0.4 gives sensitivity = 0.8. Your inputs: scores_pos = 0.9, 0.8, 0.7, 0.55, 0.4, scores_neg = 0.6, 0.45, 0.4, 0.3..., threshold = 0.5.*

```python
biomedical_science_ai999x.solve('bmed_dx_threshold_sensitivity_specificity', scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4], scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1], threshold=0.5)
# -> sensitivity = 0.8, specificity = 0.833333, TP = 4, FN = 1, TN = 5, FP = 1   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dx_threshold_sensitivity_specificity', scores_pos=[12, 15, 18, 22, 25, 30], scores_neg=[8, 10, 11, 14, 16, 19, 21], threshold=15)
# -> sensitivity = 0.833333, specificity = 0.571429, TP = 5, FN = 1, TN = 4, FP = 3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Sensitivity and specificity at a cut-off: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], threshold = 0.5 | SOLVED | sensitivity = 0.8, specificity = 0.833333, TP = 4, FN = 1, TN = 5, FP = 1 |
| Sensitivity and specificity at a cut-off: scores_pos = [12, 15, 18, 22, 25, 30], scores_neg = [8, 10, 11, 14, 16, 19, 21], threshold = 15 | SOLVED | sensitivity = 0.833333, specificity = 0.571429, TP = 5, FN = 1, TN = 4, FP = 3 |
| Sensitivity and specificity at a cut-off: scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], threshold = 0.5 | NEEDS_INPUT | bmed_dx_threshold_sensitivity_specificity needs scores_pos (scores of the diseased subjects). Example: solve('bmed_dx_th |
| Sensitivity and specificity at a cut-off: scores_pos = [0.9, 0.8, 0.7, 0.55, 0.4], scores_neg = [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], threshold = 0.5, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_dx_sensitivity_specificity_ci"></a>

### Sensitivity and specificity with Wilson score intervals, `bmed_dx_sensitivity_specificity_ci`

Sensitivity and specificity of a diagnostic test from its 2x2 counts, each with a Wilson score confidence interval at the stated level.

$$
\frac{\hat p + \frac{z^2}{2n} \pm z\sqrt{\frac{\hat p(1-\hat p)}{n} + \frac{z^2}{4n^2}}}{1 + \frac{z^2}{n}}
$$

| input | meaning | condition |
|---|---|---|
| `TP` | true positives |  |
| `FN` | false negatives |  |
| `TN` | true negatives |  |
| `FP` | false positives |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `sensitivity` | TP/(TP + FN) |
| `sens_lower` | lower Wilson limit |
| `sens_upper` | upper Wilson limit |
| `specificity` | TN/(TN + FP) |
| `spec_lower` | lower Wilson limit |
| `spec_upper` | upper Wilson limit |

- **Method:** Wilson interval (p + z^2/2n +- z sqrt(p(1 - p)/n + z^2/4n^2)) / $(1 + z^{2}/n)$
- **Assumes:** independent subjects; Wilson score interval (no continuity correction); level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Sensitivity and specificity with their 90 % Wilson intervals for the counts given (TP = 18, FN = 2, TN = 70, FP = 30) and for the same proportions with the counts scaled by 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-sensitivity-specificity-ci-intervals)

*Sensitivity and specificity with their 90 % Wilson intervals for the counts given (TP = 18, FN = 2, TN = 70, FP = 30) and for the same proportions with the counts scaled by 0.4 and 4 (the label gives the factor), each a run of the model: more data, narrower intervals. Highlighted: the counts given, sensitivity 0.9 (0.7383 to 0.9663), specificity 0.7 (0.6202 to 0.7693). Your inputs: TP = 18, FN = 2, TN = 70, FP = 30, level = 0.9.*

![TP/(TP + FN) as TP runs from 0 to 38, every other input as you entered it (39 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-sensitivity-specificity-ci-live-1)

*TP/(TP + FN) as TP runs from 0 to 38, every other input as you entered it (39 solutions). Marked: your TP = 18 gives sensitivity = 0.9. Your inputs: TP = 18, FN = 2, TN = 70, FP = 30, level = 0.9.*

![Lower Wilson limit as FP runs from 10 to 50, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-sensitivity-specificity-ci-live-2)

*Lower Wilson limit as FP runs from 10 to 50, every other input as you entered it (21 solutions). Marked: your FP = 30 gives spec_lower = 0.6202. Your inputs: TP = 18, FN = 2, TN = 70, FP = 30, level = 0.9.*

```python
biomedical_science_ai999x.solve('bmed_dx_sensitivity_specificity_ci', TP=45, FN=5, TN=90, FP=10, level=0.95)
# -> sensitivity = 0.9, sens_lower = 0.786398, sens_upper = 0.956524, specificity = 0.9, spec_lower = 0.825634, spec_upper = 0.944771   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dx_sensitivity_specificity_ci', TP=18, FN=2, TN=70, FP=30, level=0.9)
# -> sensitivity = 0.9, sens_lower = 0.738337, sens_upper = 0.966337, specificity = 0.7, spec_lower = 0.620168, spec_upper = 0.769295   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Sensitivity and specificity intervals: TP = 45, FN = 5, TN = 90, FP = 10, level = 95% | SOLVED | sensitivity = 0.9, sens_lower = 0.786398, sens_upper = 0.956524, specificity = 0.9, spec_lower = 0.825634, spec_upper = 0.944771 |
| Sensitivity and specificity intervals: TP = 18, FN = 2, TN = 70, FP = 30, level = 0.9 | SOLVED | sensitivity = 0.9, sens_lower = 0.738337, sens_upper = 0.966337, specificity = 0.7, spec_lower = 0.620168, spec_upper = 0.769295 |
| Sensitivity and specificity intervals: FN = 5, TN = 90, FP = 10, level = 95% | NEEDS_INPUT | bmed_dx_sensitivity_specificity_ci needs TP (true positives). Example: solve('bmed_dx_sensitivity_specificity_ci', TP=45 |
| Sensitivity and specificity intervals: TP = 45, FN = 5, TN = 90, FP = 10, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Sensitivity and specificity intervals: TP = 45, FN = 5, TN = 90, FP = 10 | NEEDS_INPUT | bmed_dx_sensitivity_specificity_ci needs level (confidence level (95% or 0.95)). Example: solve('bmed_dx_sensitivity_spe |

<a id="bmed_odds_ratio_ci"></a>

### Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval, `bmed_odds_ratio_ci`

The odds ratio of an exposure-outcome 2x2 table (case-control or cohort) with the Woolf logit confidence interval at the stated level.

$$
OR = \frac{ad}{bc},\quad \mathrm{SE}(\ln OR) = \sqrt{\tfrac1a + \tfrac1b + \tfrac1c + \tfrac1d}
$$

| input | meaning | condition |
|---|---|---|
| `a` | exposed with the outcome |  |
| `b` | exposed without the outcome |  |
| `c` | unexposed with the outcome |  |
| `d` | unexposed without the outcome |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `OR` | odds ratio ad/(bc) |
| `lower` | lower confidence limit |
| `upper` | upper confidence limit |

- **Method:** $OR = ad/(bc)$; SE(ln OR) = sqrt(1/a + 1/b + 1/c + 1/d)
- **Assumes:** no zero cell (no continuity correction); Woolf interval exp(ln OR +- z SE); level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Forest plot of the 2 x 2 table given (a = 45, b = 55, c = 30, d = 120): the odds ratio (this capability) and the relative risk (the model's relative-risk capability) with their 90 %, 95 % and 99 % intervals, each a run of the model](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-odds-ratio-ci-forest.svg)

*Forest plot of the 2 x 2 table given (a = 45, b = 55, c = 30, d = 120): the odds ratio (this capability) and the relative risk (the model's relative-risk capability) with their 90 %, 95 % and 99 % intervals, each a run of the model. An interval that crosses 1 (grey) is compatible with no effect. Highlighted: OR 3.273 (1.565 to 6.845) at the level given. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.99.*

![The 2 x 2 table given (rows: exposed, unexposed; columns: with, without the outcome): a = 45, b = 55, c = 30, d = 120](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-odds-ratio-ci-table)

*The 2 x 2 table given (rows: exposed, unexposed; columns: with, without the outcome): a = 45, b = 55, c = 30, d = 120. Returned: odds ratio ad/(bc) = 3.273, 99 % interval 1.565 to 6.845. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.99.*

![Odds ratio ad/(bc) as c runs from 10 to 50, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-odds-ratio-ci-live-1)

*Odds ratio ad/(bc) as c runs from 10 to 50, every other input as you entered it (41 solutions). Marked: your c = 30 gives OR = 3.273. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.99.*

![Odds ratio ad/(bc) as a runs from 25 to 65, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-odds-ratio-ci-live-2)

*Odds ratio ad/(bc) as a runs from 25 to 65, every other input as you entered it (21 solutions). Marked: your a = 45 gives OR = 3.273. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.99.*

```python
biomedical_science_ai999x.solve('bmed_odds_ratio_ci', a=20, b=80, c=10, d=90, level=0.95)
# -> OR = 2.25, lower = 0.994295, upper = 5.09155   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_odds_ratio_ci', a=45, b=55, c=30, d=120, level=0.99)
# -> OR = 3.27273, lower = 1.56471, upper = 6.84519   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Odds ratio with confidence interval: a = 20, b = 80, c = 10, d = 90, level = 95% | SOLVED | OR = 2.25, lower = 0.994295, upper = 5.09155 |
| Odds ratio with confidence interval: a = 45, b = 55, c = 30, d = 120, level = 0.99 | SOLVED | OR = 3.27273, lower = 1.56471, upper = 6.84519 |
| Odds ratio with confidence interval: b = 80, c = 10, d = 90, level = 95% | NEEDS_INPUT | bmed_odds_ratio_ci needs a (exposed with the outcome). Example: solve('bmed_odds_ratio_ci', a=20, b=80, c=10, d=90, leve |
| Odds ratio with confidence interval: a = 20, b = 80, c = 10, d = 90, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Odds ratio with confidence interval: a = 20, b = 0, c = 10, d = 90, level = 95% | INVALID_INPUT | b: must be at least 1, got 0. |

<a id="bmed_relative_risk_ci"></a>

### Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval, `bmed_relative_risk_ci`

The risk ratio of exposed to unexposed in a cohort 2x2 table with the log-scale (Katz) confidence interval at the stated level.

$$
RR = \frac{a/(a+b)}{c/(c+d)},\quad \mathrm{SE}(\ln RR) = \sqrt{\tfrac1a - \tfrac1{a+b} + \tfrac1c - \tfrac1{c+d}}
$$

| input | meaning | condition |
|---|---|---|
| `a` | exposed with the outcome |  |
| `b` | exposed without the outcome |  |
| `c` | unexposed with the outcome |  |
| `d` | unexposed without the outcome |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `RR` | risk ratio [a/(a + b)]/[c/(c + d)] |
| `lower` | lower confidence limit |
| `upper` | upper confidence limit |

- **Method:** RR = [a/(a + b)]/[c/(c + d)]; SE(ln RR) = sqrt($1/a - 1/(a + b) + 1/c - 1/(c + d)$)
- **Assumes:** cohort (risks estimable); no zero cell; log interval exp(ln RR +- z SE); level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Risk of the outcome in the exposed (45 of 100) and unexposed (30 of 150) groups given (risks = events / group size, derived from the inputs), and the model's relative risk 2](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-relative-risk-ci-risk)

*Risk of the outcome in the exposed (45 of 100) and unexposed (30 of 150) groups given (risks = events / group size, derived from the inputs), and the model's relative risk 2.25 with its 90 % interval 1.627 to 3.112. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.9.*

![Risk ratio [a/(a + b)]/[c/(c + d)] as c runs from 10 to 50, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-relative-risk-ci-live-1)

*Risk ratio [a/(a + b)]/[c/(c + d)] as c runs from 10 to 50, every other input as you entered it (41 solutions). Marked: your c = 30 gives RR = 2.25. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.9.*

![Risk ratio [a/(a + b)]/[c/(c + d)] as a runs from 25 to 65, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-relative-risk-ci-live-2)

*Risk ratio [a/(a + b)]/[c/(c + d)] as a runs from 25 to 65, every other input as you entered it (21 solutions). Marked: your a = 45 gives RR = 2.25. Your inputs: a = 45, b = 55, c = 30, d = 120, level = 0.9.*

```python
biomedical_science_ai999x.solve('bmed_relative_risk_ci', a=20, b=80, c=10, d=90, level=0.95)
# -> RR = 2, lower = 0.986563, upper = 4.05448   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_relative_risk_ci', a=45, b=55, c=30, d=120, level=0.9)
# -> RR = 2.25, lower = 1.62671, upper = 3.11211   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Relative risk with confidence interval: a = 20, b = 80, c = 10, d = 90, level = 95% | SOLVED | RR = 2, lower = 0.986563, upper = 4.05448 |
| Relative risk with confidence interval: a = 45, b = 55, c = 30, d = 120, level = 0.9 | SOLVED | RR = 2.25, lower = 1.62671, upper = 3.11211 |
| Relative risk with confidence interval: b = 80, c = 10, d = 90, level = 95% | NEEDS_INPUT | bmed_relative_risk_ci needs a (exposed with the outcome). Example: solve('bmed_relative_risk_ci', a=20, b=80, c=10, d=90 |
| Relative risk with confidence interval: a = 20, b = 80, c = 10, d = 90, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_nnt_ci"></a>

### Absolute risk reduction and number needed to treat with Wald intervals, `bmed_nnt_ci`

The absolute risk reduction (control risk minus treated risk) of a two-arm trial with its Wald confidence interval, and the number needed to treat with the interval obtained by inverting it.

$$
ARR = p_c - p_t,\quad NNT = \frac{1}{ARR},\quad \mathrm{SE} = \sqrt{\tfrac{p_t(1-p_t)}{n_t} + \tfrac{p_c(1-p_c)}{n_c}}
$$

| input | meaning | condition |
|---|---|---|
| `events_t` | events in the treated group |  |
| `n_t` | size of the treated group |  |
| `events_c` | events in the control group |  |
| `n_c` | size of the control group |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `ARR` | absolute risk reduction |
| `ARR_lower` | lower limit |
| `ARR_upper` | upper limit |
| `NNT` | number needed to treat 1/ARR |
| `NNT_lower` | 1/ARR_upper |
| `NNT_upper` | 1/ARR_lower |

- **Method:** ARR = pc - pt; $SE = sqrt$(pt(1 - pt)/nt + pc(1 - pc)/nc); NNT = 1/ARR with limits 1/upper, 1/lower
- **Assumes:** unpooled Wald interval for the difference of proportions; the interval excludes zero (a finite NNT interval); level stated; NNT not rounded
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Event risk in the treated (40 of 400) and control (80 of 410) groups given (risks derived from the inputs) and the model's absolute risk reduction ARR = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-nnt-ci-risk)

*Event risk in the treated (40 of 400) and control (80 of 410) groups given (risks derived from the inputs) and the model's absolute risk reduction ARR = 0.09512 with its 90 % interval 0.05456 to 0.1357. Returned: number needed to treat NNT = 1/ARR = 10.51 (interval 7.37 to 18.33). Your inputs: events_t = 40, n_t = 400, events_c = 80, n_c = 410, level = 0.9.*

![Number needed to treat 1/ARR as events_t runs from 20 to 60, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-nnt-ci-live-1)

*Number needed to treat 1/ARR as events_t runs from 20 to 60, every other input as you entered it (41 solutions). Marked: your events_t = 40 gives NNT = 10.51. Your inputs: events_t = 40, n_t = 400, events_c = 80, n_c = 410, level = 0.9.*

![1/ARR_lower as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-nnt-ci-live-2)

*1/ARR_lower as level runs from 0.5143 to 0.9964, every other input as you entered it (29 solutions). Marked: your level = 0.9 gives NNT_upper = 18.33. Your inputs: events_t = 40, n_t = 400, events_c = 80, n_c = 410, level = 0.9.*

```python
biomedical_science_ai999x.solve('bmed_nnt_ci', events_t=15, n_t=100, events_c=30, n_c=100, level=0.95)
# -> ARR = 0.15, ARR_lower = 0.0361364, ARR_upper = 0.263864, NNT = 6.66667, NNT_lower = 3.78984, NNT_upper = 27.6729   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_nnt_ci', events_t=40, n_t=400, events_c=80, n_c=410, level=0.9)
# -> ARR = 0.095122, ARR_lower = 0.0545621, ARR_upper = 0.135682, NNT = 10.5128, NNT_lower = 7.37019, NNT_upper = 18.3277   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Number needed to treat: events_t = 15, n_t = 100, events_c = 30, n_c = 100, level = 95% | SOLVED | ARR = 0.15, ARR_lower = 0.0361364, ARR_upper = 0.263864, NNT = 6.66667, NNT_lower = 3.78984, NNT_upper = 27.6729 |
| Number needed to treat: events_t = 40, n_t = 400, events_c = 80, n_c = 410, level = 0.9 | SOLVED | ARR = 0.095122, ARR_lower = 0.0545621, ARR_upper = 0.135682, NNT = 10.5128, NNT_lower = 7.37019, NNT_upper = 18.3277 |
| Number needed to treat: n_t = 100, events_c = 30, n_c = 100, level = 95% | NEEDS_INPUT | bmed_nnt_ci needs events_t (events in the treated group). Example: solve('bmed_nnt_ci', events_t=15, n_t=100, events_c=3 |
| Number needed to treat: events_t = 15, n_t = 100, events_c = 30, n_c = 100, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Number needed to treat: events_t = 28, n_t = 100, events_c = 30, n_c = 100, level = 95% | INVALID_INPUT | Absolute risk reduction and number needed to treat with Wald intervals: the interval for the risk reduction does not exc |

<a id="bmed_incidence_rate_ratio_ci"></a>

### Incidence rate ratio of two person-time groups with log-Wald interval, `bmed_incidence_rate_ratio_ci`

The ratio of the incidence rates (events per person-time) of two groups with the log-scale Wald confidence interval at the stated level.

$$
IRR = \frac{e_1/T_1}{e_2/T_2},\quad \mathrm{SE}(\ln IRR) = \sqrt{\tfrac1{e_1} + \tfrac1{e_2}}
$$

| input | meaning | condition |
|---|---|---|
| `events1` | events in group 1 |  |
| `time1` | person-time of group 1 (the same unit for both groups) |  |
| `events2` | events in group 2 (reference) |  |
| `time2` | person-time of group 2 |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `IRR` | incidence rate ratio group 1 / group 2 |
| `lower` | lower limit |
| `upper` | upper limit |

- **Method:** IRR = (e1/T1)/(e2/T2); SE(ln IRR) = sqrt(1/e1 + 1/e2)
- **Assumes:** Poisson counts; log-Wald interval; the same person-time unit for both groups; level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The incidence rate ratio with its 90 % interval for the counts given (events1 = 12, time1 = 2500, events2 = 40, time2 = 4100) and the same proportions with the counts scaled by 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-incidence-rate-ratio-ci-intervals)

*The incidence rate ratio with its 90 % interval for the counts given (events1 = 12, time1 = 2500, events2 = 40, time2 = 4100) and the same proportions with the counts scaled by 0.4, 2 and 4, each a run of the model: more data, narrower intervals. Highlighted: the counts given, IRR 0.4919 (0.2863 to 0.8453). Your inputs: events1 = 12, time1 = 2500.5, events2 = 40, time2 = 4100, level = 0.9.*

![IRR as events1 runs from 1 to 32, every other input as you entered it (32 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-incidence-rate-ratio-ci-live-1)

*IRR as events1 runs from 1 to 32, every other input as you entered it (32 solutions). Marked: your events1 = 12 gives IRR = 0.4919. Your inputs: events1 = 12, time1 = 2500.5, events2 = 40, time2 = 4100, level = 0.9.*

![IRR as events2 runs from 20 to 60, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-incidence-rate-ratio-ci-live-2)

*IRR as events2 runs from 20 to 60, every other input as you entered it (21 solutions). Marked: your events2 = 40 gives IRR = 0.4919. Your inputs: events1 = 12, time1 = 2500.5, events2 = 40, time2 = 4100, level = 0.9.*

```python
biomedical_science_ai999x.solve('bmed_incidence_rate_ratio_ci', events1=30, time1=1000, events2=15, time2=1200, level=0.95)
# -> IRR = 2.4, lower = 1.29133, upper = 4.46051   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_incidence_rate_ratio_ci', events1=12, time1=2500.5, events2=40, time2=4100, level=0.9)
# -> IRR = 0.491902, lower = 0.286257, upper = 0.845279   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Incidence rate ratio: events1 = 30, time1 = 1000, events2 = 15, time2 = 1200, level = 95% | SOLVED | IRR = 2.4, lower = 1.29133, upper = 4.46051 |
| Incidence rate ratio: events1 = 12, time1 = 2500.5, events2 = 40, time2 = 4100, level = 0.9 | SOLVED | IRR = 0.491902, lower = 0.286257, upper = 0.845279 |
| Incidence rate ratio: time1 = 1000, events2 = 15, time2 = 1200, level = 95% | NEEDS_INPUT | bmed_incidence_rate_ratio_ci needs events1 (events in group 1). Example: solve('bmed_incidence_rate_ratio_ci', events1=3 |
| Incidence rate ratio: events1 = 30, time1 = 1000, events2 = 15, time2 = 1200, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_smr_exact_ci"></a>

### Standardized mortality (incidence) ratio with exact Poisson interval, `bmed_smr_exact_ci`

The standardized mortality or incidence ratio observed/expected with the exact (Garwood) Poisson confidence interval at the stated level.

$$
\mathrm{SMR} = \frac{O}{E},\quad P(X \ge O;\lambda_L) = \tfrac{\alpha}{2},\ P(X \le O;\lambda_U) = \tfrac{\alpha}{2}
$$

| input | meaning | condition |
|---|---|---|
| `observed` | observed number of deaths or cases (0 to 1000) |  |
| `expected` | expected number from reference rates |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `SMR` | observed / expected |
| `lower` | lower exact limit |
| `upper` | upper exact limit |

- **Method:** limits lambda with P($X  \geq  O$; lambda) = alpha/2 and P($X  \leq  O$; lambda) = alpha/2 (bisection on the Poisson sums), divided by E
- **Assumes:** observed count Poisson; expected count treated as known; exact (Garwood) limits; level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The standardised mortality ratio with its exact 90 % interval for the counts given (observed = 3, expected = 7](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-smr-exact-ci-intervals)

*The standardised mortality ratio with its exact 90 % interval for the counts given (observed = 3, expected = 7.5) and the same proportions with the counts scaled by 0.4, 2 and 4, each a run of the model: more data, narrower intervals. Highlighted: the counts given, SMR 0.4 (0.109 to 1.034). Your inputs: observed = 3, expected = 7.5, level = 0.9.*

![SMR as observed runs from 0 to 7, every other input as you entered it (8 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-smr-exact-ci-live-1)

*SMR as observed runs from 0 to 7, every other input as you entered it (8 solutions). Marked: your observed = 3 gives SMR = 0.4. Your inputs: observed = 3, expected = 7.5, level = 0.9.*

![Lower exact limit as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-smr-exact-ci-live-2)

*Lower exact limit as level runs from 0.5124 to 0.9984, every other input as you entered it (33 solutions). Marked: your level = 0.9 gives lower = 0.109. Your inputs: observed = 3, expected = 7.5, level = 0.9.*

```python
biomedical_science_ai999x.solve('bmed_smr_exact_ci', observed=25, expected=18.4, level=0.95)
# -> SMR = 1.3587, lower = 0.879276, upper = 2.0057   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_smr_exact_ci', observed=3, expected=7.5, level=0.9)
# -> SMR = 0.4, lower = 0.109026, upper = 1.03382   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Standardized mortality ratio: observed = 25, expected = 18.4, level = 95% | SOLVED | SMR = 1.3587, lower = 0.879276, upper = 2.0057 |
| Standardized mortality ratio: observed = 3, expected = 7.5, level = 0.9 | SOLVED | SMR = 0.4, lower = 0.109026, upper = 1.03382 |
| Standardized mortality ratio: expected = 18.4, level = 95% | NEEDS_INPUT | bmed_smr_exact_ci needs observed (observed number of deaths or cases (0 to 1000)). Example: solve('bmed_smr_exact_ci', o |
| Standardized mortality ratio: observed = 25, expected = 18.4, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_mantel_haenszel_odds_ratio"></a>

### Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval, `bmed_mantel_haenszel_odds_ratio`

The Mantel-Haenszel odds ratio pooled over strata of 2x2 tables (adjusting for a confounder), with the Robins-Breslow-Greenland confidence interval at the stated level.

$$
OR_{MH} = \frac{\sum_i a_i d_i / n_i}{\sum_i b_i c_i / n_i}
$$

| input | meaning | condition |
|---|---|---|
| `tables` | one row [a, b, c, d] per stratum: exposed cases, exposed non-cases, unexposed cases, unexposed non-cases |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `OR_MH` | Mantel-Haenszel pooled odds ratio |
| `lower` | lower limit |
| `upper` | upper limit |

- **Method:** OR_MH = sum(a d / n) / sum(b c / n); variance of ln OR_MH by Robins, Breslow and Greenland
- **Assumes:** a common odds ratio across strata; Robins-Breslow-Greenland variance; level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Odds ratio of each stratum given (the model's odds ratio with its 95 % interval) and the Mantel-Haenszel pooled odds ratio over the 2 strata: stratum 1 [10, 20, 5, 30] OR 3 (0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mantel-haenszel-odds-ratio-forest)

*Odds ratio of each stratum given (the model's odds ratio with its 95 % interval) and the Mantel-Haenszel pooled odds ratio over the 2 strata: stratum 1 [10, 20, 5, 30] OR 3 (0.8915 to 10.1); stratum 2 [8, 12, 6, 20] OR 2.222 (0.6193 to 7.974). Highlighted: pooled OR_MH = 2.608 (1.084 to 6.273). Your inputs: tables = 10 20 5 30; 8 12 6 20, level = 0.95.*

![Upper limit as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mantel-haenszel-odds-ratio-live-1)

*Upper limit as level runs from 0.5082 to 0.9999, every other input as you entered it (37 solutions). Marked: your level = 0.95 gives upper = 6.273. Your inputs: tables = 10 20 5 30; 8 12 6 20, level = 0.95.*

```python
biomedical_science_ai999x.solve('bmed_mantel_haenszel_odds_ratio', tables=[[10, 20, 5, 30], [8, 12, 6, 20]], level=0.95)
# -> OR_MH = 2.60776, lower = 1.08405, upper = 6.27313   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mantel_haenszel_odds_ratio', tables=[[25, 75, 10, 90], [40, 60, 20, 80], [5, 45, 2, 48]], level=0.9)
# -> OR_MH = 2.78404, lower = 1.86901, upper = 4.14705   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Mantel-Haenszel odds ratio: tables = [[10, 20, 5, 30], [8, 12, 6, 20]], level = 95% | SOLVED | OR_MH = 2.60776, lower = 1.08405, upper = 6.27313 |
| Mantel-Haenszel odds ratio: tables = [[25, 75, 10, 90], [40, 60, 20, 80], [5, 45, 2, 48]], level = 0.9 | SOLVED | OR_MH = 2.78404, lower = 1.86901, upper = 4.14705 |
| Mantel-Haenszel odds ratio: level = 95% | NEEDS_INPUT | bmed_mantel_haenszel_odds_ratio needs tables (one row [a, b, c, d] per stratum: exposed cases, exposed non-cases, unexpo |
| Mantel-Haenszel odds ratio: tables = [[10, 20, 5, 30], [8, 12, 6, 20]], level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_cohens_kappa"></a>

### Cohen's kappa for agreement between two raters or two tests, `bmed_cohens_kappa`

Cohen's kappa (chance-corrected agreement) and the observed agreement of two raters or two diagnostic tests classifying the same subjects, from their k x k agreement table.

$$
\kappa = \frac{p_o - p_e}{1 - p_e},\quad p_e = \sum_i \frac{n_{i\cdot}\,n_{\cdot i}}{n^2}
$$

| input | meaning | condition |
|---|---|---|
| `counts` | agreement table: rows = categories given by rater A, columns = categories given by rater B |  |

| output | meaning |
|---|---|
| `kappa` | Cohen's kappa |
| `p_observed` | observed proportion of agreement |

- **Method:** $\kappa = (p_o - p_e)/(1 - p_e)$, p_e = sum of products of row and column margins / n^2
- **Assumes:** the same subjects rated by both; unweighted kappa; the same categories in rows and columns
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The agreement table given for two raters (100 subjects; rows: rater A, columns: rater B; agreements on the shaded diagonal)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cohens-kappa-table)

*The agreement table given for two raters (100 subjects; rows: rater A, columns: rater B; agreements on the shaded diagonal). Returned: observed agreement 0.85, Cohen's kappa 0.7 (agreement beyond chance). Your inputs: counts = 40 10; 5 45.*

![Cohen's kappa as counts[1,1] runs from 10 to 70, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cohens-kappa-live-1)

*Cohen's kappa as counts[1,1] runs from 10 to 70, every other input as you entered it (21 solutions). Values in this range where the inputs admit no answer (20 of 41) are left out. Marked: your counts[1,1] = 40 gives κ = 0.7. Your inputs: counts = 40 10; 5 45.*

![Cohen's kappa as counts[1,2] runs from 5 to 15, every other input as you entered it (3 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cohens-kappa-live-2)

*Cohen's kappa as counts[1,2] runs from 5 to 15, every other input as you entered it (3 solutions). Values in this range where the inputs admit no answer (12 of 15) are left out. Marked: your counts[1,2] = 10 gives κ = 0.7. Your inputs: counts = 40 10; 5 45.*

```python
biomedical_science_ai999x.solve('bmed_cohens_kappa', counts=[[40, 10], [5, 45]])
# -> kappa = 0.7, p_observed = 0.85   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_cohens_kappa', counts=[[20, 5, 1], [4, 30, 6], [0, 3, 31]])
# -> kappa = 0.710542, p_observed = 0.81   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cohen's kappa: counts = [[40, 10], [5, 45]] | SOLVED | kappa = 0.7, p_observed = 0.85 |
| Cohen's kappa: counts = [[20, 5, 1], [4, 30, 6], [0, 3, 31]] | SOLVED | kappa = 0.710542, p_observed = 0.81 |
| Cohen's kappa:  | NEEDS_INPUT | bmed_cohens_kappa needs counts (agreement table: rows = categories given by rater A, columns = categories given by rater |
| Cohen's kappa: counts = [[40, 10], [5, 45]], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_km_survival_median"></a>

### Kaplan-Meier survival estimate at a time and median survival, `bmed_km_survival_median`

The Kaplan-Meier (product-limit) estimate of the survival probability at time t and the median survival time from follow-up times with event or censoring indicators.

$$
\hat S(t) = \prod_{u_i \le t}\left(1 - \frac{d_i}{n_i}\right)
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times (one time unit; results in that unit) |  |
| `events` | 1 = event observed, 0 = censored, one per time |  |
| `t` | time at which S(t) is reported |  |

| output | meaning |
|---|---|
| `S_t` | Kaplan-Meier survival probability at t |
| `median` | median survival: first time with $S  \leq  0.5$ (none when not reached) |

- **Method:** S(t) = product over event times $u  \leq  t$ of $(1 - d/n)$, exact fractions; median = first event time with $S  \leq  1/2$
- **Assumes:** non-informative censoring; a censoring at an event time counts as at risk at that time
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The 21 subjects given as follow-up lines from 0 to their time (dot: event observed, 9; bar end: censored, 12), the data the Kaplan-Meier estimate is computed from](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-km-survival-median-follow-up.svg)

*The 21 subjects given as follow-up lines from 0 to their time (dot: event observed, 9; bar end: censored, 12), the data the Kaplan-Meier estimate is computed from. Marked: t = 12, where the model returns S = 0.7529, and the returned median survival 23. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12.*

![Kaplan-Meier survival estimate of the 21 subjects given (9 events, the rest censored), with its pointwise 95 % confidence band, both computed by the model at every step time](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-km-survival-median-survival)

*Kaplan-Meier survival estimate of the 21 subjects given (9 events, the rest censored), with its pointwise 95 % confidence band, both computed by the model at every step time. Marked: S(12) = 0.7529; median survival 23. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12.*

![Kaplan-Meier survival probability at t as t runs from 3 to 21, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-km-survival-median-live-1)

*Kaplan-Meier survival probability at t as t runs from 3 to 21, every other input as you entered it (41 solutions). Marked: your t = 12 gives S_t = 0.7529. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12.*

```python
biomedical_science_ai999x.solve('bmed_km_survival_median', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t=12)
# -> S_t = 0.752941, median = 23   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_km_survival_median', times=[2, 3, 3, 5, 8, 9, 12, 15], events=[1, 0, 1, 1, 0, 1, 1, 0], t=6)
# -> S_t = 0.6, median = 9   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Kaplan-Meier survival: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12 | SOLVED | S_t = 0.752941, median = 23 |
| Kaplan-Meier survival: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], t = 6 | SOLVED | S_t = 0.6, median = 9 |
| Kaplan-Meier survival: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12 | NEEDS_INPUT | bmed_km_survival_median needs times (follow-up times (one time unit; results in that unit)). Example: solve('bmed_km_sur |
| Kaplan-Meier survival: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Kaplan-Meier survival: times = [6, 7, 10], events = [1, 2, 1], t = 8 | INVALID_INPUT | Kaplan-Meier survival estimate at a time and median survival: event indicators are 1 (event) or 0 (censored). |

<a id="bmed_km_confidence_interval"></a>

### Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval, `bmed_km_confidence_interval`

The Kaplan-Meier survival estimate at time t with its confidence interval on the log(-log) scale (Greenwood variance; the exponential Greenwood interval) at the stated level.

$$
\hat S^{\exp\left(\pm z\,\sqrt{V}/|\ln\hat S|\right)},\quad V = \sum_{u_i \le t}\frac{d_i}{n_i(n_i - d_i)}
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times (one time unit) |  |
| `events` | 1 = event, 0 = censored |  |
| `t` | time at which S(t) is reported |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `S_t` | Kaplan-Meier estimate at t |
| `lower` | lower confidence limit |
| `upper` | upper confidence limit |

- **Method:** Greenwood sum V = sum d/(n(n - d)); limits $S^{exp}$(+-z sqrt(V)/|ln S|)
- **Assumes:** non-informative censoring; an event at or before t and S(t) > 0; level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kaplan-Meier estimate of the 21 follow-up times given with its pointwise 80 % and 99 % confidence bands, both computed by the model at every step time](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-km-confidence-interval-survival)

*Kaplan-Meier estimate of the 21 follow-up times given with its pointwise 80 % and 99 % confidence bands, both computed by the model at every step time. Marked: at t = 12 the 95 % interval 0.5032 to 0.8894 around S = 0.7529. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12, level = 0.95.*

![Kaplan-Meier estimate at t as t runs from 6](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-km-confidence-interval-live-1)

*Kaplan-Meier estimate at t as t runs from 6.15 to 21, every other input as you entered it (34 solutions). Marked: your t = 12 gives S_t = 0.7529. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12, level = 0.95.*

![Lower confidence limit as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-km-confidence-interval-live-2)

*Lower confidence limit as level runs from 0.5174 to 0.9754, every other input as you entered it (21 solutions). Marked: your level = 0.95 gives lower = 0.5032. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12, level = 0.95.*

```python
biomedical_science_ai999x.solve('bmed_km_confidence_interval', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t=12, level=0.95)
# -> S_t = 0.752941, lower = 0.5032, upper = 0.889362   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_km_confidence_interval', times=[2, 3, 3, 5, 8, 9, 12, 15], events=[1, 0, 1, 1, 0, 1, 1, 0], t=8, level=0.9)
# -> S_t = 0.6, lower = 0.258167, upper = 0.82473   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Kaplan-Meier confidence interval: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12, level = 95% | SOLVED | S_t = 0.752941, lower = 0.5032, upper = 0.889362 |
| Kaplan-Meier confidence interval: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], t = 8, level = 0.9 | SOLVED | S_t = 0.6, lower = 0.258167, upper = 0.82473 |
| Kaplan-Meier confidence interval: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12, level = 95% | NEEDS_INPUT | bmed_km_confidence_interval needs times (follow-up times (one time unit)). Example: solve('bmed_km_confidence_interval', |
| Kaplan-Meier confidence interval: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12, level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Kaplan-Meier confidence interval: times = [2, 3, 5], events = [1, 1, 1], t = 1, level = 95% | INVALID_INPUT | Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval: no event at or before t: S(t) = 1 and  |

<a id="bmed_nelson_aalen_hazard"></a>

### Nelson-Aalen cumulative hazard at a time: `bmed_nelson_aalen_hazard`

The Nelson-Aalen estimate of the cumulative hazard at time t from follow-up times with event or censoring indicators.

$$
\hat H(t) = \sum_{u_i \le t}\frac{d_i}{n_i}
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times (one time unit) |  |
| `events` | 1 = event, 0 = censored |  |
| `t` | time at which H(t) is reported |  |

| output | meaning |
|---|---|
| `H_t` | cumulative hazard at t |

- **Method:** H(t) = sum over event times $u  \leq  t$ of d/n
- **Assumes:** non-informative censoring; tied events contribute d/n (no tie smoothing)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Nelson-Aalen cumulative hazard of the 21 follow-up times given (9 events; ticks: censored), the model's estimate at every step time: each event adds 1/(number at risk)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-nelson-aalen-hazard-survival)

*Nelson-Aalen cumulative hazard of the 21 follow-up times given (9 events; ticks: censored), the model's estimate at every step time: each event adds 1/(number at risk). Marked: H(12) = 0.2683. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12.*

![Cumulative hazard at t as t runs from 3 to 21, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-nelson-aalen-hazard-live-1)

*Cumulative hazard at t as t runs from 3 to 21, every other input as you entered it (41 solutions). Marked: your t = 12 gives H_t = 0.2683. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., t = 12.*

```python
biomedical_science_ai999x.solve('bmed_nelson_aalen_hazard', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t=12)
# -> H_t = 0.268347   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_nelson_aalen_hazard', times=[2, 3, 3, 5, 8, 9, 12, 15], events=[1, 0, 1, 1, 0, 1, 1, 0], t=10)
# -> H_t = 0.80119   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Nelson-Aalen cumulative hazard: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12 | SOLVED | H_t = 0.268347 |
| Nelson-Aalen cumulative hazard: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], t = 10 | SOLVED | H_t = 0.80119 |
| Nelson-Aalen cumulative hazard: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12 | NEEDS_INPUT | bmed_nelson_aalen_hazard needs times (follow-up times (one time unit)). Example: solve('bmed_nelson_aalen_hazard', times |
| Nelson-Aalen cumulative hazard: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], t = 12, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_logrank_test"></a>

### Log-rank test comparing two survival curves, `bmed_logrank_test`

The log-rank (Mantel-Cox) chi-square statistic and p-value for equal survival in two groups from follow-up times and event indicators.

$$
\chi^2 = \frac{\left(\sum_i (d_{1i} - e_{1i})\right)^2}{\sum_i v_i}
$$

| input | meaning | condition |
|---|---|---|
| `times1` | follow-up times, group 1 |  |
| `events1` | 1 = event, 0 = censored, group 1 |  |
| `times2` | follow-up times, group 2 |  |
| `events2` | 1 = event, 0 = censored, group 2 |  |

| output | meaning |
|---|---|
| `chi_square` | log-rank chi-square (1 degree of freedom) |
| `p_value` | two-sided p-value |

- **Method:** at each event time: expected events of group 1 and the hypergeometric variance; $chi2 = (O_{1} - E_{1})^{2} / V$
- **Assumes:** non-informative censoring; hypergeometric variance at each event time
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kaplan-Meier curves of the two groups given (9 and 10 subjects; ticks mark censored times), every step the model's survival estimate](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-logrank-test-survival)

*Kaplan-Meier curves of the two groups given (9 and 10 subjects; ticks mark censored times), every step the model's survival estimate. Returned: log-rank chi-square = 13.85 (1 degree of freedom), p = 0.000198. Your inputs: times1 = 6, 6, 6, 7, 10, 13, 16..., events1 = 1, 1, 1, 1, 1, 1, 1, 1, 1, times2 = 1, 1, 2, 2, 3, 4, 4, 5, 5, 8, events2 = 1, 1, 1, 1, 1, 1, 1, 1, 1, 1.*

```python
biomedical_science_ai999x.solve('bmed_logrank_test', times1=[6, 6, 6, 7, 10, 13, 16, 22, 23], events1=[1, 1, 1, 1, 1, 1, 1, 1, 1], times2=[1, 1, 2, 2, 3, 4, 4, 5, 5, 8], events2=[1, 1, 1, 1, 1, 1, 1, 1, 1, 1])
# -> chi_square = 13.85, p_value = 0.000197999   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_logrank_test', times1=[5, 8, 12, 20, 25], events1=[1, 1, 0, 1, 0], times2=[3, 4, 9, 10, 14], events2=[1, 1, 1, 0, 1])
# -> chi_square = 1.3895, p_value = 0.238489   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Log-rank test: times1 = [6, 6, 6, 7, 10, 13, 16, 22, 23], events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [1, 1, 2, 2, 3, 4, 4, 5, 5, 8], events2 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1] | SOLVED | chi_square = 13.85, p_value = 0.000197999 |
| Log-rank test: times1 = [5, 8, 12, 20, 25], events1 = [1, 1, 0, 1, 0], times2 = [3, 4, 9, 10, 14], events2 = [1, 1, 1, 0, 1] | SOLVED | chi_square = 1.3895, p_value = 0.238489 |
| Log-rank test: events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [1, 1, 2, 2, 3, 4, 4, 5, 5, 8], events2 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1] | NEEDS_INPUT | bmed_logrank_test needs times1 (follow-up times, group 1). Example: solve('bmed_logrank_test', times1=[6, 6, 6, 7, 10, 1 |
| Log-rank test: times1 = [6, 6, 6, 7, 10, 13, 16, 22, 23], events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [1, 1, 2, 2, 3, 4, 4, 5, 5, 8], events2 = [1, 1, 1, 1, 1, 1, 1, 1, 1, 1], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_cox_hazard_ratio"></a>

### Cox proportional-hazards model with one covariate: hazard ratio (Efron ties), `bmed_cox_hazard_ratio`

The log hazard ratio, hazard ratio, its standard error and Wald confidence interval of a Cox proportional-hazards model with a single covariate, by maximum partial likelihood with Efron's handling of ties.

$$
\ell(\beta) = \sum_j\left[\beta\sum_{i\in D_j}x_i - \sum_{l=0}^{d_j-1}\ln\left(\sum_{R_j}e^{\beta x} - \tfrac{l}{d_j}\sum_{D_j}e^{\beta x}\right)\right]
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times |  |
| `events` | 1 = event, 0 = censored |  |
| `x` | covariate value of each subject (e.g. 1 = treated, 0 = control) |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `beta` | log hazard ratio per unit of x |
| `HR` | hazard ratio exp(beta) |
| `se` | standard error of beta |
| `HR_lower` | lower Wald limit of the hazard ratio |
| `HR_upper` | upper Wald limit of the hazard ratio |

- **Method:** Newton-Raphson on the Efron partial log-likelihood (step halving); SE from the observed information
- **Assumes:** proportional hazards; Efron approximation for tied event times; a finite maximum (no monotone likelihood); level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kaplan-Meier curves of the two covariate groups given (21 subjects; ticks mark censored times), each step the model's survival estimate for that group](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cox-hazard-ratio-survival)

*Kaplan-Meier curves of the two covariate groups given (21 subjects; ticks mark censored times), each step the model's survival estimate for that group. The proportional-hazards fit returns HR = 1.048 per unit of x (95 % interval 0.2799 to 3.928, beta = 0.04732, se 0.6739). Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., x = 1, 0, 1, 0, 1, 0, 1, 0..., level = 0.95.*

![Lower Wald limit of the hazard ratio as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cox-hazard-ratio-live-1)

*Lower Wald limit of the hazard ratio as level runs from 0.5082 to 0.9999, every other input as you entered it (37 solutions). Marked: your level = 0.95 gives HR_lower = 0.2799. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., x = 1, 0, 1, 0, 1, 0, 1, 0..., level = 0.95.*

```python
biomedical_science_ai999x.solve('bmed_cox_hazard_ratio', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], x=[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], level=0.95)
# -> beta = 0.0473228, HR = 1.04846, se = 0.673876, HR_lower = 0.279864, HR_upper = 3.92787   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_cox_hazard_ratio', times=[2, 3, 3, 5, 8, 9, 12, 15, 4, 7, 11, 14, 18, 21], events=[1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0], x=[1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0], level=0.9)
# -> beta = 0.690016, HR = 1.99375, se = 0.737627, HR_lower = 0.592578, HR_upper = 6.70803   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cox hazard ratio: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], x = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], level = 95% | SOLVED | beta = 0.0473228, HR = 1.04846, se = 0.673876, HR_lower = 0.279864, HR_upper = 3.92787 |
| Cox hazard ratio: times = [2, 3, 3, 5, 8, 9, 12, 15, 4, 7, 11, 14, 18, 21], events = [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0], x = [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0], level = 0.9 | SOLVED | beta = 0.690016, HR = 1.99375, se = 0.737627, HR_lower = 0.592578, HR_upper = 6.70803 |
| Cox hazard ratio: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], x = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], level = 95% | NEEDS_INPUT | bmed_cox_hazard_ratio needs times (follow-up times). Example: solve('bmed_cox_hazard_ratio', times=[6, 6, 6, 7, 10, 13,  |
| Cox hazard ratio: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], x = [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_exponential_survival_fit"></a>

### Exponential survival model: hazard rate, mean and median survival (maximum likelihood), `bmed_exponential_survival_fit`

The maximum-likelihood constant hazard of an exponential survival model from censored follow-up data, with the implied mean and median survival times.

$$
\hat\lambda = \frac{d}{\sum_i t_i},\quad \tilde t = \frac{\ln 2}{\hat\lambda}
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times, all positive (one time unit) |  |
| `events` | 1 = event, 0 = censored |  |

| output | meaning |
|---|---|
| `hazard` | events per unit time |
| `mean` | mean survival 1/hazard |
| `median` | median survival ln 2 / hazard |

- **Method:** hazard = (number of events) / (total follow-up time)
- **Assumes:** constant hazard; non-informative censoring; at least one event
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Exponential survival fitted to the 21 follow-up times given (9 events; ticks: censored): the returned hazard lambda = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-exponential-survival-fit-survival)

*Exponential survival fitted to the 21 follow-up times given (9 events; ticks: censored): the returned hazard lambda = 0.02507 per time unit gives the dashed curve exp(-lambda t) (derived), beside the model's Kaplan-Meier steps. Marked: median ln2/lambda = 27.65; mean 1/lambda = 39.89. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1....*

```python
biomedical_science_ai999x.solve('bmed_exponential_survival_fit', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0])
# -> hazard = 0.0250696, mean = 39.8889, median = 27.6489   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_exponential_survival_fit', times=[2.5, 3.1, 4.0, 7.2, 8.8], events=[1, 1, 0, 1, 0])
# -> hazard = 0.117188, mean = 8.53333, median = 5.91486   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Exponential survival fit: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] | SOLVED | hazard = 0.0250696, mean = 39.8889, median = 27.6489 |
| Exponential survival fit: times = [2.5, 3.1, 4, 7.2, 8.8], events = [1, 1, 0, 1, 0] | SOLVED | hazard = 0.117188, mean = 8.53333, median = 5.91486 |
| Exponential survival fit: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0] | NEEDS_INPUT | bmed_exponential_survival_fit needs times (follow-up times, all positive (one time unit)). Example: solve('bmed_exponent |
| Exponential survival fit: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_exponential_hazard_ratio"></a>

### Hazard ratio of two exponential survival groups with Wald interval, `bmed_exponential_hazard_ratio`

The ratio of the maximum-likelihood constant hazards of two groups (group 1 relative to group 2) with the log-scale Wald confidence interval.

$$
HR = \frac{d_1/\sum t_{1}}{d_2/\sum t_{2}},\quad \mathrm{SE}(\ln HR) = \sqrt{\tfrac1{d_1} + \tfrac1{d_2}}
$$

| input | meaning | condition |
|---|---|---|
| `times1` | follow-up times, group 1 (positive) |  |
| `events1` | 1 = event, 0 = censored, group 1 |  |
| `times2` | follow-up times, group 2 (reference) |  |
| `events2` | 1 = event, 0 = censored, group 2 |  |
| `level` | confidence level (95% or 0.95) |  |

| output | meaning |
|---|---|
| `HR` | hazard ratio group 1 / group 2 |
| `lower` | lower limit |
| `upper` | upper limit |

- **Method:** hazard_g = d_g / sum t_g; SE(ln HR) = sqrt(1/d1 + 1/d2)
- **Assumes:** constant hazard in each group; at least one event per group; level stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Kaplan-Meier curves of the two groups given (9 and 12 subjects; ticks mark censored times), every step the model's survival estimate](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-exponential-hazard-ratio-survival)

*Kaplan-Meier curves of the two groups given (9 and 12 subjects; ticks mark censored times), every step the model's survival estimate. Returned: exponential hazard ratio (group 1 / group 2) 5.161, 95 % interval 1.589 to 16.76; dashed: exp(-lambda t) from each group's exponential hazard (0.08257 and 0.016, derived). Your inputs: times1 = 6, 6, 6, 7, 10, 13, 16..., events1 = 1, 1, 1, 1, 1, 1, 1, 1, 1, times2 = 6, 9, 10, 11, 17, 19..., events2 = 1, 0, 1, 0, 0, 1, 0, 0..., level = 0.95.*

![Lower limit as level runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-exponential-hazard-ratio-live-1)

*Lower limit as level runs from 0.5082 to 0.9999, every other input as you entered it (37 solutions). Marked: your level = 0.95 gives lower = 1.589. Your inputs: times1 = 6, 6, 6, 7, 10, 13, 16..., events1 = 1, 1, 1, 1, 1, 1, 1, 1, 1, times2 = 6, 9, 10, 11, 17, 19..., events2 = 1, 0, 1, 0, 0, 1, 0, 0..., level = 0.95.*

```python
biomedical_science_ai999x.solve('bmed_exponential_hazard_ratio', times1=[6, 6, 6, 7, 10, 13, 16, 22, 23], events1=[1, 1, 1, 1, 1, 1, 1, 1, 1], times2=[6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events2=[1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], level=0.95)
# -> HR = 5.16055, lower = 1.58923, upper = 16.7573   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_exponential_hazard_ratio', times1=[5, 8, 12, 20, 25], events1=[1, 1, 0, 1, 0], times2=[3, 4, 9, 10, 14], events2=[1, 1, 1, 0, 1], level=0.9)
# -> HR = 0.428571, lower = 0.122019, upper = 1.50528   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Exponential hazard ratio: times1 = [6, 6, 6, 7, 10, 13, 16, 22, 23], events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events2 = [1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], level = 95% | SOLVED | HR = 5.16055, lower = 1.58923, upper = 16.7573 |
| Exponential hazard ratio: times1 = [5, 8, 12, 20, 25], events1 = [1, 1, 0, 1, 0], times2 = [3, 4, 9, 10, 14], events2 = [1, 1, 1, 0, 1], level = 0.9 | SOLVED | HR = 0.428571, lower = 0.122019, upper = 1.50528 |
| Exponential hazard ratio: events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events2 = [1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], level = 95% | NEEDS_INPUT | bmed_exponential_hazard_ratio needs times1 (follow-up times, group 1 (positive)). Example: solve('bmed_exponential_hazar |
| Exponential hazard ratio: times1 = [6, 6, 6, 7, 10, 13, 16, 22, 23], events1 = [1, 1, 1, 1, 1, 1, 1, 1, 1], times2 = [6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events2 = [1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], level = 95%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_restricted_mean_survival_time"></a>

### Restricted mean survival time from the Kaplan-Meier curve, `bmed_restricted_mean_survival_time`

The restricted mean survival time up to tau: the area under the Kaplan-Meier survival curve from 0 to tau.

$$
\mathrm{RMST}(\tau) = \int_0^{\tau}\hat S(t)\,dt
$$

| input | meaning | condition |
|---|---|---|
| `times` | follow-up times (one time unit) |  |
| `events` | 1 = event, 0 = censored |  |
| `tau` | restriction time (in the unit of the times) |  |

| output | meaning |
|---|---|
| `RMST` | restricted mean survival time, in the time unit |

- **Method:** RMST = integral from 0 to tau of the Kaplan-Meier step function
- **Assumes:** non-informative censoring; the Kaplan-Meier step function integrated exactly
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Restricted mean survival time: the area under the Kaplan-Meier curve of the 21 follow-up times given from 0 to tau = 20 (shaded; the curve is the model's estimate at every step)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-restricted-mean-survival-time-survival)

*Restricted mean survival time: the area under the Kaplan-Meier curve of the 21 follow-up times given from 0 to tau = 20 (shaded; the curve is the model's estimate at every step). Marked: RMST = 16.12 time units, the expected survival time within the first 20. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., tau = 20.*

![Restricted mean survival time, in the time unit as τ runs from 5 to 35, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-restricted-mean-survival-time-live-1)

*Restricted mean survival time, in the time unit as τ runs from 5 to 35, every other input as you entered it (41 solutions). Marked: your τ = 20 gives RMST = 16.12. Your inputs: times = 6, 6, 6, 7, 10, 13, 16..., events = 1, 1, 1, 1, 1, 1, 1, 1..., tau = 20.*

```python
biomedical_science_ai999x.solve('bmed_restricted_mean_survival_time', times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], tau=20)
# -> RMST = 16.1165   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_restricted_mean_survival_time', times=[2, 3, 3, 5, 8, 9, 12, 15], events=[1, 0, 1, 1, 0, 1, 1, 0], tau=10)
# -> RMST = 7.175   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Restricted mean survival time: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], tau = 20 | SOLVED | RMST = 16.1165 |
| Restricted mean survival time: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], tau = 10 | SOLVED | RMST = 7.175 |
| Restricted mean survival time: events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], tau = 20 | NEEDS_INPUT | bmed_restricted_mean_survival_time needs times (follow-up times (one time unit)). Example: solve('bmed_restricted_mean_s |
| Restricted mean survival time: times = [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], events = [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], tau = 20, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_elbow_lever_muscle_force"></a>

### Static lever model of the forearm: muscle force and joint reaction, `bmed_elbow_lever_muscle_force`

For a horizontal forearm held in static equilibrium, the elbow-flexor muscle force and the joint reaction force from the hand load, the forearm weight, their distances from the elbow and the muscle's insertion distance and angle.

$$
F_m = \frac{W d_L + W_a d_a}{d_m \sin\theta},\quad R_x = F_m\cos\theta,\quad R_y = W + W_a - F_m\sin\theta
$$

| input | meaning | condition |
|---|---|---|
| `W` | load held in the hand (weight) |  |
| `d_load` | distance of the load from the elbow |  |
| `W_arm` | weight of the forearm and hand |  |
| `d_arm` | distance of the forearm's centre of mass from the elbow |  |
| `d_muscle` | distance of the muscle insertion from the elbow |  |
| `theta` | angle between the muscle and the forearm |  |

| output | meaning |
|---|---|
| `F_muscle` | muscle force |
| `R_x` | joint reaction along the forearm (towards the hand positive) |
| `R_y` | joint reaction, upward positive |
| `R` | magnitude of the joint reaction |

- **Method:** moments about the elbow: F d_m sin(theta) = W d_load + W_arm d_arm; force balances give $R_x = F$ cos(theta), R_y = W + W_arm - F sin(theta)
- **Assumes:** static equilibrium in a vertical plane; forearm horizontal from the elbow, loads vertical; one muscle pulling up and back towards the elbow at angle theta; $0 < \theta < 180$ deg
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Free-body diagram of the forearm as a lever about the elbow (dot): load W = 50 N at 350 mm, forearm weight 15 N at 150 mm, muscle inserted at 40 mm pulling at 80 degrees to the forearm](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-elbow-lever-muscle-force-free-body)

*Free-body diagram of the forearm as a lever about the elbow (dot): load W = 50 N at 350 mm, forearm weight 15 N at 150 mm, muscle inserted at 40 mm pulling at 80 degrees to the forearm. Muscle and joint arrows are drawn to one scale, the two weights to a larger one. Returned: muscle force 501.4 N, joint reaction R = 437.5 N (R_x = 87.06 N along the forearm, R_y = -428.8 N). Your inputs: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg.*

![Muscle force and joint reaction along the forearm as θ runs from 48 to 112 deg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-elbow-lever-muscle-force-live-1)

*Muscle force and joint reaction along the forearm as θ runs from 48 to 112 deg, every other input as you entered it (41 solutions). Marked: your θ = 80 deg gives F_muscle = 501.4. Your inputs: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg.*

![Muscle force as d_muscle runs from 2](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-elbow-lever-muscle-force-live-2)

*Muscle force as d_muscle runs from 2.4 to 5.6 cm, every other input as you entered it (29 solutions). Marked: your d_muscle = 4 cm gives F_muscle = 501.4. Your inputs: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg.*

```python
biomedical_science_ai999x.solve('bmed_elbow_lever_muscle_force', W=50.0, d_load=0.35000000000000003, W_arm=15.0, d_arm=0.15, d_muscle=0.04, theta=1.3962634015954636)
# -> F_muscle = 501.367 N, R_x = 87.0614 N, R_y = -428.75 N, R = 437.5 N   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_elbow_lever_muscle_force', W=100.0, d_load=0.3, W_arm=20.0, d_arm=0.14, d_muscle=0.05, theta=1.5707963267948966)
# -> F_muscle = 656 N, R_x = 0 N, R_y = -536 N, R = 536 N   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Forearm lever muscle force: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg | SOLVED | F_muscle = 501.367 N, R_x = 87.0614 N, R_y = -428.75 N, R = 437.5 N |
| Forearm lever muscle force: W = 100 N, d_load = 0.3 m, W_arm = 20 N, d_arm = 0.14 m, d_muscle = 0.05 m, theta = 90 deg | SOLVED | F_muscle = 656 N, R_x = 0 N, R_y = -536 N, R = 536 N |
| Forearm lever muscle force: d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg | NEEDS_INPUT | bmed_elbow_lever_muscle_force needs W (load held in the hand (weight)). Example: solve('bmed_elbow_lever_muscle_force',  |
| Forearm lever muscle force: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 80 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Forearm lever muscle force: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 190 deg | INVALID_INPUT | Static lever model of the forearm: muscle force and joint reaction: the muscle angle must lie between 0 and 180 degrees. |

<a id="bmed_poiseuille_vessel_flow"></a>

### Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance, `bmed_poiseuille_vessel_flow`

Laminar (Hagen-Poiseuille) flow of a Newtonian fluid through a rigid vessel: volume flow rate, mean velocity, wall shear stress and hydraulic resistance from the pressure drop, radius, length and viscosity.

$$
Q = \frac{\pi r^4 \Delta P}{8 \mu L},\quad \tau_w = \frac{4\mu Q}{\pi r^3},\quad R = \frac{8\mu L}{\pi r^4}
$$

| input | meaning | condition |
|---|---|---|
| `dP` | pressure drop along the vessel |  |
| `r` | vessel radius |  |
| `L` | vessel length |  |
| `mu` | viscosity of the blood (Newtonian) |  |

| output | meaning |
|---|---|
| `Q` | volume flow rate |
| `v_mean` | mean velocity |
| `tau_wall` | wall shear stress |
| `R_hyd` | hydraulic resistance |

- **Method:** $Q = pi$ $r^{4}$ dP / (8 mu L); v = Q / (pi r^2); $tau_w = 4$ mu Q / (pi r^3); $R = 8$ mu L / (pi r^4)
- **Assumes:** steady laminar flow, Newtonian fluid; rigid straight cylindrical vessel, fully developed flow
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Poiseuille flow in a vessel (pressure difference 533](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-poiseuille-vessel-flow-flow)

*Poiseuille flow in a vessel (pressure difference 533.3 Pa over L = 100 mm, viscosity 0.0035 Pa s): top, the parabolic velocity profile drawn from the returned mean velocity 0.7618 m/s (centre velocity twice the mean, derived); bottom, flow against radius, 36 runs of the model: it grows with the fourth power of the radius. Marked: Q = 9.574 mL/s at r = 2 mm, hydraulic resistance 5.57e+07 Pa s/m^3. In SI units: Q = 9.574e-06 m^3/s. Your inputs: dP = 4 mmHg, r = 2 mm, L = 10 cm, mu = 3.5 mPa*s.*

![Volume flow rate as r runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-poiseuille-vessel-flow-live-1)

*Volume flow rate as r runs from 1.2 to 2.8 mm, every other input as you entered it (41 solutions). Marked: your r = 2 mm gives Q = 0.5744 L/min. Your inputs: dP = 4 mmHg, r = 2 mm, L = 10 cm, mu = 3.5 mPa*s.*

![Volume flow rate as L runs from 6 to 14 cm, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-poiseuille-vessel-flow-live-2)

*Volume flow rate as L runs from 6 to 14 cm, every other input as you entered it (29 solutions). Marked: your L = 10 cm gives Q = 0.5744 L/min. Your inputs: dP = 4 mmHg, r = 2 mm, L = 10 cm, mu = 3.5 mPa*s.*

```python
biomedical_science_ai999x.solve('bmed_poiseuille_vessel_flow', dP=533.28954966, r=0.002, L=0.1, mu=0.0035)
# -> Q = 9.57359e-06 m^3/s, v_mean = 0.761842 m/s, tau_wall = 5.3329 Pa, R_hyd = 5.57042e+07 Pa*s/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_poiseuille_vessel_flow', dP=50.0, r=9.999999999999999e-05, L=0.005, mu=0.004)
# -> Q = 9.81748e-11 m^3/s, v_mean = 0.003125 m/s, tau_wall = 0.5 Pa, R_hyd = 5.09296e+11 Pa*s/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Poiseuille vessel flow: dP = 4 mmHg, r = 2 mm, L = 10 cm, mu = 3.5 mPa*s | SOLVED | Q = 9.57359e-06 m^3/s, v_mean = 0.761842 m/s, tau_wall = 5.3329 Pa, R_hyd = 5.57042e+07 Pa*s/m^3 |
| Poiseuille vessel flow: dP = 50 Pa, r = 100 um, L = 5 mm, mu = 0.004 Pa*s | SOLVED | Q = 9.81748e-11 m^3/s, v_mean = 0.003125 m/s, tau_wall = 0.5 Pa, R_hyd = 5.09296e+11 Pa*s/m^3 |
| Poiseuille vessel flow: r = 2 mm, L = 10 cm, mu = 3.5 mPa*s | NEEDS_INPUT | bmed_poiseuille_vessel_flow needs dP (pressure drop along the vessel). Example: solve('bmed_poiseuille_vessel_flow', dP= |
| Poiseuille vessel flow: dP = 4 mmHg, r = 2 mm, L = 10 cm, mu = 3.5 mPa*s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_windkessel_diastolic_decay"></a>

### Two-element Windkessel: diastolic pressure decay, `bmed_windkessel_diastolic_decay`

The arterial pressure of a two-element Windkessel model during diastole (no inflow), decaying from P0 towards the venous pressure with time constant RC.

$$
P(t) = P_v + (P_0 - P_v)\,e^{-t/(RC)}
$$

| input | meaning | condition |
|---|---|---|
| `P0` | pressure at the start of diastole |  |
| `Pv` | venous (outflow) pressure |  |
| `R` | peripheral resistance |  |
| `C` | arterial compliance |  |
| `t` | time into diastole |  |

| output | meaning |
|---|---|
| `P` | arterial pressure at time t |

- **Method:** C dP/dt = -(P - Pv)/R: $P = Pv + (P_{0} - Pv)$ exp(-t/(R C))
- **Assumes:** no inflow during diastole; linear resistance and compliance
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Arterial pressure at time t as P0 runs from 72 to 168 mmHg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-windkessel-diastolic-decay-live-1)

*Arterial pressure at time t as P0 runs from 72 to 168 mmHg, every other input as you entered it (41 solutions). Marked: your P0 = 120 mmHg gives P = 11.65 kPa. Your inputs: P0 = 120 mmHg, Pv = 5 mmHg, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, t = 0.5 s.*

![Arterial pressure at time t as R runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-windkessel-diastolic-decay-live-2)

*Arterial pressure at time t as R runs from 0.6 to 1.4 mmHg*s/mL, every other input as you entered it (29 solutions). Marked: your R = 1 mmHg*s/mL gives P = 11.65 kPa. Your inputs: P0 = 120 mmHg, Pv = 5 mmHg, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, t = 0.5 s.*

```python
biomedical_science_ai999x.solve('bmed_windkessel_diastolic_decay', P0=15998.6864898, Pv=666.611937075, R=133322387.41500002, C=1.1250923637684845e-08, t=0.5)
# -> P = 11652.5 Pa   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_windkessel_diastolic_decay', P0=13332.238741500001, Pv=0.0, R=159986864.89800003, C=8.25067733430222e-09, t=0.8)
# -> P = 7272.68 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Windkessel diastolic decay: P0 = 120 mmHg, Pv = 5 mmHg, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, t = 0.5 s | SOLVED | P = 11652.5 Pa |
| Windkessel diastolic decay: P0 = 100 mmHg, Pv = 0 mmHg, R = 1.2 mmHg*s/mL, C = 1.1 mL/mmHg, t = 0.8 s | SOLVED | P = 7272.68 Pa |
| Windkessel diastolic decay: Pv = 5 mmHg, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, t = 0.5 s | NEEDS_INPUT | bmed_windkessel_diastolic_decay needs P0 (pressure at the start of diastole). Example: solve('bmed_windkessel_diastolic_ |
| Windkessel diastolic decay: P0 = 120 mmHg, Pv = 5 mmHg, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, t = 0.5 s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_windkessel_input_impedance"></a>

### Windkessel models (two- and three-element): input impedance at a frequency, `bmed_windkessel_input_impedance`

The modulus and phase of the input impedance of a two-element (R, C) or three-element (Zc, R, C) Windkessel model of the arterial tree at a stated frequency.

$$
Z(\omega) = Z_c + \frac{R}{1 + j\omega R C}
$$

| input | meaning | condition |
|---|---|---|
| `model` | Windkessel model |  |
| `R` | peripheral resistance |  |
| `C` | arterial compliance |  |
| `f` | frequency |  |
| `Zc` | characteristic impedance of the aorta (three-element model only) |  |

| output | meaning |
|---|---|
| `Z_mod` | impedance modulus |
| `phase_deg` | impedance phase in degrees |

- **Method:** Z = Zc + R/(1 + j 2 pi f R C) (Zc = 0 for two elements); modulus and phase
- **Assumes:** linear model, sinusoidal steady state; Zc only for the three-element model
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Input impedance of the arterial Windkessel (R = 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-windkessel-input-impedance-bode)

*Input impedance of the arterial Windkessel (R = 1.333e+08 Pa s/m^3, C = 1.125e-08 m^3/Pa) against frequency, modulus (top) and phase (bottom), 60 runs per line of the model, for the two-element model and the three-element model with Zc = 6.666e+06 Pa s/m^3: the third element levels the modulus off at high frequency and brings the phase back towards 0. Marked: f = 1.2 Hz, |Z| = 1.174e+07 Pa s/m^3, phase -84.95 degrees (two element). Your inputs: model = two_element, R = 133322387.41500002 Pa*s/m^3, C = 1.1250923637684845e-08..., f = 1.2 Hz.*

![Impedance modulus as C runs from 6](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-windkessel-input-impedance-live-1)

*Impedance modulus as C runs from 6.751e-09 to 1.575e-08 m^3/Pa, every other input as you entered it (41 solutions). Marked: your C = 1.125e-08 m^3/Pa gives Z_mod = 1.174e+07. Your inputs: model = two_element, R = 133322387.41500002 Pa*s/m^3, C = 1.1250923637684845e-08..., f = 1.2 Hz.*

![Impedance modulus as f runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-windkessel-input-impedance-live-2)

*Impedance modulus as f runs from 0.72 to 1.68 Hz, every other input as you entered it (29 solutions). Marked: your f = 1.2 Hz gives Z_mod = 1.174e+07. Your inputs: model = two_element, R = 133322387.41500002 Pa*s/m^3, C = 1.1250923637684845e-08..., f = 1.2 Hz.*

```python
biomedical_science_ai999x.solve('bmed_windkessel_input_impedance', model='two_element', R=133322387.41500002, C=1.1250923637684845e-08, f=1.2)
# -> Z_mod = 1.17425e+07 Pa*s/m^3, phase_deg = -84.9471   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_windkessel_input_impedance', model='three_element', R=133322387.41500002, C=1.1250923637684845e-08, f=2.0, Zc=6666119.370750002)
# -> Z_mod = 9.96556e+06 Pa*s/m^3, phase_deg = -45.0521   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Windkessel impedance: model = two_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 1.2 Hz | SOLVED | Z_mod = 1.17425e+07 Pa*s/m^3, phase_deg = -84.9471 |
| Windkessel impedance: model = three_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 2 Hz, Zc = 0.05 mmHg*s/mL | SOLVED | Z_mod = 9.96556e+06 Pa*s/m^3, phase_deg = -45.0521 |
| Windkessel impedance: R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 1.2 Hz | NEEDS_INPUT | bmed_windkessel_input_impedance needs model (Windkessel model). Example: solve('bmed_windkessel_input_impedance', model= |
| Windkessel impedance: model = two_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 1.2 Hz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Windkessel impedance: model = three_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 2 Hz | INVALID_INPUT | Windkessel models (two- and three-element): input impedance at a frequency: the three-element model needs the characteri |

<a id="bmed_kelvin_voigt_creep"></a>

### Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress, `bmed_kelvin_voigt_creep`

The strain of a Kelvin-Voigt (spring parallel to dashpot) tissue model a time t after a constant stress is applied.

$$
\varepsilon(t) = \frac{\sigma}{E}\left(1 - e^{-E t/\eta}\right)
$$

| input | meaning | condition |
|---|---|---|
| `sigma` | applied constant stress |  |
| `E` | spring modulus |  |
| `eta` | dashpot viscosity $(Pa s)$ |  |
| `t` | time since loading |  |

| output | meaning |
|---|---|
| `strain` | strain at time t |

- **Method:** eta de/dt + E $e = \sigma$: $e = (\sigma/E)$(1 - exp(-E t/eta))
- **Assumes:** linear viscoelasticity; stress applied as a step at $t = 0$, no initial strain
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Strain at time t as σ runs from 6 to 14 kPa, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-kelvin-voigt-creep-live-1)

*Strain at time t as σ runs from 6 to 14 kPa, every other input as you entered it (41 solutions). Marked: your σ = 10 kPa gives strain = 0.04512. Your inputs: sigma = 10 kPa, E = 100 kPa, eta = 500 kPa*s, t = 3 s.*

![Strain at time t as t runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-kelvin-voigt-creep-live-2)

*Strain at time t as t runs from 1.8 to 4.2 s, every other input as you entered it (29 solutions). Marked: your t = 3 s gives strain = 0.04512. Your inputs: sigma = 10 kPa, E = 100 kPa, eta = 500 kPa*s, t = 3 s.*

```python
biomedical_science_ai999x.solve('bmed_kelvin_voigt_creep', sigma=10000.0, E=100000.0, eta=500000.0, t=3.0)
# -> strain = 0.0451188   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_kelvin_voigt_creep', sigma=2000000.0, E=1000000000.0, eta=5000000000.0, t=10.0)
# -> strain = 0.00172933   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Kelvin-Voigt creep: sigma = 10 kPa, E = 100 kPa, eta = 500 kPa*s, t = 3 s | SOLVED | strain = 0.0451188 |
| Kelvin-Voigt creep: sigma = 2 MPa, E = 1 GPa, eta = 5 GPa*s, t = 10 s | SOLVED | strain = 0.00172933 |
| Kelvin-Voigt creep: E = 100 kPa, eta = 500 kPa*s, t = 3 s | NEEDS_INPUT | bmed_kelvin_voigt_creep needs sigma (applied constant stress). Example: solve('bmed_kelvin_voigt_creep', sigma='10000 Pa |
| Kelvin-Voigt creep: sigma = 10 kPa, E = 100 kPa, eta = 500 kPa*s, t = 3 s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_sls_creep"></a>

### Standard linear solid (Zener) tissue model: creep under constant stress, `bmed_sls_creep`

The creep strain of a standard linear solid (a spring E_inf parallel to a Maxwell arm E1-eta) a time t after a constant stress is applied, and the final strain.

$$
\varepsilon(t) = \frac{\sigma}{E_\infty} - \left(\frac{\sigma}{E_\infty} - \frac{\sigma}{E_\infty + E_1}\right)e^{-t/\tau_c},\ \tau_c = \frac{\eta(E_\infty + E_1)}{E_\infty E_1}
$$

| input | meaning | condition |
|---|---|---|
| `sigma` | applied constant stress |  |
| `E_inf` | equilibrium (parallel) spring modulus |  |
| `E1` | spring modulus of the Maxwell arm |  |
| `eta` | dashpot viscosity of the Maxwell arm |  |
| `t` | time since loading |  |

| output | meaning |
|---|---|
| `strain` | strain at time t |
| `strain_inf` | final strain sigma/E_inf |

- **Method:** e(t) = sigma/E_inf - (sigma/E_inf - sigma/(E_inf + E1)) exp(-t/tau_c), $tau_c = \eta$ (E_inf + E1)/(E_inf E1)
- **Assumes:** linear viscoelasticity; step stress at $t = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Strain at time t as σ runs from 6 to 14 kPa, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-sls-creep-live-1)

*Strain at time t as σ runs from 6 to 14 kPa, every other input as you entered it (41 solutions). Marked: your σ = 10 kPa gives strain = 0.1045. Your inputs: sigma = 10 kPa, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 2 s.*

![Strain at time t as E_inf runs from 30 to 70 kPa, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-sls-creep-live-2)

*Strain at time t as E_inf runs from 30 to 70 kPa, every other input as you entered it (29 solutions). Marked: your E_inf = 50 kPa gives strain = 0.1045. Your inputs: sigma = 10 kPa, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 2 s.*

```python
biomedical_science_ai999x.solve('bmed_sls_creep', sigma=10000.0, E_inf=50000.0, E1=100000.0, eta=200000.0, t=2.0)
# -> strain = 0.104462, strain_inf = 0.2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_sls_creep', sigma=1000000.0, E_inf=200000000.0, E1=800000000.0, eta=2000000000.0, t=1.0)
# -> strain = 0.00130753, strain_inf = 0.005   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Standard linear solid creep: sigma = 10 kPa, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 2 s | SOLVED | strain = 0.104462, strain_inf = 0.2 |
| Standard linear solid creep: sigma = 1 MPa, E_inf = 200 MPa, E1 = 800 MPa, eta = 2 GPa*s, t = 1 s | SOLVED | strain = 0.00130753, strain_inf = 0.005 |
| Standard linear solid creep: E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 2 s | NEEDS_INPUT | bmed_sls_creep needs sigma (applied constant stress). Example: solve('bmed_sls_creep', sigma='10000 Pa', E_inf='50000 Pa |
| Standard linear solid creep: sigma = 10 kPa, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 2 s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_sls_stress_relaxation"></a>

### Standard linear solid (Zener) tissue model: stress relaxation at constant strain, `bmed_sls_stress_relaxation`

The stress of a standard linear solid a time t after a step strain eps0 is imposed, and the relaxed (equilibrium) stress.

$$
\sigma(t) = \varepsilon_0\left(E_\infty + E_1 e^{-E_1 t/\eta}\right)
$$

| input | meaning | condition |
|---|---|---|
| `eps0` | imposed step strain (a pure number, e.g. 0.05) |  |
| `E_inf` | equilibrium spring modulus |  |
| `E1` | spring modulus of the Maxwell arm |  |
| `eta` | dashpot viscosity of the Maxwell arm |  |
| `t` | time since the strain step |  |

| output | meaning |
|---|---|
| `stress` | stress at time t |
| `stress_inf` | relaxed stress E_inf eps0 |

- **Method:** sigma(t) = eps0 (E_inf + E1 exp(-E1 t/eta))
- **Assumes:** linear viscoelasticity; step strain at $t = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Stress at time t and relaxed stress E_inf eps0 as ε0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-sls-stress-relaxation-live-1)

*Stress at time t and relaxed stress E_inf eps0 as ε0 runs from 0.0125 to 0.0875, every other input as you entered it (41 solutions). Marked: your ε0 = 0.05 gives stress = 5533. Your inputs: eps0 = 0.05, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 1 s.*

![Stress at time t and relaxed stress E_inf eps0 as E_inf runs from 30 to 70 kPa, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-sls-stress-relaxation-live-2)

*Stress at time t and relaxed stress E_inf eps0 as E_inf runs from 30 to 70 kPa, every other input as you entered it (29 solutions). Marked: your E_inf = 50 kPa gives stress = 5533. Your inputs: eps0 = 0.05, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 1 s.*

```python
biomedical_science_ai999x.solve('bmed_sls_stress_relaxation', eps0=0.05, E_inf=50000.0, E1=100000.0, eta=200000.0, t=1.0)
# -> stress = 5532.65 Pa, stress_inf = 2500 Pa   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_sls_stress_relaxation', eps0=0.01, E_inf=200000000.0, E1=800000000.0, eta=2000000000.0, t=5.0)
# -> stress = 3.08268e+06 Pa, stress_inf = 2e+06 Pa   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Standard linear solid relaxation: eps0 = 0.05, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 1 s | SOLVED | stress = 5532.65 Pa, stress_inf = 2500 Pa |
| Standard linear solid relaxation: eps0 = 0.01, E_inf = 200 MPa, E1 = 800 MPa, eta = 2 GPa*s, t = 5 s | SOLVED | stress = 3.08268e+06 Pa, stress_inf = 2e+06 Pa |
| Standard linear solid relaxation: E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 1 s | NEEDS_INPUT | bmed_sls_stress_relaxation needs eps0 (imposed step strain (a pure number, e.g. 0.05)). Example: solve('bmed_sls_stress_ |
| Standard linear solid relaxation: eps0 = 0.05, E_inf = 50 kPa, E1 = 100 kPa, eta = 200 kPa*s, t = 1 s, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_indicator_dilution_cardiac_output"></a>

### Cardiac output by indicator dilution (Stewart-Hamilton), `bmed_indicator_dilution_cardiac_output`

The cardiac output (volume flow) from the mass of an injected indicator and its sampled first-pass concentration curve, by the Stewart-Hamilton principle with a trapezoidal area.

$$
CO = \frac{m}{\int_0^{\infty} C(t)\,dt}
$$

| input | meaning | condition |
|---|---|---|
| `m` | mass of indicator injected |  |
| `t` | sampling times in s, strictly increasing |  |
| `C` | indicator concentrations in mg/L (first pass, recirculation excluded) |  |

| output | meaning |
|---|---|
| `AUC` | area under the concentration curve, $mg s/L$ |
| `CO` | cardiac output m / AUC |

- **Method:** CO = m / integral of C dt (linear trapezoidal rule)
- **Assumes:** complete mixing, no indicator loss; first-pass curve (recirculation removed by the user); times in s, concentrations in mg/L; linear trapezoids
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Indicator-dilution curve: the 9 concentration samples given after injecting m = 5 mg of indicator, with the area under them computed by the model, AUC = 29](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-indicator-dilution-cardiac-output-fit)

*Indicator-dilution curve: the 9 concentration samples given after injecting m = 5 mg of indicator, with the area under them computed by the model, AUC = 29.6 mg s/L (shaded). Returned: cardiac output m/AUC = 10.14 L/min. In SI units: CO = 0.0001689 m^3/s. Your inputs: m = 4.9999999999999996e-06 kg, t = 0, 2, 4, 6, 8, 10, 12..., C = 0, 0.5, 2.8, 4.1, 3.5....*

![CO as m runs from 3e-06 to 7e-06 kg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-indicator-dilution-cardiac-output-live-1)

*CO as m runs from 3e-06 to 7e-06 kg, every other input as you entered it (41 solutions). Marked: your m = 5e-06 kg gives CO = 0.1689 L/s. Your inputs: m = 4.9999999999999996e-06 kg, t = 0, 2, 4, 6, 8, 10, 12..., C = 0, 0.5, 2.8, 4.1, 3.5....*

```python
biomedical_science_ai999x.solve('bmed_indicator_dilution_cardiac_output', m=4.9999999999999996e-06, t=[0, 2, 4, 6, 8, 10, 12, 14, 16], C=[0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2])
# -> AUC = 29.6, CO = 0.000168919 m^3/s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_indicator_dilution_cardiac_output', m=9.999999999999999e-06, t=[0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], C=[0, 0, 1.2, 4.5, 6.8, 6.0, 4.1, 2.5, 1.3, 0.6, 0.2])
# -> AUC = 27.1, CO = 0.000369004 m^3/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Indicator dilution cardiac output: m = 5 mg, t = [0, 2, 4, 6, 8, 10, 12, 14, 16], C = [0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2] | SOLVED | AUC = 29.6, CO = 0.000168919 m^3/s |
| Indicator dilution cardiac output: m = 10 mg, t = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], C = [0, 0, 1.2, 4.5, 6.8, 6, 4.1, 2.5, 1.3, 0.6, 0.2] | SOLVED | AUC = 27.1, CO = 0.000369004 m^3/s |
| Indicator dilution cardiac output: t = [0, 2, 4, 6, 8, 10, 12, 14, 16], C = [0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2] | NEEDS_INPUT | bmed_indicator_dilution_cardiac_output needs m (mass of indicator injected). Example: solve('bmed_indicator_dilution_car |
| Indicator dilution cardiac output: m = 5 mg, t = [0, 2, 4, 6, 8, 10, 12, 14, 16], C = [0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_respiratory_mechanics_fit"></a>

### Single-compartment respiratory mechanics: resistance and compliance by least squares, `bmed_respiratory_mechanics_fit`

Airway resistance, respiratory compliance and the end-expiratory pressure of the single-compartment equation of motion $P = R$ $Q + V/C + P_{0}$, fitted by least squares to sampled pressure, flow and volume.

$$
P = R\,\dot V + \frac{V}{C} + P_0
$$

| input | meaning | condition |
|---|---|---|
| `P` | airway pressures in cmH2O |  |
| `Q` | flows in L/s at the same instants |  |
| `V` | volumes above end-expiration in L at the same instants |  |

| output | meaning |
|---|---|
| `R` | resistance, cmH2O*s/L |
| `C` | compliance, L/cmH2O |
| `P0` | pressure at zero flow and volume, cmH2O |

- **Method:** multiple linear regression of P on Q, V and a constant; C = 1/elastance
- **Assumes:** linear single-compartment model; pressures in cmH2O, flows in L/s, volumes in L; ordinary least squares
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Single-compartment fit of the 6 samples of pressure, flow and volume given (dots: the measured pressure): the line is P0 + R Q + V/C from the returned resistance R = 16](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-respiratory-mechanics-fit-fit)

*Single-compartment fit of the 6 samples of pressure, flow and volume given (dots: the measured pressure): the line is P0 + R Q + V/C from the returned resistance R = 16.71 cmH2O s/L, compliance C = 0.02166 L/cmH2O and P0 = -3.169 cmH2O (derived); grey bars: the elastic part V/C above P0, the gap to the line being the resistive part R Q. Your inputs: P = 5.2, 9.8, 13.1, 15.9..., Q = 0.5, 0.5, 0.45, 0.4..., V = 0, 0.1, 0.19, 0.27....*

```python
biomedical_science_ai999x.solve('bmed_respiratory_mechanics_fit', P=[5.2, 9.8, 13.1, 15.9, 18.6, 20.2], Q=[0.5, 0.5, 0.45, 0.4, 0.35, 0.3], V=[0.0, 0.1, 0.19, 0.27, 0.34, 0.4])
# -> R = 16.7141, C = 0.0216558, P0 = -3.16919   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_respiratory_mechanics_fit', P=[8, 12.5, 16, 19.2, 22, 24.1, 25.5], Q=[0.8, 0.75, 0.7, 0.6, 0.5, 0.4, 0.3], V=[0, 0.08, 0.15, 0.22, 0.27, 0.32, 0.35])
# -> R = 3.60921, C = 0.0182636, P0 = 5.225   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Respiratory mechanics fit: P = [5.2, 9.8, 13.1, 15.9, 18.6, 20.2], Q = [0.5, 0.5, 0.45, 0.4, 0.35, 0.3], V = [0, 0.1, 0.19, 0.27, 0.34, 0.4] | SOLVED | R = 16.7141, C = 0.0216558, P0 = -3.16919 |
| Respiratory mechanics fit: P = [8, 12.5, 16, 19.2, 22, 24.1, 25.5], Q = [0.8, 0.75, 0.7, 0.6, 0.5, 0.4, 0.3], V = [0, 0.08, 0.15, 0.22, 0.27, 0.32, 0.35] | SOLVED | R = 3.60921, C = 0.0182636, P0 = 5.225 |
| Respiratory mechanics fit: Q = [0.5, 0.5, 0.45, 0.4, 0.35, 0.3], V = [0, 0.1, 0.19, 0.27, 0.34, 0.4] | NEEDS_INPUT | bmed_respiratory_mechanics_fit needs P (airway pressures in cmH2O). Example: solve('bmed_respiratory_mechanics_fit', P=[ |
| Respiratory mechanics fit: P = [5.2, 9.8, 13.1, 15.9, 18.6, 20.2], Q = [0.5, 0.5, 0.45, 0.4, 0.35, 0.3], V = [0, 0.1, 0.19, 0.27, 0.34, 0.4], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_ghk_voltage"></a>

### Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl, `bmed_ghk_voltage`

The resting membrane potential of the Goldman-Hodgkin-Katz voltage equation from the relative permeabilities and the inside and outside concentrations of K+, Na+ and Cl-.

$$
V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}
$$

| input | meaning | condition |
|---|---|---|
| `T` | absolute temperature (or degC) |  |
| `PK` | relative permeability to K+ (> 0) |  |
| `PNa` | relative permeability to Na+ |  |
| `PCl` | relative permeability to Cl- |  |
| `K_o` | outside K+ |  |
| `K_i` | inside K+ |  |
| `Na_o` | outside Na+ |  |
| `Na_i` | inside Na+ |  |
| `Cl_o` | outside Cl- |  |
| `Cl_i` | inside Cl- |  |

| output | meaning |
|---|---|
| `V_m` | membrane potential, inside relative to outside |

- **Method:** $V = (RT/F)$ ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o))
- **Assumes:** steady state with zero net current; constant field in the membrane, independent ions; only K+, Na+ and Cl- permeate; only ratios of permeabilities matter
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Membrane potential, inside relative to outside as T runs from 270 to 350 K, every other input as you entered it (41 solutions)](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-ghk-voltage-live-1.svg)

*Membrane potential, inside relative to outside as T runs from 270 to 350 K, every other input as you entered it (41 solutions). Marked: your T = 310 K gives V_m = -75.07 mV. Your inputs: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L.*

![Membrane potential, inside relative to outside as K_i runs from 84 to 196 mmol/L, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ghk-voltage-live-2)

*Membrane potential, inside relative to outside as K_i runs from 84 to 196 mmol/L, every other input as you entered it (29 solutions). Marked: your K_i = 140 mmol/L gives V_m = -75.07 mV. Your inputs: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L.*

```python
biomedical_science_ai999x.solve('bmed_ghk_voltage', T=310.0, PK=1, PNa=0.04, PCl=0.45, K_o=4.0, K_i=140.0, Na_o=145.0, Na_i=12.0, Cl_o=116.0, Cl_i=4.0)
# -> V_m = -0.0750663 V   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_ghk_voltage', T=279.45, PK=1, PNa=0.03, PCl=0.1, K_o=20.0, K_i=400.0, Na_o=440.0, Na_i=50.0, Cl_o=560.0, Cl_i=52.0)
# -> V_m = -0.0596663 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| GHK voltage: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L | SOLVED | V_m = -0.0750663 V |
| GHK voltage: T = 6.3 degC, PK = 1, PNa = 0.03, PCl = 0.1, K_o = 20 mM, K_i = 400 mM, Na_o = 440 mM, Na_i = 50 mM, Cl_o = 560 mM, Cl_i = 52 mM | SOLVED | V_m = -0.0596663 V |
| GHK voltage: PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L | NEEDS_INPUT | bmed_ghk_voltage needs T (absolute temperature (or degC)). Example: solve('bmed_ghk_voltage', T='310 K', PK=1, PNa=0.04, |
| GHK voltage: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| GHK voltage: T = 36.85 degC, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L | SOLVED | V_m = -0.0750663 V |

<a id="bmed_ghk_current_density"></a>

### Goldman-Hodgkin-Katz current equation: ionic current density through a membrane, `bmed_ghk_current_density`

The current density carried by one ion species through a membrane of given permeability at a stated membrane potential, from the GHK current (constant-field) equation; outward current positive.

$$
I = P z^2 \frac{F^2 V}{RT}\,\frac{[C]_i - [C]_o e^{-zFV/RT}}{1 - e^{-zFV/RT}}
$$

| input | meaning | condition |
|---|---|---|
| `P` | membrane permeability to the ion |  |
| `z` | valence of the ion (+1, +2, -1, ...) |  |
| `C_in` | inside concentration |  |
| `C_out` | outside concentration |  |
| `V` | membrane potential, inside relative to outside |  |
| `T` | absolute temperature (or degC) |  |

| output | meaning |
|---|---|
| `I` | current density, outward positive |

- **Method:** $I = P$ $z^{2}$ $F^{2}$ V/(RT) (C_in - C_out exp(-u))/(1 - exp(-u)), u = zFV/(RT) (written stably as P z F [C_in g(u) - C_out g(-u)], g(u) = u/(1 - e^-u))
- **Assumes:** constant field across the membrane; independent ion movement; outward current positive
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Current density, outward positive as C_in runs from 84 to 196 mmol/L, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ghk-current-density-live-1)

*Current density, outward positive as C_in runs from 84 to 196 mmol/L, every other input as you entered it (41 solutions). Marked: your C_in = 140 mmol/L gives I = 0.08528. Your inputs: P = 1e-8 m/s, z = 1, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K.*

![Current density, outward positive as P runs from 6e-09 to 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ghk-current-density-live-2)

*Current density, outward positive as P runs from 6e-09 to 1.4e-08 m/s, every other input as you entered it (29 solutions). Marked: your P = 1e-08 m/s gives I = 0.08528. Your inputs: P = 1e-8 m/s, z = 1, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K.*

```python
biomedical_science_ai999x.solve('bmed_ghk_current_density', P=1e-08, z=1, C_in=140.0, C_out=4.0, V=-0.02, T=310.0)
# -> I = 0.0852825 A/m^2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_ghk_current_density', P=5e-09, z=2, C_in=0.0001, C_out=2.0, V=-0.07, T=310.15)
# -> I = -0.0101622 A/m^2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| GHK current density: P = 1e-8 m/s, z = 1, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K | SOLVED | I = 0.0852825 A/m^2 |
| GHK current density: P = 5e-9 m/s, z = 2, C_in = 0.0001 mmol/L, C_out = 2 mmol/L, V = -70 mV, T = 37 degC | SOLVED | I = -0.0101622 A/m^2 |
| GHK current density: z = 1, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K | NEEDS_INPUT | bmed_ghk_current_density needs P (membrane permeability to the ion). Example: solve('bmed_ghk_current_density', P='1e-08 |
| GHK current density: P = 1e-8 m/s, z = 1, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| GHK current density: P = 1e-8 m/s, z = 0, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K | INVALID_INPUT | Goldman-Hodgkin-Katz current equation: ionic current density through a membrane: z is a nonzero whole number from -3 to  |

<a id="bmed_lif_firing_rate"></a>

### Leaky integrate-and-fire neuron: firing rate under constant current, `bmed_lif_firing_rate`

The interspike interval and firing rate of a leaky integrate-and-fire neuron driven by a constant current, with reset and an absolute refractory period.

$$
T = t_{ref} + \tau_m \ln\frac{V_\infty - V_{reset}}{V_\infty - V_{th}},\quad V_\infty = V_{rest} + R_m I
$$

| input | meaning | condition |
|---|---|---|
| `tau_m` | membrane time constant |  |
| `R_m` | membrane resistance |  |
| `I` | constant input current |  |
| `V_rest` | resting potential |  |
| `V_th` | spike threshold |  |
| `V_reset` | reset potential |  |
| `t_ref` | absolute refractory period |  |

| output | meaning |
|---|---|
| `rate` | firing rate |
| `ISI` | interspike interval |

- **Method:** ISI = t_ref + tau ln((V_inf - V_reset)/(V_inf - V_th)), V_inf = V_rest + R I; rate = 1/ISI
- **Assumes:** V_rest + R $I > V_th$ (otherwise no firing); V_reset < V_th; deterministic constant input
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Firing rate as V_rest runs from -79](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-lif-firing-rate-live-1)

*Firing rate as V_rest runs from -79.8 to -42 mV, every other input as you entered it (28 solutions). Marked: your V_rest = -70 mV gives rate = 41.71. Your inputs: tau_m = 20 ms, R_m = 100 Mohm, I = 0.3 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms.*

![Firing rate as R_m runs from 68](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-lif-firing-rate-live-2)

*Firing rate as R_m runs from 68.57 to 140 Mohm, every other input as you entered it (26 solutions). Marked: your R_m = 100 Mohm gives rate = 41.71. Your inputs: tau_m = 20 ms, R_m = 100 Mohm, I = 0.3 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms.*

```python
biomedical_science_ai999x.solve('bmed_lif_firing_rate', tau_m=0.02, R_m=100000000.0, I=3e-10, V_rest=-0.07, V_th=-0.05, V_reset=-0.07, t_ref=0.002)
# -> rate = 41.7149 Hz, ISI = 0.0239722 s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_lif_firing_rate', tau_m=0.01, R_m=50000000.0, I=5e-10, V_rest=-0.065, V_th=-0.05, V_reset=-0.068, t_ref=0.001)
# -> rate = 88.5254 Hz, ISI = 0.0112962 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| LIF firing rate: tau_m = 20 ms, R_m = 100 Mohm, I = 0.3 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms | SOLVED | rate = 41.7149 Hz, ISI = 0.0239722 s |
| LIF firing rate: tau_m = 10 ms, R_m = 50 Mohm, I = 500 pA, V_rest = -65 mV, V_th = -50 mV, V_reset = -68 mV, t_ref = 1 ms | SOLVED | rate = 88.5254 Hz, ISI = 0.0112962 s |
| LIF firing rate: R_m = 100 Mohm, I = 0.3 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms | NEEDS_INPUT | bmed_lif_firing_rate needs tau_m (membrane time constant). Example: solve('bmed_lif_firing_rate', tau_m='0.02 s', R_m='1 |
| LIF firing rate: tau_m = 20 ms, R_m = 100 Mohm, I = 0.3 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| LIF firing rate: tau_m = 20 ms, R_m = 100 Mohm, I = 0.1 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms | INVALID_INPUT | Leaky integrate-and-fire neuron: firing rate under constant current: the input does not bring the membrane to threshold: |

<a id="bmed_lif_subthreshold_voltage"></a>

### Passive membrane (leaky integrator): voltage at a time under constant current, `bmed_lif_subthreshold_voltage`

The membrane potential of a passive RC membrane (the subthreshold leaky integrate-and-fire neuron) a time t after a constant current starts, from its initial potential.

$$
V(t) = V_\infty + (V_0 - V_\infty)e^{-t/\tau_m},\quad V_\infty = V_{rest} + R_m I
$$

| input | meaning | condition |
|---|---|---|
| `tau_m` | membrane time constant |  |
| `R_m` | membrane resistance |  |
| `I` | constant input current |  |
| `V_rest` | resting potential |  |
| `V0` | potential at $t = 0$ |  |
| `t` | time |  |

| output | meaning |
|---|---|
| `V` | membrane potential at time t |

- **Method:** tau dV/dt = -(V - V_rest) + R I: V = V_inf + (V0 - V_inf) exp(-t/tau)
- **Assumes:** passive membrane, no spike or threshold; constant current from $t = 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Membrane potential at time t as V0 runs from -98 to -42 mV, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-lif-subthreshold-voltage-live-1)

*Membrane potential at time t as V0 runs from -98 to -42 mV, every other input as you entered it (41 solutions). Marked: your V0 = -70 mV gives V = -64.1 mV. Your inputs: tau_m = 20 ms, R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms.*

![Membrane potential at time t as V_rest runs from -98 to -42 mV, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-lif-subthreshold-voltage-live-2)

*Membrane potential at time t as V_rest runs from -98 to -42 mV, every other input as you entered it (29 solutions). Marked: your V_rest = -70 mV gives V = -64.1 mV. Your inputs: tau_m = 20 ms, R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms.*

```python
biomedical_science_ai999x.solve('bmed_lif_subthreshold_voltage', tau_m=0.02, R_m=100000000.0, I=1.5e-10, V_rest=-0.07, V0=-0.07, t=0.01)
# -> V = -0.064098 V   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_lif_subthreshold_voltage', tau_m=0.01, R_m=200000000.0, I=-5e-11, V_rest=-0.065, V0=-0.06, t=0.025)
# -> V = -0.0737687 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Passive membrane voltage: tau_m = 20 ms, R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms | SOLVED | V = -0.064098 V |
| Passive membrane voltage: tau_m = 10 ms, R_m = 200 Mohm, I = -50 pA, V_rest = -65 mV, V0 = -60 mV, t = 25 ms | SOLVED | V = -0.0737687 V |
| Passive membrane voltage: R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms | NEEDS_INPUT | bmed_lif_subthreshold_voltage needs tau_m (membrane time constant). Example: solve('bmed_lif_subthreshold_voltage', tau_ |
| Passive membrane voltage: tau_m = 20 ms, R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_cable_steady_state"></a>

### Passive cable (dendrite or axon): steady-state voltage along it and input resistance, `bmed_cable_steady_state`

The steady-state voltage at distance x along a passive cylindrical cable held at V0 at $x = 0$, and the input resistance there, for a sealed end, a killed (short-circuited) end or a semi-infinite cable.

$$
\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0\frac{\cosh((L - x)/\lambda)}{\cosh(L/\lambda)}\ \text{(sealed)}
$$

| input | meaning | condition |
|---|---|---|
| `Rm` | specific membrane resistance |  |
| `Ri` | axial (cytoplasmic) resistivity |  |
| `d` | cable diameter |  |
| `x` | distance from the stimulated end |  |
| `V0` | steady voltage at $x = 0$ (relative to rest) |  |
| `end` | far-end condition |  |
| `L` | cable length (sealed or killed end only) |  |

| output | meaning |
|---|---|
| `V_x` | voltage at x (relative to rest) |
| `R_in` | input resistance at $x = 0$ |

- **Method:** $\lambda = sqrt$(Rm d/(4 Ri)); sealed V0 cosh((L - x)/lambda)/cosh(L/lambda), killed V0 sinh((L - x)/lambda)/sinh(L/lambda), infinite V0 exp(-x/lambda); $R_in = r_i$ lambda times coth, tanh or 1
- **Assumes:** passive membrane, steady state; uniform cylinder; L only for a finite cable
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![V_x (voltage at x (relative to rest)) against L (cable length (sealed or killed end only)) from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cable-steady-state-curve-2)

*V_x (voltage at x (relative to rest)) against L (cable length (sealed or killed end only)) from 0.0001 to 0.01, one curve for each value of V0 (steady voltage at x = 0 (relative to rest)): V0 = 10 mV, V0 = 5 mV, V0 = 20 mV. The other inputs as given; 31 points per curve, each a run of the model. Marked: L = 0.001, V_x = 0.008134. Your inputs: Rm = 2 ohm*m^2, Ri = 1 ohm*m, d = 2e-06 m, x = 0.0003 m, V0 = 0.01 V, end = sealed, L = 0.001 m.*

![Voltage at x (relative to rest) as d runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cable-steady-state-live-1)

*Voltage at x (relative to rest) as d runs from 1.2e-06 to 2.8e-06 m, every other input as you entered it (41 solutions). Marked: your d = 2e-06 m gives V_x = 8.134 mV. Your inputs: Rm = 2 ohm*m^2, Ri = 1 ohm*m, d = 2e-06 m, x = 0.0003 m, V0 = 0.01 V, end = sealed, L = 0.001 m.*

![Voltage at x (relative to rest) for every choice of end, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cable-steady-state-live-2)

*Voltage at x (relative to rest) for every choice of end, every other input as you entered it (highlighted: your choice). sealed: 0.008134; killed: 0.006455; infinite: no answer. Your inputs: Rm = 2 ohm*m^2, Ri = 1 ohm*m, d = 2e-06 m, x = 0.0003 m, V0 = 0.01 V, end = sealed, L = 0.001 m.*

![Voltage at x (relative to rest) as V0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-cable-steady-state-live-3)

*Voltage at x (relative to rest) as V0 runs from 0.006 to 0.014 V, every other input as you entered it (29 solutions). Marked: your V0 = 0.01 V gives V_x = 8.134 mV. Your inputs: Rm = 2 ohm*m^2, Ri = 1 ohm*m, d = 2e-06 m, x = 0.0003 m, V0 = 0.01 V, end = sealed, L = 0.001 m.*

```python
biomedical_science_ai999x.solve('bmed_cable_steady_state', Rm=2.0, Ri=1.0, d=2e-06, x=0.0003, V0=0.01, end='sealed', L=0.001)
# -> V_x = 0.00813418 V, R_in = 4.17952e+08 ohm   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_cable_steady_state', Rm=1.0, Ri=2.0, d=4e-06, x=0.0005, V0=-0.005, end='infinite')
# -> V_x = -0.00246534 V, R_in = 1.1254e+08 ohm   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = sealed, L = 1 mm | SOLVED | V_x = 0.00813418 V, R_in = 4.17952e+08 ohm |
| Cable steady state: Rm = 10000 ohm*cm^2, Ri = 200 ohm*cm, d = 4 um, x = 500 um, V0 = -5 mV, end = infinite | SOLVED | V_x = -0.00246534 V, R_in = 1.1254e+08 ohm |
| Cable steady state: Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = sealed, L = 1 mm | NEEDS_INPUT | bmed_cable_steady_state needs Rm (specific membrane resistance). Example: solve('bmed_cable_steady_state', Rm='2 ohm*m^2 |
| Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = sealed, L = 1 mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = sealed | INVALID_INPUT | Passive cable (dendrite or axon): steady-state voltage along it and input resistance: a finite cable needs its length L. |
| Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = open, L = 1 mm | INVALID_INPUT | end: one of sealed, killed, infinite; got 'open'. |

<a id="bmed_hh_gating_steady_state"></a>

### Hodgkin-Huxley model: steady-state gating variables at a membrane potential, `bmed_hh_gating_steady_state`

The steady-state values of the Hodgkin-Huxley gating variables m, h and n at a clamped membrane potential, from the 1952 squid-axon rate functions.

$$
x_\infty(V) = \frac{\alpha_x(V)}{\alpha_x(V) + \beta_x(V)}
$$

| input | meaning | condition |
|---|---|---|
| `V` | clamped membrane potential |  |

| output | meaning |
|---|---|
| `m_inf` | Na activation at steady state |
| `h_inf` | Na inactivation at steady state |
| `n_inf` | K activation at steady state |

- **Method:** x_inf = alpha_x / (alpha_x + beta_x) for $x = m$, h, n
- **Assumes:** Hodgkin-Huxley 1952 rate functions, modern convention (rest near -65 mV), 6.3 degC; -150 $mV  \leq  V$ <= +100 mV
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![K activation at steady state as V runs from -91 to -39 mV, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hh-gating-steady-state-live-1)

*K activation at steady state as V runs from -91 to -39 mV, every other input as you entered it (41 solutions). Marked: your V = -65 mV gives n_inf = 0.3177. Your inputs: V = -65 mV.*

```python
biomedical_science_ai999x.solve('bmed_hh_gating_steady_state', V=-0.065)
# -> m_inf = 0.0529325, h_inf = 0.596121, n_inf = 0.317677   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_hh_gating_steady_state', V=-0.02)
# -> m_inf = 0.875694, h_inf = 0.00894348, n_inf = 0.835178   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hodgkin-Huxley gating steady state: V = -65 mV | SOLVED | m_inf = 0.0529325, h_inf = 0.596121, n_inf = 0.317677 |
| Hodgkin-Huxley gating steady state: V = -20 mV | SOLVED | m_inf = 0.875694, h_inf = 0.00894348, n_inf = 0.835178 |
| Hodgkin-Huxley gating steady state:  | NEEDS_INPUT | bmed_hh_gating_steady_state needs V (clamped membrane potential). Example: solve('bmed_hh_gating_steady_state', V='-0.06 |
| Hodgkin-Huxley gating steady state: V = -65 mV, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_hh_resting_potential"></a>

### Hodgkin-Huxley model: zero-current steady (resting) membrane potential, `bmed_hh_resting_potential`

The membrane potential at which the steady-state Na, K and leak currents of the Hodgkin-Huxley model sum to zero (the resting potential), from the maximal conductances and reversal potentials.

$$
\bar g_{Na} m_\infty^3 h_\infty (V - E_{Na}) + \bar g_K n_\infty^4 (V - E_K) + g_L (V - E_L) = 0
$$

| input | meaning | condition |
|---|---|---|
| `gNa` | maximal Na conductance |  |
| `gK` | maximal K conductance |  |
| `gL` | leak conductance |  |
| `ENa` | Na reversal potential |  |
| `EK` | K reversal potential |  |
| `EL` | leak reversal potential |  |

| output | meaning |
|---|---|
| `V_rest` | zero-current steady-state potential |

- **Method:** root of gNa m_inf^3 h_inf (V - ENa) + gK n_inf^4 (V - EK) + gL $(V - EL) = 0$ (scan and bisection)
- **Assumes:** Hodgkin-Huxley 1952 rate functions (modern convention), gates at steady state; a unique zero-current potential between -120 and +60 mV (checked); that state stable (Routh-Hurwitz test of the linearised model; otherwise refused); EK < EL < ENa
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Zero-current steady-state potential as EL runs from -76](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hh-resting-potential-live-1)

*Zero-current steady-state potential as EL runs from -76.14 to -32.63 mV, every other input as you entered it (41 solutions). Marked: your EL = -54.39 mV gives V_rest = -65 mV. Your inputs: gNa = 120 mS/cm^2, gK = 36 mS/cm^2, gL = 0.3 mS/cm^2, ENa = 50 mV, EK = -77 mV, EL = -54.387 mV.*

![Zero-current steady-state potential as EK runs from -107](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hh-resting-potential-live-2)

*Zero-current steady-state potential as EK runs from -107.8 to -68.2 mV, every other input as you entered it (19 solutions). Marked: your EK = -77 mV gives V_rest = -65 mV. Your inputs: gNa = 120 mS/cm^2, gK = 36 mS/cm^2, gL = 0.3 mS/cm^2, ENa = 50 mV, EK = -77 mV, EL = -54.387 mV.*

```python
biomedical_science_ai999x.solve('bmed_hh_resting_potential', gNa=1200.0, gK=360.0, gL=3.0, ENa=0.05, EK=-0.077, EL=-0.054387000000000005)
# -> V_rest = -0.0649964 V   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_hh_resting_potential', gNa=1000.0, gK=400.0, gL=2.0, ENa=0.055, EK=-0.08, EL=-0.06)
# -> V_rest = -0.0687945 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hodgkin-Huxley resting potential: gNa = 120 mS/cm^2, gK = 36 mS/cm^2, gL = 0.3 mS/cm^2, ENa = 50 mV, EK = -77 mV, EL = -54.387 mV | SOLVED | V_rest = -0.0649964 V |
| Hodgkin-Huxley resting potential: gNa = 100 mS/cm^2, gK = 40 mS/cm^2, gL = 0.2 mS/cm^2, ENa = 55 mV, EK = -80 mV, EL = -60 mV | SOLVED | V_rest = -0.0687945 V |
| Hodgkin-Huxley resting potential: gK = 36 mS/cm^2, gL = 0.3 mS/cm^2, ENa = 50 mV, EK = -77 mV, EL = -54.387 mV | NEEDS_INPUT | bmed_hh_resting_potential needs gNa (maximal Na conductance). Example: solve('bmed_hh_resting_potential', gNa='1200 S/m^ |
| Hodgkin-Huxley resting potential: gNa = 120 mS/cm^2, gK = 36 mS/cm^2, gL = 0.3 mS/cm^2, ENa = 50 mV, EK = -77 mV, EL = -54.387 mV, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_hh_potassium_conductance_clamp"></a>

### Hodgkin-Huxley model: potassium conductance after a voltage-clamp step, `bmed_hh_potassium_conductance_clamp`

The potassium conductance gK = gK_max $n^{4}$ and the gate n a time t after the membrane is stepped from a holding potential (at steady state) to a new clamped potential, in the Hodgkin-Huxley model.

$$
n(t) = n_\infty - (n_\infty - n_0)e^{-t/\tau_n},\quad g_K = \bar g_K n^4
$$

| input | meaning | condition |
|---|---|---|
| `gK_max` | maximal K conductance |  |
| `V_hold` | holding potential before the step |  |
| `V_step` | clamped potential after the step |  |
| `t` | time after the step |  |

| output | meaning |
|---|---|
| `g_K` | K conductance at time t |
| `n` | K gate n at time t |

- **Method:** n(t) = n_inf(V1) - (n_inf(V1) - n_inf(V0)) exp(-(alpha_n + beta_n) t); g_K = gK_max $n^{4}$
- **Assumes:** ideal voltage clamp; gates at steady state at the holding potential; Hodgkin-Huxley 1952 rate functions
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![K conductance at time t as t runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hh-potassium-conductance-clamp-live-1)

*K conductance at time t as t runs from 1.2 to 2.8 ms, every other input as you entered it (41 solutions). Marked: your t = 2 ms gives g_K = 104.2. Your inputs: gK_max = 36 mS/cm^2, V_hold = -65 mV, V_step = 0 mV, t = 2 ms.*

![K conductance at time t as V_hold runs from -91 to -39 mV, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hh-potassium-conductance-clamp-live-2)

*K conductance at time t as V_hold runs from -91 to -39 mV, every other input as you entered it (29 solutions). Marked: your V_hold = -65 mV gives g_K = 104.2. Your inputs: gK_max = 36 mS/cm^2, V_hold = -65 mV, V_step = 0 mV, t = 2 ms.*

```python
biomedical_science_ai999x.solve('bmed_hh_potassium_conductance_clamp', gK_max=360.0, V_hold=-0.065, V_step=0.0, t=0.002)
# -> g_K = 104.172 S/m^2, n = 0.733436   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_hh_potassium_conductance_clamp', gK_max=360.0, V_hold=-0.07, V_step=-0.03, t=0.005)
# -> g_K = 77.5423 S/m^2, n = 0.681254   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hodgkin-Huxley potassium conductance: gK_max = 36 mS/cm^2, V_hold = -65 mV, V_step = 0 mV, t = 2 ms | SOLVED | g_K = 104.172 S/m^2, n = 0.733436 |
| Hodgkin-Huxley potassium conductance: gK_max = 36 mS/cm^2, V_hold = -70 mV, V_step = -30 mV, t = 5 ms | SOLVED | g_K = 77.5423 S/m^2, n = 0.681254 |
| Hodgkin-Huxley potassium conductance: V_hold = -65 mV, V_step = 0 mV, t = 2 ms | NEEDS_INPUT | bmed_hh_potassium_conductance_clamp needs gK_max (maximal K conductance). Example: solve('bmed_hh_potassium_conductance_ |
| Hodgkin-Huxley potassium conductance: gK_max = 36 mS/cm^2, V_hold = -65 mV, V_step = 0 mV, t = 2 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_postsynaptic_potential"></a>

### Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current, `bmed_postsynaptic_potential`

The postsynaptic potential of a passive RC membrane receiving a synaptic current I0 exp(-t/tau_s) at $t = 0$: its value at time t, the time of its peak and the peak value.

$$
V(t) = \frac{R I_0 \tau_s}{\tau_s - \tau_m}\left(e^{-t/\tau_s} - e^{-t/\tau_m}\right),\quad t_{peak} = \frac{\tau_m\tau_s}{\tau_s - \tau_m}\ln\frac{\tau_s}{\tau_m}
$$

| input | meaning | condition |
|---|---|---|
| `tau_m` | membrane time constant |  |
| `tau_s` | synaptic current decay time constant |  |
| `R_m` | membrane resistance |  |
| `I0` | peak synaptic current into the cell (positive depolarises; an outward, inhibitory current is negative) |  |
| `t` | time after the synaptic event |  |

| output | meaning |
|---|---|
| `V_t` | potential change at time t |
| `t_peak` | time of the peak |
| `V_peak` | peak potential change |

- **Method:** tau_m $dV/dt = -V + R$ I0 exp(-t/tau_s): $V = R$ I0 tau_s/(tau_s - tau_m) (exp(-t/tau_s) - exp(-t/tau_m)); t_peak = tau_m tau_s ln(tau_s/tau_m)/(tau_s - tau_m)
- **Assumes:** passive membrane starting at rest; current jumps to I0 and decays exponentially; $tau_s  \neq  tau_m$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Peak potential change and potential change at time t as R_m runs from 60 to 140 Mohm, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-postsynaptic-potential-live-1)

*Peak potential change and potential change at time t as R_m runs from 60 to 140 Mohm, every other input as you entered it (41 solutions). Marked: your R_m = 100 Mohm gives V_peak = 0.7875 mV. Your inputs: tau_m = 20 ms, tau_s = 5 ms, R_m = 100 Mohm, I0 = 50 pA, t = 10 ms.*

![Peak potential change and potential change at time t as I0 runs from 30 to 70 pA, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-postsynaptic-potential-live-2)

*Peak potential change and potential change at time t as I0 runs from 30 to 70 pA, every other input as you entered it (29 solutions). Marked: your I0 = 50 pA gives V_peak = 0.7875 mV. Your inputs: tau_m = 20 ms, tau_s = 5 ms, R_m = 100 Mohm, I0 = 50 pA, t = 10 ms.*

```python
biomedical_science_ai999x.solve('bmed_postsynaptic_potential', tau_m=0.02, tau_s=0.005, R_m=100000000.0, I0=5e-11, t=0.01)
# -> V_t = 0.000785326 V, t_peak = 0.00924196 s, V_peak = 0.000787451 V   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_postsynaptic_potential', tau_m=0.01, tau_s=0.002, R_m=200000000.0, I0=-3e-11, t=0.003)
# -> V_t = -0.000776532 V, t_peak = 0.00402359 s, V_peak = -0.000802488 V   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Postsynaptic potential: tau_m = 20 ms, tau_s = 5 ms, R_m = 100 Mohm, I0 = 50 pA, t = 10 ms | SOLVED | V_t = 0.000785326 V, t_peak = 0.00924196 s, V_peak = 0.000787451 V |
| Postsynaptic potential: tau_m = 10 ms, tau_s = 2 ms, R_m = 200 Mohm, I0 = -30 pA, t = 3 ms | SOLVED | V_t = -0.000776532 V, t_peak = 0.00402359 s, V_peak = -0.000802488 V |
| Postsynaptic potential: tau_s = 5 ms, R_m = 100 Mohm, I0 = 50 pA, t = 10 ms | NEEDS_INPUT | bmed_postsynaptic_potential needs tau_m (membrane time constant). Example: solve('bmed_postsynaptic_potential', tau_m='0 |
| Postsynaptic potential: tau_m = 20 ms, tau_s = 5 ms, R_m = 100 Mohm, I0 = 50 pA, t = 10 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_fitzhugh_nagumo_fixed_point"></a>

### FitzHugh-Nagumo neuron model: fixed point and its stability, `bmed_fitzhugh_nagumo_fixed_point`

The resting (fixed) point of the FitzHugh-Nagumo model v' = $v - v^{3}/3 - w + I$, w' = eps (v + a - b w), the trace and determinant of its Jacobian and whether it is stable (resting) or unstable (repetitive firing).

$$
v - \frac{v^3}{3} - \frac{v + a}{b} + I = 0,\quad J = \begin{pmatrix}1 - v^2 & -1\\ \varepsilon & -\varepsilon b\end{pmatrix}
$$

| input | meaning | condition |
|---|---|---|
| `a` | parameter a |  |
| `b` | parameter b $(0 < b < 1)$ |  |
| `eps` | time-scale ratio eps (> 0) |  |
| `I` | applied current (dimensionless) |  |

| output | meaning |
|---|---|
| `v_star` | fixed-point v |
| `w_star` | fixed-point w |
| `trace` | trace of the Jacobian |
| `determinant` | determinant of the Jacobian |
| `stable` | true when the fixed point is stable |

- **Method:** v* solves $v - v^{3}/3 - (v + a)/b + I = 0$ (bisection); w* = (v* + a)/b; J = [[$1 - v$*^2, -1], [eps, -eps b]]; stable when trace < 0 (det > 0 here)
- **Assumes:** dimensionless FitzHugh-Nagumo model; $0 < b < 1$ (a unique fixed point)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![stable (true when the fixed point is stable) over a from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-fitzhugh-nagumo-fixed-point-decision)

*stable (true when the fixed point is stable) over a from 0.35 to 1.05 and b from 0.4 to 1.2: each of the 21 x 13 cells is one run of the model, coloured by the verdict it returned (no: 51 cells, yes: 138 cells). Marked: the inputs given, a = 0.7, b = 0.8 gives yes. In SI units: v_star = -1.199. Your inputs: a = 0.7, b = 0.8, eps = 0.08, I = 0.*

![Fixed point v* of the FitzHugh-Nagumo model (a = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-fitzhugh-nagumo-fixed-point-stability)

*Fixed point v* of the FitzHugh-Nagumo model (a = 0.7, b = 0.8, eps = 0.08) against the applied current from -0.5 to 2, 101 runs, coloured by the stability the model returns (43 unstable). Marked: I = 0, v* = -1.199, w* = -0.6243, trace -0.5026, determinant 0.1081, stable. Your inputs: a = 0.7, b = 0.8, eps = 0.08, I = 0.*

![Fixed-point v as a runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-fitzhugh-nagumo-fixed-point-live-1)

*Fixed-point v as a runs from 0.175 to 1.225, every other input as you entered it (41 solutions). Marked: your a = 0.7 gives v_star = -1.199. Shaded by true when the fixed point is stable: no, yes. Your inputs: a = 0.7, b = 0.8, eps = 0.08, I = 0.*

![True when the fixed point is stable for 25 5 x 5 combinations of a and I around your inputs (the other inputs as entered); the ring marks your inputs, which give yes](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-fitzhugh-nagumo-fixed-point-live-2)

*True when the fixed point is stable for 25 5 x 5 combinations of a and I around your inputs (the other inputs as entered); the ring marks your inputs, which give yes. Blank cells admit no answer. Your inputs: a = 0.7, b = 0.8, eps = 0.08, I = 0.*

![Fixed-point v as I runs from -1 to 1, every other input as you entered it (27 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-fitzhugh-nagumo-fixed-point-live-3)

*Fixed-point v as I runs from -1 to 1, every other input as you entered it (27 solutions). Marked: your I = 0 gives v_star = -1.199. Shaded by true when the fixed point is stable: yes, no. Your inputs: a = 0.7, b = 0.8, eps = 0.08, I = 0.*

```python
biomedical_science_ai999x.solve('bmed_fitzhugh_nagumo_fixed_point', a=0.7, b=0.8, eps=0.08, I=0)
# -> v_star = -1.19941, w_star = -0.62426, trace = -0.50258, determinant = 0.108069, stable = True   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_fitzhugh_nagumo_fixed_point', a=0.7, b=0.8, eps=0.08, I=0.5)
# -> v_star = -0.804848, w_star = -0.13106, trace = 0.28822, determinant = 0.0574579, stable = False   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| FitzHugh-Nagumo fixed point: a = 0.7, b = 0.8, eps = 0.08, I = 0 | SOLVED | v_star = -1.19941, w_star = -0.62426, trace = -0.50258, determinant = 0.108069, stable = True |
| FitzHugh-Nagumo fixed point: a = 0.7, b = 0.8, eps = 0.08, I = 0.5 | SOLVED | v_star = -0.804848, w_star = -0.13106, trace = 0.28822, determinant = 0.0574579, stable = False |
| FitzHugh-Nagumo fixed point: b = 0.8, eps = 0.08, I = 0 | NEEDS_INPUT | bmed_fitzhugh_nagumo_fixed_point needs a (parameter a). Example: solve('bmed_fitzhugh_nagumo_fixed_point', a=0.7, b=0.8, |
| FitzHugh-Nagumo fixed point: a = 0.7, b = 0.8, eps = 0.08, I = 0, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| FitzHugh-Nagumo fixed point: a = 0.7, b = 1.5, eps = 0.08, I = 0 | INVALID_INPUT | FitzHugh-Nagumo neuron model: fixed point and its stability: this calculation needs 0 < b < 1 (then the fixed point is u |

<a id="bmed_xray_attenuation_layers"></a>

### X-ray attenuation through tissue layers (narrow monoenergetic beam), `bmed_xray_attenuation_layers`

The transmitted fraction and intensity of a narrow monoenergetic X-ray beam crossing successive tissue layers, from each layer's mass attenuation coefficient, density and thickness.

$$
\frac{I}{I_0} = \exp\left(-\sum_i \left(\frac{\mu}{\rho}\right)_i \rho_i x_i\right)
$$

| input | meaning | condition |
|---|---|---|
| `I0` | incident intensity |  |
| `mu_rho` | mass attenuation coefficient of each layer, $cm^{2}/g$ |  |
| `rho` | density of each layer, $g/cm^{3}$ |  |
| `x` | thickness of each layer, cm |  |

| output | meaning |
|---|---|
| `T` | transmitted fraction |
| `I` | transmitted intensity |

- **Method:** $T = exp$(-sum (mu/rho)_i rho_i x_i); $I = I_{0}$ T
- **Assumes:** narrow beam (no scatter reaching the detector); one photon energy; units $cm^{2}/g$, $g/cm^{3}$, cm
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![X-ray beam through 3 layers drawn to scale (thicknesses 2, 1, 3 cm, mass attenuation 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-xray-attenuation-layers-layers)

*X-ray beam through 3 layers drawn to scale (thicknesses 2, 1, 3 cm, mass attenuation 0.2, 0.5, 0.2 cm^2/g, densities 1, 1.85, 1 g/cm^3, as given): each point is the model's intensity after the depth crossed so far (log axis, steeper in a denser layer). Marked: I0 = 1000 W/m^2 gives I = 145.9 W/m^2, transmitted fraction T = 0.1459. Your inputs: I0 = 1000 W/m^2, mu_rho = 0.2, 0.5, 0.2, rho = 1, 1.85, 1, x = 2, 1, 3.*

![Transmitted intensity as I0 runs from 600 to 1400 W/m^2, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-xray-attenuation-layers-live-1)

*Transmitted intensity as I0 runs from 600 to 1400 W/m^2, every other input as you entered it (41 solutions). Marked: your I0 = 1000 W/m^2 gives I = 145.9. Your inputs: I0 = 1000 W/m^2, mu_rho = 0.2, 0.5, 0.2, rho = 1, 1.85, 1, x = 2, 1, 3.*

```python
biomedical_science_ai999x.solve('bmed_xray_attenuation_layers', I0=1000.0, mu_rho=[0.2, 0.5, 0.2], rho=[1.0, 1.85, 1.0], x=[2, 1, 3])
# -> T = 0.145876, I = 145.876 W/m^2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_xray_attenuation_layers', I0=50.0, mu_rho=[0.18], rho=[1.06], x=[10])
# -> T = 0.148377, I = 7.41884 W/m^2   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| X-ray attenuation through layers: I0 = 1000 W/m^2, mu_rho = [0.2, 0.5, 0.2], rho = [1, 1.85, 1], x = [2, 1, 3] | SOLVED | T = 0.145876, I = 145.876 W/m^2 |
| X-ray attenuation through layers: I0 = 50 W/m^2, mu_rho = [0.18], rho = [1.06], x = [10] | SOLVED | T = 0.148377, I = 7.41884 W/m^2 |
| X-ray attenuation through layers: mu_rho = [0.2, 0.5, 0.2], rho = [1, 1.85, 1], x = [2, 1, 3] | NEEDS_INPUT | bmed_xray_attenuation_layers needs I0 (incident intensity). Example: solve('bmed_xray_attenuation_layers', I0='1000 W/m^ |
| X-ray attenuation through layers: I0 = 1000 W/m^2, mu_rho = [0.2, 0.5, 0.2], rho = [1, 1.85, 1], x = [2, 1, 3], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_mri_spin_echo_signal"></a>

### MRI spin-echo signal from T1, T2, TR and TE (simplified model), `bmed_mri_spin_echo_signal`

The steady-state spin-echo signal $S = M_{0}$ (1 - exp(-TR/T1)) exp(-TE/T2) of a tissue with relaxation times T1 and T2, for repetition time TR and echo time TE.

$$
S = M_0\left(1 - e^{-TR/T_1}\right)e^{-TE/T_2}
$$

| input | meaning | condition |
|---|---|---|
| `M0` | equilibrium magnetisation (relative proton density) |  |
| `TR` | repetition time |  |
| `TE` | echo time |  |
| `T1` | longitudinal relaxation time |  |
| `T2` | transverse relaxation time |  |

| output | meaning |
|---|---|
| `S` | signal in the units of M0 |

- **Method:** $S = M_{0}$ (1 - exp(-TR/T1)) exp(-TE/T2)
- **Assumes:** 90 degree excitations every TR, transverse magnetisation spoiled before each; ideal refocusing without effect on Mz (TE << TR); $TE < TR$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Signal in the units of M0 as M0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-spin-echo-signal-live-1)

*Signal in the units of M0 as M0 runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your M0 = 1 gives S = 0.3669. Your inputs: M0 = 1, TR = 500 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms.*

![Signal in the units of M0 as TR runs from 300 to 700 ms, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-spin-echo-signal-live-2)

*Signal in the units of M0 as TR runs from 300 to 700 ms, every other input as you entered it (29 solutions). Marked: your TR = 500 ms gives S = 0.3669. Your inputs: M0 = 1, TR = 500 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms.*

```python
biomedical_science_ai999x.solve('bmed_mri_spin_echo_signal', M0=1, TR=0.5, TE=0.015, T1=0.9, T2=0.1)
# -> S = 0.366874   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_spin_echo_signal', M0=0.8, TR=3.0, TE=0.1, T1=2.5, T2=0.25)
# -> S = 0.374739   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Spin-echo signal: M0 = 1, TR = 500 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms | SOLVED | S = 0.366874 |
| Spin-echo signal: M0 = 0.8, TR = 3 s, TE = 100 ms, T1 = 2.5 s, T2 = 250 ms | SOLVED | S = 0.374739 |
| Spin-echo signal: TR = 500 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms | NEEDS_INPUT | bmed_mri_spin_echo_signal needs M0 (equilibrium magnetisation (relative proton density)). Example: solve('bmed_mri_spin_ |
| Spin-echo signal: M0 = 1, TR = 500 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Spin-echo signal: M0 = 1, TR = 10 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms | INVALID_INPUT | MRI spin-echo signal from T1, T2, TR and TE (simplified model): TE must be shorter than TR. |

<a id="bmed_mri_inversion_recovery"></a>

### MRI inversion recovery: longitudinal magnetisation at TI and the null point, `bmed_mri_inversion_recovery`

The longitudinal magnetisation (fraction of M0, signed) at inversion time TI in a repeated inversion-recovery sequence with repetition time TR, and the inversion time that nulls the tissue.

$$
\frac{M_z(TI)}{M_0} = 1 - 2e^{-TI/T_1} + e^{-TR/T_1},\quad TI_{null} = T_1\ln\frac{2}{1 + e^{-TR/T_1}}
$$

| input | meaning | condition |
|---|---|---|
| `T1` | longitudinal relaxation time |  |
| `TR` | repetition time |  |
| `TI` | inversion time |  |

| output | meaning |
|---|---|
| `Mz_ratio` | $Mz/M_{0}$ at TI (negative before the null) |
| `TI_null` | inversion time of zero signal |

- **Method:** Mz(TI)/M0 = 1 - 2 exp(-TI/T1) + exp(-TR/T1); null at $TI = T_{1}$ ln(2/(1 + exp(-TR/T1)))
- **Assumes:** ideal 180 degree inversion and 90 degree readout at TI; readout leaves $Mz = 0$; steady state, $TI < TR$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Inversion time of zero signal as T1 runs from 150 to 350 ms, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-inversion-recovery-live-1)

*Inversion time of zero signal as T1 runs from 150 to 350 ms, every other input as you entered it (41 solutions). Marked: your T1 = 250 ms gives TI_null = 0.1732. Your inputs: T1 = 250 ms, TR = 2000 ms, TI = 150 ms.*

![Mz/M0 at TI as TI runs from 90 to 210 ms, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-inversion-recovery-live-2)

*Mz/M0 at TI as TI runs from 90 to 210 ms, every other input as you entered it (29 solutions). Marked: your TI = 150 ms gives Mz_ratio = -0.09729. Your inputs: T1 = 250 ms, TR = 2000 ms, TI = 150 ms.*

```python
biomedical_science_ai999x.solve('bmed_mri_inversion_recovery', T1=0.25, TR=2.0, TI=0.15)
# -> Mz_ratio = -0.0972878, TI_null = 0.173203 s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_inversion_recovery', T1=4.0, TR=9.0, TI=2.5)
# -> Mz_ratio = 0.0348764, TI_null = 2.37176 s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Inversion recovery: T1 = 250 ms, TR = 2000 ms, TI = 150 ms | SOLVED | Mz_ratio = -0.0972878, TI_null = 0.173203 s |
| Inversion recovery: T1 = 4 s, TR = 9 s, TI = 2.5 s | SOLVED | Mz_ratio = 0.0348764, TI_null = 2.37176 s |
| Inversion recovery: TR = 2000 ms, TI = 150 ms | NEEDS_INPUT | bmed_mri_inversion_recovery needs T1 (longitudinal relaxation time). Example: solve('bmed_mri_inversion_recovery', T1='0 |
| Inversion recovery: T1 = 250 ms, TR = 2000 ms, TI = 150 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_mri_spoiled_gradient_echo"></a>

### MRI spoiled gradient-echo signal (Ernst equation), `bmed_mri_spoiled_gradient_echo`

The steady-state signal of a spoiled gradient-echo sequence with flip angle alpha, repetition time TR and echo time TE for a tissue with T1 and $T_{2} .$

$$
S = M_0 \sin\alpha\,\frac{1 - E_1}{1 - E_1\cos\alpha}\,e^{-TE/T_2^*},\quad E_1 = e^{-TR/T_1}
$$

| input | meaning | condition |
|---|---|---|
| `M0` | equilibrium magnetisation (relative proton density) |  |
| `TR` | repetition time |  |
| `TE` | echo time |  |
| `T1` | longitudinal relaxation time |  |
| `T2star` | effective transverse relaxation time T2* |  |
| `alpha` | flip angle |  |

| output | meaning |
|---|---|
| `S` | signal in the units of M0 |

- **Method:** $S = M_{0}$ sin(alpha) (1 - E1)/(1 - cos(alpha) E1) exp($-TE/T_{2}$*), $E_{1} = e^{\frac{-TR}{T_{1}}}$
- **Assumes:** ideal spoiling of transverse magnetisation before each pulse; steady state; $0 < \alpha  \leq  90$ deg, $TE < TR$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Signal in the units of M0 as M0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-spoiled-gradient-echo-live-1)

*Signal in the units of M0 as M0 runs from 0.25 to 1.75, every other input as you entered it (41 solutions). Marked: your M0 = 1 gives S = 0.0656. Your inputs: M0 = 1, TR = 10 ms, TE = 3 ms, T1 = 1000 ms, T2star = 40 ms, alpha = 8 deg.*

![Signal in the units of M0 as TR runs from 6 to 14 ms, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-spoiled-gradient-echo-live-2)

*Signal in the units of M0 as TR runs from 6 to 14 ms, every other input as you entered it (29 solutions). Marked: your TR = 10 ms gives S = 0.0656. Your inputs: M0 = 1, TR = 10 ms, TE = 3 ms, T1 = 1000 ms, T2star = 40 ms, alpha = 8 deg.*

```python
biomedical_science_ai999x.solve('bmed_mri_spoiled_gradient_echo', M0=1, TR=0.01, TE=0.003, T1=1.0, T2star=0.04, alpha=0.13962634015954636)
# -> S = 0.065597   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_spoiled_gradient_echo', M0=0.9, TR=0.02, TE=0.002, T1=0.6, T2star=0.03, alpha=0.5235987755982988)
# -> S = 0.0850011   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Spoiled gradient-echo signal: M0 = 1, TR = 10 ms, TE = 3 ms, T1 = 1000 ms, T2star = 40 ms, alpha = 8 deg | SOLVED | S = 0.065597 |
| Spoiled gradient-echo signal: M0 = 0.9, TR = 20 ms, TE = 2 ms, T1 = 600 ms, T2star = 30 ms, alpha = 30 deg | SOLVED | S = 0.0850011 |
| Spoiled gradient-echo signal: TR = 10 ms, TE = 3 ms, T1 = 1000 ms, T2star = 40 ms, alpha = 8 deg | NEEDS_INPUT | bmed_mri_spoiled_gradient_echo needs M0 (equilibrium magnetisation (relative proton density)). Example: solve('bmed_mri_ |
| Spoiled gradient-echo signal: M0 = 1, TR = 10 ms, TE = 3 ms, T1 = 1000 ms, T2star = 40 ms, alpha = 8 deg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_ultrasound_interface"></a>

### Ultrasound reflection and transmission at a tissue interface (normal incidence), `bmed_ultrasound_interface`

The intensity reflection and transmission coefficients and the pressure amplitude coefficients of an ultrasound wave meeting a flat interface between two media at normal incidence.

$$
R = \left(\frac{Z_2 - Z_1}{Z_2 + Z_1}\right)^2,\quad T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2}
$$

| input | meaning | condition |
|---|---|---|
| `Z1` | acoustic impedance of the first medium |  |
| `Z2` | acoustic impedance of the second medium |  |

| output | meaning |
|---|---|
| `R` | intensity reflection coefficient |
| `T` | intensity transmission coefficient |
| `r_p` | pressure reflection coefficient |
| `t_p` | pressure transmission coefficient |

- **Method:** $r = (Z_{2} - Z_{1})/(Z_{2} + Z_{1})$, $t = 2$ $Z_{2}/(Z_{1} + Z_{2})$; $R = r^{2}$, $T = 4$ Z1 $Z_{2}/(Z_{1} + Z_{2})^{2}$
- **Assumes:** normal incidence on a flat interface; lossless media, plane waves
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![R (intensity reflection coefficient) over Z1 from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-interface-field)

*R (intensity reflection coefficient) over Z1 from 1.38e+05 to 1.38e+07 and Z2 from 42.9 to 4290 (logarithmic axes): each of the 21 x 15 cells is one run of the model, shaded by its value (light 0.883 to dark 1); iso-lines at R = 0.9, 0.92, 0.94, 0.96, 0.98. Marked: the inputs given, Z1 = 1.38e+06, Z2 = 429, R = 0.9988. Your inputs: Z1 = 1.38e6 kg/(m^2*s), Z2 = 429 kg/(m^2*s).*

![Intensity reflection at four tissue interfaces (log axis), one run of the model each with stated acoustic impedances (MRayl: fat 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-interface-bars)

*Intensity reflection at four tissue interfaces (log axis), one run of the model each with stated acoustic impedances (MRayl: fat 1.38, muscle 1.70, soft tissue 1.63, blood 1.66, bone 7.8, air 0.00043): fat/muscle R = 0.01079; blood/muscle R = 0.0001417; tissue/bone R = 0.4281; tissue/air R = 0.9989; given/Z1, Z2 R = 0.9988. Highlighted: the impedances given (Z1 = 1.38e+06, Z2 = 429 kg/(m^2 s)), R = 0.9988, T = 0.001243; air reflects almost everything, hence coupling gel. Your inputs: Z1 = 1.38e6 kg/(m^2*s), Z2 = 429 kg/(m^2*s).*

![Intensity reflection coefficient as Z2 runs from 257](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-interface-live-1)

*Intensity reflection coefficient as Z2 runs from 257.4 to 600.6 kg/(m^2*s), every other input as you entered it (41 solutions). Marked: your Z2 = 429 kg/(m^2*s) gives R = 0.9988. Your inputs: Z1 = 1.38e6 kg/(m^2*s), Z2 = 429 kg/(m^2*s).*

![Intensity reflection coefficient as Z1 runs from 8](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-interface-live-2)

*Intensity reflection coefficient as Z1 runs from 8.28e+05 to 1.932e+06 kg/(m^2*s), every other input as you entered it (29 solutions). Marked: your Z1 = 1.38e+06 kg/(m^2*s) gives R = 0.9988. Your inputs: Z1 = 1.38e6 kg/(m^2*s), Z2 = 429 kg/(m^2*s).*

```python
biomedical_science_ai999x.solve('bmed_ultrasound_interface', Z1=1630000.0, Z2=7800000.0)
# -> R = 0.428102, T = 0.571898, r_p = 0.654295, t_p = 1.65429   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_ultrasound_interface', Z1=1380000.0, Z2=429.0)
# -> R = 0.998757, T = 0.00124271, r_p = -0.999378, t_p = 0.000621546   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Ultrasound interface reflection: Z1 = 1.63 MRayl, Z2 = 7.8 MRayl | SOLVED | R = 0.428102, T = 0.571898, r_p = 0.654295, t_p = 1.65429 |
| Ultrasound interface reflection: Z1 = 1.38e6 kg/(m^2*s), Z2 = 429 kg/(m^2*s) | SOLVED | R = 0.998757, T = 0.00124271, r_p = -0.999378, t_p = 0.000621546 |
| Ultrasound interface reflection: Z2 = 7.8 MRayl | NEEDS_INPUT | bmed_ultrasound_interface needs Z1 (acoustic impedance of the first medium). Example: solve('bmed_ultrasound_interface', |
| Ultrasound interface reflection: Z1 = 1.63 MRayl, Z2 = 7.8 MRayl, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_ultrasound_matching_layer"></a>

### Ultrasound transmission through a matching layer (normal incidence), `bmed_ultrasound_matching_layer`

The intensity transmission coefficient from a transducer (impedance Z1) through a matching layer (impedance Zm, thickness d, sound speed c_m) into tissue (Z2) at frequency f, and the layer's phase thickness.

$$
T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2\cos^2 kd + \left(Z_m + \frac{Z_1 Z_2}{Z_m}\right)^2\sin^2 kd}
$$

| input | meaning | condition |
|---|---|---|
| `Z1` | impedance of the source medium (transducer) |  |
| `Zm` | impedance of the matching layer |  |
| `Z2` | impedance of the load medium (tissue) |  |
| `d` | layer thickness |  |
| `c_m` | sound speed in the layer |  |
| `f` | frequency |  |

| output | meaning |
|---|---|
| `T` | intensity transmission coefficient |
| `kd` | phase thickness of the layer (rad) |

- **Method:** $T = 4$ Z1 Z2 / [$(Z_{1} + Z_{2})^{2}$ $cos^{2}$($kd) + (Zm + Z_{1}$ Z2/Zm)^2 $sin^{2}$(kd)], $k = 2$ pi f / c_m
- **Assumes:** normal incidence, plane waves, lossless layers; semi-infinite source and load media
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Intensity transmission coefficient as c_m runs from 1440 to 3360 m/s, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-matching-layer-live-1)

*Intensity transmission coefficient as c_m runs from 1440 to 3360 m/s, every other input as you entered it (41 solutions). Marked: your c_m = 2400 m/s gives T = 1. Your inputs: Z1 = 30000000 kg/(m^2*s), Zm = 6900000 kg/(m^2*s), Z2 = 1600000 kg/(m^2*s), d = 0.00015 m, c_m = 2400 m/s, f = 4000000 Hz.*

![Intensity transmission coefficient as d runs from 9e-05 to 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-matching-layer-live-2)

*Intensity transmission coefficient as d runs from 9e-05 to 0.00021 m, every other input as you entered it (29 solutions). Marked: your d = 0.00015 m gives T = 1. Your inputs: Z1 = 30000000 kg/(m^2*s), Zm = 6900000 kg/(m^2*s), Z2 = 1600000 kg/(m^2*s), d = 0.00015 m, c_m = 2400 m/s, f = 4000000 Hz.*

```python
biomedical_science_ai999x.solve('bmed_ultrasound_matching_layer', Z1=30000000.0, Zm=6900000.0, Z2=1600000.0, d=0.00015, c_m=2400.0, f=4000000.0)
# -> T = 0.999983, kd = 1.5708   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_ultrasound_matching_layer', Z1=30000000.0, Zm=3000000.0, Z2=1500000.0, d=0.0002, c_m=2000.0, f=2000000.0)
# -> T = 0.464142, kd = 1.25664   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Ultrasound matching layer: Z1 = 30 MRayl, Zm = 6.9 MRayl, Z2 = 1.6 MRayl, d = 0.15 mm, c_m = 2400 m/s, f = 4 MHz | SOLVED | T = 0.999983, kd = 1.5708 |
| Ultrasound matching layer: Z1 = 30 MRayl, Zm = 3 MRayl, Z2 = 1.5 MRayl, d = 0.2 mm, c_m = 2000 m/s, f = 2 MHz | SOLVED | T = 0.464142, kd = 1.25664 |
| Ultrasound matching layer: Zm = 6.9 MRayl, Z2 = 1.6 MRayl, d = 0.15 mm, c_m = 2400 m/s, f = 4 MHz | NEEDS_INPUT | bmed_ultrasound_matching_layer needs Z1 (impedance of the source medium (transducer)). Example: solve('bmed_ultrasound_m |
| Ultrasound matching layer: Z1 = 30 MRayl, Zm = 6.9 MRayl, Z2 = 1.6 MRayl, d = 0.15 mm, c_m = 2400 m/s, f = 4 MHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_gaussian_psf_mtf"></a>

### Imaging resolution: MTF of a Gaussian point-spread function, `bmed_gaussian_psf_mtf`

The modulation transfer function at a spatial frequency of an imaging system with a Gaussian point-spread function of stated FWHM, and the frequency at which the MTF falls to 50 %.

$$
\mathrm{MTF}(f) = e^{-2\pi^2\sigma^2 f^2},\quad \sigma = \frac{\mathrm{FWHM}}{2\sqrt{2\ln 2}}
$$

| input | meaning | condition |
|---|---|---|
| `fwhm` | full width at half maximum of the point-spread function |  |
| `f` | spatial frequency (cycles per length) |  |

| output | meaning |
|---|---|
| `MTF` | modulation transfer at f |
| `f50` | spatial frequency where the MTF is 0.5 |

- **Method:** sigma = FWHM/(2 sqrt(2 ln 2)); MTF = exp(-2 $pi^{2}$ $\sigma^{2}$ f^2); $f50 = sqrt$(ln $2 / 2)/(pi$ sigma)
- **Assumes:** Gaussian, shift-invariant point-spread function; one dimension (a line-spread function)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![MTF (modulation transfer at f) over fwhm from 5e-05 to 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-gaussian-psf-mtf-field)

*MTF (modulation transfer at f) over fwhm from 5e-05 to 0.005 and f from 100 to 1e+04 (logarithmic axes): each of the 21 x 15 cells is one run of the model, shaded by its value (light 0 to dark 0.9999); iso-lines at MTF = 0.2, 0.4, 0.6, 0.8. Marked: the inputs given, fwhm = 0.0005, f = 1000, MTF = 0.4107. Your inputs: fwhm = 0.5 mm, f = 1 1/mm.*

![Modulation transfer at f as fwhm runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-gaussian-psf-mtf-live-1)

*Modulation transfer at f as fwhm runs from 0.3 to 0.7 mm, every other input as you entered it (41 solutions). Marked: your fwhm = 0.5 mm gives MTF = 0.4107. Your inputs: fwhm = 0.5 mm, f = 1 1/mm.*

![Modulation transfer at f as f runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-gaussian-psf-mtf-live-2)

*Modulation transfer at f as f runs from 0.6 to 1.4 1/mm, every other input as you entered it (29 solutions). Marked: your f = 1 1/mm gives MTF = 0.4107. Your inputs: fwhm = 0.5 mm, f = 1 1/mm.*

```python
biomedical_science_ai999x.solve('bmed_gaussian_psf_mtf', fwhm=0.0005, f=1000.0)
# -> MTF = 0.410686, f50 = 882.542 1/m   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_gaussian_psf_mtf', fwhm=0.004, f=100.0)
# -> MTF = 0.565778, f50 = 110.318 1/m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Gaussian PSF MTF: fwhm = 0.5 mm, f = 1 1/mm | SOLVED | MTF = 0.410686, f50 = 882.542 1/m |
| Gaussian PSF MTF: fwhm = 4 mm, f = 0.1 1/mm | SOLVED | MTF = 0.565778, f50 = 110.318 1/m |
| Gaussian PSF MTF: f = 1 1/mm | NEEDS_INPUT | bmed_gaussian_psf_mtf needs fwhm (full width at half maximum of the point-spread function). Example: solve('bmed_gaussia |
| Gaussian PSF MTF: fwhm = 0.5 mm, f = 1 1/mm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_mri_adc_fit"></a>

### Diffusion MRI: apparent diffusion coefficient from signals at several b-values, `bmed_mri_adc_fit`

The apparent diffusion coefficient and the $b = 0$ signal from diffusion-weighted signals at two or more b-values, by a least-squares fit of ln S against b (mono-exponential decay).

$$
\ln S = \ln S_0 - b\cdot ADC
$$

| input | meaning | condition |
|---|---|---|
| `b` | b-values in $s/mm^{2}$ |  |
| `S` | signals at those b-values (any one unit) |  |

| output | meaning |
|---|---|
| `ADC` | apparent diffusion coefficient, $mm^{2}/s$ |
| `S0` | fitted signal at $b = 0$ (unit of S) |

- **Method:** least-squares line ln $S = ln$ S0 - ADC b
- **Assumes:** mono-exponential decay $S = S_{0}$ exp(-b ADC); unweighted least squares on ln S; b in $s/mm^{2}$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The diffusion-weighted signals given at b = 0, 1000 s/mm^2 (log axis) and the mono-exponential decay drawn from the returned ADC = 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-adc-fit-fit)

*The diffusion-weighted signals given at b = 0, 1000 s/mm^2 (log axis) and the mono-exponential decay drawn from the returned ADC = 0.00105 mm^2/s and S0 = 1200 (derived), extended to b = 1500. Your inputs: b = 0, 1000, S = 1200, 420.*

```python
biomedical_science_ai999x.solve('bmed_mri_adc_fit', b=[0, 1000], S=[1200, 420])
# -> ADC = 0.00104982, S0 = 1200   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_adc_fit', b=[0, 200, 500, 1000], S=[980, 820, 620, 390])
# -> ADC = 0.000923418, S0 = 983.024   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| ADC fit: b = [0, 1000], S = [1200, 420] | SOLVED | ADC = 0.00104982, S0 = 1200 |
| ADC fit: b = [0, 200, 500, 1000], S = [980, 820, 620, 390] | SOLVED | ADC = 0.000923418, S0 = 983.024 |
| ADC fit: S = [1200, 420] | NEEDS_INPUT | bmed_mri_adc_fit needs b (b-values in s/mm^2). Example: solve('bmed_mri_adc_fit', b=[0, 1000], S=[1200, 420]). |
| ADC fit: b = [0, 1000], S = [1200, 420], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_mri_t2_fit"></a>

### MRI multi-echo T2 estimate (log-linear fit), `bmed_mri_t2_fit`

The transverse relaxation time T2 and the extrapolated signal at $TE = 0$ from signals at several echo times, by a least-squares fit of ln S against TE.

$$
\ln S = \ln S_0 - \frac{TE}{T_2}
$$

| input | meaning | condition |
|---|---|---|
| `TE` | echo times in ms |  |
| `S` | signals at those echo times (any one unit) |  |

| output | meaning |
|---|---|
| `T2` | transverse relaxation time, ms |
| `S0` | fitted signal at $TE = 0$ (unit of S) |

- **Method:** least-squares line ln $S = ln$ $S_{0} - TE/T_{2}$
- **Assumes:** mono-exponential decay $S = S_{0}$ exp(-TE/T2); unweighted least squares on ln S; TE in ms
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![The 5 spin-echo signals given and the decay S0 exp(-TE/T2) drawn from the returned T2 = 99](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-t2-fit-fit)

*The 5 spin-echo signals given and the decay S0 exp(-TE/T2) drawn from the returned T2 = 99.45 ms and S0 = 999.8 (derived). Marked: at TE = T2 the signal is S0/e. Your inputs: TE = 10, 30, 50, 80, 120, S = 905, 740, 605, 445, 300.*

```python
biomedical_science_ai999x.solve('bmed_mri_t2_fit', TE=[10, 30, 50, 80, 120], S=[905, 740, 605, 445, 300])
# -> T2 = 99.4459, S0 = 999.81   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_t2_fit', TE=[20, 100], S=[800, 260])
# -> T2 = 71.1788, S0 = 1059.54   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Multi-echo T2 fit: TE = [10, 30, 50, 80, 120], S = [905, 740, 605, 445, 300] | SOLVED | T2 = 99.4459, S0 = 999.81 |
| Multi-echo T2 fit: TE = [20, 100], S = [800, 260] | SOLVED | T2 = 71.1788, S0 = 1059.54 |
| Multi-echo T2 fit: S = [905, 740, 605, 445, 300] | NEEDS_INPUT | bmed_mri_t2_fit needs TE (echo times in ms). Example: solve('bmed_mri_t2_fit', TE=[10, 30, 50, 80, 120], S=[905, 740, 60 |
| Multi-echo T2 fit: TE = [10, 30, 50, 80, 120], S = [905, 740, 605, 445, 300], zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |

<a id="bmed_snr_averaging"></a>

### Signal-to-noise ratio gained by signal averaging, `bmed_snr_averaging`

The signal-to-noise ratio after averaging N acquisitions with independent noise, SNR_N = SNR_1 sqrt(N), or the smallest number of averages that reaches a target SNR.

$$
SNR_N = SNR_1\sqrt{N}
$$

| input | meaning | condition |
|---|---|---|
| `SNR1` | signal-to-noise ratio of one acquisition |  |
| `N` | number of averages (give N or SNR_target) |  |
| `SNR_target` | target SNR (give N or SNR_target) |  |

| output | meaning |
|---|---|
| `SNR` | SNR after averaging |
| `gain` | SNR gain sqrt(N) |
| `N` | number of averages |

- **Method:** SNR_N = SNR_1 sqrt(N); N needed = ceil((SNR_target/SNR_1)^2)
- **Assumes:** independent, zero-mean noise of equal variance in every acquisition; an identical signal
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![SNR (SNR after averaging) over SNR1 from 5 to 15 and N from 8 to 24: each of the 21 x 15 cells is one run of the model, shaded by its value (light 14](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-snr-averaging-field)

*SNR (SNR after averaging) over SNR1 from 5 to 15 and N from 8 to 24: each of the 21 x 15 cells is one run of the model, shaded by its value (light 14.14 to dark 73.48); iso-lines at SNR = 20, 30, 40, 50, 60, 70. Marked: the inputs given, SNR1 = 10, N = 16, SNR = 40. Your inputs: SNR1 = 10, N = 16.*

![SNR after averaging as SNR1 runs from 2](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-snr-averaging-live-1)

*SNR after averaging as SNR1 runs from 2.5 to 17.5, every other input as you entered it (41 solutions). Marked: your SNR1 = 10 gives SNR = 40. Your inputs: SNR1 = 10, N = 16.*

![SNR after averaging as N runs from 1 to 35, every other input as you entered it (19 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-snr-averaging-live-2)

*SNR after averaging as N runs from 1 to 35, every other input as you entered it (19 solutions). Marked: your N = 16 gives SNR = 40. Your inputs: SNR1 = 10, N = 16.*

```python
biomedical_science_ai999x.solve('bmed_snr_averaging', SNR1=10, N=16)
# -> SNR = 40, gain = 4, N = 16   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_snr_averaging', SNR1=5, SNR_target=20)
# -> SNR = 20, gain = 4, N = 16   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SNR averaging: SNR1 = 10, N = 16 | SOLVED | SNR = 40, gain = 4, N = 16 |
| SNR averaging: SNR1 = 5, SNR_target = 20 | SOLVED | SNR = 20, gain = 4, N = 16 |
| SNR averaging: N = 16 | NEEDS_INPUT | bmed_snr_averaging needs SNR1 (signal-to-noise ratio of one acquisition). Example: solve('bmed_snr_averaging', SNR1=10,  |
| SNR averaging: SNR1 = 10, N = 16, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| SNR averaging: SNR1 = 10, N = 16, SNR_target = 50 | INVALID_INPUT | Signal-to-noise ratio gained by signal averaging: give the number of averages N or a target SNR (one of them). |

<a id="bmed_mri_ernst_angle"></a>

### MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal), `bmed_mri_ernst_angle`

The Ernst angle alpha_E = arccos(exp(-TR/T1)) that maximises the steady-state signal of a spoiled gradient-echo sequence, and that largest signal as a fraction of M0 (T2* decay excluded).

$$
\alpha_E = \arccos\left(e^{-TR/T_1}\right)
$$

| input | meaning | condition |
|---|---|---|
| `TR` | repetition time |  |
| `T1` | longitudinal relaxation time |  |

| output | meaning |
|---|---|
| `alpha` | Ernst angle |
| `alpha_deg` | Ernst angle in degrees |
| `S_rel` | signal at the Ernst angle, fraction of M0 (no T2* decay) |

- **Method:** cos(alpha_E) = exp(-TR/T1); $S = sin$(alpha)(1 - E1)/(1 - E1 cos alpha)
- **Assumes:** ideal spoiling, steady state; TR, $T_{1} > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![alpha_deg (Ernst angle in degrees) over TR from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-ernst-angle-field)

*alpha_deg (Ernst angle in degrees) over TR from 0.005 to 0.015 and T1 from 0.5 to 1.5: each of the 21 x 15 cells is one run of the model, shaded by its value (light 4.676 to dark 13.96); iso-lines at alpha_deg = 6, 8, 10, 12. Marked: the inputs given, TR = 0.01, T1 = 1, alpha_deg = 8.089. Your inputs: TR = 10 ms, T1 = 1000 ms.*

![Ernst angle in degrees as TR runs from 6 to 14 ms, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-ernst-angle-live-1)

*Ernst angle in degrees as TR runs from 6 to 14 ms, every other input as you entered it (41 solutions). Marked: your TR = 10 ms gives α_deg = 8.089. Your inputs: TR = 10 ms, T1 = 1000 ms.*

![Ernst angle in degrees as T1 runs from 600 to 1400 ms, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-mri-ernst-angle-live-2)

*Ernst angle in degrees as T1 runs from 600 to 1400 ms, every other input as you entered it (29 solutions). Marked: your T1 = 1000 ms gives α_deg = 8.089. Your inputs: TR = 10 ms, T1 = 1000 ms.*

```python
biomedical_science_ai999x.solve('bmed_mri_ernst_angle', TR=0.01, T1=1.0)
# -> alpha = 0.141186 rad, alpha_deg = 8.08935, S_rel = 0.0707104   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_mri_ernst_angle', TR=0.5, T1=0.9)
# -> alpha = 0.959715 rad, alpha_deg = 54.9876, S_rel = 0.52043   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Ernst angle: TR = 10 ms, T1 = 1000 ms | SOLVED | alpha = 0.141186 rad, alpha_deg = 8.08935, S_rel = 0.0707104 |
| Ernst angle: TR = 500 ms, T1 = 900 ms | SOLVED | alpha = 0.959715 rad, alpha_deg = 54.9876, S_rel = 0.52043 |
| Ernst angle: T1 = 1000 ms | NEEDS_INPUT | bmed_mri_ernst_angle needs TR (repetition time). Example: solve('bmed_mri_ernst_angle', TR='0.01 s', T1='1 s'). |
| Ernst angle: TR = 10 ms, T1 = 1000 ms, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Ernst angle: TR = -10 ms, T1 = 1000 ms | INVALID_INPUT | MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal): TR and T1 must be a positive number. |

<a id="bmed_epi_seir_peak"></a>

### SEIR epidemic model: peak prevalence and the time of the peak, `bmed_epi_seir_peak`

The largest infectious fraction of an SEIR epidemic and the time at which it occurs, from the transmission, incubation and recovery rates and the initial fractions.

$$
\dot s = -\beta s i,\ \dot e = \beta s i - \sigma e,\ \dot i = \sigma e - \gamma i;\quad t_{peak}:\ \sigma e = \gamma i
$$

| input | meaning | condition |
|---|---|---|
| `beta` | transmission rate |  |
| `sigma` | rate of leaving the latent stage (1/latent period) |  |
| `gamma` | recovery rate |  |
| `s0` | initial susceptible fraction |  |
| `e0` | initial exposed (latent) fraction |  |
| `i0` | initial infectious fraction |  |

| output | meaning |
|---|---|
| `t_peak` | time of the peak of the infectious fraction |
| `i_max` | peak infectious fraction |
| `s_at_peak` | susceptible fraction at the peak |
| `e_at_peak` | exposed fraction at the peak |

- **Method:** Taylor-series integration (order 26, steps set by the series' tail) of the SEIR system to the first sign change of sigma e - gamma i, the root located on the series
- **Assumes:** closed population, homogeneous mixing, frequency-dependent transmission; R0 $s_{0} > 1$, $e_{0} + i_{0} > 0$; the peak is the first maximum of i after $t = 0$ and exceeds i0
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Time of the peak of the infectious fraction as s0 runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-peak-live-1)

*Time of the peak of the infectious fraction as s0 runs from 0.2475 to 0.99, every other input as you entered it (21 solutions). Marked: your s0 = 0.99 gives t_peak = 36.77 day. Your inputs: beta = 0.5 1/day, sigma = 0.2 1/day, gamma = 0.1 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002.*

![Time of the peak of the infectious fraction as γ runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-seir-peak-live-2)

*Time of the peak of the infectious fraction as γ runs from 0.06 to 0.14 1/day, every other input as you entered it (29 solutions). Marked: your γ = 0.1 1/day gives t_peak = 36.77 day. Your inputs: beta = 0.5 1/day, sigma = 0.2 1/day, gamma = 0.1 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002.*

```python
biomedical_science_ai999x.solve('bmed_epi_seir_peak', beta=5.787037037037037e-06, sigma=2.3148148148148148e-06, gamma=1.1574074074074074e-06, s0=0.99, e0=0.008, i0=0.002)
# -> t_peak = 3.17722e+06 s, i_max = 0.30625, s_at_peak = 0.122176, e_at_peak = 0.153125   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_seir_peak', beta=3.472222222222222e-06, sigma=5.787037037037037e-06, gamma=1.736111111111111e-06, s0=0.95, e0=0.01, i0=0.005)
# -> t_peak = 3.52552e+06 s, i_max = 0.110337, s_at_peak = 0.475211, e_at_peak = 0.033101   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| SEIR peak: beta = 0.5 1/day, sigma = 0.2 1/day, gamma = 0.1 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002 | SOLVED | t_peak = 3.17722e+06 s, i_max = 0.30625, s_at_peak = 0.122176, e_at_peak = 0.153125 |
| SEIR peak: beta = 0.3 1/day, sigma = 0.5 1/day, gamma = 0.15 1/day, s0 = 0.95, e0 = 0.01, i0 = 0.005 | SOLVED | t_peak = 3.52552e+06 s, i_max = 0.110337, s_at_peak = 0.475211, e_at_peak = 0.033101 |
| SEIR peak: sigma = 0.2 1/day, gamma = 0.1 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002 | NEEDS_INPUT | bmed_epi_seir_peak needs beta (transmission rate). Example: solve('bmed_epi_seir_peak', beta='5.787037037037037e-06 1/s' |
| SEIR peak: beta = 0.5 1/day, sigma = 0.2 1/day, gamma = 0.1 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| SEIR peak: beta = 0.1 1/day, sigma = 0.2 1/day, gamma = 0.2 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002 | INVALID_INPUT | SEIR epidemic model: peak prevalence and the time of the peak: no epidemic peak: R0 s0 must exceed 1. |

<a id="bmed_bmi"></a>

### Body mass index (a calculation, not medical advice), `bmed_bmi`

The body mass index BMI = m/h^2 $(kg/m^{2})$ and BMI prime (BMI/25) as a calculation from mass and height; no category or health advice is given.

$$
\mathrm{BMI} = \frac{m}{h^2}
$$

| input | meaning | condition |
|---|---|---|
| `m` | body mass |  |
| `h` | height |  |

| output | meaning |
|---|---|
| `BMI` | body mass index in $kg/m^{2}$ |
| `BMI_prime` | BMI/25 |

- **Method:** BMI = m/h^2
- **Assumes:** a calculation only: not medical advice; $1  \leq  m  \leq  700$ kg, $0.3  \leq  h  \leq  2.8$ m
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BMI (body mass index in kg/m^2) over h from 0](https://huggingface.co/AI999X/biomedical-science-ai999x/resolve/main/figures/bmed-bmi-field.svg)

*BMI (body mass index in kg/m^2) over h from 0.875 to 2.625 and m from 35 to 105: each of the 21 x 15 cells is one run of the model, shaded by its value (light 5.079 to dark 137.1); iso-lines at BMI = 20, 40, 60, 80, 100, 120. Marked: the inputs given, h = 1.75, m = 70, BMI = 22.86. Your inputs: m = 70 kg, h = 1.75 m.*

![BMI as h runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmi-live-1)

*BMI as h runs from 1.05 to 2.45 m, every other input as you entered it (41 solutions). Marked: your h = 1.75 m gives BMI = 22.86. Your inputs: m = 70 kg, h = 1.75 m.*

![BMI as m runs from 42 to 98 kg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmi-live-2)

*BMI as m runs from 42 to 98 kg, every other input as you entered it (29 solutions). Marked: your m = 70 kg gives BMI = 22.86. Your inputs: m = 70 kg, h = 1.75 m.*

```python
biomedical_science_ai999x.solve('bmed_bmi', m=70.0, h=1.75)
# -> BMI = 22.8571, BMI_prime = 0.914286   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_bmi', m=90.0, h=1.8)
# -> BMI = 27.7778, BMI_prime = 1.11111   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Body mass index: m = 70 kg, h = 1.75 m | SOLVED | BMI = 22.8571, BMI_prime = 0.914286 |
| Body mass index: m = 90 kg, h = 180 cm | SOLVED | BMI = 27.7778, BMI_prime = 1.11111 |
| Body mass index: h = 1.75 m | NEEDS_INPUT | bmed_bmi needs m (body mass). Example: solve('bmed_bmi', m='70 kg', h='1.75 m'). |
| Body mass index: m = 70 kg, h = 1.75 m, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Body mass index: m = 70 kg, h = 5 m | INVALID_INPUT | Body mass index (a calculation, not medical advice): the height must lie between 0.3 and 2.8 m and the mass between 1 an |

<a id="bmed_bmr"></a>

### Basal metabolic rate by a stated regression (a calculation, not medical advice), `bmed_bmr`

The basal metabolic rate estimated by the Mifflin-St Jeor (1990) or the revised Harris-Benedict (Roza and Shizgal 1984) regression from mass, height, age and sex, in kcal/day, W and MJ/day; a model calculation, not dietary or medical advice.

$$
\mathrm{BMR} = 10\,m + 6.25\,h - 5\,a + s
$$

| input | meaning | condition |
|---|---|---|
| `m` | body mass |  |
| `h` | height |  |
| `age` | age in years (adults) |  |
| `sex` | sex (the equations have a male and a female form) |  |
| `equation` | regression: mifflin_st_jeor or harris_benedict_revised |  |

| output | meaning |
|---|---|
| `BMR_kcal_day` | basal metabolic rate in kcal/day |
| `BMR_W` | basal metabolic rate (W) |
| `BMR_MJ_day` | basal metabolic rate in MJ/day |

- **Method:** Mifflin-St Jeor: 10 $m + 6.25$ $h_cm - 5$ age + 5 (male) $or - 161$ (female); revised Harris-Benedict: $88.362 + 13.397$ $m + 4.799$ $h_cm - 5.677$ age (male), $447.593 + 9.247$ $m + 3.098$ $h_cm - 4.330$ age (female)
- **Assumes:** a population regression, not a measurement and not advice; adults 18 to 120 years; 1 kcal = 4.184 kJ
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![BMR_kcal_day (basal metabolic rate in kcal/day) against m (body mass) from 35 to 105, one curve for each value of h (height): h = 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmr-curve-2)

*BMR_kcal_day (basal metabolic rate in kcal/day) against m (body mass) from 35 to 105, one curve for each value of h (height): h = 1.75 m, h = 875 mm. The other inputs as given; 31 points per curve, each a run of the model. Marked: m = 70, BMR_kcal_day = 1649. Your inputs: m = 70 kg, h = 1.75 m, age = 30, sex = male, equation = mifflin_st_jeor.*

![Basal metabolic rate in kcal/day as h runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmr-live-1)

*Basal metabolic rate in kcal/day as h runs from 1.05 to 2.45 m, every other input as you entered it (41 solutions). Marked: your h = 1.75 m gives BMR_kcal_day = 1649. Your inputs: m = 70 kg, h = 1.75 m, age = 30, sex = male, equation = mifflin_st_jeor.*

![Basal metabolic rate in kcal/day for every choice of sex, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmr-live-2)

*Basal metabolic rate in kcal/day for every choice of sex, every other input as you entered it (highlighted: your choice). male: 1649; female: 1483. Your inputs: m = 70 kg, h = 1.75 m, age = 30, sex = male, equation = mifflin_st_jeor.*

![Basal metabolic rate in kcal/day as m runs from 42 to 98 kg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-bmr-live-3)

*Basal metabolic rate in kcal/day as m runs from 42 to 98 kg, every other input as you entered it (29 solutions). Marked: your m = 70 kg gives BMR_kcal_day = 1649. Your inputs: m = 70 kg, h = 1.75 m, age = 30, sex = male, equation = mifflin_st_jeor.*

```python
biomedical_science_ai999x.solve('bmed_bmr', m=70.0, h=1.75, age=30, sex='male', equation='mifflin_st_jeor')
# -> BMR_kcal_day = 1648.75, BMR_W = 79.8422 W, BMR_MJ_day = 6.89837   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_bmr', m=60.0, h=1.6500000000000001, age=45, sex='female', equation='harris_benedict_revised')
# -> BMR_kcal_day = 1318.73, BMR_W = 63.8609 W, BMR_MJ_day = 5.51758   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Basal metabolic rate: m = 70 kg, h = 175 cm, age = 30, sex = male, equation = mifflin_st_jeor | SOLVED | BMR_kcal_day = 1648.75, BMR_W = 79.8422 W, BMR_MJ_day = 6.89837 |
| Basal metabolic rate: m = 60 kg, h = 165 cm, age = 45, sex = female, equation = harris_benedict_revised | SOLVED | BMR_kcal_day = 1318.73, BMR_W = 63.8609 W, BMR_MJ_day = 5.51758 |
| Basal metabolic rate: h = 175 cm, age = 30, sex = male, equation = mifflin_st_jeor | NEEDS_INPUT | bmed_bmr needs m (body mass). Example: solve('bmed_bmr', m='70 kg', h='1.75 m', age=30, sex='male', equation='mifflin_st |
| Basal metabolic rate: m = 70 kg, h = 175 cm, age = 30, sex = male, equation = mifflin_st_jeor, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Basal metabolic rate: m = 70 kg, h = 175 cm, age = 10, sex = male, equation = mifflin_st_jeor | INVALID_INPUT | Basal metabolic rate by a stated regression (a calculation, not medical advice): the equations are stated for adults: ag |

<a id="bmed_hounsfield_units"></a>

### CT number in Hounsfield units: `bmed_hounsfield_units`

The CT number $HU = 1000$ (mu - mu_water)/(mu_water - mu_air) of a material from its linear attenuation coefficient and those of water and air (air 0 by default, then 1000 (mu - mu_water)/mu_water).

$$
HU = 1000\,\frac{\mu - \mu_{water}}{\mu_{water} - \mu_{air}}
$$

| input | meaning | condition |
|---|---|---|
| `mu` | linear attenuation coefficient of the material |  |
| `mu_water` | linear attenuation coefficient of water |  |
| `mu_air` | linear attenuation coefficient of air (default 0) |  |

| output | meaning |
|---|---|
| `HU` | CT number in Hounsfield units |
| `mu_rel` | mu/mu_water |

- **Method:** $HU = 1000$ (mu - mu_water)/(mu_water - mu_air)
- **Assumes:** coefficients at the same effective energy; 0 <= mu_air < mu_water
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![CT number in Hounsfield units as μ runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hounsfield-units-live-1)

*CT number in Hounsfield units as μ runs from 0.228 to 0.532 1/cm, every other input as you entered it (41 solutions). Marked: your μ = 0.38 1/cm gives HU = 1000. Your inputs: mu = 0.38 1/cm, mu_water = 0.19 1/cm.*

![CT number in Hounsfield units as μ_water runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-hounsfield-units-live-2)

*CT number in Hounsfield units as μ_water runs from 0.114 to 0.266 1/cm, every other input as you entered it (29 solutions). Marked: your μ_water = 0.19 1/cm gives HU = 1000. Your inputs: mu = 0.38 1/cm, mu_water = 0.19 1/cm.*

```python
biomedical_science_ai999x.solve('bmed_hounsfield_units', mu=38.0, mu_water=19.0)
# -> HU = 1000, mu_rel = 2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_hounsfield_units', mu=20.0, mu_water=19.0, mu_air=0.02)
# -> HU = 52.687, mu_rel = 1.05263   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Hounsfield units: mu = 0.38 1/cm, mu_water = 0.19 1/cm | SOLVED | HU = 1000, mu_rel = 2 |
| Hounsfield units: mu = 0.2 1/cm, mu_water = 0.19 1/cm, mu_air = 0.0002 1/cm | SOLVED | HU = 52.687, mu_rel = 1.05263 |
| Hounsfield units: mu_water = 0.19 1/cm | NEEDS_INPUT | bmed_hounsfield_units needs mu (linear attenuation coefficient of the material). Example: solve('bmed_hounsfield_units', |
| Hounsfield units: mu = 0.38 1/cm, mu_water = 0.19 1/cm, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Hounsfield units: mu = -0.38 1/cm, mu_water = 0.19 1/cm | INVALID_INPUT | CT number in Hounsfield units: the attenuation coefficients must be a positive number. |

<a id="bmed_larmor_frequency"></a>

### Larmor (precession) frequency in a magnetic field, `bmed_larmor_frequency`

The Larmor frequency f = (gamma/2 pi) B of nuclear spins in a field B: protons by default (42.577478461 MHz/T, CODATA 2022), or another nucleus with its stated $\gamma/2$ pi in MHz/T.

$$
f = \frac{\gamma}{2\pi}\,B_0
$$

| input | meaning | condition |
|---|---|---|
| `B` | magnetic flux density (T) |  |
| `gamma_bar` | reduced gyromagnetic ratio $\gamma/2$ pi in MHz/T (default: the proton) |  |

| output | meaning |
|---|---|
| `f` | Larmor frequency |
| `f_MHz` | Larmor frequency in MHz |
| `omega` | angular Larmor frequency |

- **Method:** f = (gamma/2 pi) B
- **Assumes:** $B > 0$; bare nucleus (no chemical shielding)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Larmor frequency in MHz as B runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-larmor-frequency-live-1)

*Larmor frequency in MHz as B runs from 0.9 to 2.1 T, every other input as you entered it (41 solutions). Marked: your B = 1.5 T gives f_MHz = 63.87. Your inputs: B = 1.5 T.*

```python
biomedical_science_ai999x.solve('bmed_larmor_frequency', B=1.5)
# -> f = 6.38662e+07 Hz, f_MHz = 63.8662, omega = 4.01283e+08 rad/s   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_larmor_frequency', B=3.0, gamma_bar=17.235)
# -> f = 5.1705e+07 Hz, f_MHz = 51.705, omega = 3.24872e+08 rad/s   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Larmor frequency: B = 1.5 T | SOLVED | f = 6.38662e+07 Hz, f_MHz = 63.8662, omega = 4.01283e+08 rad/s |
| Larmor frequency: B = 3 T, gamma_bar = 17.235 | SOLVED | f = 5.1705e+07 Hz, f_MHz = 51.705, omega = 3.24872e+08 rad/s |
| Larmor frequency:  | NEEDS_INPUT | bmed_larmor_frequency needs B (magnetic flux density (T)). Example: solve('bmed_larmor_frequency', B='1.5 T'). |
| Larmor frequency: B = 1.5 T, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Larmor frequency: B = -3 T | INVALID_INPUT | Larmor (precession) frequency in a magnetic field: the magnetic field must be a positive number. |

<a id="bmed_ultrasound_wavelength"></a>

### Ultrasound wavelength in tissue: `bmed_ultrasound_wavelength`

The wavelength $\lambda = c/f$ of ultrasound of frequency f in a medium with speed of sound c (1540 m/s, the soft-tissue convention, by default), with the period and wavenumber.

$$
\lambda = \frac{c}{f}
$$

| input | meaning | condition |
|---|---|---|
| `f` | ultrasound frequency |  |
| `c` | speed of sound (default 1540 m/s) |  |

| output | meaning |
|---|---|
| `wavelength` | wavelength |
| `wavelength_mm` | wavelength in mm |
| `period` | period $1/f$ |
| `k` | wavenumber 2 pi/lambda |

- **Method:** $\lambda = c/f$
- **Assumes:** f, $c > 0$; $c = 1540$ m/s unless stated
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Wavelength in mm as f runs from 3 to 7 MHz, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-ultrasound-wavelength-live-1)

*Wavelength in mm as f runs from 3 to 7 MHz, every other input as you entered it (41 solutions). Marked: your f = 5 MHz gives wavelength_mm = 0.308. Your inputs: f = 5 MHz.*

```python
biomedical_science_ai999x.solve('bmed_ultrasound_wavelength', f=5000000.0)
# -> wavelength = 0.000308 m, wavelength_mm = 0.308, period = 2e-07 s, k = 20400 1/m   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_ultrasound_wavelength', f=3500000.0, c=1580.0)
# -> wavelength = 0.000451429 m, wavelength_mm = 0.451429, period = 2.85714e-07 s, k = 13918.4 1/m   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Ultrasound wavelength: f = 5 MHz | SOLVED | wavelength = 0.000308 m, wavelength_mm = 0.308, period = 2e-07 s, k = 20400 1/m |
| Ultrasound wavelength: f = 3.5 MHz, c = 1580 m/s | SOLVED | wavelength = 0.000451429 m, wavelength_mm = 0.451429, period = 2.85714e-07 s, k = 13918.4 1/m |
| Ultrasound wavelength:  | NEEDS_INPUT | bmed_ultrasound_wavelength needs f (ultrasound frequency). Example: solve('bmed_ultrasound_wavelength', f='5e+06 Hz'). |
| Ultrasound wavelength: f = 5 MHz, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Ultrasound wavelength: f = 0 MHz | INVALID_INPUT | Ultrasound wavelength in tissue: the frequency and the speed of sound must be a positive number. |

<a id="bmed_epi_basic_rates"></a>

### Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality, `bmed_epi_basic_rates`

The incidence proportion (attack rate) and, when given, the incidence rate per person-year, the point prevalence, the case-fatality rate and the mortality, from case counts in one population.

$$
IP = \frac{\text{new cases}}{N},\quad P = \frac{\text{existing cases}}{N},\quad CFR = \frac{\text{deaths}}{\text{cases}}
$$

| input | meaning | condition |
|---|---|---|
| `cases` | new cases during the period |  |
| `population` | population at risk at the start of the period |  |
| `prevalent` | existing cases at a point in time (optional, for the prevalence) |  |
| `deaths` | deaths from the disease among its cases (optional) |  |
| `period` | length of the observation period (optional, for rates per person-year) |  |

| output | meaning |
|---|---|
| `incidence_proportion` | incidence proportion (attack rate) = new cases / population |
| `incidence_per_100000` | incidence proportion per 100,000 |
| `incidence_rate` | incidence rate per person and unit time (when the period is given) |
| `incidence_per_100000_person_years` | incidence rate per 100,000 person-years (when the period is given) |
| `prevalence` | point prevalence = existing cases / population (when given) |
| `prevalence_per_100000` | prevalence per 100,000 |
| `case_fatality` | case-fatality rate = deaths / cases (when deaths are given) |
| `mortality_per_100000` | disease mortality per 100,000 population over the period |
| `mortality_per_100000_person_years` | mortality rate per 100,000 person-years (deaths and period given) |

- **Method:** incidence proportion = cases/N; incidence rate = cases/(N T); prevalence = existing cases/N; case fatality = deaths/cases; mortality = deaths/N
- **Assumes:** 0 <= cases <= population; existing cases <= population; deaths <= cases; one closed population, constant over the period
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Incidence rate per person and unit time](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-basic-rates-live-1)

*Incidence rate per person and unit time... as period runs from 0.6 to 1.4 yr, every other input as you entered it (41 solutions). Marked: your period = 1 yr gives incidence_rate = 4.753e-10. Your inputs: cases = 150, population = 10000, prevalent = 400, deaths = 6, period = 1 yr.*

![Mortality rate per 100,000 person-years](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-basic-rates-live-2)

*Mortality rate per 100,000 person-years... as deaths runs from 1 to 11, every other input as you entered it (11 solutions). Marked: your deaths = 6 gives mortality_per_100000_person_years = 60. Your inputs: cases = 150, population = 10000, prevalent = 400, deaths = 6, period = 1 yr.*

```python
biomedical_science_ai999x.solve('bmed_epi_basic_rates', cases=150, population=10000, prevalent=400, deaths=6, period=31557600.0)
# -> incidence_proportion = 0.015, incidence_per_100000 = 1500, incidence_rate = 4.75321e-10 Hz, incidence_per_100000_person_years = 1500, prevalence = 0.04, prevalence_per_100000 = 4000, case_fatality = 0.04, mortality_per_100000 = 60, mortality_per_100000_person_years = 60   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_basic_rates', cases=30, population=1200, deaths=3)
# -> incidence_proportion = 0.025, incidence_per_100000 = 2500, case_fatality = 0.1, mortality_per_100000 = 250   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Epidemiological rates: cases = 150, population = 10000, prevalent = 400, deaths = 6, period = 1 yr | SOLVED | incidence_proportion = 0.015, incidence_per_100000 = 1500, incidence_rate = 4.75321e-10 Hz, incidence_per_100000_person_years = 1500, prevalence = 0.04, prevalence_per_100000 = 4000, case_fatality = 0.04, mortality_per_100000 = 60, mortality_per_100000_person_years = 60 |
| Epidemiological rates: cases = 30, population = 1200, deaths = 3 | SOLVED | incidence_proportion = 0.025, incidence_per_100000 = 2500, case_fatality = 0.1, mortality_per_100000 = 250 |
| Epidemiological rates: population = 10000, prevalent = 400, deaths = 6, period = 1 yr | NEEDS_INPUT | bmed_epi_basic_rates needs cases (new cases during the period). Example: solve('bmed_epi_basic_rates', cases=150, popula |
| Epidemiological rates: cases = 150, population = 10000, prevalent = 400, deaths = 6, period = 1 yr, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Attack rate and case fatality: cases = 30, population = 1200, deaths = 3 | SOLVED | incidence_proportion = 0.025, incidence_per_100000 = 2500, case_fatality = 0.1, mortality_per_100000 = 250 |
| Epidemiological rates: cases = 30, population = 1200, deaths = 31 | INVALID_INPUT | Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality: the deaths from the diseas |

<a id="bmed_dx_predictive_values"></a>

### Predictive values and likelihood ratios of a diagnostic test, `bmed_dx_predictive_values`

The positive and negative predictive values of a test at a stated prevalence (pre-test probability), and its positive and negative likelihood ratios and diagnostic odds ratio.

$$
PPV = \frac{Se\,p}{Se\,p + (1-Sp)(1-p)},\quad NPV = \frac{Sp(1-p)}{Sp(1-p) + (1-Se)p},\quad LR^{+} = \frac{Se}{1-Sp}
$$

| input | meaning | condition |
|---|---|---|
| `sensitivity` | sensitivity P(positive \| disease), a fraction or % |  |
| `specificity` | specificity P(negative \| no disease), a fraction or % |  |
| `prevalence` | prevalence (pre-test probability of disease), a fraction or % |  |

| output | meaning |
|---|---|
| `PPV` | positive predictive value P(disease \| positive) |
| `NPV` | negative predictive value P(no disease \| negative) |
| `LR_pos` | positive likelihood ratio sens/(1 - spec) |
| `LR_neg` | negative likelihood ratio (1 - sens)/spec |
| `DOR` | diagnostic odds ratio LR+/LR- |

- **Method:** PPV = Se p / (Se $p + (1 - Sp)$$(1 - p)$); NPV = Sp (1 - p) / (Sp $(1 - p) + (1 - Se)$ p); LR+ = $Se/(1 - Sp)$; LR- = $(1 - Se)/Sp$
- **Assumes:** 0 < sensitivity, specificity, prevalence < 1; the test's accuracy does not depend on prevalence
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![PPV as specificity runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-predictive-values-live-1)

*PPV as specificity runs from 0.225 to 0.9675, every other input as you entered it (23 solutions). Marked: your specificity = 0.9 gives PPV = 0.7742. Your inputs: sensitivity = 0.8, specificity = 0.9, prevalence = 0.3.*

![PPV as sensitivity runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dx-predictive-values-live-2)

*PPV as sensitivity runs from 0.2 to 0.9714, every other input as you entered it (19 solutions). Marked: your sensitivity = 0.8 gives PPV = 0.7742. Your inputs: sensitivity = 0.8, specificity = 0.9, prevalence = 0.3.*

```python
biomedical_science_ai999x.solve('bmed_dx_predictive_values', sensitivity=0.9, specificity=0.95, prevalence=0.02)
# -> PPV = 0.268657, NPV = 0.997856, LR_pos = 18, LR_neg = 0.105263, DOR = 171   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dx_predictive_values', sensitivity=0.8, specificity=0.9, prevalence=0.3)
# -> PPV = 0.774194, NPV = 0.913043, LR_pos = 8, LR_neg = 0.222222, DOR = 36   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Predictive values and likelihood ratios: sensitivity = 90%, specificity = 95%, prevalence = 2% | SOLVED | PPV = 0.268657, NPV = 0.997856, LR_pos = 18, LR_neg = 0.105263, DOR = 171 |
| Predictive values and likelihood ratios: sensitivity = 0.8, specificity = 0.9, prevalence = 0.3 | SOLVED | PPV = 0.774194, NPV = 0.913043, LR_pos = 8, LR_neg = 0.222222, DOR = 36 |
| Predictive values and likelihood ratios: specificity = 95%, prevalence = 2% | NEEDS_INPUT | bmed_dx_predictive_values needs sensitivity (sensitivity P(positive \| disease), a fraction or %). Example: solve('bmed_d |
| Predictive values and likelihood ratios: sensitivity = 90%, specificity = 95%, prevalence = 2%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Likelihood ratios of a test: sensitivity = 0.9, specificity = 0.95, prevalence = 0.02 | SOLVED | PPV = 0.268657, NPV = 0.997856, LR_pos = 18, LR_neg = 0.105263, DOR = 171 |
| Predictive values and likelihood ratios: sensitivity = 90%, specificity = 100%, prevalence = 2% | INVALID_INPUT | Predictive values and likelihood ratios of a diagnostic test: the specificity must be a fraction between 0 and 1. |

<a id="bmed_epi_herd_immunity"></a>

### Herd-immunity threshold and critical vaccination coverage, `bmed_epi_herd_immunity`

The fraction of a population that must be immune to stop sustained spread, $1 - 1/R_{0}$, and, for a vaccine of stated efficacy, the vaccination coverage that achieves it (and whether it is reachable).

$$
H = 1 - \frac{1}{R_0},\quad V_c = \frac{1 - 1/R_0}{E}
$$

| input | meaning | condition |
|---|---|---|
| `R0` | basic reproduction number |  |
| `efficacy` | vaccine efficacy against infection, a fraction or % (optional) |  |

| output | meaning |
|---|---|
| `threshold` | herd-immunity threshold $1 - 1/R_{0}$ (immune fraction) |
| `threshold_percent` | threshold in % |
| `coverage` | critical vaccination coverage $(1 - 1/R_{0})/E$ (when the efficacy is given) |
| `coverage_percent` | critical coverage in % |
| `reachable` | reachable (coverage <= 100 %) or not reachable with this vaccine |

- **Method:** $H = 1 - 1/R_{0}$; $V_c = H / E$
- **Assumes:** $1 < R_{0}  \leq  100$; 0 < efficacy <= 1; homogeneous mixing, lasting all-or-nothing immunity
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Critical coverage in % as efficacy runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-herd-immunity-live-1)

*Critical coverage in % as efficacy runs from 0.2425 to 0.97, every other input as you entered it (21 solutions). Marked: your efficacy = 0.97 gives coverage_percent = 96.22. Shaded by reachable (coverage <= 100 %) or not reachable with this vaccine: not reachable with this vaccine, reachable. Your inputs: R0 = 15, efficacy = 0.97.*

![Reachable (coverage <= 100 %) or not reachable with this vaccine for 15 5 x 5 combinations of efficacy and R0 around your inputs (the other inputs as entered); the ring marks your inputs, which give reachable](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-herd-immunity-live-2)

*Reachable (coverage <= 100 %) or not reachable with this vaccine for 15 5 x 5 combinations of efficacy and R0 around your inputs (the other inputs as entered); the ring marks your inputs, which give reachable. Blank cells admit no answer. Your inputs: R0 = 15, efficacy = 0.97.*

![Herd-immunity threshold 1 - 1/R0 (immune](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-epi-herd-immunity-live-3)

*Herd-immunity threshold 1 - 1/R0 (immune... as R0 runs from 3.75 to 26.25, every other input as you entered it (29 solutions). Marked: your R0 = 15 gives threshold = 0.9333. Your inputs: R0 = 15, efficacy = 0.97.*

```python
biomedical_science_ai999x.solve('bmed_epi_herd_immunity', R0=4, efficacy=0.9)
# -> threshold = 0.75, threshold_percent = 75, coverage = 0.833333, coverage_percent = 83.3333, reachable = reachable   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_herd_immunity', R0=15, efficacy=0.97)
# -> threshold = 0.933333, threshold_percent = 93.3333, coverage = 0.962199, coverage_percent = 96.2199, reachable = reachable   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_epi_herd_immunity', R0=18, efficacy=0.9)
# -> threshold = 0.944444, threshold_percent = 94.4444, coverage = 1.04938, coverage_percent = 104.938, reachable = not reachable with this vaccine   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Herd-immunity threshold: R0 = 4, efficacy = 90% | SOLVED | threshold = 0.75, threshold_percent = 75, coverage = 0.833333, coverage_percent = 83.3333, reachable = reachable |
| Herd-immunity threshold: R0 = 15, efficacy = 0.97 | SOLVED | threshold = 0.933333, threshold_percent = 93.3333, coverage = 0.962199, coverage_percent = 96.2199, reachable = reachable |
| Herd-immunity threshold: R0 = 18, efficacy = 0.9 | SOLVED | threshold = 0.944444, threshold_percent = 94.4444, coverage = 1.04938, coverage_percent = 104.938, reachable = not reachable with this vaccine |
| Herd-immunity threshold: efficacy = 90% | NEEDS_INPUT | bmed_epi_herd_immunity needs R0 (basic reproduction number). Example: solve('bmed_epi_herd_immunity', R0=4, efficacy=0.9 |
| Herd-immunity threshold: R0 = 4, efficacy = 90%, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Critical vaccination coverage: R0 = 4, efficacy = 0.9 | SOLVED | threshold = 0.75, threshold_percent = 75, coverage = 0.833333, coverage_percent = 83.3333, reachable = reachable |
| Herd-immunity threshold: R0 = 0.8, efficacy = 90% | INVALID_INPUT | Herd-immunity threshold and critical vaccination coverage: R0 must exceed 1 (below 1 an infection dies out without vacci |

<a id="bmed_vaccine_efficacy"></a>

### Vaccine efficacy from attack rates: `bmed_vaccine_efficacy`

Vaccine efficacy VE = (ARU - ARV)/ARU = 1 - RR from the attack rates of the vaccinated and the unvaccinated, with the risk ratio, the absolute risk reduction and the number needed to vaccinate.

$$
VE = \frac{ARU - ARV}{ARU} = 1 - \frac{ARV}{ARU}
$$

| input | meaning | condition |
|---|---|---|
| `cases_v` | cases among the vaccinated |  |
| `n_v` | number vaccinated |  |
| `cases_u` | cases among the unvaccinated |  |
| `n_u` | number unvaccinated |  |

| output | meaning |
|---|---|
| `ARV` | attack rate in the vaccinated |
| `ARU` | attack rate in the unvaccinated |
| `RR` | risk ratio ARV/ARU |
| `VE` | vaccine efficacy 1 - ARV/ARU (fraction) |
| `VE_percent` | vaccine efficacy in % |
| `ARR` | absolute risk reduction ARU - ARV |
| `NNV` | number needed to vaccinate to prevent one case, 1/ARR (when ARR > 0) |

- **Method:** ARV = cases_v/n_v; ARU = cases_u/n_u; VE = (ARU - ARV)/ARU = 1 - ARV/ARU
- **Assumes:** cases <= group size; at least one unvaccinated case; comparable groups, equal follow-up
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Vaccine efficacy 1 - ARV/ARU as cases_v runs from 0 to 40, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-vaccine-efficacy-live-1)

*Vaccine efficacy 1 - ARV/ARU as cases_v runs from 0 to 40, every other input as you entered it (41 solutions). Marked: your cases_v = 20 gives VE = 0.8. Your inputs: cases_v = 20, n_v = 1000, cases_u = 100, n_u = 1000.*

![Vaccine efficacy 1 - ARV/ARU as cases_u runs from 80 to 120, every other input as you entered it (21 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-vaccine-efficacy-live-2)

*Vaccine efficacy 1 - ARV/ARU as cases_u runs from 80 to 120, every other input as you entered it (21 solutions). Marked: your cases_u = 100 gives VE = 0.8. Your inputs: cases_v = 20, n_v = 1000, cases_u = 100, n_u = 1000.*

```python
biomedical_science_ai999x.solve('bmed_vaccine_efficacy', cases_v=20, n_v=1000, cases_u=100, n_u=1000)
# -> ARV = 0.02, ARU = 0.1, RR = 0.2, VE = 0.8, VE_percent = 80, ARR = 0.08, NNV = 12.5   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_vaccine_efficacy', cases_v=15, n_v=600, cases_u=60, n_u=400)
# -> ARV = 0.025, ARU = 0.15, RR = 0.166667, VE = 0.833333, VE_percent = 83.3333, ARR = 0.125, NNV = 8   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Vaccine efficacy: cases_v = 20, n_v = 1000, cases_u = 100, n_u = 1000 | SOLVED | ARV = 0.02, ARU = 0.1, RR = 0.2, VE = 0.8, VE_percent = 80, ARR = 0.08, NNV = 12.5 |
| Vaccine efficacy: cases_v = 15, n_v = 600, cases_u = 60, n_u = 400 | SOLVED | ARV = 0.025, ARU = 0.15, RR = 0.166667, VE = 0.833333, VE_percent = 83.3333, ARR = 0.125, NNV = 8 |
| Vaccine efficacy: n_v = 1000, cases_u = 100, n_u = 1000 | NEEDS_INPUT | bmed_vaccine_efficacy needs cases_v (cases among the vaccinated). Example: solve('bmed_vaccine_efficacy', cases_v=20, n_ |
| Vaccine efficacy: cases_v = 20, n_v = 1000, cases_u = 100, n_u = 1000, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Vaccine efficacy: cases_v = 700, n_v = 600, cases_u = 60, n_u = 400 | INVALID_INPUT | Vaccine efficacy from attack rates: the cases cannot exceed the group size. |

<a id="bmed_dose_weight_based"></a>

### Weight-based dose (dosage arithmetic, not medical advice), `bmed_dose_weight_based`

The dose for a body weight from an order in mg/kg, per dose or per day, with the daily total or the single dose when the number of doses per day is given; arithmetic only, no dose is judged safe.

$$
D = d_{kg}\,W,\qquad D_{dose} = \frac{d_{kg,day}\,W}{n}
$$

| input | meaning | condition |
|---|---|---|
| `dose_mg_per_kg` | ordered dose per kilogram of body weight, a number in mg/kg |  |
| `W` | body weight |  |
| `basis` | whether the mg/kg figure is per dose or per day |  |
| `doses_per_day` | number of doses per day (needed for a per-day order) |  |

| output | meaning |
|---|---|
| `dose` | single dose |
| `dose_mg` | single dose in mg |
| `daily_dose` | total dose per day (when the doses per day are known) |
| `daily_dose_mg` | total dose per day in mg |

- **Method:** dose = (mg/kg) x weight in kg; per day: single dose = daily dose / doses per day
- **Assumes:** arithmetic only: not medical advice; 0 < dose per kilogram <= 1000 mg/kg; 0.3 kg <= body weight <= 400 kg; a per-day order needs the number of doses per day (1 to 24)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Total dose per day (when the doses per day](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-weight-based-live-1)

*Total dose per day (when the doses per day... as doses_per_day runs from 1 to 8, every other input as you entered it (8 solutions). Marked: your doses_per_day = 4 gives daily_dose = 1.2 g. Your inputs: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4.*

![Single dose for every choice of basis, every other input as you entered it (highlighted: your choice)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-weight-based-live-2)

*Single dose for every choice of basis, every other input as you entered it (highlighted: your choice). per_dose: 0.0003; per_day: 7.5e-05. Your inputs: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4.*

![Single dose and total dose per day as dose_mg_per_kg runs from 3](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-weight-based-live-3)

*Single dose and total dose per day as dose_mg_per_kg runs from 3.75 to 26.25, every other input as you entered it (29 solutions). Marked: your dose_mg_per_kg = 15 gives dose = 0.3 g. Your inputs: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4.*

```python
biomedical_science_ai999x.solve('bmed_dose_weight_based', dose_mg_per_kg=15, W=20.0, basis='per_dose', doses_per_day=4)
# -> dose_mg = 300, daily_dose_mg = 1200, dose = 0.0003 kg, daily_dose = 0.0012 kg   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dose_weight_based', dose_mg_per_kg=40, W=25.0, basis='per_day', doses_per_day=2)
# -> dose_mg = 500, daily_dose_mg = 1000, dose = 0.0005 kg, daily_dose = 0.001 kg   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dose_weight_based', dose_mg_per_kg=10, W=3.5, basis='per_dose')
# -> dose_mg = 35, dose = 3.5e-05 kg   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Weight-based dose: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4 | SOLVED | dose_mg = 300, daily_dose_mg = 1200, dose = 0.0003 kg, daily_dose = 0.0012 kg |
| Weight-based dose: dose_mg_per_kg = 40, W = 25 kg, basis = per_day, doses_per_day = 2 | SOLVED | dose_mg = 500, daily_dose_mg = 1000, dose = 0.0005 kg, daily_dose = 0.001 kg |
| Weight-based dose: dose_mg_per_kg = 10, W = 3500 g, basis = per_dose | SOLVED | dose_mg = 35, dose = 3.5e-05 kg |
| Weight-based dose: W = 20 kg, basis = per_dose, doses_per_day = 4 | NEEDS_INPUT | bmed_dose_weight_based needs dose_mg_per_kg (ordered dose per kilogram of body weight, a number in mg/kg). Example: solv |
| Weight-based dose: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Dose by body weight: dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4 | SOLVED | dose_mg = 300, daily_dose_mg = 1200, dose = 0.0003 kg, daily_dose = 0.0012 kg |
| Weight-based dose: dose_mg_per_kg = 40, W = 25 kg, basis = per_day | INVALID_INPUT | Weight-based dose (dosage arithmetic, not medical advice): a dose per kilogram per day needs the number of doses per day |
| Weight-based dose: dose_mg_per_kg = 15, W = 900 kg, basis = per_dose | INVALID_INPUT | Weight-based dose (dosage arithmetic, not medical advice): the body weight must lie between 0.3 and 400 kg. |

<a id="bmed_dose_tablets"></a>

### Tablets to give: desired over have (dosage arithmetic, not medical advice), `bmed_dose_tablets`

The number of tablets (or capsules) for an ordered dose from the strength of one tablet, tablets = D/H; arithmetic only (whether a tablet may be split is not judged).

$$
n = \frac{D}{H}
$$

| input | meaning | condition |
|---|---|---|
| `D` | ordered (desired) dose |  |
| `H` | strength of one tablet or capsule (have) |  |

| output | meaning |
|---|---|
| `tablets` | number of tablets to give, D/H |

- **Method:** tablets = D/H (desired over have)
- **Assumes:** arithmetic only: not medical advice; $D > 0$, $H > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Number of tablets to give, D/H as D runs from 300 to 700 mg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-tablets-live-1)

*Number of tablets to give, D/H as D runs from 300 to 700 mg, every other input as you entered it (41 solutions). Marked: your D = 500 mg gives tablets = 2. Your inputs: D = 500 mg, H = 250 mg.*

![Number of tablets to give, D/H as H runs from 150 to 350 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-tablets-live-2)

*Number of tablets to give, D/H as H runs from 150 to 350 mg, every other input as you entered it (29 solutions). Marked: your H = 250 mg gives tablets = 2. Your inputs: D = 500 mg, H = 250 mg.*

```python
biomedical_science_ai999x.solve('bmed_dose_tablets', D=0.0005, H=0.00025)
# -> tablets = 2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dose_tablets', D=1.25e-07, H=2.5e-07)
# -> tablets = 0.5   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dose_tablets', D=0.00075, H=0.0005)
# -> tablets = 1.5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Tablets to give: D = 500 mg, H = 250 mg | SOLVED | tablets = 2 |
| Tablets to give: D = 0.125 mg, H = 0.25 mg | SOLVED | tablets = 0.5 |
| Tablets to give: D = 750 mg, H = 500 mg | SOLVED | tablets = 1.5 |
| Tablets to give: H = 250 mg | NEEDS_INPUT | bmed_dose_tablets needs D (ordered (desired) dose). Example: solve('bmed_dose_tablets', D='0.0005 kg', H='0.00025 kg'). |
| Tablets to give: D = 500 mg, H = 250 mg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Number of tablets: D = 0.5 g, H = 250 mg | SOLVED | tablets = 2 |
| Tablets to give: D = 500 mg, H = 0 mg | INVALID_INPUT | Tablets to give: desired over have (dosage arithmetic, not medical advice): the ordered dose D and the tablet strength H |

<a id="bmed_dose_liquid_volume"></a>

### Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice), `bmed_dose_liquid_volume`

The volume of a liquid or injection stock to give for an ordered dose, $V = D/H$ x Q, where the stock holds H in a volume Q; also the stock concentration H/Q.

$$
V = \frac{D}{H}\,Q
$$

| input | meaning | condition |
|---|---|---|
| `D` | ordered (desired) dose |  |
| `H` | amount of drug in the stock volume (have) |  |
| `Q` | stock volume that holds H |  |

| output | meaning |
|---|---|
| `V` | volume to give |
| `V_mL` | volume to give in mL |
| `C` | stock concentration H/Q (1 $kg/m^{3} = 1$ mg/mL) |

- **Method:** $V = (D/H)$ x Q (desired over have times quantity)
- **Assumes:** arithmetic only: not medical advice; $D > 0$, $H > 0$, $Q > 0$
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Volume to give as H runs from 75 to 175 mg, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-liquid-volume-live-1)

*Volume to give as H runs from 75 to 175 mg, every other input as you entered it (41 solutions). Marked: your H = 125 mg gives V = 10 mL. Your inputs: D = 250 mg, H = 125 mg, Q = 5 mL.*

![Volume to give as Q runs from 3 to 7 mL, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-dose-liquid-volume-live-2)

*Volume to give as Q runs from 3 to 7 mL, every other input as you entered it (29 solutions). Marked: your Q = 5 mL gives V = 10 mL. Your inputs: D = 250 mg, H = 125 mg, Q = 5 mL.*

```python
biomedical_science_ai999x.solve('bmed_dose_liquid_volume', D=0.00025, H=0.000125, Q=4.9999999999999996e-06)
# -> V = 1e-05 m^3, V_mL = 10, C = 25 kg/m^3   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_dose_liquid_volume', D=7.5e-05, H=9.999999999999999e-05, Q=2e-06)
# -> V = 1.5e-06 m^3, V_mL = 1.5, C = 50 kg/m^3   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Liquid dose volume: D = 250 mg, H = 125 mg, Q = 5 mL | SOLVED | V = 1e-05 m^3, V_mL = 10, C = 25 kg/m^3 |
| Liquid dose volume: D = 75 mg, H = 100 mg, Q = 2 mL | SOLVED | V = 1.5e-06 m^3, V_mL = 1.5, C = 50 kg/m^3 |
| Liquid dose volume: H = 125 mg, Q = 5 mL | NEEDS_INPUT | bmed_dose_liquid_volume needs D (ordered (desired) dose). Example: solve('bmed_dose_liquid_volume', D='0.00025 kg', H='0 |
| Liquid dose volume: D = 250 mg, H = 125 mg, Q = 5 mL, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Desired over have: D = 250 mg, H = 125 mg, Q = 5 mL | SOLVED | V = 1e-05 m^3, V_mL = 10, C = 25 kg/m^3 |
| Liquid dose volume: D = 250 mg, H = 125 mg, Q = -5 mL | INVALID_INPUT | Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice): the ordered dose D, the stock amo |

<a id="bmed_reconstitution_concentration"></a>

### Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice), `bmed_reconstitution_concentration`

The concentration of a powder drug after reconstitution, C = drug/(diluent + displacement volume), and the volume to give for an ordered dose, D/C.

$$
C = \frac{m}{V_{dil} + V_{disp}},\qquad V = \frac{D}{C}
$$

| input | meaning | condition |
|---|---|---|
| `powder` | drug amount in the vial |  |
| `diluent` | diluent volume added |  |
| `displacement` | displacement volume of the powder (optional, default 0) |  |
| `D` | ordered dose (optional) |  |

| output | meaning |
|---|---|
| `V_total` | final volume after reconstitution |
| `V_total_mL` | final volume in mL |
| `C` | concentration after reconstitution (1 $kg/m^{3} = 1$ mg/mL) |
| `C_mg_mL` | concentration in mg/mL |
| `V_give` | volume to give for the ordered dose (when given) |
| `V_give_mL` | volume to give in mL |

- **Method:** final volume = diluent + displacement; C = drug / final volume; volume to give = D / C
- **Assumes:** arithmetic only: not medical advice; drug > 0, diluent > 0, displacement >= 0; ordered dose <= drug in the vial
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Concentration after reconstitution as powder runs from 0](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-reconstitution-concentration-live-1)

*Concentration after reconstitution as powder runs from 0.6 to 1.4 g, every other input as you entered it (41 solutions). Marked: your powder = 1 g gives C = 250 mg/mL. Your inputs: powder = 1 g, diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg.*

![Volume to give in mL as D runs from 300 to 700 mg, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-reconstitution-concentration-live-2)

*Volume to give in mL as D runs from 300 to 700 mg, every other input as you entered it (29 solutions). Marked: your D = 500 mg gives V_give_mL = 2. Your inputs: powder = 1 g, diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg.*

```python
biomedical_science_ai999x.solve('bmed_reconstitution_concentration', powder=0.001, diluent=3.3999999999999996e-06, displacement=6e-07, D=0.0005)
# -> V_total = 4e-06 m^3, V_total_mL = 4, C = 250 kg/m^3, C_mg_mL = 250, V_give = 2e-06 m^3, V_give_mL = 2   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_reconstitution_concentration', powder=0.0005, diluent=9.999999999999999e-06)
# -> V_total = 1e-05 m^3, V_total_mL = 10, C = 50 kg/m^3, C_mg_mL = 50   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| Reconstitution: powder = 1 g, diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg | SOLVED | V_total = 4e-06 m^3, V_total_mL = 4, C = 250 kg/m^3, C_mg_mL = 250, V_give = 2e-06 m^3, V_give_mL = 2 |
| Reconstitution: powder = 500 mg, diluent = 10 mL | SOLVED | V_total = 1e-05 m^3, V_total_mL = 10, C = 50 kg/m^3, C_mg_mL = 50 |
| Reconstitution: diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg | NEEDS_INPUT | bmed_reconstitution_concentration needs powder (drug amount in the vial). Example: solve('bmed_reconstitution_concentrat |
| Reconstitution: powder = 1 g, diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Reconstituted concentration: powder = 500 mg, diluent = 10 mL | SOLVED | V_total = 1e-05 m^3, V_total_mL = 10, C = 50 kg/m^3, C_mg_mL = 50 |
| Reconstitution: powder = 500 mg, diluent = 10 mL, D = 750 mg | INVALID_INPUT | Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice): the ordered dose cannot  |

<a id="bmed_iv_flow_rate"></a>

### IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice), `bmed_iv_flow_rate`

The flow rate of an infusion in mL/h from the volume and the time (or the infusion time from the volume and the rate), and the drip rate in drops per minute from the set's drop factor.

$$
\dot V = \frac{V}{T},\qquad \text{gtt/min} = \frac{V\,[\mathrm{mL}]}{T\,[\mathrm{min}]}\times\text{drop factor}
$$

| input | meaning | condition |
|---|---|---|
| `V` | volume to infuse |  |
| `T` | infusion time (give T or rate) |  |
| `rate` | flow rate (give rate or T) |  |
| `drop_factor` | drop factor of the giving set, drops per mL (optional) |  |

| output | meaning |
|---|---|
| `flow` | flow rate |
| `flow_mL_h` | flow rate in mL/h |
| `duration` | infusion time |
| `duration_h` | infusion time in h |
| `duration_min` | infusion time in min |
| `drops_per_min` | drip rate in drops per minute (when the drop factor is given) |

- **Method:** mL/h = volume / hours; time = volume / rate; drops/min = (mL/h / 60) x drop factor
- **Assumes:** arithmetic only: not medical advice; $V > 0$; exactly one of $T > 0$ or rate > 0; drop factor a whole number of drops per mL, 1 to 100
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Flow rate in mL/h as T runs from 4](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-iv-flow-rate-live-1)

*Flow rate in mL/h as T runs from 4.8 to 11.2 h, every other input as you entered it (41 solutions). Marked: your T = 8 h gives flow_mL_h = 125. Your inputs: V = 1000 mL, T = 8 h, drop_factor = 15.*

![Flow rate in mL/h as V runs from 600 to 1400 mL, every other input as you entered it (29 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-iv-flow-rate-live-2)

*Flow rate in mL/h as V runs from 600 to 1400 mL, every other input as you entered it (29 solutions). Marked: your V = 1000 mL gives flow_mL_h = 125. Your inputs: V = 1000 mL, T = 8 h, drop_factor = 15.*

```python
biomedical_science_ai999x.solve('bmed_iv_flow_rate', V=0.001, T=28800.0, drop_factor=15)
# -> flow = 3.47222e-08 m^3/s, flow_mL_h = 125, duration = 28800 s, duration_h = 8, duration_min = 480, drops_per_min = 31.25   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_iv_flow_rate', V=0.0005, rate=2.7777777777777777e-08)
# -> flow = 2.77778e-08 m^3/s, flow_mL_h = 100, duration = 18000 s, duration_h = 5, duration_min = 300   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_iv_flow_rate', V=0.00025, T=1800.0, drop_factor=20)
# -> flow = 1.38889e-07 m^3/s, flow_mL_h = 500, duration = 1800 s, duration_h = 0.5, duration_min = 30, drops_per_min = 166.667   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| IV flow rate: V = 1000 mL, T = 8 h, drop_factor = 15 | SOLVED | flow = 3.47222e-08 m^3/s, flow_mL_h = 125, duration = 28800 s, duration_h = 8, duration_min = 480, drops_per_min = 31.25 |
| IV flow rate: V = 500 mL, rate = 100 mL/h | SOLVED | flow = 2.77778e-08 m^3/s, flow_mL_h = 100, duration = 18000 s, duration_h = 5, duration_min = 300 |
| IV flow rate: V = 250 mL, T = 30 min, drop_factor = 20 | SOLVED | flow = 1.38889e-07 m^3/s, flow_mL_h = 500, duration = 1800 s, duration_h = 0.5, duration_min = 30, drops_per_min = 166.667 |
| IV flow rate: T = 8 h, drop_factor = 15 | NEEDS_INPUT | bmed_iv_flow_rate needs V (volume to infuse). Example: solve('bmed_iv_flow_rate', V='0.001 m^3', T='28800 s', drop_facto |
| IV flow rate: V = 1000 mL, T = 8 h, drop_factor = 15, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| IV drip rate: V = 1 L, T = 480 min, drop_factor = 15 | SOLVED | flow = 3.47222e-08 m^3/s, flow_mL_h = 125, duration = 28800 s, duration_h = 8, duration_min = 480, drops_per_min = 31.25 |
| IV flow rate: V = 1000 mL, T = 8 h, rate = 125 mL/h | INVALID_INPUT | IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice): give either the infusion time T |

<a id="bmed_iv_rate_from_dose_rate"></a>

### Infusion pump rate from a dose rate (arithmetic, not medical advice), `bmed_iv_rate_from_dose_rate`

The pump rate in mL/h that delivers an ordered dose rate (mg/h, or ug/kg/min with the body weight) from a bag holding a stated amount of drug in a stated volume; also the drip rate for a drop factor.

$$
\dot V = \frac{\dot D}{C},\qquad C = \frac{m}{V_{bag}},\qquad \dot D = d\,W
$$

| input | meaning | condition |
|---|---|---|
| `drug` | amount of drug in the bag |  |
| `bag_volume` | volume of the bag |  |
| `dose_rate` | ordered dose rate, e.g. mg/h (give this or dose_ug_per_kg_min) |  |
| `dose_ug_per_kg_min` | ordered dose per kilogram per minute, a number in ug/kg/min (optional) |  |
| `W` | body weight (with dose_ug_per_kg_min) |  |
| `drop_factor` | drop factor of the giving set, drops per mL (optional) |  |

| output | meaning |
|---|---|
| `C` | concentration in the bag (1 $kg/m^{3} = 1$ mg/mL) |
| `dose_rate` | dose rate delivered |
| `dose_mg_h` | dose rate in mg/h |
| `flow` | pump rate |
| `flow_mL_h` | pump rate in mL/h |
| `drops_per_min` | drip rate in drops per minute (when the drop factor is given) |

- **Method:** C = drug/bag volume; dose rate = (ug/kg/min) x W x $60 / 1000$ mg/h; $mL/h = (mg/h) / (mg/mL)$
- **Assumes:** arithmetic only: not medical advice; drug > 0, bag volume > 0; exactly one of a dose rate or a dose per kilogram per minute (with 0.3 $kg  \leq  W  \leq  400$ kg)
- **Evidence:** certified procedure (gates: conditions, control, examples, fast_enough, questions, reference, sensitivity, templates)

![Pump rate in mL/h as bag_volume runs from 150 to 350 mL, every other input as you entered it (41 solutions)](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-iv-rate-from-dose-rate-live-1)

*Pump rate in mL/h as bag_volume runs from 150 to 350 mL, every other input as you entered it (41 solutions). Marked: your bag_volume = 250 mL gives flow_mL_h = 15. Your inputs: drug = 400 mg, bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg.*

![Pump rate in mL/h as dose_ug_per_kg_min runs from 1](https://www.ai999x.com/?ax_fig=biomedical-science-ai999x/bmed-iv-rate-from-dose-rate-live-2)

*Pump rate in mL/h as dose_ug_per_kg_min runs from 1.25 to 8.75, every other input as you entered it (29 solutions). Marked: your dose_ug_per_kg_min = 5 gives flow_mL_h = 15. Your inputs: drug = 400 mg, bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg.*

```python
biomedical_science_ai999x.solve('bmed_iv_rate_from_dose_rate', drug=0.00039999999999999996, bag_volume=0.00025, dose_ug_per_kg_min=5, W=80.0)
# -> C = 1.6 kg/m^3, dose_rate = 6.66667e-09 kg/s, dose_mg_h = 24, flow = 4.16667e-09 m^3/s, flow_mL_h = 15   (SOLVED)
```

```python
biomedical_science_ai999x.solve('bmed_iv_rate_from_dose_rate', drug=9.999999999999999e-05, bag_volume=9.999999999999999e-05, dose_rate=1.3888888888888888e-09, drop_factor=60)
# -> C = 1 kg/m^3, dose_rate = 1.38889e-09 kg/s, dose_mg_h = 5, flow = 1.38889e-09 m^3/s, flow_mL_h = 5, drops_per_min = 5   (SOLVED)
```

**Questions in words (documented forms), as executed:**

| question | outcome | result |
|---|---|---|
| IV rate from dose rate: drug = 400 mg, bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg | SOLVED | C = 1.6 kg/m^3, dose_rate = 6.66667e-09 kg/s, dose_mg_h = 24, flow = 4.16667e-09 m^3/s, flow_mL_h = 15 |
| IV rate from dose rate: drug = 100 mg, bag_volume = 100 mL, dose_rate = 5 mg/h, drop_factor = 60 | SOLVED | C = 1 kg/m^3, dose_rate = 1.38889e-09 kg/s, dose_mg_h = 5, flow = 1.38889e-09 m^3/s, flow_mL_h = 5, drops_per_min = 5 |
| IV rate from dose rate: bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg | NEEDS_INPUT | bmed_iv_rate_from_dose_rate needs drug (amount of drug in the bag). Example: solve('bmed_iv_rate_from_dose_rate', drug=' |
| IV rate from dose rate: drug = 400 mg, bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg, zz = 1 | UNRECOGNIZED_FORMAT | I could not match this question to a documented form, so I will not guess. Every form: biomedical_science_ai999x.templat |
| Pump rate from dose rate: drug = 100 mg, bag_volume = 100 mL, dose_rate = 5 mg/h, drop_factor = 60 | SOLVED | C = 1 kg/m^3, dose_rate = 1.38889e-09 kg/s, dose_mg_h = 5, flow = 1.38889e-09 m^3/s, flow_mL_h = 5, drops_per_min = 5 |
| IV rate from dose rate: drug = 400 mg, bag_volume = 250 mL, dose_rate = 5 mg/h, W = 80 kg | INVALID_INPUT | Infusion pump rate from a dose rate (arithmetic, not medical advice): the body weight is used only with a dose per kilog |
