//! Survival, diagnostic-test and epidemiology figures: Kaplan-Meier steps from the model's own estimate at every step
//! time, ROC curves from its sensitivity and specificity at each threshold, interval rows and forest plots from its
//! interval capabilities, risk comparisons and count tables.

use crate::figs_common::{clip, esc, tick, Axis, Chart, CX0, CX1};
use crate::figs_med::{self as m, fi, ii, km, km_at, other, steps, ticks_at, vi, Row};
use ax_engine::figkit::{self as k, Solved};
use ax_engine::figures::Figure;

pub fn live(s: &Solved) -> Vec<Option<Figure>> {
    match s.key {
        "bmed_cox_hazard_ratio" => vec![cox(s)],
        "bmed_exponential_hazard_ratio" => vec![two_groups(s, true)],
        "bmed_logrank_test" => vec![two_groups(s, false)],
        "bmed_exponential_survival_fit" => vec![exp_fit(s)],
        "bmed_km_confidence_interval" => vec![km_bands(s)],
        "bmed_km_survival_median" => vec![follow_up(s), km_median(s)],
        "bmed_nelson_aalen_hazard" => vec![nelson_aalen(s)],
        "bmed_restricted_mean_survival_time" => vec![rmst(s)],
        "bmed_dx_roc_auc" => vec![roc(s, false)],
        "bmed_dx_threshold_sensitivity_specificity" => vec![roc(s, true)],
        "bmed_dx_sensitivity_specificity_ci" => vec![sens_spec_rows(s)],
        "bmed_incidence_rate_ratio_ci" => vec![irr_rows(s)],
        "bmed_smr_exact_ci" => vec![smr_rows(s)],
        "bmed_mantel_haenszel_odds_ratio" => vec![mh_forest(s)],
        "bmed_odds_ratio_ci" => vec![or_forest(s), or_table(s)],
        "bmed_nnt_ci" => vec![risk(s, true)],
        "bmed_relative_risk_ci" => vec![risk(s, false)],
        "bmed_cohens_kappa" => vec![kappa_table(s)],
        _ => vec![],
    }
}

fn surv_chart(tmax: f64, ylab: &str, header: &str, ymax: f64) -> Chart {
    Chart::new((0.0, tmax), (0.0, ymax), (false, false), (false, false), "time (as given)", ylab, header)
}

fn tmax(t: &[f64]) -> f64 {
    t.iter().cloned().fold(0.0, f64::max)
}

fn cox(s: &Solved) -> Option<Figure> {
    let (t, e, x) = (s.vec("times")?, s.vec("events")?, s.vec("x")?);
    let (hr, lo, hi, beta, se) = (s.out("HR")?, s.out("HR_lower")?, s.out("HR_upper")?, s.out("beta")?, s.out("se")?);
    if t.len() != e.len() || t.len() != x.len() {
        return None;
    }
    let mut vals: Vec<f64> = x.clone();
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    vals.dedup();
    if vals.len() != 2 {
        return None;
    }
    let mut ch = surv_chart(tmax(&t), "survival", &format!("HR = {} per unit of x", tick(hr)), 1.0);
    for (i, g) in vals.iter().rev().enumerate() {
        let tg: Vec<f64> = t.iter().zip(&x).filter(|p| p.1 == g).map(|p| *p.0).collect();
        let eg: Vec<f64> = e.iter().zip(&x).filter(|p| p.1 == g).map(|p| *p.0).collect();
        let (path, cens) = km(s, &tg, &eg)?;
        ch.line(&path, ["axp-c", "axp-c2"][i], &format!("x = {} ({} subjects)", tick(*g), tg.len()), false);
        ticks_at(&mut ch, &cens, ["axp-c", "axp-c2"][i]);
    }
    let body = ch.body();
    Some(s.chart("survival", &format!("Survival by group: hazard ratio {}", tick(hr)),
                 &format!("Kaplan-Meier curves of the two covariate groups given ({} subjects; ticks mark censored times), \
                           each step the model's survival estimate for that group. The proportional-hazards fit returns \
                           HR = {} per unit of x ({} interval {} to {}, beta = {}, se {}).", t.len(), tick(hr),
                          m::pct(m::level(s)), tick(lo), tick(hi), tick(beta), tick(se)), &body))
}

fn group(s: &Solved, i: usize) -> Option<(Vec<f64>, Vec<f64>)> {
    let (t, e) = (s.vec(&format!("times{i}"))?, s.vec(&format!("events{i}"))?);
    (t.len() == e.len()).then_some((t, e))
}

