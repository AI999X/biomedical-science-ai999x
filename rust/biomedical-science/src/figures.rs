//! Biomedical Science's own live figures (ax_engine::figkit), drawn for the visitor's inputs from what the model
//! computes: value maps over two inputs, curve families, pharmacokinetic curves (figures_pk.rs), survival, ROC,
//! interval and risk figures (figures_stat.rs), fits, a free-body diagram, a compartment schematic, a flow profile,
//! a Bode diagram and an attenuation profile (docs/FIGURE_PARITY.md).

use crate::figs_common::{self as c, esc, tick, Chart, FieldSpec, CX0, CX1};
use crate::figs_med::{self as m, fi, vi};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;
use ax_engine::Input;

/// The field maps: capability, output shown, input across, input up, logarithmic (across, up), title.
const FIELDS: &[(&str, &str, &str, &str, (bool, bool), &str)] = &[
    ("bmed_bmi", "BMI", "h", "m", (false, false), "Body mass index over height and body mass (a calculation, not medical advice)"),
    ("bmed_gaussian_psf_mtf", "MTF", "fwhm", "f", (true, true),
     "Modulation transfer of a Gaussian point-spread function over its width and the spatial frequency"),
    ("bmed_mri_ernst_angle", "alpha_deg", "TR", "T1", (false, false), "MRI Ernst angle over repetition time and T1"),
    ("bmed_snr_averaging", "SNR", "SNR1", "N", (false, false), "Signal-to-noise ratio after averaging over single SNR and averages"),
    ("bmed_ultrasound_interface", "R", "Z1", "Z2", (true, true),
     "Ultrasound intensity reflection over the two acoustic impedances"),
];

/// The figures for one solved request (the dispatcher ax_engine calls: Pack::figures).
pub fn live(s: &Solved) -> Vec<Figure> {
    let mut v: Vec<Option<Figure>> = vec![];
    if let Some(f) = FIELDS.iter().find(|f| f.0 == s.key) {
        v.push(c::field_map(s, &FieldSpec { z: f.1, x: f.2, y: f.3, log: f.4, title: f.5, zunit: "" }));
    }
    v.push(crate::figures_pk::live(s));
    v.extend(crate::figures_stat::live(s));
    match s.key {
        "bmed_pk_auc_trapezoid" => v.push(crate::figures_pk::trapezoids(s)),
        "bmed_bmr" => v.push(bmr(s)),
        "bmed_cable_steady_state" => v.push(cable(s)),
        "bmed_fitzhugh_nagumo_fixed_point" => {
            // the stability sweep first (101 solves), then a 21 x 13 verdict grid: both within the 400 solves
            let st = fhn_stability(s);
            v.push(fhn_decision(s));
            v.push(st);
        }
        "bmed_epi_reed_frost_final_size" => v.push(reed_frost(s)),
        "bmed_indicator_dilution_cardiac_output" => v.push(dilution(s)),
        "bmed_mri_adc_fit" => v.push(decay_fit(s, "b", "S", "ADC", "S0", false)),
        "bmed_mri_t2_fit" => v.push(decay_fit(s, "TE", "S", "T2", "S0", true)),
        "bmed_pk_terminal_half_life_fit" => v.push(half_life_fit(s)),
        "bmed_respiratory_mechanics_fit" => v.push(resp_fit(s)),
        "bmed_elbow_lever_muscle_force" => v.push(free_body(s)),
        "bmed_pk_two_compartment_micro_constants" => v.push(compartments(s)),
        "bmed_poiseuille_vessel_flow" => v.push(poiseuille(s)),
        "bmed_windkessel_input_impedance" => v.push(windkessel(s)),
        "bmed_xray_attenuation_layers" => v.push(layers(s)),
        "bmed_ultrasound_interface" => v.push(ultrasound_bars(s)),
        _ => {}
    }
    v.into_iter().flatten().collect()
}

// ------------------------------------------------------------------------------------------------ families
#[allow(clippy::too_many_arguments)]
fn family_fig(s: &Solved, kind: &str, title: &str, y: &str, x: &str, xs: &[f64], logx: bool, param: &str,
              values: &[f64], unit: &str) -> Option<Figure> {
    let (x0, y0) = (c::getv(s, x)?, c::getv(s, y)?);
    let fam = m::family(s, x, xs, y, param, values);
    if fam.is_empty() {
        return None;
    }
    let all: Vec<f64> = fam.iter().flat_map(|f| f.1.iter().map(|p| p.1)).chain([y0]).collect();
    let mut ch = Chart::new((xs[0], xs[xs.len() - 1]), Chart::range(&all), (logx, false), (false, true),
                            &c::axis_label(s, x), &c::axis_label(s, y), &format!("{y} = {}", tick(y0)));
    for (i, (p, pts)) in fam.iter().enumerate() {
        ch.line(pts, c::LINES[i % 4], &k::eq(param, *p, unit), false);
    }
    ch.mark((x0, y0), &format!("{x} = {}", tick(x0)));
    let body = ch.body();
    Some(s.chart(kind, title, &format!("{y} ({}) against {x} ({}) from {} to {}, one curve for each value of {param} ({}): {}. \
                                        The other inputs as given; {} points per curve, each a run of the model. Marked: \
                                        {x} = {}, {y} = {}.", c::meaning(s, y), c::meaning(s, x), tick(xs[0]),
                                       tick(xs[xs.len() - 1]), c::meaning(s, param),
                                       fam.iter().map(|f| k::eq(param, f.0, unit)).collect::<Vec<_>>().join(", "), xs.len(),
                                       tick(x0), tick(y0)), &body))
}

