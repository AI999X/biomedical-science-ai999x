//! Pharmacokinetics figures: concentration curves, each point a run of the model's own concentration capability
//! (this one or a sibling), times shown in hours and concentrations in mg/L.

use crate::figs_common::{self as c, tick, Chart};
use crate::figs_med::{fi, ii, other};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;

const H: f64 = 3600.0;

fn hr(t: f64) -> f64 {
    t / H
}
fn mg(cv: f64) -> f64 {
    cv * 1000.0
}
/// Concentration points (s, kg/m^3) as (h, mg/L).
fn conv(pts: &[(f64, f64)]) -> Vec<(f64, f64)> {
    pts.iter().map(|p| (hr(p.0), mg(p.1))).collect()
}
fn ymax(series: &[&[(f64, f64)]]) -> f64 {
    series.iter().flat_map(|s| s.iter().map(|p| p.1)).filter(|v| v.is_finite()).fold(0.0, f64::max)
}
/// "6.86 mg/L" from kg/m^3.
fn mgl(cv: f64) -> String {
    format!("{} mg/L", tick(mg(cv)))
}
fn hours(t: f64) -> String {
    format!("{} h", tick(hr(t)))
}

/// The concentration of an IV bolus (D, V, k) at t, from the model's bolus capability.
fn bolus(s: &Solved, d: f64, v: f64, kk: f64, t: f64) -> Option<f64> {
    other(s, "bmed_pk_iv_bolus_concentration", &[("D", fi(d)), ("V", fi(v)), ("k", fi(kk)), ("t", fi(t))], "C")
}

/// This capability's output y against its input t over 0..tend (n points).
fn self_curve(s: &Solved, y: &str, tend: f64, n: usize) -> Vec<(f64, f64)> {
    c::curve(s, "t", &k::span(0.0, tend, n, false), y)
}

pub fn live(s: &Solved) -> Option<Figure> {
    match s.key {
        "bmed_pk_auc_iv_bolus" => auc_iv_bolus(s),
        "bmed_pk_bioavailability_from_profiles" => bioavailability(s),
        "bmed_pk_effect_site_concentration" => effect_site(s),
        "bmed_pk_infusion_concentration" => infusion(s),
        "bmed_pk_iv_bolus_concentration" => iv_bolus(s),
        "bmed_pk_loading_maintenance" => loading(s),
        "bmed_pk_michaelis_menten_concentration" => mm_conc(s),
        "bmed_pk_michaelis_menten_time" => mm_time(s),
        "bmed_pk_nca_iv_bolus" => nca(s),
        "bmed_pk_nth_dose_iv" => nth_dose(s),
        "bmed_pk_oral_concentration" => oral(s),
        "bmed_pk_oral_peak" => oral_peak(s),
        "bmed_pk_short_infusion" => short_infusion(s),
        "bmed_pk_two_compartment_bolus" => two_comp(s),
        _ => None,
    }
}

fn pk(s: &Solved, title: &str, caption: &str, body: &str) -> Option<Figure> {
    Some(s.chart("pharmacokinetics", title, caption, body))
}

fn auc_iv_bolus(s: &Solved) -> Option<Figure> {
    let (d, cl, v, t) = (s.inp("D")?, s.inp("CL")?, s.inp("V")?, s.inp("t")?);
    let (inf, part) = (s.out("AUC_inf")?, s.out("AUC_0_t")?);
    if !(v > 0.0 && cl > 0.0 && t > 0.0) {
        return None;
    }
    let kk = cl / v;
    let tend = 3.0 * t;
    let pts: Vec<(f64, f64)> = k::span(0.0, tend, 91, false).into_iter()
        .filter_map(|x| bolus(s, d, v, kk, x).map(|y| (x, y))).collect();
    if pts.len() < 2 {
        return None;
    }
    let p = conv(&pts);
    let shade: Vec<(f64, f64)> = p.iter().copied().filter(|q| q.0 <= hr(t) + 1e-12).collect();
    let zero: Vec<(f64, f64)> = shade.iter().map(|q| (q.0, 0.0)).collect();
    let auc = |x: f64| tick(x * 1000.0 / H);
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&[&p])), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("AUC 0-t = {} of {} mg h/L", auc(part), auc(inf)));
    ch.area(&zero, &shade, "axp-s", "");
    ch.line(&p, "axp-c", "", false);
    ch.vguide(hr(t), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Area under the concentration curve after an IV bolus",
       &format!("Concentration after an intravenous bolus (D = {} mg, CL = {} L/h, V = {} L): each point is the model's \
                 bolus concentration with k = CL/V = {} 1/h, 91 runs over {} h. Shaded: the area from 0 to t = {}, \
                 AUC = {} mg h/L, of the area to infinity {} mg h/L.", tick(d * 1e6), tick(cl * 1000.0 * H),
                tick(v * 1000.0), tick(kk * H), tick(hr(tend)), hours(t), auc(part), auc(inf)), &body)
}