fn two_groups(s: &Solved, exp: bool) -> Option<Figure> {
    let (t1, e1) = group(s, 1)?;
    let (t2, e2) = group(s, 2)?;
    let tm = tmax(&t1).max(tmax(&t2));
    let header = if exp { format!("HR = {}", tick(s.out("HR")?)) } else { format!("log-rank p = {}", tick(s.out("p_value")?)) };
    let mut ch = surv_chart(tm, "survival", &header, 1.0);
    let mut lam = vec![];
    for (i, (t, e)) in [(&t1, &e1), (&t2, &e2)].iter().enumerate() {
        let (path, cens) = km(s, t, e)?;
        let class = ["axp-c", "axp-c2"][i];
        ch.line(&path, class, &format!("group {} ({})", i + 1, t.len()), false);
        ticks_at(&mut ch, &cens, class);
        if exp {
            if let Some(l) = other(s, "bmed_exponential_survival_fit", &[("times", vi(t)), ("events", vi(e))], "hazard") {
                let pts: Vec<(f64, f64)> = k::span(0.0, tm, 60, false).into_iter().map(|x| (x, (-l * x).exp())).collect();
                ch.line(&pts, class, "", true);
                lam.push(l);
            }
        }
    }
    let body = ch.body();
    let (title, cap) = if exp {
        (format!("Two survival curves: hazard ratio {}", tick(s.out("HR")?)),
         format!("Kaplan-Meier curves of the two groups given ({} and {} subjects; ticks mark censored times), every step \
                  the model's survival estimate. Returned: exponential hazard ratio (group 1 / group 2) {}, {} interval {} \
                  to {}; dashed: exp(-lambda t) from each group's exponential hazard ({}, derived).", t1.len(), t2.len(),
                 tick(s.out("HR")?), m::pct(m::level(s)), tick(s.out("lower")?), tick(s.out("upper")?),
                 lam.iter().map(|l| tick(*l)).collect::<Vec<_>>().join(" and ")))
    } else {
        (format!("Two survival curves: log-rank p = {}", tick(s.out("p_value")?)),
         format!("Kaplan-Meier curves of the two groups given ({} and {} subjects; ticks mark censored times), every step \
                  the model's survival estimate. Returned: log-rank chi-square = {} (1 degree of freedom), p = {}.",
                 t1.len(), t2.len(), tick(s.out("chi_square")?), tick(s.out("p_value")?)))
    };
    Some(s.chart("survival", &title, &cap, &body))
}

fn exp_fit(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let (l, mean, med) = (s.out("hazard")?, s.out("mean")?, s.out("median")?);
    let tm = tmax(&t).max(med);
    let (path, cens) = km(s, &t, &e)?;
    let mut ch = surv_chart(tm, "survival", &format!("hazard = {}", tick(l)), 1.0);
    ch.line(&path, "axp-c", "Kaplan-Meier", false);
    ticks_at(&mut ch, &cens, "axp-c");
    let pts: Vec<(f64, f64)> = k::span(0.0, tm, 60, false).into_iter().map(|x| (x, (-l * x).exp())).collect();
    ch.line(&pts, "axp-c3", "exponential fit", true);
    ch.hguide(0.5, "");
    ch.mark((med, 0.5), &format!("median {}", tick(med)));
    let body = ch.body();
    Some(s.chart("survival", "Exponential fit against the Kaplan-Meier curve",
                 &format!("Exponential survival fitted to the {} follow-up times given ({} events; ticks: censored): the \
                           returned hazard lambda = {} per time unit gives the dashed curve exp(-lambda t) (derived), beside \
                           the model's Kaplan-Meier steps. Marked: median ln2/lambda = {}; mean 1/lambda = {}.", t.len(),
                          e.iter().filter(|x| **x != 0.0).count(), tick(l), tick(med), tick(mean)), &body))
}

fn km_bands(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let (st, lo, hi, tv) = (s.out("S_t")?, s.out("lower")?, s.out("upper")?, s.inp("t").or_else(|| crate::figs_common::getv(s, "t"))?);
    let (path, cens) = km(s, &t, &e)?;
    let mut ch = surv_chart(tmax(&t), "survival", &format!("S({}) = {}", tick(tv), tick(st)), 1.0);
    for (lv, class) in [(0.99, "axp-s4"), (0.8, "axp-s")] {
        let lower = steps(&t, &e, &|x| m::run(s, &[("t", fi(x)), ("level", fi(lv))], "lower"), 1.0)?.0;
        let upper = steps(&t, &e, &|x| m::run(s, &[("t", fi(x)), ("level", fi(lv))], "upper"), 1.0)?.0;
        ch.area(&lower, &upper, class, "");
    }
    ch.line(&path, "axp-c", "", false);
    ticks_at(&mut ch, &cens, "axp-c");
    let (px, a, b) = (ch.xa.px(tv), ch.ya.px(lo), ch.ya.px(hi));
    ch.s.push_str(&format!("<path class=\"axp-c2\" d=\"M{px:.1},{a:.1} L{px:.1},{b:.1} M{:.1},{a:.1} L{:.1},{a:.1} M{:.1},{b:.1} L{:.1},{b:.1}\"/>",
                           px - 5.0, px + 5.0, px - 5.0, px + 5.0));
    ch.mark((tv, st), "");
    ch.text(CX1 - 6.0, 62.0, "end", "axp-lt", "bands: 80 % (dark), 99 % (light)");
    let body = ch.body();
    Some(s.chart("survival", "Survival estimate with 80 % and 99 % confidence bands",
                 &format!("Kaplan-Meier estimate of the {} follow-up times given with its pointwise 80 % and 99 % \
                           confidence bands, both computed by the model at every step time. Marked: at t = {} the {} \
                           interval {} to {} around S = {}.", t.len(), tick(tv), m::pct(m::level(s)), tick(lo), tick(hi),
                          tick(st)), &body))
}

