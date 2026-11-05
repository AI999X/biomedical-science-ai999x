//! Biomedical Science's drawing helpers beyond figs_common.rs and figs_extra.rs: inputs for further solves, curve
//! families, survival step curves read from the model's own Kaplan-Meier estimate, interval rows (forest plots).
#![allow(dead_code)]

use crate::figs_common::{self as c, clip, esc, tick, Axis, Chart};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::{Answer, Input};

pub fn fi(x: f64) -> Input {
    Input::Float(x)
}
pub fn ii(x: i64) -> Input {
    Input::Int(x)
}
pub fn ti(t: &str) -> Input {
    Input::Text(t.to_string())
}
pub fn vi(v: &[f64]) -> Input {
    Input::Json(serde_json::json!(v))
}
pub fn mi(m: &[Vec<f64>]) -> Input {
    Input::Json(serde_json::json!(m))
}

/// The same capability with some inputs replaced, one number output read.
pub fn run(s: &Solved, set: &[(&str, Input)], out: &str) -> Option<f64> {
    let a = s.with_input(set)?;
    k::num_of(&a, out)
}

/// Another capability of the model, one number output read.
pub fn other(s: &Solved, key: &str, set: &[(&str, Input)], out: &str) -> Option<f64> {
    let a: Answer = s.solve_other(key, set)?;
    k::num_of(&a, out)
}

/// The confidence level as given (0.95 for "95%" or 0.95), 0.95 when not given.
pub fn level(s: &Solved) -> f64 {
    s.inp("level").or_else(|| match s.val("level")? {
        ax_engine::Val::Num(x) => Some(x),
        _ => None,
    }).map(|l| if l > 1.0 { l / 100.0 } else { l }).unwrap_or(0.95)
}

/// "95 %" for 0.95.
pub fn pct(l: f64) -> String {
    format!("{} %", tick(l * 100.0))
}

// ------------------------------------------------------------------------------------------------ families
/// A family of curves: y against x over xs, one curve per value of `param` (SI), each run of the capability.
pub fn family(s: &Solved, x: &str, xs: &[f64], y: &str, param: &str, values: &[f64]) -> Vec<(f64, Vec<(f64, f64)>)> {
    let mut out = vec![];
    for v in values {
        let pts: Vec<(f64, f64)> = xs.iter().filter_map(|xv| {
            let a = s.with_input(&[(x, c::num_input(s, x, *xv)), (param, c::num_input(s, param, *v))])?;
            k::num_of(&a, y).map(|yv| (*xv, yv))
        }).collect();
        if pts.len() >= 2 {
            out.push((*v, pts));
        }
    }
    out
}

// ------------------------------------------------------------------------------------------------ survival
/// The step curve of a survival or cumulative-hazard estimate: `f(t)` is the model's value at time t (right
/// continuous); returns the step path from t = 0 to the last time, and the value at each censored time.
pub fn steps(times: &[f64], events: &[f64], f: &dyn Fn(f64) -> Option<f64>, start: f64)
             -> Option<(Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    if times.is_empty() || times.len() != events.len() {
        return None;
    }
    let mut ts: Vec<f64> = times.to_vec();
    ts.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    ts.dedup();
    if ts.len() > 90 {
        return None;
    }
    let mut path = vec![(0.0, start)];
    let mut cur = start;
    let mut vals: Vec<(f64, f64)> = vec![];
    for t in &ts {
        let v = f(*t)?;
        path.push((*t, cur));
        path.push((*t, v));
        cur = v;
        vals.push((*t, v));
    }
    let cens: Vec<(f64, f64)> = times.iter().zip(events).filter(|(_, e)| **e == 0.0)
        .filter_map(|(t, _)| vals.iter().find(|v| v.0 == *t).map(|v| (*t, v.1))).collect();
    Some((path, cens))
}

/// The Kaplan-Meier estimate of a sample at t, as the model's Kaplan-Meier capability returns it.
pub fn km_at(s: &Solved, times: &[f64], events: &[f64], t: f64) -> Option<f64> {
    other(s, "bmed_km_survival_median", &[("times", vi(times)), ("events", vi(events)), ("t", fi(t))], "S_t")
}

/// The Kaplan-Meier step curve of a sample (and its censored points).
pub fn km(s: &Solved, times: &[f64], events: &[f64]) -> Option<(Vec<(f64, f64)>, Vec<(f64, f64)>)> {
    steps(times, events, &|t| km_at(s, times, events, t), 1.0)
}

/// Censoring ticks (short vertical marks) on a chart.
pub fn ticks_at(ch: &mut Chart, pts: &[(f64, f64)], class: &str) {
    for p in pts {
        if ch.xa.inside(p.0) && ch.ya.inside(p.1) {
            let (x, y) = (ch.xa.px(p.0), ch.ya.px(p.1));
            ch.s.push_str(&format!("<path class=\"{class}\" d=\"M{x:.1},{:.1} L{x:.1},{:.1}\"/>", y - 6.0, y + 6.0));
        }
    }
}