fn bioavailability(s: &Solved) -> Option<Figure> {
    let (ti_, ci) = (s.vec("t_iv")?, s.vec("C_iv")?);
    let (to, co) = (s.vec("t_oral")?, s.vec("C_oral")?);
    let (ai, ao, f) = (s.out("AUC_iv")?, s.out("AUC_oral")?, s.out("F")?);
    if ti_.len() != ci.len() || to.len() != co.len() {
        return None;
    }
    let pi: Vec<(f64, f64)> = ti_.iter().copied().zip(ci.iter().copied()).collect();
    let po: Vec<(f64, f64)> = to.iter().copied().zip(co.iter().copied()).collect();
    let xs: Vec<f64> = ti_.iter().chain(to.iter()).copied().collect();
    let ys: Vec<f64> = ci.iter().chain(co.iter()).copied().chain([0.0]).collect();
    let mut ch = Chart::new(Chart::range(&xs), Chart::range(&ys), (false, false), (false, true), "time (as given)",
                            "concentration (as given)", &format!("F = {}", tick(f)));
    ch.line(&pi, "axp-c", &format!("IV, AUC = {}", tick(ai)), false);
    ch.dots(&pi, "axp-d", 3.5);
    ch.line(&po, "axp-c2", &format!("oral, AUC = {}", tick(ao)), false);
    ch.dots(&po, "axp-d2", 3.5);
    let body = ch.body();
    pk(s, "IV and oral profiles and the absolute bioavailability",
       &format!("The intravenous and oral concentration profiles as given, with the areas the model computes: AUC_iv = {}, \
                 AUC_oral = {}. Dose-normalised, their ratio is the absolute bioavailability F = {}.", tick(ai), tick(ao),
                tick(f)), &body)
}

fn effect_site(s: &Solved) -> Option<Figure> {
    let (d, v, kk, t) = (s.inp("D")?, s.inp("V")?, s.inp("k")?, s.inp("t")?);
    let (ce, tp, cp) = (s.out("Ce")?, s.out("t_peak")?, s.out("Ce_peak")?);
    let tend = (4.0 * tp).max(1.5 * t);
    let xs = k::span(0.0, tend, 81, false);
    let e = conv(&c::curve(s, "t", &xs, "Ce"));
    let p = conv(&xs.iter().filter_map(|x| bolus(s, d, v, kk, *x).map(|y| (*x, y))).collect::<Vec<_>>());
    if e.len() < 2 {
        return None;
    }
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&[&e, &p])), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("peak {} at {}", mgl(cp), hours(tp)));
    ch.line(&p, "axp-c4", "plasma", false);
    ch.line(&e, "axp-c", "effect site", false);
    ch.dots(&[(hr(tp), mg(cp))], "axp-d", 4.0);
    ch.mark((hr(t), mg(ce)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Effect-site concentration lags the plasma after a bolus",
       &format!("Effect-site concentration after an intravenous bolus (D = {} mg, V = {} L, k = {} 1/h, ke0 = {} 1/h), \
                 81 runs over {} h, beside the plasma concentration of the same bolus (the model's one-compartment \
                 bolus). Dot: the peak t_peak = {}, Ce_peak = {}; marked: t = {}, Ce = {}.", tick(d * 1e6),
                tick(v * 1000.0), tick(kk * H), tick(s.inp("ke0")? * H), tick(hr(tend)), hours(tp), mgl(cp), hours(t),
                mgl(ce)), &body)
}