fn km_median(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let st = s.out("S_t")?;
    let tv = s.inp("t").or_else(|| crate::figs_common::getv(s, "t"))?;
    let med = s.out("median");
    let (path, cens) = km(s, &t, &e)?;
    let ci = |x: f64, o: &str| other(s, "bmed_km_confidence_interval",
                                     &[("times", vi(&t)), ("events", vi(&e)), ("t", fi(x)), ("level", fi(0.95))], o);
    let lower = steps(&t, &e, &|x| ci(x, "lower"), 1.0).map(|p| p.0);
    let upper = steps(&t, &e, &|x| ci(x, "upper"), 1.0).map(|p| p.0);
    let mut ch = surv_chart(tmax(&t), "survival", &format!("S({}) = {}", tick(tv), tick(st)), 1.0);
    if let (Some(l), Some(u)) = (&lower, &upper) {
        ch.area(l, u, "axp-s", "");
    }
    ch.line(&path, "axp-c", "", false);
    ticks_at(&mut ch, &cens, "axp-c");
    ch.hguide(0.5, "");
    if let Some(md) = med {
        ch.vguide(md, &format!("median {}", tick(md)));
    }
    ch.mark((tv, st), &format!("t = {}", tick(tv)));
    let body = ch.body();
    Some(s.chart("survival", "Kaplan-Meier survival curve with its confidence band",
                 &format!("Kaplan-Meier survival estimate of the {} subjects given ({} events, the rest censored){}, both \
                           computed by the model at every step time. Marked: S({}) = {}{}.", t.len(),
                          e.iter().filter(|x| **x != 0.0).count(),
                          if lower.is_some() { ", with its pointwise 95 % confidence band" } else { "" }, tick(tv), tick(st),
                          med.map(|md| format!("; median survival {}", tick(md))).unwrap_or_default()), &body))
}

fn follow_up(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let st = s.out("S_t")?;
    let tv = s.inp("t").or_else(|| crate::figs_common::getv(s, "t"))?;
    let med = s.out("median");
    if t.len() != e.len() || t.is_empty() || t.len() > 40 {
        return None;
    }
    let mut idx: Vec<usize> = (0..t.len()).collect();
    idx.sort_by(|a, b| t[*a].partial_cmp(&t[*b]).unwrap_or(std::cmp::Ordering::Equal));
    let h = 360.0;
    let (y0, y1) = (40.0, 300.0);
    let xa = Axis::new(0.0, tmax(&t).max(tv), false, CX0, CX1, true);
    let mut out = String::new();
    out.push_str(&format!("<text class=\"axp-mt\" x=\"{CX0}\" y=\"22\">S({}) = {}{}</text>", esc(&tick(tv)), esc(&tick(st)),
                          esc(&med.map(|md| format!(", median {}", tick(md))).unwrap_or_default())));
    for tk in xa.ticks() {
        let x = xa.px(tk);
        out.push_str(&format!("<line class=\"axp-g\" x1=\"{x:.1}\" y1=\"{y0}\" x2=\"{x:.1}\" y2=\"{y1}\"/>\
                               <text class=\"axp-t\" x=\"{x:.1}\" y=\"{}\" text-anchor=\"middle\">{}</text>", y1 + 20.0, esc(&tick(tk))));
    }
    let rh = (y1 - y0) / t.len() as f64;
    for (r, i) in idx.iter().enumerate() {
        let y = y0 + rh * (r as f64 + 0.5);
        let (a, b) = (xa.px(0.0), xa.px(t[*i]));
        let ev = e[*i] != 0.0;
        out.push_str(&format!("<path class=\"{}\" style=\"stroke-width:{:.1}\" d=\"M{a:.1},{y:.1} L{b:.1},{y:.1}\"/>",
                              if ev { "axp-c" } else { "axp-c4" }, (rh * 0.45).clamp(1.2, 3.0)));
        if ev {
            out.push_str(&format!("<circle class=\"axp-d\" cx=\"{b:.1}\" cy=\"{y:.1}\" r=\"{:.1}\"/>", (rh * 0.4).clamp(2.0, 4.5)));
        } else {
            out.push_str(&format!("<path class=\"axp-c4\" d=\"M{b:.1},{:.1} L{b:.1},{:.1}\"/>", y - rh * 0.45, y + rh * 0.45));
        }
    }
    for (x, lab) in [(Some(tv), format!("t = {}", tick(tv))), (med, format!("median {}", tick(med.unwrap_or(0.0))))] {
        if let Some(x) = x {
            let px = xa.px(x);
            out.push_str(&format!("<path class=\"axp-c2\" stroke-dasharray=\"5 4\" d=\"M{px:.1},{y0} L{px:.1},{y1}\"/>"));
            // the two labels on opposite sides of their lines when the lines are close
            let left_side = match (lab.starts_with('t'), med) {
                (true, Some(md)) => tv <= md,
                (false, _) => tv > med.unwrap_or(0.0),
                _ => px > 300.0,
            };
            let anchor = if left_side { "end" } else { "start" };
            let dx = if anchor == "end" { -4.0 } else { 4.0 };
            out.push_str(&format!("<text class=\"axp-vt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{anchor}\">{}</text>", px + dx,
                                  if lab.starts_with('t') { y0 + 12.0 } else { y0 + 28.0 }, esc(&lab)));
        }
    }
    out.push_str(&format!("<rect class=\"axp-f\" x=\"{CX0}\" y=\"{y0}\" width=\"{}\" height=\"{}\"/>", CX1 - CX0, y1 - y0));
    out.push_str(&format!("<text class=\"axp-l\" x=\"{:.1}\" y=\"346\" text-anchor=\"middle\">follow-up time (as given)</text>",
                          (CX0 + CX1) / 2.0));
    out.push_str(&format!("<text class=\"axp-l\" transform=\"translate(16,{:.1}) rotate(-90)\" text-anchor=\"middle\">subjects</text>",
                          (y0 + y1) / 2.0));
    let nev = e.iter().filter(|x| **x != 0.0).count();
    Some(s.figure("follow-up", "Follow-up of each subject with the median survival",
                  &format!("The {} subjects given as follow-up lines from 0 to their time (dot: event observed, {}; bar \
                            end: censored, {}), the data the Kaplan-Meier estimate is computed from. Marked: t = {}, where \
                            the model returns S = {}{}.", t.len(), nev, t.len() - nev, tick(tv), tick(st),
                           med.map(|md| format!(", and the returned median survival {}", tick(md))).unwrap_or_default()),
                  &out, h))
}