fn bmr(s: &Solved) -> Option<Figure> {
    let (mm, h) = (s.inp("m")?, s.inp("h")?);
    family_fig(s, "curve-2", "Basal metabolic rate against body mass for two heights (a calculation, not medical advice)",
               "BMR_kcal_day", "m", &k::span(mm * 0.5, mm * 1.5, 31, false), false, "h", &[h, h / 2.0], "m")
}

fn cable(s: &Solved) -> Option<Figure> {
    let (l, v0) = (s.inp("L")?, s.inp("V0")?);
    if s.txt("end").map(|e| e == "infinite").unwrap_or(false) || !(l > 0.0) {
        return None;
    }
    family_fig(s, "curve-2", "Passive cable: steady-state voltage at x against the cable length", "V_x", "L",
               &k::span(l / 10.0, l * 10.0, 31, true), true, "V0", &[v0, v0 / 2.0, v0 * 2.0], "V")
}

// ------------------------------------------------------------------------------------------------ FitzHugh-Nagumo
fn fhn_decision(s: &Solved) -> Option<Figure> {
    let (a, b) = (s.inp("a").or_else(|| c::getv(s, "a"))?, s.inp("b").or_else(|| c::getv(s, "b"))?);
    c::verdict_map_n(s, "FitzHugh-Nagumo fixed point: where its stability changes", "stable", "a", c::around(a, false)?, "b",
                     c::around(b, false)?, (false, false), (21, 13))
}

fn fhn_stability(s: &Solved) -> Option<Figure> {
    let i0 = c::getv(s, "I")?;
    let (v0, w0) = (s.out("v_star")?, s.out("w_star")?);
    let st0 = s.out_text("stable")?;
    let is: Vec<f64> = k::span(i0 - 0.5, i0 + 2.0, 101, false);
    let mut st = vec![];
    let mut un = vec![];
    let mut n_un = 0;
    for iv in &is {
        let Some(a) = s.with_input(&[("I", fi(*iv))]) else { continue };
        let Some(v) = k::num_of(&a, "v_star") else { continue };
        let stable = a.values.iter().find(|x| x.0 == "stable").map(|x| x.1 == serde_json::Value::Bool(true)).unwrap_or(false);
        if stable {
            st.push((*iv, v));
            un.push((*iv, f64::NAN));
        } else {
            n_un += 1;
            st.push((*iv, f64::NAN));
            un.push((*iv, v));
        }
    }
    let all: Vec<f64> = st.iter().chain(un.iter()).map(|p| p.1).chain([v0]).collect();
    if all.iter().filter(|x| x.is_finite()).count() < 3 {
        return None;
    }
    let mut ch = Chart::new((is[0], is[is.len() - 1]), Chart::range(&all), (false, false), (false, true),
                            "applied current I", "fixed point v*", &format!("v* = {}, {}", tick(v0), if st0 == "yes" { "stable" } else { "unstable" }));
    ch.line(&st, "axp-c2", "stable", false);
    ch.line(&un, "axp-c", "unstable", false);
    ch.mark((i0, v0), &format!("I = {}", tick(i0)));
    let body = ch.body();
    Some(s.chart("stability", "Rest state stability against the applied current",
                 &format!("Fixed point v* of the FitzHugh-Nagumo model (a = {}, b = {}, eps = {}) against the applied current \
                           from {} to {}, {} runs, coloured by the stability the model returns ({} unstable). Marked: I = {}, \
                           v* = {}, w* = {}, trace {}, determinant {}, {}.", tick(c::getv(s, "a")?), tick(c::getv(s, "b")?),
                          tick(c::getv(s, "eps")?), tick(is[0]), tick(is[is.len() - 1]), is.len(), n_un, tick(i0), tick(v0),
                          tick(w0), tick(s.out("trace")?), tick(s.out("determinant")?),
                          if st0 == "yes" { "stable" } else { "unstable" }), &body))
}