fn infusion(s: &Solved) -> Option<Figure> {
    let (cl, v, t) = (s.inp("CL")?, s.inp("V")?, s.inp("t")?);
    let (cv, css) = (s.out("C")?, s.out("C_ss")?);
    let tend = (3.0 * t).max(4.0 * v / cl.max(1e-300)).min(20.0 * t.max(1.0));
    let xs = k::span(0.0, tend, 81, false);
    let fam = crate::figs_med::family(s, "t", &xs, "C", "V", &[v, v / 2.0, 2.0 * v]);
    if fam.is_empty() {
        return None;
    }
    let ser: Vec<(f64, Vec<(f64, f64)>)> = fam.iter().map(|(p, pts)| (*p, conv(pts))).collect();
    let all: Vec<&[(f64, f64)]> = ser.iter().map(|x| x.1.as_slice()).collect();
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&all).max(mg(css))), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("C_ss = {}", mgl(css)));
    for (i, (p, pts)) in ser.iter().enumerate() {
        ch.line(pts, c::LINES[i], &format!("V = {} L", tick(p * 1000.0)), false);
    }
    ch.hguide(mg(css), "steady state");
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Constant infusion approaching its steady state",
       &format!("Constant-rate infusion (R0 = {} mg/h, CL = {} L/h): the concentration rises towards the steady state \
                 R0/CL = {}, sooner for a smaller volume of distribution (half, the given and twice V, 81 runs each). \
                 Marked: t = {}, C = {}.", tick(s.inp("R0")? * 1e6 * H), tick(cl * 1000.0 * H), mgl(css), hours(t),
                mgl(cv)), &body)
}

fn iv_bolus(s: &Solved) -> Option<Figure> {
    let (kk, t) = (s.inp("k")?, s.inp("t")?);
    let (cv, fr) = (s.out("C")?, s.out("fraction_remaining")?);
    let tend = if t > 0.0 { 8.0 * t } else { 5.0 / kk.max(1e-300) };
    let xs = k::span(0.0, tend, 97, false);
    let fam = crate::figs_med::family(s, "t", &xs, "C", "k", &[kk, kk / 2.0, kk * 2.0]);
    if fam.is_empty() {
        return None;
    }
    let ser: Vec<(f64, Vec<(f64, f64)>)> = fam.iter().map(|(p, pts)| (*p, conv(pts))).collect();
    let ys: Vec<f64> = ser.iter().flat_map(|x| x.1.iter().map(|p| p.1)).collect();
    let (lo, hi) = Chart::range_pos(&ys);
    let mut ch = Chart::new((0.0, hr(tend)), (lo.max(hi * 1e-6), hi), (false, true), (false, true), "time (h)",
                            "concentration (mg/L, log)", &format!("C = {}", mgl(cv)));
    for (i, (p, pts)) in ser.iter().enumerate() {
        ch.line(pts, c::LINES[i], &format!("k = {} 1/h", tick(p * H)), false);
    }
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "IV bolus: the concentration decays exponentially",
       &format!("One-compartment intravenous bolus: plasma concentration over {} h (log axis, so each line is straight) \
                 for D = {} mg, V = {} L and three elimination rate constants (the given k = {} 1/h, half and double), \
                 97 runs each. Marked: t = {}, C = {}, fraction of the dose remaining {}.", tick(hr(tend)),
                tick(s.inp("D")? * 1e6), tick(s.inp("V")? * 1000.0), tick(kk * H), hours(t), mgl(cv), tick(fr)), &body)
}