fn nelson_aalen(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let ht = s.out("H_t")?;
    let tv = s.inp("t").or_else(|| crate::figs_common::getv(s, "t"))?;
    let (path, cens) = steps(&t, &e, &|x| m::run(s, &[("t", fi(x))], "H_t"), 0.0)?;
    let ym = path.iter().map(|p| p.1).fold(ht, f64::max);
    let mut ch = surv_chart(tmax(&t).max(tv), "cumulative hazard", &format!("H({}) = {}", tick(tv), tick(ht)), ym * 1.08);
    ch.line(&path, "axp-c", "", false);
    ticks_at(&mut ch, &cens, "axp-c");
    ch.mark((tv, ht), &format!("t = {}", tick(tv)));
    let body = ch.body();
    Some(s.chart("survival", "Cumulative hazard step curve",
                 &format!("Nelson-Aalen cumulative hazard of the {} follow-up times given ({} events; ticks: censored), the \
                           model's estimate at every step time: each event adds 1/(number at risk). Marked: H({}) = {}.",
                          t.len(), e.iter().filter(|x| **x != 0.0).count(), tick(tv), tick(ht)), &body))
}

fn rmst(s: &Solved) -> Option<Figure> {
    let (t, e) = (s.vec("times")?, s.vec("events")?);
    let r = s.out("RMST")?;
    let tau = s.inp("tau").or_else(|| crate::figs_common::getv(s, "tau"))?;
    let (path, cens) = km(s, &t, &e)?;
    let mut area: Vec<(f64, f64)> = path.iter().copied().filter(|p| p.0 <= tau).collect();
    let stau = km_at(s, &t, &e, tau)?;
    area.push((tau, area.last().map(|p| p.1).unwrap_or(1.0).min(1.0)));
    let _ = stau;
    let zero: Vec<(f64, f64)> = area.iter().map(|p| (p.0, 0.0)).collect();
    let mut ch = surv_chart(tmax(&t).max(tau), "survival", &format!("RMST = {}", tick(r)), 1.0);
    ch.area(&zero, &area, "axp-s", "");
    ch.line(&path, "axp-c", "", false);
    ticks_at(&mut ch, &cens, "axp-c");
    ch.vguide(tau, &format!("tau = {}", tick(tau)));
    let body = ch.body();
    Some(s.chart("survival", "Restricted mean survival time as an area",
                 &format!("Restricted mean survival time: the area under the Kaplan-Meier curve of the {} follow-up times \
                           given from 0 to tau = {} (shaded; the curve is the model's estimate at every step). Marked: \
                           RMST = {} time units, the expected survival time within the first {}.", t.len(), tick(tau),
                          tick(r), tick(tau)), &body))
}