// ------------------------------------------------------------------------------------------------ Reed-Frost
fn reed_frost(s: &Solved) -> Option<Figure> {
    let p = c::getv(s, "p")?;
    let n = c::getv(s, "n")? as usize;
    if n > 30 {
        return None;
    }
    let mut ps = vec![p / 2.0, p];
    if p * 2.0 < 1.0 {
        ps.push(p * 2.0);
    }
    let mut groups = vec![];
    let mut notes = vec![];
    for pv in &ps {
        let a = s.with_input(&[("p", fi(*pv))])?;
        let d = k::list_of(&a, "distribution")?;
        if d.len() != n + 1 {
            return None;
        }
        notes.push(format!("p = {}: mean {}, no spread {}", tick(*pv), tick(k::num_of(&a, "mean_final")?),
                           tick(k::num_of(&a, "P_no_spread")?)));
        groups.push((format!("p = {}", tick(*pv)), d.iter().map(|x| Some(*x)).collect::<Vec<_>>()));
    }
    let ymax = groups.iter().flat_map(|g| g.1.iter().flatten()).fold(0.0f64, |a, b| a.max(*b));
    let mut ch = Chart::new((-0.5, n as f64 + 0.5), (0.0, ymax), (false, false), (false, true), "susceptibles infected",
                            "probability", &format!("mean {} at p = {}", tick(s.out("mean_final")?), tick(p)));
    let ng = groups.len() as f64;
    let slot = (ch.xa.px(1.0) - ch.xa.px(0.0)) * 0.8;
    let bw = slot / ng;
    let base = ch.ya.px(0.0);
    for (gi, (name, vals)) in groups.iter().enumerate() {
        let class = [ "axp-b3", "axp-b", "axp-b2"][gi % 3];
        for (i, v) in vals.iter().enumerate() {
            let Some(v) = v else { continue };
            let x = ch.xa.px(i as f64) - slot / 2.0 + bw * gi as f64;
            let y = ch.ya.px(*v);
            ch.s.push_str(&format!("<rect class=\"{class}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{:.1}\"/>",
                                   bw * 0.92, (base - y).max(0.5)));
            ch.pix.push((x, y));
        }
        ch.legend.push(([ "axp-c3", "axp-c", "axp-c2"][gi % 3].to_string(), name.clone()));
    }
    let body = ch.body();
    Some(s.chart("distribution", "Household outbreak size distribution",
                 &format!("Reed-Frost chain-binomial final size: probability that 0, 1, ..., {n} of {n} susceptibles are \
                           infected, starting from {} infective(s), for three per-contact infection probabilities (half, the \
                           given and twice p, one run each): {}.", tick(c::getv(s, "m")?), notes.join("; ")), &body))
}

// ------------------------------------------------------------------------------------------------ fits
fn dilution(s: &Solved) -> Option<Figure> {
    let (t, cv) = (s.vec("t")?, s.vec("C")?);
    let (auc, co) = (s.out("AUC")?, s.out("CO")?);
    if t.len() != cv.len() || t.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(cv.iter().copied()).collect();
    let zero: Vec<(f64, f64)> = pts.iter().map(|p| (p.0, 0.0)).collect();
    let lmin = co * 60000.0;
    let mut ch = Chart::new(Chart::range(&t), (0.0, Chart::range(&cv).1), (false, false), (false, true), "time (s)",
                            "concentration (mg/L)", &format!("cardiac output {} L/min", tick(lmin)));
    ch.area(&zero, &pts, "axp-s", "");
    ch.line(&pts, "axp-c", "", false);
    ch.dots(&pts, "axp-d", 3.5);
    ch.text(CX1 - 6.0, 64.0, "end", "axp-vt", &format!("AUC = {} mg s/L", tick(auc)));
    let body = ch.body();
    Some(s.chart("fit", &format!("Indicator dilution: cardiac output {} L/min", tick(lmin)),
                 &format!("Indicator-dilution curve: the {} concentration samples given after injecting m = {} mg of \
                           indicator, with the area under them computed by the model, AUC = {} mg s/L (shaded). Returned: \
                           cardiac output m/AUC = {} L/min.", t.len(), tick(s.inp("m")? * 1e6), tick(auc), tick(lmin)), &body))
}

/// A signal decay fit S0 exp(-x/T) (or exp(-x D)): data and the decay drawn from the returned parameters.
fn decay_fit(s: &Solved, xs_: &str, ys_: &str, par: &str, s0n: &str, time_const: bool) -> Option<Figure> {
    let (x, y) = (s.vec(xs_)?, s.vec(ys_)?);
    let (p, s0) = (s.out(par)?, s.out(s0n)?);
    if x.len() != y.len() || x.len() < 2 || !(p > 0.0) {
        return None;
    }
    let pts: Vec<(f64, f64)> = x.iter().copied().zip(y.iter().copied()).collect();
    let xmax = Chart::range(&x).1 * 1.5;
    let f = |v: f64| if time_const { s0 * (-v / p).exp() } else { s0 * (-v * p).exp() };
    let curve: Vec<(f64, f64)> = k::span(0.0, xmax, 60, false).into_iter().map(|v| (v, f(v))).collect();
    let ys: Vec<f64> = y.iter().copied().chain(curve.iter().map(|q| q.1)).collect();
    let (title, xl, cap) = if time_const {
        ("T2 decay fitted to spin echoes", "echo time TE (ms)",
         format!("The {} spin-echo signals given and the decay S0 exp(-TE/T2) drawn from the returned T2 = {} ms and \
                  S0 = {} (derived). Marked: at TE = T2 the signal is S0/e.", x.len(), tick(p), tick(s0)))
    } else {
        ("Diffusion signal decay and its ADC", "b value (s/mm^2)",
         format!("The diffusion-weighted signals given at b = {} s/mm^2 (log axis) and the mono-exponential decay drawn from \
                  the returned ADC = {} mm^2/s and S0 = {} (derived), extended to b = {}.", m::list_text(&x, 8), tick(p), tick(s0),
                 tick(xmax)))
    };
    let mut ch = Chart::new((0.0, xmax), Chart::range_pos(&ys), (false, true), (false, true), xl, "signal (log)",
                            &format!("{par} = {}", tick(p)));
    ch.line(&curve, "axp-c", "fitted decay", true);
    ch.dots(&pts, "axp-d2", 5.0);
    if time_const {
        ch.mark((p, s0 / std::f64::consts::E), &format!("TE = T2 = {}", tick(p)));
    } else {
        ch.dots(&[(0.0, s0)], "axp-d", 4.0);
    }
    let body = ch.body();
    Some(s.chart("fit", title, &cap, &body))
}