fn loading(s: &Solved) -> Option<Figure> {
    let (ct, v, cl) = (s.inp("C_target")?, s.inp("V")?, s.inp("CL")?);
    let (ld, r0) = (s.out("LD")?, s.out("R0")?);
    if !(v > 0.0 && cl > 0.0) {
        return None;
    }
    let kk = cl / v;
    let tend = 5.0 / kk;
    let xs = k::span(0.0, tend, 61, false);
    let mut b = vec![];
    let mut inf = vec![];
    let mut sum = vec![];
    for x in &xs {
        let cb = bolus(s, ld, v, kk, *x)?;
        let ci = other(s, "bmed_pk_infusion_concentration", &[("R0", fi(r0)), ("CL", fi(cl)), ("V", fi(v)), ("t", fi(*x))], "C")?;
        b.push((*x, cb));
        inf.push((*x, ci));
        sum.push((*x, cb + ci));
    }
    let (b, inf, sum) = (conv(&b), conv(&inf), conv(&sum));
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&[&b, &inf, &sum]).max(mg(ct))), (false, false), (false, true),
                            "time (h)", "concentration (mg/L)", &format!("target {}", mgl(ct)));
    ch.line(&b, "axp-c3", "loading bolus", true);
    ch.line(&inf, "axp-c2", "infusion", true);
    ch.line(&sum, "axp-c", "together", false);
    ch.hguide(mg(ct), "");
    let body = ch.body();
    pk(s, "Loading dose plus infusion holds the target from the start",
       &format!("Loading dose and maintenance infusion for a target of {} (V = {} L, CL = {} L/h): the model returns \
                 LD = {} mg and R0 = {} mg/h. Drawn: the model's bolus concentration of LD (k = CL/V), its infusion \
                 concentration at R0, 61 runs each, and their sum (superposition, derived).", mgl(ct), tick(v * 1000.0),
                tick(cl * 1000.0 * H), tick(ld * 1e6), tick(r0 * 1e6 * H)), &body)
}

fn mm_conc(s: &Solved) -> Option<Figure> {
    let (c0, t) = (s.inp("C0")?, s.inp("t")?);
    let cv = s.out("C")?;
    let tend = if t > 0.0 { 3.0 * t } else { return None };
    let xs = k::span(0.0, tend, 73, false);
    let fam = crate::figs_med::family(s, "t", &xs, "C", "C0", &[c0, c0 / 2.0, c0 * 2.0]);
    if fam.is_empty() {
        return None;
    }
    let ser: Vec<(f64, Vec<(f64, f64)>)> = fam.iter().map(|(p, pts)| (*p, conv(pts))).collect();
    let all: Vec<&[(f64, f64)]> = ser.iter().map(|x| x.1.as_slice()).collect();
    let km = s.inp("Km")?;
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&all)), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("C = {}", mgl(cv)));
    for (i, (p, pts)) in ser.iter().enumerate() {
        ch.line(pts, c::LINES[i], &format!("C0 = {}", mgl(*p)), false);
    }
    ch.hguide(mg(km), "Km");
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Saturable elimination: a nearly linear fall, then exponential",
       &format!("Saturable (Michaelis-Menten) elimination: concentration over {} h for Vmax = {} mg/h, Km = {}, V = {} L \
                 and three starting concentrations (half, the given and twice C0), 73 runs each. Far above Km the fall \
                 is nearly linear, below Km it becomes exponential. Marked: t = {}, C = {}.", tick(hr(tend)),
                tick(s.inp("Vmax")? * 1e6 * H), mgl(km), tick(s.inp("V")? * 1000.0), hours(t), mgl(cv)), &body)
}