// ------------------------------------------------------------------------------------------------ ROC
fn roc(s: &Solved, mine: bool) -> Option<Figure> {
    let (p, n) = (s.vec("scores_pos")?, s.vec("scores_neg")?);
    if p.is_empty() || n.is_empty() || p.len() + n.len() > 80 {
        return None;
    }
    let mut th: Vec<f64> = p.iter().chain(n.iter()).copied().collect();
    th.sort_by(|a, b| a.partial_cmp(b).unwrap());
    th.dedup();
    let top = th.last().copied()? + (th.last().copied()? - th[0]).abs().max(1.0) * 0.1;
    th.push(top);
    let pt = |x: f64| -> Option<(f64, f64)> {
        let set = [("scores_pos", vi(&p)), ("scores_neg", vi(&n)), ("threshold", fi(x))];
        let a = if mine { s.with_input(&[("threshold", fi(x))])? } else {
            s.solve_other("bmed_dx_threshold_sensitivity_specificity", &set)?
        };
        Some((1.0 - k::num_of(&a, "specificity")?, k::num_of(&a, "sensitivity")?))
    };
    let mut pts: Vec<(f64, f64, f64)> = th.iter().filter_map(|x| pt(*x).map(|q| (q.0, q.1, *x))).collect();
    if pts.len() < 2 {
        return None;
    }
    pts.sort_by(|a, b| (a.0, a.1).partial_cmp(&(b.0, b.1)).unwrap());
    let auc = if mine {
        other(s, "bmed_dx_roc_auc", &[("scores_pos", vi(&p)), ("scores_neg", vi(&n))], "AUC")
    } else {
        s.out("AUC")
    };
    let header = auc.map(|a| format!("AUC = {}", tick(a))).unwrap_or_default();
    let mut ch = Chart::new((0.0, 1.0), (0.0, 1.0), (false, false), (true, true), "1 - specificity",
                            "sensitivity", &header);
    let line: Vec<(f64, f64)> = pts.iter().map(|q| (q.0, q.1)).collect();
    let zero: Vec<(f64, f64)> = line.iter().map(|q| (q.0, 0.0)).collect();
    ch.area(&zero, &line, "axp-s", "");
    ch.line(&[(0.0, 0.0), (1.0, 1.0)], "axp-c4", "", true);
    ch.line(&line, "axp-c", "", false);
    ch.dots(&line, "axp-d", 3.0);
    let thr = if mine { s.inp("threshold").or_else(|| crate::figs_common::getv(s, "threshold")) } else { None };
    if let Some(tv) = thr {
        let (se, sp) = (s.out("sensitivity")?, s.out("specificity")?);
        ch.mark((1.0 - sp, se), &format!("cut-off {}", tick(tv)));
    }
    // the scores below, on their own axis
    let mut body = ch.body();
    let all: Vec<f64> = p.iter().chain(n.iter()).copied().chain(thr).collect();
    let xa = Axis::new(Chart::range(&all).0, Chart::range(&all).1, false, CX0, CX1, true);
    for tk in xa.ticks() {
        body.push_str(&format!("<text class=\"axp-t\" x=\"{:.1}\" y=\"458\" text-anchor=\"middle\" font-size=\"12\">{}</text>",
                               xa.px(tk), esc(&tick(tk))));
    }
    for (lab, v, y, class) in [("diseased", &p, 380.0, "axp-d"), ("healthy", &n, 416.0, "axp-d4")] {
        body.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{lab}</text>\
                                <path class=\"axp-c4\" style=\"stroke-width:1\" d=\"M{CX0},{y} L{CX1},{y}\"/>", CX0 - 6.0, y + 4.0));
        let mut seen: Vec<(f64, usize)> = vec![];
        for x in v.iter() {
            let c = seen.iter().filter(|q| (q.0 - x).abs() < 1e-12).count();
            seen.push((*x, c));
            body.push_str(&format!("<circle class=\"{class}\" cx=\"{:.1}\" cy=\"{:.1}\" r=\"4.5\"/>", xa.px(*x), y - 9.0 * c as f64));
        }
    }
    if let Some(tv) = thr {
        let x = xa.px(tv);
        body.push_str(&format!("<path class=\"axp-c2\" stroke-dasharray=\"4 3\" d=\"M{x:.1},358 L{x:.1},428\"/>"));
    }
    body.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"474\" text-anchor=\"middle\">score</text>", (CX0 + CX1) / 2.0));
    let cap = if mine {
        format!("ROC curve of a test with {} diseased and {} healthy scores (dots below): sensitivity and specificity \
                 computed by the model at {} thresholds{}. Marked: the cut-off given, {}, sensitivity {}, specificity {}.",
                p.len(), n.len(), pts.len(), auc.map(|a| format!("; area under the curve {}", tick(a))).unwrap_or_default(),
                tick(thr.unwrap_or(0.0)), tick(s.out("sensitivity")?), tick(s.out("specificity")?))
    } else {
        format!("Scores of the {} diseased and {} healthy subjects given (bottom) and the ROC curve they give (top; \
                 sensitivity and specificity from the model at each of {} thresholds). The model's area under the curve \
                 AUC = {}, equal to the Mann-Whitney U = {} divided by {} pairs.", p.len(), n.len(), pts.len(),
                tick(auc?), tick(s.out("U")?), p.len() * n.len())
    };
    Some(s.figure("roc", if mine { "ROC curve of a diagnostic test with the cut-off given" } else { "ROC area from the two score groups" },
                  &cap, &body, 480.0))
}

// ------------------------------------------------------------------------------------------------ intervals
fn sens_spec_rows(s: &Solved) -> Option<Figure> {
    let (tp, fne, tn, fp) = (crate::figs_common::getv(s, "TP")?, crate::figs_common::getv(s, "FN")?,
                             crate::figs_common::getv(s, "TN")?, crate::figs_common::getv(s, "FP")?);
    let mut rows = vec![];
    for (what, o, lo, hi) in [("sens", "sensitivity", "sens_lower", "sens_upper"), ("spec", "specificity", "spec_lower", "spec_upper")] {
        for f in [0.4, 1.0, 4.0] {
            let sc = |x: f64| ii((x * f).round() as i64);
            let a = s.with_input(&[("TP", sc(tp)), ("FN", sc(fne)), ("TN", sc(tn)), ("FP", sc(fp))])?;
            rows.push(Row { label: format!("{what} x {}", tick(f)), est: k::num_of(&a, o)?, lo: k::num_of(&a, lo)?,
                            hi: k::num_of(&a, hi)?, hl: f == 1.0 });
        }
    }
    let (body, h) = m::interval_rows(&rows, "proportion", &format!("sensitivity {}, specificity {}", tick(s.out("sensitivity")?),
                                                                   tick(s.out("specificity")?)), false, None)?;
    Some(s.figure("intervals", "Sensitivity and specificity intervals narrow with sample size",
                  &format!("Sensitivity and specificity with their {} Wilson intervals for the counts given (TP = {}, FN = {}, \
                            TN = {}, FP = {}) and for the same proportions with the counts scaled by 0.4 and 4 (the label \
                            gives the factor), each a run of the model: more data, narrower intervals. Highlighted: the \
                            counts given, sensitivity {} ({} to {}), specificity {} ({} to {}).", m::pct(m::level(s)),
                           tick(tp), tick(fne), tick(tn), tick(fp), tick(s.out("sensitivity")?), tick(s.out("sens_lower")?),
                           tick(s.out("sens_upper")?), tick(s.out("specificity")?), tick(s.out("spec_lower")?),
                           tick(s.out("spec_upper")?)), &body, h))
}