fn half_life_fit(s: &Solved) -> Option<Figure> {
    let (t, cv) = (s.vec("t")?, s.vec("C")?);
    let (kk, th, c0) = (s.out("k")?, s.out("t_half")?, s.out("C0")?);
    if t.len() != cv.len() || t.len() < 2 {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(cv.iter().copied()).collect();
    let tm = Chart::range(&t).1.max(th);
    let line: Vec<(f64, f64)> = k::span(0.0, tm, 40, false).into_iter().map(|x| (x, c0 * (-kk * x).exp())).collect();
    let ys: Vec<f64> = cv.iter().copied().chain(line.iter().map(|q| q.1)).collect();
    let mut ch = Chart::new((0.0, tm), Chart::range_pos(&ys), (false, true), (false, true), "time (as given)",
                            "concentration (log)", &format!("t1/2 = {}", tick(th)));
    ch.line(&line, "axp-c", "fitted line", true);
    ch.dots(&pts, "axp-d2", 4.0);
    ch.dots(&[(0.0, c0)], "axp-d", 4.5);
    ch.mark((th, c0 / 2.0), &format!("t1/2 = {}", tick(th)));
    let body = ch.body();
    Some(s.chart("fit", "Log-linear fit of the elimination phase",
                 &format!("Log-linear fit of the {} concentration samples given (log axis). The dashed line is C0 exp(-k t) \
                           drawn from the returned k = {} per time unit and C0 = {} (derived). Marked: the half-life t1/2 = {} \
                           where the line reaches C0/2; dot: the back-extrapolated C0.", t.len(), tick(kk), tick(c0), tick(th)), &body))
}

fn resp_fit(s: &Solved) -> Option<Figure> {
    let (p, q, v) = (s.vec("P")?, s.vec("Q")?, s.vec("V")?);
    let (r, cc, p0) = (s.out("R")?, s.out("C")?, s.out("P0")?);
    let n = p.len();
    if n < 2 || q.len() != n || v.len() != n || cc == 0.0 || n > 40 {
        return None;
    }
    let fit: Vec<(f64, f64)> = (0..n).map(|i| ((i + 1) as f64, p0 + r * q[i] + v[i] / cc)).collect();
    let el: Vec<f64> = (0..n).map(|i| p0 + v[i] / cc).collect();
    let data: Vec<(f64, f64)> = (0..n).map(|i| ((i + 1) as f64, p[i])).collect();
    let all: Vec<f64> = p.iter().copied().chain(fit.iter().map(|f| f.1)).chain(el.iter().copied()).chain([p0, 0.0]).collect();
    let mut ch = Chart::new((0.5, n as f64 + 0.5), Chart::range(&all), (false, false), (false, true), "sample",
                            "pressure (cmH2O)", &format!("R = {}, C = {}", tick(r), tick(cc)));
    let w = ((CX1 - CX0) / n as f64 * 0.5).min(40.0);
    for i in 0..n {
        let x = ch.xa.px((i + 1) as f64);
        let (ya, yb) = (ch.ya.px(p0), ch.ya.px(el[i]));
        ch.s.push_str(&format!("<rect class=\"axp-b4\" x=\"{:.1}\" y=\"{:.1}\" width=\"{w:.1}\" height=\"{:.1}\"/>", x - w / 2.0,
                               ya.min(yb), (ya - yb).abs().max(0.5)));
        ch.pix.push((x, yb));
    }
    ch.hguide(p0, &format!("P0 = {}", tick(p0)));
    ch.line(&fit, "axp-c", "P0 + R Q + V/C", false);
    ch.dots(&data, "axp-d2", 4.0);
    let body = ch.body();
    Some(s.chart("fit", "Breath pressure split into resistive and elastic parts",
                 &format!("Single-compartment fit of the {n} samples of pressure, flow and volume given (dots: the measured \
                           pressure): the line is P0 + R Q + V/C from the returned resistance R = {} cmH2O s/L, compliance \
                           C = {} L/cmH2O and P0 = {} cmH2O (derived); grey bars: the elastic part V/C above P0, the gap to \
                           the line being the resistive part R Q.", tick(r), tick(cc), tick(p0)), &body))
}

// ------------------------------------------------------------------------------------------------ drawings
fn arrow(out: &mut String, a: (f64, f64), b: (f64, f64), class: &str, dot: &str) {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let d = (dx * dx + dy * dy).sqrt();
    if d < 3.0 {
        return;
    }
    let (ux, uy) = (dx / d, dy / d);
    let (hx, hy) = (b.0 - ux * 9.0, b.1 - uy * 9.0);
    out.push_str(&format!("<path class=\"{class}\" style=\"stroke-width:2.6\" d=\"M{:.1},{:.1} L{hx:.1},{hy:.1}\"/>\
                           <path class=\"{dot}\" d=\"M{:.1},{:.1} L{:.1},{:.1} L{:.1},{:.1} Z\"/>", a.0, a.1, b.0, b.1,
                          hx - uy * 6.0, hy + ux * 6.0, hx + uy * 6.0, hy - ux * 6.0));
}

fn text(out: &mut String, x: f64, y: f64, anchor: &str, class: &str, t: &str) {
    let w = t.chars().count() as f64 * if class == "axp-mt" { 9.0 } else { 7.6 };
    let left = match anchor { "middle" => x - w / 2.0, "end" => x - w, _ => x };
    let shift = if left < 4.0 { 4.0 - left } else if left + w > 516.0 { 516.0 - left - w } else { 0.0 };
    out.push_str(&format!("<text class=\"{class}\" x=\"{:.1}\" y=\"{y:.1}\" text-anchor=\"{anchor}\">{}</text>", x + shift, esc(t)));
}

fn free_body(s: &Solved) -> Option<Figure> {
    let (w, dl, wa, da, dm, th) = (s.inp("W")?, s.inp("d_load")?, s.inp("W_arm")?, s.inp("d_arm")?, s.inp("d_muscle")?,
                                   s.inp("theta")?);
    let (fm, rx, ry, rr) = (s.out("F_muscle")?, s.out("R_x")?, s.out("R_y")?, s.out("R")?);
    let lmax = dl.max(da).max(dm);
    if !(lmax > 0.0) {
        return None;
    }
    let (ex, ey) = (90.0, 150.0);
    let sc = 360.0 / lmax;
    let mut out = String::new();
    out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">muscle force {} N</text>", esc(&tick(fm))));
    out.push_str(&format!("<rect class=\"axp-ka\" x=\"{ex}\" y=\"{:.1}\" width=\"{:.1}\" height=\"12\"/>\
                           <rect class=\"axp-k\" x=\"{ex}\" y=\"{:.1}\" width=\"{:.1}\" height=\"12\" rx=\"5\"/>",
                          ey - 6.0, lmax * sc, ey - 6.0, lmax * sc));
    // forces: muscle and joint to one scale, the weights to another
    let fs = 100.0 / fm.abs().max(rr.abs()).max(1e-12);
    let ws = 60.0 / w.abs().max(wa.abs()).max(1e-12);
    let mx = ex + dm * sc;
    let (ux, uy) = (-th.cos(), -th.sin());
    arrow(&mut out, (mx, ey), (mx + ux * fm * fs, ey + uy * fm * fs), "axp-c", "axp-d");
    text(&mut out, mx + ux * fm * fs + 8.0, (ey + uy * fm * fs).max(40.0), "start", "axp-vt", &format!("F = {} N", tick(fm)));
    let rv = (rx * fs, -ry * fs);
    arrow(&mut out, (ex, ey), (ex + rv.0, ey + rv.1), "axp-c3", "axp-d3");
    text(&mut out, ex + rv.0 + 10.0, (ey + rv.1 + 4.0).clamp(40.0, 270.0), "start", "axp-vt", &format!("joint R = {} N", tick(rr)));
    for (d, wt, lab) in [(da, wa, "forearm"), (dl, w, "load")] {
        let x = ex + d * sc;
        arrow(&mut out, (x, ey + 6.0), (x, ey + 6.0 + wt * ws), "axp-c2", "axp-d2");
        text(&mut out, x + 6.0, ey + 20.0 + wt * ws * 0.6, "start", "axp-vt", &format!("{lab} {} N", tick(wt)));
        text(&mut out, x, ey - 14.0, "middle", "axp-lt", &k::q(d, "m"));
    }
    text(&mut out, mx + 6.0, ey + 24.0, "start", "axp-lt", &format!("muscle at {}", k::q(dm, "m")));
    out.push_str(&format!("<circle class=\"axp-kd\" cx=\"{ex}\" cy=\"{ey}\" r=\"6\"/>"));
    text(&mut out, ex - 12.0, ey + 4.0, "end", "axp-lt", "elbow");
    text(&mut out, 16.0, 290.0, "start", "axp-lt", &format!("angle {} deg; R_x = {} N, R_y = {} N", tick(th.to_degrees()), tick(rx), tick(ry)));
    Some(s.figure("free-body", "Forearm lever: the muscle force many times the load",
                  &format!("Free-body diagram of the forearm as a lever about the elbow (dot): load W = {} N at {}, forearm \
                            weight {} N at {}, muscle inserted at {} pulling at {} degrees to the forearm. Muscle and joint \
                            arrows are drawn to one scale, the two weights to a larger one. Returned: muscle force {} N, joint \
                            reaction R = {} N (R_x = {} N along the forearm, R_y = {} N).", tick(w), k::q(dl, "m"), tick(wa),
                           k::q(da, "m"), k::q(dm, "m"), tick(th.to_degrees()), tick(fm), tick(rr), tick(rx), tick(ry)),
                  &out, 300.0))
}