fn mm_time(s: &Solved) -> Option<Figure> {
    let (c0, cg) = (s.inp("C0")?, s.inp("C")?);
    let tt = s.out("t")?;
    let lo = (cg / 3.0).min(c0 / 20.0);
    let cs = k::span(c0 * 0.999, lo, 60, true);
    let mut pts: Vec<(f64, f64)> = cs.iter().filter_map(|cv| {
        let a = s.with_input(&[("C", fi(*cv))])?;
        k::num_of(&a, "t").map(|x| (x, *cv))
    }).collect();
    pts.insert(0, (0.0, c0));
    if pts.len() < 3 {
        return None;
    }
    let p = conv(&pts);
    let mut ch = Chart::new(Chart::range(&p.iter().map(|q| q.0).collect::<Vec<_>>()), (0.0, mg(c0)), (false, false),
                            (false, true), "time (h)", "concentration (mg/L)", &format!("t = {}", hours(tt)));
    ch.line(&p, "axp-c", "", false);
    ch.hguide(mg(cg), "target");
    ch.mark((hr(tt), mg(cg)), &format!("C = {}", mgl(cg)));
    let body = ch.body();
    pk(s, "Saturable elimination: the time to reach a concentration",
       &format!("Time for the concentration to fall from C0 = {} to each lower value under saturable elimination \
                 (Vmax = {} mg/h, Km = {}, V = {} L), {} runs of the model, drawn as concentration against time. \
                 Marked: the target {} is reached after t = {}.", mgl(c0), tick(s.inp("Vmax")? * 1e6 * H),
                mgl(s.inp("Km")?), tick(s.inp("V")? * 1000.0), pts.len() - 1, mgl(cg), hours(tt)), &body)
}