fn irr_rows(s: &Solved) -> Option<Figure> {
    let (e1, t1, e2, t2) = (crate::figs_common::getv(s, "events1")?, crate::figs_common::getv(s, "time1")?,
                            crate::figs_common::getv(s, "events2")?, crate::figs_common::getv(s, "time2")?);
    let mut rows = vec![];
    for f in [0.4, 1.0, 2.0, 4.0] {
        let a = s.with_input(&[("events1", ii((e1 * f).round() as i64)), ("time1", fi(t1 * f)),
                               ("events2", ii((e2 * f).round() as i64)), ("time2", fi(t2 * f))])?;
        rows.push(Row { label: format!("counts x {}", tick(f)), est: k::num_of(&a, "IRR")?, lo: k::num_of(&a, "lower")?,
                        hi: k::num_of(&a, "upper")?, hl: f == 1.0 });
    }
    let (body, h) = m::interval_rows(&rows, "incidence rate ratio (log)", &format!("IRR = {}", tick(s.out("IRR")?)), true, Some(1.0))?;
    Some(s.figure("intervals", "Incidence rate ratio: the interval against the amount of data",
                  &format!("The incidence rate ratio with its {} interval for the counts given (events1 = {}, time1 = {}, \
                            events2 = {}, time2 = {}) and the same proportions with the counts scaled by 0.4, 2 and 4, each \
                            a run of the model: more data, narrower intervals. Highlighted: the counts given, IRR {} ({} to \
                            {}).", m::pct(m::level(s)), tick(e1), tick(t1), tick(e2), tick(t2), tick(s.out("IRR")?),
                           tick(s.out("lower")?), tick(s.out("upper")?)), &body, h))
}

fn smr_rows(s: &Solved) -> Option<Figure> {
    let (o, e) = (crate::figs_common::getv(s, "observed")?, crate::figs_common::getv(s, "expected")?);
    let mut rows = vec![];
    for f in [0.4, 1.0, 2.0, 4.0] {
        let a = s.with_input(&[("observed", ii((o * f).round() as i64)), ("expected", fi(e * f))])?;
        rows.push(Row { label: format!("counts x {}", tick(f)), est: k::num_of(&a, "SMR")?, lo: k::num_of(&a, "lower")?,
                        hi: k::num_of(&a, "upper")?, hl: f == 1.0 });
    }
    let (body, h) = m::interval_rows(&rows, "standardised mortality ratio (log)", &format!("SMR = {}", tick(s.out("SMR")?)), true,
                                     Some(1.0))?;
    Some(s.figure("intervals", "Standardised mortality ratio: the exact interval against the counts",
                  &format!("The standardised mortality ratio with its exact {} interval for the counts given (observed = {}, \
                            expected = {}) and the same proportions with the counts scaled by 0.4, 2 and 4, each a run of the \
                            model: more data, narrower intervals. Highlighted: the counts given, SMR {} ({} to {}).",
                           m::pct(m::level(s)), tick(o), tick(e), tick(s.out("SMR")?), tick(s.out("lower")?),
                           tick(s.out("upper")?)), &body, h))
}

fn or_of(s: &Solved, abcd: &[f64], lv: f64) -> Option<(f64, f64, f64)> {
    let a = s.solve_other("bmed_odds_ratio_ci", &[("a", ii(abcd[0].round() as i64)), ("b", ii(abcd[1].round() as i64)),
                                                  ("c", ii(abcd[2].round() as i64)), ("d", ii(abcd[3].round() as i64)),
                                                  ("level", fi(lv))])?;
    Some((k::num_of(&a, "OR")?, k::num_of(&a, "lower")?, k::num_of(&a, "upper")?))
}

fn mh_forest(s: &Solved) -> Option<Figure> {
    let t = s.mat("tables")?;
    let lv = m::level(s);
    if t.is_empty() || t.len() > 8 || t.iter().any(|r| r.len() != 4) {
        return None;
    }
    let mut rows = vec![];
    let mut parts = vec![];
    for (i, r) in t.iter().enumerate() {
        let (o, lo, hi) = or_of(s, r, lv)?;
        parts.push(format!("stratum {} [{}] OR {} ({} to {})", i + 1, m::list_text(r, 4), tick(o), tick(lo), tick(hi)));
        rows.push(Row { label: format!("stratum {}", i + 1), est: o, lo, hi, hl: false });
    }
    let (p, pl, pu) = (s.out("OR_MH")?, s.out("lower")?, s.out("upper")?);
    rows.push(Row { label: "pooled (MH)".into(), est: p, lo: pl, hi: pu, hl: true });
    let (body, h) = m::interval_rows(&rows, "odds ratio (log)", &format!("pooled OR = {}", tick(p)), true, Some(1.0))?;
    Some(s.figure("forest", "Stratum odds ratios and their pooled estimate",
                  &format!("Odds ratio of each stratum given (the model's odds ratio with its {} interval) and the \
                            Mantel-Haenszel pooled odds ratio over the {} strata: {}. Highlighted: pooled OR_MH = {} ({} to {}).",
                           m::pct(lv), t.len(), parts.join("; "), tick(p), tick(pl), tick(pu)), &body, h))
}