fn compartments(s: &Solved) -> Option<Figure> {
    let (v1, k10, k12, k21, cl, vss) = (s.out("V1")?, s.out("k10")?, s.out("k12")?, s.out("k21")?, s.out("CL")?, s.out("Vss")?);
    let h = 3600.0;
    let mut cv = k::Canvas::new(280.0);
    cv.block(70.0, 90.0, 150.0, 80.0, "central");
    cv.block(320.0, 90.0, 150.0, 80.0, "peripheral");
    cv.value(145.0, 158.0, "middle", &format!("V1 = {} L", tick(v1 * 1000.0)));
    cv.arrow((220.0, 112.0), (320.0, 112.0));
    cv.arrow((320.0, 150.0), (220.0, 150.0));
    cv.value(270.0, 104.0, "middle", &format!("k12 = {}", tick(k12 * h)));
    cv.value(270.0, 168.0, "middle", &format!("k21 = {}", tick(k21 * h)));
    cv.arrow((145.0, 170.0), (145.0, 228.0));
    cv.value(155.0, 210.0, "start", &format!("k10 = {} 1/h", tick(k10 * h)));
    cv.arrow((145.0, 40.0), (145.0, 90.0));
    cv.name(155.0, 62.0, "start", &format!("dose D = {} mg", tick(s.inp("D")? * 1e6)));
    cv.bold(16.0, 24.0, "start", "Two-compartment model");
    cv.value(16.0, 262.0, "start", &format!("CL = {} L/h, Vss = {} L", tick(cl * 1000.0 * h), tick(vss * 1000.0)));
    Some(s.figure("schematic", "Two-compartment model with its rate constants",
                  &format!("The two-compartment model recovered from the bi-exponential coefficients given (D = {} mg, A = {} \
                            mg/L, alpha = {} 1/h, B = {} mg/L, beta = {} 1/h). Marked: V1 = {} L, k10 = {} 1/h, k12 = {} 1/h, \
                            k21 = {} 1/h; also returned CL = {} L/h and Vss = {} L.", tick(s.inp("D")? * 1e6),
                           tick(s.inp("A")? * 1000.0), tick(s.inp("alpha")? * h), tick(s.inp("B")? * 1000.0),
                           tick(s.inp("beta")? * h), tick(v1 * 1000.0), tick(k10 * h), tick(k12 * h), tick(k21 * h),
                           tick(cl * 1000.0 * h), tick(vss * 1000.0)), cv.body(), 280.0))
}