fn nca(s: &Solved) -> Option<Figure> {
    let (t, cv) = (s.vec("t")?, s.vec("C")?);
    let nt = c::getv(s, "n_terminal")? as usize;
    let (lz, th, al, ai, cl, vz) = (s.out("lambda_z")?, s.out("t_half")?, s.out("AUC_last")?, s.out("AUC_inf")?,
                                    s.out("CL")?, s.out("Vz")?);
    if t.len() != cv.len() || nt < 2 || nt > t.len() {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(cv.iter().copied()).filter(|p| p.1 > 0.0).collect();
    let (tl, cl_) = (*t.last()?, *cv.last()?);
    let t0 = t[t.len() - nt];
    let line: Vec<(f64, f64)> = k::span(t0, tl, 20, false).into_iter().map(|x| (x, cl_ * (-lz * (x - tl)).exp())).collect();
    let ys: Vec<f64> = pts.iter().map(|p| p.1).chain(line.iter().map(|p| p.1)).collect();
    let mut ch = Chart::new(Chart::range(&t), Chart::range_pos(&ys), (false, true), (false, true), "time (as given)",
                            "concentration (log)", &format!("t1/2 = {}", tick(th)));
    let term: Vec<(f64, f64)> = pts.iter().copied().filter(|p| p.0 >= t0).collect();
    ch.dots(&pts, "axp-d4", 3.5);
    ch.dots(&term, "axp-d", 4.5);
    ch.line(&line, "axp-c2", &format!("terminal slope {}", tick(lz)), true);
    let body = ch.body();
    pk(s, "Non-compartmental analysis: the terminal phase and its half-life",
       &format!("Non-compartmental analysis of the {} samples given after an intravenous bolus (log axis). The dashed \
                 line has the model's terminal slope lambda_z = {} from the last {} samples, drawn through the last \
                 sample (derived). Returned: t1/2 = {}, AUC_last = {}, AUC_inf = {}, CL = {}, Vz = {}.", t.len(),
                tick(lz), nt, tick(th), tick(al), tick(ai), tick(cl), tick(vz)), &body)
}

fn nth_dose(s: &Solved) -> Option<Figure> {
    let (tau, t) = (s.inp("tau")?, s.inp("t")?);
    let n = c::getv(s, "n")? as i64;
    let cv = s.out("C")?;
    if !(tau > 0.0) || n < 1 {
        return None;
    }
    let nd = (n + 2).clamp(3, 8);
    let mut pts = vec![];
    for j in 1..=nd {
        for x in k::span(0.0, tau, 21, false) {
            let a = s.with_input(&[("n", ii(j)), ("t", fi(x))])?;
            pts.push(((j - 1) as f64 * tau + x, k::num_of(&a, "C")?));
        }
        pts.push((f64::NAN, f64::NAN));
    }
    let mut p = conv(&pts.iter().copied().filter(|q| q.0.is_finite()).collect::<Vec<_>>());
    // a break between doses keeps the jump vertical
    let mut joined = vec![];
    for (i, q) in p.drain(..).enumerate() {
        if i > 0 && i % 21 == 0 {
            if let Some(last) = joined.last().copied() {
                let (lx, _): (f64, f64) = last;
                joined.push((lx, q.1));
            }
        }
        joined.push(q);
    }
    let xm = hr((n - 1) as f64 * tau + t);
    let mut ch = Chart::new((0.0, hr(nd as f64 * tau)), (0.0, ymax(&[&joined])), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("dose {n}: C = {}", mgl(cv)));
    ch.line(&joined, "axp-c", "", false);
    ch.mark((xm, mg(cv)), &format!("dose {n}, t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Repeated IV doses accumulate towards a steady state",
       &format!("{nd} intravenous doses of {} mg every {} (V = {} L, k = {} 1/h): the concentration the model computes \
                 at 21 times in each interval rises towards its steady state. Marked: {} after dose {n}, C = {}.",
                tick(s.inp("D")? * 1e6), hours(tau), tick(s.inp("V")? * 1000.0), tick(s.inp("k")? * H), hours(t), mgl(cv)),
       &body)
}

fn oral(s: &Solved) -> Option<Figure> {
    let (f, d, v, ka, kk, t) = (s.inp("F")?, s.inp("D")?, s.inp("V")?, s.inp("ka")?, s.inp("k")?, s.inp("t")?);
    let cv = s.out("C")?;
    let pk_ = s.solve_other("bmed_pk_oral_peak", &[("F", fi(f)), ("D", fi(d)), ("V", fi(v)), ("ka", fi(ka)), ("k", fi(kk))]);
    let peak = pk_.as_ref().and_then(|a| Some((k::num_of(a, "t_max")?, k::num_of(a, "C_max")?)));
    let tend = (8.0 * t).max(peak.map(|p| 6.0 * p.0).unwrap_or(0.0));
    let p = conv(&self_curve(s, "C", tend, 120));
    if p.len() < 2 {
        return None;
    }
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&[&p])), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("C = {}", mgl(cv)));
    ch.line(&p, "axp-c", "", false);
    if let Some((tm, cm)) = peak {
        ch.dots(&[(hr(tm), mg(cm))], "axp-d3", 4.5);
        ch.text(ch.xa.px(hr(tm)) + 10.0, ch.ya.px(mg(cm)) + 4.0, "start", "axp-lt", &format!("peak {}", mgl(cm)));
    }
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Oral dose: concentration against time",
       &format!("Plasma concentration after one oral dose (D = {} mg, F = {}, V = {} L, ka = {} 1/h, k = {} 1/h), computed \
                 by the model at 120 times over {} h{}. Marked: t = {}, C = {}.", tick(d * 1e6), tick(f), tick(v * 1000.0),
                tick(ka * H), tick(kk * H), tick(hr(tend)),
                peak.map(|(tm, cm)| format!("; dot: the peak the model returns, t_max = {}, C_max = {}", hours(tm), mgl(cm)))
                    .unwrap_or_default(), hours(t), mgl(cv)), &body)
}