// ------------------------------------------------------------------------------------------------ interval rows
/// One row of an interval plot: label, estimate, lower, upper, highlighted.
pub struct Row {
    pub label: String,
    pub est: f64,
    pub lo: f64,
    pub hi: f64,
    pub hl: bool,
}

/// Rows of estimates with their intervals (a forest plot): labels on the left, a horizontal value axis (logarithmic
/// for ratios), a reference line (1 for a ratio); the estimate and interval written at the right. Body and height.
pub fn interval_rows(rows: &[Row], xlab: &str, header: &str, log: bool, reference: Option<f64>) -> Option<(String, f64)> {
    if rows.is_empty() || rows.iter().any(|r| !(r.est.is_finite() && r.lo.is_finite() && r.hi.is_finite())) {
        return None;
    }
    if log && rows.iter().any(|r| r.lo <= 0.0) {
        return None;
    }
    let mut all: Vec<f64> = rows.iter().flat_map(|r| [r.lo, r.hi, r.est]).collect();
    if let Some(rf) = reference {
        all.push(rf);
    }
    let (lo, hi) = Chart::range(&all);
    let (x0, x1) = (150.0, 330.0);
    let xa = Axis::new(lo, hi, log, x0, x1, true);
    let top = 44.0;
    let rh = 30.0;
    let bottom = top + rh * rows.len() as f64;
    let h = bottom + 56.0;
    let mut out = String::new();
    if !header.is_empty() {
        out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(&clip(header, 54))));
    }
    // ticks at least 44 px apart
    let mut last = f64::NEG_INFINITY;
    let tks: Vec<f64> = xa.ticks().into_iter().filter(|t| {
        let x = xa.px(*t);
        let keep = x - last >= 44.0;
        if keep {
            last = x;
        }
        keep
    }).collect();
    for t in tks {
        let x = xa.px(t);
        out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{top}\" x2=\"{x:.1}\" y2=\"{bottom}\"/>\
                               <text class=\"axp-t\" x=\"{x:.1}\" y=\"{:.1}\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                              bottom + 18.0, esc(&tick(t))));
    }
    if let Some(rf) = reference {
        let x = xa.px(rf);
        out.push_str(&format!("<path class=\"axp-c4\" stroke-dasharray=\"5 4\" d=\"M{x:.1},{top} L{x:.1},{bottom}\"/>"));
    }
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>", (x0 + x1) / 2.0,
                          bottom + 42.0, esc(&clip(xlab, 40))));
    out.push_str(&format!("<text class=\"axp-lt\" x=\"508\" y=\"{:.1}\" text-anchor=\"end\">estimate (interval)</text>", top - 8.0));
    for (i, r) in rows.iter().enumerate() {
        let y = top + rh * (i as f64 + 0.5);
        if r.hl {
            out.push_str(&format!("<rect class=\"axp-ka\" x=\"8\" y=\"{:.1}\" width=\"504\" height=\"{rh}\"/>", y - rh / 2.0));
        }
        out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 10.0, y + 5.0,
                              esc(&clip(&r.label, 17))));
        let (a, b, e) = (xa.px(r.lo), xa.px(r.hi), xa.px(r.est));
        let crosses = reference.map(|rf| r.lo <= rf && r.hi >= rf).unwrap_or(false);
        let (lc, dc) = if crosses { ("axp-c4", "axp-d4") } else { ("axp-c", "axp-d") };
        out.push_str(&format!("<path class=\"{lc}\" d=\"M{a:.1},{y:.1} L{b:.1},{y:.1} M{a:.1},{:.1} L{a:.1},{:.1} M{b:.1},{:.1} L{b:.1},{:.1}\"/>\
                               <rect class=\"{dc}\" x=\"{:.1}\" y=\"{:.1}\" width=\"10\" height=\"10\"/>",
                              y - 5.0, y + 5.0, y - 5.0, y + 5.0, e - 5.0, y - 5.0));
        out.push_str(&format!("<text class=\"axp-vt\" x=\"508\" y=\"{:.1}\" text-anchor=\"end\" font-size=\"12\">{} ({} to {})</text>",
                              y + 5.0, esc(&tick(r.est)), esc(&tick(r.lo)), esc(&tick(r.hi))));
    }
    Some((out, h))
}

/// Values of a list written short ("6, 6, 6, 7, ..." when long).
pub fn list_text(v: &[f64], max: usize) -> String {
    let mut s: Vec<String> = v.iter().take(max).map(|x| tick(*x)).collect();
    if v.len() > max {
        s.push("...".into());
    }
    s.join(", ")
}