fn poiseuille(s: &Solved) -> Option<Figure> {
    let r = s.inp("r")?;
    let (q, vm, rh) = (s.out("Q")?, s.out("v_mean")?, s.out("R_hyd")?);
    if !(r > 0.0) {
        return None;
    }
    let mut out = String::new();
    out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"20\">Q = {} mL/s at r = {}</text>", esc(&tick(q * 1e6)),
                          esc(&k::q(r, "m"))));
    // the vessel and its parabolic profile (centre velocity twice the mean, derived)
    let (x0, yc, rp) = (110.0, 70.0, 40.0);
    out.push_str(&format!("<path class=\"axp-k\" d=\"M{x0},{:.1} L430,{:.1} M{x0},{:.1} L430,{:.1}\"/>", yc - rp, yc - rp, yc + rp, yc + rp));
    let vmax = 2.0 * vm;
    let lmax = 220.0;
    let mut prof = String::new();
    for i in 0..=8 {
        let yr = -1.0 + 2.0 * i as f64 / 8.0;
        let v = vmax * (1.0 - yr * yr);
        let y = yc + yr * rp;
        let l = lmax * v / vmax;
        if l > 4.0 {
            arrow(&mut out, (x0 + 10.0, y), (x0 + 10.0 + l, y), "axp-c4", "axp-d4");
        }
        prof.push_str(&format!("{}{:.1},{y:.1} ", if i == 0 { "M" } else { "L" }, x0 + 10.0 + l));
    }
    out.push_str(&format!("<path class=\"axp-c\" d=\"{}\"/>", prof.trim_end()));
    text(&mut out, 440.0, yc - 6.0, "start", "axp-vt", &format!("{} m/s", tick(vmax)));
    text(&mut out, 440.0, yc + 12.0, "start", "axp-lt", "centre");
    text(&mut out, 16.0, yc + 4.0, "start", "axp-lt", &format!("mean {} m/s", tick(vm)));
    // flow against radius
    let rs = k::span(r * 0.5, r * 1.5, 36, false);
    let pts = c::curve(s, "r", &rs, "Q");
    if pts.len() < 2 {
        return None;
    }
    let ml: Vec<(f64, f64)> = pts.iter().map(|p| (p.0 * 1000.0, p.1 * 1e6)).collect();
    let mut ch = Chart::new((ml[0].0, ml[ml.len() - 1].0), Chart::range(&ml.iter().map(|p| p.1).collect::<Vec<_>>()),
                            (false, false), (false, true), "vessel radius r (mm)", "flow Q (mL/s)", "");
    ch.line(&ml, "axp-c", "", false);
    ch.mark((r * 1000.0, q * 1e6), &format!("r = {} mm", tick(r * 1000.0)));
    let body = ch.body();
    out.push_str(&format!("<g transform=\"translate(0,114)\">{body}</g>"));
    Some(s.figure("flow", "Blood flow profile and the fourth-power radius law",
                  &format!("Poiseuille flow in a vessel (pressure difference {} Pa over L = {}, viscosity {} Pa s): top, the \
                            parabolic velocity profile drawn from the returned mean velocity {} m/s (centre velocity twice the \
                            mean, derived); bottom, flow against radius, {} runs of the model: it grows with the fourth power \
                            of the radius. Marked: Q = {} mL/s at r = {}, hydraulic resistance {} Pa s/m^3.",
                           tick(s.inp("dP")?), k::q(s.inp("L")?, "m"), tick(s.inp("mu")?), tick(vm), pts.len(), tick(q * 1e6),
                           k::q(r, "m"), tick(rh)), &out, 454.0))
}