fn oral_peak(s: &Solved) -> Option<Figure> {
    let (f, d, v, ka, kk) = (s.inp("F")?, s.inp("D")?, s.inp("V")?, s.inp("ka")?, s.inp("k")?);
    let (tm, cm) = (s.out("t_max")?, s.out("C_max")?);
    let tend = 5.0 * tm;
    let mut series = vec![];
    let mut peaks = vec![];
    for (i, kav) in [ka, ka * 3.0, ka / 3.0].iter().enumerate() {
        let pts: Vec<(f64, f64)> = k::span(0.0, tend, 81, false).into_iter().filter_map(|x| {
            other(s, "bmed_pk_oral_concentration", &[("F", fi(f)), ("D", fi(d)), ("V", fi(v)), ("ka", fi(*kav)), ("k", fi(kk)),
                                                    ("t", fi(x))], "C").map(|y| (x, y))
        }).collect();
        let a = s.with_input(&[("ka", fi(*kav))]);
        if let Some(a) = a {
            if let (Some(t1), Some(c1)) = (k::num_of(&a, "t_max"), k::num_of(&a, "C_max")) {
                peaks.push((i, hr(t1), mg(c1)));
            }
        }
        if pts.len() >= 2 {
            series.push((*kav, conv(&pts)));
        }
    }
    if series.is_empty() {
        return None;
    }
    let all: Vec<&[(f64, f64)]> = series.iter().map(|x| x.1.as_slice()).collect();
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&all)), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("t_max = {}, C_max = {}", hours(tm), mgl(cm)));
    for (i, (kav, pts)) in series.iter().enumerate() {
        ch.line(pts, c::LINES[i], &format!("ka = {} 1/h", tick(kav * H)), false);
    }
    for (i, x, y) in &peaks {
        ch.dots(&[(*x, *y)], c::DOTS[*i], 4.5);
    }
    ch.mark((hr(tm), mg(cm)), "");
    let body = ch.body();
    pk(s, "Oral dose: the absorption rate sets the peak",
       &format!("Oral dose (F = {}, D = {} mg, V = {} L, k = {} 1/h) for three absorption rate constants (the given ka, three \
                 times and a third of it): concentration curves (81 runs each of the model's oral concentration) with the \
                 peak the model returns on each: {}. Faster absorption gives an earlier, higher peak.", tick(f), tick(d * 1e6),
                tick(v * 1000.0), tick(kk * H),
                peaks.iter().map(|(i, x, y)| format!("ka = {} 1/h: t_max {} h, C_max {} mg/L", tick(series.get(*i).map(|z| z.0).unwrap_or(0.0) * H),
                                                     tick(*x), tick(*y))).collect::<Vec<_>>().join("; ")), &body)
}

fn short_infusion(s: &Solved) -> Option<Figure> {
    let (ti_, t) = (s.inp("T_inf")?, s.inp("t")?);
    let (cv, ce) = (s.out("C")?, s.out("C_end")?);
    let tend = (3.0 * t).max(4.0 * ti_);
    let p = conv(&self_curve(s, "C", tend, 97));
    if p.len() < 2 {
        return None;
    }
    let mut ch = Chart::new((0.0, hr(tend)), (0.0, ymax(&[&p])), (false, false), (false, true), "time (h)",
                            "concentration (mg/L)", &format!("C_end = {}", mgl(ce)));
    ch.xband(0.0, hr(ti_), "axp-s4");
    ch.line(&p, "axp-c", "", false);
    ch.dots(&[(hr(ti_), mg(ce))], "axp-d3", 4.5);
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Short infusion: a rise, then an exponential decline",
       &format!("A short infusion of R0 = {} mg/h for T_inf = {} (CL = {} L/h, V = {} L, shaded: the infusion): \
                 concentration over {} h, 97 runs. Dot: at the end of the infusion C_end = {}; marked: t = {}, C = {}.",
                tick(s.inp("R0")? * 1e6 * H), hours(ti_), tick(s.inp("CL")? * 1000.0 * H), tick(s.inp("V")? * 1000.0),
                tick(hr(tend)), mgl(ce), hours(t), mgl(cv)), &body)
}