fn or_forest(s: &Solved) -> Option<Figure> {
    let abcd: Vec<f64> = ["a", "b", "c", "d"].iter().map(|x| crate::figs_common::getv(s, x)).collect::<Option<Vec<_>>>()?;
    let lv = m::level(s);
    let mut rows = vec![];
    for l in [0.90, 0.95, 0.99] {
        let a = s.with_input(&[("level", fi(l))])?;
        rows.push(Row { label: format!("OR, {}", m::pct(l)), est: k::num_of(&a, "OR")?, lo: k::num_of(&a, "lower")?,
                        hi: k::num_of(&a, "upper")?, hl: (l - lv).abs() < 1e-9 });
    }
    for l in [0.90, 0.95, 0.99] {
        let a = s.solve_other("bmed_relative_risk_ci", &[("a", ii(abcd[0] as i64)), ("b", ii(abcd[1] as i64)),
                                                          ("c", ii(abcd[2] as i64)), ("d", ii(abcd[3] as i64)), ("level", fi(l))])?;
        rows.push(Row { label: format!("RR, {}", m::pct(l)), est: k::num_of(&a, "RR")?, lo: k::num_of(&a, "lower")?,
                        hi: k::num_of(&a, "upper")?, hl: false });
    }
    let (body, h) = m::interval_rows(&rows, "ratio (log)", &format!("OR = {}", tick(s.out("OR")?)), true, Some(1.0))?;
    Some(s.figure("forest", "Odds ratio and relative risk of the table at three confidence levels",
                  &format!("Forest plot of the 2 x 2 table given (a = {}, b = {}, c = {}, d = {}): the odds ratio (this \
                            capability) and the relative risk (the model's relative-risk capability) with their 90 %, 95 % \
                            and 99 % intervals, each a run of the model. An interval that crosses 1 (grey) is compatible \
                            with no effect. Highlighted: OR {} ({} to {}) at the level given.", tick(abcd[0]), tick(abcd[1]),
                           tick(abcd[2]), tick(abcd[3]), tick(s.out("OR")?), tick(s.out("lower")?), tick(s.out("upper")?)),
                  &body, h))
}

/// A table of counts: header row and column labels, cells (value, shaded).
fn count_table(rows: &[String], cols: &[String], cells: &[Vec<(f64, bool)>], corner: &str, notes: &[String]) -> (String, f64) {
    let nc = cols.len() as f64;
    let cw = (300.0 / nc).min(110.0);
    let x0 = 150.0 + (360.0 - cw * nc) / 2.0 - 30.0;
    let rh = 34.0;
    let y0 = 30.0;
    let mut out = String::new();
    out.push_str(&format!("<text class=\"axp-lt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 10.0, y0 + 22.0,
                          esc(&clip(corner, 18))));
    for (j, cname) in cols.iter().enumerate() {
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                              x0 + cw * (j as f64 + 0.5), y0 + 22.0, esc(&clip(cname, (cw / 7.6) as usize))));
    }
    for (i, rname) in rows.iter().enumerate() {
        let y = y0 + rh * (i as f64 + 1.0);
        out.push_str(&format!("<text class=\"axp-bt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"end\">{}</text>", x0 - 10.0,
                              y + 22.0, esc(&clip(rname, 18))));
        for (j, (v, sh)) in cells[i].iter().enumerate() {
            let x = x0 + cw * j as f64;
            out.push_str(&format!("<rect class=\"{}\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{rh}\"/>\
                                   <rect class=\"axp-f\" x=\"{x:.1}\" y=\"{y:.1}\" width=\"{:.1}\" height=\"{rh}\"/>\
                                   <text class=\"axp-vt\" x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"middle\">{}</text>",
                                  if *sh { "axp-s" } else { "axp-ka" }, cw, cw, x + cw / 2.0, y + 22.0, esc(&tick(*v))));
        }
    }
    let mut y = y0 + rh * (rows.len() as f64 + 1.0) + 26.0;
    for n in notes {
        out.push_str(&format!("<text class=\"axp-vt\" x=\"16\" y=\"{y:.1}\">{}</text>", esc(&clip(n, 62))));
        y += 20.0;
    }
    (out, y)
}

fn or_table(s: &Solved) -> Option<Figure> {
    let v: Vec<f64> = ["a", "b", "c", "d"].iter().map(|x| crate::figs_common::getv(s, x)).collect::<Option<Vec<_>>>()?;
    let (o, lo, hi) = (s.out("OR")?, s.out("lower")?, s.out("upper")?);
    let (body, h) = count_table(&["exposed".into(), "unexposed".into()], &["outcome".into(), "no outcome".into()],
                                &[vec![(v[0], true), (v[1], false)], vec![(v[2], false), (v[3], true)]], "",
                                &[format!("odds ratio ad/(bc) = {}", tick(o)), format!("{} interval {} to {}", m::pct(m::level(s)),
                                                                                       tick(lo), tick(hi))]);
    Some(s.figure("table", "2 x 2 table and its odds ratio",
                  &format!("The 2 x 2 table given (rows: exposed, unexposed; columns: with, without the outcome): a = {}, \
                            b = {}, c = {}, d = {}. Returned: odds ratio ad/(bc) = {}, {} interval {} to {}.", tick(v[0]),
                           tick(v[1]), tick(v[2]), tick(v[3]), tick(o), m::pct(m::level(s)), tick(lo), tick(hi)), &body, h.max(200.0)))
}