fn windkessel(s: &Solved) -> Option<Figure> {
    let (r, f) = (s.inp("R")?, s.inp("f")?);
    let (zm, ph) = (s.out("Z_mod")?, s.out("phase_deg")?);
    let zc = s.inp("Zc").unwrap_or(0.05 * r);
    let fs = k::span(f / 30.0, f * 30.0, 60, true);
    let mut mods = vec![];
    let mut phs = vec![];
    for (name, extra) in [("two-element", vec![("model", Input::Text("two_element".into()))]),
                          ("three-element", vec![("model", Input::Text("three_element".into())), ("Zc", fi(zc))])] {
        let mut mp = vec![];
        let mut pp = vec![];
        for fv in &fs {
            let mut set = extra.clone();
            set.push(("f", fi(*fv)));
            let Some(a) = s.with_input(&set) else { continue };
            if let (Some(z), Some(p)) = (k::num_of(&a, "Z_mod"), k::num_of(&a, "phase_deg")) {
                mp.push((*fv, z));
                pp.push((*fv, p));
            }
        }
        if mp.len() >= 2 {
            mods.push((name, mp));
            phs.push((name, pp));
        }
    }
    if mods.is_empty() {
        return None;
    }
    let zs: Vec<f64> = mods.iter().flat_map(|m| m.1.iter().map(|p| p.1)).chain([zm]).collect();
    let mut c1 = Chart::new((fs[0], fs[fs.len() - 1]), Chart::range_pos(&zs), (true, true), (false, true), "frequency f (Hz)",
                            "|Z| (Pa s/m^3)", &format!("|Z| = {} Pa s/m^3", tick(zm)));
    for (i, (n, p)) in mods.iter().enumerate() {
        c1.line(p, c::LINES[i], n, false);
    }
    c1.mark((f, zm), &format!("f = {} Hz", tick(f)));
    let ps: Vec<f64> = phs.iter().flat_map(|m| m.1.iter().map(|p| p.1)).chain([ph, 0.0]).collect();
    let mut c2 = Chart::new((fs[0], fs[fs.len() - 1]), Chart::range(&ps), (true, false), (false, true), "frequency f (Hz)",
                            "phase (degrees)", &format!("phase = {} degrees", tick(ph)));
    for (i, (n, p)) in phs.iter().enumerate() {
        c2.line(p, c::LINES[i], n, false);
    }
    c2.mark((f, ph), "");
    let (body, h) = k::stack(&[c1.body(), c2.body()]);
    Some(s.figure("bode", "Arterial input impedance: two- and three-element models",
                  &format!("Input impedance of the arterial Windkessel (R = {} Pa s/m^3, C = {} m^3/Pa) against frequency, \
                            modulus (top) and phase (bottom), {} runs per line of the model, for the two-element model and \
                            the three-element model with Zc = {} Pa s/m^3: the third element levels the modulus off at high \
                            frequency and brings the phase back towards 0. Marked: f = {} Hz, |Z| = {} Pa s/m^3, phase {} \
                            degrees ({}).", tick(r), tick(s.inp("C")?), fs.len(), tick(zc), tick(f), tick(zm), tick(ph),
                           s.txt("model").unwrap_or_default().replace('_', " ")), &body, h))
}