fn two_comp(s: &Solved) -> Option<Figure> {
    let t = s.inp("t")?;
    let (al, be, a, b, cv) = (s.out("alpha")?, s.out("beta")?, s.out("A")?, s.out("B")?, s.out("C")?);
    let tend = if t > 0.0 { 6.0 * t } else { 3.0 / be.max(1e-300) };
    let p = conv(&self_curve(s, "C", tend, 97));
    if p.len() < 2 {
        return None;
    }
    let xs = k::span(0.0, tend, 60, false);
    let fa = conv(&xs.iter().map(|x| (*x, a * (-al * x).exp())).collect::<Vec<_>>());
    let fb = conv(&xs.iter().map(|x| (*x, b * (-be * x).exp())).collect::<Vec<_>>());
    let ys: Vec<f64> = p.iter().map(|q| q.1).collect();
    let (lo, hi) = Chart::range_pos(&ys);
    let mut ch = Chart::new((0.0, hr(tend)), (lo.max(hi * 1e-4), hi * 1.05), (false, true), (false, true), "time (h)",
                            "central concentration (mg/L, log)", &format!("C = {}", mgl(cv)));
    ch.line(&fa, "axp-c3", &format!("A, alpha = {} 1/h", tick(al * H)), true);
    ch.line(&fb, "axp-c2", &format!("B, beta = {} 1/h", tick(be * H)), true);
    ch.line(&p, "axp-c", "central", false);
    ch.mark((hr(t), mg(cv)), &format!("t = {}", hours(t)));
    let body = ch.body();
    pk(s, "Two-compartment bolus: distribution, then elimination",
       &format!("Two-compartment intravenous bolus (D = {} mg, V1 = {} L, k10 = {}, k12 = {}, k21 = {} 1/h): central \
                 concentration over {} h (log axis, 97 runs) and its two exponential phases drawn from the returned \
                 A = {}, alpha = {} 1/h and B = {}, beta = {} 1/h (derived). Marked: t = {}, C = {}.", tick(s.inp("D")? * 1e6),
                tick(s.inp("V1")? * 1000.0), tick(s.inp("k10")? * H), tick(s.inp("k12")? * H), tick(s.inp("k21")? * H),
                tick(hr(tend)), mgl(a), tick(al * H), mgl(b), tick(be * H), hours(t), mgl(cv)), &body)
}

/// Trapezoids under sampled concentrations (kind "trapezoids").
pub fn trapezoids(s: &Solved) -> Option<Figure> {
    let (t, cv) = (s.vec("t")?, s.vec("C")?);
    let auc = s.out("AUC")?;
    let m = s.txt("method")?;
    if t.len() != cv.len() || t.len() < 2 || t.len() > 60 {
        return None;
    }
    let pts: Vec<(f64, f64)> = t.iter().copied().zip(cv.iter().copied()).collect();
    let mut ch = Chart::new(Chart::range(&t), (0.0, ymax(&[&pts])), (false, false), (false, true), "time (as given)",
                            "concentration (as given)", &format!("AUC = {} ({})", tick(auc), m.replace('_', " ")));
    for i in 0..pts.len() - 1 {
        let (a, b) = (pts[i], pts[i + 1]);
        let (xa, xb) = (ch.xa.px(a.0), ch.xa.px(b.0));
        let (ya, yb, y0) = (ch.ya.px(a.1), ch.ya.px(b.1), ch.ya.px(0.0));
        let class = if i % 2 == 0 { "axp-s" } else { "axp-s3" };
        ch.s.push_str(&format!("<path class=\"{class}\" d=\"M{xa:.1},{y0:.1} L{xa:.1},{ya:.1} L{xb:.1},{yb:.1} L{xb:.1},{y0:.1} Z\"/>\
                                <path class=\"axp-c4\" style=\"stroke-width:1\" d=\"M{xb:.1},{y0:.1} L{xb:.1},{yb:.1}\"/>"));
    }
    ch.line(&pts, "axp-c", "", false);
    ch.dots(&pts, "axp-d", 3.5);
    let body = ch.body();
    let others: Vec<String> = c::choices(s, "method").into_iter().filter(|o| *o != m).filter_map(|o| {
        let a = s.with_input(&[("method", ax_engine::Input::Text(o.clone()))])?;
        Some(format!("with the method {} the model gives AUC = {}", o.replace('_', " "), tick(k::num_of(&a, "AUC")?)))
    }).collect();
    Some(s.chart("trapezoids", "Area under sampled concentrations by trapezoids",
                 &format!("The {} concentration samples given and the {} trapezoids between them; the model's AUC by the {} \
                           method is {}{}.", t.len(), t.len() - 1, m.replace('_', " "), tick(auc),
                          if others.is_empty() { String::new() } else { format!(" ({})", others.join("; ")) }), &body))
}