fn kappa_table(s: &Solved) -> Option<Figure> {
    let mx = s.mat("counts")?;
    let n = mx.len();
    if n < 2 || n > 6 || mx.iter().any(|r| r.len() != n) {
        return None;
    }
    let (kap, po) = (s.out("kappa")?, s.out("p_observed")?);
    let names: Vec<String> = (1..=n).map(|i| format!("category {i}")).collect();
    let cells: Vec<Vec<(f64, bool)>> = mx.iter().enumerate().map(|(i, r)| r.iter().enumerate().map(|(j, v)| (*v, i == j)).collect()).collect();
    let total: f64 = mx.iter().flatten().sum();
    let (body, h) = count_table(&names.iter().map(|x| format!("A: {x}")).collect::<Vec<_>>(),
                                &names.iter().map(|x| format!("B: {}", x.replace("category ", "cat. "))).collect::<Vec<_>>(),
                                &cells, "", &[format!("observed agreement = {}", tick(po)), format!("Cohen's kappa = {}", tick(kap))]);
    Some(s.figure("table", "Agreement table of two raters",
                  &format!("The agreement table given for two raters ({} subjects; rows: rater A, columns: rater B; \
                            agreements on the shaded diagonal). Returned: observed agreement {}, Cohen's kappa {} (agreement \
                            beyond chance).", tick(total), tick(po), tick(kap)), &body, h.max(200.0)))
}

// ------------------------------------------------------------------------------------------------ risks
fn risk(s: &Solved, nnt: bool) -> Option<Figure> {
    let g = |x: &str| crate::figs_common::getv(s, x);
    let (lab1, lab2, e1, n1, e2, n2) = if nnt {
        ("treated", "control", g("events_t")?, g("n_t")?, g("events_c")?, g("n_c")?)
    } else {
        let (a, b, c, d) = (g("a")?, g("b")?, g("c")?, g("d")?);
        ("exposed", "unexposed", a, a + b, c, c + d)
    };
    if !(n1 > 0.0 && n2 > 0.0) {
        return None;
    }
    let (r1, r2) = (e1 / n1, e2 / n2);
    let (est, lo, hi, name) = if nnt {
        (s.out("ARR")?, s.out("ARR_lower")?, s.out("ARR_upper")?, "absolute risk reduction")
    } else {
        (s.out("RR")?, s.out("lower")?, s.out("upper")?, "relative risk")
    };
    let mut out = String::new();
    let header = if nnt { format!("NNT = {}", tick(s.out("NNT")?)) } else { format!("RR = {}", tick(est)) };
    out.push_str(&format!("<text class=\"axp-mt\" x=\"16\" y=\"22\">{}</text>", esc(&header)));
    let xa = Axis::new(0.0, r1.max(r2).max(1e-9), false, 150.0, 430.0, false);
    out.push_str("<text class=\"axp-bt\" x=\"16\" y=\"52\">risk of the outcome (events / group size)</text>");
    for (i, (lab, e, n, r)) in [(lab1, e1, n1, r1), (lab2, e2, n2, r2)].iter().enumerate() {
        let y = 66.0 + 44.0 * i as f64;
        out.push_str(&format!("<text class=\"axp-lt\" x=\"140\" y=\"{:.1}\" text-anchor=\"end\">{lab} {}/{}</text>\
                               <rect class=\"{}\" x=\"150\" y=\"{y:.1}\" width=\"{:.1}\" height=\"26\"/>\
                               <text class=\"axp-vt\" x=\"{:.1}\" y=\"{:.1}\">{}</text>", y + 18.0, tick(*e), tick(*n),
                              ["axp-b", "axp-b4"][i], (xa.px(*r) - 150.0).max(1.0), xa.px(*r) + 6.0, y + 18.0, esc(&tick(*r))));
    }
    let row = vec![m::Row { label: if nnt { "ARR".into() } else { "RR".into() }, est, lo, hi, hl: true }];
    let (ib, ih) = m::interval_rows(&row, if nnt { "absolute risk reduction" } else { "relative risk (log)" }, "", !nnt,
                                    Some(if nnt { 0.0 } else { 1.0 }))?;
    out.push_str(&format!("<text class=\"axp-bt\" x=\"16\" y=\"176\">{} with its {} interval (returned)</text>", name,
                          m::pct(m::level(s))));
    let mut h = 160.0 + ih;
    if nnt {
        out.push_str(&format!("<text class=\"axp-vt\" x=\"16\" y=\"{:.1}\">number needed to treat 1/ARR = {} ({} to {})</text>",
                              h + 6.0, esc(&tick(s.out("NNT")?)), esc(&tick(s.out("NNT_lower")?)), esc(&tick(s.out("NNT_upper")?))));
        h += 24.0;
    }
    // the interval panel last (a translated group), so every text after it keeps its own position
    out.push_str(&format!("<g transform=\"translate(0,160)\">{ib}</g>"));
    let cap = if nnt {
        format!("Event risk in the treated ({} of {}) and control ({} of {}) groups given (risks derived from the inputs) and \
                 the model's absolute risk reduction ARR = {} with its {} interval {} to {}. Returned: number needed to treat \
                 NNT = 1/ARR = {} (interval {} to {}).", tick(e1), tick(n1), tick(e2), tick(n2), tick(est), m::pct(m::level(s)),
                tick(lo), tick(hi), tick(s.out("NNT")?), tick(s.out("NNT_lower")?), tick(s.out("NNT_upper")?))
    } else {
        format!("Risk of the outcome in the exposed ({} of {}) and unexposed ({} of {}) groups given (risks = events / \
                 group size, derived from the inputs), and the model's relative risk {} with its {} interval {} to {}.",
                tick(e1), tick(n1), tick(e2), tick(n2), tick(est), m::pct(m::level(s)), tick(lo), tick(hi))
    };
    Some(s.figure("risk", &if nnt { format!("Number needed to treat {} from two group risks", tick(s.out("NNT")?)) } else {
                      format!("Relative risk {}: two group risks compared", tick(est))
                  }, &cap, &out, h + 8.0))
}