fn layers(s: &Solved) -> Option<Figure> {
    let (mu, rho, x) = (s.vec("mu_rho")?, s.vec("rho")?, s.vec("x")?);
    let (i0, tf, iend) = (s.inp("I0")?, s.out("T")?, s.out("I")?);
    let n = x.len();
    if n == 0 || mu.len() != n || rho.len() != n || n > 10 || x.iter().any(|v| *v < 0.0) {
        return None;
    }
    let mut pts = vec![(0.0, i0)];
    let mut start = 0.0;
    for j in 0..n {
        for q in 1..=6 {
            let mut xs: Vec<f64> = x[..j].to_vec();
            xs.push(x[j] * q as f64 / 6.0);
            let iv = m::run(s, &[("mu_rho", vi(&mu[..=j])), ("rho", vi(&rho[..=j])), ("x", vi(&xs))], "I")?;
            pts.push((start + x[j] * q as f64 / 6.0, iv));
        }
        start += x[j];
    }
    let total = start;
    let ys: Vec<f64> = pts.iter().map(|p| p.1).collect();
    let mut ch = Chart::new((0.0, total), Chart::range_pos(&ys), (false, true), (false, true), "depth (cm, as given)",
                            "intensity (W/m^2, log)", &format!("T = {}", tick(tf)));
    let mut a = 0.0;
    for j in 0..n {
        ch.xband(a, a + x[j], if j % 2 == 0 { "axp-s" } else { "axp-s3" });
        let mid = ch.xa.px(a + x[j] / 2.0);
        let lab = format!("\u{3c1} {}", tick(rho[j]));
        if (ch.xa.px(a + x[j]) - ch.xa.px(a)) > lab.chars().count() as f64 * 7.0 {
            ch.text(mid, 62.0, "middle", "axp-lt", &lab);
        }
        a += x[j];
    }
    ch.line(&pts, "axp-c", "", false);
    ch.mark((total, iend), &format!("I = {}", tick(iend)));
    let body = ch.body();
    Some(s.figure("layers", "X-ray intensity through tissue layers",
                  &format!("X-ray beam through {n} layers drawn to scale (thicknesses {} cm, mass attenuation {} cm^2/g, \
                            densities {} g/cm^3, as given): each point is the model's intensity after the depth crossed so far \
                            (log axis, steeper in a denser layer). Marked: I0 = {} W/m^2 gives I = {} W/m^2, transmitted \
                            fraction T = {}.", m::list_text(&x, 10), m::list_text(&mu, 10), m::list_text(&rho, 10), tick(i0),
                           tick(iend), tick(tf)), &body, 340.0))
}

fn ultrasound_bars(s: &Solved) -> Option<Figure> {
    let (z1, z2, r) = (s.inp("Z1")?, s.inp("Z2")?, s.out("R")?);
    let pairs = [("fat\nmuscle", 1.38, 1.70), ("blood\nmuscle", 1.66, 1.70), ("tissue\nbone", 1.63, 7.8),
                 ("tissue\nair", 1.63, 0.00043)];
    let mut labels = vec![];
    let mut vals = vec![];
    for (n, a, b) in pairs {
        let Some(v) = m::run(s, &[("Z1", fi(a * 1e6)), ("Z2", fi(b * 1e6))], "R") else { continue };
        if v > 0.0 {
            labels.push(n.to_string());
            vals.push(v);
        }
    }
    if r > 0.0 {
        labels.push("given\nZ1, Z2".into());
        vals.push(r);
    }
    if labels.len() < 2 {
        return None;
    }
    let mut classes = vec!["axp-b4"; labels.len()];
    *classes.last_mut()? = "axp-b";
    let body = crate::figs_extra::bars_full(&labels, &vals, &classes, None, "intensity reflection R (log)",
                                            &format!("R = {}", tick(r)), true, None)?;
    let labels: Vec<String> = labels.iter().map(|l| l.replace('\n', "/")).collect();
    Some(s.figure("bars", "Ultrasound reflection at tissue interfaces",
                  &format!("Intensity reflection at four tissue interfaces (log axis), one run of the model each with stated \
                            acoustic impedances (MRayl: fat 1.38, muscle 1.70, soft tissue 1.63, blood 1.66, bone 7.8, air \
                            0.00043): {}. Highlighted: the impedances given (Z1 = {}, Z2 = {} kg/(m^2 s)), R = {}, T = {}; air \
                            reflects almost everything, hence coupling gel.",
                           labels.iter().zip(&vals).map(|(l, v)| format!("{l} R = {}", tick(*v))).collect::<Vec<_>>().join("; "),
                           tick(z1), tick(z2), tick(r), tick(s.out("T")?)), &body, 340.0))
}
