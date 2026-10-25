"""Biomedical science procedures: deterministic model calculations with typed inputs and several outputs (domain code,
loaded by the pack; enabled only once tools/certify_procedures.py has certified them).

QUANTITATIVE SCIENCE ONLY. These are textbook model calculations (compartment models, epidemic models, test statistics,
survival estimators, mechanical and electrical models of tissue and neurons, imaging physics). They are NOT medical
advice, give no dosing recommendation for any real patient and must not be used for clinical decisions.

Topics (the biomedical view): pharmacokinetic models (one-compartment IV bolus, infusion and oral absorption,
multiple dosing to steady state, loading and maintenance rates of a model, Michaelis-Menten elimination,
two-compartment models, hepatic clearance models, effect-site models, non-compartmental analysis of data), epidemic
models (SIR and SEIR final size, SIR peak prevalence and timing, SEIR early growth, reproduction number from a growth
rate, SIS prevalence, the Reed-Frost chain binomial), diagnostic test statistics (ROC area, Youden cut-off,
sensitivity and specificity with Wilson intervals, odds ratio, relative risk, number needed to treat, incidence rate
ratio, standardized mortality ratio, Mantel-Haenszel odds ratio, Cohen's kappa), survival analysis (Kaplan-Meier
estimate, median and interval, Nelson-Aalen hazard, log-rank test, Cox and exponential hazard ratios, restricted mean
survival time), biomechanics (static lever model of the elbow, Poiseuille flow in a vessel, Windkessel models,
viscoelastic tissue models, indicator dilution, respiratory mechanics), neuron models (Goldman-Hodgkin-Katz voltage
and current, leaky integrate-and-fire neuron, passive cable, Hodgkin-Huxley gating and resting potential,
postsynaptic potential, FitzHugh-Nagumo fixed point) and medical-imaging physics (X-ray attenuation through tissue
layers, MRI spin-echo, inversion-recovery and spoiled gradient-echo signals, ultrasound reflection and matching
layers, Gaussian point-spread function and MTF, diffusion and T2 fits).

Each procedure states its inputs, outputs, conditions, method, LaTeX, the documented question form it parses, worked
examples, an INDEPENDENT reference computation (used only by certification; the checking libraries are imported inside
the reference functions) and a deliberately wrong variant that the checks must reject. The shipped code (run, verify,
parse) is plain Python (math, re, statistics) plus mpmath where stated.

How the references differ from the shipped code:
- pharmacokinetics: ODE integration (scipy DOP853, rtol 1e-13) of the compartment models, with root refinement by
  Newton steps on fresh integrations for peak times; repeated dosing simulated dose after dose with the interval map
  obtained by ODE integration; steady states as the limit of that simulation; quadrature (scipy quad) of
  concentration curves and of dt/dC for Michaelis-Menten times; numpy eigenvalues and linear solves for
  two-compartment macro-constants; moment analysis (quadrature of t C) and the initial-slope identity for the
  micro-constants, confirmed by ODE integration; scipy trapezoid integration and statsmodels OLS for data;
- epidemics: ODE integration of SIR, SEIR and SIS to equilibrium or to the peak (event refined by Newton steps), numpy
  eigenvalues of the linearised SEIR system, quadrature of the gamma generation-interval Laplace transform, Ludwig's
  triangular system solved in exact rational arithmetic for the Reed-Frost distribution;
- diagnostic statistics: scikit-learn (roc_auc_score, roc_curve, recall_score, confusion_matrix, cohen_kappa_score),
  scipy mannwhitneyu, statsmodels (Table2x2, StratifiedTable, proportion_confint, confint_proportions_2indep,
  confint_poisson_2indep), scipy chi-square quantiles for the exact Poisson interval;
- survival: lifelines (KaplanMeierFitter, NelsonAalenFitter, logrank_test, CoxPHFitter, restricted_mean_survival_time),
  statsmodels Poisson GLM with a log-time offset for exponential rates and hazard ratios;
- biomechanics: numpy solve of the equilibrium equations with moments about a different point, quadrature of the
  Poiseuille velocity profile and a force balance for the wall stress, ODE integration of Windkessel and viscoelastic
  models, scipy.signal.freqs for impedances, scipy trapezoid and statsmodels OLS for data;
- neurons: scipy brentq on the summed GHK currents, Nernst-Planck constant-field shooting (ODE) for the GHK current,
  quadrature of dt/dV for integrate-and-fire intervals, ODE shooting for the cable equation, ODE integration of the
  gating equations and of the full Hodgkin-Huxley system to rest, numpy roots and eigenvalues for FitzHugh-Nagumo;
- imaging: ODE integration of the attenuation through layers, Bloch-equation simulation (matrix exponentials, scipy
  expm) of pulse sequences repeated to steady state, numpy solves of the acoustic boundary conditions, quadrature of the
  point-spread function's cosine transform, statsmodels OLS for the log-linear fits.

Conventions (fixed, so run and reference agree exactly):
- SI internally. Physical values are read with their units by the shared reader and arrive here in SI; concentrations
  of drugs are mass concentrations (kg/m^3; 1 mg/L = 1e-3 kg/m^3), doses are masses, clearances volume flow rates,
  rate constants per second, ion concentrations molar (mol/m^3 = mmol/L). Units the shared reader lacks are converted
  in the question reader and the conversion is stated: mM, uM, M (molar), Rayl and MRayl.
- Lists carry no unit: each list's unit is fixed in its input's meaning (times in h for pharmacokinetic data, mg/L,
  b-values in s/mm^2, echo times in ms ...) or results are given in the units of the list, as stated.
- Pharmacokinetics: linear one- and two-compartment models with first-order elimination unless Michaelis-Menten is
  named; instantaneous bolus doses; first-order absorption; constant-rate infusions; F is a fraction.
- Epidemics: populations as fractions of a closed population without births or deaths; frequency-dependent
  transmission; R0 = beta/gamma (SIR, SEIR); the final-size relation counts the removed fraction at the start as
  immune; the attack fraction is s0 - s_inf.
- Diagnostics: a score at or above the threshold is test-positive; higher scores mean disease; ties in the ROC area
  count one half; confidence levels are stated (never assumed); 2x2 tables are [exposed cases, exposed non-cases,
  unexposed cases, unexposed non-cases]; zero cells are refused (no continuity correction).
- Survival: times in one unit (results in that unit); events 1 = event, 0 = censored; a censoring at an event time is
  at risk at that time; the median is the first time with S(t) <= 0.5; the Kaplan-Meier interval is the log(-log)
  (exponential Greenwood) interval as lifelines computes it; Nelson-Aalen uses d/n; Cox ties by Efron's method.
- Neurons: R = 8.31446261815324 J/(mol K), F = 96485.3321233100 C/mol; outward current positive; Hodgkin-Huxley rate
  functions of the 1952 squid axon in the modern convention (rest near -65 mV, V in mV, rates in 1/ms, 6.3 degC).
- Imaging: normal incidence and lossless media for ultrasound; MRI sequences with ideal pulses and ideal spoiling;
  the simplified spin-echo model (refocusing assumed ideal, TE much shorter than TR).
- Results built from exp, log or roots are rounded to 12 significant digits (identical on every platform and Python
  version); values that are zero by construction are written as exactly 0.

Question form (one per procedure): the capability phrase, a colon, then `name = value` pairs separated by commas
(`IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h`). Physical values carry a unit; fractions may be
written with %; lists are written in brackets; choices are words. Any other number, an unknown name, a name given
twice or free prose is refused, never guessed. The reading guard (tools/ask_sweep.py, 2026-10-23):
  1. every stated number is bound: the question is split into 'name = value' pairs and any other text or number
     refuses the whole question;
  2. the capability phrase must open the question and be followed by the colon, so wording that names a different
     capability (SIR vs SEIR, bolus vs infusion, well-stirred vs parallel-tube, odds ratio vs relative risk) never
     reaches this reader;
  3. a convention is an input (confidence level, AUC method, cable end, Windkessel model): required, never assumed;
  4. physical values are passed to the runtime with their units, so a value is converted exactly once.
"""
from __future__ import annotations

import math
import re
import statistics as _stats

# ================================================================================================ constants
R_GAS = 8.31446261815324          # J/(mol K)  (exact: N_A k)
F_FARADAY = 96485.33212331001     # C/mol      (exact: N_A e)
LN2 = math.log(2.0)


# ================================================================================================ shared helpers
def _r(x):
    """12 significant digits, no negative zero (identical results on every platform and Python version)."""
    if isinstance(x, bool) or isinstance(x, str) or x is None:
        return x
    if isinstance(x, int):
        return x
    if isinstance(x, (list, tuple)):
        return [_r(v) for v in x]
    if x == 0 or not math.isfinite(x):
        return 0.0 if x == 0 else x
    return float(f"{x:.12g}") + 0.0


def _out(**kw):
    return {k: _r(v) for k, v in kw.items()}


def _rel(a, b):
    if isinstance(a, (list, tuple)) or isinstance(b, (list, tuple)):
        if not (isinstance(a, (list, tuple)) and isinstance(b, (list, tuple))) or len(a) != len(b):
            return math.inf
        return max([_rel(x, y) for x, y in zip(a, b)] + [0.0])
    if isinstance(a, (str, bool)) or isinstance(b, (str, bool)) or a is None or b is None:
        return 0.0 if a == b else math.inf
    try:
        a, b = float(a), float(b)
    except (TypeError, ValueError):
        return math.inf
    if not (math.isfinite(a) and math.isfinite(b)):
        return 0.0 if a == b else math.inf
    d = abs(a - b)
    if d == 0:
        return 0.0
    if abs(a) < 1e-15 and abs(b) < 1e-15:
        return 0.0
    return d / max(abs(a), abs(b))


def _recheck(fn):
    """A verifier: every output the procedure gives is recomputed and compared (relative); a missing output fails.
    The recomputation of one input set is remembered (the procedure is deterministic), so a verifier asked about the
    same inputs again (a perturbed or deliberately wrong output) does not repeat a long computation."""
    memo = {}

    def verify(inputs, outputs):
        key = repr(sorted(inputs.items()))
        if key not in memo:
            try:
                memo[key] = fn(**inputs)
            except Exception:
                memo[key] = None
            if len(memo) > 64:
                memo.pop(next(iter(memo)))
        want = memo[key]
        if want is None:
            return math.inf
        worst = 0.0
        for k, v in want.items():
            if k not in outputs:
                return math.inf
            worst = max(worst, _rel(outputs[k], v))
        return worst
    return verify


def _safe(fn):
    """Guard a wrong variant against exceptions (a variant that raises cannot be compared)."""
    def g(**kw):
        try:
            return fn(**kw)
        except Exception:
            return {"_failed": True}
    return g


def _need(ok, msg):
    if not ok:
        raise ValueError(msg)


def _fin(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v), f"{name} must be a finite number")


def _pos(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v > 0,
              f"{name} must be a positive number")


def _nonneg(name, *vals):
    for v in vals:
        _need(isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v) and v >= 0,
              f"{name} must be zero or positive")


def _frac(name, v, lo_open=False, hi_open=False):
    _fin(name, v)
    _need((v > 0 if lo_open else v >= 0) and (v < 1 if hi_open else v <= 1), f"{name} must be a fraction between 0 and 1")


def _level(level):
    _fin("the confidence level", level)
    _need(0.5 <= level < 1, "the confidence level must be a fraction such as 0.95 (or 95%)")
    return _stats.NormalDist().inv_cdf(0.5 + level / 2.0)


def _count(name, v, lo=0):
    _need(isinstance(v, int) and not isinstance(v, bool) and v >= lo, f"{name} must be a whole number of at least {lo}")
    return v


def _lu(rng, a, b):
    return math.exp(rng.uniform(math.log(a), math.log(b)))


def _sig(x, n=4):
    """x rounded to n significant digits (random inputs written as a user would write them)."""
    return float(f"{x:.{n}g}")


def _bisect(f, a, b, n=400):
    """Root of f on [a, b] by bisection (f(a), f(b) of opposite signs), to full double precision."""
    fa, fb = f(a), f(b)
    if fa == 0:
        return a
    if fb == 0:
        return b
    _need((fa > 0) != (fb > 0), "no root in the interval")
    for _ in range(n):
        m = 0.5 * (a + b)
        if m == a or m == b:
            break
        fm = f(m)
        if fm == 0:
            return m
        if (fm > 0) == (fa > 0):
            a, fa = m, fm
        else:
            b = m
    return 0.5 * (a + b)


def _solve_lin(A, b):
    """Gaussian elimination with partial pivoting; singular -> ValueError."""
    n = len(A)
    M = [list(map(float, row)) + [float(b[i])] for i, row in enumerate(A)]
    scale = max((abs(x) for row in A for x in row), default=1.0) or 1.0
    for c in range(n):
        p = max(range(c, n), key=lambda r: abs(M[r][c]))
        _need(abs(M[p][c]) > 1e-12 * scale, "the equations are singular (the data do not determine the parameters)")
        M[c], M[p] = M[p], M[c]
        for r in range(c + 1, n):
            f = M[r][c] / M[c][c]
            if f != 0:
                for k in range(c, n + 1):
                    M[r][k] -= f * M[c][k]
    x = [0.0] * n
    for r in reversed(range(n)):
        x[r] = (M[r][n] - sum(M[r][k] * x[k] for k in range(r + 1, n))) / M[r][r]
    return x


def _ols(X, y):
    """Ordinary least squares by the normal equations solved in exact-ish double arithmetic (columns of X given)."""
    p = len(X)
    A = [[math.fsum(X[i][k] * X[j][k] for k in range(len(y))) for j in range(p)] for i in range(p)]
    b = [math.fsum(X[i][k] * y[k] for k in range(len(y))) for i in range(p)]
    return _solve_lin(A, b)


def _loglin(t, y, name="the values"):
    """Least-squares line through (t, ln y): returns slope, intercept."""
    _need(len(t) == len(y), "give one value per time")
    _need(len(t) >= 2, "give at least two points")
    _need(all(v > 0 for v in y), f"{name} must be positive (a logarithm is taken)")
    _need(len(set(t)) >= 2, "the times must not all be equal")
    ly = [math.log(v) for v in y]
    n = len(t)
    mt, my = math.fsum(t) / n, math.fsum(ly) / n
    sxx = math.fsum((a - mt) ** 2 for a in t)
    sxy = math.fsum((a - mt) * (b - my) for a, b in zip(t, ly))
    slope = sxy / sxx
    return slope, my - slope * mt


# ------------------------------------------------------------------ reference helpers (scipy, used only by references)
def _ivp(fun, t_end, y0, rtol=1e-13, atol=None, t0=0.0, dense=False):
    """scipy DOP853 integration from t0 to t_end; returns the final state (or the solution object when dense)."""
    from scipy.integrate import solve_ivp
    if atol is None:
        atol = [1e-18 * max(1.0, abs(v)) for v in y0]
    if t_end == t0:
        return [float(v) for v in y0] if not dense else None
    sol = solve_ivp(fun, (t0, t_end), list(y0), method="DOP853", rtol=rtol, atol=atol, dense_output=dense)
    if not sol.success:
        raise RuntimeError(sol.message)
    return sol if dense else [float(v) for v in sol.y[:, -1]]


def _ivp_root(fun, y0, g, gdot, t_hi, atol=None, newton=3):
    """Time in (0, t_hi) where g(y(t)) = 0 for the ODE y' = fun(t, y), y(0) = y0: a root of the dense output (brentq),
    then Newton steps t -= g/gdot, each on a fresh integration from 0 (dense output alone is good to ~1e-9 only)."""
    from scipy.optimize import brentq
    sol = _ivp(fun, t_hi, y0, atol=atol, dense=True)
    t = brentq(lambda s: g(sol.sol(s)), 0.0 if g(y0) != 0 else 1e-12 * t_hi, t_hi, xtol=1e-14 * t_hi, rtol=8.9e-16)
    y = None
    for _ in range(newton):
        y = _ivp(fun, t, y0, atol=atol)
        dt = g(y) / gdot(t, y)
        t -= dt
        if abs(dt) <= 1e-15 * abs(t):
            break
    y = _ivp(fun, t, y0, atol=atol)
    return t, y


def _quad(f, a, b, **kw):
    from scipy.integrate import quad
    v, _ = quad(f, a, b, epsabs=0.0, epsrel=1e-13, limit=500, **kw)
    return float(v)


def _brentq(f, a, b):
    from scipy.optimize import brentq
    return float(brentq(f, a, b, xtol=1e-300, rtol=8.9e-16, maxiter=500))


# ================================================================================================ units
_KU = {"mass": "kg", "volume": "m^3", "time": "s", "first_order_rate_constant": "1/s", "flow_rate": "m^3/s",
       "mass_concentration": "kg/m^3", "mass_flow_rate": "kg/s", "concentration_time_integral": "kg*s/m^3",
       "pressure": "Pa", "length": "m", "dynamic_viscosity": "Pa*s", "speed": "m/s", "shear_stress": "Pa",
       "hydraulic_resistance": "Pa*s/m^3", "hydraulic_compliance": "m^3/Pa", "frequency": "Hz", "stress": "Pa",
       "elastic_modulus": "Pa", "force": "N", "angle": "rad", "torque": "N*m", "temperature": "K",
       "molar_concentration": "mol/m^3", "voltage": "V", "current": "A", "resistance": "ohm",
       "membrane_permeability": "m/s", "current_density": "A/m^2", "specific_membrane_resistance": "ohm*m^2",
       "resistivity": "ohm*m", "conductance_per_area": "S/m^2", "intensity": "W/m^2",
       "acoustic_impedance": "kg/(m^2*s)", "inverse_length": "1/m", "number": ""}

# spellings the shared unit reader lacks -> (factor to the SI unit, SI unit); written forms are converted here
_UCONV = {"mM": (1.0, "mol/m^3"), "uM": (1e-3, "mol/m^3"), "µM": (1e-3, "mol/m^3"), "μM": (1e-3, "mol/m^3"),
          "nM": (1e-6, "mol/m^3"), "M": (1000.0, "mol/m^3"), "Rayl": (1.0, "kg/(m^2*s)"),
          "rayl": (1.0, "kg/(m^2*s)"), "MRayl": (1e6, "kg/(m^2*s)"), "Mrayl": (1e6, "kg/(m^2*s)")}


def _written_unit(num: str, unit: str, kind: str):
    """'value unit' for the runtime, with the few units the shared reader lacks converted here (a bare unit only,
    e.g. '4 mM' or '1.5 MRayl'); None when it cannot be read."""
    u = unit.strip()
    if u in _UCONV:
        f, si = _UCONV[u]
        return f"{float(num) * f!r} {si}"
    atoms = re.findall(r"[A-Za-z°µμ]+", u)
    if any(a in _UCONV for a in atoms):
        return None                               # a lacking unit inside a compound unit is not read
    return f"{num} {u}"


# a small unit reader for the written examples in this file (the runtime reads questions with its own reader)
_ATOM_SI = {"g": 1e-3, "m": 1.0, "s": 1.0, "L": 1e-3, "mol": 1.0, "V": 1.0, "A": 1.0, "Pa": 1.0, "Hz": 1.0,
            "S": 1.0, "F": 1.0, "ohm": 1.0, "N": 1.0, "J": 1.0, "W": 1.0, "K": 1.0, "rad": 1.0}
_ATOM_FIXED = {"kg": 1.0, "min": 60.0, "h": 3600.0, "day": 86400.0, "mmHg": 133.322387415, "deg": math.pi / 180.0,
               "1": 1.0}
_PREF = {"k": 1e3, "M": 1e6, "G": 1e9, "m": 1e-3, "c": 1e-2, "u": 1e-6, "µ": 1e-6, "n": 1e-9, "p": 1e-12, "d": 1e-1}


def _atom_factor(a):
    if a in _ATOM_FIXED:
        return _ATOM_FIXED[a]
    if a in _ATOM_SI:
        return _ATOM_SI[a]
    if a in _UCONV:
        return _UCONV[a][0]
    if len(a) > 1 and a[0] in _PREF and a[1:] in _ATOM_SI:
        return _PREF[a[0]] * _ATOM_SI[a[1:]]
    raise ValueError(f"unit atom {a!r} not known to the example reader")


def _unit_factor(u):
    toks = re.findall(r"\^-?\d+|[*/()]|[A-Za-zµ]+|1", u.replace(" ", ""))
    pos = 0

    def factor():
        nonlocal pos
        t = toks[pos]
        pos += 1
        if t == "(":
            v = expr()
            pos += 1
        else:
            v = _atom_factor(t)
        if pos < len(toks) and toks[pos].startswith("^"):
            v = v ** int(toks[pos][1:])
            pos += 1
        return v

    def expr():
        nonlocal pos
        v = factor()
        while pos < len(toks) and toks[pos] in "*/":
            op = toks[pos]
            pos += 1
            w = factor()
            v = v * w if op == "*" else v / w
        return v
    return expr()


def _si(text):
    """SI value of a written example quantity ('500 mg' -> 5e-4); degC is converted to K."""
    m = re.fullmatch(r"\s*([-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?)\s*(.*?)\s*", text)
    v, u = float(m.group(1)), m.group(2)
    if u in ("degC", "°C"):
        return v + 273.15
    if u == "%":
        return v / 100.0
    return v * _unit_factor(u)


# ================================================================================================ the question reader
_NUM = r"[-+]?(?:\d+(?:\.\d*)?|\.\d+)(?:[eE][-+]?\d+)?"
_SEG = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(" + _NUM + r")\s*(.*?)\s*$")
_WORD = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*([A-Za-z][A-Za-z_0-9 \-]*?)\s*$")
_VEC = re.compile(r"^\s*(?:and\s+)?([A-Za-z][A-Za-z_0-9]*)\s*=\s*(\[[^\]]*\](?:\s*;\s*\[[^\]]*\])*|\[\[.*\]\])\s*$")
_PCT = {"%": 0.01, "percent": 0.01, "per cent": 0.01}


def _split(rest):
    """Split on commas outside brackets."""
    parts, depth, cur = [], 0, ""
    for ch in rest:
        if ch == "[":
            depth += 1
        elif ch == "]":
            depth -= 1
        if ch == "," and depth == 0:
            parts.append(cur)
            cur = ""
        else:
            cur += ch
    parts.append(cur)
    return parts


def _reader(phrases, inputs, aliases=None):
    """The one documented question form, '<phrase>: name = value, ...', for a procedure with these inputs.

    The reading guard (tools/ask_sweep.py, 2026-10-23: question-reader errors in finished models) is built in:
      1. every stated number is bound: the question is split into 'name = value' pairs and any other text or number
         refuses the whole question (a stray number can never be ignored);
      2. the capability phrase must open the question and be followed by the colon, so wording that names a different
         capability never reaches this reader; no name is shared by two meanings within one procedure;
      3. a convention is an input (confidence level, method, model, end condition): required, never assumed; a value
         without its unit (a physical quantity) or with a unit on a pure number is refused;
      4. physical values are passed to the runtime with their units (converted here only when the shared reader lacks
         the unit), so a value is converted exactly once.
    """
    rx = re.compile(r"\s*(?:" + "|".join(re.escape(p) for p in phrases) + r")\s*:\s*", re.I)
    names, by = {}, {}
    for i in inputs:
        names[i["symbol"]] = i["symbol"]
        by[i["symbol"]] = i
    lowers = {}
    for i in inputs:
        lowers.setdefault(i["symbol"].lower(), []).append(i["symbol"])
    for lo, syms in lowers.items():         # a lower-case spelling only where it names exactly one input
        if len(syms) == 1:
            names.setdefault(lo, syms[0])
    for a, s in (aliases or {}).items():
        names[a] = s
        names.setdefault(a.lower(), s)

    def parse(question):
        if not isinstance(question, str):
            return None
        m = rx.match(question)
        if not m:
            return None
        rest = re.sub(r"[.?!]\s*$", "", question[m.end():].strip()).strip()
        if not rest:
            return {}
        out = {}
        for seg in _split(rest):
            if not seg.strip():
                return None
            mv = _VEC.match(seg)
            if mv:
                key = names.get(mv.group(1), names.get(mv.group(1).lower()))
                if key is None or key in out or by[key].get("type") not in ("vector", "matrix"):
                    return None
                text = mv.group(2).strip()
                if by[key]["type"] == "matrix":
                    if text.startswith("[["):
                        text = text[1:-1]
                    rows = re.findall(r"\[([^\]]*)\]", text)
                    text = "; ".join(r.strip() for r in rows)
                out[key] = text
                continue
            mw = _WORD.match(seg)
            if mw:
                key = names.get(mw.group(1), names.get(mw.group(1).lower()))
                if key is not None and by[key].get("type") in ("choice", "boolean"):
                    if key in out:
                        return None
                    out[key] = mw.group(2).strip().lower().replace(" ", "_").replace("-", "_")
                    continue
                return None
            ms = _SEG.match(seg)
            if not ms:
                return None
            key = names.get(ms.group(1), names.get(ms.group(1).lower()))
            if key is None or key in out:
                return None
            spec = by[key]
            t = spec.get("type", "number")
            num, unit = ms.group(2), ms.group(3).strip()
            if t in ("vector", "matrix", "choice", "boolean", "text"):
                return None
            kind = spec.get("kind", "number")
            if t == "integer":
                if unit:
                    return None
                out[key] = num
                continue
            if kind != "number":
                if not unit:
                    return None
                w = _written_unit(num, unit, kind)
                if w is None:
                    return None
                out[key] = w
                continue
            if unit:
                if spec.get("pct") and unit.lower() in _PCT:
                    out[key] = float(num) * _PCT[unit.lower()]
                    continue
                return None
            if spec.get("pct") and abs(float(num)) > 1:
                return None                       # '20' for a fraction: 20 % or 20? refused; write 20% or 0.2
            out[key] = float(num)
        return out
    return parse


def _written(spec, v):
    """An input value as a user writes it in the documented form."""
    t = spec.get("type", "number")
    if t == "vector":
        return "[" + ", ".join(_fmt(x) for x in v) + "]"
    if t == "matrix":
        return "[" + ", ".join("[" + ", ".join(_fmt(x) for x in row) + "]" for row in v) + "]"
    if t in ("choice", "text"):
        return str(v)
    if t == "integer":
        return str(int(v))
    kind = spec.get("kind", "number")
    if kind != "number":
        return f"{_fmt(v)} {_KU[kind]}"
    return _fmt(v)


def _fmt(x):
    x = float(x)
    if x == int(x) and abs(x) < 1e15:
        return str(int(x))
    return repr(x)


# ================================================================================================ input/output specs
def _phys(sym, kind, meaning, required=True, **extra):
    _KU[kind]                                   # every physical kind has its SI unit here
    return {"symbol": sym, "kind": kind, "meaning": meaning, "required": required, **extra}


def _numin(sym, meaning, required=True, pct=False, **extra):
    d = {"symbol": sym, "kind": "number", "meaning": meaning, "required": required, **extra}
    if pct:
        d["pct"] = True
    return d


def _intin(sym, meaning, lo=None, hi=None, required=True):
    d = {"symbol": sym, "type": "integer", "meaning": meaning, "required": required}
    if lo is not None:
        d["min"] = lo
    if hi is not None:
        d["max"] = hi
    return d


def _vecin(sym, meaning, lo=1, hi=200, required=True):
    return {"symbol": sym, "type": "vector", "kind": "number", "meaning": meaning, "required": required,
            "min_length": lo, "max_length": hi}


def _matin(sym, meaning, required=True):
    return {"symbol": sym, "type": "matrix", "kind": "number", "meaning": meaning, "required": required}


def _choice(sym, choices, meaning, required=True):
    return {"symbol": sym, "type": "choice", "choices": list(choices), "meaning": meaning, "required": required}


def _o(sym, kind, meaning):
    return {"symbol": sym, "kind": kind, "meaning": meaning}


PROCEDURES: dict = {}
LEVELS: dict = {}


def _ex(written, outputs=None):
    """A worked example given as a user writes it: physical values as '500 mg' strings (converted to SI here for
    the example's inputs, kept as written for its question), lists as lists, choices as words."""
    inputs, shown = {}, {}
    for k, v in written.items():
        if isinstance(v, str) and re.match(r"\s*[-+]?[\d.]", v) and re.search(r"[A-Za-z%°µ]", v):
            inputs[k] = _si(v)
            shown[k] = v
        else:
            inputs[k] = v
    return {"inputs": inputs, "outputs": outputs or {}, "_shown": shown}


def _add(key, *, name, topic, purpose, inputs, outputs, primary, method, latex, run, reference, wrong, rand,
         phrases, examples, level, conditions=(), aliases=None, extra_q=(), verify=None):
    """Register a procedure: the documented question form is built from its phrases and inputs; templates and
    questions are written from the worked examples exactly as a user writes them."""
    parse = _reader(phrases, inputs, aliases)
    by = {i["symbol"]: i for i in inputs}

    def ask_text(ex, drop=None):
        shown = ex.get("_shown", {})
        return f"{phrases[0]}: " + ", ".join(f"{k} = {shown[k] if k in shown else _written(by[k], v)}"
                                             for k, v in ex["inputs"].items() if k != drop)
    templates = [ask_text(e) for e in examples[:2]]
    questions = [{"q": ask_text(e), "outputs": e["outputs"]} for e in examples]
    req = [i["symbol"] for i in inputs if i.get("required")]
    if req and examples:
        questions.append({"q": ask_text(examples[0], drop=req[0]), "outcome": "NEEDS_INPUT"})
        questions.append({"q": ask_text(examples[0]) + ", zz = 1", "outcome": "UNRECOGNIZED_FORMAT"})
    questions += [{"q": q["q"], "outputs": examples[q["like"]]["outputs"]} if "like" in q else q for q in extra_q]
    exs = [{"inputs": e["inputs"], "outputs": e["outputs"]} for e in examples]
    for e, src in zip(exs, examples):           # keep one outputs dict per example (filled from _EXV below)
        e["outputs"] = src["outputs"]
    PROCEDURES[key] = {
        "name": name, "topic": topic, "purpose": purpose, "inputs": inputs, "outputs": outputs, "primary": primary,
        "conditions": list(conditions), "method": method, "latex": latex, "run": run,
        "verify": verify or _recheck(run), "parse": parse, "reference": reference, "wrong": _safe(wrong),
        "random_inputs": rand, "templates": templates, "examples": exs, "questions": questions}
    LEVELS[key] = level


# ================================================================================================ PHARMACOKINETICS
_PK = "pharmacokinetic models"
_PKD = "pharmacokinetic data analysis"
_NOTE = " (a model calculation, not medical advice)"


def _rand_dose(rng):
    return _sig(_lu(rng, 1e-5, 2e-3))                  # 10 mg .. 2 g, in kg


def _rand_vol(rng):
    return _sig(_lu(rng, 5e-3, 0.3))                   # 5 .. 300 L, in m^3


def _rand_k(rng, lo=2e-6, hi=5e-4):
    return _sig(_lu(rng, lo, hi))                      # 1/s


# ---------------------------------------------------------------- one-compartment IV bolus
def pk_iv_bolus(D, V, k, t):
    _pos("the dose D, the volume V and the rate constant k", D, V, k)
    _nonneg("the time t", t)
    f = math.exp(-k * t)
    return _out(C=D / V * f, fraction_remaining=f)


def _ref_pk_iv_bolus(D, V, k, t):
    y = _ivp(lambda s, y: [-k * y[0]], t, [1.0])
    return {"C": D / V * y[0], "fraction_remaining": y[0]}


def _rand_pk_iv_bolus(rng):
    k = _rand_k(rng)
    return {"D": _rand_dose(rng), "V": _rand_vol(rng), "k": k, "t": _sig(rng.uniform(0.05, 5.0) / k)}


_add("bmed_pk_iv_bolus_concentration",
     name="One-compartment IV bolus: concentration at a time" + _NOTE, topic=_PK,
     purpose="The plasma concentration of a one-compartment model a time t after an intravenous bolus dose, with first-order elimination; also the fraction of the dose remaining.",
     inputs=[_phys("D", "mass", "bolus dose"), _phys("V", "volume", "volume of distribution"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"), _phys("t", "time", "time after the dose")],
     outputs=[_o("C", "mass_concentration", "concentration at time t"),
              _o("fraction_remaining", "number", "fraction of the dose still in the body")],
     primary="C", conditions=["one compartment, instantaneous mixing", "first-order elimination", "t >= 0"],
     method="C(t) = (D/V) exp(-k t)", latex=r"C(t) = \frac{D}{V}\,e^{-k t}",
     run=pk_iv_bolus, reference=_ref_pk_iv_bolus, rand=_rand_pk_iv_bolus,
     wrong=lambda D, V, k, t: _out(C=D / V * 10 ** (-k * t), fraction_remaining=10 ** (-k * t)),
     phrases=("IV bolus concentration", "One-compartment IV bolus concentration"),
     examples=[_ex({"D": "500 mg", "V": "40 L", "k": "0.1 1/h", "t": "6 h"}),
               _ex({"D": "1 g", "V": "50 L", "k": "0.25 1/h", "t": "90 min"})],
     level="beginner",
     extra_q=[{"q": "IV bolus concentration: D = 0.5 g, V = 40000 mL, k = 0.1 1/h, t = 360 min", "like": 0},
              {"q": "IV bolus concentration: D = 500 mg, V = 40 L, k = -0.1 1/h, t = 6 h", "outcome": "INVALID_INPUT"},
              {"q": "IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6", "outcome": "UNRECOGNIZED_FORMAT"}])


# ---------------------------------------------------------------- terminal half-life (log-linear fit)
def pk_half_life_fit(t, C):
    _need(len(t) == len(C) and len(t) >= 3, "give at least three times and as many concentrations")
    slope, b = _loglin(t, C, "the concentrations")
    _need(slope < 0, "the concentrations do not decline (the fitted slope of ln C is not negative)")
    k = -slope
    return _out(k=k, t_half=LN2 / k, C0=math.exp(b))


def _ref_pk_half_life_fit(t, C):
    import numpy as np
    import statsmodels.api as sm
    p = sm.OLS(np.log(np.array(C, dtype=float)), sm.add_constant(np.array(t, dtype=float))).fit().params
    k, C0 = -float(p[1]), math.exp(float(p[0]))
    th = _brentq(lambda s: C0 * math.exp(-k * s) - C0 / 2.0, 0.0, 100.0 / k)
    return {"k": k, "t_half": th, "C0": C0}


def _wrong_pk_half_life_fit(t, C):
    slope, b = _loglin(t, C)
    s10 = slope / math.log(10.0)                   # the slope of log10 C taken as the rate constant
    return _out(k=-s10, t_half=LN2 / -s10, C0=math.exp(b))


def _rand_pk_half_life_fit(rng):
    n = rng.randint(3, 10)
    k = _lu(rng, 0.05, 1.0)
    span = rng.uniform(1.5, 5.0) / k
    t0 = rng.uniform(0.0, 2.0)
    ts = sorted(round(t0 + rng.uniform(0, span), 2) for _ in range(n))
    C0 = _lu(rng, 0.5, 200.0)
    C = [_sig(C0 * math.exp(-k * s) * (1 + rng.uniform(-0.05, 0.05))) for s in ts]
    return {"t": ts, "C": C}


_add("bmed_pk_terminal_half_life_fit",
     name="Elimination rate constant and half-life from concentration data (log-linear fit)", topic=_PKD,
     purpose="The first-order elimination rate constant, half-life and back-extrapolated concentration from a straight-line least-squares fit of ln C against time (mono-exponential decline).",
     inputs=[_vecin("t", "sampling times, in one time unit (results are per that unit)", 3),
             _vecin("C", "concentrations at those times, in one concentration unit (C0 is in that unit)", 3)],
     outputs=[_o("k", "number", "elimination rate constant, per time unit of t"),
              _o("t_half", "number", "half-life, in the time unit of t"),
              _o("C0", "number", "back-extrapolated concentration at t = 0, in the unit of C")],
     primary="k", conditions=["mono-exponential decline over the sampled times", "all concentrations positive",
                              "unweighted least squares on ln C", "at least three points"],
     method="least-squares line ln C = ln C0 - k t; t_half = ln 2 / k", latex=r"\ln C = \ln C_0 - k t,\quad t_{1/2} = \frac{\ln 2}{k}",
     run=pk_half_life_fit, reference=_ref_pk_half_life_fit, wrong=_wrong_pk_half_life_fit, rand=_rand_pk_half_life_fit,
     phrases=("Terminal half-life fit", "Elimination half-life fit"),
     examples=[_ex({"t": [1, 2, 4, 6, 8, 12], "C": [8.2, 6.9, 4.8, 3.4, 2.4, 1.2]}),
               _ex({"t": [0.5, 1, 3, 5], "C": [95, 80, 41, 22]})],
     level="intermediate",
     extra_q=[{"q": "Terminal half-life fit: t = [1, 2, 4], C = [8.2, 9.1, 10.5]", "outcome": "INVALID_INPUT"},
              {"q": "Terminal half-life fit: t = [1, 2, 4, 6], C = [8.2, 6.9, 4.8, 3.4] mg/L", "outcome": "UNRECOGNIZED_FORMAT"}])


# ---------------------------------------------------------------- AUC of an IV bolus
def pk_auc_iv_bolus(D, CL, V, t):
    _pos("the dose D, the clearance CL and the volume V", D, CL, V)
    _nonneg("the time t", t)
    k = CL / V
    return _out(AUC_inf=D / CL, AUC_0_t=D / CL * -math.expm1(-k * t))


def _ref_pk_auc_iv_bolus(D, CL, V, t):
    k = CL / V
    f = lambda s, y: [-k * y[0], y[0]]
    y_t = _ivp(f, t, [1.0, 0.0])
    y_inf = _ivp(f, 60.0 / k, [1.0, 0.0])
    return {"AUC_inf": D / V * y_inf[1], "AUC_0_t": D / V * y_t[1]}


def _rand_pk_auc_iv_bolus(rng):
    V = _rand_vol(rng)
    k = _rand_k(rng)
    return {"D": _rand_dose(rng), "CL": _sig(k * V), "V": V, "t": _sig(rng.uniform(0.05, 5.0) / k)}


_add("bmed_pk_auc_iv_bolus",
     name="Area under the concentration curve of a one-compartment IV bolus" + _NOTE, topic=_PK,
     purpose="The area under the plasma concentration-time curve from 0 to infinity and from 0 to t after an IV bolus in a one-compartment model.",
     inputs=[_phys("D", "mass", "bolus dose"), _phys("CL", "flow_rate", "clearance"),
             _phys("V", "volume", "volume of distribution"), _phys("t", "time", "end of the partial area")],
     outputs=[_o("AUC_inf", "concentration_time_integral", "area from 0 to infinity"),
              _o("AUC_0_t", "concentration_time_integral", "area from 0 to t")],
     primary="AUC_inf", conditions=["one compartment", "first-order elimination k = CL/V"],
     method="AUC(0-inf) = D/CL; AUC(0-t) = (D/CL)(1 - exp(-CL t / V))",
     latex=r"AUC_{0-\infty} = \frac{D}{CL},\quad AUC_{0-t} = \frac{D}{CL}\left(1 - e^{-CL\,t/V}\right)",
     run=pk_auc_iv_bolus, reference=_ref_pk_auc_iv_bolus, rand=_rand_pk_auc_iv_bolus,
     wrong=lambda D, CL, V, t: _out(AUC_inf=D / V * LN2 * V / CL, AUC_0_t=D / V * LN2 * V / CL * -math.expm1(-CL / V * t)),
     phrases=("AUC of an IV bolus", "IV bolus AUC"),
     examples=[_ex({"D": "500 mg", "CL": "5 L/h", "V": "40 L", "t": "8 h"}),
               _ex({"D": "100 mg", "CL": "120 mL/min", "V": "30 L", "t": "1 day"})],
     level="beginner",
     extra_q=[{"q": "AUC of an IV bolus: D = 500 mg, CL = 0 L/h, V = 40 L, t = 8 h", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- AUC by the trapezoidal rule
def pk_auc_trapezoid(t, C, method):
    _need(len(t) == len(C) and len(t) >= 2, "give at least two times and as many concentrations")
    _need(all(b > a for a, b in zip(t, t[1:])), "the times must increase strictly")
    _need(all(c >= 0 for c in C), "concentrations cannot be negative")
    _need(method in ("linear", "linear_up_log_down"), "method is linear or linear_up_log_down")
    parts = []
    for (t1, c1), (t2, c2) in zip(zip(t, C), zip(t[1:], C[1:])):
        dt = t2 - t1
        if method == "linear_up_log_down" and 0 < c2 < c1:
            parts.append(dt * (c1 - c2) / math.log(c1 / c2))
        else:
            parts.append(dt * (c1 + c2) / 2.0)
    return _out(AUC=math.fsum(parts))


def _ref_pk_auc_trapezoid(t, C, method):
    from scipy.integrate import trapezoid
    if method == "linear":
        return {"AUC": float(trapezoid(C, t))}
    tot = []
    for (t1, c1), (t2, c2) in zip(zip(t, C), zip(t[1:], C[1:])):
        if 0 < c2 < c1:                            # quadrature of the exponential through the two points
            tot.append(_quad(lambda s: c1 * (c2 / c1) ** ((s - t1) / (t2 - t1)), t1, t2))
        else:
            tot.append(float(trapezoid([c1, c2], [t1, t2])))
    return {"AUC": math.fsum(tot)}


def _rand_pk_auc_trapezoid(rng):
    n = rng.randint(4, 12)
    ka, k = _lu(rng, 0.5, 3.0), _lu(rng, 0.05, 0.4)
    ts = [0.0] + sorted(set(round(rng.uniform(0.1, 24.0), 2) for _ in range(n - 1)))
    C = [_sig(10 * (math.exp(-k * s) - math.exp(-ka * s)) * (1 + rng.uniform(-0.03, 0.03))) for s in ts]
    return {"t": ts, "C": [max(0.0, c) for c in C], "method": rng.choice(["linear", "linear_up_log_down"])}


_add("bmed_pk_auc_trapezoid",
     name="AUC from concentration-time data by the trapezoidal rule", topic=_PKD,
     purpose="The area under a sampled concentration-time curve from the first to the last sample, by the linear trapezoidal rule or the linear-up/log-down rule (stated).",
     inputs=[_vecin("t", "sampling times, strictly increasing, in one time unit", 2),
             _vecin("C", "concentrations at those times, in one concentration unit", 2),
             _choice("method", ("linear", "linear_up_log_down"), "linear trapezoids, or linear while rising and logarithmic while falling")],
     outputs=[_o("AUC", "number", "area from the first to the last sample, in (unit of C) x (unit of t)")],
     primary="AUC", conditions=["times strictly increasing", "concentrations >= 0",
                                "log-down segments need both concentrations positive (otherwise linear)"],
     method="sum of segment areas: linear (C1 + C2) dt / 2; log-down (C1 - C2) dt / ln(C1/C2)",
     latex=r"AUC = \sum_i \frac{(C_i + C_{i+1})\,\Delta t_i}{2}\quad\text{or}\quad \frac{(C_i - C_{i+1})\,\Delta t_i}{\ln(C_i/C_{i+1})}",
     run=pk_auc_trapezoid, reference=_ref_pk_auc_trapezoid, rand=_rand_pk_auc_trapezoid,
     wrong=lambda t, C, method: _out(AUC=math.fsum(c1 * (t2 - t1) for t1, t2, c1 in zip(t, t[1:], C))),
     phrases=("AUC by the trapezoidal rule", "Trapezoidal AUC"),
     examples=[_ex({"t": [0, 0.5, 1, 2, 4, 6, 8, 12], "C": [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3.0, 1.5], "method": "linear"}),
               _ex({"t": [0, 0.5, 1, 2, 4, 6, 8, 12], "C": [0, 4.2, 6.8, 7.9, 6.1, 4.3, 3.0, 1.5], "method": "linear_up_log_down"})],
     level="beginner",
     extra_q=[{"q": "AUC by the trapezoidal rule: t = [0, 1, 2], C = [0, 4, 3]", "outcome": "NEEDS_INPUT"},
              {"q": "AUC by the trapezoidal rule: t = [0, 2, 1], C = [0, 4, 3], method = linear", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- non-compartmental analysis of an IV bolus profile
def pk_nca_iv(D, t, C, n_terminal):
    _pos("the dose D", D)
    n = len(t)
    _need(len(C) == n and n >= 4, "give at least four times and as many concentrations")
    _need(t[0] == 0, "the first sample must be at t = 0 (for a bolus, the measured or back-extrapolated C0)")
    _need(all(b > a for a, b in zip(t, t[1:])), "the times must increase strictly")
    _need(all(c >= 0 for c in C), "concentrations cannot be negative")
    _need(isinstance(n_terminal, int) and 3 <= n_terminal <= n - 1, "n_terminal (points used for lambda_z) is 3 to n - 1")
    tt, cc = t[-n_terminal:], C[-n_terminal:]
    slope, b = _loglin(tt, cc, "the terminal concentrations")
    _need(slope < 0, "the terminal concentrations do not decline")
    lz = -slope
    auc = math.fsum((t2 - t1) * (c1 + c2) / 2.0 for t1, t2, c1, c2 in zip(t, t[1:], C, C[1:]))
    auc_inf = auc + C[-1] / lz
    dmg = D * 1e6
    cl = dmg / auc_inf
    return _out(AUC_last=auc, lambda_z=lz, t_half=LN2 / lz, AUC_inf=auc_inf, CL=cl, Vz=cl / lz)


def _ref_pk_nca_iv(D, t, C, n_terminal):
    import numpy as np
    import statsmodels.api as sm
    from scipy.integrate import trapezoid
    auc = float(trapezoid(C, t))
    p = sm.OLS(np.log(np.array(C[-n_terminal:], dtype=float)),
               sm.add_constant(np.array(t[-n_terminal:], dtype=float))).fit().params
    lz, c0 = -float(p[1]), math.exp(float(p[0]))
    th = _brentq(lambda s: c0 * math.exp(-lz * s) - c0 / 2.0, 0.0, 100.0 / lz)
    auc_inf = auc + C[-1] / lz
    cl = D * 1e6 / auc_inf
    return {"AUC_last": auc, "lambda_z": lz, "t_half": th, "AUC_inf": auc_inf, "CL": cl, "Vz": cl / lz}


def _rand_pk_nca_iv(rng):
    a, al, b, be = _lu(rng, 1, 10), _lu(rng, 0.8, 3), _lu(rng, 0.5, 5), _lu(rng, 0.05, 0.3)
    ts = [0.0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24][: rng.randint(6, 10)]
    C = [_sig((a * math.exp(-al * s) + b * math.exp(-be * s)) * (1 + (rng.uniform(-0.03, 0.03) if s else 0))) for s in ts]
    return {"D": _sig(_lu(rng, 1e-5, 1e-3)), "t": ts, "C": C, "n_terminal": rng.randint(3, min(5, len(ts) - 1))}


_add("bmed_pk_nca_iv_bolus",
     name="Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz)", topic=_PKD,
     purpose="Non-compartmental parameters from a concentration-time profile after an IV bolus: AUC to the last sample (linear trapezoids), terminal rate constant from the last n points, half-life, AUC to infinity, clearance and terminal volume.",
     inputs=[_phys("D", "mass", "IV bolus dose"),
             _vecin("t", "sampling times in h, the first at 0 (measured or back-extrapolated C0)", 4),
             _vecin("C", "concentrations in mg/L at those times", 4),
             _intin("n_terminal", "number of last points used for the terminal slope (3 to n - 1)", 3, 50)],
     outputs=[_o("AUC_last", "number", "AUC from 0 to the last sample, mg*h/L"),
              _o("lambda_z", "number", "terminal rate constant, 1/h"), _o("t_half", "number", "terminal half-life, h"),
              _o("AUC_inf", "number", "AUC from 0 to infinity (observed C_last / lambda_z added), mg*h/L"),
              _o("CL", "number", "clearance D / AUC_inf, L/h"), _o("Vz", "number", "terminal volume CL / lambda_z, L")],
     primary="CL", conditions=["linear trapezoidal rule", "lambda_z by unweighted least squares on ln C of the last n points",
                               "extrapolation with the observed last concentration", "times in h, concentrations in mg/L"],
     method="AUC by trapezoids; ln C = a - lambda_z t on the last points; AUC_inf = AUC_last + C_last/lambda_z; CL = D/AUC_inf; Vz = CL/lambda_z",
     latex=r"AUC_{0-\infty} = AUC_{0-t_{last}} + \frac{C_{last}}{\lambda_z},\quad CL = \frac{D}{AUC_{0-\infty}},\quad V_z = \frac{CL}{\lambda_z}",
     run=pk_nca_iv, reference=_ref_pk_nca_iv, rand=_rand_pk_nca_iv,
     wrong=lambda D, t, C, n_terminal: (lambda r: _out(**dict(r, AUC_inf=r["AUC_last"] + C[-1] * r["lambda_z"])))(pk_nca_iv(D, t, C, n_terminal)),
     phrases=("Non-compartmental analysis of an IV bolus", "NCA of an IV bolus"),
     examples=[_ex({"D": "100 mg", "t": [0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24],
                    "C": [5.0, 4.0, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112], "n_terminal": 4}),
               _ex({"D": "250 mg", "t": [0, 1, 2, 4, 8, 12], "C": [12.5, 9.8, 7.9, 5.1, 2.2, 0.95], "n_terminal": 3})],
     level="advanced",
     extra_q=[{"q": "Non-compartmental analysis of an IV bolus: D = 100 mg, t = [1, 2, 4, 8, 12], C = [4, 3, 2, 1, 0.5], n_terminal = 3",
               "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- oral absorption (Bateman function)
def _ka_k(ka, k):
    _pos("the rate constants ka and k", ka, k)
    _need(abs(ka - k) > 1e-6 * max(ka, k), "ka equals k: the Bateman form does not apply (ka = k is not covered)")


def pk_oral(F, D, V, ka, k, t):
    _frac("the bioavailability F", F, lo_open=True)
    _pos("the dose D and the volume V", D, V)
    _ka_k(ka, k)
    _nonneg("the time t", t)
    a = F * D / V * ka / (ka - k)
    C = a * (math.exp(-k * t) - math.exp(-ka * t))
    auc = a * (-math.expm1(-k * t) / k + math.expm1(-ka * t) / ka)
    return _out(C=C, AUC_0_t=auc)


def _ref_pk_oral(F, D, V, ka, k, t):
    y = _ivp(lambda s, y: [-ka * y[0], ka * y[0] - k * y[1], y[1]], t, [F, 0.0, 0.0], atol=[1e-20, 1e-20, 1e-16])
    return {"C": D / V * y[1], "AUC_0_t": D / V * y[2]}


def _rand_pk_oral(rng):
    k = _rand_k(rng, 1e-5, 3e-4)
    ka = _sig(k * (_lu(rng, 1.5, 20.0) if rng.random() < 0.8 else _lu(rng, 0.05, 0.67)))
    return {"F": round(rng.uniform(0.2, 1.0), 2), "D": _rand_dose(rng), "V": _rand_vol(rng), "ka": ka, "k": k,
            "t": _sig(rng.uniform(0.05, 6.0) / k)}


_add("bmed_pk_oral_concentration",
     name="Oral dose, first-order absorption: concentration at a time (Bateman function)" + _NOTE, topic=_PK,
     purpose="The plasma concentration and partial AUC of a one-compartment model with first-order absorption (rate ka) and elimination (rate k) after a single oral dose with bioavailability F.",
     inputs=[_numin("F", "bioavailable fraction (0 < F <= 1; 80% or 0.8)", pct=True), _phys("D", "mass", "oral dose"),
             _phys("V", "volume", "volume of distribution"), _phys("ka", "first_order_rate_constant", "absorption rate constant"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"), _phys("t", "time", "time after the dose")],
     outputs=[_o("C", "mass_concentration", "concentration at time t"),
              _o("AUC_0_t", "concentration_time_integral", "area under the curve from 0 to t")],
     primary="C", conditions=["one compartment", "first-order absorption and elimination", "ka != k", "no lag time"],
     method="Bateman function C = F D ka / (V (ka - k)) (exp(-k t) - exp(-ka t)) and its integral",
     latex=r"C(t) = \frac{F D k_a}{V (k_a - k)}\left(e^{-k t} - e^{-k_a t}\right)",
     run=pk_oral, reference=_ref_pk_oral, rand=_rand_pk_oral,
     wrong=lambda F, D, V, ka, k, t: _out(C=F * D / V * k / (ka - k) * (math.exp(-k * t) - math.exp(-ka * t)), AUC_0_t=0.0),
     phrases=("Oral dose concentration", "Bateman concentration"),
     examples=[_ex({"F": 0.8, "D": "250 mg", "V": "35 L", "ka": "1.2 1/h", "k": "0.15 1/h", "t": "3 h"}),
               _ex({"F": "60%", "D": "50 mg", "V": "100 L", "ka": "0.05 1/h", "k": "0.2 1/h", "t": "12 h"})],
     level="beginner",
     extra_q=[{"q": "Oral dose concentration: F = 80, D = 250 mg, V = 35 L, ka = 1.2 1/h, k = 0.15 1/h, t = 3 h", "outcome": "UNRECOGNIZED_FORMAT"},
              {"q": "Oral dose concentration: F = 0.8, D = 250 mg, V = 35 L, ka = 0.15 1/h, k = 0.15 1/h, t = 3 h", "outcome": "INVALID_INPUT"}])


def pk_oral_peak(F, D, V, ka, k):
    _frac("the bioavailability F", F, lo_open=True)
    _pos("the dose D and the volume V", D, V)
    _ka_k(ka, k)
    tm = math.log(ka / k) / (ka - k)
    return _out(t_max=tm, C_max=F * D / V * math.exp(-k * tm))


def _ref_pk_oral_peak(F, D, V, ka, k):
    fun = lambda s, y: [-ka * y[0], ka * y[0] - k * y[1]]
    g = lambda y: ka * y[0] - k * y[1]
    gdot = lambda s, y: ka * (-ka * y[0]) - k * (ka * y[0] - k * y[1])
    tm, y = _ivp_root(fun, [F, 0.0], g, gdot, 40.0 / min(ka, k), atol=[1e-22, 1e-22])
    return {"t_max": tm, "C_max": D / V * y[1]}


def _rand_pk_oral_peak(rng):
    r = _rand_pk_oral(rng)
    r.pop("t")
    return r


_add("bmed_pk_oral_peak",
     name="Oral dose: time and height of the peak concentration (tmax, Cmax)" + _NOTE, topic=_PK,
     purpose="The time of the peak plasma concentration and the peak concentration of a one-compartment model with first-order absorption after a single oral dose.",
     inputs=[_numin("F", "bioavailable fraction (0 < F <= 1)", pct=True), _phys("D", "mass", "oral dose"),
             _phys("V", "volume", "volume of distribution"), _phys("ka", "first_order_rate_constant", "absorption rate constant"),
             _phys("k", "first_order_rate_constant", "elimination rate constant")],
     outputs=[_o("t_max", "time", "time of the peak"), _o("C_max", "mass_concentration", "peak concentration")],
     primary="t_max", conditions=["one compartment", "first-order absorption and elimination", "ka != k"],
     method="dC/dt = 0: t_max = ln(ka/k)/(ka - k); C_max = (F D / V) exp(-k t_max)",
     latex=r"t_{max} = \frac{\ln(k_a/k)}{k_a - k},\quad C_{max} = \frac{F D}{V} e^{-k t_{max}}",
     run=pk_oral_peak, reference=_ref_pk_oral_peak, rand=_rand_pk_oral_peak,
     wrong=lambda F, D, V, ka, k: (lambda tm: _out(t_max=tm, C_max=F * D / V * math.exp(-k * tm)))(math.log(ka / k) / (ka + k)),
     phrases=("Oral dose peak", "Oral tmax and Cmax"),
     examples=[_ex({"F": 0.8, "D": "250 mg", "V": "35 L", "ka": "1.2 1/h", "k": "0.15 1/h"}),
               _ex({"F": 1, "D": "500 mg", "V": "60 L", "ka": "0.9 1/h", "k": "0.087 1/h"})],
     level="beginner")


# ---------------------------------------------------------------- repeated IV bolus doses
def pk_nth_dose(D, V, k, tau, n, t):
    _pos("the dose D, the volume V, the rate constant k and the interval tau", D, V, k, tau)
    _need(isinstance(n, int) and 1 <= n <= 200, "n (the dose number) is a whole number from 1 to 200")
    _nonneg("the time t", t)
    _need(t <= tau, "t is the time after the nth dose and must lie within the dosing interval (t <= tau)")
    r = math.exp(-k * tau)
    acc = -math.expm1(-n * k * tau) / -math.expm1(-k * tau)
    cmax = D / V * acc
    return _out(C_max_n=cmax, C=cmax * math.exp(-k * t))


def _ref_pk_nth_dose(D, V, k, tau, n, t):
    f = _ivp(lambda s, y: [-k * y[0]], tau, [1.0])[0]       # the interval map, from an ODE integration
    A = 0.0
    for i in range(n):                                       # dose after dose
        A += 1.0
        if i < n - 1:
            A *= f
    g = _ivp(lambda s, y: [-k * y[0]], t, [1.0])[0]
    return {"C_max_n": D / V * A, "C": D / V * A * g}


def _rand_pk_nth_dose(rng):
    k = _rand_k(rng)
    tau = _sig(rng.uniform(0.1, 3.0) / k)
    return {"D": _rand_dose(rng), "V": _rand_vol(rng), "k": k, "tau": tau, "n": rng.randint(1, 30),
            "t": _sig(rng.uniform(0, 1) * tau)}


_add("bmed_pk_nth_dose_iv",
     name="Repeated IV bolus doses: concentration after the nth dose" + _NOTE, topic=_PK,
     purpose="The peak concentration just after the nth of equal IV bolus doses given every tau, and the concentration a time t later, in a one-compartment model (superposition).",
     inputs=[_phys("D", "mass", "each bolus dose"), _phys("V", "volume", "volume of distribution"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"), _phys("tau", "time", "dosing interval"),
             _intin("n", "number of the dose (1 = first)", 1, 200), _phys("t", "time", "time after the nth dose (0 to tau)")],
     outputs=[_o("C_max_n", "mass_concentration", "concentration just after the nth dose"),
              _o("C", "mass_concentration", "concentration a time t after the nth dose")],
     primary="C", conditions=["equal doses at equal intervals", "linear one-compartment model", "0 <= t <= tau"],
     method="superposition: C = (D/V) (1 - r^n)/(1 - r) exp(-k t), r = exp(-k tau)",
     latex=r"C_n(t) = \frac{D}{V}\,\frac{1 - e^{-n k \tau}}{1 - e^{-k \tau}}\,e^{-k t}",
     run=pk_nth_dose, reference=_ref_pk_nth_dose, rand=_rand_pk_nth_dose,
     wrong=lambda D, V, k, tau, n, t: (lambda c: _out(C_max_n=c, C=c * math.exp(-k * (t + tau))))(
         D / V * (1 - math.exp(-n * k * tau)) / (1 - math.exp(-k * tau))),
     phrases=("Concentration after the nth IV dose", "Nth dose concentration"),
     examples=[_ex({"D": "500 mg", "V": "40 L", "k": "0.1 1/h", "tau": "8 h", "n": 3, "t": "2 h"}),
               _ex({"D": "1 g", "V": "60 L", "k": "0.2 1/h", "tau": "12 h", "n": 1, "t": "0 h"})],
     level="intermediate",
     extra_q=[{"q": "Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 3, t = 9 h", "outcome": "INVALID_INPUT"},
              {"q": "Concentration after the nth IV dose: D = 500 mg, V = 40 L, k = 0.1 1/h, tau = 8 h, n = 2.5, t = 2 h", "outcome": "INVALID_INPUT"}])


def _ss_iterate(step, x0, tol=1e-17, nmax=200000):
    """Iterate a dose-to-dose map until it stops changing (the simulated steady state)."""
    x = x0
    for _ in range(nmax):
        y = step(x)
        if all(abs(a - b) <= tol * max(1.0, abs(a)) for a, b in zip(y, x)):
            return y
        x = y
    raise RuntimeError("the dose-to-dose simulation did not settle")


def pk_ss_iv(D, V, k, tau):
    _pos("the dose D, the volume V, the rate constant k and the interval tau", D, V, k, tau)
    r = math.exp(-k * tau)
    R = 1.0 / -math.expm1(-k * tau)
    cmax = D / V * R
    return _out(C_max_ss=cmax, C_min_ss=cmax * r, C_avg_ss=D / (V * k * tau), R_ac=R)


def _ref_pk_ss_iv(D, V, k, tau):
    f, g = _ivp(lambda s, y: [-k * y[0], y[0]], tau, [1.0, 0.0])
    pre = _ss_iterate(lambda x: [(x[0] + 1.0) * f], [0.0])[0]
    peak = pre + 1.0
    return {"C_max_ss": D / V * peak, "C_min_ss": D / V * peak * f, "C_avg_ss": D / V * peak * g / tau, "R_ac": peak}


def _rand_pk_ss_iv(rng):
    k = _rand_k(rng)
    return {"D": _rand_dose(rng), "V": _rand_vol(rng), "k": k, "tau": _sig(rng.uniform(0.1, 4.0) / k)}


_add("bmed_pk_steady_state_iv",
     name="Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor" + _NOTE, topic=_PK,
     purpose="The steady-state peak, trough and average concentrations and the accumulation factor of a one-compartment model given equal IV bolus doses every tau.",
     inputs=[_phys("D", "mass", "each bolus dose"), _phys("V", "volume", "volume of distribution"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"), _phys("tau", "time", "dosing interval")],
     outputs=[_o("C_max_ss", "mass_concentration", "steady-state peak (just after a dose)"),
              _o("C_min_ss", "mass_concentration", "steady-state trough (just before a dose)"),
              _o("C_avg_ss", "mass_concentration", "average concentration over an interval at steady state"),
              _o("R_ac", "number", "accumulation factor: steady-state peak over first-dose peak")],
     primary="C_max_ss", conditions=["equal doses at equal intervals", "linear one-compartment model"],
     method="geometric series: C_max,ss = (D/V)/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k tau); C_avg = D/(V k tau)",
     latex=r"C_{max,ss} = \frac{D/V}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k\tau},\quad \bar C_{ss} = \frac{D}{V k \tau}",
     run=pk_ss_iv, reference=_ref_pk_ss_iv, rand=_rand_pk_ss_iv,
     wrong=lambda D, V, k, tau: (lambda R: _out(C_max_ss=D / V * R, C_min_ss=D / V * R * math.exp(-k * tau), C_avg_ss=D / (V * k * tau), R_ac=R))(
         1.0 / (1.0 + math.exp(-k * tau))),
     phrases=("Steady state of repeated IV doses", "IV multiple-dose steady state"),
     examples=[_ex({"D": "500 mg", "V": "40 L", "k": "0.1 1/h", "tau": "8 h"}),
               _ex({"D": "250 mg", "V": "25 L", "k": "0.0578 1/h", "tau": "12 h"})],
     level="intermediate")


def pk_ss_oral(F, D, V, ka, k, tau):
    _frac("the bioavailability F", F, lo_open=True)
    _pos("the dose D, the volume V and the interval tau", D, V, tau)
    _ka_k(ka, k)
    a1, b1 = -math.expm1(-k * tau), -math.expm1(-ka * tau)
    pre = F * D * ka / (V * (ka - k))
    css = lambda s: pre * (math.exp(-k * s) / a1 - math.exp(-ka * s) / b1)
    tm = math.log(ka * a1 / (k * b1)) / (ka - k)
    _need(0 < tm < tau, "the steady-state peak does not fall inside the dosing interval")
    return _out(C_max_ss=css(tm), t_max_ss=tm, C_min_ss=css(tau))


def _ref_pk_ss_oral(F, D, V, ka, k, tau):
    fun = lambda s, y: [-ka * y[0], ka * y[0] - k * y[1]]
    at = [1e-22, 1e-22]
    c1, c2 = _ivp(fun, tau, [1.0, 0.0], atol=at), _ivp(fun, tau, [0.0, 1.0], atol=at)
    step = lambda x: [c1[0] * (x[0] + F) + c2[0] * x[1], c1[1] * (x[0] + F) + c2[1] * x[1]]
    pre = _ss_iterate(step, [0.0, 0.0])
    x0 = [pre[0] + F, pre[1]]
    g = lambda y: ka * y[0] - k * y[1]
    gdot = lambda s, y: ka * (-ka * y[0]) - k * (ka * y[0] - k * y[1])
    tm, y = _ivp_root(fun, x0, g, gdot, tau, atol=at)
    return {"C_max_ss": D / V * y[1], "t_max_ss": tm, "C_min_ss": D / V * pre[1]}


def _rand_pk_ss_oral(rng):
    k = _rand_k(rng, 1e-5, 3e-4)
    return {"F": round(rng.uniform(0.3, 1.0), 2), "D": _rand_dose(rng), "V": _rand_vol(rng),
            "ka": _sig(k * _lu(rng, 2.0, 15.0)), "k": k, "tau": _sig(rng.uniform(0.3, 3.0) / k)}


def _wrong_pk_ss_oral(F, D, V, ka, k, tau):
    r = pk_ss_oral(F, D, V, ka, k, tau)
    tm = math.log(ka / k) / (ka - k)                  # the single-dose peak time used at steady state
    return dict(r, t_max_ss=_r(tm))


_add("bmed_pk_steady_state_oral",
     name="Repeated oral dosing: steady-state peak, its time and the trough" + _NOTE, topic=_PK,
     purpose="The steady-state peak concentration, the time of the peak after a dose and the trough of a one-compartment model with first-order absorption given equal oral doses every tau.",
     inputs=[_numin("F", "bioavailable fraction (0 < F <= 1)", pct=True), _phys("D", "mass", "each oral dose"),
             _phys("V", "volume", "volume of distribution"), _phys("ka", "first_order_rate_constant", "absorption rate constant"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"), _phys("tau", "time", "dosing interval")],
     outputs=[_o("C_max_ss", "mass_concentration", "steady-state peak"), _o("t_max_ss", "time", "time of the peak after a dose"),
              _o("C_min_ss", "mass_concentration", "steady-state trough (at the dosing times)")],
     primary="C_max_ss", conditions=["equal doses at equal intervals", "linear one-compartment model, first-order absorption",
                                     "ka != k", "the steady-state peak lies inside the interval"],
     method="superposition to steady state: C_ss(t) = F D ka/(V(ka - k)) [exp(-k t)/(1 - exp(-k tau)) - exp(-ka t)/(1 - exp(-ka tau))]; dC/dt = 0 for the peak",
     latex=r"C_{ss}(t) = \frac{F D k_a}{V (k_a - k)}\left[\frac{e^{-k t}}{1 - e^{-k\tau}} - \frac{e^{-k_a t}}{1 - e^{-k_a \tau}}\right]",
     run=pk_ss_oral, reference=_ref_pk_ss_oral, wrong=_wrong_pk_ss_oral, rand=_rand_pk_ss_oral,
     phrases=("Steady state of repeated oral doses", "Oral multiple-dose steady state"),
     examples=[_ex({"F": 0.8, "D": "250 mg", "V": "35 L", "ka": "1.2 1/h", "k": "0.15 1/h", "tau": "8 h"}),
               _ex({"F": 0.5, "D": "100 mg", "V": "70 L", "ka": "2 1/h", "k": "0.1 1/h", "tau": "24 h"})],
     level="expert")


# ---------------------------------------------------------------- constant-rate infusion
def pk_infusion(R0, CL, V, t):
    _pos("the infusion rate R0, the clearance CL and the volume V", R0, CL, V)
    _nonneg("the time t", t)
    css = R0 / CL
    return _out(C=css * -math.expm1(-CL / V * t), C_ss=css)


def _ref_pk_infusion(R0, CL, V, t):
    k = CL / V
    f = lambda s, y: [1.0 - k * y[0]]                   # amount in units of R0 * 1 s
    y = _ivp(f, t, [0.0], atol=[1e-14])[0]
    yss = _ivp(f, 60.0 / k, [0.0], atol=[1e-14])[0]
    return {"C": R0 * y / V, "C_ss": R0 * yss / V}


def _rand_pk_infusion(rng):
    V = _rand_vol(rng)
    k = _rand_k(rng)
    return {"R0": _sig(_lu(rng, 1e-10, 1e-7)), "CL": _sig(k * V), "V": V, "t": _sig(rng.uniform(0.05, 5) / k)}


_add("bmed_pk_infusion_concentration",
     name="Constant-rate IV infusion: concentration at a time and steady state" + _NOTE, topic=_PK,
     purpose="The plasma concentration of a one-compartment model during a constant-rate intravenous infusion started at t = 0, and the steady-state concentration it approaches.",
     inputs=[_phys("R0", "mass_flow_rate", "infusion rate"), _phys("CL", "flow_rate", "clearance"),
             _phys("V", "volume", "volume of distribution"), _phys("t", "time", "time since the start of the infusion")],
     outputs=[_o("C", "mass_concentration", "concentration at time t"), _o("C_ss", "mass_concentration", "steady-state concentration R0/CL")],
     primary="C", conditions=["one compartment, first-order elimination", "infusion still running at t", "no drug at t = 0"],
     method="C = (R0/CL)(1 - exp(-CL t / V))", latex=r"C(t) = \frac{R_0}{CL}\left(1 - e^{-CL\,t/V}\right),\quad C_{ss} = \frac{R_0}{CL}",
     run=pk_infusion, reference=_ref_pk_infusion, rand=_rand_pk_infusion,
     wrong=lambda R0, CL, V, t: _out(C=R0 / V * t, C_ss=R0 / CL),
     phrases=("Infusion concentration", "Constant-rate infusion concentration"),
     examples=[_ex({"R0": "50 mg/h", "CL": "5 L/h", "V": "40 L", "t": "6 h"}),
               _ex({"R0": "2 mg/min", "CL": "100 mL/min", "V": "20 L", "t": "1 day"})],
     level="beginner")


def pk_short_infusion(R0, T_inf, CL, V, t):
    _pos("the infusion rate R0, the duration T_inf, the clearance CL and the volume V", R0, T_inf, CL, V)
    _nonneg("the time t", t)
    k = CL / V
    cend = R0 / CL * -math.expm1(-k * T_inf)
    C = R0 / CL * -math.expm1(-k * t) if t <= T_inf else cend * math.exp(-k * (t - T_inf))
    return _out(C=C, C_end=cend)


def _ref_pk_short_infusion(R0, T_inf, CL, V, t):
    k = CL / V
    on, off = (lambda s, y: [1.0 - k * y[0]]), (lambda s, y: [-k * y[0]])
    yend = _ivp(on, T_inf, [0.0], atol=[1e-14])[0]
    y = _ivp(on, t, [0.0], atol=[1e-14])[0] if t <= T_inf else _ivp(off, t - T_inf, [yend], atol=[1e-14])[0]
    return {"C": R0 * y / V, "C_end": R0 * yend / V}


def _rand_pk_short_infusion(rng):
    V, k = _rand_vol(rng), _rand_k(rng)
    T = _sig(rng.uniform(0.05, 2.0) / k)
    return {"R0": _sig(_lu(rng, 1e-10, 1e-7)), "T_inf": T, "CL": _sig(k * V), "V": V, "t": _sig(rng.uniform(0, 4) * T)}


_add("bmed_pk_short_infusion",
     name="Infusion of fixed duration: concentration during and after it" + _NOTE, topic=_PK,
     purpose="The plasma concentration of a one-compartment model at time t after the start of a constant-rate infusion lasting T_inf (during or after the infusion), and the concentration at the end of the infusion.",
     inputs=[_phys("R0", "mass_flow_rate", "infusion rate"), _phys("T_inf", "time", "duration of the infusion"),
             _phys("CL", "flow_rate", "clearance"), _phys("V", "volume", "volume of distribution"),
             _phys("t", "time", "time since the start of the infusion")],
     outputs=[_o("C", "mass_concentration", "concentration at time t"), _o("C_end", "mass_concentration", "concentration at the end of the infusion")],
     primary="C", conditions=["one compartment, first-order elimination", "single infusion, no drug before it"],
     method="during: (R0/CL)(1 - exp(-k t)); after: C_end exp(-k (t - T_inf)), k = CL/V",
     latex=r"C(t) = \begin{cases}\frac{R_0}{CL}(1 - e^{-kt}) & t \le T\\ \frac{R_0}{CL}(1 - e^{-kT})\,e^{-k(t - T)} & t > T\end{cases}",
     run=pk_short_infusion, reference=_ref_pk_short_infusion, rand=_rand_pk_short_infusion,
     wrong=lambda R0, T_inf, CL, V, t: (lambda k, ce: _out(C=R0 / CL * -math.expm1(-k * t) if t <= T_inf else ce * math.exp(-k * t), C_end=ce))(
         CL / V, R0 / CL * -math.expm1(-CL / V * T_inf)),
     phrases=("Short infusion concentration", "Fixed-duration infusion concentration"),
     examples=[_ex({"R0": "200 mg/h", "T_inf": "1 h", "CL": "6 L/h", "V": "30 L", "t": "4 h"}),
               _ex({"R0": "100 mg/h", "T_inf": "2 h", "CL": "4 L/h", "V": "50 L", "t": "1.5 h"})],
     level="intermediate")


def pk_loading_maintenance(C_target, V, CL):
    _pos("the target concentration, the volume V and the clearance CL", C_target, V, CL)
    return _out(LD=C_target * V, R0=C_target * CL)


def _ref_pk_loading_maintenance(C_target, V, CL):
    import numpy as np
    k = CL / V
    t1, t2 = 0.7 / k, 2.3 / k
    bol = lambda s, y: [-k * y[0]]
    inf = lambda s, y: [1.0 - k * y[0]]
    ub = [_ivp(bol, t, [1.0])[0] / V for t in (t1, t2)]               # concentration per unit bolus
    ui = [_ivp(inf, t, [0.0], atol=[1e-14])[0] / V for t in (t1, t2)]  # per unit infusion rate
    LD, R0 = np.linalg.solve(np.array([[ub[0], ui[0]], [ub[1], ui[1]]]), np.array([C_target, C_target]))
    return {"LD": float(LD), "R0": float(R0)}


_add("bmed_pk_loading_maintenance",
     name="Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation)", topic=_PK,
     purpose="For a one-compartment model, the bolus and the constant infusion rate that together hold the model concentration at a target value from the start (model calculation only, not a dosing recommendation).",
     inputs=[_phys("C_target", "mass_concentration", "target model concentration"), _phys("V", "volume", "volume of distribution"),
             _phys("CL", "flow_rate", "clearance")],
     outputs=[_o("LD", "mass", "loading bolus of the model"), _o("R0", "mass_flow_rate", "maintenance infusion rate of the model")],
     primary="LD", conditions=["one compartment, first-order elimination", "bolus and infusion start together"],
     method="LD = C V fills the volume; R0 = C CL replaces what is cleared",
     latex=r"LD = C_{target} V,\quad R_0 = C_{target}\,CL",
     run=pk_loading_maintenance, reference=_ref_pk_loading_maintenance,
     rand=lambda rng: {"C_target": _sig(_lu(rng, 1e-4, 0.05)), "V": _rand_vol(rng), "CL": _sig(_lu(rng, 1e-6, 1e-4))},
     wrong=lambda C_target, V, CL: _out(LD=C_target * V, R0=C_target * CL * LN2),
     phrases=("Loading dose and maintenance rate model", "Loading and maintenance model"),
     examples=[_ex({"C_target": "15 mg/L", "V": "50 L", "CL": "4 L/h"}),
               _ex({"C_target": "2 mg/L", "V": "350 L", "CL": "0.5 L/min"})],
     level="intermediate")


def pk_intermittent_infusion(R0, T_inf, tau, CL, V):
    _pos("R0, T_inf, tau, CL and V", R0, T_inf, tau, CL, V)
    _need(T_inf < tau, "the infusion must be shorter than the dosing interval")
    k = CL / V
    cmax = R0 / CL * -math.expm1(-k * T_inf) / -math.expm1(-k * tau)
    return _out(C_max_ss=cmax, C_min_ss=cmax * math.exp(-k * (tau - T_inf)))


def _ref_pk_intermittent_infusion(R0, T_inf, tau, CL, V):
    k = CL / V
    on, off = (lambda s, y: [1.0 - k * y[0]]), (lambda s, y: [-k * y[0]])
    a = _ivp(off, T_inf, [1.0])[0]
    b = _ivp(on, T_inf, [0.0], atol=[1e-14])[0]
    c = _ivp(off, tau - T_inf, [1.0])[0]
    pre = _ss_iterate(lambda x: [c * (a * x[0] + b)], [0.0])[0]
    return {"C_max_ss": R0 * (a * pre + b) / V, "C_min_ss": R0 * pre / V}


def _rand_pk_intermittent(rng):
    V, k = _rand_vol(rng), _rand_k(rng)
    tau = _sig(rng.uniform(0.2, 3.0) / k)
    return {"R0": _sig(_lu(rng, 1e-10, 1e-7)), "T_inf": _sig(tau * rng.uniform(0.05, 0.8)), "tau": tau, "CL": _sig(k * V), "V": V}


_add("bmed_pk_intermittent_infusion_steady_state",
     name="Repeated short infusions: steady-state peak and trough" + _NOTE, topic=_PK,
     purpose="The steady-state peak (end of infusion) and trough (end of interval) of a one-compartment model given infusions of duration T_inf at rate R0 every tau.",
     inputs=[_phys("R0", "mass_flow_rate", "infusion rate"), _phys("T_inf", "time", "duration of each infusion"),
             _phys("tau", "time", "dosing interval"), _phys("CL", "flow_rate", "clearance"), _phys("V", "volume", "volume of distribution")],
     outputs=[_o("C_max_ss", "mass_concentration", "steady-state concentration at the end of an infusion"),
              _o("C_min_ss", "mass_concentration", "steady-state concentration at the end of an interval")],
     primary="C_max_ss", conditions=["one compartment, first-order elimination", "T_inf < tau", "identical infusions"],
     method="C_max,ss = (R0/CL)(1 - exp(-k T))/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k(tau - T)), k = CL/V",
     latex=r"C_{max,ss} = \frac{R_0}{CL}\,\frac{1 - e^{-kT}}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k(\tau - T)}",
     run=pk_intermittent_infusion, reference=_ref_pk_intermittent_infusion, rand=_rand_pk_intermittent,
     wrong=lambda R0, T_inf, tau, CL, V: (lambda r: dict(r, C_min_ss=_r(r["C_max_ss"] * math.exp(-CL / V * tau))))(
         pk_intermittent_infusion(R0, T_inf, tau, CL, V)),
     phrases=("Intermittent infusion steady state", "Repeated short infusion steady state"),
     examples=[_ex({"R0": "1000 mg/h", "T_inf": "1 h", "tau": "8 h", "CL": "6 L/h", "V": "30 L"}),
               _ex({"R0": "500 mg/h", "T_inf": "30 min", "tau": "12 h", "CL": "5 L/h", "V": "20 L"})],
     level="advanced",
     extra_q=[{"q": "Intermittent infusion steady state: R0 = 1000 mg/h, T_inf = 9 h, tau = 8 h, CL = 6 L/h, V = 30 L", "outcome": "INVALID_INPUT"}])


def pk_time_to_fraction_ss(CL, V, f):
    _pos("the clearance CL and the volume V", CL, V)
    _frac("the fraction f", f, lo_open=True, hi_open=True)
    return _out(t=-math.log1p(-f) / (CL / V))


def _ref_pk_time_to_fraction_ss(CL, V, f):
    k = CL / V
    t, _ = _ivp_root(lambda s, y: [1.0 - k * y[0]], [0.0], lambda y: k * y[0] - f, lambda s, y: k * (1.0 - k * y[0]),
                     60.0 / k, atol=[1e-16])
    return {"t": t}


_add("bmed_pk_time_to_steady_state_fraction",
     name="Constant infusion: time to reach a fraction of steady state" + _NOTE, topic=_PK,
     purpose="The time after the start of a constant-rate infusion at which a one-compartment model reaches a stated fraction of its steady-state concentration.",
     inputs=[_phys("CL", "flow_rate", "clearance"), _phys("V", "volume", "volume of distribution"),
             _numin("f", "fraction of steady state (90% or 0.9)", pct=True)],
     outputs=[_o("t", "time", "time to reach f of steady state")],
     primary="t", conditions=["one compartment, first-order elimination", "no drug at the start", "0 < f < 1"],
     method="1 - exp(-k t) = f: t = -ln(1 - f)/k, k = CL/V", latex=r"t_f = -\frac{\ln(1 - f)}{CL/V}",
     run=pk_time_to_fraction_ss, reference=_ref_pk_time_to_fraction_ss,
     rand=lambda rng: {"CL": _sig(_lu(rng, 1e-6, 1e-4)), "V": _rand_vol(rng), "f": round(rng.uniform(0.05, 0.999), 3)},
     wrong=lambda CL, V, f: _out(t=-math.log(f) / (CL / V)),
     phrases=("Time to fraction of steady state", "Time to reach steady state fraction"),
     examples=[_ex({"CL": "5 L/h", "V": "40 L", "f": "90%"}), _ex({"CL": "2 L/h", "V": "100 L", "f": 0.5})],
     level="beginner",
     extra_q=[{"q": "Time to fraction of steady state: CL = 5 L/h, V = 40 L, f = 100%", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- Michaelis-Menten elimination
def _w_log(L):
    """w > 0 with w + ln w = L (the Lambert W function of exp(L)), by safeguarded Newton steps."""
    lo, hi = 1e-300, max(1.0, L) + 1.0
    w = math.exp(L) if L < 1 else L - math.log(L)
    w = min(max(w, lo), hi)
    for _ in range(200):
        h = w + math.log(w) - L
        if h > 0:
            hi = w
        else:
            lo = w
        w_new = w - h / (1.0 + 1.0 / w)
        if not (lo < w_new < hi):
            w_new = math.sqrt(lo * hi) if lo > 0 else 0.5 * hi
        if abs(w_new - w) <= 1e-16 * w:
            return w_new
        w = w_new
    return w


def pk_mm_concentration(C0, Vmax, Km, V, t):
    _pos("C0, Vmax, Km and V", C0, Vmax, Km, V)
    _nonneg("the time t", t)
    vm = Vmax / V
    L = math.log(C0 / Km) + (C0 - vm * t) / Km
    return _out(C=Km * _w_log(L))


def _ref_pk_mm_concentration(C0, Vmax, Km, V, t):
    vm = Vmax / V
    y = _ivp(lambda s, y: [-y[0] / (1.0 + y[0])], vm * t / Km, [C0 / Km], atol=[1e-300])
    return {"C": Km * y[0]}


def _rand_pk_mm(rng):
    Km = _sig(_lu(rng, 1e-3, 0.05))
    C0 = _sig(Km * _lu(rng, 0.1, 20.0))
    V = _rand_vol(rng)
    Vmax = _sig(_lu(rng, 1e-9, 1e-6))
    t = _sig(rng.uniform(0.05, 3.0) * C0 * V / Vmax)
    return {"C0": C0, "Vmax": Vmax, "Km": Km, "V": V, "t": t}


_add("bmed_pk_michaelis_menten_concentration",
     name="Michaelis-Menten (saturable) elimination: concentration at a time" + _NOTE, topic=_PK,
     purpose="The concentration a time t after an IV bolus in a one-compartment model with saturable Michaelis-Menten elimination, from the closed form with the Lambert W function.",
     inputs=[_phys("C0", "mass_concentration", "initial concentration"), _phys("Vmax", "mass_flow_rate", "maximum elimination rate (amount per time)"),
             _phys("Km", "mass_concentration", "Michaelis constant"), _phys("V", "volume", "volume of distribution"),
             _phys("t", "time", "time after the dose")],
     outputs=[_o("C", "mass_concentration", "concentration at time t")],
     primary="C", conditions=["one compartment", "elimination rate Vmax C/(Km + C)", "no other elimination"],
     method="V dC/dt = -Vmax C/(Km + C) integrates to C = Km W((C0/Km) exp((C0 - Vmax t/V)/Km)); W found from w + ln w = L by Newton steps",
     latex=r"C(t) = K_m\,W\!\left(\frac{C_0}{K_m}\exp\frac{C_0 - V_{max} t / V}{K_m}\right)",
     run=pk_mm_concentration, reference=_ref_pk_mm_concentration, rand=_rand_pk_mm,
     wrong=lambda C0, Vmax, Km, V, t: _out(C=C0 * math.exp(-Vmax / V * t / Km)),
     phrases=("Michaelis-Menten concentration", "Saturable elimination concentration"),
     examples=[_ex({"C0": "20 mg/L", "Vmax": "500 mg/day", "Km": "4 mg/L", "V": "50 L", "t": "2 day"}),
               _ex({"C0": "5 mg/L", "Vmax": "10 mg/h", "Km": "10 mg/L", "V": "20 L", "t": "8 h"})],
     level="expert")


def pk_mm_time(C0, C, Vmax, Km, V):
    _pos("C0, C, Vmax, Km and V", C0, C, Vmax, Km, V)
    _need(C < C0, "the target concentration must be below C0 (the concentration only falls)")
    return _out(t=V * ((C0 - C) + Km * math.log(C0 / C)) / Vmax)


def _ref_pk_mm_time(C0, C, Vmax, Km, V):
    return {"t": _quad(lambda c: V * (Km + c) / (Vmax * c), C, C0)}


def _rand_pk_mm_time(rng):
    r = _rand_pk_mm(rng)
    r.pop("t")
    r["C"] = _sig(r["C0"] * rng.uniform(0.01, 0.9))
    return r


_add("bmed_pk_michaelis_menten_time",
     name="Michaelis-Menten elimination: time to fall to a concentration" + _NOTE, topic=_PK,
     purpose="The time for the concentration of a one-compartment model with Michaelis-Menten elimination to fall from C0 to C.",
     inputs=[_phys("C0", "mass_concentration", "initial concentration"), _phys("C", "mass_concentration", "target concentration (< C0)"),
             _phys("Vmax", "mass_flow_rate", "maximum elimination rate"), _phys("Km", "mass_concentration", "Michaelis constant"),
             _phys("V", "volume", "volume of distribution")],
     outputs=[_o("t", "time", "time to fall from C0 to C")],
     primary="t", conditions=["one compartment", "Michaelis-Menten elimination only", "C < C0"],
     method="t = V [(C0 - C) + Km ln(C0/C)] / Vmax", latex=r"t = \frac{V}{V_{max}}\left[(C_0 - C) + K_m \ln\frac{C_0}{C}\right]",
     run=pk_mm_time, reference=_ref_pk_mm_time, rand=_rand_pk_mm_time,
     wrong=lambda C0, C, Vmax, Km, V: _out(t=V * (C0 - C) / Vmax),
     phrases=("Michaelis-Menten time to concentration", "Saturable elimination time"),
     examples=[_ex({"C0": "20 mg/L", "C": "10 mg/L", "Vmax": "500 mg/day", "Km": "4 mg/L", "V": "50 L"}),
               _ex({"C0": "30 mg/L", "C": "3 mg/L", "Vmax": "20 mg/h", "Km": "5 mg/L", "V": "40 L"})],
     level="advanced",
     extra_q=[{"q": "Michaelis-Menten time to concentration: C0 = 10 mg/L, C = 20 mg/L, Vmax = 500 mg/day, Km = 4 mg/L, V = 50 L", "outcome": "INVALID_INPUT"}])


def pk_mm_steady_state(R0, Vmax, Km):
    _pos("R0, Vmax and Km", R0, Vmax, Km)
    _need(R0 < Vmax, "no steady state: the dosing rate must be below Vmax")
    return _out(C_ss=Km * R0 / (Vmax - R0))


def _ref_pk_mm_steady_state(R0, Vmax, Km):
    rho = R0 / Vmax
    f = lambda s, y: [rho - y[0] / (1.0 + y[0])]       # c = C/Km, time in units of V Km / Vmax
    y = _ivp(f, 8000.0, [0.0], atol=[1e-18])            # relaxation rate >= (1 - 0.9)^2 = 0.01 in the random range
    return {"C_ss": Km * y[0]}


_add("bmed_pk_michaelis_menten_steady_state",
     name="Michaelis-Menten elimination: steady-state concentration under a constant dosing rate" + _NOTE, topic=_PK,
     purpose="The steady-state concentration of a one-compartment model with Michaelis-Menten elimination given a constant dosing rate below Vmax.",
     inputs=[_phys("R0", "mass_flow_rate", "constant dosing rate (amount per time)"), _phys("Vmax", "mass_flow_rate", "maximum elimination rate"),
             _phys("Km", "mass_concentration", "Michaelis constant")],
     outputs=[_o("C_ss", "mass_concentration", "steady-state concentration")],
     primary="C_ss", conditions=["Michaelis-Menten elimination only", "R0 < Vmax (otherwise no steady state)"],
     method="R0 = Vmax C/(Km + C): C_ss = Km R0/(Vmax - R0)", latex=r"C_{ss} = \frac{K_m R_0}{V_{max} - R_0}",
     run=pk_mm_steady_state, reference=_ref_pk_mm_steady_state,
     rand=lambda rng: (lambda vm: {"R0": _sig(vm * rng.uniform(0.05, 0.9)), "Vmax": vm, "Km": _sig(_lu(rng, 1e-3, 0.05))})(_sig(_lu(rng, 1e-9, 1e-6))),
     wrong=lambda R0, Vmax, Km: _out(C_ss=Km * R0 / Vmax),
     phrases=("Michaelis-Menten steady state", "Saturable elimination steady state"),
     examples=[_ex({"R0": "300 mg/day", "Vmax": "500 mg/day", "Km": "4 mg/L"}),
               _ex({"R0": "15 mg/h", "Vmax": "20 mg/h", "Km": "5 mg/L"})],
     level="intermediate",
     extra_q=[{"q": "Michaelis-Menten steady state: R0 = 600 mg/day, Vmax = 500 mg/day, Km = 4 mg/L", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- two-compartment model
def _macro(k10, k12, k21):
    s = k10 + k12 + k21
    disc = math.sqrt(max(0.0, s * s - 4.0 * k10 * k21))
    alpha = (s + disc) / 2.0
    return alpha, k10 * k21 / alpha


def pk_two_comp(D, V1, k10, k12, k21, t):
    _pos("D, V1, k10, k12 and k21", D, V1, k10, k12, k21)
    _nonneg("the time t", t)
    al, be = _macro(k10, k12, k21)
    _need(al - be > 1e-9 * al, "the two exponents coincide (degenerate model)")
    c0 = D / V1
    A = c0 * (al - k21) / (al - be)
    B = c0 * (k21 - be) / (al - be)
    return _out(alpha=al, beta=be, A=A, B=B, C=A * math.exp(-al * t) + B * math.exp(-be * t))


def _ref_pk_two_comp(D, V1, k10, k12, k21, t):
    import numpy as np
    K = np.array([[-(k10 + k12), k21], [k12, -k21]])
    ev = sorted((-float(np.real(v)) for v in np.linalg.eigvals(K)), reverse=True)
    al, be = ev
    c0 = D / V1
    A, B = np.linalg.solve(np.array([[1.0, 1.0], [al, be]]), np.array([c0, (k10 + k12) * c0]))
    y = _ivp(lambda s, y: [-(k10 + k12) * y[0] + k21 * y[1], k12 * y[0] - k21 * y[1]], t, [1.0, 0.0], atol=[1e-20, 1e-20])
    return {"alpha": al, "beta": be, "A": float(A), "B": float(B), "C": c0 * y[0]}


def _rand_pk_two_comp(rng):
    k10, k12, k21 = (_sig(_lu(rng, 1e-5, 1e-3)) for _ in range(3))
    al, be = _macro(k10, k12, k21)
    return {"D": _rand_dose(rng), "V1": _rand_vol(rng), "k10": k10, "k12": k12, "k21": k21, "t": _sig(rng.uniform(0.05, 4) / be)}


def _wrong_pk_two_comp(D, V1, k10, k12, k21, t):
    s = k10 + k12 + k21
    disc = math.sqrt(max(0.0, s * s - 4.0 * k10 * k12))
    al = (s + disc) / 2.0
    be = (s - disc) / 2.0
    c0 = D / V1
    A, B = c0 * (al - k21) / (al - be), c0 * (k21 - be) / (al - be)
    return _out(alpha=al, beta=be, A=A, B=B, C=A * math.exp(-al * t) + B * math.exp(-be * t))


_add("bmed_pk_two_compartment_bolus",
     name="Two-compartment IV bolus: macro-constants and concentration from micro-constants" + _NOTE, topic=_PK,
     purpose="The hybrid rate constants alpha and beta, the coefficients A and B and the central concentration at time t of a two-compartment model after an IV bolus, from the volume V1 and the micro-constants k10, k12, k21.",
     inputs=[_phys("D", "mass", "bolus dose"), _phys("V1", "volume", "central volume"),
             _phys("k10", "first_order_rate_constant", "elimination rate constant from the central compartment"),
             _phys("k12", "first_order_rate_constant", "transfer rate constant central to peripheral"),
             _phys("k21", "first_order_rate_constant", "transfer rate constant peripheral to central"),
             _phys("t", "time", "time after the dose")],
     outputs=[_o("alpha", "first_order_rate_constant", "fast (distribution) rate constant"),
              _o("beta", "first_order_rate_constant", "slow (terminal) rate constant"),
              _o("A", "mass_concentration", "coefficient of exp(-alpha t)"), _o("B", "mass_concentration", "coefficient of exp(-beta t)"),
              _o("C", "mass_concentration", "central concentration at time t")],
     primary="C", conditions=["linear two-compartment model, elimination from the central compartment", "alpha > beta"],
     method="alpha, beta roots of x^2 - (k10 + k12 + k21) x + k10 k21 = 0; A = (D/V1)(alpha - k21)/(alpha - beta), B = (D/V1)(k21 - beta)/(alpha - beta)",
     latex=r"C(t) = A e^{-\alpha t} + B e^{-\beta t},\quad \alpha\beta = k_{10}k_{21},\ \alpha + \beta = k_{10} + k_{12} + k_{21}",
     run=pk_two_comp, reference=_ref_pk_two_comp, wrong=_wrong_pk_two_comp, rand=_rand_pk_two_comp,
     phrases=("Two-compartment bolus", "Two-compartment IV bolus"),
     examples=[_ex({"D": "100 mg", "V1": "10 L", "k10": "0.2 1/h", "k12": "0.5 1/h", "k21": "0.3 1/h", "t": "4 h"}),
               _ex({"D": "500 mg", "V1": "20 L", "k10": "0.1 1/h", "k12": "1.2 1/h", "k21": "0.4 1/h", "t": "30 min"})],
     level="advanced")


def pk_micro_from_macro(D, A, alpha, B, beta):
    _pos("D, A, alpha, B and beta", D, A, alpha, B, beta)
    _need(alpha > beta * (1 + 1e-9), "alpha must exceed beta")
    V1 = D / (A + B)
    k21 = (A * beta + B * alpha) / (A + B)
    k10 = alpha * beta / k21
    k12 = alpha + beta - k21 - k10
    return _out(V1=V1, k10=k10, k12=k12, k21=k21, CL=k10 * V1, Vss=V1 * (1.0 + k12 / k21))


def _ref_pk_micro_from_macro(D, A, alpha, B, beta):
    curve = lambda s: A * math.exp(-alpha * s) + B * math.exp(-beta * s)
    auc = _quad(lambda u: curve(u / beta), 0.0, math.inf) / beta            # area, by quadrature
    aumc = _quad(lambda u: (u / beta) * curve(u / beta), 0.0, math.inf) / beta
    V1 = D / curve(0.0)
    k10 = D / (V1 * auc)                                                     # CL = D/AUC = k10 V1
    sum_out = (A * alpha + B * beta) / (A + B)                               # -C'(0)/C(0) = k10 + k12
    k12 = sum_out - k10
    k21 = alpha + beta - sum_out                                             # trace of the rate matrix
    cl = D / auc
    vss = cl * aumc / auc                                                    # moment analysis: CL x MRT
    for s in (0.3 / alpha, 1.0 / beta, 3.0 / beta):                          # ODE confirmation of the model
        y = _ivp(lambda q, y: [-(k10 + k12) * y[0] + k21 * y[1], k12 * y[0] - k21 * y[1]], s, [1.0, 0.0], atol=[1e-20, 1e-20])
        if abs(D / V1 * y[0] - curve(s)) > 1e-10 * curve(s):
            raise RuntimeError("micro-constants do not reproduce the curve")
    return {"V1": V1, "k10": k10, "k12": k12, "k21": k21, "CL": cl, "Vss": vss}


def _rand_pk_micro(rng):
    be = _sig(_lu(rng, 1e-5, 2e-4))
    return {"D": _rand_dose(rng), "A": _sig(_lu(rng, 1e-4, 0.05)), "alpha": _sig(be * _lu(rng, 3, 30)), "B": _sig(_lu(rng, 1e-4, 0.05)), "beta": be}


def _wrong_pk_micro(D, A, alpha, B, beta):
    V1 = D / (A + B)
    k21 = (A * alpha + B * beta) / (A + B)
    k10 = alpha * beta / k21
    k12 = alpha + beta - k21 - k10
    return _out(V1=V1, k10=k10, k12=k12, k21=k21, CL=k10 * V1, Vss=V1 * (1.0 + k12 / k21))


_add("bmed_pk_two_compartment_micro_constants",
     name="Two-compartment model: micro-constants, clearance and Vss from a biexponential curve" + _NOTE, topic=_PK,
     purpose="The central volume, the micro rate constants k10, k12, k21, the clearance and the steady-state volume of a two-compartment model from the biexponential fit C = A exp(-alpha t) + B exp(-beta t) of an IV bolus.",
     inputs=[_phys("D", "mass", "bolus dose"), _phys("A", "mass_concentration", "coefficient of the fast exponential"),
             _phys("alpha", "first_order_rate_constant", "fast rate constant"), _phys("B", "mass_concentration", "coefficient of the slow exponential"),
             _phys("beta", "first_order_rate_constant", "slow rate constant")],
     outputs=[_o("V1", "volume", "central volume"), _o("k10", "first_order_rate_constant", "elimination rate constant"),
              _o("k12", "first_order_rate_constant", "central to peripheral"), _o("k21", "first_order_rate_constant", "peripheral to central"),
              _o("CL", "flow_rate", "clearance"), _o("Vss", "volume", "volume of distribution at steady state")],
     primary="k21", conditions=["IV bolus, linear two-compartment model, elimination from the central compartment",
                                "alpha > beta > 0, A > 0, B > 0"],
     method="V1 = D/(A + B); k21 = (A beta + B alpha)/(A + B); k10 = alpha beta / k21; k12 = alpha + beta - k21 - k10; CL = k10 V1; Vss = V1 (1 + k12/k21)",
     latex=r"k_{21} = \frac{A\beta + B\alpha}{A + B},\quad k_{10} = \frac{\alpha\beta}{k_{21}},\quad k_{12} = \alpha + \beta - k_{21} - k_{10}",
     run=pk_micro_from_macro, reference=_ref_pk_micro_from_macro, wrong=_wrong_pk_micro, rand=_rand_pk_micro,
     phrases=("Two-compartment micro-constants", "Two-compartment micro constants"),
     examples=[_ex({"D": "100 mg", "A": "6 mg/L", "alpha": "1.5 1/h", "B": "2 mg/L", "beta": "0.15 1/h"}),
               _ex({"D": "500 mg", "A": "20 mg/L", "alpha": "3 1/h", "B": "5 mg/L", "beta": "0.2 1/h"})],
     level="expert",
     extra_q=[{"q": "Two-compartment micro-constants: D = 100 mg, A = 6 mg/L, alpha = 0.1 1/h, B = 2 mg/L, beta = 0.15 1/h", "outcome": "INVALID_INPUT"}])


# ---------------------------------------------------------------- bioavailability from two profiles
def _trap(t, C, label):
    _need(len(t) == len(C) and len(t) >= 2, f"{label}: give at least two times and as many concentrations")
    _need(all(b > a for a, b in zip(t, t[1:])), f"{label}: the times must increase strictly")
    _need(all(c >= 0 for c in C), f"{label}: concentrations cannot be negative")
    return math.fsum((t2 - t1) * (c1 + c2) / 2.0 for t1, t2, c1, c2 in zip(t, t[1:], C, C[1:]))


def pk_bioavailability(D_iv, t_iv, C_iv, D_oral, t_oral, C_oral):
    _pos("the doses", D_iv, D_oral)
    a_iv, a_or = _trap(t_iv, C_iv, "IV profile"), _trap(t_oral, C_oral, "oral profile")
    _need(a_iv > 0, "the IV profile has no area")
    return _out(AUC_iv=a_iv, AUC_oral=a_or, F=(a_or / D_oral) / (a_iv / D_iv))


def _ref_pk_bioavailability(D_iv, t_iv, C_iv, D_oral, t_oral, C_oral):
    from scipy.integrate import trapezoid
    a_iv, a_or = float(trapezoid(C_iv, t_iv)), float(trapezoid(C_oral, t_oral))
    return {"AUC_iv": a_iv, "AUC_oral": a_or, "F": (a_or * D_iv) / (a_iv * D_oral)}


def _rand_pk_bio(rng):
    ts = [0, 0.5, 1, 2, 4, 6, 8, 12, 24][: rng.randint(5, 9)]
    k, ka = _lu(rng, 0.05, 0.3), _lu(rng, 0.5, 3)
    Fv = rng.uniform(0.2, 1.0)
    d1, d2 = _sig(_lu(rng, 1e-5, 1e-3)), _sig(_lu(rng, 1e-5, 1e-3))
    civ = [_sig(d1 * 1e5 * math.exp(-k * s)) for s in ts]
    cor = [_sig(Fv * d2 * 1e5 * ka / (ka - k) * (math.exp(-k * s) - math.exp(-ka * s))) for s in ts]
    return {"D_iv": d1, "t_iv": [float(s) for s in ts], "C_iv": civ, "D_oral": d2, "t_oral": [float(s) for s in ts],
            "C_oral": [max(0.0, c) for c in cor]}


_add("bmed_pk_bioavailability_from_profiles",
     name="Absolute bioavailability from IV and oral concentration profiles" + _NOTE, topic=_PKD,
     purpose="The absolute bioavailability F from the dose-normalised areas under an oral and an IV concentration-time profile (linear trapezoids from the first to the last sample).",
     inputs=[_phys("D_iv", "mass", "IV dose"), _vecin("t_iv", "IV sampling times (one time unit for both profiles)", 2),
             _vecin("C_iv", "IV concentrations (one concentration unit for both profiles)", 2),
             _phys("D_oral", "mass", "oral dose"), _vecin("t_oral", "oral sampling times", 2), _vecin("C_oral", "oral concentrations", 2)],
     outputs=[_o("AUC_iv", "number", "IV area (units of the lists)"), _o("AUC_oral", "number", "oral area (units of the lists)"),
              _o("F", "number", "absolute bioavailability (AUC_oral/D_oral)/(AUC_iv/D_iv)")],
     primary="F", conditions=["linear pharmacokinetics (dose-proportional areas)", "both profiles sampled to a comparable end",
                              "areas to the last sample, no extrapolation", "the same units in both profiles"],
     method="F = (AUC_oral D_iv)/(AUC_iv D_oral) with linear-trapezoid areas", latex=r"F = \frac{AUC_{oral}\,D_{iv}}{AUC_{iv}\,D_{oral}}",
     run=pk_bioavailability, reference=_ref_pk_bioavailability, rand=_rand_pk_bio,
     wrong=lambda D_iv, t_iv, C_iv, D_oral, t_oral, C_oral: (lambda r: dict(r, F=_r(r["AUC_oral"] / r["AUC_iv"])))(
         pk_bioavailability(D_iv, t_iv, C_iv, D_oral, t_oral, C_oral)),
     phrases=("Bioavailability from profiles", "Absolute bioavailability"),
     examples=[_ex({"D_iv": "100 mg", "t_iv": [0, 1, 2, 4, 8, 12], "C_iv": [10, 8.2, 6.7, 4.5, 2.0, 0.9],
                    "D_oral": "200 mg", "t_oral": [0, 1, 2, 4, 8, 12], "C_oral": [0, 6.1, 8.0, 7.2, 3.9, 1.8]}),
               _ex({"D_iv": "50 mg", "t_iv": [0, 2, 4, 8], "C_iv": [5, 3.5, 2.4, 1.2],
                    "D_oral": "50 mg", "t_oral": [0, 2, 4, 8], "C_oral": [0, 2.2, 2.0, 1.1]})],
     level="intermediate")


# ---------------------------------------------------------------- hepatic clearance models
def _hep_inputs(Q, fu, CL_int):
    _pos("the blood flow Q and the intrinsic clearance CL_int", Q, CL_int)
    _frac("the unbound fraction fu", fu, lo_open=True)


def pk_well_stirred(Q, fu, CL_int):
    _hep_inputs(Q, fu, CL_int)
    cl = Q * fu * CL_int / (Q + fu * CL_int)
    return _out(CL_H=cl, E=cl / Q)


def _ref_pk_well_stirred(Q, fu, CL_int):
    VL = 1.0                                          # a well-mixed liver of any volume, inflow concentration 1
    y = _ivp(lambda s, y: [(Q * (1.0 - y[0]) - fu * CL_int * y[0]) / VL], 80.0 * VL / (Q + fu * CL_int), [0.0])
    E = 1.0 - y[0]
    return {"CL_H": Q * E, "E": E}


def _rand_hep(rng):
    Q = _sig(_lu(rng, 1e-5, 3e-5))
    return {"Q": Q, "fu": round(rng.uniform(0.02, 1.0), 3), "CL_int": _sig(Q * _lu(rng, 0.05, 50.0))}


_add("bmed_pk_hepatic_clearance_well_stirred",
     name="Hepatic clearance: well-stirred liver model" + _NOTE, topic=_PK,
     purpose="The hepatic clearance and extraction ratio of the well-stirred (venous equilibrium) liver model from hepatic blood flow, unbound fraction and intrinsic clearance.",
     inputs=[_phys("Q", "flow_rate", "hepatic blood flow"), _numin("fu", "unbound fraction in blood (0 < fu <= 1)", pct=True),
             _phys("CL_int", "flow_rate", "intrinsic clearance of unbound drug")],
     outputs=[_o("CL_H", "flow_rate", "hepatic clearance"), _o("E", "number", "extraction ratio")],
     primary="CL_H", conditions=["well-stirred liver: outflow concentration equals the liver concentration", "linear elimination"],
     method="CL_H = Q fu CL_int / (Q + fu CL_int); E = CL_H / Q", latex=r"CL_H = \frac{Q\,f_u\,CL_{int}}{Q + f_u\,CL_{int}}",
     run=pk_well_stirred, reference=_ref_pk_well_stirred, rand=_rand_hep,
     wrong=lambda Q, fu, CL_int: (lambda cl: _out(CL_H=cl, E=cl / Q))(Q * fu * CL_int / (Q + CL_int)),
     phrases=("Well-stirred hepatic clearance", "Well-stirred liver model"),
     examples=[_ex({"Q": "90 L/h", "fu": 0.1, "CL_int": "500 L/h"}), _ex({"Q": "1.5 L/min", "fu": "50%", "CL_int": "0.6 L/min"})],
     level="intermediate")


def pk_parallel_tube(Q, fu, CL_int):
    _hep_inputs(Q, fu, CL_int)
    E = -math.expm1(-fu * CL_int / Q)
    return _out(CL_H=Q * E, E=E)


def _ref_pk_parallel_tube(Q, fu, CL_int):
    y = _ivp(lambda z, y: [-(fu * CL_int / Q) * y[0]], 1.0, [1.0])       # concentration along the sinusoid
    E = 1.0 - y[0]
    return {"CL_H": Q * E, "E": E}


_add("bmed_pk_hepatic_clearance_parallel_tube",
     name="Hepatic clearance: parallel-tube (sinusoidal) liver model" + _NOTE, topic=_PK,
     purpose="The hepatic clearance and extraction ratio of the parallel-tube liver model, in which the concentration falls along the sinusoids, from blood flow, unbound fraction and intrinsic clearance.",
     inputs=[_phys("Q", "flow_rate", "hepatic blood flow"), _numin("fu", "unbound fraction in blood (0 < fu <= 1)", pct=True),
             _phys("CL_int", "flow_rate", "intrinsic clearance of unbound drug")],
     outputs=[_o("CL_H", "flow_rate", "hepatic clearance"), _o("E", "number", "extraction ratio")],
     primary="CL_H", conditions=["plug flow along identical sinusoids, enzymes evenly distributed", "linear elimination"],
     method="E = 1 - exp(-fu CL_int / Q); CL_H = Q E", latex=r"CL_H = Q\left(1 - e^{-f_u CL_{int}/Q}\right)",
     run=pk_parallel_tube, reference=_ref_pk_parallel_tube, rand=_rand_hep,
     wrong=lambda Q, fu, CL_int: pk_well_stirred(Q, fu, CL_int),
     phrases=("Parallel-tube hepatic clearance", "Parallel-tube liver model"),
     examples=[_ex({"Q": "90 L/h", "fu": 0.1, "CL_int": "500 L/h"}), _ex({"Q": "1.5 L/min", "fu": "50%", "CL_int": "0.6 L/min"})],
     level="advanced")


# ---------------------------------------------------------------- effect compartment
def pk_effect_site(D, V, k, ke0, t):
    _pos("the dose D and the volume V", D, V)
    _ka_k(ke0, k)
    _nonneg("the time t", t)
    ce = D / V * ke0 / (ke0 - k) * (math.exp(-k * t) - math.exp(-ke0 * t))
    tp = math.log(ke0 / k) / (ke0 - k)
    return _out(Ce=ce, t_peak=tp, Ce_peak=D / V * math.exp(-k * tp))


def _ref_pk_effect_site(D, V, k, ke0, t):
    fun = lambda s, y: [-k * y[0], ke0 * (y[0] - y[1])]
    y = _ivp(fun, t, [1.0, 0.0], atol=[1e-22, 1e-22])
    g = lambda y: y[0] - y[1]
    gdot = lambda s, y: -k * y[0] - ke0 * (y[0] - y[1])
    tp, yp = _ivp_root(fun, [1.0, 0.0], g, gdot, 40.0 / min(k, ke0), atol=[1e-22, 1e-22])
    return {"Ce": D / V * y[1], "t_peak": tp, "Ce_peak": D / V * yp[1]}


def _rand_pk_effect(rng):
    k = _rand_k(rng, 1e-5, 3e-4)
    return {"D": _rand_dose(rng), "V": _rand_vol(rng), "k": k, "ke0": _sig(k * (_lu(rng, 1.5, 30) if rng.random() < 0.8 else _lu(rng, 0.05, 0.6))),
            "t": _sig(rng.uniform(0.05, 5) / k)}


_add("bmed_pk_effect_site_concentration",
     name="Effect-compartment model: effect-site concentration and its peak after an IV bolus" + _NOTE, topic=_PK,
     purpose="The effect-site concentration of a one-compartment model linked to an effect compartment (rate ke0) after an IV bolus, the time of its peak and the peak value.",
     inputs=[_phys("D", "mass", "bolus dose"), _phys("V", "volume", "volume of distribution"),
             _phys("k", "first_order_rate_constant", "elimination rate constant"),
             _phys("ke0", "first_order_rate_constant", "effect-site equilibration rate constant"), _phys("t", "time", "time after the dose")],
     outputs=[_o("Ce", "mass_concentration", "effect-site concentration at time t"), _o("t_peak", "time", "time of the effect-site peak"),
              _o("Ce_peak", "mass_concentration", "peak effect-site concentration")],
     primary="Ce", conditions=["one compartment plus an effect compartment of negligible volume", "ke0 != k"],
     method="dCe/dt = ke0 (C - Ce): Ce = (D/V) ke0/(ke0 - k) (exp(-k t) - exp(-ke0 t)); peak where Ce = C",
     latex=r"C_e(t) = \frac{D}{V}\,\frac{k_{e0}}{k_{e0} - k}\left(e^{-k t} - e^{-k_{e0} t}\right),\quad t_{peak} = \frac{\ln(k_{e0}/k)}{k_{e0} - k}",
     run=pk_effect_site, reference=_ref_pk_effect_site, rand=_rand_pk_effect,
     wrong=lambda D, V, k, ke0, t: (lambda r: dict(r, t_peak=_r(math.log(ke0 / k) / (ke0 + k))))(pk_effect_site(D, V, k, ke0, t)),
     phrases=("Effect-site concentration", "Effect compartment concentration"),
     examples=[_ex({"D": "10 mg", "V": "20 L", "k": "0.3 1/h", "ke0": "2 1/h", "t": "30 min"}),
               _ex({"D": "100 mg", "V": "50 L", "k": "0.1 1/h", "ke0": "0.5 1/h", "t": "4 h"})],
     level="advanced")


# ================================================================================================ EPIDEMIC MODELS
_EPI = "epidemic models"


def _final_size(R0, s0, n0):
    """s_inf of ln(s0/s) = R0 (s0 + n0 - s), n0 = initially infected (and exposed) fraction > 0."""
    f = lambda s: math.log(s0 / s) - R0 * (s0 + n0 - s)
    lo = 0.5 * s0 * math.exp(-R0 * (s0 + n0))
    hi = min(s0, 1.0 / R0)
    return _bisect(f, lo, hi)


def epi_sir_final_size(R0, s0, i0):
    _pos("R0", R0)
    _frac("s0", s0, lo_open=True)
    _frac("i0", i0, lo_open=True, hi_open=True)
    _need(s0 + i0 <= 1 + 1e-12, "s0 + i0 cannot exceed 1")
    s = _final_size(R0, s0, i0)
    return _out(s_inf=s, attack=s0 - s)


def _epi_to_equilibrium(fun, y0, idx, chunk):
    y, T = list(y0), 0.0
    for _ in range(400):
        y = _ivp(fun, T + chunk, y, t0=T, atol=[1e-24] * len(y0))
        T += chunk
        if all(y[j] < 1e-22 for j in idx):
            return y
    raise RuntimeError("the epidemic did not end")


def _ref_epi_sir_final_size(R0, s0, i0):
    y = _epi_to_equilibrium(lambda t, y: [-R0 * y[0] * y[1], R0 * y[0] * y[1] - y[1]], [s0, i0], [1], 200.0)
    return {"s_inf": y[0], "attack": s0 - y[0]}


def _rand_epi_fs(rng):
    i0 = _sig(_lu(rng, 1e-3, 0.05), 3)
    return {"R0": _sig(_lu(rng, 1.2, 8.0), 3), "s0": round(rng.uniform(0.5, 1.0 - i0), 3), "i0": i0}


_add("bmed_epi_sir_final_size",
     name="SIR epidemic model: final size (final susceptible fraction and attack fraction)", topic=_EPI,
     purpose="The fraction still susceptible when an SIR epidemic ends and the fraction infected during it, from R0 and the initial susceptible and infected fractions (the final-size relation).",
     inputs=[_numin("R0", "basic reproduction number beta/gamma"), _numin("s0", "initial susceptible fraction", pct=True),
             _numin("i0", "initial infected fraction (> 0)", pct=True)],
     outputs=[_o("s_inf", "number", "susceptible fraction at the end"), _o("attack", "number", "fraction infected during the epidemic, s0 - s_inf")],
     primary="s_inf", conditions=["closed population, no births or deaths, homogeneous mixing", "fractions of the population",
                                  "the rest of the population (1 - s0 - i0) is immune"],
     method="final-size relation ln(s0/s_inf) = R0 (s0 + i0 - s_inf), solved by bisection on (0, min(s0, 1/R0)]",
     latex=r"\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + i_0 - s_\infty)",
     run=epi_sir_final_size, reference=_ref_epi_sir_final_size, rand=_rand_epi_fs,
     wrong=lambda R0, s0, i0: (lambda s: _out(s_inf=s, attack=s0 - s))(_final_size(R0, s0, 1e-300)),
     phrases=("SIR final size", "SIR epidemic final size"),
     examples=[_ex({"R0": 2.5, "s0": 0.99, "i0": 0.01}), _ex({"R0": 1.5, "s0": 0.7, "i0": 0.001})],
     level="intermediate",
     extra_q=[{"q": "SIR final size: R0 = 2.5, s0 = 99%, i0 = 1%", "like": 0},
              {"q": "SIR final size: R0 = 2.5, s0 = 0.99, i0 = 0", "outcome": "INVALID_INPUT"}])


def epi_seir_final_size(R0, s0, e0, i0):
    _pos("R0", R0)
    _frac("s0", s0, lo_open=True)
    _frac("e0", e0)
    _frac("i0", i0)
    _need(e0 + i0 > 0, "some exposed or infected must be present at the start")
    _need(s0 + e0 + i0 <= 1 + 1e-12, "s0 + e0 + i0 cannot exceed 1")
    s = _final_size(R0, s0, e0 + i0)
    return _out(s_inf=s, attack=s0 - s)


def _ref_epi_seir_final_size(R0, s0, e0, i0):
    sig = 0.6                                            # any incubation rate gives the same final size
    f = lambda t, y: [-R0 * y[0] * y[2], R0 * y[0] * y[2] - sig * y[1], sig * y[1] - y[2]]
    y = _epi_to_equilibrium(f, [s0, e0, i0], [1, 2], 300.0)
    return {"s_inf": y[0], "attack": s0 - y[0]}


def _rand_epi_seir_fs(rng):
    e0, i0 = _sig(_lu(rng, 1e-3, 0.03), 3), _sig(_lu(rng, 1e-3, 0.03), 3)
    return {"R0": _sig(_lu(rng, 1.2, 8.0), 3), "s0": round(rng.uniform(0.5, 1.0 - e0 - i0), 3), "e0": e0, "i0": i0}


_add("bmed_epi_seir_final_size",
     name="SEIR epidemic model: final size", topic=_EPI,
     purpose="The final susceptible fraction and the attack fraction of an SEIR epidemic (the latent stage does not change the final size) from R0 and the initial susceptible, exposed and infectious fractions.",
     inputs=[_numin("R0", "basic reproduction number beta/gamma"), _numin("s0", "initial susceptible fraction", pct=True),
             _numin("e0", "initial exposed (latent) fraction", pct=True), _numin("i0", "initial infectious fraction", pct=True)],
     outputs=[_o("s_inf", "number", "susceptible fraction at the end"), _o("attack", "number", "fraction infected during the epidemic, s0 - s_inf")],
     primary="s_inf", conditions=["closed population, homogeneous mixing", "exponentially distributed latent and infectious periods",
                                  "e0 + i0 > 0"],
     method="final-size relation ln(s0/s_inf) = R0 (s0 + e0 + i0 - s_inf), solved by bisection",
     latex=r"\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + e_0 + i_0 - s_\infty)",
     run=epi_seir_final_size, reference=_ref_epi_seir_final_size, rand=_rand_epi_seir_fs,
     wrong=lambda R0, s0, e0, i0: (lambda s: _out(s_inf=s, attack=s0 - s))(_final_size(R0, s0, i0 if i0 > 0 else 1e-300)),
     phrases=("SEIR final size", "SEIR epidemic final size"),
     examples=[_ex({"R0": 3, "s0": 0.98, "e0": 0.01, "i0": 0.01}), _ex({"R0": 1.8, "s0": 0.6, "e0": 0.002, "i0": 0})],
     level="advanced")


def _sir_peak_time(beta, gamma, s0, i0):
    import mpmath as mp
    R0 = beta / gamma
    with mp.workdps(25):
        b, g, S0, I0 = mp.mpf(beta), mp.mpf(gamma), mp.mpf(s0), mp.mpf(i0)
        sp = g / b
        f = lambda s: 1 / (b * s * (S0 + I0 - s + mp.log(s / S0) * g / b))
        pts = [sp] + [S0 - I0 * m for m in (1000, 100, 10, 1) if S0 - I0 * m > sp] + [S0]
        return float(mp.quad(f, pts))


def epi_sir_peak(beta, gamma, s0, i0):
    _pos("beta and gamma", beta, gamma)
    _frac("s0", s0, lo_open=True)
    _frac("i0", i0, lo_open=True, hi_open=True)
    _need(s0 + i0 <= 1 + 1e-12, "s0 + i0 cannot exceed 1")
    R0 = beta / gamma
    _need(R0 * s0 > 1.0 + 1e-6, "no epidemic peak: R0 s0 must exceed 1 (prevalence only falls)")
    imax = s0 + i0 - (1.0 + math.log(R0 * s0)) / R0
    return _out(i_max=imax, t_peak=_sir_peak_time(beta, gamma, s0, i0))


def _ref_epi_sir_peak(beta, gamma, s0, i0):
    fun = lambda t, y: [-beta * y[0] * y[1], beta * y[0] * y[1] - gamma * y[1]]
    g = lambda y: beta * y[0] - gamma
    gdot = lambda t, y: -beta * beta * y[0] * y[1]
    tp, y = _ivp_root(fun, [s0, i0], g, gdot, 600.0 / gamma, atol=[1e-22, 1e-22])
    return {"i_max": y[1], "t_peak": tp}


def _rand_epi_sir_peak(rng):
    gamma = _sig(_lu(rng, 2e-7, 5e-6), 3)
    i0 = _sig(_lu(rng, 1e-3, 0.05), 3)
    s0 = round(rng.uniform(0.6, 1.0 - i0), 3)
    R0 = _lu(rng, 1.15 / s0, 6.0)
    return {"beta": _sig(R0 * gamma, 3), "gamma": gamma, "s0": s0, "i0": i0}


_add("bmed_epi_sir_peak",
     name="SIR epidemic model: peak prevalence and the time of the peak", topic=_EPI,
     purpose="The largest infected fraction of an SIR epidemic and the time at which it occurs, from the transmission and recovery rates and the initial fractions.",
     inputs=[_phys("beta", "first_order_rate_constant", "transmission rate"), _phys("gamma", "first_order_rate_constant", "recovery rate"),
             _numin("s0", "initial susceptible fraction", pct=True), _numin("i0", "initial infected fraction (> 0)", pct=True)],
     outputs=[_o("i_max", "number", "peak infected fraction"), _o("t_peak", "time", "time of the peak")],
     primary="i_max", conditions=["closed population, homogeneous mixing, frequency-dependent transmission", "R0 s0 > 1"],
     method="i_max = s0 + i0 - (1 + ln(R0 s0))/R0 from the first integral of the SIR system; t_peak = integral over s from 1/R0 to s0 of ds / (beta s i(s)) (mpmath quadrature)",
     latex=r"i_{max} = s_0 + i_0 - \frac{1 + \ln(R_0 s_0)}{R_0},\quad t_{peak} = \int_{1/R_0}^{s_0}\frac{ds}{\beta s\,i(s)}",
     run=epi_sir_peak, reference=_ref_epi_sir_peak, rand=_rand_epi_sir_peak,
     wrong=lambda beta, gamma, s0, i0: _out(i_max=s0 + i0 - (1.0 - math.log(beta / gamma * s0)) * gamma / beta),
     phrases=("SIR peak", "SIR epidemic peak"),
     examples=[_ex({"beta": "0.3 1/day", "gamma": "0.1 1/day", "s0": 0.999, "i0": 0.001}),
               _ex({"beta": "0.5 1/day", "gamma": "0.2 1/day", "s0": 0.9, "i0": 0.01})],
     level="expert",
     extra_q=[{"q": "SIR peak: beta = 0.1 1/day, gamma = 0.2 1/day, s0 = 0.999, i0 = 0.001", "outcome": "INVALID_INPUT"}])


def epi_seir_growth(R0, sigma, gamma):
    _pos("R0, sigma and gamma", R0, sigma, gamma)
    a = sigma + gamma
    d = 4.0 * sigma * gamma * (R0 - 1.0)
    root = math.sqrt(a * a + d)
    r = d / (2.0 * (a + root)) if d >= 0 else (root - a) / 2.0
    return _out(r=r)


def _ref_epi_seir_growth(R0, sigma, gamma):
    import numpy as np
    J = np.array([[-sigma, R0 * gamma], [sigma, -gamma]])
    return {"r": float(max(np.real(np.linalg.eigvals(J))))}


def _rand_epi_seir_growth(rng):
    return {"R0": _sig(_lu(rng, 1.3, 10.0) if rng.random() < 0.85 else rng.uniform(0.3, 0.8), 3),
            "sigma": _sig(_lu(rng, 5e-7, 5e-6), 3), "gamma": _sig(_lu(rng, 5e-7, 5e-6), 3)}


_add("bmed_epi_seir_growth_rate",
     name="SEIR epidemic model: initial exponential growth rate from R0", topic=_EPI,
     purpose="The early exponential growth (or decline) rate of an SEIR epidemic in a fully susceptible population, from R0, the rate of leaving the latent stage and the recovery rate.",
     inputs=[_numin("R0", "basic reproduction number beta/gamma"), _phys("sigma", "first_order_rate_constant", "rate of leaving the latent stage (1/latent period)"),
             _phys("gamma", "first_order_rate_constant", "recovery rate (1/infectious period)")],
     outputs=[_o("r", "first_order_rate_constant", "initial growth rate (negative when R0 < 1)")],
     primary="r", conditions=["fully susceptible population at the start", "exponential latent and infectious periods"],
     method="largest root of r^2 + (sigma + gamma) r - sigma gamma (R0 - 1) = 0",
     latex=r"r = \frac{-(\sigma + \gamma) + \sqrt{(\sigma + \gamma)^2 + 4\sigma\gamma(R_0 - 1)}}{2}",
     run=epi_seir_growth, reference=_ref_epi_seir_growth, rand=_rand_epi_seir_growth,
     wrong=lambda R0, sigma, gamma: _out(r=gamma * (R0 - 1.0)),
     phrases=("SEIR growth rate", "SEIR initial growth rate"),
     examples=[_ex({"R0": 2.5, "sigma": "0.2 1/day", "gamma": "0.1 1/day"}), _ex({"R0": 0.8, "sigma": "0.5 1/day", "gamma": "0.25 1/day"})],
     level="advanced")


def epi_r_from_growth(r, Tg_mean, Tg_sd):
    _fin("the growth rate r", r)
    _pos("the mean and standard deviation of the generation interval", Tg_mean, Tg_sd)
    k = (Tg_mean / Tg_sd) ** 2
    th = Tg_sd ** 2 / Tg_mean
    _need(1.0 + r * th > 0, "the decline is too fast for this generation interval (1 + r sd^2/mean must be positive)")
    return _out(R=math.exp(k * math.log1p(r * th)))


def _ref_epi_r_from_growth(r, Tg_mean, Tg_sd):
    from scipy.stats import gamma as G
    k = (Tg_mean / Tg_sd) ** 2
    th = Tg_sd ** 2 / Tg_mean
    f = lambda u: math.exp(-r * th * u + float(G.logpdf(u, k))) if u > 0 else 0.0   # in units of the scale theta
    cut = max(10.0, 10.0 * k)
    return {"R": 1.0 / (_quad(f, 0.0, cut) + _quad(f, cut, math.inf))}


def _rand_epi_r_growth(rng):
    m = _sig(_lu(rng, 2.0, 15.0) * 86400.0, 4)
    sd = _sig(m * rng.uniform(0.25, 0.9), 4)
    x = rng.uniform(0.05, 1.5) if rng.random() < 0.8 else -rng.uniform(0.05, 0.5)
    return {"r": _sig(x / m, 4), "Tg_mean": m, "Tg_sd": sd}


_add("bmed_epi_r_from_growth_rate",
     name="Reproduction number from an epidemic growth rate (gamma-distributed generation interval)", topic=_EPI,
     purpose="The reproduction number implied by an observed exponential growth rate when the generation interval follows a gamma distribution of stated mean and standard deviation (Wallinga-Lipsitch).",
     inputs=[_phys("r", "first_order_rate_constant", "exponential growth rate (negative for a decline)"),
             _phys("Tg_mean", "time", "mean generation interval"), _phys("Tg_sd", "time", "standard deviation of the generation interval")],
     outputs=[_o("R", "number", "reproduction number")],
     primary="R", conditions=["exponential growth or decline", "gamma-distributed generation interval"],
     method="R = 1 / M(-r), M the moment generating function of the gamma distribution: R = (1 + r theta)^k, k = mean^2/sd^2, theta = sd^2/mean",
     latex=r"R = \left(1 + r\,\frac{s^2}{\mu}\right)^{\mu^2/s^2}",
     run=epi_r_from_growth, reference=_ref_epi_r_from_growth, rand=_rand_epi_r_growth,
     wrong=lambda r, Tg_mean, Tg_sd: _out(R=1.0 + r * Tg_mean),
     phrases=("Reproduction number from growth rate", "R from growth rate"),
     examples=[_ex({"r": "0.1 1/day", "Tg_mean": "5 day", "Tg_sd": "2 day"}), _ex({"r": "-0.05 1/day", "Tg_mean": "6.5 day", "Tg_sd": "4 day"})],
     level="advanced")


def epi_sis(beta, gamma, i0, t):
    _pos("beta and gamma", beta, gamma)
    _frac("i0", i0, lo_open=True)
    _nonneg("the time t", t)
    r = beta - gamma
    if r == 0:
        i = i0 / (1.0 + beta * i0 * t)
    else:
        e = math.exp(r * t)
        i = i0 * e / (1.0 + i0 * beta * math.expm1(r * t) / r)
    return _out(i=i, i_star=max(0.0, 1.0 - gamma / beta))


def _ref_epi_sis(beta, gamma, i0, t):
    f = lambda s, y: [beta * y[0] * (1.0 - y[0]) - gamma * y[0]]
    y = _ivp(f, t, [i0], atol=[1e-24])
    ye = _ivp(f, 200.0 / abs(beta - gamma), [i0], atol=[1e-24])
    return {"i": y[0], "i_star": ye[0] if ye[0] > 1e-15 else 0.0}


def _rand_epi_sis(rng):
    gamma = _sig(_lu(rng, 5e-7, 5e-6), 3)
    beta = _sig(gamma * (_lu(rng, 1.15, 5.0) if rng.random() < 0.8 else rng.uniform(0.3, 0.87)), 3)
    return {"beta": beta, "gamma": gamma, "i0": _sig(_lu(rng, 1e-3, 0.3), 3), "t": _sig(rng.uniform(0.1, 8.0) / abs(beta - gamma))}


_add("bmed_epi_sis_prevalence",
     name="SIS epidemic model: prevalence at a time and the endemic level", topic=_EPI,
     purpose="The infected fraction of an SIS model (no immunity after recovery) at time t and the endemic equilibrium it approaches.",
     inputs=[_phys("beta", "first_order_rate_constant", "transmission rate"), _phys("gamma", "first_order_rate_constant", "recovery rate"),
             _numin("i0", "initial infected fraction", pct=True), _phys("t", "time", "time")],
     outputs=[_o("i", "number", "infected fraction at time t"), _o("i_star", "number", "endemic infected fraction (0 when beta <= gamma)")],
     primary="i", conditions=["closed population, homogeneous mixing", "recovered individuals are susceptible again"],
     method="logistic solution of di/dt = beta i (1 - i) - gamma i: i = i0 e^{rt} / (1 + i0 beta (e^{rt} - 1)/r), r = beta - gamma",
     latex=r"i(t) = \frac{i_0\,e^{rt}}{1 + i_0\,\beta\,(e^{rt} - 1)/r},\quad r = \beta - \gamma,\quad i^* = 1 - \gamma/\beta",
     run=epi_sis, reference=_ref_epi_sis, rand=_rand_epi_sis,
     wrong=lambda beta, gamma, i0, t: (lambda r: _out(i=i0 * math.exp(r * t) / (1.0 + i0 * math.expm1(r * t)), i_star=1.0))(beta - gamma),
     phrases=("SIS prevalence", "SIS epidemic prevalence"),
     examples=[_ex({"beta": "0.4 1/day", "gamma": "0.1 1/day", "i0": 0.01, "t": "10 day"}),
               _ex({"beta": "0.1 1/day", "gamma": "0.2 1/day", "i0": 0.3, "t": "5 day"})],
     level="intermediate")


def epi_reed_frost(n, m, p):
    _need(isinstance(n, int) and 1 <= n <= 30, "n (initial susceptibles) is a whole number from 1 to 30")
    _need(isinstance(m, int) and 1 <= m <= 30, "m (initial infectives) is a whole number from 1 to 30")
    _frac("p", p, lo_open=True, hi_open=True)
    q = 1.0 - p
    states = {(n, m): 1.0}
    final = [0.0] * (n + 1)
    while states:
        nxt = {}
        for (s, i), pr in states.items():
            if i == 0 or s == 0:
                final[n - s] += pr
                continue
            qi = q ** i
            pi = 1.0 - qi
            for j in range(s + 1):
                w = pr * math.comb(s, j) * pi ** j * qi ** (s - j)
                if w:
                    nxt[(s - j, j)] = nxt.get((s - j, j), 0.0) + w
        states = nxt
    return _out(mean_final=math.fsum(k * v for k, v in enumerate(final)), P_no_spread=final[0], distribution=final)


def _ref_epi_reed_frost(n, m, p):
    from fractions import Fraction as Fr
    from math import comb
    q = 1 - Fr(repr(p))
    P = []
    for k in range(n + 1):
        acc = Fr(comb(n, k))
        for j in range(k):
            acc -= comb(n - j, k - j) * P[j] / q ** ((m + j) * (n - k))
        P.append(acc * q ** ((m + k) * (n - k)))
    dist = [float(v) for v in P]
    return {"mean_final": float(sum(k * v for k, v in enumerate(P))), "P_no_spread": dist[0], "distribution": dist}


def _wrong_epi_reed_frost(n, m, p):
    states, final = {(n, m): 1.0}, [0.0] * (n + 1)
    while states:
        nxt = {}
        for (s, i), pr in states.items():
            if i == 0 or s == 0:
                final[n - s] += pr
                continue
            pi = min(1.0, p * i)                          # linearised infection chance
            for j in range(s + 1):
                w = pr * math.comb(s, j) * pi ** j * (1 - pi) ** (s - j)
                if w:
                    nxt[(s - j, j)] = nxt.get((s - j, j), 0.0) + w
        states = nxt
    return _out(mean_final=sum(k * v for k, v in enumerate(final)), P_no_spread=final[0], distribution=final)


_add("bmed_epi_reed_frost_final_size",
     name="Reed-Frost chain-binomial epidemic: distribution and mean of the final size", topic=_EPI,
     purpose="The probability distribution of the number of susceptibles eventually infected in a Reed-Frost chain-binomial epidemic in a small closed group, its mean and the chance of no spread.",
     inputs=[_intin("n", "initial number of susceptibles (1 to 30)", 1, 30), _intin("m", "initial number of infectives (1 to 30)", 1, 30),
             _numin("p", "probability that one infective infects one given susceptible in a generation (0 < p < 1)")],
     outputs=[_o("mean_final", "number", "expected number of susceptibles infected"),
              _o("P_no_spread", "number", "probability that no susceptible is infected"),
              _o("distribution", "number", "probabilities of 0, 1, ..., n susceptibles infected")],
     primary="mean_final", conditions=["discrete generations, each infective infectious for one generation", "independent contacts"],
     method="Markov chain over (susceptibles, infectives): a susceptible escapes i infectives with probability (1 - p)^i; new infections are binomial; probabilities summed over all paths",
     latex=r"S_{t+1} \sim \mathrm{Bin}\!\left(S_t, (1 - p)^{I_t}\right),\quad I_{t+1} = S_t - S_{t+1}",
     run=epi_reed_frost, reference=_ref_epi_reed_frost, wrong=_wrong_epi_reed_frost,
     rand=lambda rng: {"n": rng.randint(1, 20), "m": rng.randint(1, 3), "p": _sig(_lu(rng, 0.02, 0.5), 3)},
     phrases=("Reed-Frost final size", "Reed-Frost epidemic"),
     examples=[_ex({"n": 4, "m": 1, "p": 0.2}), _ex({"n": 10, "m": 2, "p": 0.05})],
     level="expert",
     extra_q=[{"q": "Reed-Frost final size: n = 4.5, m = 1, p = 0.2", "outcome": "INVALID_INPUT"},
              {"q": "Reed-Frost final size: n = 4, m = 1, p = 1.2", "outcome": "INVALID_INPUT"}])


# ================================================================================================ DIAGNOSTIC STATISTICS
_DX = "diagnostic test statistics"
_EP = "epidemiological measures of association"


def _two_groups(pos, neg):
    _need(len(pos) >= 1 and len(neg) >= 1, "give at least one score in each group")
    for v in list(pos) + list(neg):
        _fin("every score", v)


def dx_roc_auc(scores_pos, scores_neg):
    _two_groups(scores_pos, scores_neg)
    u = math.fsum(1.0 if a > b else (0.5 if a == b else 0.0) for a in scores_pos for b in scores_neg)
    return _out(AUC=u / (len(scores_pos) * len(scores_neg)), U=u)


def _ref_dx_roc_auc(scores_pos, scores_neg):
    from sklearn.metrics import roc_auc_score
    from scipy.stats import mannwhitneyu
    y = [1] * len(scores_pos) + [0] * len(scores_neg)
    return {"AUC": float(roc_auc_score(y, list(scores_pos) + list(scores_neg))),
            "U": float(mannwhitneyu(scores_pos, scores_neg).statistic)}


def _rand_dx_scores(rng):
    n1, n0 = rng.randint(3, 40), rng.randint(3, 40)
    d = rng.uniform(0.2, 2.0)
    return {"scores_pos": [round(rng.gauss(d, 1.0), 1) for _ in range(n1)], "scores_neg": [round(rng.gauss(0, 1.0), 1) for _ in range(n0)]}


_add("bmed_dx_roc_auc",
     name="ROC curve area of a diagnostic score (Mann-Whitney form)", topic=_DX,
     purpose="The area under the ROC curve of a continuous diagnostic score from the scores of diseased and non-diseased subjects: the probability that a diseased subject scores higher (ties count one half), with the Mann-Whitney U.",
     inputs=[_vecin("scores_pos", "scores of the diseased (condition-positive) subjects", 1),
             _vecin("scores_neg", "scores of the non-diseased subjects", 1)],
     outputs=[_o("AUC", "number", "area under the ROC curve"), _o("U", "number", "Mann-Whitney U of the diseased group")],
     primary="AUC", conditions=["higher scores indicate disease", "ties count one half"],
     method="AUC = U/(n1 n0), U = number of (diseased, non-diseased) pairs with the diseased score higher + half the ties",
     latex=r"AUC = \frac{1}{n_1 n_0}\sum_{i,j}\left[\mathbb{1}(x_i > y_j) + \tfrac12\mathbb{1}(x_i = y_j)\right]",
     run=dx_roc_auc, reference=_ref_dx_roc_auc, rand=_rand_dx_scores,
     wrong=lambda scores_pos, scores_neg: (lambda r: _out(AUC=1 - r["AUC"], U=len(scores_pos) * len(scores_neg) - r["U"]))(dx_roc_auc(scores_pos, scores_neg)),
     phrases=("ROC AUC", "ROC curve area"),
     examples=[_ex({"scores_pos": [0.9, 0.8, 0.7, 0.55, 0.4], "scores_neg": [0.6, 0.45, 0.4, 0.3, 0.2, 0.1]}),
               _ex({"scores_pos": [3, 4, 5, 5], "scores_neg": [1, 2, 3, 5]})],
     level="intermediate")


def _youden(scores_pos, scores_neg):
    from fractions import Fraction as Fr
    n1, n0 = len(scores_pos), len(scores_neg)
    best = None
    for thr in sorted(set(scores_pos) | set(scores_neg), reverse=True):
        tp = sum(1 for v in scores_pos if v >= thr)
        fp = sum(1 for v in scores_neg if v >= thr)
        J = Fr(tp, n1) - Fr(fp, n0)
        if best is None or J > best[0]:
            best = (J, thr, tp, fp)
    return best, n1, n0


def dx_youden(scores_pos, scores_neg):
    _two_groups(scores_pos, scores_neg)
    (J, thr, tp, fp), n1, n0 = _youden(scores_pos, scores_neg)
    _need(J > 0, "the scores do not separate the groups (no cut-off with positive Youden index)")
    return _out(threshold=float(thr), sensitivity=tp / n1, specificity=1 - fp / n0, J=float(J))


def _ref_dx_youden(scores_pos, scores_neg):
    import numpy as np
    from sklearn.metrics import roc_curve
    y = [1] * len(scores_pos) + [0] * len(scores_neg)
    fpr, tpr, thr = roc_curve(y, list(scores_pos) + list(scores_neg), drop_intermediate=False)
    J = np.round(tpr - fpr, 12)
    i = 1 + int(np.argmax(J[1:]))                      # the highest threshold among the maxima
    return {"threshold": float(thr[i]), "sensitivity": float(tpr[i]), "specificity": float(1 - fpr[i]), "J": float(tpr[i] - fpr[i])}


def _wrong_dx_youden(scores_pos, scores_neg):
    n1, n0 = len(scores_pos), len(scores_neg)
    best = None
    for thr in sorted(set(scores_pos) | set(scores_neg), reverse=True):
        tp = sum(1 for v in scores_pos if v >= thr)
        fp = sum(1 for v in scores_neg if v >= thr)
        J = tp / n1 - (1 - fp / n0)                    # sensitivity minus specificity
        if best is None or J > best[0]:
            best = (J, thr, tp, fp)
    J, thr, tp, fp = best
    return _out(threshold=float(thr), sensitivity=tp / n1, specificity=1 - fp / n0, J=tp / n1 - fp / n0)


_add("bmed_dx_youden_cutoff",
     name="Optimal diagnostic cut-off by the Youden index", topic=_DX,
     purpose="The score threshold that maximises the Youden index J = sensitivity + specificity - 1 over all observed cut-offs, with the sensitivity and specificity there.",
     inputs=[_vecin("scores_pos", "scores of the diseased subjects", 1), _vecin("scores_neg", "scores of the non-diseased subjects", 1)],
     outputs=[_o("threshold", "number", "optimal cut-off (score >= threshold is test-positive)"),
              _o("sensitivity", "number", "sensitivity at the cut-off"), _o("specificity", "number", "specificity at the cut-off"),
              _o("J", "number", "Youden index at the cut-off")],
     primary="threshold", conditions=["higher scores indicate disease", "cut-offs at the observed scores",
                                      "ties in J: the highest threshold (most specific)", "some cut-off has J > 0"],
     method="for every observed score as threshold: J = TP/n1 - FP/n0 (exact fractions); the maximum, highest threshold first",
     latex=r"J(c) = \mathrm{Se}(c) + \mathrm{Sp}(c) - 1,\quad c^* = \arg\max_c J(c)",
     run=dx_youden, reference=_ref_dx_youden, wrong=_wrong_dx_youden, rand=_rand_dx_scores,
     phrases=("Youden cut-off", "Youden optimal cut-off"),
     examples=[_ex({"scores_pos": [0.9, 0.8, 0.7, 0.55, 0.4], "scores_neg": [0.6, 0.45, 0.4, 0.3, 0.2, 0.1]}),
               _ex({"scores_pos": [12, 15, 18, 22, 25, 30], "scores_neg": [8, 10, 11, 14, 16, 19, 21]})],
     level="intermediate")


def dx_threshold(scores_pos, scores_neg, threshold):
    _two_groups(scores_pos, scores_neg)
    _fin("the threshold", threshold)
    tp = sum(1 for v in scores_pos if v >= threshold)
    fp = sum(1 for v in scores_neg if v >= threshold)
    fn, tn = len(scores_pos) - tp, len(scores_neg) - fp
    return _out(sensitivity=tp / (tp + fn), specificity=tn / (tn + fp), TP=tp, FN=fn, TN=tn, FP=fp)


def _ref_dx_threshold(scores_pos, scores_neg, threshold):
    from sklearn.metrics import confusion_matrix, recall_score
    y = [1] * len(scores_pos) + [0] * len(scores_neg)
    yp = [1 if v >= threshold else 0 for v in list(scores_pos) + list(scores_neg)]
    tn, fp, fn, tp = (int(v) for v in confusion_matrix(y, yp, labels=[0, 1]).ravel())
    return {"sensitivity": float(recall_score(y, yp, pos_label=1, zero_division=0)),
            "specificity": float(recall_score(y, yp, pos_label=0, zero_division=0)), "TP": tp, "FN": fn, "TN": tn, "FP": fp}


_add("bmed_dx_threshold_sensitivity_specificity",
     name="Sensitivity and specificity of a diagnostic score at a cut-off", topic=_DX,
     purpose="The confusion-matrix counts, sensitivity and specificity of a continuous diagnostic score when scores at or above a threshold are called positive.",
     inputs=[_vecin("scores_pos", "scores of the diseased subjects", 1), _vecin("scores_neg", "scores of the non-diseased subjects", 1),
             _numin("threshold", "cut-off: a score >= threshold is test-positive")],
     outputs=[_o("sensitivity", "number", "TP/(TP + FN)"), _o("specificity", "number", "TN/(TN + FP)"),
              _o("TP", "number", "true positives"), _o("FN", "number", "false negatives"), _o("TN", "number", "true negatives"),
              _o("FP", "number", "false positives")],
     primary="sensitivity", conditions=["a score at or above the threshold is positive"],
     method="count the 2x2 table at the cut-off; sensitivity = TP/(TP + FN), specificity = TN/(TN + FP)",
     latex=r"\mathrm{Se} = \frac{TP}{TP + FN},\quad \mathrm{Sp} = \frac{TN}{TN + FP}",
     run=dx_threshold, reference=_ref_dx_threshold,
     rand=lambda rng: dict(_rand_dx_scores(rng), threshold=round(rng.uniform(-0.5, 1.5), 1)),
     wrong=lambda scores_pos, scores_neg, threshold: (lambda r: dict(r, specificity=_r(r["FP"] / (r["FP"] + r["TN"]))))(
         dx_threshold(scores_pos, scores_neg, threshold)),
     phrases=("Sensitivity and specificity at a cut-off", "Sensitivity and specificity at threshold"),
     examples=[_ex({"scores_pos": [0.9, 0.8, 0.7, 0.55, 0.4], "scores_neg": [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], "threshold": 0.5}),
               _ex({"scores_pos": [12, 15, 18, 22, 25, 30], "scores_neg": [8, 10, 11, 14, 16, 19, 21], "threshold": 15})],
     level="beginner")


def _wilson(x, n, z):
    p = x / n
    den = 1.0 + z * z / n
    c = (p + z * z / (2 * n)) / den
    h = z * math.sqrt(p * (1 - p) / n + z * z / (4 * n * n)) / den
    return c - h, c + h


def dx_sens_spec_ci(TP, FN, TN, FP, level):
    for nm, v in (("TP", TP), ("FN", FN), ("TN", TN), ("FP", FP)):
        _count(nm, v)
    _need(TP + FN >= 1 and TN + FP >= 1, "each group (diseased, non-diseased) needs at least one subject")
    z = _level(level)
    sl, su = _wilson(TP, TP + FN, z)
    pl, pu = _wilson(TN, TN + FP, z)
    return _out(sensitivity=TP / (TP + FN), sens_lower=sl, sens_upper=su, specificity=TN / (TN + FP), spec_lower=pl, spec_upper=pu)


def _ref_dx_sens_spec_ci(TP, FN, TN, FP, level):
    from sklearn.metrics import recall_score
    from statsmodels.stats.proportion import proportion_confint
    y = [1] * (TP + FN) + [0] * (TN + FP)
    yp = [1] * TP + [0] * FN + [0] * TN + [1] * FP
    sl, su = proportion_confint(TP, TP + FN, alpha=1 - level, method="wilson")
    pl, pu = proportion_confint(TN, TN + FP, alpha=1 - level, method="wilson")
    return {"sensitivity": float(recall_score(y, yp, pos_label=1)), "sens_lower": float(sl), "sens_upper": float(su),
            "specificity": float(recall_score(y, yp, pos_label=0)), "spec_lower": float(pl), "spec_upper": float(pu)}


def _wrong_dx_sens_spec_ci(TP, FN, TN, FP, level):
    z = _level(level)
    wald = lambda x, n: (x / n - z * math.sqrt(x / n * (1 - x / n) / n), x / n + z * math.sqrt(x / n * (1 - x / n) / n))
    sl, su = wald(TP, TP + FN)
    pl, pu = wald(TN, TN + FP)
    return _out(sensitivity=TP / (TP + FN), sens_lower=sl, sens_upper=su, specificity=TN / (TN + FP), spec_lower=pl, spec_upper=pu)


_add("bmed_dx_sensitivity_specificity_ci",
     name="Sensitivity and specificity with Wilson score intervals", topic=_DX,
     purpose="Sensitivity and specificity of a diagnostic test from its 2x2 counts, each with a Wilson score confidence interval at the stated level.",
     inputs=[_intin("TP", "true positives", 0), _intin("FN", "false negatives", 0), _intin("TN", "true negatives", 0),
             _intin("FP", "false positives", 0), _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("sensitivity", "number", "TP/(TP + FN)"), _o("sens_lower", "number", "lower Wilson limit"), _o("sens_upper", "number", "upper Wilson limit"),
              _o("specificity", "number", "TN/(TN + FP)"), _o("spec_lower", "number", "lower Wilson limit"), _o("spec_upper", "number", "upper Wilson limit")],
     primary="sensitivity", conditions=["independent subjects", "Wilson score interval (no continuity correction)", "level stated"],
     method="Wilson interval (p + z^2/2n +- z sqrt(p(1 - p)/n + z^2/4n^2)) / (1 + z^2/n)",
     latex=r"\frac{\hat p + \frac{z^2}{2n} \pm z\sqrt{\frac{\hat p(1-\hat p)}{n} + \frac{z^2}{4n^2}}}{1 + \frac{z^2}{n}}",
     run=dx_sens_spec_ci, reference=_ref_dx_sens_spec_ci, wrong=_wrong_dx_sens_spec_ci,
     rand=lambda rng: {"TP": rng.randint(1, 200), "FN": rng.randint(1, 60), "TN": rng.randint(1, 200), "FP": rng.randint(1, 60),
                       "level": rng.choice([0.9, 0.95, 0.99])},
     phrases=("Sensitivity and specificity intervals", "Sensitivity and specificity with confidence intervals"),
     examples=[_ex({"TP": 45, "FN": 5, "TN": 90, "FP": 10, "level": "95%"}), _ex({"TP": 18, "FN": 2, "TN": 70, "FP": 30, "level": 0.9})],
     level="intermediate",
     extra_q=[{"q": "Sensitivity and specificity intervals: TP = 45, FN = 5, TN = 90, FP = 10", "outcome": "NEEDS_INPUT"}])


def _table(a, b, c, d):
    for nm, v in (("a", a), ("b", b), ("c", c), ("d", d)):
        _count(nm, v, 1)


def dx_odds_ratio(a, b, c, d, level):
    _table(a, b, c, d)
    z = _level(level)
    orr = a * d / (b * c)
    se = math.sqrt(1 / a + 1 / b + 1 / c + 1 / d)
    return _out(OR=orr, lower=orr * math.exp(-z * se), upper=orr * math.exp(z * se))


def _ref_dx_odds_ratio(a, b, c, d, level):
    import numpy as np
    from statsmodels.stats.contingency_tables import Table2x2
    t = Table2x2(np.array([[a, b], [c, d]], dtype=float))
    lo, hi = t.oddsratio_confint(alpha=1 - level, method="normal")
    return {"OR": float(t.oddsratio), "lower": float(lo), "upper": float(hi)}


def _rand_table(rng):
    return {"a": rng.randint(1, 120), "b": rng.randint(1, 200), "c": rng.randint(1, 120), "d": rng.randint(1, 200),
            "level": rng.choice([0.9, 0.95, 0.99])}


_TAB = [_intin("a", "exposed with the outcome", 1), _intin("b", "exposed without the outcome", 1),
        _intin("c", "unexposed with the outcome", 1), _intin("d", "unexposed without the outcome", 1),
        _numin("level", "confidence level (95% or 0.95)", pct=True)]

_add("bmed_odds_ratio_ci",
     name="Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval", topic=_EP,
     purpose="The odds ratio of an exposure-outcome 2x2 table (case-control or cohort) with the Woolf logit confidence interval at the stated level.",
     inputs=list(_TAB),
     outputs=[_o("OR", "number", "odds ratio ad/(bc)"), _o("lower", "number", "lower confidence limit"), _o("upper", "number", "upper confidence limit")],
     primary="OR", conditions=["no zero cell (no continuity correction)", "Woolf interval exp(ln OR +- z SE)", "level stated"],
     method="OR = ad/(bc); SE(ln OR) = sqrt(1/a + 1/b + 1/c + 1/d)",
     latex=r"OR = \frac{ad}{bc},\quad \mathrm{SE}(\ln OR) = \sqrt{\tfrac1a + \tfrac1b + \tfrac1c + \tfrac1d}",
     run=dx_odds_ratio, reference=_ref_dx_odds_ratio, rand=_rand_table,
     wrong=lambda a, b, c, d, level: (lambda z, orr, se: _out(OR=orr, lower=orr * math.exp(-z * se), upper=orr * math.exp(z * se)))(
         _level(level), a * d / (b * c), math.sqrt(1 / a + 1 / b + 1 / c)),
     phrases=("Odds ratio with confidence interval", "Odds ratio with CI"),
     examples=[_ex({"a": 20, "b": 80, "c": 10, "d": 90, "level": "95%"}), _ex({"a": 45, "b": 55, "c": 30, "d": 120, "level": 0.99})],
     level="beginner",
     extra_q=[{"q": "Odds ratio with confidence interval: a = 20, b = 0, c = 10, d = 90, level = 95%", "outcome": "INVALID_INPUT"}])


def dx_relative_risk(a, b, c, d, level):
    _table(a, b, c, d)
    z = _level(level)
    rr = (a / (a + b)) / (c / (c + d))
    se = math.sqrt(1 / a - 1 / (a + b) + 1 / c - 1 / (c + d))
    return _out(RR=rr, lower=rr * math.exp(-z * se), upper=rr * math.exp(z * se))


def _ref_dx_relative_risk(a, b, c, d, level):
    import numpy as np
    from statsmodels.stats.contingency_tables import Table2x2
    t = Table2x2(np.array([[a, b], [c, d]], dtype=float))
    lo, hi = t.riskratio_confint(alpha=1 - level, method="normal")
    return {"RR": float(t.riskratio), "lower": float(lo), "upper": float(hi)}


_add("bmed_relative_risk_ci",
     name="Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval", topic=_EP,
     purpose="The risk ratio of exposed to unexposed in a cohort 2x2 table with the log-scale (Katz) confidence interval at the stated level.",
     inputs=list(_TAB),
     outputs=[_o("RR", "number", "risk ratio [a/(a + b)]/[c/(c + d)]"), _o("lower", "number", "lower confidence limit"),
              _o("upper", "number", "upper confidence limit")],
     primary="RR", conditions=["cohort (risks estimable)", "no zero cell", "log interval exp(ln RR +- z SE)", "level stated"],
     method="RR = [a/(a + b)]/[c/(c + d)]; SE(ln RR) = sqrt(1/a - 1/(a + b) + 1/c - 1/(c + d))",
     latex=r"RR = \frac{a/(a+b)}{c/(c+d)},\quad \mathrm{SE}(\ln RR) = \sqrt{\tfrac1a - \tfrac1{a+b} + \tfrac1c - \tfrac1{c+d}}",
     run=dx_relative_risk, reference=_ref_dx_relative_risk, rand=_rand_table,
     wrong=lambda a, b, c, d, level: (lambda z, rr, se: _out(RR=rr, lower=rr * math.exp(-z * se), upper=rr * math.exp(z * se)))(
         _level(level), (a / (a + b)) / (c / (c + d)), math.sqrt(1 / a + 1 / c)),
     phrases=("Relative risk with confidence interval", "Risk ratio with confidence interval"),
     examples=[_ex({"a": 20, "b": 80, "c": 10, "d": 90, "level": "95%"}), _ex({"a": 45, "b": 55, "c": 30, "d": 120, "level": 0.9})],
     level="beginner")


def dx_nnt(events_t, n_t, events_c, n_c, level):
    _count("events_t", events_t)
    _count("events_c", events_c)
    _count("n_t", n_t, 1)
    _count("n_c", n_c, 1)
    _need(events_t <= n_t and events_c <= n_c, "events cannot exceed the group size")
    z = _level(level)
    pt, pc = events_t / n_t, events_c / n_c
    arr = pc - pt
    se = math.sqrt(pt * (1 - pt) / n_t + pc * (1 - pc) / n_c)
    lo, hi = arr - z * se, arr + z * se
    _need(lo > 0, "the interval for the risk reduction does not exclude zero (or shows harm): the NNT interval is not finite")
    return _out(ARR=arr, ARR_lower=lo, ARR_upper=hi, NNT=1 / arr, NNT_lower=1 / hi, NNT_upper=1 / lo)


def _ref_dx_nnt(events_t, n_t, events_c, n_c, level):
    from statsmodels.stats.proportion import confint_proportions_2indep
    lo, hi = confint_proportions_2indep(events_c, n_c, events_t, n_t, method="wald", compare="diff", alpha=1 - level)
    arr = events_c / n_c - events_t / n_t
    return {"ARR": arr, "ARR_lower": float(lo), "ARR_upper": float(hi), "NNT": 1 / arr, "NNT_lower": 1 / float(hi), "NNT_upper": 1 / float(lo)}


def _wrong_dx_nnt(events_t, n_t, events_c, n_c, level):
    z = _level(level)
    pt, pc = events_t / n_t, events_c / n_c
    p = (events_t + events_c) / (n_t + n_c)                    # pooled variance
    se = math.sqrt(p * (1 - p) * (1 / n_t + 1 / n_c))
    arr = pc - pt
    return _out(ARR=arr, ARR_lower=arr - z * se, ARR_upper=arr + z * se, NNT=1 / arr, NNT_lower=1 / (arr + z * se), NNT_upper=1 / (arr - z * se))


def _rand_dx_nnt(rng):
    n_t, n_c = rng.randint(80, 600), rng.randint(80, 600)
    pc = rng.uniform(0.2, 0.6)
    pt = pc - rng.uniform(0.08, 0.2)
    return {"events_t": int(round(pt * n_t)), "n_t": n_t, "events_c": int(round(pc * n_c)), "n_c": n_c, "level": rng.choice([0.9, 0.95])}


_add("bmed_nnt_ci",
     name="Absolute risk reduction and number needed to treat with Wald intervals", topic=_EP,
     purpose="The absolute risk reduction (control risk minus treated risk) of a two-arm trial with its Wald confidence interval, and the number needed to treat with the interval obtained by inverting it.",
     inputs=[_intin("events_t", "events in the treated group", 0), _intin("n_t", "size of the treated group", 1),
             _intin("events_c", "events in the control group", 0), _intin("n_c", "size of the control group", 1),
             _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("ARR", "number", "absolute risk reduction"), _o("ARR_lower", "number", "lower limit"), _o("ARR_upper", "number", "upper limit"),
              _o("NNT", "number", "number needed to treat 1/ARR"), _o("NNT_lower", "number", "1/ARR_upper"), _o("NNT_upper", "number", "1/ARR_lower")],
     primary="NNT", conditions=["unpooled Wald interval for the difference of proportions", "the interval excludes zero (a finite NNT interval)",
                                "level stated", "NNT not rounded"],
     method="ARR = pc - pt; SE = sqrt(pt(1 - pt)/nt + pc(1 - pc)/nc); NNT = 1/ARR with limits 1/upper, 1/lower",
     latex=r"ARR = p_c - p_t,\quad NNT = \frac{1}{ARR},\quad \mathrm{SE} = \sqrt{\tfrac{p_t(1-p_t)}{n_t} + \tfrac{p_c(1-p_c)}{n_c}}",
     run=dx_nnt, reference=_ref_dx_nnt, wrong=_wrong_dx_nnt, rand=_rand_dx_nnt,
     phrases=("Number needed to treat", "NNT with confidence interval"),
     examples=[_ex({"events_t": 15, "n_t": 100, "events_c": 30, "n_c": 100, "level": "95%"}),
               _ex({"events_t": 40, "n_t": 400, "events_c": 80, "n_c": 410, "level": 0.9})],
     level="intermediate",
     extra_q=[{"q": "Number needed to treat: events_t = 28, n_t = 100, events_c = 30, n_c = 100, level = 95%", "outcome": "INVALID_INPUT"}])


def dx_irr(events1, time1, events2, time2, level):
    _count("events1", events1, 1)
    _count("events2", events2, 1)
    _pos("the person-times", time1, time2)
    z = _level(level)
    irr = (events1 / time1) / (events2 / time2)
    se = math.sqrt(1 / events1 + 1 / events2)
    return _out(IRR=irr, lower=irr * math.exp(-z * se), upper=irr * math.exp(z * se))


def _ref_dx_irr(events1, time1, events2, time2, level):
    import numpy as np
    import statsmodels.api as sm
    from statsmodels.stats import rates
    lo, hi = rates.confint_poisson_2indep(events1, time1, events2, time2, method="wald-log", compare="ratio", alpha=1 - level)
    X = np.array([[1.0, 1.0], [1.0, 0.0]])
    mdl = sm.GLM(np.array([events1, events2], dtype=float), X, family=sm.families.Poisson(),
                 offset=np.log(np.array([time1, time2], dtype=float)))
    fit = mdl.fit(tol=1e-15)
    fit = mdl.fit(start_params=fit.params, tol=1e-15)
    return {"IRR": math.exp(float(fit.params[1])), "lower": float(lo), "upper": float(hi)}


_add("bmed_incidence_rate_ratio_ci",
     name="Incidence rate ratio of two person-time groups with log-Wald interval", topic=_EP,
     purpose="The ratio of the incidence rates (events per person-time) of two groups with the log-scale Wald confidence interval at the stated level.",
     inputs=[_intin("events1", "events in group 1", 1), _numin("time1", "person-time of group 1 (the same unit for both groups)"),
             _intin("events2", "events in group 2 (reference)", 1), _numin("time2", "person-time of group 2"),
             _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("IRR", "number", "incidence rate ratio group 1 / group 2"), _o("lower", "number", "lower limit"), _o("upper", "number", "upper limit")],
     primary="IRR", conditions=["Poisson counts", "log-Wald interval", "the same person-time unit for both groups", "level stated"],
     method="IRR = (e1/T1)/(e2/T2); SE(ln IRR) = sqrt(1/e1 + 1/e2)",
     latex=r"IRR = \frac{e_1/T_1}{e_2/T_2},\quad \mathrm{SE}(\ln IRR) = \sqrt{\tfrac1{e_1} + \tfrac1{e_2}}",
     run=dx_irr, reference=_ref_dx_irr,
     rand=lambda rng: {"events1": rng.randint(1, 200), "time1": _sig(_lu(rng, 100, 1e5)), "events2": rng.randint(1, 200),
                       "time2": _sig(_lu(rng, 100, 1e5)), "level": rng.choice([0.9, 0.95, 0.99])},
     wrong=lambda events1, time1, events2, time2, level: (lambda z, irr, se: _out(IRR=irr, lower=irr * math.exp(-z * se), upper=irr * math.exp(z * se)))(
         _level(level), (events1 / time1) / (events2 / time2), math.sqrt(1 / (events1 + events2))),
     phrases=("Incidence rate ratio", "Rate ratio with confidence interval"),
     examples=[_ex({"events1": 30, "time1": 1000, "events2": 15, "time2": 1200, "level": "95%"}),
               _ex({"events1": 12, "time1": 2500.5, "events2": 40, "time2": 4100, "level": 0.9})],
     level="intermediate")


def _pois_cdf(k, lam):
    """P(X <= k) for X ~ Poisson(lam), summed in log space."""
    if lam == 0:
        return 1.0
    ll = math.log(lam)
    terms = [-lam + j * ll - math.lgamma(j + 1) for j in range(k + 1)]
    mx = max(terms)
    return min(1.0, math.exp(mx) * math.fsum(math.exp(t - mx) for t in terms))


def dx_smr(observed, expected, level):
    _count("observed", observed)
    _need(observed <= 1000, "observed counts above 1000 are not supported")
    _pos("the expected count", expected)
    _level(level)
    a = (1 - level) / 2
    hi_b = observed + 20.0 * math.sqrt(observed + 1.0) + 50.0
    if observed == 0:
        lo = 0.0
    else:
        lo = _bisect(lambda lam: (1.0 - _pois_cdf(observed - 1, lam)) - a, 0.0, hi_b, 200)
    up = _bisect(lambda lam: _pois_cdf(observed, lam) - a, 0.0, hi_b, 200)
    return _out(SMR=observed / expected, lower=lo / expected, upper=up / expected)


def _ref_dx_smr(observed, expected, level):
    from scipy.stats import chi2
    a = (1 - level) / 2
    lo = chi2.ppf(a, 2 * observed) / 2 if observed > 0 else 0.0
    up = chi2.ppf(1 - a, 2 * observed + 2) / 2
    return {"SMR": observed / expected, "lower": float(lo) / expected, "upper": float(up) / expected}


def _wrong_dx_smr(observed, expected, level):
    r = dx_smr(observed, expected, level)
    a = (1 - level) / 2
    hi_b = observed + 20.0 * math.sqrt(observed + 1.0) + 50.0
    up = _bisect(lambda lam: _pois_cdf(max(observed - 1, 0), lam) - a, 0.0, hi_b, 200)   # 2 O degrees of freedom
    return dict(r, upper=_r(up / expected))


_add("bmed_smr_exact_ci",
     name="Standardized mortality (incidence) ratio with exact Poisson interval", topic=_EP,
     purpose="The standardized mortality or incidence ratio observed/expected with the exact (Garwood) Poisson confidence interval at the stated level.",
     inputs=[_intin("observed", "observed number of deaths or cases (0 to 1000)", 0, 1000),
             _numin("expected", "expected number from reference rates"), _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("SMR", "number", "observed / expected"), _o("lower", "number", "lower exact limit"), _o("upper", "number", "upper exact limit")],
     primary="SMR", conditions=["observed count Poisson", "expected count treated as known", "exact (Garwood) limits", "level stated"],
     method="limits lambda with P(X >= O; lambda) = alpha/2 and P(X <= O; lambda) = alpha/2 (bisection on the Poisson sums), divided by E",
     latex=r"\mathrm{SMR} = \frac{O}{E},\quad P(X \ge O;\lambda_L) = \tfrac{\alpha}{2},\ P(X \le O;\lambda_U) = \tfrac{\alpha}{2}",
     run=dx_smr, reference=_ref_dx_smr,
     rand=lambda rng: {"observed": rng.randint(0, 300), "expected": _sig(_lu(rng, 0.5, 300)), "level": rng.choice([0.9, 0.95, 0.99])},
     wrong=lambda observed, expected, level: _wrong_dx_smr(observed, expected, level),
     phrases=("Standardized mortality ratio", "SMR with exact interval"),
     examples=[_ex({"observed": 25, "expected": 18.4, "level": "95%"}), _ex({"observed": 3, "expected": 7.5, "level": 0.9})],
     level="advanced")


def dx_mantel_haenszel(tables, level):
    _need(len(tables) >= 1 and all(len(r) == 4 for r in tables), "give one row [a, b, c, d] per stratum")
    for r in tables:
        for v in r:
            _need(v >= 0 and float(v) == int(v), "cell counts are whole numbers >= 0")
        _need(sum(r) > 0, "a stratum is empty")
    z = _level(level)
    R = S = sPR = sPSQR = sQS = 0.0
    parts = []
    for a, b, c, d in tables:
        n = a + b + c + d
        Ri, Si = a * d / n, b * c / n
        Pi, Qi = (a + d) / n, (b + c) / n
        parts.append((Ri, Si, Pi, Qi))
    R = math.fsum(p[0] for p in parts)
    S = math.fsum(p[1] for p in parts)
    _need(R > 0 and S > 0, "the pooled odds ratio is zero or infinite")
    var = (math.fsum(p[2] * p[0] for p in parts) / (2 * R * R)
           + math.fsum(p[2] * p[1] + p[3] * p[0] for p in parts) / (2 * R * S)
           + math.fsum(p[3] * p[1] for p in parts) / (2 * S * S))
    orr = R / S
    se = math.sqrt(var)
    return _out(OR_MH=orr, lower=orr * math.exp(-z * se), upper=orr * math.exp(z * se))


def _ref_dx_mantel_haenszel(tables, level):
    import numpy as np
    from statsmodels.stats.contingency_tables import StratifiedTable
    st = StratifiedTable([np.array([[a, b], [c, d]], dtype=float) for a, b, c, d in tables])
    lo, hi = st.oddsratio_pooled_confint(alpha=1 - level)
    return {"OR_MH": float(st.oddsratio_pooled), "lower": float(lo), "upper": float(hi)}


def _wrong_dx_mh(tables, level):
    r = dx_mantel_haenszel(tables, level)
    A, B, C, D = (sum(t[i] for t in tables) for i in range(4))
    return dict(r, OR_MH=_r(A * D / (B * C)))                    # the crude (collapsed) odds ratio


def _rand_dx_mh(rng):
    K = rng.randint(2, 6)
    return {"tables": [[float(rng.randint(1, 60)), float(rng.randint(1, 90)), float(rng.randint(1, 60)), float(rng.randint(1, 90))] for _ in range(K)],
            "level": rng.choice([0.9, 0.95, 0.99])}


_add("bmed_mantel_haenszel_odds_ratio",
     name="Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval", topic=_EP,
     purpose="The Mantel-Haenszel odds ratio pooled over strata of 2x2 tables (adjusting for a confounder), with the Robins-Breslow-Greenland confidence interval at the stated level.",
     inputs=[_matin("tables", "one row [a, b, c, d] per stratum: exposed cases, exposed non-cases, unexposed cases, unexposed non-cases"),
             _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("OR_MH", "number", "Mantel-Haenszel pooled odds ratio"), _o("lower", "number", "lower limit"), _o("upper", "number", "upper limit")],
     primary="OR_MH", conditions=["a common odds ratio across strata", "Robins-Breslow-Greenland variance", "level stated"],
     method="OR_MH = sum(a d / n) / sum(b c / n); variance of ln OR_MH by Robins, Breslow and Greenland",
     latex=r"OR_{MH} = \frac{\sum_i a_i d_i / n_i}{\sum_i b_i c_i / n_i}",
     run=dx_mantel_haenszel, reference=_ref_dx_mantel_haenszel, wrong=_wrong_dx_mh, rand=_rand_dx_mh,
     phrases=("Mantel-Haenszel odds ratio", "Mantel-Haenszel pooled odds ratio"),
     examples=[_ex({"tables": [[10, 20, 5, 30], [8, 12, 6, 20]], "level": "95%"}),
               _ex({"tables": [[25, 75, 10, 90], [40, 60, 20, 80], [5, 45, 2, 48]], "level": 0.9})],
     level="advanced")


def dx_kappa(counts):
    k = len(counts)
    _need(2 <= k <= 8 and all(len(r) == k for r in counts), "give a square k x k table of counts (2 to 8 categories)")
    for r in counts:
        for v in r:
            _need(v >= 0 and float(v) == int(v), "counts are whole numbers >= 0")
    n = sum(sum(r) for r in counts)
    _need(n > 0, "the table is empty")
    po = sum(counts[i][i] for i in range(k)) / n
    pe = math.fsum(sum(counts[i]) * sum(counts[j][i] for j in range(k)) for i in range(k)) / (n * n)
    _need(pe < 1, "chance agreement is 1: kappa is undefined")
    return _out(kappa=(po - pe) / (1 - pe), p_observed=po)


def _ref_dx_kappa(counts):
    from sklearn.metrics import accuracy_score, cohen_kappa_score
    a, b = [], []
    for i, row in enumerate(counts):
        for j, v in enumerate(row):
            a += [i] * int(v)
            b += [j] * int(v)
    return {"kappa": float(cohen_kappa_score(a, b)), "p_observed": float(accuracy_score(a, b))}


def _wrong_dx_kappa(counts):
    k = len(counts)
    n = sum(sum(r) for r in counts)
    po = sum(counts[i][i] for i in range(k)) / n
    pe = math.fsum(sum(counts[i]) ** 2 for i in range(k)) / (n * n)          # row margins only
    return _out(kappa=(po - pe) / (1 - pe), p_observed=po)


def _rand_dx_kappa(rng):
    k = rng.randint(2, 5)
    return {"counts": [[float(rng.randint(5, 60) if i == j else rng.randint(0, 15)) for j in range(k)] for i in range(k)]}


_add("bmed_cohens_kappa",
     name="Cohen's kappa for agreement between two raters or two tests", topic=_DX,
     purpose="Cohen's kappa (chance-corrected agreement) and the observed agreement of two raters or two diagnostic tests classifying the same subjects, from their k x k agreement table.",
     inputs=[_matin("counts", "agreement table: rows = categories given by rater A, columns = categories given by rater B")],
     outputs=[_o("kappa", "number", "Cohen's kappa"), _o("p_observed", "number", "observed proportion of agreement")],
     primary="kappa", conditions=["the same subjects rated by both", "unweighted kappa", "the same categories in rows and columns"],
     method="kappa = (p_o - p_e)/(1 - p_e), p_e = sum of products of row and column margins / n^2",
     latex=r"\kappa = \frac{p_o - p_e}{1 - p_e},\quad p_e = \sum_i \frac{n_{i\cdot}\,n_{\cdot i}}{n^2}",
     run=dx_kappa, reference=_ref_dx_kappa, wrong=_wrong_dx_kappa, rand=_rand_dx_kappa,
     phrases=("Cohen's kappa", "Cohen kappa"),
     examples=[_ex({"counts": [[40, 10], [5, 45]]}), _ex({"counts": [[20, 5, 1], [4, 30, 6], [0, 3, 31]]})],
     level="intermediate")


# ================================================================================================ SURVIVAL ANALYSIS
_SV = "survival analysis"


def _surv(times, events, label="times"):
    _need(len(times) == len(events) and len(times) >= 1, f"give one event indicator per time ({label})")
    for v in times:
        _need(math.isfinite(v) and v >= 0, "times must be finite and >= 0")
    for e in events:
        _need(e in (0, 1), "event indicators are 1 (event) or 0 (censored)")
    return [float(v) for v in times], [int(e) for e in events]


def _km_steps(T, E):
    """[(event time, n at risk, deaths)] in time order."""
    out = []
    for u in sorted(set(t for t, e in zip(T, E) if e == 1)):
        out.append((u, sum(1 for t in T if t >= u), sum(1 for t, e in zip(T, E) if t == u and e == 1)))
    return out


def _km_curve(T, E):
    from fractions import Fraction as Fr
    s, curve = Fr(1), []
    for u, n, d in _km_steps(T, E):
        s *= Fr(n - d, n)
        curve.append((u, s, n, d))
    return curve


def sv_km(times, events, t):
    T, E = _surv(times, events)
    _nonneg("the time t", t)
    curve = _km_curve(T, E)
    st = 1
    med = None
    for u, s, n, d in curve:
        if u <= t:
            st = s
        if med is None and s <= Fraction_half():
            med = u
    return _out(S_t=float(st), median=med)


def Fraction_half():
    from fractions import Fraction as Fr
    return Fr(1, 2)


def _ref_sv_km(times, events, t):
    from lifelines import KaplanMeierFitter
    k = KaplanMeierFitter().fit(times, event_observed=events)
    sf = k.survival_function_.iloc[:, 0]
    below = [float(i) for i, v in sf.items() if v <= 0.5 + 1e-12]
    return {"S_t": float(k.survival_function_at_times(t).iloc[0]), "median": min(below) if below else None}


def _rand_surv(rng, n=None):
    n = n or rng.randint(5, 40)
    T = [float(rng.randint(1, 40)) for _ in range(n)]
    E = [1 if rng.random() < 0.7 else 0 for _ in range(n)]
    return T, E


def _wrong_sv_km(times, events, t):
    T = [v for v, e in zip(times, events) if e == 1]           # censored subjects dropped
    E = [1] * len(T)
    return sv_km(T, E, t) if T else {"S_t": -1.0}


_add("bmed_km_survival_median",
     name="Kaplan-Meier survival estimate at a time and median survival", topic=_SV,
     purpose="The Kaplan-Meier (product-limit) estimate of the survival probability at time t and the median survival time from follow-up times with event or censoring indicators.",
     inputs=[_vecin("times", "follow-up times (one time unit; results in that unit)", 1),
             _vecin("events", "1 = event observed, 0 = censored, one per time", 1), _numin("t", "time at which S(t) is reported")],
     outputs=[_o("S_t", "number", "Kaplan-Meier survival probability at t"),
              _o("median", "number", "median survival: first time with S <= 0.5 (none when not reached)")],
     primary="S_t", conditions=["non-informative censoring", "a censoring at an event time counts as at risk at that time"],
     method="S(t) = product over event times u <= t of (1 - d/n), exact fractions; median = first event time with S <= 1/2",
     latex=r"\hat S(t) = \prod_{u_i \le t}\left(1 - \frac{d_i}{n_i}\right)",
     run=sv_km, reference=_ref_sv_km, wrong=_wrong_sv_km,
     rand=lambda rng: (lambda te: {"times": te[0], "events": te[1], "t": float(rng.randint(0, 45))})(_rand_surv(rng)),
     phrases=("Kaplan-Meier survival", "Kaplan-Meier estimate"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "t": 12}),
               _ex({"times": [2, 3, 3, 5, 8, 9, 12, 15], "events": [1, 0, 1, 1, 0, 1, 1, 0], "t": 6})],
     level="beginner",
     extra_q=[{"q": "Kaplan-Meier survival: times = [6, 7, 10], events = [1, 2, 1], t = 8", "outcome": "INVALID_INPUT"}])


def sv_km_ci(times, events, t, level):
    T, E = _surv(times, events)
    _nonneg("the time t", t)
    z = _level(level)
    s, V, seen = 1.0, 0.0, False
    for u, n, d in _km_steps(T, E):
        if u > t:
            break
        _need(d < n, "the survival estimate reaches 0 by t: the log(-log) interval is undefined")
        s *= (n - d) / n
        V += d / (n * (n - d))
        seen = True
    _need(seen, "no event at or before t: S(t) = 1 and the interval is undefined")
    ls = math.log(s)
    se = math.sqrt(V) / abs(ls)
    return _out(S_t=s, lower=s ** math.exp(z * se), upper=s ** math.exp(-z * se))


def _ref_sv_km_ci(times, events, t, level):
    from lifelines import KaplanMeierFitter
    k = KaplanMeierFitter(alpha=1 - level).fit(times, event_observed=events)
    ci = k.confidence_interval_
    idx = [i for i in ci.index if i <= t][-1]
    return {"S_t": float(k.survival_function_at_times(t).iloc[0]), "lower": float(ci.loc[idx].iloc[0]), "upper": float(ci.loc[idx].iloc[1])}


def _wrong_sv_km_ci(times, events, t, level):
    r = sv_km_ci(times, events, t, level)
    T, E = _surv(times, events)
    z = _level(level)
    V = math.fsum(d / (n * (n - d)) for u, n, d in _km_steps(T, E) if u <= t)
    se = r["S_t"] * math.sqrt(V)                                   # plain (linear) Greenwood interval
    return dict(r, lower=_r(r["S_t"] - z * se), upper=_r(r["S_t"] + z * se))


_add("bmed_km_confidence_interval",
     name="Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval", topic=_SV,
     purpose="The Kaplan-Meier survival estimate at time t with its confidence interval on the log(-log) scale (Greenwood variance; the exponential Greenwood interval) at the stated level.",
     inputs=[_vecin("times", "follow-up times (one time unit)", 1), _vecin("events", "1 = event, 0 = censored", 1),
             _numin("t", "time at which S(t) is reported"), _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("S_t", "number", "Kaplan-Meier estimate at t"), _o("lower", "number", "lower confidence limit"),
              _o("upper", "number", "upper confidence limit")],
     primary="S_t", conditions=["non-informative censoring", "an event at or before t and S(t) > 0", "level stated"],
     method="Greenwood sum V = sum d/(n(n - d)); limits S^exp(+-z sqrt(V)/|ln S|)",
     latex=r"\hat S^{\exp\left(\pm z\,\sqrt{V}/|\ln\hat S|\right)},\quad V = \sum_{u_i \le t}\frac{d_i}{n_i(n_i - d_i)}",
     run=sv_km_ci, reference=_ref_sv_km_ci, wrong=_wrong_sv_km_ci,
     rand=lambda rng: (lambda te: {"times": te[0], "events": te[1], "t": float(rng.randint(5, 30)), "level": rng.choice([0.9, 0.95, 0.99])})(_rand_surv(rng)),
     phrases=("Kaplan-Meier confidence interval", "Kaplan-Meier interval"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "t": 12, "level": "95%"}),
               _ex({"times": [2, 3, 3, 5, 8, 9, 12, 15], "events": [1, 0, 1, 1, 0, 1, 1, 0], "t": 8, "level": 0.9})],
     level="advanced",
     extra_q=[{"q": "Kaplan-Meier confidence interval: times = [2, 3, 5], events = [1, 1, 1], t = 1, level = 95%", "outcome": "INVALID_INPUT"}])


def sv_nelson_aalen(times, events, t):
    T, E = _surv(times, events)
    _nonneg("the time t", t)
    H = math.fsum(d / n for u, n, d in _km_steps(T, E) if u <= t)
    return _out(H_t=H)


def _ref_sv_nelson_aalen(times, events, t):
    from lifelines import NelsonAalenFitter
    na = NelsonAalenFitter(nelson_aalen_smoothing=False).fit(times, event_observed=events)
    return {"H_t": float(na.cumulative_hazard_at_times(t).iloc[0])}


_add("bmed_nelson_aalen_hazard",
     name="Nelson-Aalen cumulative hazard at a time", topic=_SV,
     purpose="The Nelson-Aalen estimate of the cumulative hazard at time t from follow-up times with event or censoring indicators.",
     inputs=[_vecin("times", "follow-up times (one time unit)", 1), _vecin("events", "1 = event, 0 = censored", 1),
             _numin("t", "time at which H(t) is reported")],
     outputs=[_o("H_t", "number", "cumulative hazard at t")],
     primary="H_t", conditions=["non-informative censoring", "tied events contribute d/n (no tie smoothing)"],
     method="H(t) = sum over event times u <= t of d/n", latex=r"\hat H(t) = \sum_{u_i \le t}\frac{d_i}{n_i}",
     run=sv_nelson_aalen, reference=_ref_sv_nelson_aalen,
     rand=lambda rng: (lambda te: {"times": te[0], "events": te[1], "t": float(rng.randint(1, 45))})(_rand_surv(rng)),
     wrong=lambda times, events, t: (lambda T, E: _out(H_t=math.fsum(d / (n - d) if n > d else 0.0 for u, n, d in _km_steps(T, E) if u <= t)))(*_surv(times, events)),
     phrases=("Nelson-Aalen cumulative hazard", "Nelson-Aalen hazard"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "t": 12}),
               _ex({"times": [2, 3, 3, 5, 8, 9, 12, 15], "events": [1, 0, 1, 1, 0, 1, 1, 0], "t": 10})],
     level="intermediate")


def sv_logrank(times1, events1, times2, events2):
    T1, E1 = _surv(times1, events1, "group 1")
    T2, E2 = _surv(times2, events2, "group 2")
    o1 = e1 = v = 0.0
    for u in sorted(set(t for t, e in zip(T1 + T2, E1 + E2) if e == 1)):
        n1, n2 = sum(1 for t in T1 if t >= u), sum(1 for t in T2 if t >= u)
        d1 = sum(1 for t, e in zip(T1, E1) if t == u and e == 1)
        d = d1 + sum(1 for t, e in zip(T2, E2) if t == u and e == 1)
        n = n1 + n2
        o1 += d1
        e1 += d * n1 / n
        if n > 1:
            v += d * (n1 / n) * (n2 / n) * (n - d) / (n - 1)
    _need(v > 0, "the log-rank variance is zero (no informative event times)")
    chi2 = (o1 - e1) ** 2 / v
    return _out(chi_square=chi2, p_value=math.erfc(math.sqrt(chi2 / 2.0)))


def _ref_sv_logrank(times1, events1, times2, events2):
    from lifelines.statistics import logrank_test
    r = logrank_test(times1, times2, event_observed_A=events1, event_observed_B=events2)
    return {"chi_square": float(r.test_statistic), "p_value": float(r.p_value)}


def _rand_sv_two(rng):
    a, b = _rand_surv(rng), _rand_surv(rng)
    b = ([float(min(60, v + rng.randint(0, 8))) for v in b[0]], b[1])
    return {"times1": a[0], "events1": a[1], "times2": b[0], "events2": b[1]}


_add("bmed_logrank_test",
     name="Log-rank test comparing two survival curves", topic=_SV,
     purpose="The log-rank (Mantel-Cox) chi-square statistic and p-value for equal survival in two groups from follow-up times and event indicators.",
     inputs=[_vecin("times1", "follow-up times, group 1", 1), _vecin("events1", "1 = event, 0 = censored, group 1", 1),
             _vecin("times2", "follow-up times, group 2", 1), _vecin("events2", "1 = event, 0 = censored, group 2", 1)],
     outputs=[_o("chi_square", "number", "log-rank chi-square (1 degree of freedom)"), _o("p_value", "number", "two-sided p-value")],
     primary="chi_square", conditions=["non-informative censoring", "hypergeometric variance at each event time"],
     method="at each event time: expected events of group 1 and the hypergeometric variance; chi2 = (O1 - E1)^2 / V",
     latex=r"\chi^2 = \frac{\left(\sum_i (d_{1i} - e_{1i})\right)^2}{\sum_i v_i}",
     run=sv_logrank, reference=_ref_sv_logrank, rand=_rand_sv_two,
     wrong=lambda times1, events1, times2, events2: (lambda r: dict(r, p_value=_r(math.erfc(math.sqrt(r["chi_square"])))))(sv_logrank(times1, events1, times2, events2)),
     phrases=("Log-rank test", "Logrank test"),
     examples=[_ex({"times1": [6, 6, 6, 7, 10, 13, 16, 22, 23], "events1": [1, 1, 1, 1, 1, 1, 1, 1, 1],
                    "times2": [1, 1, 2, 2, 3, 4, 4, 5, 5, 8], "events2": [1, 1, 1, 1, 1, 1, 1, 1, 1, 1]}),
               _ex({"times1": [5, 8, 12, 20, 25], "events1": [1, 1, 0, 1, 0], "times2": [3, 4, 9, 10, 14], "events2": [1, 1, 1, 0, 1]})],
     level="advanced")


def _cox_terms(T, E, x, beta):
    ll = U = I = 0.0
    w = [math.exp(beta * v) for v in x]
    for u in sorted(set(t for t, e in zip(T, E) if e == 1)):
        R = [j for j, t in enumerate(T) if t >= u]
        D = [j for j, (t, e) in enumerate(zip(T, E)) if t == u and e == 1]
        d = len(D)
        s0r = math.fsum(w[j] for j in R)
        s1r = math.fsum(w[j] * x[j] for j in R)
        s2r = math.fsum(w[j] * x[j] * x[j] for j in R)
        s0d = math.fsum(w[j] for j in D)
        s1d = math.fsum(w[j] * x[j] for j in D)
        s2d = math.fsum(w[j] * x[j] * x[j] for j in D)
        ll += beta * math.fsum(x[j] for j in D)
        U += math.fsum(x[j] for j in D)
        for l in range(d):
            f = l / d
            a0, a1, a2 = s0r - f * s0d, s1r - f * s1d, s2r - f * s2d
            ll -= math.log(a0)
            U -= a1 / a0
            I += a2 / a0 - (a1 / a0) ** 2
    return ll, U, I


def sv_cox(times, events, x, level):
    T, E = _surv(times, events)
    _need(len(x) == len(T), "give one covariate value per subject")
    for v in x:
        _fin("every covariate value", v)
    _need(sum(E) >= 2, "at least two events are needed")
    _need(len(set(x)) >= 2, "the covariate does not vary")
    z = _level(level)
    m = math.fsum(x) / len(x)
    xc = [v - m for v in x]
    beta = 0.0
    ll, U, I = _cox_terms(T, E, xc, beta)
    for _ in range(200):
        _need(I > 0, "the partial likelihood is flat: the hazard ratio is not estimable")
        step = U / I
        nb = beta + step
        nll, nU, nI = _cox_terms(T, E, xc, nb)
        halv = 0
        while nll < ll - 1e-12 * abs(ll) and halv < 60:
            step /= 2.0
            nb = beta + step
            nll, nU, nI = _cox_terms(T, E, xc, nb)
            halv += 1
        _need(abs(nb) < 25, "the estimate diverges (monotone likelihood): the hazard ratio is not finite")
        done = abs(nb - beta) <= 1e-14 * max(1.0, abs(nb))
        beta, ll, U, I = nb, nll, nU, nI
        if done:
            break
    se = 1.0 / math.sqrt(I)
    return _out(beta=beta, HR=math.exp(beta), se=se, HR_lower=math.exp(beta - z * se), HR_upper=math.exp(beta + z * se))


def _ref_sv_cox(times, events, x, level):
    # statsmodels PHReg with Efron ties (lifelines' optimiser stops about 1e-8 short of the maximum, so it is not used);
    # the estimate is finished with Newton steps on statsmodels' own score and Hessian
    import numpy as np
    from scipy.stats import norm
    from statsmodels.duration.hazard_regression import PHReg
    m = PHReg(np.array(times, dtype=float), np.array(x, dtype=float)[:, None], status=np.array(events, dtype=int), ties="efron")
    b = np.array([float(m.fit().params[0])])
    for _ in range(4):
        b = b - m.score(b) / m.hessian(b)[0, 0]
    se = float(np.sqrt(-1.0 / m.hessian(b)[0, 0]))
    z = float(norm.ppf(0.5 + level / 2.0))
    b = float(b[0])
    return {"beta": b, "HR": math.exp(b), "se": se, "HR_lower": math.exp(b - z * se), "HR_upper": math.exp(b + z * se)}


def _rand_sv_cox(rng):
    n = rng.randint(15, 45)
    binary = rng.random() < 0.6
    x = [float(rng.randint(0, 1)) if binary else round(rng.gauss(0, 1), 1) for _ in range(n)]
    b = rng.uniform(-1.2, 1.2)
    T = [float(max(1, math.ceil(rng.expovariate(0.1 * math.exp(b * v))))) for v in x]
    E = [1 if rng.random() < 0.75 else 0 for _ in range(n)]
    return {"times": T, "events": E, "x": x, "level": rng.choice([0.9, 0.95])}


_add("bmed_cox_hazard_ratio",
     name="Cox proportional-hazards model with one covariate: hazard ratio (Efron ties)", topic=_SV,
     purpose="The log hazard ratio, hazard ratio, its standard error and Wald confidence interval of a Cox proportional-hazards model with a single covariate, by maximum partial likelihood with Efron's handling of ties.",
     inputs=[_vecin("times", "follow-up times", 2), _vecin("events", "1 = event, 0 = censored", 2),
             _vecin("x", "covariate value of each subject (e.g. 1 = treated, 0 = control)", 2),
             _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("beta", "number", "log hazard ratio per unit of x"), _o("HR", "number", "hazard ratio exp(beta)"),
              _o("se", "number", "standard error of beta"), _o("HR_lower", "number", "lower Wald limit of the hazard ratio"),
              _o("HR_upper", "number", "upper Wald limit of the hazard ratio")],
     primary="HR", conditions=["proportional hazards", "Efron approximation for tied event times", "a finite maximum (no monotone likelihood)",
                               "level stated"],
     method="Newton-Raphson on the Efron partial log-likelihood (step halving); SE from the observed information",
     latex=r"\ell(\beta) = \sum_j\left[\beta\sum_{i\in D_j}x_i - \sum_{l=0}^{d_j-1}\ln\left(\sum_{R_j}e^{\beta x} - \tfrac{l}{d_j}\sum_{D_j}e^{\beta x}\right)\right]",
     run=sv_cox, reference=_ref_sv_cox, rand=_rand_sv_cox,
     wrong=lambda times, events, x, level: (lambda r: dict(r, HR=_r(math.exp(-r["beta"]))))(sv_cox(times, events, x, level)),
     phrases=("Cox hazard ratio", "Cox proportional hazards"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                    "x": [1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1], "level": "95%"}),
               _ex({"times": [2, 3, 3, 5, 8, 9, 12, 15, 4, 7, 11, 14, 18, 21], "events": [1, 0, 1, 1, 0, 1, 1, 0, 1, 1, 1, 0, 1, 0],
                    "x": [1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0], "level": 0.9})],
     level="expert")


def _exp_rate(T, E):
    _need(sum(E) >= 1, "at least one event is needed")
    _need(all(t > 0 for t in T), "times must be positive")
    return sum(E) / math.fsum(T)


def sv_exponential(times, events):
    T, E = _surv(times, events)
    lam = _exp_rate(T, E)
    return _out(hazard=lam, mean=1.0 / lam, median=LN2 / lam)


def _ref_sv_exponential(times, events):
    import numpy as np
    import statsmodels.api as sm
    mdl = sm.GLM(np.array(events, dtype=float), np.ones((len(times), 1)), family=sm.families.Poisson(),
                 offset=np.log(np.array(times, dtype=float)))
    fit = mdl.fit(tol=1e-15)
    fit = mdl.fit(start_params=fit.params, tol=1e-15)
    lam = math.exp(float(fit.params[0]))
    mean = _quad(lambda s: math.exp(-lam * s), 0.0, 60.0 / lam)            # mean = area under S(t)
    med = _brentq(lambda s: math.exp(-lam * s) - 0.5, 0.0, 100.0 / lam)
    return {"hazard": lam, "mean": mean, "median": med}


_add("bmed_exponential_survival_fit",
     name="Exponential survival model: hazard rate, mean and median survival (maximum likelihood)", topic=_SV,
     purpose="The maximum-likelihood constant hazard of an exponential survival model from censored follow-up data, with the implied mean and median survival times.",
     inputs=[_vecin("times", "follow-up times, all positive (one time unit)", 1), _vecin("events", "1 = event, 0 = censored", 1)],
     outputs=[_o("hazard", "number", "events per unit time"), _o("mean", "number", "mean survival 1/hazard"),
              _o("median", "number", "median survival ln 2 / hazard")],
     primary="hazard", conditions=["constant hazard", "non-informative censoring", "at least one event"],
     method="hazard = (number of events) / (total follow-up time)", latex=r"\hat\lambda = \frac{d}{\sum_i t_i},\quad \tilde t = \frac{\ln 2}{\hat\lambda}",
     run=sv_exponential, reference=_ref_sv_exponential,
     rand=lambda rng: (lambda te: {"times": te[0], "events": te[1] if sum(te[1]) else [1] + te[1][1:]})(_rand_surv(rng)),
     wrong=lambda times, events: (lambda lam: _out(hazard=lam, mean=1 / lam, median=LN2 / lam))(
         sum(events) / math.fsum(t for t, e in zip(times, events) if e == 1)),
     phrases=("Exponential survival fit", "Exponential survival model"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]}),
               _ex({"times": [2.5, 3.1, 4.0, 7.2, 8.8], "events": [1, 1, 0, 1, 0]})],
     level="intermediate")


def sv_exp_hr(times1, events1, times2, events2, level):
    T1, E1 = _surv(times1, events1, "group 1")
    T2, E2 = _surv(times2, events2, "group 2")
    l1, l2 = _exp_rate(T1, E1), _exp_rate(T2, E2)
    z = _level(level)
    hr = l1 / l2
    se = math.sqrt(1.0 / sum(E1) + 1.0 / sum(E2))
    return _out(HR=hr, lower=hr * math.exp(-z * se), upper=hr * math.exp(z * se))


def _ref_sv_exp_hr(times1, events1, times2, events2, level):
    import numpy as np
    import statsmodels.api as sm
    y = np.array(list(events1) + list(events2), dtype=float)
    X = np.column_stack([np.ones(len(y)), [1.0] * len(times1) + [0.0] * len(times2)])
    mdl = sm.GLM(y, X, family=sm.families.Poisson(), offset=np.log(np.array(list(times1) + list(times2), dtype=float)))
    fit = mdl.fit(tol=1e-15)
    fit = mdl.fit(start_params=fit.params, tol=1e-15)            # covariance from weights at the converged estimate
    lo, hi = fit.conf_int(alpha=1 - level)[1]
    return {"HR": math.exp(float(fit.params[1])), "lower": math.exp(float(lo)), "upper": math.exp(float(hi))}


def _rand_sv_exp_hr(rng):
    r = _rand_sv_two(rng)
    for k in ("events1", "events2"):
        if sum(r[k]) == 0:
            r[k][0] = 1
    return dict(r, level=rng.choice([0.9, 0.95, 0.99]))


_add("bmed_exponential_hazard_ratio",
     name="Hazard ratio of two exponential survival groups with Wald interval", topic=_SV,
     purpose="The ratio of the maximum-likelihood constant hazards of two groups (group 1 relative to group 2) with the log-scale Wald confidence interval.",
     inputs=[_vecin("times1", "follow-up times, group 1 (positive)", 1), _vecin("events1", "1 = event, 0 = censored, group 1", 1),
             _vecin("times2", "follow-up times, group 2 (reference)", 1), _vecin("events2", "1 = event, 0 = censored, group 2", 1),
             _numin("level", "confidence level (95% or 0.95)", pct=True)],
     outputs=[_o("HR", "number", "hazard ratio group 1 / group 2"), _o("lower", "number", "lower limit"), _o("upper", "number", "upper limit")],
     primary="HR", conditions=["constant hazard in each group", "at least one event per group", "level stated"],
     method="hazard_g = d_g / sum t_g; SE(ln HR) = sqrt(1/d1 + 1/d2)", latex=r"HR = \frac{d_1/\sum t_{1}}{d_2/\sum t_{2}},\quad \mathrm{SE}(\ln HR) = \sqrt{\tfrac1{d_1} + \tfrac1{d_2}}",
     run=sv_exp_hr, reference=_ref_sv_exp_hr, rand=_rand_sv_exp_hr,
     wrong=lambda times1, events1, times2, events2, level: (lambda r, z: dict(r, lower=_r(r["HR"] * math.exp(-z * math.sqrt(1 / (sum(events1) + sum(events2))))),
                                                                       upper=_r(r["HR"] * math.exp(z * math.sqrt(1 / (sum(events1) + sum(events2)))))))(
         sv_exp_hr(times1, events1, times2, events2, level), _level(level)),
     phrases=("Exponential hazard ratio", "Exponential survival hazard ratio"),
     examples=[_ex({"times1": [6, 6, 6, 7, 10, 13, 16, 22, 23], "events1": [1, 1, 1, 1, 1, 1, 1, 1, 1],
                    "times2": [6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35], "events2": [1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0], "level": "95%"}),
               _ex({"times1": [5, 8, 12, 20, 25], "events1": [1, 1, 0, 1, 0], "times2": [3, 4, 9, 10, 14], "events2": [1, 1, 1, 0, 1], "level": 0.9})],
     level="advanced")


def sv_rmst(times, events, tau):
    T, E = _surv(times, events)
    _pos("tau", tau)
    area, s, last = 0.0, 1.0, 0.0
    parts = []
    for u, n, d in _km_steps(T, E):
        if u >= tau:
            break
        parts.append(s * (u - last))
        s *= (n - d) / n
        last = u
    parts.append(s * (tau - last))
    return _out(RMST=math.fsum(parts))


def _ref_sv_rmst(times, events, tau):
    from lifelines import KaplanMeierFitter
    from lifelines.utils import restricted_mean_survival_time
    k = KaplanMeierFitter().fit(times, event_observed=events)
    return {"RMST": float(restricted_mean_survival_time(k, t=tau))}


def _wrong_sv_rmst(times, events, tau):
    T, E = _surv(times, events)
    pts, s = [(0.0, 1.0)], 1.0
    for u, n, d in _km_steps(T, E):
        if u >= tau:
            break
        s *= (n - d) / n
        pts.append((u, s))
    pts.append((tau, s))
    return _out(RMST=math.fsum((b[0] - a[0]) * (a[1] + b[1]) / 2 for a, b in zip(pts, pts[1:])))     # trapezoids, not steps


_add("bmed_restricted_mean_survival_time",
     name="Restricted mean survival time from the Kaplan-Meier curve", topic=_SV,
     purpose="The restricted mean survival time up to tau: the area under the Kaplan-Meier survival curve from 0 to tau.",
     inputs=[_vecin("times", "follow-up times (one time unit)", 1), _vecin("events", "1 = event, 0 = censored", 1),
             _numin("tau", "restriction time (in the unit of the times)")],
     outputs=[_o("RMST", "number", "restricted mean survival time, in the time unit")],
     primary="RMST", conditions=["non-informative censoring", "the Kaplan-Meier step function integrated exactly"],
     method="RMST = integral from 0 to tau of the Kaplan-Meier step function", latex=r"\mathrm{RMST}(\tau) = \int_0^{\tau}\hat S(t)\,dt",
     run=sv_rmst, reference=_ref_sv_rmst, wrong=_wrong_sv_rmst,
     rand=lambda rng: (lambda te: {"times": te[0], "events": te[1], "tau": float(rng.randint(5, 40)) + 0.5})(_rand_surv(rng)),
     phrases=("Restricted mean survival time", "RMST"),
     examples=[_ex({"times": [6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35],
                    "events": [1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0], "tau": 20}),
               _ex({"times": [2, 3, 3, 5, 8, 9, 12, 15], "events": [1, 0, 1, 1, 0, 1, 1, 0], "tau": 10})],
     level="advanced")


# ================================================================================================ BIOMECHANICS
_BM = "biomechanics"


def bm_lever(W, d_load, W_arm, d_arm, d_muscle, theta):
    _nonneg("the weights W and W_arm", W, W_arm)
    _pos("the distances d_load, d_arm and d_muscle", d_load, d_arm, d_muscle)
    _need(W + W_arm > 0, "there is no load")
    _need(0 < theta < math.pi, "the muscle angle must lie between 0 and 180 degrees")
    s, c = math.sin(theta), math.cos(theta)
    F = (W * d_load + W_arm * d_arm) / (d_muscle * s)
    Rx, Ry = F * c, W + W_arm - F * s
    if abs(Rx) <= 1e-12 * F:
        Rx = 0.0
    return _out(F_muscle=F, R_x=Rx, R_y=Ry, R=math.hypot(Rx, Ry))


def _ref_bm_lever(W, d_load, W_arm, d_arm, d_muscle, theta):
    import numpy as np
    s, c = math.sin(theta), math.cos(theta)
    A = np.array([[-c, 1.0, 0.0], [s, 0.0, 1.0], [(d_muscle - d_load) * s, 0.0, -d_load]])     # moments about the load point
    b = np.array([0.0, W + W_arm, (d_arm - d_load) * W_arm])
    F, Rx, Ry = (float(v) for v in np.linalg.solve(A, b))
    if abs(Rx) <= 1e-12 * F:
        Rx = 0.0
    return {"F_muscle": F, "R_x": Rx, "R_y": Ry, "R": math.hypot(Rx, Ry)}


def _rand_bm_lever(rng):
    th = math.radians(rng.choice([rng.uniform(20, 80), rng.uniform(100, 160)]))
    return {"W": _sig(rng.uniform(0, 200)), "d_load": _sig(rng.uniform(0.25, 0.4)), "W_arm": _sig(rng.uniform(5, 25)),
            "d_arm": _sig(rng.uniform(0.1, 0.2)), "d_muscle": _sig(rng.uniform(0.02, 0.06)), "theta": _sig(th)}


_add("bmed_elbow_lever_muscle_force",
     name="Static lever model of the forearm: muscle force and joint reaction", topic=_BM,
     purpose="For a horizontal forearm held in static equilibrium, the elbow-flexor muscle force and the joint reaction force from the hand load, the forearm weight, their distances from the elbow and the muscle's insertion distance and angle.",
     inputs=[_phys("W", "force", "load held in the hand (weight)"), _phys("d_load", "length", "distance of the load from the elbow"),
             _phys("W_arm", "force", "weight of the forearm and hand"), _phys("d_arm", "length", "distance of the forearm's centre of mass from the elbow"),
             _phys("d_muscle", "length", "distance of the muscle insertion from the elbow"),
             _phys("theta", "angle", "angle between the muscle and the forearm")],
     outputs=[_o("F_muscle", "force", "muscle force"), _o("R_x", "force", "joint reaction along the forearm (towards the hand positive)"),
              _o("R_y", "force", "joint reaction, upward positive"), _o("R", "force", "magnitude of the joint reaction")],
     primary="F_muscle", conditions=["static equilibrium in a vertical plane", "forearm horizontal from the elbow, loads vertical",
                                     "one muscle pulling up and back towards the elbow at angle theta", "0 < theta < 180 deg"],
     method="moments about the elbow: F d_m sin(theta) = W d_load + W_arm d_arm; force balances give R_x = F cos(theta), R_y = W + W_arm - F sin(theta)",
     latex=r"F_m = \frac{W d_L + W_a d_a}{d_m \sin\theta},\quad R_x = F_m\cos\theta,\quad R_y = W + W_a - F_m\sin\theta",
     run=bm_lever, reference=_ref_bm_lever, rand=_rand_bm_lever,
     wrong=lambda W, d_load, W_arm, d_arm, d_muscle, theta: (lambda F: _out(F_muscle=F, R_x=F * math.cos(theta), R_y=W + W_arm - F * math.sin(theta),
                                                                          R=math.hypot(F * math.cos(theta), W + W_arm - F * math.sin(theta))))(
         (W * d_load + W_arm * d_arm) / d_muscle),
     phrases=("Forearm lever muscle force", "Elbow lever model"),
     examples=[_ex({"W": "50 N", "d_load": "35 cm", "W_arm": "15 N", "d_arm": "15 cm", "d_muscle": "4 cm", "theta": "80 deg"}),
               _ex({"W": "100 N", "d_load": "0.3 m", "W_arm": "20 N", "d_arm": "0.14 m", "d_muscle": "0.05 m", "theta": "90 deg"})],
     level="beginner",
     extra_q=[{"q": "Forearm lever muscle force: W = 50 N, d_load = 35 cm, W_arm = 15 N, d_arm = 15 cm, d_muscle = 4 cm, theta = 190 deg", "outcome": "INVALID_INPUT"}])


def bm_poiseuille(dP, r, L, mu):
    _pos("dP, r, L and mu", dP, r, L, mu)
    Q = math.pi * r ** 4 * dP / (8.0 * mu * L)
    return _out(Q=Q, v_mean=Q / (math.pi * r * r), tau_wall=4.0 * mu * Q / (math.pi * r ** 3), R_hyd=8.0 * mu * L / (math.pi * r ** 4))


def _ref_bm_poiseuille(dP, r, L, mu):
    Q = _quad(lambda s: dP / (4.0 * mu * L) * (r * r - s * s) * 2.0 * math.pi * s, 0.0, r)      # flow through the cross-section
    return {"Q": Q, "v_mean": Q / (math.pi * r * r), "tau_wall": dP * r / (2.0 * L), "R_hyd": dP / Q}


_add("bmed_poiseuille_vessel_flow",
     name="Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance", topic=_BM,
     purpose="Laminar (Hagen-Poiseuille) flow of a Newtonian fluid through a rigid vessel: volume flow rate, mean velocity, wall shear stress and hydraulic resistance from the pressure drop, radius, length and viscosity.",
     inputs=[_phys("dP", "pressure", "pressure drop along the vessel"), _phys("r", "length", "vessel radius"),
             _phys("L", "length", "vessel length"), _phys("mu", "dynamic_viscosity", "viscosity of the blood (Newtonian)")],
     outputs=[_o("Q", "flow_rate", "volume flow rate"), _o("v_mean", "speed", "mean velocity"),
              _o("tau_wall", "shear_stress", "wall shear stress"), _o("R_hyd", "hydraulic_resistance", "hydraulic resistance")],
     primary="Q", conditions=["steady laminar flow, Newtonian fluid", "rigid straight cylindrical vessel, fully developed flow"],
     method="Q = pi r^4 dP / (8 mu L); v = Q / (pi r^2); tau_w = 4 mu Q / (pi r^3); R = 8 mu L / (pi r^4)",
     latex=r"Q = \frac{\pi r^4 \Delta P}{8 \mu L},\quad \tau_w = \frac{4\mu Q}{\pi r^3},\quad R = \frac{8\mu L}{\pi r^4}",
     run=bm_poiseuille, reference=_ref_bm_poiseuille,
     rand=lambda rng: {"dP": _sig(_lu(rng, 10, 5000)), "r": _sig(_lu(rng, 1e-5, 0.015)), "L": _sig(_lu(rng, 1e-3, 0.5)), "mu": _sig(rng.uniform(1e-3, 5e-3))},
     wrong=lambda dP, r, L, mu: (lambda Q: _out(Q=Q, v_mean=Q / (math.pi * r * r), tau_wall=4.0 * mu * Q / (math.pi * r ** 3), R_hyd=dP / Q))(
         math.pi * r ** 4 * dP / (128.0 * mu * L)),
     phrases=("Poiseuille vessel flow", "Poiseuille blood flow"),
     examples=[_ex({"dP": "4 mmHg", "r": "2 mm", "L": "10 cm", "mu": "3.5 mPa*s"}),
               _ex({"dP": "50 Pa", "r": "100 um", "L": "5 mm", "mu": "0.004 Pa*s"})],
     level="beginner")


def bm_windkessel_decay(P0, Pv, R, C, t):
    _pos("P0, R and C", P0, R, C)
    _nonneg("the venous pressure Pv and the time t", Pv, t)
    _need(Pv < P0, "the starting pressure must exceed the venous pressure")
    return _out(P=Pv + (P0 - Pv) * math.exp(-t / (R * C)))


def _ref_bm_windkessel_decay(P0, Pv, R, C, t):
    y = _ivp(lambda s, y: [-(y[0] - Pv) / (R * C)], t, [P0], atol=[1e-12])
    return {"P": y[0]}


_add("bmed_windkessel_diastolic_decay",
     name="Two-element Windkessel: diastolic pressure decay", topic=_BM,
     purpose="The arterial pressure of a two-element Windkessel model during diastole (no inflow), decaying from P0 towards the venous pressure with time constant RC.",
     inputs=[_phys("P0", "pressure", "pressure at the start of diastole"), _phys("Pv", "pressure", "venous (outflow) pressure"),
             _phys("R", "hydraulic_resistance", "peripheral resistance"), _phys("C", "hydraulic_compliance", "arterial compliance"),
             _phys("t", "time", "time into diastole")],
     outputs=[_o("P", "pressure", "arterial pressure at time t")],
     primary="P", conditions=["no inflow during diastole", "linear resistance and compliance"],
     method="C dP/dt = -(P - Pv)/R: P = Pv + (P0 - Pv) exp(-t/(R C))", latex=r"P(t) = P_v + (P_0 - P_v)\,e^{-t/(RC)}",
     run=bm_windkessel_decay, reference=_ref_bm_windkessel_decay,
     rand=lambda rng: (lambda R, C: {"P0": _sig(rng.uniform(1.2e4, 1.8e4)), "Pv": _sig(rng.uniform(100, 2000)), "R": R, "C": C,
                                     "t": _sig(rng.uniform(0.05, 3) * R * C)})(_sig(_lu(rng, 5e7, 3e8)), _sig(_lu(rng, 5e-9, 3e-8))),
     wrong=lambda P0, Pv, R, C, t: _out(P=P0 * math.exp(-t / (R * C))),
     phrases=("Windkessel diastolic decay", "Windkessel pressure decay"),
     examples=[_ex({"P0": "120 mmHg", "Pv": "5 mmHg", "R": "1 mmHg*s/mL", "C": "1.5 mL/mmHg", "t": "0.5 s"}),
               _ex({"P0": "100 mmHg", "Pv": "0 mmHg", "R": "1.2 mmHg*s/mL", "C": "1.1 mL/mmHg", "t": "0.8 s"})],
     level="beginner")


def bm_windkessel_impedance(model, R, C, f, Zc=None):
    _pos("R, C and f", R, C, f)
    _need(model in ("two_element", "three_element"), "model is two_element or three_element")
    if model == "three_element":
        _need(Zc is not None, "the three-element model needs the characteristic impedance Zc")
        _pos("Zc", Zc)
    else:
        _need(Zc is None, "Zc belongs to the three-element model, not the two-element model")
    w = 2.0 * math.pi * f
    den = 1.0 + (w * R * C) ** 2
    zr = R / den + (Zc or 0.0)
    zi = -w * R * R * C / den
    return _out(Z_mod=math.hypot(zr, zi), phase_deg=math.degrees(math.atan2(zi, zr)))


def _ref_bm_windkessel_impedance(model, R, C, f, Zc=None):
    import numpy as np
    from scipy.signal import freqs
    zc = Zc or 0.0
    b, a = [zc * R * C, zc + R], [R * C, 1.0]
    _, h = freqs(b, a, worN=[2.0 * math.pi * f])
    return {"Z_mod": float(np.abs(h[0])), "phase_deg": float(np.degrees(np.angle(h[0])))}


def _rand_bm_wk(rng):
    m = rng.choice(["two_element", "three_element"])
    R, C = _sig(_lu(rng, 5e7, 3e8)), _sig(_lu(rng, 5e-9, 3e-8))
    d = {"model": m, "R": R, "C": C, "f": _sig(rng.uniform(0.2, 20) / (2 * math.pi * R * C))}
    if m == "three_element":
        d["Zc"] = _sig(R * rng.uniform(0.03, 0.15))
    return d


_add("bmed_windkessel_input_impedance",
     name="Windkessel models (two- and three-element): input impedance at a frequency", topic=_BM,
     purpose="The modulus and phase of the input impedance of a two-element (R, C) or three-element (Zc, R, C) Windkessel model of the arterial tree at a stated frequency.",
     inputs=[_choice("model", ("two_element", "three_element"), "Windkessel model"), _phys("R", "hydraulic_resistance", "peripheral resistance"),
             _phys("C", "hydraulic_compliance", "arterial compliance"), _phys("f", "frequency", "frequency"),
             _phys("Zc", "hydraulic_resistance", "characteristic impedance of the aorta (three-element model only)", required=False)],
     outputs=[_o("Z_mod", "hydraulic_resistance", "impedance modulus"), _o("phase_deg", "number", "impedance phase in degrees")],
     primary="Z_mod", conditions=["linear model, sinusoidal steady state", "Zc only for the three-element model"],
     method="Z = Zc + R/(1 + j 2 pi f R C) (Zc = 0 for two elements); modulus and phase",
     latex=r"Z(\omega) = Z_c + \frac{R}{1 + j\omega R C}",
     run=bm_windkessel_impedance, reference=_ref_bm_windkessel_impedance, rand=_rand_bm_wk,
     wrong=lambda model, R, C, f, Zc=None: (lambda r: dict(r, phase_deg=-r["phase_deg"]))(bm_windkessel_impedance(model, R, C, f, Zc)),
     phrases=("Windkessel impedance", "Windkessel input impedance"),
     examples=[_ex({"model": "two_element", "R": "1 mmHg*s/mL", "C": "1.5 mL/mmHg", "f": "1.2 Hz"}),
               _ex({"model": "three_element", "R": "1 mmHg*s/mL", "C": "1.5 mL/mmHg", "f": "2 Hz", "Zc": "0.05 mmHg*s/mL"})],
     level="advanced",
     extra_q=[{"q": "Windkessel impedance: model = three_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 2 Hz", "outcome": "INVALID_INPUT"}])


def bm_kelvin_voigt(sigma, E, eta, t):
    _pos("sigma, E and eta", sigma, E, eta)
    _nonneg("the time t", t)
    return _out(strain=sigma / E * -math.expm1(-E * t / eta))


def _ref_bm_kelvin_voigt(sigma, E, eta, t):
    y = _ivp(lambda s, y: [(sigma - E * y[0]) / eta], t, [0.0], atol=[1e-20])
    return {"strain": y[0]}


_add("bmed_kelvin_voigt_creep",
     name="Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress", topic=_BM,
     purpose="The strain of a Kelvin-Voigt (spring parallel to dashpot) tissue model a time t after a constant stress is applied.",
     inputs=[_phys("sigma", "stress", "applied constant stress"), _phys("E", "elastic_modulus", "spring modulus"),
             _phys("eta", "dynamic_viscosity", "dashpot viscosity (Pa*s)"), _phys("t", "time", "time since loading")],
     outputs=[_o("strain", "number", "strain at time t")],
     primary="strain", conditions=["linear viscoelasticity", "stress applied as a step at t = 0, no initial strain"],
     method="eta de/dt + E e = sigma: e = (sigma/E)(1 - exp(-E t/eta))", latex=r"\varepsilon(t) = \frac{\sigma}{E}\left(1 - e^{-E t/\eta}\right)",
     run=bm_kelvin_voigt, reference=_ref_bm_kelvin_voigt,
     rand=lambda rng: (lambda E, eta: {"sigma": _sig(E * rng.uniform(0.01, 0.2)), "E": E, "eta": eta, "t": _sig(rng.uniform(0.05, 5) * eta / E)})(
         _sig(_lu(rng, 1e4, 1e8)), _sig(_lu(rng, 1e3, 1e8))),
     wrong=lambda sigma, E, eta, t: _out(strain=sigma / E * math.exp(-E * t / eta)),
     phrases=("Kelvin-Voigt creep", "Kelvin-Voigt model creep"),
     examples=[_ex({"sigma": "10 kPa", "E": "100 kPa", "eta": "500 kPa*s", "t": "3 s"}),
               _ex({"sigma": "2 MPa", "E": "1 GPa", "eta": "5 GPa*s", "t": "10 s"})],
     level="beginner")


def bm_sls_creep(sigma, E_inf, E1, eta, t):
    _pos("sigma, E_inf, E1 and eta", sigma, E_inf, E1, eta)
    _nonneg("the time t", t)
    tc = eta * (E_inf + E1) / (E_inf * E1)
    e_inf, e0 = sigma / E_inf, sigma / (E_inf + E1)
    return _out(strain=e_inf - (e_inf - e0) * math.exp(-t / tc), strain_inf=e_inf)


def _ref_bm_sls_creep(sigma, E_inf, E1, eta, t):
    f = lambda s, y: [(sigma - E_inf * y[0]) / (eta * (1.0 + E_inf / E1))]
    y0 = [sigma / (E_inf + E1)]                       # instantaneous response: the dashpot is rigid
    rate = E_inf / (eta * (1.0 + E_inf / E1))         # the ODE's own relaxation rate (its linear coefficient)
    y = _ivp(f, t, y0, atol=[1e-20])
    yl = _ivp(f, 60.0 / rate, y0, atol=[1e-20])
    return {"strain": y[0], "strain_inf": yl[0]}


_add("bmed_sls_creep",
     name="Standard linear solid (Zener) tissue model: creep under constant stress", topic=_BM,
     purpose="The creep strain of a standard linear solid (a spring E_inf parallel to a Maxwell arm E1-eta) a time t after a constant stress is applied, and the final strain.",
     inputs=[_phys("sigma", "stress", "applied constant stress"), _phys("E_inf", "elastic_modulus", "equilibrium (parallel) spring modulus"),
             _phys("E1", "elastic_modulus", "spring modulus of the Maxwell arm"), _phys("eta", "dynamic_viscosity", "dashpot viscosity of the Maxwell arm"),
             _phys("t", "time", "time since loading")],
     outputs=[_o("strain", "number", "strain at time t"), _o("strain_inf", "number", "final strain sigma/E_inf")],
     primary="strain", conditions=["linear viscoelasticity", "step stress at t = 0"],
     method="e(t) = sigma/E_inf - (sigma/E_inf - sigma/(E_inf + E1)) exp(-t/tau_c), tau_c = eta (E_inf + E1)/(E_inf E1)",
     latex=r"\varepsilon(t) = \frac{\sigma}{E_\infty} - \left(\frac{\sigma}{E_\infty} - \frac{\sigma}{E_\infty + E_1}\right)e^{-t/\tau_c},\ \tau_c = \frac{\eta(E_\infty + E_1)}{E_\infty E_1}",
     run=bm_sls_creep, reference=_ref_bm_sls_creep,
     rand=lambda rng: (lambda Ei, E1, eta: {"sigma": _sig(Ei * rng.uniform(0.01, 0.2)), "E_inf": Ei, "E1": E1, "eta": eta,
                                            "t": _sig(rng.uniform(0.05, 5) * eta / E1)})(_sig(_lu(rng, 1e4, 1e8)), _sig(_lu(rng, 1e4, 1e8)), _sig(_lu(rng, 1e3, 1e8))),
     wrong=lambda sigma, E_inf, E1, eta, t: (lambda ei, e0: _out(strain=ei - (ei - e0) * math.exp(-t * E1 / eta), strain_inf=ei))(sigma / E_inf, sigma / (E_inf + E1)),
     phrases=("Standard linear solid creep", "Zener model creep"),
     examples=[_ex({"sigma": "10 kPa", "E_inf": "50 kPa", "E1": "100 kPa", "eta": "200 kPa*s", "t": "2 s"}),
               _ex({"sigma": "1 MPa", "E_inf": "200 MPa", "E1": "800 MPa", "eta": "2 GPa*s", "t": "1 s"})],
     level="advanced")


def bm_sls_relaxation(eps0, E_inf, E1, eta, t):
    _pos("eps0, E_inf, E1 and eta", eps0, E_inf, E1, eta)
    _nonneg("the time t", t)
    return _out(stress=eps0 * (E_inf + E1 * math.exp(-E1 * t / eta)), stress_inf=eps0 * E_inf)


def _ref_bm_sls_relaxation(eps0, E_inf, E1, eta, t):
    f = lambda s, y: [-E1 * y[0] / eta]                # the Maxwell arm's stress at fixed strain
    y = _ivp(f, t, [E1 * eps0], atol=[1e-12])
    yl = _ivp(f, 60.0 * eta / E1, [E1 * eps0], atol=[1e-30])
    return {"stress": E_inf * eps0 + y[0], "stress_inf": E_inf * eps0 + yl[0]}


_add("bmed_sls_stress_relaxation",
     name="Standard linear solid (Zener) tissue model: stress relaxation at constant strain", topic=_BM,
     purpose="The stress of a standard linear solid a time t after a step strain eps0 is imposed, and the relaxed (equilibrium) stress.",
     inputs=[_numin("eps0", "imposed step strain (a pure number, e.g. 0.05)"), _phys("E_inf", "elastic_modulus", "equilibrium spring modulus"),
             _phys("E1", "elastic_modulus", "spring modulus of the Maxwell arm"), _phys("eta", "dynamic_viscosity", "dashpot viscosity of the Maxwell arm"),
             _phys("t", "time", "time since the strain step")],
     outputs=[_o("stress", "stress", "stress at time t"), _o("stress_inf", "stress", "relaxed stress E_inf eps0")],
     primary="stress", conditions=["linear viscoelasticity", "step strain at t = 0"],
     method="sigma(t) = eps0 (E_inf + E1 exp(-E1 t/eta))", latex=r"\sigma(t) = \varepsilon_0\left(E_\infty + E_1 e^{-E_1 t/\eta}\right)",
     run=bm_sls_relaxation, reference=_ref_bm_sls_relaxation,
     rand=lambda rng: (lambda Ei, E1, eta: {"eps0": round(rng.uniform(0.005, 0.2), 4), "E_inf": Ei, "E1": E1, "eta": eta,
                                            "t": _sig(rng.uniform(0.05, 5) * eta / E1)})(_sig(_lu(rng, 1e4, 1e8)), _sig(_lu(rng, 1e4, 1e8)), _sig(_lu(rng, 1e3, 1e8))),
     wrong=lambda eps0, E_inf, E1, eta, t: _out(stress=eps0 * (E_inf + E1 * math.exp(-(E_inf + E1) * t / eta)), stress_inf=eps0 * E_inf),
     phrases=("Standard linear solid relaxation", "Zener model stress relaxation"),
     examples=[_ex({"eps0": 0.05, "E_inf": "50 kPa", "E1": "100 kPa", "eta": "200 kPa*s", "t": "1 s"}),
               _ex({"eps0": 0.01, "E_inf": "200 MPa", "E1": "800 MPa", "eta": "2 GPa*s", "t": "5 s"})],
     level="intermediate")


def bm_indicator_dilution(m, t, C):
    _pos("the indicator mass m", m)
    auc = _trap(t, C, "the indicator curve")
    _need(auc > 0, "the indicator curve has no area")
    return _out(AUC=auc, CO=m / (auc * 1e-3))


def _ref_bm_indicator_dilution(m, t, C):
    from scipy.integrate import trapezoid
    auc = float(trapezoid(C, t))
    return {"AUC": auc, "CO": m / (auc * 1e-3)}


def _rand_bm_ind(rng):
    ts = [float(v) for v in range(0, rng.randint(12, 30), rng.choice([1, 2]))]
    t0, a = rng.uniform(2, 5), rng.uniform(2, 6)
    C = [_sig(max(0.0, (s - t0)) ** 2 * math.exp(-(s - t0) / a) * rng.uniform(0.5, 2)) if s > t0 else 0.0 for s in ts]
    if max(C) == 0:
        C[-1] = 1.0
    return {"m": _sig(_lu(rng, 1e-6, 1e-5)), "t": ts, "C": C}


_add("bmed_indicator_dilution_cardiac_output",
     name="Cardiac output by indicator dilution (Stewart-Hamilton)", topic=_BM,
     purpose="The cardiac output (volume flow) from the mass of an injected indicator and its sampled first-pass concentration curve, by the Stewart-Hamilton principle with a trapezoidal area.",
     inputs=[_phys("m", "mass", "mass of indicator injected"), _vecin("t", "sampling times in s, strictly increasing", 2),
             _vecin("C", "indicator concentrations in mg/L (first pass, recirculation excluded)", 2)],
     outputs=[_o("AUC", "number", "area under the concentration curve, mg*s/L"), _o("CO", "flow_rate", "cardiac output m / AUC")],
     primary="CO", conditions=["complete mixing, no indicator loss", "first-pass curve (recirculation removed by the user)",
                               "times in s, concentrations in mg/L", "linear trapezoids"],
     method="CO = m / integral of C dt (linear trapezoidal rule)", latex=r"CO = \frac{m}{\int_0^{\infty} C(t)\,dt}",
     run=bm_indicator_dilution, reference=_ref_bm_indicator_dilution, rand=_rand_bm_ind,
     wrong=lambda m, t, C: (lambda a: _out(AUC=a, CO=m / (a * 1e-3)))(math.fsum(c * (t2 - t1) for t1, t2, c in zip(t, t[1:], C))),
     phrases=("Indicator dilution cardiac output", "Stewart-Hamilton cardiac output"),
     examples=[_ex({"m": "5 mg", "t": [0, 2, 4, 6, 8, 10, 12, 14, 16], "C": [0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2]}),
               _ex({"m": "10 mg", "t": [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10], "C": [0, 0, 1.2, 4.5, 6.8, 6.0, 4.1, 2.5, 1.3, 0.6, 0.2]})],
     level="intermediate")


def bm_respiratory_fit(P, Q, V):
    n = len(P)
    _need(len(Q) == n and len(V) == n and n >= 4, "give at least four samples of P, Q and V")
    for v in list(P) + list(Q) + list(V):
        _fin("every sample", v)
    Rr, E, P0 = _ols([list(Q), list(V), [1.0] * n], list(P))
    _need(E > 0, "the fitted elastance is not positive")
    return _out(R=Rr, C=1.0 / E, P0=P0)


def _ref_bm_respiratory_fit(P, Q, V):
    import numpy as np
    import statsmodels.api as sm
    X = np.column_stack([np.array(Q, dtype=float), np.array(V, dtype=float), np.ones(len(P))])
    p = sm.OLS(np.array(P, dtype=float), X).fit().params
    return {"R": float(p[0]), "C": 1.0 / float(p[1]), "P0": float(p[2])}


def _rand_bm_resp(rng):
    n = rng.randint(6, 25)
    R, C, P0 = rng.uniform(2, 20), rng.uniform(0.02, 0.1), rng.uniform(0, 10)
    ts = [i * 0.1 for i in range(n)]
    Q = [round(0.5 * math.sin(1.3 * s) + rng.uniform(-0.05, 0.05) + 0.4, 3) for s in ts]
    V = [round(0.5 * (1 - math.cos(1.3 * s)) / 1.3 + 0.4 * s + rng.uniform(-0.01, 0.01), 3) for s in ts]
    P = [round(R * q + v / C + P0 + rng.uniform(-0.3, 0.3), 2) for q, v in zip(Q, V)]
    return {"P": P, "Q": Q, "V": V}


_add("bmed_respiratory_mechanics_fit",
     name="Single-compartment respiratory mechanics: resistance and compliance by least squares", topic=_BM,
     purpose="Airway resistance, respiratory compliance and the end-expiratory pressure of the single-compartment equation of motion P = R Q + V/C + P0, fitted by least squares to sampled pressure, flow and volume.",
     inputs=[_vecin("P", "airway pressures in cmH2O", 4), _vecin("Q", "flows in L/s at the same instants", 4),
             _vecin("V", "volumes above end-expiration in L at the same instants", 4)],
     outputs=[_o("R", "number", "resistance, cmH2O*s/L"), _o("C", "number", "compliance, L/cmH2O"), _o("P0", "number", "pressure at zero flow and volume, cmH2O")],
     primary="C", conditions=["linear single-compartment model", "pressures in cmH2O, flows in L/s, volumes in L", "ordinary least squares"],
     method="multiple linear regression of P on Q, V and a constant; C = 1/elastance", latex=r"P = R\,\dot V + \frac{V}{C} + P_0",
     run=bm_respiratory_fit, reference=_ref_bm_respiratory_fit, rand=_rand_bm_resp,
     wrong=lambda P, Q, V: (lambda c: _out(R=c[0], C=1.0 / c[1], P0=0.0))(_ols([list(Q), list(V)], list(P))),
     phrases=("Respiratory mechanics fit", "Equation of motion fit"),
     examples=[_ex({"P": [5.2, 9.8, 13.1, 15.9, 18.6, 20.2], "Q": [0.5, 0.5, 0.45, 0.4, 0.35, 0.3], "V": [0.0, 0.1, 0.19, 0.27, 0.34, 0.4]}),
               _ex({"P": [8, 12.5, 16, 19.2, 22, 24.1, 25.5], "Q": [0.8, 0.75, 0.7, 0.6, 0.5, 0.4, 0.3], "V": [0, 0.08, 0.15, 0.22, 0.27, 0.32, 0.35]})],
     level="advanced")


# ================================================================================================ NEURON MODELS
_NE = "neuron models"


def _ghk_terms(PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i):
    return PK * K_o + PNa * Na_o + PCl * Cl_i, PK * K_i + PNa * Na_i + PCl * Cl_o


def ne_ghk_voltage(T, PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i):
    _pos("the temperature T and PK", T, PK)
    _nonneg("PNa and PCl", PNa, PCl)
    _pos("every concentration", K_o, K_i, Na_o, Na_i, Cl_o, Cl_i)
    num, den = _ghk_terms(PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i)
    return _out(V_m=R_GAS * T / F_FARADAY * math.log(num / den))


def _ghk_flux_g(u):
    return 1.0 if u == 0 else u / -math.expm1(-u)


def _ref_ne_ghk_voltage(T, PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i):
    from scipy.constants import N_A, e, k
    Rg, Fc = N_A * k, N_A * e

    def total(V):                                  # sum of the GHK currents (outward positive) at voltage V
        u = Fc * V / (Rg * T)
        cur = 0.0
        for P, z, ci, co in ((PK, 1, K_i, K_o), (PNa, 1, Na_i, Na_o), (PCl, -1, Cl_i, Cl_o)):
            if P:
                cur += P * z * (ci * _ghk_flux_g(z * u) - co * _ghk_flux_g(-z * u))
        return cur
    return {"V_m": _brentq(total, -0.5, 0.5)}


def _rand_ne_ghk(rng):
    return {"T": _sig(rng.uniform(280, 315)), "PK": 1.0, "PNa": _sig(_lu(rng, 0.005, 0.5), 3), "PCl": _sig(_lu(rng, 0.05, 1.0), 3),
            "K_o": _sig(rng.uniform(2, 20), 3), "K_i": _sig(rng.uniform(100, 160), 3), "Na_o": _sig(rng.uniform(100, 160), 3),
            "Na_i": _sig(rng.uniform(5, 30), 3), "Cl_o": _sig(rng.uniform(80, 130), 3), "Cl_i": _sig(rng.uniform(4, 40), 3)}


_add("bmed_ghk_voltage",
     name="Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl", topic=_NE,
     purpose="The resting membrane potential of the Goldman-Hodgkin-Katz voltage equation from the relative permeabilities and the inside and outside concentrations of K+, Na+ and Cl-.",
     inputs=[_phys("T", "temperature", "absolute temperature (or degC)"), _numin("PK", "relative permeability to K+ (> 0)"),
             _numin("PNa", "relative permeability to Na+"), _numin("PCl", "relative permeability to Cl-"),
             _phys("K_o", "molar_concentration", "outside K+"), _phys("K_i", "molar_concentration", "inside K+"),
             _phys("Na_o", "molar_concentration", "outside Na+"), _phys("Na_i", "molar_concentration", "inside Na+"),
             _phys("Cl_o", "molar_concentration", "outside Cl-"), _phys("Cl_i", "molar_concentration", "inside Cl-")],
     outputs=[_o("V_m", "voltage", "membrane potential, inside relative to outside")],
     primary="V_m", conditions=["steady state with zero net current", "constant field in the membrane, independent ions",
                                "only K+, Na+ and Cl- permeate", "only ratios of permeabilities matter"],
     method="V = (RT/F) ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o))",
     latex=r"V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}",
     run=ne_ghk_voltage, reference=_ref_ne_ghk_voltage, rand=_rand_ne_ghk,
     wrong=lambda T, PK, PNa, PCl, K_o, K_i, Na_o, Na_i, Cl_o, Cl_i: _out(V_m=R_GAS * T / F_FARADAY * math.log(
         (PK * K_o + PNa * Na_o + PCl * Cl_o) / (PK * K_i + PNa * Na_i + PCl * Cl_i))),
     phrases=("GHK voltage", "Goldman-Hodgkin-Katz voltage"),
     examples=[_ex({"T": "310 K", "PK": 1, "PNa": 0.04, "PCl": 0.45, "K_o": "4 mmol/L", "K_i": "140 mmol/L", "Na_o": "145 mmol/L",
                    "Na_i": "12 mmol/L", "Cl_o": "116 mmol/L", "Cl_i": "4 mmol/L"}),
               _ex({"T": "6.3 degC", "PK": 1, "PNa": 0.03, "PCl": 0.1, "K_o": "20 mM", "K_i": "400 mM", "Na_o": "440 mM",
                    "Na_i": "50 mM", "Cl_o": "560 mM", "Cl_i": "52 mM"})],
     level="intermediate",
     extra_q=[{"q": "GHK voltage: T = 36.85 degC, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L",
               "like": 0}])


def ne_ghk_current(P, z, C_in, C_out, V, T):
    _pos("the permeability P and the temperature T", P, T)
    _need(isinstance(z, int) and z != 0 and abs(z) <= 3, "z is a nonzero whole number from -3 to 3")
    _nonneg("the concentrations", C_in, C_out)
    _fin("the voltage V", V)
    u = z * F_FARADAY * V / (R_GAS * T)
    return _out(I=P * z * F_FARADAY * (C_in * _ghk_flux_g(u) - C_out * _ghk_flux_g(-u)))


def _ref_ne_ghk_current(P, z, C_in, C_out, V, T):
    from scipy.constants import N_A, e, k
    Fc = N_A * e
    u0 = Fc * V / (N_A * k * T)

    def end(J):                                    # Nernst-Planck across the membrane (x from inside 0 to outside 1)
        return _ivp(lambda x, c: [z * u0 * c[0] - J / P], 1.0, [C_in], atol=[1e-14])[0]
    a, b = end(0.0), end(P) - end(0.0)             # c(1) is linear in the flux J
    J = (C_out - a) / b * P
    return {"I": z * Fc * J}


def _rand_ne_ghk_i(rng):
    V = rng.uniform(0.001, 0.1) * rng.choice([-1, 1])
    return {"P": _sig(_lu(rng, 1e-9, 1e-6)), "z": rng.choice([1, 1, 2, -1]), "C_in": _sig(_lu(rng, 1e-4, 200)),
            "C_out": _sig(_lu(rng, 1e-4, 200)), "V": _sig(V), "T": _sig(rng.uniform(280, 315))}


_add("bmed_ghk_current_density",
     name="Goldman-Hodgkin-Katz current equation: ionic current density through a membrane", topic=_NE,
     purpose="The current density carried by one ion species through a membrane of given permeability at a stated membrane potential, from the GHK current (constant-field) equation; outward current positive.",
     inputs=[_phys("P", "membrane_permeability", "membrane permeability to the ion"), _intin("z", "valence of the ion (+1, +2, -1, ...)", -3, 3),
             _phys("C_in", "molar_concentration", "inside concentration"), _phys("C_out", "molar_concentration", "outside concentration"),
             _phys("V", "voltage", "membrane potential, inside relative to outside"), _phys("T", "temperature", "absolute temperature (or degC)")],
     outputs=[_o("I", "current_density", "current density, outward positive")],
     primary="I", conditions=["constant field across the membrane", "independent ion movement", "outward current positive"],
     method="I = P z^2 F^2 V/(RT) (C_in - C_out exp(-u))/(1 - exp(-u)), u = zFV/(RT) (written stably as P z F [C_in g(u) - C_out g(-u)], g(u) = u/(1 - e^-u))",
     latex=r"I = P z^2 \frac{F^2 V}{RT}\,\frac{[C]_i - [C]_o e^{-zFV/RT}}{1 - e^{-zFV/RT}}",
     run=ne_ghk_current, reference=_ref_ne_ghk_current, rand=_rand_ne_ghk_i,
     wrong=lambda P, z, C_in, C_out, V, T: (lambda u: _out(I=P * z * F_FARADAY * (C_out * _ghk_flux_g(u) - C_in * _ghk_flux_g(-u))))(z * F_FARADAY * V / (R_GAS * T)),
     phrases=("GHK current density", "Goldman-Hodgkin-Katz current"),
     examples=[_ex({"P": "1e-8 m/s", "z": 1, "C_in": "140 mmol/L", "C_out": "4 mmol/L", "V": "-20 mV", "T": "310 K"}),
               _ex({"P": "5e-9 m/s", "z": 2, "C_in": "0.0001 mmol/L", "C_out": "2 mmol/L", "V": "-70 mV", "T": "37 degC"})],
     level="expert",
     extra_q=[{"q": "GHK current density: P = 1e-8 m/s, z = 0, C_in = 140 mmol/L, C_out = 4 mmol/L, V = -20 mV, T = 310 K", "outcome": "INVALID_INPUT"}])


def ne_lif_rate(tau_m, R_m, I, V_rest, V_th, V_reset, t_ref):
    _pos("tau_m and R_m", tau_m, R_m)
    _nonneg("the refractory period t_ref", t_ref)
    _fin("I, V_rest, V_th and V_reset", I, V_rest, V_th, V_reset)
    _need(V_reset < V_th, "the reset potential must lie below the threshold")
    Vinf = V_rest + R_m * I
    _need(Vinf > V_th, "the input does not bring the membrane to threshold: the neuron does not fire")
    isi = t_ref + tau_m * math.log((Vinf - V_reset) / (Vinf - V_th))
    return _out(rate=1.0 / isi, ISI=isi)


def _ref_ne_lif_rate(tau_m, R_m, I, V_rest, V_th, V_reset, t_ref):
    Vinf = V_rest + R_m * I
    isi = t_ref + _quad(lambda v: tau_m / (Vinf - v), V_reset, V_th)       # dt = tau dV / (V_inf - V)
    return {"rate": 1.0 / isi, "ISI": isi}


def _rand_ne_lif(rng):
    vr, vth = rng.uniform(-0.075, -0.06), rng.uniform(-0.055, -0.045)
    R = _sig(_lu(rng, 1e7, 5e8))
    over = rng.uniform(0.001, 0.03)
    return {"tau_m": _sig(rng.uniform(0.005, 0.03)), "R_m": R, "I": _sig((vth - vr + over) / R), "V_rest": _sig(vr), "V_th": _sig(vth),
            "V_reset": _sig(vr - rng.uniform(0, 0.01)), "t_ref": _sig(rng.uniform(0.001, 0.005))}


_add("bmed_lif_firing_rate",
     name="Leaky integrate-and-fire neuron: firing rate under constant current", topic=_NE,
     purpose="The interspike interval and firing rate of a leaky integrate-and-fire neuron driven by a constant current, with reset and an absolute refractory period.",
     inputs=[_phys("tau_m", "time", "membrane time constant"), _phys("R_m", "resistance", "membrane resistance"),
             _phys("I", "current", "constant input current"), _phys("V_rest", "voltage", "resting potential"),
             _phys("V_th", "voltage", "spike threshold"), _phys("V_reset", "voltage", "reset potential"),
             _phys("t_ref", "time", "absolute refractory period")],
     outputs=[_o("rate", "frequency", "firing rate"), _o("ISI", "time", "interspike interval")],
     primary="rate", conditions=["V_rest + R I > V_th (otherwise no firing)", "V_reset < V_th", "deterministic constant input"],
     method="ISI = t_ref + tau ln((V_inf - V_reset)/(V_inf - V_th)), V_inf = V_rest + R I; rate = 1/ISI",
     latex=r"T = t_{ref} + \tau_m \ln\frac{V_\infty - V_{reset}}{V_\infty - V_{th}},\quad V_\infty = V_{rest} + R_m I",
     run=ne_lif_rate, reference=_ref_ne_lif_rate, rand=_rand_ne_lif,
     wrong=lambda tau_m, R_m, I, V_rest, V_th, V_reset, t_ref: (lambda isi: _out(rate=1 / isi, ISI=isi))(
         tau_m * math.log((V_rest + R_m * I - V_reset) / (V_rest + R_m * I - V_th))),
     phrases=("LIF firing rate", "Leaky integrate-and-fire firing rate"),
     examples=[_ex({"tau_m": "20 ms", "R_m": "100 Mohm", "I": "0.3 nA", "V_rest": "-70 mV", "V_th": "-50 mV", "V_reset": "-70 mV", "t_ref": "2 ms"}),
               _ex({"tau_m": "10 ms", "R_m": "50 Mohm", "I": "500 pA", "V_rest": "-65 mV", "V_th": "-50 mV", "V_reset": "-68 mV", "t_ref": "1 ms"})],
     level="intermediate",
     extra_q=[{"q": "LIF firing rate: tau_m = 20 ms, R_m = 100 Mohm, I = 0.1 nA, V_rest = -70 mV, V_th = -50 mV, V_reset = -70 mV, t_ref = 2 ms",
               "outcome": "INVALID_INPUT"}])


def ne_lif_voltage(tau_m, R_m, I, V_rest, V0, t):
    _pos("tau_m and R_m", tau_m, R_m)
    _fin("I, V_rest and V0", I, V_rest, V0)
    _nonneg("the time t", t)
    vinf = V_rest + R_m * I
    return _out(V=vinf + (V0 - vinf) * math.exp(-t / tau_m))


def _ref_ne_lif_voltage(tau_m, R_m, I, V_rest, V0, t):
    y = _ivp(lambda s, y: [(-(y[0] - V_rest) + R_m * I) / tau_m], t, [V0], atol=[1e-18])
    return {"V": y[0]}


_add("bmed_lif_subthreshold_voltage",
     name="Passive membrane (leaky integrator): voltage at a time under constant current", topic=_NE,
     purpose="The membrane potential of a passive RC membrane (the subthreshold leaky integrate-and-fire neuron) a time t after a constant current starts, from its initial potential.",
     inputs=[_phys("tau_m", "time", "membrane time constant"), _phys("R_m", "resistance", "membrane resistance"),
             _phys("I", "current", "constant input current"), _phys("V_rest", "voltage", "resting potential"),
             _phys("V0", "voltage", "potential at t = 0"), _phys("t", "time", "time")],
     outputs=[_o("V", "voltage", "membrane potential at time t")],
     primary="V", conditions=["passive membrane, no spike or threshold", "constant current from t = 0"],
     method="tau dV/dt = -(V - V_rest) + R I: V = V_inf + (V0 - V_inf) exp(-t/tau)", latex=r"V(t) = V_\infty + (V_0 - V_\infty)e^{-t/\tau_m},\quad V_\infty = V_{rest} + R_m I",
     run=ne_lif_voltage, reference=_ref_ne_lif_voltage,
     rand=lambda rng: {"tau_m": _sig(rng.uniform(0.005, 0.03)), "R_m": _sig(_lu(rng, 1e7, 5e8)), "I": _sig(rng.uniform(-2e-10, 5e-10)),
                       "V_rest": _sig(rng.uniform(-0.08, -0.06)), "V0": _sig(rng.uniform(-0.09, -0.04)), "t": _sig(rng.uniform(0.001, 0.08))},
     wrong=lambda tau_m, R_m, I, V_rest, V0, t: (lambda vinf: _out(V=V0 + (vinf - V0) * math.exp(-t / tau_m)))(V_rest + R_m * I),
     phrases=("Passive membrane voltage", "Leaky integrator voltage"),
     examples=[_ex({"tau_m": "20 ms", "R_m": "100 Mohm", "I": "0.15 nA", "V_rest": "-70 mV", "V0": "-70 mV", "t": "10 ms"}),
               _ex({"tau_m": "10 ms", "R_m": "200 Mohm", "I": "-50 pA", "V_rest": "-65 mV", "V0": "-60 mV", "t": "25 ms"})],
     level="beginner")


def ne_cable(Rm, Ri, d, x, V0, end, L=None):
    _pos("Rm, Ri and d", Rm, Ri, d)
    _nonneg("the position x", x)
    _fin("V0", V0)
    _need(end in ("sealed", "killed", "infinite"), "end is sealed, killed or infinite")
    lam = math.sqrt(Rm * d / (4.0 * Ri))
    rinf = 4.0 * Ri / (math.pi * d * d) * lam
    if end == "infinite":
        _need(L is None, "an infinite cable has no length L")
        return _out(V_x=V0 * math.exp(-x / lam), R_in=rinf)
    _need(L is not None, "a finite cable needs its length L")
    _pos("the length L", L)
    _need(x <= L, "x must lie on the cable (x <= L)")
    a, e1, e2 = L / lam, math.exp(-x / lam), math.exp(-(2.0 * L - x) / lam)
    q = math.exp(-2.0 * a)
    if end == "sealed":
        return _out(V_x=V0 * (e1 + e2) / (1.0 + q), R_in=rinf * (1.0 + q) / (1.0 - q))
    return _out(V_x=V0 * (e1 - e2) / (1.0 - q), R_in=rinf * (1.0 - q) / (1.0 + q))


def _ref_ne_cable(Rm, Ri, d, x, V0, end, L=None):
    rm, ri = Rm / (math.pi * d), 4.0 * Ri / (math.pi * d * d)       # per unit length
    lam = math.sqrt(rm / ri)
    Lc = 40.0 * lam if end == "infinite" else L
    y_end = [1.0, 0.0] if end == "sealed" else [0.0, -1.0]
    f = lambda s, y: [y[1], ri / rm * y[0]]
    at = [1e-300, 1e-300]
    y0 = _ivp(f, 0.0, y_end, t0=Lc, atol=at)                          # integrate back from the far end
    yx = _ivp(f, x, y_end, t0=Lc, atol=at) if x < Lc else y_end
    s = V0 / y0[0]
    return {"V_x": s * yx[0], "R_in": V0 / (-(s * y0[1]) / ri)}


def _rand_ne_cable(rng):
    Rm, Ri, d = _sig(_lu(rng, 0.1, 5.0)), _sig(_lu(rng, 0.5, 3.0)), _sig(_lu(rng, 5e-7, 2e-5))
    lam = math.sqrt(Rm * d / (4 * Ri))
    end = rng.choice(["sealed", "killed", "infinite"])
    r = {"Rm": Rm, "Ri": Ri, "d": d, "V0": _sig(rng.uniform(-0.02, 0.02)), "end": end}
    if end == "infinite":
        r["x"] = _sig(rng.uniform(0, 3) * lam)
    else:
        L = _sig(rng.uniform(0.2, 3) * lam)
        r["L"], r["x"] = L, _sig(rng.uniform(0, 1) * L)
        r["x"] = min(r["x"], L)
    if r["V0"] == 0:
        r["V0"] = 0.01
    return r


_add("bmed_cable_steady_state",
     name="Passive cable (dendrite or axon): steady-state voltage along it and input resistance", topic=_NE,
     purpose="The steady-state voltage at distance x along a passive cylindrical cable held at V0 at x = 0, and the input resistance there, for a sealed end, a killed (short-circuited) end or a semi-infinite cable.",
     inputs=[_phys("Rm", "specific_membrane_resistance", "specific membrane resistance"), _phys("Ri", "resistivity", "axial (cytoplasmic) resistivity"),
             _phys("d", "length", "cable diameter"), _phys("x", "length", "distance from the stimulated end"),
             _phys("V0", "voltage", "steady voltage at x = 0 (relative to rest)"),
             _choice("end", ("sealed", "killed", "infinite"), "far-end condition"),
             _phys("L", "length", "cable length (sealed or killed end only)", required=False)],
     outputs=[_o("V_x", "voltage", "voltage at x (relative to rest)"), _o("R_in", "resistance", "input resistance at x = 0")],
     primary="V_x", conditions=["passive membrane, steady state", "uniform cylinder", "L only for a finite cable"],
     method="lambda = sqrt(Rm d/(4 Ri)); sealed V0 cosh((L - x)/lambda)/cosh(L/lambda), killed V0 sinh((L - x)/lambda)/sinh(L/lambda), infinite V0 exp(-x/lambda); R_in = r_i lambda times coth, tanh or 1",
     latex=r"\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0\frac{\cosh((L - x)/\lambda)}{\cosh(L/\lambda)}\ \text{(sealed)}",
     run=ne_cable, reference=_ref_ne_cable, rand=_rand_ne_cable,
     wrong=lambda Rm, Ri, d, x, V0, end, L=None: ne_cable(Rm * 4.0, Ri, d, x, V0, end, L),
     phrases=("Cable steady state", "Passive cable steady state"),
     examples=[_ex({"Rm": "20000 ohm*cm^2", "Ri": "100 ohm*cm", "d": "2 um", "x": "300 um", "V0": "10 mV", "end": "sealed", "L": "1 mm"}),
               _ex({"Rm": "10000 ohm*cm^2", "Ri": "200 ohm*cm", "d": "4 um", "x": "500 um", "V0": "-5 mV", "end": "infinite"})],
     level="expert",
     extra_q=[{"q": "Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = sealed", "outcome": "INVALID_INPUT"},
              {"q": "Cable steady state: Rm = 20000 ohm*cm^2, Ri = 100 ohm*cm, d = 2 um, x = 300 um, V0 = 10 mV, end = open, L = 1 mm", "outcome": "INVALID_INPUT"}])


def _g(u):
    return 1.0 if u == 0 else u / -math.expm1(-u)


def _hh_rates(v):
    """Hodgkin-Huxley (1952) rate functions, V in mV (rest near -65 mV), rates in 1/ms."""
    an, bn = 0.1 * _g((v + 55.0) / 10.0), 0.125 * math.exp(-(v + 65.0) / 80.0)
    am, bm = 1.0 * _g((v + 40.0) / 10.0), 4.0 * math.exp(-(v + 65.0) / 18.0)
    ah, bh = 0.07 * math.exp(-(v + 65.0) / 20.0), 1.0 / (1.0 + math.exp(-(v + 35.0) / 10.0))
    return (am, bm), (ah, bh), (an, bn)


def _hh_rates_ref(v):
    """The same model re-typed in its printed form (reference only): alpha_n = 0.01 (V + 55)/(1 - exp(-(V + 55)/10)), ..."""
    an = 0.01 * (v + 55.0) / (1.0 - math.exp(-(v + 55.0) / 10.0))
    bn = 0.125 * math.exp(-(v + 65.0) / 80.0)
    am = 0.1 * (v + 40.0) / (1.0 - math.exp(-(v + 40.0) / 10.0))
    bm = 4.0 * math.exp(-(v + 65.0) / 18.0)
    ah = 0.07 * math.exp(-(v + 65.0) / 20.0)
    bh = 1.0 / (1.0 + math.exp(-(v + 35.0) / 10.0))
    return (am, bm), (ah, bh), (an, bn)


def ne_hh_gating(V):
    _fin("V", V)
    v = V * 1000.0
    _need(-150.0 <= v <= 100.0, "V must lie between -150 mV and +100 mV")
    (am, bm), (ah, bh), (an, bn) = _hh_rates(v)
    return _out(m_inf=am / (am + bm), h_inf=ah / (ah + bh), n_inf=an / (an + bn))


def _ref_ne_hh_gating(V):
    out = {}
    for nm, (a, b) in zip(("m_inf", "h_inf", "n_inf"), _hh_rates_ref(V * 1000.0)):
        y = _ivp(lambda t, x: [a * (1.0 - x[0]) - b * x[0]], 60.0 / (a + b), [0.0], atol=[1e-22])
        out[nm] = y[0]
    return out


_add("bmed_hh_gating_steady_state",
     name="Hodgkin-Huxley model: steady-state gating variables at a membrane potential", topic=_NE,
     purpose="The steady-state values of the Hodgkin-Huxley gating variables m, h and n at a clamped membrane potential, from the 1952 squid-axon rate functions.",
     inputs=[_phys("V", "voltage", "clamped membrane potential")],
     outputs=[_o("m_inf", "number", "Na activation at steady state"), _o("h_inf", "number", "Na inactivation at steady state"),
              _o("n_inf", "number", "K activation at steady state")],
     primary="n_inf", conditions=["Hodgkin-Huxley 1952 rate functions, modern convention (rest near -65 mV), 6.3 degC",
                                  "-150 mV <= V <= +100 mV"],
     method="x_inf = alpha_x / (alpha_x + beta_x) for x = m, h, n",
     latex=r"x_\infty(V) = \frac{\alpha_x(V)}{\alpha_x(V) + \beta_x(V)}",
     run=ne_hh_gating, reference=_ref_ne_hh_gating,
     rand=lambda rng: {"V": _sig(rng.uniform(-0.1, 0.04))},
     wrong=lambda V: (lambda r: _out(m_inf=r[0][0] / r[0][1], h_inf=r[1][0] / (r[1][0] + r[1][1]), n_inf=r[2][0] / (r[2][0] + r[2][1])))(_hh_rates(V * 1000.0)),
     phrases=("Hodgkin-Huxley gating steady state", "HH gating steady state"),
     examples=[_ex({"V": "-65 mV"}), _ex({"V": "-20 mV"})],
     level="intermediate")


def _hh_iv(v, gna, gk, gl, ena, ek, el, mpow=3):
    (am, bm), (ah, bh), (an, bn) = _hh_rates(v)
    m, h, n = am / (am + bm), ah / (ah + bh), an / (an + bn)
    return gna * m ** mpow * h * (v - ena) + gk * n ** 4 * (v - ek) + gl * (v - el)


def _hh_rest(gNa, gK, gL, ENa, EK, EL, mpow=3):
    g = (gNa / 10.0, gK / 10.0, gL / 10.0)                            # S/m^2 -> mS/cm^2
    e = (ENa * 1000.0, EK * 1000.0, EL * 1000.0)                      # V -> mV
    f = lambda v: _hh_iv(v, *g, *e, mpow=mpow)
    grid = [-120.0 + 0.25 * i for i in range(721)]
    vals = [f(v) for v in grid]
    br = [(a, b) for a, b, fa, fb in zip(grid, grid[1:], vals, vals[1:]) if (fa > 0) != (fb > 0) or fa == 0]
    _need(len(br) == 1, "the zero-current steady state is not unique between -120 and +60 mV")
    return _bisect(f, *br[0]) / 1000.0


def _hh_stable(v, gNa, gK, gL, ENa, EK, EL):
    """Routh-Hurwitz test of the full 4-variable model linearised at the zero-current state (finite differences)."""
    g = (gNa / 10.0, gK / 10.0, gL / 10.0)
    e = (ENa * 1000.0, EK * 1000.0, EL * 1000.0)

    def F(y):
        vv, m, h, n = y
        (am, bm), (ah, bh), (an, bn) = _hh_rates(vv)
        return [-(g[0] * m ** 3 * h * (vv - e[0]) + g[1] * n ** 4 * (vv - e[1]) + g[2] * (vv - e[2])),
                am * (1 - m) - bm * m, ah * (1 - h) - bh * h, an * (1 - n) - bn * n]
    (am, bm), (ah, bh), (an, bn) = _hh_rates(v)
    y0 = [v, am / (am + bm), ah / (ah + bh), an / (an + bn)]
    J = [[0.0] * 4 for _ in range(4)]
    for j in range(4):
        hstep = 1e-6 * (1.0 if j == 0 else 0.01)
        yp, ym = list(y0), list(y0)
        yp[j] += hstep
        ym[j] -= hstep
        fp, fm = F(yp), F(ym)
        for i in range(4):
            J[i][j] = (fp[i] - fm[i]) / (2 * hstep)
    # characteristic polynomial x^4 + c1 x^3 + c2 x^2 + c3 x + c4 by Faddeev-LeVerrier
    n, M, c = 4, [[0.0] * 4 for _ in range(4)], [1.0]
    for k in range(1, n + 1):
        AM = [[sum(J[i][l] * M[l][j] for l in range(4)) for j in range(4)] for i in range(4)]
        if k == 1:
            AM = [row[:] for row in J]
        ck = -sum(AM[i][i] for i in range(4)) / k
        c.append(ck)
        M = [[AM[i][j] + (ck if i == j else 0.0) for j in range(4)] for i in range(4)]
    c1, c2, c3, c4 = c[1:]
    return c1 > 0 and c2 > 0 and c3 > 0 and c4 > 0 and c1 * c2 > c3 and c1 * c2 * c3 > c3 * c3 + c1 * c1 * c4


def ne_hh_rest(gNa, gK, gL, ENa, EK, EL):
    _pos("the conductances", gNa, gK, gL)
    _fin("the reversal potentials", ENa, EK, EL)
    _need(EK < EL < ENa, "the reversal potentials must satisfy EK < EL < ENa")
    v = _hh_rest(gNa, gK, gL, ENa, EK, EL)
    _need(_hh_stable(v * 1000.0, gNa, gK, gL, ENa, EK, EL),
          "the zero-current state is unstable (the model fires repetitively): there is no resting potential")
    return _out(V_rest=v)


def _ref_ne_hh_rest(gNa, gK, gL, ENa, EK, EL):
    gna, gk, gl = gNa / 10.0, gK / 10.0, gL / 10.0
    ena, ek, el = ENa * 1000.0, EK * 1000.0, EL * 1000.0

    def f(t, y):
        v, m, h, n = y
        (am, bm), (ah, bh), (an, bn) = _hh_rates_ref(v)
        return [-(gna * m ** 3 * h * (v - ena) + gk * n ** 4 * (v - ek) + gl * (v - el)),    # C_m = 1 uF/cm^2, ms
                am * (1 - m) - bm * m, ah * (1 - h) - bh * h, an * (1 - n) - bn * n]
    v0 = -65.0
    (am, bm), (ah, bh), (an, bn) = _hh_rates_ref(v0)
    y = [v0, am / (am + bm), ah / (ah + bh), an / (an + bn)]
    from scipy.optimize import fsolve
    ys = fsolve(lambda q: f(0, q), y, xtol=1e-14)                      # the steady state of the full 4-variable system
    if max(abs(v) for v in f(0, ys)) > 1e-10:
        raise RuntimeError("no steady state found")
    yp = [ys[0] + 1e-3, ys[1], ys[2], ys[3]]                            # stability: a small kick must die away
    ye = _ivp(f, 500.0, yp, rtol=1e-11, atol=[1e-14] * 4)
    if abs(ye[0] - ys[0]) > 0.5e-3:
        raise RuntimeError("the steady state is not stable")
    return {"V_rest": float(ys[0]) / 1000.0}


def _rand_ne_hh_rest(rng):
    return {"gNa": _sig(rng.uniform(800, 1600), 3), "gK": _sig(rng.uniform(250, 500), 3), "gL": _sig(rng.uniform(1.0, 5.0), 3),
            "ENa": _sig(rng.uniform(0.045, 0.06), 3), "EK": _sig(rng.uniform(-0.085, -0.072), 3), "EL": _sig(rng.uniform(-0.064, -0.055), 3)}


_add("bmed_hh_resting_potential",
     name="Hodgkin-Huxley model: zero-current steady (resting) membrane potential", topic=_NE,
     purpose="The membrane potential at which the steady-state Na, K and leak currents of the Hodgkin-Huxley model sum to zero (the resting potential), from the maximal conductances and reversal potentials.",
     inputs=[_phys("gNa", "conductance_per_area", "maximal Na conductance"), _phys("gK", "conductance_per_area", "maximal K conductance"),
             _phys("gL", "conductance_per_area", "leak conductance"), _phys("ENa", "voltage", "Na reversal potential"),
             _phys("EK", "voltage", "K reversal potential"), _phys("EL", "voltage", "leak reversal potential")],
     outputs=[_o("V_rest", "voltage", "zero-current steady-state potential")],
     primary="V_rest", conditions=["Hodgkin-Huxley 1952 rate functions (modern convention), gates at steady state",
                                   "a unique zero-current potential between -120 and +60 mV (checked)",
                                   "that state stable (Routh-Hurwitz test of the linearised model; otherwise refused)", "EK < EL < ENa"],
     method="root of gNa m_inf^3 h_inf (V - ENa) + gK n_inf^4 (V - EK) + gL (V - EL) = 0 (scan and bisection)",
     latex=r"\bar g_{Na} m_\infty^3 h_\infty (V - E_{Na}) + \bar g_K n_\infty^4 (V - E_K) + g_L (V - E_L) = 0",
     run=ne_hh_rest, reference=_ref_ne_hh_rest, rand=_rand_ne_hh_rest,
     wrong=lambda gNa, gK, gL, ENa, EK, EL: _out(V_rest=_hh_rest(gNa, gK, gL, ENa, EK, EL, mpow=2)),
     phrases=("Hodgkin-Huxley resting potential", "HH resting potential"),
     examples=[_ex({"gNa": "120 mS/cm^2", "gK": "36 mS/cm^2", "gL": "0.3 mS/cm^2", "ENa": "50 mV", "EK": "-77 mV", "EL": "-54.387 mV"}),
               _ex({"gNa": "100 mS/cm^2", "gK": "40 mS/cm^2", "gL": "0.2 mS/cm^2", "ENa": "55 mV", "EK": "-80 mV", "EL": "-60 mV"})],
     level="expert")


def ne_hh_gk_clamp(gK_max, V_hold, V_step, t):
    _pos("gK_max", gK_max)
    _fin("V_hold and V_step", V_hold, V_step)
    _nonneg("the time t", t)
    for vv in (V_hold, V_step):
        _need(-0.15 <= vv <= 0.1, "potentials must lie between -150 mV and +100 mV")
    (_, _), (_, _), (a0, b0) = _hh_rates(V_hold * 1000.0)
    (_, _), (_, _), (a1, b1) = _hh_rates(V_step * 1000.0)
    n0, n1 = a0 / (a0 + b0), a1 / (a1 + b1)
    n = n1 - (n1 - n0) * math.exp(-(a1 + b1) * t * 1000.0)
    return _out(g_K=gK_max * n ** 4, n=n)


def _ref_ne_hh_gk_clamp(gK_max, V_hold, V_step, t):
    (_, _), (_, _), (a0, b0) = _hh_rates_ref(V_hold * 1000.0)
    (_, _), (_, _), (a1, b1) = _hh_rates_ref(V_step * 1000.0)
    y = _ivp(lambda s, x: [a1 * (1 - x[0]) - b1 * x[0]], t * 1000.0, [a0 / (a0 + b0)], atol=[1e-20])
    return {"g_K": gK_max * y[0] ** 4, "n": y[0]}


_add("bmed_hh_potassium_conductance_clamp",
     name="Hodgkin-Huxley model: potassium conductance after a voltage-clamp step", topic=_NE,
     purpose="The potassium conductance gK = gK_max n^4 and the gate n a time t after the membrane is stepped from a holding potential (at steady state) to a new clamped potential, in the Hodgkin-Huxley model.",
     inputs=[_phys("gK_max", "conductance_per_area", "maximal K conductance"), _phys("V_hold", "voltage", "holding potential before the step"),
             _phys("V_step", "voltage", "clamped potential after the step"), _phys("t", "time", "time after the step")],
     outputs=[_o("g_K", "conductance_per_area", "K conductance at time t"), _o("n", "number", "K gate n at time t")],
     primary="g_K", conditions=["ideal voltage clamp", "gates at steady state at the holding potential", "Hodgkin-Huxley 1952 rate functions"],
     method="n(t) = n_inf(V1) - (n_inf(V1) - n_inf(V0)) exp(-(alpha_n + beta_n) t); g_K = gK_max n^4",
     latex=r"n(t) = n_\infty - (n_\infty - n_0)e^{-t/\tau_n},\quad g_K = \bar g_K n^4",
     run=ne_hh_gk_clamp, reference=_ref_ne_hh_gk_clamp,
     rand=lambda rng: {"gK_max": _sig(rng.uniform(200, 500)), "V_hold": _sig(rng.uniform(-0.09, -0.06)), "V_step": _sig(rng.uniform(-0.05, 0.05)),
                       "t": _sig(rng.uniform(0.0002, 0.01))},
     wrong=lambda gK_max, V_hold, V_step, t: (lambda r: dict(r, g_K=_r(gK_max * r["n"] ** 3)))(ne_hh_gk_clamp(gK_max, V_hold, V_step, t)),
     phrases=("Hodgkin-Huxley potassium conductance", "HH potassium conductance clamp"),
     examples=[_ex({"gK_max": "36 mS/cm^2", "V_hold": "-65 mV", "V_step": "0 mV", "t": "2 ms"}),
               _ex({"gK_max": "36 mS/cm^2", "V_hold": "-70 mV", "V_step": "-30 mV", "t": "5 ms"})],
     level="advanced")


def ne_psp(tau_m, tau_s, R_m, I0, t):
    _pos("tau_m, tau_s and R_m", tau_m, tau_s, R_m)
    _fin("I0", I0)
    _need(I0 != 0, "the synaptic current amplitude I0 must not be zero")
    _nonneg("the time t", t)
    _need(abs(tau_s - tau_m) > 1e-6 * max(tau_s, tau_m), "tau_s equals tau_m: the difference-of-exponentials form does not apply")
    a = R_m * I0 * tau_s / (tau_s - tau_m)
    V = lambda s: a * (math.exp(-s / tau_s) - math.exp(-s / tau_m))
    tp = tau_m * tau_s * math.log(tau_s / tau_m) / (tau_s - tau_m)
    return _out(V_t=V(t), t_peak=tp, V_peak=V(tp))


def _ref_ne_psp(tau_m, tau_s, R_m, I0, t):
    fun = lambda s, y: [(-y[0] + R_m * y[1]) / tau_m, -y[1] / tau_s]
    at = [1e-22, 1e-22]
    y = _ivp(fun, t, [0.0, I0], atol=at)
    g = lambda y: (-y[0] + R_m * y[1]) * (1.0 if I0 > 0 else -1.0)
    gdot = lambda s, y: (-fun(s, y)[0] + R_m * fun(s, y)[1]) * (1.0 if I0 > 0 else -1.0)
    tp, yp = _ivp_root(fun, [0.0, I0], g, gdot, 40.0 * max(tau_m, tau_s), atol=at)
    return {"V_t": y[0], "t_peak": tp, "V_peak": yp[0]}


def _rand_ne_psp(rng):
    tm = _sig(rng.uniform(0.005, 0.03))
    ts = _sig(tm * (_lu(rng, 0.05, 0.7) if rng.random() < 0.7 else _lu(rng, 1.5, 5)))
    return {"tau_m": tm, "tau_s": ts, "R_m": _sig(_lu(rng, 1e7, 5e8)), "I0": _sig(rng.uniform(1e-11, 5e-10) * rng.choice([-1, 1])),
            "t": _sig(rng.uniform(0.0005, 0.06))}


_add("bmed_postsynaptic_potential",
     name="Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current", topic=_NE,
     purpose="The postsynaptic potential of a passive RC membrane receiving a synaptic current I0 exp(-t/tau_s) at t = 0: its value at time t, the time of its peak and the peak value.",
     inputs=[_phys("tau_m", "time", "membrane time constant"), _phys("tau_s", "time", "synaptic current decay time constant"),
             _phys("R_m", "resistance", "membrane resistance"), _phys("I0", "current", "peak synaptic current (negative for inward)"),
             _phys("t", "time", "time after the synaptic event")],
     outputs=[_o("V_t", "voltage", "potential change at time t"), _o("t_peak", "time", "time of the peak"),
              _o("V_peak", "voltage", "peak potential change")],
     primary="V_peak", conditions=["passive membrane starting at rest", "current jumps to I0 and decays exponentially", "tau_s != tau_m"],
     method="tau_m dV/dt = -V + R I0 exp(-t/tau_s): V = R I0 tau_s/(tau_s - tau_m) (exp(-t/tau_s) - exp(-t/tau_m)); t_peak = tau_m tau_s ln(tau_s/tau_m)/(tau_s - tau_m)",
     latex=r"V(t) = \frac{R I_0 \tau_s}{\tau_s - \tau_m}\left(e^{-t/\tau_s} - e^{-t/\tau_m}\right),\quad t_{peak} = \frac{\tau_m\tau_s}{\tau_s - \tau_m}\ln\frac{\tau_s}{\tau_m}",
     run=ne_psp, reference=_ref_ne_psp, rand=_rand_ne_psp,
     wrong=lambda tau_m, tau_s, R_m, I0, t: (lambda r: dict(r, t_peak=_r(tau_m * tau_s * math.log(tau_s / tau_m) / (tau_s + tau_m))))(ne_psp(tau_m, tau_s, R_m, I0, t)),
     phrases=("Postsynaptic potential", "PSP of a passive membrane"),
     examples=[_ex({"tau_m": "20 ms", "tau_s": "5 ms", "R_m": "100 Mohm", "I0": "50 pA", "t": "10 ms"}),
               _ex({"tau_m": "10 ms", "tau_s": "2 ms", "R_m": "200 Mohm", "I0": "-30 pA", "t": "3 ms"})],
     level="advanced")


def ne_fhn(a, b, eps, I):
    _fin("a, b, eps and I", a, b, eps, I)
    _need(0 < b < 1, "this calculation needs 0 < b < 1 (then the fixed point is unique)")
    _pos("eps", eps)
    h = lambda v: v - v ** 3 / 3.0 - (v + a) / b + I
    B = 1.0
    while h(-B) <= 0 or h(B) >= 0:
        B *= 2.0
    v = _bisect(h, -B, B)
    w = (v + a) / b
    tr = 1.0 - v * v - eps * b
    det = eps * (1.0 - b + b * v * v)
    return _out(v_star=v, w_star=w, trace=tr, determinant=det, stable=bool(tr < 0))


def _ref_ne_fhn(a, b, eps, I):
    import numpy as np
    rts = np.roots([-1.0 / 3.0, 0.0, 1.0 - 1.0 / b, I - a / b])
    v = float(np.real(rts[int(np.argmin(np.abs(np.imag(rts))))]))
    w = (v + a) / b
    F = lambda vv, ww: (vv - vv ** 3 / 3.0 - ww + I, eps * (vv + a - b * ww))
    hstep = 1e-20                                    # complex-step derivatives of the vector field
    J = np.array([[F(v + 1j * hstep, w)[0].imag / hstep, F(v, w + 1j * hstep)[0].imag / hstep],
                  [F(v + 1j * hstep, w)[1].imag / hstep, F(v, w + 1j * hstep)[1].imag / hstep]])
    ev = np.linalg.eigvals(J)
    return {"v_star": v, "w_star": w, "trace": float(np.real(ev.sum())), "determinant": float(np.real(ev.prod())),
            "stable": bool(np.all(np.real(ev) < 0))}


def _rand_ne_fhn(rng):
    while True:
        r = {"a": round(rng.uniform(0.5, 1.0), 3), "b": round(rng.uniform(0.3, 0.95), 3), "eps": round(rng.uniform(0.02, 0.2), 3),
             "I": round(rng.uniform(0.0, 1.5), 3)}
        v = ne_fhn(**r)
        if abs(v["trace"]) > 1e-3:
            return r


_add("bmed_fitzhugh_nagumo_fixed_point",
     name="FitzHugh-Nagumo neuron model: fixed point and its stability", topic=_NE,
     purpose="The resting (fixed) point of the FitzHugh-Nagumo model v' = v - v^3/3 - w + I, w' = eps (v + a - b w), the trace and determinant of its Jacobian and whether it is stable (resting) or unstable (repetitive firing).",
     inputs=[_numin("a", "parameter a"), _numin("b", "parameter b (0 < b < 1)"), _numin("eps", "time-scale ratio eps (> 0)"),
             _numin("I", "applied current (dimensionless)")],
     outputs=[_o("v_star", "number", "fixed-point v"), _o("w_star", "number", "fixed-point w"), _o("trace", "number", "trace of the Jacobian"),
              _o("determinant", "number", "determinant of the Jacobian"), _o("stable", "number", "true when the fixed point is stable")],
     primary="v_star", conditions=["dimensionless FitzHugh-Nagumo model", "0 < b < 1 (a unique fixed point)"],
     method="v* solves v - v^3/3 - (v + a)/b + I = 0 (bisection); w* = (v* + a)/b; J = [[1 - v*^2, -1], [eps, -eps b]]; stable when trace < 0 (det > 0 here)",
     latex=r"v - \frac{v^3}{3} - \frac{v + a}{b} + I = 0,\quad J = \begin{pmatrix}1 - v^2 & -1\\ \varepsilon & -\varepsilon b\end{pmatrix}",
     run=ne_fhn, reference=_ref_ne_fhn, rand=_rand_ne_fhn,
     wrong=lambda a, b, eps, I: (lambda r: dict(r, w_star=_r((r["v_star"] - a) / b)))(ne_fhn(a, b, eps, I)),
     phrases=("FitzHugh-Nagumo fixed point", "FitzHugh-Nagumo equilibrium"),
     examples=[_ex({"a": 0.7, "b": 0.8, "eps": 0.08, "I": 0}), _ex({"a": 0.7, "b": 0.8, "eps": 0.08, "I": 0.5})],
     level="advanced",
     extra_q=[{"q": "FitzHugh-Nagumo fixed point: a = 0.7, b = 1.5, eps = 0.08, I = 0", "outcome": "INVALID_INPUT"}])


# ================================================================================================ MEDICAL-IMAGING PHYSICS
_IM = "medical-imaging physics"


def im_xray(I0, mu_rho, rho, x):
    _pos("I0", I0)
    n = len(mu_rho)
    _need(n >= 1 and len(rho) == n and len(x) == n, "give one mass attenuation coefficient, density and thickness per layer")
    for v in list(mu_rho) + list(rho):
        _need(v > 0, "attenuation coefficients and densities must be positive")
    for v in x:
        _need(v >= 0, "thicknesses cannot be negative")
    T = math.exp(-math.fsum(a * b * c for a, b, c in zip(mu_rho, rho, x)))
    return _out(T=T, I=I0 * T)


def _ref_im_xray(I0, mu_rho, rho, x):
    y = [1.0]
    for a, b, c in zip(mu_rho, rho, x):                     # dI/ds = -mu I through each layer in turn
        if c > 0:
            y = _ivp(lambda s, yy: [-a * b * yy[0]], c, y, atol=[1e-300])
    return {"T": y[0], "I": I0 * y[0]}


def _rand_im_xray(rng):
    n = rng.randint(1, 5)
    return {"I0": _sig(_lu(rng, 1, 1e4)), "mu_rho": [_sig(_lu(rng, 0.15, 5.0), 3) for _ in range(n)],
            "rho": [_sig(rng.choice([rng.uniform(0.2, 0.9), rng.uniform(1.1, 2.5)]), 3) for _ in range(n)],
            "x": [_sig(rng.uniform(0.1, 6.0), 3) for _ in range(n)]}


_add("bmed_xray_attenuation_layers",
     name="X-ray attenuation through tissue layers (narrow monoenergetic beam)", topic=_IM,
     purpose="The transmitted fraction and intensity of a narrow monoenergetic X-ray beam crossing successive tissue layers, from each layer's mass attenuation coefficient, density and thickness.",
     inputs=[_phys("I0", "intensity", "incident intensity"), _vecin("mu_rho", "mass attenuation coefficient of each layer, cm^2/g", 1, 20),
             _vecin("rho", "density of each layer, g/cm^3", 1, 20), _vecin("x", "thickness of each layer, cm", 1, 20)],
     outputs=[_o("T", "number", "transmitted fraction"), _o("I", "intensity", "transmitted intensity")],
     primary="T", conditions=["narrow beam (no scatter reaching the detector)", "one photon energy", "units cm^2/g, g/cm^3, cm"],
     method="T = exp(-sum (mu/rho)_i rho_i x_i); I = I0 T", latex=r"\frac{I}{I_0} = \exp\left(-\sum_i \left(\frac{\mu}{\rho}\right)_i \rho_i x_i\right)",
     run=im_xray, reference=_ref_im_xray, rand=_rand_im_xray,
     wrong=lambda I0, mu_rho, rho, x: (lambda T: _out(T=T, I=I0 * T))(math.exp(-math.fsum(a * c for a, c in zip(mu_rho, x)))),
     phrases=("X-ray attenuation through layers", "X-ray tissue attenuation"),
     examples=[_ex({"I0": "1000 W/m^2", "mu_rho": [0.2, 0.5, 0.2], "rho": [1.0, 1.85, 1.0], "x": [2, 1, 3]}),
               _ex({"I0": "50 W/m^2", "mu_rho": [0.18], "rho": [1.06], "x": [10]})],
     level="beginner")


def im_spin_echo(M0, TR, TE, T1, T2):
    _pos("M0, TR, TE, T1 and T2", M0, TR, TE, T1, T2)
    _need(TE < TR, "TE must be shorter than TR")
    return _out(S=M0 * -math.expm1(-TR / T1) * math.exp(-TE / T2))


def _bloch_gen(T1, T2, M0):
    import numpy as np
    return np.array([[-1 / T2, 0, 0, 0], [0, -1 / T2, 0, 0], [0, 0, -1 / T1, M0 / T1], [0, 0, 0, 0]], dtype=float)


def _rotx(alpha):
    import numpy as np
    c, s = math.cos(alpha), math.sin(alpha)
    return np.array([[1, 0, 0, 0], [0, c, s, 0], [0, -s, c, 0], [0, 0, 0, 1]], dtype=float)


def _bloch_ss(seq, M0):
    """Repeat a pulse sequence (a list of 4x4 steps; 'read' marks the readout, 'spoil' zeroes Mxy) to steady state."""
    import numpy as np
    M = np.array([0.0, 0.0, M0, 1.0])
    read = None
    for _ in range(200000):
        prev = M.copy()
        for st in seq:
            if isinstance(st, str):
                if st == "read":
                    read = M.copy()
                elif st == "spoil":
                    M[0] = M[1] = 0.0
            else:
                M = st @ M
        if np.max(np.abs(M - prev)) <= 1e-17 * max(1.0, M0):
            return read, M
    raise RuntimeError("no steady state")


def _ref_im_spin_echo(M0, TR, TE, T1, T2):
    from scipy.linalg import expm
    A = _bloch_gen(T1, T2, M0)
    read, _ = _bloch_ss([_rotx(math.pi / 2), expm(A * TE), "read", expm(A * (TR - TE)), "spoil"], M0)
    return {"S": math.hypot(read[0], read[1])}


_add("bmed_mri_spin_echo_signal",
     name="MRI spin-echo signal from T1, T2, TR and TE (simplified model)", topic=_IM,
     purpose="The steady-state spin-echo signal S = M0 (1 - exp(-TR/T1)) exp(-TE/T2) of a tissue with relaxation times T1 and T2, for repetition time TR and echo time TE.",
     inputs=[_numin("M0", "equilibrium magnetisation (relative proton density)"), _phys("TR", "time", "repetition time"),
             _phys("TE", "time", "echo time"), _phys("T1", "time", "longitudinal relaxation time"), _phys("T2", "time", "transverse relaxation time")],
     outputs=[_o("S", "number", "signal in the units of M0")],
     primary="S", conditions=["90 degree excitations every TR, transverse magnetisation spoiled before each", "ideal refocusing without effect on Mz (TE << TR)",
                              "TE < TR"],
     method="S = M0 (1 - exp(-TR/T1)) exp(-TE/T2)", latex=r"S = M_0\left(1 - e^{-TR/T_1}\right)e^{-TE/T_2}",
     run=im_spin_echo, reference=_ref_im_spin_echo,
     rand=lambda rng: (lambda T1: {"M0": _sig(rng.uniform(0.5, 1.0), 3), "TR": _sig(T1 * rng.uniform(0.2, 5)), "TE": _sig(rng.uniform(0.005, 0.12)),
                                   "T1": T1, "T2": _sig(rng.uniform(0.02, 0.3))})(_sig(rng.uniform(0.2, 2.5))),
     wrong=lambda M0, TR, TE, T1, T2: _out(S=M0 * -math.expm1(-TE / T1) * math.exp(-TR / T2)),
     phrases=("Spin-echo signal", "MRI spin-echo signal"),
     examples=[_ex({"M0": 1, "TR": "500 ms", "TE": "15 ms", "T1": "900 ms", "T2": "100 ms"}),
               _ex({"M0": 0.8, "TR": "3 s", "TE": "100 ms", "T1": "2.5 s", "T2": "250 ms"})],
     level="beginner",
     extra_q=[{"q": "Spin-echo signal: M0 = 1, TR = 10 ms, TE = 15 ms, T1 = 900 ms, T2 = 100 ms", "outcome": "INVALID_INPUT"}])


def im_inversion_recovery(T1, TR, TI):
    _pos("T1, TR and TI", T1, TR, TI)
    _need(TI < TR, "TI must be shorter than TR")
    e = math.exp(-TR / T1)
    return _out(Mz_ratio=1.0 - 2.0 * math.exp(-TI / T1) + e, TI_null=T1 * math.log(2.0 / (1.0 + e)))


def _ref_im_inversion_recovery(T1, TR, TI):
    import numpy as np
    from scipy.linalg import expm
    A = np.array([[-1 / T1, 1 / T1], [0.0, 0.0]])        # longitudinal Bloch equation, M0 = 1, augmented
    inv = np.array([[-1.0, 0.0], [0.0, 1.0]])

    def mz(ti):
        E1, E2 = expm(A * ti), expm(A * (TR - ti))
        M = np.array([1.0, 1.0])
        for _ in range(200000):
            prev = M.copy()
            M = E1 @ (inv @ M)
            read = M[0]
            M = E2 @ np.array([0.0, 1.0])                    # 90 degree readout leaves Mz = 0, then recovery
            if abs(M[0] - prev[0]) <= 1e-17:
                return read
        raise RuntimeError("no steady state")
    return {"Mz_ratio": float(mz(TI)), "TI_null": _brentq(mz, 1e-9 * TR, TR * (1 - 1e-9))}


_add("bmed_mri_inversion_recovery",
     name="MRI inversion recovery: longitudinal magnetisation at TI and the null point", topic=_IM,
     purpose="The longitudinal magnetisation (fraction of M0, signed) at inversion time TI in a repeated inversion-recovery sequence with repetition time TR, and the inversion time that nulls the tissue.",
     inputs=[_phys("T1", "time", "longitudinal relaxation time"), _phys("TR", "time", "repetition time"), _phys("TI", "time", "inversion time")],
     outputs=[_o("Mz_ratio", "number", "Mz/M0 at TI (negative before the null)"), _o("TI_null", "time", "inversion time of zero signal")],
     primary="TI_null", conditions=["ideal 180 degree inversion and 90 degree readout at TI", "readout leaves Mz = 0", "steady state, TI < TR"],
     method="Mz(TI)/M0 = 1 - 2 exp(-TI/T1) + exp(-TR/T1); null at TI = T1 ln(2/(1 + exp(-TR/T1)))",
     latex=r"\frac{M_z(TI)}{M_0} = 1 - 2e^{-TI/T_1} + e^{-TR/T_1},\quad TI_{null} = T_1\ln\frac{2}{1 + e^{-TR/T_1}}",
     run=im_inversion_recovery, reference=_ref_im_inversion_recovery,
     rand=lambda rng: (lambda T1: (lambda TR: {"T1": T1, "TR": TR, "TI": _sig(TR * rng.uniform(0.05, 0.9))})(_sig(T1 * rng.uniform(0.8, 6))))(_sig(rng.uniform(0.2, 3.0))),
     wrong=lambda T1, TR, TI: (lambda r: dict(r, TI_null=_r(T1 * LN2)))(im_inversion_recovery(T1, TR, TI)),
     phrases=("Inversion recovery", "MRI inversion recovery"),
     examples=[_ex({"T1": "250 ms", "TR": "2000 ms", "TI": "150 ms"}), _ex({"T1": "4 s", "TR": "9 s", "TI": "2.5 s"})],
     level="intermediate")


def im_spoiled_gre(M0, TR, TE, T1, T2star, alpha):
    _pos("M0, TR, TE, T1 and T2star", M0, TR, TE, T1, T2star)
    _need(TE < TR, "TE must be shorter than TR")
    _need(0 < alpha <= math.pi / 2, "the flip angle must lie between 0 and 90 degrees")
    E1 = math.exp(-TR / T1)
    return _out(S=M0 * math.sin(alpha) * (1 - E1) / (1 - math.cos(alpha) * E1) * math.exp(-TE / T2star))


def _ref_im_spoiled_gre(M0, TR, TE, T1, T2star, alpha):
    from scipy.linalg import expm
    A = _bloch_gen(T1, T2star, M0)
    read, _ = _bloch_ss([_rotx(alpha), expm(A * TE), "read", expm(A * (TR - TE)), "spoil"], M0)
    return {"S": math.hypot(read[0], read[1])}


_add("bmed_mri_spoiled_gradient_echo",
     name="MRI spoiled gradient-echo signal (Ernst equation)", topic=_IM,
     purpose="The steady-state signal of a spoiled gradient-echo sequence with flip angle alpha, repetition time TR and echo time TE for a tissue with T1 and T2*.",
     inputs=[_numin("M0", "equilibrium magnetisation (relative proton density)"), _phys("TR", "time", "repetition time"),
             _phys("TE", "time", "echo time"), _phys("T1", "time", "longitudinal relaxation time"),
             _phys("T2star", "time", "effective transverse relaxation time T2*"), _phys("alpha", "angle", "flip angle")],
     outputs=[_o("S", "number", "signal in the units of M0")],
     primary="S", conditions=["ideal spoiling of transverse magnetisation before each pulse", "steady state", "0 < alpha <= 90 deg, TE < TR"],
     method="S = M0 sin(alpha) (1 - E1)/(1 - cos(alpha) E1) exp(-TE/T2*), E1 = exp(-TR/T1)",
     latex=r"S = M_0 \sin\alpha\,\frac{1 - E_1}{1 - E_1\cos\alpha}\,e^{-TE/T_2^*},\quad E_1 = e^{-TR/T_1}",
     run=im_spoiled_gre, reference=_ref_im_spoiled_gre,
     rand=lambda rng: (lambda T1: {"M0": _sig(rng.uniform(0.5, 1.0), 3), "TR": _sig(T1 * rng.uniform(0.005, 0.5)), "TE": _sig(rng.uniform(0.001, 0.004)),
                                   "T1": T1, "T2star": _sig(rng.uniform(0.01, 0.1)), "alpha": _sig(math.radians(rng.uniform(3, 90)))})(_sig(rng.uniform(0.3, 2.5))),
     wrong=lambda M0, TR, TE, T1, T2star, alpha: (lambda E1: _out(S=M0 * (1 - E1) / (1 - math.cos(alpha) * E1) * math.exp(-TE / T2star)))(math.exp(-TR / T1)),
     phrases=("Spoiled gradient-echo signal", "Ernst equation signal"),
     examples=[_ex({"M0": 1, "TR": "10 ms", "TE": "3 ms", "T1": "1000 ms", "T2star": "40 ms", "alpha": "8 deg"}),
               _ex({"M0": 0.9, "TR": "20 ms", "TE": "2 ms", "T1": "600 ms", "T2star": "30 ms", "alpha": "30 deg"})],
     level="advanced")


def im_us_interface(Z1, Z2):
    _pos("the impedances Z1 and Z2", Z1, Z2)
    r = (Z2 - Z1) / (Z2 + Z1)
    return _out(R=r * r, T=4.0 * Z1 * Z2 / (Z1 + Z2) ** 2, r_p=r, t_p=2.0 * Z2 / (Z1 + Z2))


def _ref_im_us_interface(Z1, Z2):
    import numpy as np
    pr, pt = np.linalg.solve(np.array([[1.0, -1.0], [1.0 / Z1, 1.0 / Z2]]), np.array([-1.0, 1.0 / Z1]))   # pressure and velocity continuous
    pr, pt = float(pr), float(pt)
    return {"R": pr * pr, "T": pt * pt * Z1 / Z2, "r_p": pr, "t_p": pt}


_add("bmed_ultrasound_interface",
     name="Ultrasound reflection and transmission at a tissue interface (normal incidence)", topic=_IM,
     purpose="The intensity reflection and transmission coefficients and the pressure amplitude coefficients of an ultrasound wave meeting a flat interface between two media at normal incidence.",
     inputs=[_phys("Z1", "acoustic_impedance", "acoustic impedance of the first medium"), _phys("Z2", "acoustic_impedance", "acoustic impedance of the second medium")],
     outputs=[_o("R", "number", "intensity reflection coefficient"), _o("T", "number", "intensity transmission coefficient"),
              _o("r_p", "number", "pressure reflection coefficient"), _o("t_p", "number", "pressure transmission coefficient")],
     primary="R", conditions=["normal incidence on a flat interface", "lossless media, plane waves"],
     method="r = (Z2 - Z1)/(Z2 + Z1), t = 2 Z2/(Z1 + Z2); R = r^2, T = 4 Z1 Z2/(Z1 + Z2)^2",
     latex=r"R = \left(\frac{Z_2 - Z_1}{Z_2 + Z_1}\right)^2,\quad T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2}",
     run=im_us_interface, reference=_ref_im_us_interface,
     rand=lambda rng: {"Z1": _sig(_lu(rng, 400, 8e6)), "Z2": _sig(_lu(rng, 400, 8e6))},
     wrong=lambda Z1, Z2: (lambda r: _out(R=abs(r), T=1 - abs(r), r_p=r, t_p=1 + r))((Z2 - Z1) / (Z2 + Z1)),
     phrases=("Ultrasound interface reflection", "Ultrasound reflection and transmission"),
     examples=[_ex({"Z1": "1.63 MRayl", "Z2": "7.8 MRayl"}), _ex({"Z1": "1.38e6 kg/(m^2*s)", "Z2": "429 kg/(m^2*s)"})],
     level="beginner")


def im_us_matching(Z1, Zm, Z2, d, c_m, f):
    _pos("Z1, Zm, Z2, d, c_m and f", Z1, Zm, Z2, d, c_m, f)
    kd = 2.0 * math.pi * f / c_m * d
    c, s = math.cos(kd), math.sin(kd)
    den = (Z1 + Z2) ** 2 * c * c + (Zm + Z1 * Z2 / Zm) ** 2 * s * s
    return _out(T=4.0 * Z1 * Z2 / den, kd=kd)


def _ref_im_us_matching(Z1, Zm, Z2, d, c_m, f):
    import numpy as np
    k = 2.0 * math.pi * f / c_m
    em, ep = np.exp(-1j * k * d), np.exp(1j * k * d)
    # unknowns B (reflected), C, D (layer forward/backward), E (transmitted); incident pressure 1
    A = np.array([[-1, 1, 1, 0], [1 / Z1, 1 / Zm, -1 / Zm, 0], [0, em, ep, -1], [0, em / Zm, -ep / Zm, -1 / Z2]], dtype=complex)
    b = np.array([1, 1 / Z1, 0, 0], dtype=complex)
    B, C, D, E = np.linalg.solve(A, b)
    return {"T": float(abs(E) ** 2 * Z1 / Z2), "kd": k * d}


def _rand_im_us_match(rng):
    c = _sig(rng.uniform(1500, 3000))
    f = _sig(_lu(rng, 1e6, 1e7))
    lam = c / f
    return {"Z1": _sig(_lu(rng, 1e6, 4e7)), "Zm": _sig(_lu(rng, 1e6, 1e7)), "Z2": _sig(rng.uniform(1.4e6, 1.7e6)),
            "d": _sig(lam * rng.uniform(0.05, 0.6)), "c_m": c, "f": f}


_add("bmed_ultrasound_matching_layer",
     name="Ultrasound transmission through a matching layer (normal incidence)", topic=_IM,
     purpose="The intensity transmission coefficient from a transducer (impedance Z1) through a matching layer (impedance Zm, thickness d, sound speed c_m) into tissue (Z2) at frequency f, and the layer's phase thickness.",
     inputs=[_phys("Z1", "acoustic_impedance", "impedance of the source medium (transducer)"), _phys("Zm", "acoustic_impedance", "impedance of the matching layer"),
             _phys("Z2", "acoustic_impedance", "impedance of the load medium (tissue)"), _phys("d", "length", "layer thickness"),
             _phys("c_m", "speed", "sound speed in the layer"), _phys("f", "frequency", "frequency")],
     outputs=[_o("T", "number", "intensity transmission coefficient"), _o("kd", "number", "phase thickness of the layer (rad)")],
     primary="T", conditions=["normal incidence, plane waves, lossless layers", "semi-infinite source and load media"],
     method="T = 4 Z1 Z2 / [(Z1 + Z2)^2 cos^2(kd) + (Zm + Z1 Z2/Zm)^2 sin^2(kd)], k = 2 pi f / c_m",
     latex=r"T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2\cos^2 kd + \left(Z_m + \frac{Z_1 Z_2}{Z_m}\right)^2\sin^2 kd}",
     run=im_us_matching, reference=_ref_im_us_matching, rand=_rand_im_us_match,
     wrong=lambda Z1, Zm, Z2, d, c_m, f: _out(T=4.0 * Z1 * Z2 / (Z1 + Z2) ** 2, kd=2.0 * math.pi * f / c_m * d),
     phrases=("Ultrasound matching layer", "Matching layer transmission"),
     examples=[_ex({"Z1": "30 MRayl", "Zm": "6.9 MRayl", "Z2": "1.6 MRayl", "d": "0.15 mm", "c_m": "2400 m/s", "f": "4 MHz"}),
               _ex({"Z1": "30 MRayl", "Zm": "3 MRayl", "Z2": "1.5 MRayl", "d": "0.2 mm", "c_m": "2000 m/s", "f": "2 MHz"})],
     level="advanced")


_FWHM_SIGMA = 2.0 * math.sqrt(2.0 * LN2)


def im_psf_mtf(fwhm, f):
    _pos("the FWHM", fwhm)
    _nonneg("the spatial frequency f", f)
    s = fwhm / _FWHM_SIGMA
    return _out(MTF=math.exp(-2.0 * math.pi ** 2 * s * s * f * f), f50=math.sqrt(LN2 / 2.0) / (math.pi * s))


def _ref_im_psf_mtf(fwhm, f):
    s = fwhm / _FWHM_SIGMA
    norm = _quad(lambda u: math.exp(-u * u / 2.0), 0.0, 40.0)
    mtf = lambda nu: _quad(lambda u: math.exp(-u * u / 2.0) * math.cos(2.0 * math.pi * nu * u), 0.0, 40.0) / norm   # nu = f sigma
    return {"MTF": mtf(f * s), "f50": _brentq(lambda nu: mtf(nu) - 0.5, 0.01, 1.0) / s}


_add("bmed_gaussian_psf_mtf",
     name="Imaging resolution: MTF of a Gaussian point-spread function", topic=_IM,
     purpose="The modulation transfer function at a spatial frequency of an imaging system with a Gaussian point-spread function of stated FWHM, and the frequency at which the MTF falls to 50 %.",
     inputs=[_phys("fwhm", "length", "full width at half maximum of the point-spread function"),
             _phys("f", "inverse_length", "spatial frequency (cycles per length)")],
     outputs=[_o("MTF", "number", "modulation transfer at f"), _o("f50", "inverse_length", "spatial frequency where the MTF is 0.5")],
     primary="MTF", conditions=["Gaussian, shift-invariant point-spread function", "one dimension (a line-spread function)"],
     method="sigma = FWHM/(2 sqrt(2 ln 2)); MTF = exp(-2 pi^2 sigma^2 f^2); f50 = sqrt(ln 2 / 2)/(pi sigma)",
     latex=r"\mathrm{MTF}(f) = e^{-2\pi^2\sigma^2 f^2},\quad \sigma = \frac{\mathrm{FWHM}}{2\sqrt{2\ln 2}}",
     run=im_psf_mtf, reference=_ref_im_psf_mtf,
     rand=lambda rng: (lambda fw: {"fwhm": fw, "f": _sig(rng.uniform(0.05, 1.2) / fw)})(_sig(_lu(rng, 1e-5, 5e-3))),
     wrong=lambda fwhm, f: (lambda s: _out(MTF=math.exp(-2.0 * math.pi ** 2 * s * s * f * f), f50=math.sqrt(LN2 / 2.0) / (math.pi * s)))(fwhm / 2.0),
     phrases=("Gaussian PSF MTF", "MTF of a Gaussian point-spread function"),
     examples=[_ex({"fwhm": "0.5 mm", "f": "1 1/mm"}), _ex({"fwhm": "4 mm", "f": "0.1 1/mm"})],
     level="intermediate")


def _loglin_fit_out(x, S, name):
    _need(len(x) == len(S) and len(x) >= 2, "give at least two points and one signal per point")
    slope, b = _loglin(x, S, "the signals")
    _need(slope < 0, f"the signal does not decay with {name}")
    return slope, b


def im_adc(b, S):
    slope, c = _loglin_fit_out(b, S, "b")
    return _out(ADC=-slope, S0=math.exp(c))


def _ref_loglin(x, S):
    import numpy as np
    import statsmodels.api as sm
    p = sm.OLS(np.log(np.array(S, dtype=float)), sm.add_constant(np.array(x, dtype=float))).fit().params
    return float(p[1]), float(p[0])


def _rand_im_adc(rng):
    n = rng.randint(2, 8)
    adc = _lu(rng, 2e-4, 3e-3)
    bs = sorted(set(float(rng.choice([0, 50, 100, 200, 400, 500, 600, 800, 1000, 1500, 2000])) for _ in range(n)))
    if len(bs) < 2:
        bs = [0.0, 1000.0]
    S0 = _lu(rng, 100, 2000)
    return {"b": bs, "S": [_sig(S0 * math.exp(-adc * v) * (1 + rng.uniform(-0.02, 0.02))) for v in bs]}


_add("bmed_mri_adc_fit",
     name="Diffusion MRI: apparent diffusion coefficient from signals at several b-values", topic=_IM,
     purpose="The apparent diffusion coefficient and the b = 0 signal from diffusion-weighted signals at two or more b-values, by a least-squares fit of ln S against b (mono-exponential decay).",
     inputs=[_vecin("b", "b-values in s/mm^2", 2), _vecin("S", "signals at those b-values (any one unit)", 2)],
     outputs=[_o("ADC", "number", "apparent diffusion coefficient, mm^2/s"), _o("S0", "number", "fitted signal at b = 0 (unit of S)")],
     primary="ADC", conditions=["mono-exponential decay S = S0 exp(-b ADC)", "unweighted least squares on ln S", "b in s/mm^2"],
     method="least-squares line ln S = ln S0 - ADC b", latex=r"\ln S = \ln S_0 - b\cdot ADC",
     run=im_adc, reference=lambda b, S: (lambda p: {"ADC": -p[0], "S0": math.exp(p[1])})(_ref_loglin(b, S)), rand=_rand_im_adc,
     wrong=lambda b, S: (lambda sb: _out(ADC=-sb[0] / math.log(10.0), S0=math.exp(sb[1])))(_loglin(b, S)),
     phrases=("ADC fit", "Apparent diffusion coefficient fit"),
     examples=[_ex({"b": [0, 1000], "S": [1200, 420]}), _ex({"b": [0, 200, 500, 1000], "S": [980, 820, 620, 390]})],
     level="intermediate")


def im_t2_fit(TE, S):
    slope, c = _loglin_fit_out(TE, S, "echo time")
    return _out(T2=-1.0 / slope, S0=math.exp(c))


def _rand_im_t2(rng):
    n = rng.randint(2, 10)
    T2 = _lu(rng, 20, 300)
    te = sorted(set(round(rng.uniform(5, 3 * T2), 1) for _ in range(n)))
    if len(te) < 2:
        te = [10.0, 80.0]
    S0 = _lu(rng, 100, 2000)
    return {"TE": te, "S": [_sig(S0 * math.exp(-v / T2) * (1 + rng.uniform(-0.02, 0.02))) for v in te]}


_add("bmed_mri_t2_fit",
     name="MRI multi-echo T2 estimate (log-linear fit)", topic=_IM,
     purpose="The transverse relaxation time T2 and the extrapolated signal at TE = 0 from signals at several echo times, by a least-squares fit of ln S against TE.",
     inputs=[_vecin("TE", "echo times in ms", 2), _vecin("S", "signals at those echo times (any one unit)", 2)],
     outputs=[_o("T2", "number", "transverse relaxation time, ms"), _o("S0", "number", "fitted signal at TE = 0 (unit of S)")],
     primary="T2", conditions=["mono-exponential decay S = S0 exp(-TE/T2)", "unweighted least squares on ln S", "TE in ms"],
     method="least-squares line ln S = ln S0 - TE/T2", latex=r"\ln S = \ln S_0 - \frac{TE}{T_2}",
     run=im_t2_fit, reference=lambda TE, S: (lambda p: {"T2": -1.0 / p[0], "S0": math.exp(p[1])})(_ref_loglin(TE, S)), rand=_rand_im_t2,
     wrong=lambda TE, S: (lambda sb: _out(T2=-sb[0], S0=math.exp(sb[1])))(_loglin(TE, S)),
     phrases=("Multi-echo T2 fit", "T2 fit"),
     examples=[_ex({"TE": [10, 30, 50, 80, 120], "S": [905, 740, 605, 445, 300]}), _ex({"TE": [20, 100], "S": [800, 260]})],
     level="intermediate")


# ================================================================================================ DEFINITIONAL ADDITIONS
# (owner, 2026-10-23: a capability that passes every gate ships; a weaker reference is documented). SNR averaging,
# Ernst angle, SEIR peak timing, BMI and BMR (calculations only, never advice), Hounsfield units, Larmor frequency,
# ultrasound wavelength.
_KU["magnetic_field"] = "T"
_KU["power"] = "W"
_ATOM_SI["T"] = 1.0                                # tesla, for the written examples of the Larmor frequency


def _isnum(v):
    return isinstance(v, (int, float)) and not isinstance(v, bool) and math.isfinite(v)


def snr_averaging(SNR1, N=None, SNR_target=None):
    _pos("the single-acquisition SNR", SNR1)
    _need((N is None) != (SNR_target is None), "give the number of averages N or a target SNR (one of them)")
    if N is not None:
        _need(isinstance(N, int) and 1 <= N <= 10 ** 7, "N must be a whole number from 1 to 10 million")
        return _out(SNR=SNR1 * math.sqrt(N), gain=math.sqrt(N), N=float(N))
    _pos("the target SNR", SNR_target)
    _need(SNR_target / SNR1 <= 3000, "the target needs more than 10 million averages")
    n = max(1, math.ceil((SNR_target / SNR1) ** 2 * (1 - 1e-12)))
    return _out(SNR=SNR1 * math.sqrt(n), gain=math.sqrt(n), N=float(n))


def _ref_snr_averaging(SNR1, N=None, SNR_target=None):
    """Averaging N independent acquisitions: the signal adds coherently (N S) and the noise variances add (N sigma^2),
    computed in exact arithmetic with sympy; the number needed by searching the smallest N whose SNR reaches the
    target."""
    import sympy as sp
    if N is None:
        r = sp.Rational(repr(SNR_target)) / sp.Rational(repr(SNR1))
        N = int(sp.ceiling(r ** 2 * (1 - sp.Rational(1, 10 ** 12))))
        N = max(N, 1)
    snr = sp.Rational(repr(SNR1)) * N / sp.sqrt(N)
    return {"SNR": float(snr), "gain": float(sp.sqrt(N)), "N": float(N)}


_add("bmed_snr_averaging",
     name="Signal-to-noise ratio gained by signal averaging", topic=_IM,
     purpose="The signal-to-noise ratio after averaging N acquisitions with independent noise, SNR_N = SNR_1 sqrt(N), or the smallest number of averages that reaches a target SNR.",
     inputs=[_numin("SNR1", "signal-to-noise ratio of one acquisition"), _intin("N", "number of averages (give N or SNR_target)", 1, 10 ** 7, required=False),
             _numin("SNR_target", "target SNR (give N or SNR_target)", required=False)],
     outputs=[_o("SNR", "number", "SNR after averaging"), _o("gain", "number", "SNR gain sqrt(N)"), _o("N", "number", "number of averages")],
     primary="SNR", conditions=["independent, zero-mean noise of equal variance in every acquisition", "an identical signal"],
     method="SNR_N = SNR_1 sqrt(N); N needed = ceil((SNR_target/SNR_1)^2)", latex=r"SNR_N = SNR_1\sqrt{N}",
     run=snr_averaging, reference=_ref_snr_averaging,
     wrong=lambda SNR1, N=None, SNR_target=None: (lambda n: _out(SNR=SNR1 * n, gain=float(n), N=float(n)))(N if N is not None else max(1, math.ceil(SNR_target / SNR1))),
     rand=lambda rng: (lambda s: {"SNR1": s, "N": rng.randint(2, 4096)} if rng.random() < 0.6 else {"SNR1": s, "SNR_target": _sig(s * rng.uniform(1.5, 60))})(_sig(_lu(rng, 0.5, 200))),
     phrases=("SNR averaging", "Signal averaging SNR", "SNR after averaging"),
     examples=[_ex({"SNR1": 10, "N": 16}), _ex({"SNR1": 5, "SNR_target": 20})],
     level="beginner",
     extra_q=[{"q": "SNR averaging: SNR1 = 10, N = 16, SNR_target = 50", "outcome": "INVALID_INPUT"}])


def ernst_angle(TR, T1):
    _pos("TR and T1", TR, T1)
    E1 = math.exp(-TR / T1)
    a = math.acos(E1)
    S = math.sin(a) * (1 - E1) / (1 - E1 * math.cos(a))           # = sqrt((1 - E1)/(1 + E1))
    return _out(alpha=a, alpha_deg=math.degrees(a), S_rel=S)


def _ref_ernst_angle(TR, T1):
    """The flip angle that maximises the spoiled gradient-echo steady-state signal: the root of dS/dalpha computed by a
    complex-step derivative (numpy), found by scipy brentq on (0, pi/2)."""
    import numpy as np
    E1 = math.exp(-TR / T1)
    S = lambda a: np.sin(a) * (1 - E1) / (1 - E1 * np.cos(a))                 # noqa: E731
    dS = lambda a: float(np.imag(S(complex(a, 1e-30))) / 1e-30)              # noqa: E731
    a = _brentq(dS, 1e-12, math.pi / 2)
    return {"alpha": a, "alpha_deg": math.degrees(a), "S_rel": float(S(a))}


_add("bmed_mri_ernst_angle",
     name="MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal)", topic=_IM,
     purpose="The Ernst angle alpha_E = arccos(exp(-TR/T1)) that maximises the steady-state signal of a spoiled gradient-echo sequence, and that largest signal as a fraction of M0 (T2* decay excluded).",
     inputs=[_phys("TR", "time", "repetition time"), _phys("T1", "time", "longitudinal relaxation time")],
     outputs=[_o("alpha", "angle", "Ernst angle"), _o("alpha_deg", "number", "Ernst angle in degrees"),
              _o("S_rel", "number", "signal at the Ernst angle, fraction of M0 (no T2* decay)")],
     primary="alpha_deg", conditions=["ideal spoiling, steady state", "TR, T1 > 0"],
     method="cos(alpha_E) = exp(-TR/T1); S = sin(alpha)(1 - E1)/(1 - E1 cos alpha)", latex=r"\alpha_E = \arccos\left(e^{-TR/T_1}\right)",
     run=ernst_angle, reference=_ref_ernst_angle,
     wrong=lambda TR, T1: (lambda a: _out(alpha=a, alpha_deg=math.degrees(a), S_rel=math.sin(a) * (1 - math.exp(-TR / T1)) / (1 - math.exp(-TR / T1) * math.cos(a))))(
         math.acos(math.exp(-T1 / TR)) if TR < T1 else math.acos(math.exp(-TR / T1) ** 2)),
     rand=lambda rng: (lambda T1: {"T1": T1, "TR": _sig(T1 * rng.uniform(0.002, 1.5))})(_sig(rng.uniform(0.2, 3.0))),
     phrases=("Ernst angle", "MRI Ernst angle"),
     examples=[_ex({"TR": "10 ms", "T1": "1000 ms"}), _ex({"TR": "500 ms", "T1": "900 ms"})],
     level="intermediate",
     extra_q=[{"q": "Ernst angle: TR = -10 ms, T1 = 1000 ms", "outcome": "INVALID_INPUT"}])


def _seir_taylor_peak(b, c, s0, e0, i0, K=26):
    """Time (in units of 1/gamma) and state at the peak of i for s' = -b s i, e' = b s i - c e, i' = c e - i: Taylor
    series of order K stepped to the first +/- sign change of g = c e - i (i' = gamma g), the root found on the series."""
    s, e, i, tau = s0, e0, i0, 0.0
    risen = (c * e - i) > 0
    for _ in range(200000):
        S, E, I = [s], [e], [i]
        for k in range(K):
            p = math.fsum(S[j] * I[k - j] for j in range(k + 1))
            S.append(-b * p / (k + 1))
            E.append((b * p - c * E[k]) / (k + 1))
            I.append((c * E[k] - I[k]) / (k + 1))
        h = 2.0
        for X in (S, E, I):
            top = max(abs(X[K]), abs(X[K - 1]) ** (K / (K - 1)) if X[K - 1] else 0.0)
            if top > 0 and X[0] > 0:
                h = min(h, (1e-17 * X[0] / top) ** (1.0 / K))
        G = [c * E[k] - I[k] for k in range(K + 1)]
        ev = lambda P, x: sum(P[k] * x ** k for k in range(len(P)))      # noqa: E731
        g_end = ev(G, h)
        if not risen and g_end > 0:
            risen = True
        elif risen and g_end <= 0:
            lo, hi = 0.0, h                                                # g(0) > 0 >= g(h): bisection on the series
            for _ in range(200):
                m = 0.5 * (lo + hi)
                if m in (lo, hi):
                    break
                if ev(G, m) > 0:
                    lo = m
                else:
                    hi = m
            th = 0.5 * (lo + hi)
            return tau + th, ev(S, th), ev(E, th), ev(I, th)
        s, e, i = ev(S, h), ev(E, h), ev(I, h)
        tau += h
        if not risen and b * s <= 1.0:
            break
        if tau > 1e5:
            break
    raise ValueError("no epidemic peak: the infectious fraction never rises")


def epi_seir_peak(beta, sigma, gamma, s0, e0, i0):
    _pos("beta, sigma and gamma", beta, sigma, gamma)
    _frac("s0", s0, lo_open=True)
    for nm, v in (("e0", e0), ("i0", i0)):
        _frac(nm, v)
    _need(e0 + i0 > 0, "an epidemic needs e0 + i0 > 0")
    _need(s0 + e0 + i0 <= 1 + 1e-12, "s0 + e0 + i0 cannot exceed 1")
    R0 = beta / gamma
    _need(R0 * s0 > 1.0 + 1e-6, "no epidemic peak: R0 s0 must exceed 1")
    tau, s, e, i = _seir_taylor_peak(R0, sigma / gamma, s0, e0, i0)
    _need(i > i0 * (1 + 1e-9), "the infectious fraction never rises above its initial value")
    return _out(t_peak=tau / gamma, i_max=i, s_at_peak=s, e_at_peak=e)


def _ref_epi_seir_peak(beta, sigma, gamma, s0, e0, i0):
    fun = lambda t, y: [-beta * y[0] * y[2], beta * y[0] * y[2] - sigma * y[1], sigma * y[1] - gamma * y[2]]   # noqa: E731
    g = lambda y: sigma * y[1] - gamma * y[2]                                                                  # noqa: E731
    gdot = lambda t, y: sigma * (beta * y[0] * y[2] - sigma * y[1]) - gamma * (sigma * y[1] - gamma * y[2])     # noqa: E731
    from scipy.integrate import solve_ivp
    y0, atol = [s0, e0, i0], [1e-22, 1e-22, 1e-22]
    ev = lambda t, y: sigma * y[1] - gamma * y[2]                                                              # noqa: E731
    ev.terminal, ev.direction = True, -1                 # the first fall of i: its peak
    sol = solve_ivp(fun, (0.0, 400.0 / min(gamma, sigma)), y0, method="DOP853", rtol=1e-13, atol=atol, events=ev)
    t = float(sol.t_events[0][0])
    for _ in range(3):                                   # Newton steps on fresh integrations (dense output ~1e-9 only)
        y = _ivp(fun, t, y0, atol=atol)
        dt = g(y) / gdot(t, y)
        t -= dt
        if abs(dt) <= 1e-15 * abs(t):
            break
    y = _ivp(fun, t, y0, atol=atol)
    return {"t_peak": t, "i_max": y[2], "s_at_peak": y[0], "e_at_peak": y[1]}


def _rand_epi_seir_peak(rng):
    gamma = _sig(_lu(rng, 2e-7, 5e-6), 3)
    sigma = _sig(gamma * _lu(rng, 0.3, 5.0), 3)
    i0 = _sig(_lu(rng, 1e-4, 0.01), 3)
    e0 = _sig(i0 * gamma / sigma * rng.uniform(1.5, 5.0), 3)
    s0 = round(rng.uniform(0.6, 1.0 - i0 - e0), 3)
    R0 = _lu(rng, 1.3 / s0, 6.0)
    return {"beta": _sig(R0 * gamma, 3), "sigma": sigma, "gamma": gamma, "s0": s0, "e0": e0, "i0": i0}


_add("bmed_epi_seir_peak",
     name="SEIR epidemic model: peak prevalence and the time of the peak", topic=_EPI,
     purpose="The largest infectious fraction of an SEIR epidemic and the time at which it occurs, from the transmission, incubation and recovery rates and the initial fractions.",
     inputs=[_phys("beta", "first_order_rate_constant", "transmission rate"), _phys("sigma", "first_order_rate_constant", "rate of leaving the latent stage (1/latent period)"),
             _phys("gamma", "first_order_rate_constant", "recovery rate"), _numin("s0", "initial susceptible fraction", pct=True),
             _numin("e0", "initial exposed (latent) fraction", pct=True), _numin("i0", "initial infectious fraction", pct=True)],
     outputs=[_o("t_peak", "time", "time of the peak of the infectious fraction"), _o("i_max", "number", "peak infectious fraction"),
              _o("s_at_peak", "number", "susceptible fraction at the peak"), _o("e_at_peak", "number", "exposed fraction at the peak")],
     primary="t_peak", conditions=["closed population, homogeneous mixing, frequency-dependent transmission", "R0 s0 > 1, e0 + i0 > 0",
                                   "the peak is the first maximum of i after t = 0 and exceeds i0"],
     method="Taylor-series integration (order 26, steps set by the series' tail) of the SEIR system to the first sign change of sigma e - gamma i, the root located on the series",
     latex=r"\dot s = -\beta s i,\ \dot e = \beta s i - \sigma e,\ \dot i = \sigma e - \gamma i;\quad t_{peak}:\ \sigma e = \gamma i",
     run=epi_seir_peak, reference=_ref_epi_seir_peak, rand=_rand_epi_seir_peak,
     wrong=lambda beta, sigma, gamma, s0, e0, i0: (lambda o: o | _out(t_peak=o["t_peak"] - 1.0 / sigma))(epi_seir_peak(beta, sigma, gamma, s0, e0, i0)),
     phrases=("SEIR peak", "SEIR epidemic peak"),
     examples=[_ex({"beta": "0.5 1/day", "sigma": "0.2 1/day", "gamma": "0.1 1/day", "s0": 0.99, "e0": 0.008, "i0": 0.002}),
               _ex({"beta": "0.3 1/day", "sigma": "0.5 1/day", "gamma": "0.15 1/day", "s0": 0.95, "e0": 0.01, "i0": 0.005})],
     level="expert",
     extra_q=[{"q": "SEIR peak: beta = 0.1 1/day, sigma = 0.2 1/day, gamma = 0.2 1/day, s0 = 0.99, e0 = 0.008, i0 = 0.002", "outcome": "INVALID_INPUT"}])


def bmi(m, h):
    _pos("the mass and the height", m, h)
    _need(0.3 <= h <= 2.8 and 1 <= m <= 700, "the height must lie between 0.3 and 2.8 m and the mass between 1 and 700 kg")
    v = m / (h * h)
    return _out(BMI=v, BMI_prime=v / 25.0)


def _ref_bmi(m, h):
    """Mass over squared height in exact rational arithmetic (sympy)."""
    import sympy as sp
    v = sp.Rational(repr(m)) / sp.Rational(repr(h)) ** 2
    return {"BMI": float(v), "BMI_prime": float(v / 25)}


_add("bmed_bmi",
     name="Body mass index (a calculation, not medical advice)", topic=_BM,
     purpose="The body mass index BMI = m/h^2 (kg/m^2) and BMI prime (BMI/25) as a calculation from mass and height; no category or health advice is given.",
     inputs=[_phys("m", "mass", "body mass"), _phys("h", "length", "height")],
     outputs=[_o("BMI", "number", "body mass index in kg/m^2"), _o("BMI_prime", "number", "BMI/25")],
     primary="BMI", conditions=["a calculation only: not medical advice", "1 <= m <= 700 kg, 0.3 <= h <= 2.8 m"],
     method="BMI = m/h^2", latex=r"\mathrm{BMI} = \frac{m}{h^2}",
     run=bmi, reference=_ref_bmi,
     wrong=lambda m, h: _out(BMI=m / h, BMI_prime=m / h / 25.0),
     rand=lambda rng: {"m": _sig(rng.uniform(30, 150)), "h": _sig(rng.uniform(1.4, 2.05))},
     phrases=("Body mass index", "BMI"),
     examples=[_ex({"m": "70 kg", "h": "1.75 m"}), _ex({"m": "90 kg", "h": "180 cm"})],
     level="beginner",
     extra_q=[{"q": "Body mass index: m = 70 kg, h = 5 m", "outcome": "INVALID_INPUT"}])

_BMR_EQ = ("mifflin_st_jeor", "harris_benedict_revised")
KCAL_DAY_W = 4184.0 / 86400.0


def bmr(m, h, age, sex, equation):
    _pos("the mass and the height", m, h)
    _need(0.5 <= h <= 2.8 and 10 <= m <= 400, "the height must lie between 0.5 and 2.8 m and the mass between 10 and 400 kg")
    _need(_isnum(age) and 18 <= age <= 120, "the equations are stated for adults: age 18 to 120 years")
    _need(sex in ("male", "female"), "sex must be male or female (the equations' two forms)")
    _need(equation in _BMR_EQ, "equation must be mifflin_st_jeor or harris_benedict_revised")
    hc = h * 100.0
    if equation == "mifflin_st_jeor":
        k = 10.0 * m + 6.25 * hc - 5.0 * age + (5.0 if sex == "male" else -161.0)
    elif sex == "male":
        k = 88.362 + 13.397 * m + 4.799 * hc - 5.677 * age
    else:
        k = 447.593 + 9.247 * m + 3.098 * hc - 4.330 * age
    return _out(BMR_kcal_day=k, BMR_W=k * KCAL_DAY_W, BMR_MJ_day=k * 4.184e-3)


def _ref_bmr(m, h, age, sex, equation):
    """The stated regression as a dot product of its coefficients and (1, m, h in cm, age) in exact arithmetic (sympy);
    the thermochemical calorie 4.184 J."""
    import sympy as sp
    coef = {("mifflin_st_jeor", "male"): ("5", "10", "6.25", "-5"), ("mifflin_st_jeor", "female"): ("-161", "10", "6.25", "-5"),
            ("harris_benedict_revised", "male"): ("88.362", "13.397", "4.799", "-5.677"),
            ("harris_benedict_revised", "female"): ("447.593", "9.247", "3.098", "-4.330")}[(equation, sex)]
    x = (1, sp.Rational(repr(m)), sp.Rational(repr(h)) * 100, sp.Rational(repr(float(age))))
    k = sum(sp.Rational(c) * v for c, v in zip(coef, x))
    return {"BMR_kcal_day": float(k), "BMR_W": float(k * 4184 / 86400), "BMR_MJ_day": float(k * sp.Rational("0.004184"))}


_add("bmed_bmr",
     name="Basal metabolic rate by a stated regression (a calculation, not medical advice)", topic=_BM,
     purpose="The basal metabolic rate estimated by the Mifflin-St Jeor (1990) or the revised Harris-Benedict (Roza and Shizgal 1984) regression from mass, height, age and sex, in kcal/day, W and MJ/day; a model calculation, not dietary or medical advice.",
     inputs=[_phys("m", "mass", "body mass"), _phys("h", "length", "height"), _numin("age", "age in years (adults)"),
             _choice("sex", ["male", "female"], "sex (the equations have a male and a female form)"),
             _choice("equation", list(_BMR_EQ), "regression: mifflin_st_jeor or harris_benedict_revised")],
     outputs=[_o("BMR_kcal_day", "number", "basal metabolic rate in kcal/day"), _o("BMR_W", "power", "basal metabolic rate (W)"),
              _o("BMR_MJ_day", "number", "basal metabolic rate in MJ/day")],
     primary="BMR_kcal_day", conditions=["a population regression, not a measurement and not advice", "adults 18 to 120 years", "1 kcal = 4.184 kJ"],
     method="Mifflin-St Jeor: 10 m + 6.25 h_cm - 5 age + 5 (male) or - 161 (female); revised Harris-Benedict: 88.362 + 13.397 m + 4.799 h_cm - 5.677 age (male), 447.593 + 9.247 m + 3.098 h_cm - 4.330 age (female)",
     latex=r"\mathrm{BMR} = 10\,m + 6.25\,h - 5\,a + s",
     run=bmr, reference=_ref_bmr,
     wrong=lambda m, h, age, sex, equation: (lambda o: o | _out(BMR_kcal_day=o["BMR_kcal_day"] + 6.25 * h * 100 * 0.99 - 6.25 * h * 100, BMR_W=(o["BMR_kcal_day"] - 0.0625 * h * 100) * KCAL_DAY_W,
                                                             BMR_MJ_day=(o["BMR_kcal_day"] - 0.0625 * h * 100) * 4.184e-3) if equation == "mifflin_st_jeor"
                                             else o | _out(BMR_kcal_day=o["BMR_kcal_day"] - 4.0 * h * 100 * (1 if sex == "male" else 0.7)))(bmr(m, h, age, sex, equation)),
     rand=lambda rng: {"m": _sig(rng.uniform(40, 150)), "h": _sig(rng.uniform(1.45, 2.0)), "age": float(rng.randint(18, 90)),
                       "sex": rng.choice(["male", "female"]), "equation": rng.choice(list(_BMR_EQ))},
     phrases=("Basal metabolic rate", "BMR"),
     examples=[_ex({"m": "70 kg", "h": "175 cm", "age": 30, "sex": "male", "equation": "mifflin_st_jeor"}),
               _ex({"m": "60 kg", "h": "165 cm", "age": 45, "sex": "female", "equation": "harris_benedict_revised"})],
     level="beginner",
     extra_q=[{"q": "Basal metabolic rate: m = 70 kg, h = 175 cm, age = 10, sex = male, equation = mifflin_st_jeor", "outcome": "INVALID_INPUT"}])


def hounsfield(mu, mu_water, mu_air=0.0):
    _pos("the attenuation coefficients", mu, mu_water)
    _need(mu_air >= 0 and mu_air < mu_water, "the air coefficient must lie in [0, mu_water)")
    HU = 1000.0 * (mu - mu_water) / (mu_water - mu_air)
    return _out(HU=HU, mu_rel=mu / mu_water)


def _ref_hounsfield(mu, mu_water, mu_air=0.0):
    """The CT scale as the straight line through the two calibration points (mu_air, -1000) and (mu_water, 0),
    evaluated by numpy's polynomial fit through those points (extrapolated beyond water)."""
    import numpy as np
    coef = np.polyfit([mu_air, mu_water], [-1000.0, 0.0], 1)
    return {"HU": float(np.polyval(coef, mu)), "mu_rel": mu / mu_water}


_add("bmed_hounsfield_units",
     name="CT number in Hounsfield units", topic=_IM,
     purpose="The CT number HU = 1000 (mu - mu_water)/(mu_water - mu_air) of a material from its linear attenuation coefficient and those of water and air (air 0 by default, then 1000 (mu - mu_water)/mu_water).",
     inputs=[_phys("mu", "inverse_length", "linear attenuation coefficient of the material"), _phys("mu_water", "inverse_length", "linear attenuation coefficient of water"),
             _phys("mu_air", "inverse_length", "linear attenuation coefficient of air (default 0)", required=False)],
     outputs=[_o("HU", "number", "CT number in Hounsfield units"), _o("mu_rel", "number", "mu/mu_water")],
     primary="HU", conditions=["coefficients at the same effective energy", "0 <= mu_air < mu_water"],
     method="HU = 1000 (mu - mu_water)/(mu_water - mu_air)", latex=r"HU = 1000\,\frac{\mu - \mu_{water}}{\mu_{water} - \mu_{air}}",
     run=hounsfield, reference=_ref_hounsfield,
     wrong=lambda mu, mu_water, mu_air=0.0: _out(HU=1000.0 * (mu - mu_water) / mu, mu_rel=mu / mu_water),
     rand=lambda rng: (lambda w: {"mu_water": w, "mu": _sig(w * rng.uniform(0.05, 3.0)), **({"mu_air": _sig(w * rng.uniform(0.0, 0.01))} if rng.random() < 0.4 else {})})(
         _sig(rng.uniform(15, 25))),
     phrases=("Hounsfield units", "CT number"),
     examples=[_ex({"mu": "0.38 1/cm", "mu_water": "0.19 1/cm"}), _ex({"mu": "0.2 1/cm", "mu_water": "0.19 1/cm", "mu_air": "0.0002 1/cm"})],
     level="beginner",
     extra_q=[{"q": "Hounsfield units: mu = -0.38 1/cm, mu_water = 0.19 1/cm", "outcome": "INVALID_INPUT"}])

GAMMA_P_MHZ_T = 42.577478461                       # proton gyromagnetic ratio / 2 pi, MHz/T (CODATA 2022)


def larmor(B, gamma_bar=None):
    _pos("the magnetic field", B)
    g = GAMMA_P_MHZ_T if gamma_bar is None else gamma_bar
    _need(_isnum(g) and 0 < abs(g) <= 100, "the reduced gyromagnetic ratio must be given in MHz/T (|gamma/2 pi| <= 100)")
    f = abs(g) * 1e6 * B
    return _out(f=f, f_MHz=f / 1e6, omega=2 * math.pi * f)


def _ref_larmor(B, gamma_bar=None):
    """The precession frequency from scipy's CODATA proton gyromagnetic ratio (rad/(s T)) for protons, or from the
    stated ratio: omega = gamma B, f = omega/(2 pi)."""
    from scipy import constants as sc
    if gamma_bar is None:
        w = sc.physical_constants["proton gyromag. ratio"][0] * B
    else:
        w = 2 * math.pi * abs(gamma_bar) * 1e6 * B
    return {"f": w / (2 * math.pi), "f_MHz": w / (2 * math.pi) / 1e6, "omega": w}


_add("bmed_larmor_frequency",
     name="Larmor (precession) frequency in a magnetic field", topic=_IM,
     purpose="The Larmor frequency f = (gamma/2 pi) B of nuclear spins in a field B: protons by default (42.577478461 MHz/T, CODATA 2022), or another nucleus with its stated gamma/2 pi in MHz/T.",
     inputs=[_phys("B", "magnetic_field", "magnetic flux density (T)"),
             _numin("gamma_bar", "reduced gyromagnetic ratio gamma/2 pi in MHz/T (default: the proton)", required=False)],
     outputs=[_o("f", "frequency", "Larmor frequency"), _o("f_MHz", "number", "Larmor frequency in MHz"), _o("omega", "angular_frequency", "angular Larmor frequency")],
     primary="f_MHz", conditions=["B > 0", "bare nucleus (no chemical shielding)"],
     method="f = (gamma/2 pi) B", latex=r"f = \frac{\gamma}{2\pi}\,B_0",
     run=larmor, reference=_ref_larmor,
     wrong=lambda B, gamma_bar=None: (lambda g: _out(f=g * 1e6 * B * 2 * math.pi, f_MHz=g * B * 2 * math.pi, omega=g * 1e6 * B * 4 * math.pi ** 2))(GAMMA_P_MHZ_T if gamma_bar is None else abs(gamma_bar)),
     rand=lambda rng: {"B": _sig(rng.uniform(0.05, 12.0)), **({"gamma_bar": rng.choice([10.7084, 40.078, 17.235, 11.262])} if rng.random() < 0.4 else {})},
     phrases=("Larmor frequency", "Precession frequency"),
     examples=[_ex({"B": "1.5 T"}), _ex({"B": "3 T", "gamma_bar": 17.235})],
     level="beginner",
     extra_q=[{"q": "Larmor frequency: B = -3 T", "outcome": "INVALID_INPUT"}])


def us_wavelength(f, c=1540.0):
    _pos("the frequency and the speed of sound", f, c)
    lam = c / f
    return _out(wavelength=lam, wavelength_mm=lam * 1e3, period=1.0 / f, k=2 * math.pi / lam)


def _ref_us_wavelength(f, c=1540.0):
    """The distance a wavefront travels in one period, by quadrature of the speed over one period (scipy)."""
    T = 1.0 / f
    lam = _quad(lambda t: c, 0.0, T)
    return {"wavelength": lam, "wavelength_mm": lam * 1e3, "period": T, "k": 2 * math.pi / lam}


_add("bmed_ultrasound_wavelength",
     name="Ultrasound wavelength in tissue", topic=_IM,
     purpose="The wavelength lambda = c/f of ultrasound of frequency f in a medium with speed of sound c (1540 m/s, the soft-tissue convention, by default), with the period and wavenumber.",
     inputs=[_phys("f", "frequency", "ultrasound frequency"), _phys("c", "speed", "speed of sound (default 1540 m/s)", required=False)],
     outputs=[_o("wavelength", "length", "wavelength"), _o("wavelength_mm", "number", "wavelength in mm"), _o("period", "time", "period 1/f"),
              _o("k", "inverse_length", "wavenumber 2 pi/lambda")],
     primary="wavelength_mm", conditions=["f, c > 0", "c = 1540 m/s unless stated"],
     method="lambda = c/f", latex=r"\lambda = \frac{c}{f}",
     run=us_wavelength, reference=_ref_us_wavelength,
     wrong=lambda f, c=1540.0: (lambda lam: _out(wavelength=lam, wavelength_mm=lam * 1e3, period=1.0 / f, k=2 * math.pi / lam))(c / (2 * math.pi * f)),
     rand=lambda rng: {"f": _sig(rng.uniform(1e6, 2e7)), **({"c": _sig(rng.uniform(1400, 4100))} if rng.random() < 0.5 else {})},
     phrases=("Ultrasound wavelength", "Wavelength of ultrasound"),
     examples=[_ex({"f": "5 MHz"}), _ex({"f": "3.5 MHz", "c": "1580 m/s"})],
     level="beginner",
     extra_q=[{"q": "Ultrasound wavelength: f = 0 MHz", "outcome": "INVALID_INPUT"}])



# @@EXV-BEGIN@@
# worked-example outputs, computed by run and checked against the reference computations on the box
_EXV = {'bmed_cable_steady_state': [{'R_in': 417952112.283, 'V_x': 0.0081341763827}, {'R_in': 112539539.52, 'V_x': -0.00246534345698}],
 'bmed_cohens_kappa': [{'kappa': 0.7, 'p_observed': 0.85}, {'kappa': 0.710542352224, 'p_observed': 0.81}],
 'bmed_cox_hazard_ratio': [{'HR': 1.0484603901, 'HR_lower': 0.279864240599, 'HR_upper': 3.92786583685, 'beta': 0.047322792962, 'se': 0.67387638333},
                           {'HR': 1.99374753838, 'HR_lower': 0.592577770425, 'HR_upper': 6.70802963794, 'beta': 0.690016052882, 'se': 0.73762746392}],
 'bmed_dx_roc_auc': [{'AUC': 0.883333333333, 'U': 26.5}, {'AUC': 0.78125, 'U': 12.5}],
 'bmed_dx_sensitivity_specificity_ci': [{'sens_lower': 0.786397685625,
                                         'sens_upper': 0.956524235068,
                                         'sensitivity': 0.9,
                                         'spec_lower': 0.825634338495,
                                         'spec_upper': 0.944770862939,
                                         'specificity': 0.9},
                                        {'sens_lower': 0.738336953673,
                                         'sens_upper': 0.966336776438,
                                         'sensitivity': 0.9,
                                         'spec_lower': 0.620167866056,
                                         'spec_upper': 0.769295045631,
                                         'specificity': 0.7}],
 'bmed_dx_threshold_sensitivity_specificity': [{'FN': 1, 'FP': 1, 'TN': 5, 'TP': 4, 'sensitivity': 0.8, 'specificity': 0.833333333333},
                                               {'FN': 1, 'FP': 3, 'TN': 4, 'TP': 5, 'sensitivity': 0.833333333333, 'specificity': 0.571428571429}],
 'bmed_dx_youden_cutoff': [{'J': 0.633333333333, 'sensitivity': 0.8, 'specificity': 0.833333333333, 'threshold': 0.55},
                           {'J': 0.5, 'sensitivity': 0.5, 'specificity': 1.0, 'threshold': 22.0}],
 'bmed_elbow_lever_muscle_force': [{'F_muscle': 501.366889619, 'R': 437.50000915, 'R_x': 87.0614467248, 'R_y': -428.75},
                                   {'F_muscle': 656.0, 'R': 536.0, 'R_x': 0.0, 'R_y': -536.0}],
 'bmed_epi_r_from_growth_rate': [{'R': 1.61770181142}, {'R': 0.706940248754}],
 'bmed_epi_reed_frost_final_size': [{'P_no_spread': 0.4096,
                                     'distribution': [0.4096, 0.2097152, 0.163577856, 0.1351614464, 0.0819454976],
                                     'mean_final': 1.2701372416},
                                    {'P_no_spread': 0.358485922409,
                                     'distribution': [0.358485922409,
                                                      0.244085487499,
                                                      0.159570440671,
                                                      0.10291044883,
                                                      0.0640931869702,
                                                      0.0374640511926,
                                                      0.0198552532867,
                                                      0.00911594235476,
                                                      0.00338709781558,
                                                      0.000902915019467,
                                                      0.000129253952282],
                                     'mean_final': 1.5351093926}],
 'bmed_epi_seir_final_size': [{'attack': 0.92192202553, 's_inf': 0.0580779744696}, {'attack': 0.106026129269, 's_inf': 0.493973870731}],
 'bmed_epi_seir_growth_rate': [{'r': 9.15842416062e-07}, {'r': -4.04667098167e-07}],
 'bmed_epi_sir_final_size': [{'attack': 0.884105805664, 's_inf': 0.105894194336}, {'attack': 0.0812153695704, 's_inf': 0.61878463043}],
 'bmed_epi_sir_peak': [{'i_max': 0.300796070555, 't_peak': 3313898.58286}, {'i_max': 0.185627913513, 't_peak': 1568670.89994}],
 'bmed_epi_sis_prevalence': [{'i': 0.160111247542, 'i_star': 0.75}, {'i': 0.162748262473, 'i_star': 0.0}],
 'bmed_exponential_hazard_ratio': [{'HR': 5.16055045872, 'lower': 1.58923294897, 'upper': 16.7573174557},
                                   {'HR': 0.428571428571, 'lower': 0.122019358677, 'upper': 1.50528138633}],
 'bmed_exponential_survival_fit': [{'hazard': 0.025069637883, 'mean': 39.8888888889, 'median': 27.648870869},
                                   {'hazard': 0.1171875, 'mean': 8.53333333333, 'median': 5.91485594078}],
 'bmed_fitzhugh_nagumo_fixed_point': [{'determinant': 0.108069096641,
                                       'stable': True,
                                       'trace': -0.502579635008,
                                       'v_star': -1.19940803524,
                                       'w_star': -0.624260044055},
                                      {'determinant': 0.0574579133353,
                                       'stable': False,
                                       'trace': 0.288220104136,
                                       'v_star': -0.804847747008,
                                       'w_star': -0.13105968376}],
 'bmed_gaussian_psf_mtf': [{'MTF': 0.410685800378, 'f50': 882.542400611}, {'MTF': 0.565778187198, 'f50': 110.317800076}],
 'bmed_ghk_current_density': [{'I': 0.085282548488}, {'I': -0.0101621748993}],
 'bmed_ghk_voltage': [{'V_m': -0.0750662761527}, {'V_m': -0.0596663029087}],
 'bmed_hh_gating_steady_state': [{'h_inf': 0.596120753508, 'm_inf': 0.0529324852572, 'n_inf': 0.317676914061},
                                 {'h_inf': 0.00894348028244, 'm_inf': 0.875693546092, 'n_inf': 0.83517846271}],
 'bmed_hh_potassium_conductance_clamp': [{'g_K': 104.172166871, 'n': 0.733436128726}, {'g_K': 77.5423407387, 'n': 0.681254056831}],
 'bmed_hh_resting_potential': [{'V_rest': -0.0649963793312}, {'V_rest': -0.0687944556326}],
 'bmed_incidence_rate_ratio_ci': [{'IRR': 2.4, 'lower': 1.29133130431, 'upper': 4.46051294567},
                                  {'IRR': 0.491901619676, 'lower': 0.286257320869, 'upper': 0.845278655949}],
 'bmed_indicator_dilution_cardiac_output': [{'AUC': 29.6, 'CO': 0.000168918918919}, {'AUC': 27.1, 'CO': 0.000369003690037}],
 'bmed_kelvin_voigt_creep': [{'strain': 0.0451188363906}, {'strain': 0.00172932943353}],
 'bmed_km_confidence_interval': [{'S_t': 0.752941176471, 'lower': 0.5031995108, 'upper': 0.889361835169},
                                 {'S_t': 0.6, 'lower': 0.258166790598, 'upper': 0.824730437997}],
 'bmed_km_survival_median': [{'S_t': 0.752941176471, 'median': 23.0}, {'S_t': 0.6, 'median': 9.0}],
 'bmed_lif_firing_rate': [{'ISI': 0.0239722457734, 'rate': 41.7149068741}, {'ISI': 0.0112961941718, 'rate': 88.5253904802}],
 'bmed_lif_subthreshold_voltage': [{'V': -0.0640979598957}, {'V': -0.0737687250206}],
 'bmed_logrank_test': [{'chi_square': 13.8499816234, 'p_value': 0.000197998673203}, {'chi_square': 1.38950421152, 'p_value': 0.238488844263}],
 'bmed_mantel_haenszel_odds_ratio': [{'OR_MH': 2.60775862069, 'lower': 1.08405343824, 'upper': 6.27312712078},
                                     {'OR_MH': 2.78403755869, 'lower': 1.86900853213, 'upper': 4.14704641253}],
 'bmed_mri_adc_fit': [{'ADC': 0.0010498221245, 'S0': 1200.0}, {'ADC': 0.000923418473421, 'S0': 983.024096897}],
 'bmed_mri_inversion_recovery': [{'Mz_ratio': -0.0972878095602, 'TI_null': 0.173202943547}, {'Mz_ratio': 0.0348763675239, 'TI_null': 2.37176248657}],
 'bmed_mri_spin_echo_signal': [{'S': 0.366873830695}, {'S': 0.374738822433}],
 'bmed_mri_spoiled_gradient_echo': [{'S': 0.0655970249817}, {'S': 0.0850010521594}],
 'bmed_mri_t2_fit': [{'S0': 999.810389132, 'T2': 99.4459180042}, {'S0': 1059.54436656, 'T2': 71.1788039472}],
 'bmed_nelson_aalen_hazard': [{'H_t': 0.268347338936}, {'H_t': 0.80119047619}],
 'bmed_nnt_ci': [{'ARR': 0.15,
                  'ARR_lower': 0.0361363819307,
                  'ARR_upper': 0.263863618069,
                  'NNT': 6.66666666667,
                  'NNT_lower': 3.78983661073,
                  'NNT_upper': 27.6729419652},
                 {'ARR': 0.0951219512195,
                  'ARR_lower': 0.0545621401257,
                  'ARR_upper': 0.135681762313,
                  'NNT': 10.5128205128,
                  'NNT_lower': 7.37018728936,
                  'NNT_upper': 18.3277268395}],
 'bmed_odds_ratio_ci': [{'OR': 2.25, 'lower': 0.994294892282, 'upper': 5.09154782882},
                        {'OR': 3.27272727273, 'lower': 1.56470993442, 'upper': 6.84519447728}],
 'bmed_pk_auc_iv_bolus': [{'AUC_0_t': 227.563401178, 'AUC_inf': 360.0}, {'AUC_0_t': 49.8424444201, 'AUC_inf': 50.0}],
 'bmed_pk_auc_trapezoid': [{'AUC': 51.85}, {'AUC': 51.2461627577}],
 'bmed_pk_bioavailability_from_profiles': [{'AUC_iv': 46.55, 'AUC_oral': 58.9, 'F': 0.632653061224},
                                           {'AUC_iv': 21.6, 'AUC_oral': 12.6, 'F': 0.583333333333}],
 'bmed_pk_effect_site_concentration': [{'Ce': 0.000289899138384, 'Ce_peak': 0.000357745635584, 't_peak': 4017.43055623},
                                       {'Ce': 0.001337461907, 'Ce_peak': 0.00133748060995, 't_peak': 14484.9412119}],
 'bmed_pk_hepatic_clearance_parallel_tube': [{'CL_H': 1.06561644816e-05, 'E': 0.426246579263}, {'CL_H': 4.53173117305e-06, 'E': 0.181269246922}],
 'bmed_pk_hepatic_clearance_well_stirred': [{'CL_H': 8.92857142857e-06, 'E': 0.357142857143}, {'CL_H': 4.16666666667e-06, 'E': 0.166666666667}],
 'bmed_pk_infusion_concentration': [{'C': 0.00527633447259, 'C_ss': 0.01}, {'C': 0.0199850682838, 'C_ss': 0.02}],
 'bmed_pk_intermittent_infusion_steady_state': [{'C_max_ss': 0.0378541653242, 'C_min_ss': 0.0093347222415},
                                                {'C_max_ss': 0.0123659754044, 'C_min_ss': 0.000697640593517}],
 'bmed_pk_iv_bolus_concentration': [{'C': 0.00686014545118, 'fraction_remaining': 0.548811636094},
                                    {'C': 0.0137457855758, 'fraction_remaining': 0.687289278791}],
 'bmed_pk_loading_maintenance': [{'LD': 0.00075, 'R0': 1.66666666667e-08}, {'LD': 0.0007, 'R0': 1.66666666667e-08}],
 'bmed_pk_michaelis_menten_concentration': [{'C': 0.00530689866097}, {'C': 0.00378470353884}],
 'bmed_pk_michaelis_menten_steady_state': [{'C_ss': 0.006}, {'C_ss': 0.015}],
 'bmed_pk_michaelis_menten_time': [{'t': 110355.16656}, {'t': 277293.063348}],
 'bmed_pk_nca_iv_bolus': [{'AUC_inf': 19.4145444084,
                           'AUC_last': 18.4825,
                           'CL': 5.15077757667,
                           'Vz': 42.8638699926,
                           'lambda_z': 0.120165948095,
                           't_half': 5.76824958776},
                          {'AUC_inf': 58.4223725019,
                           'AUC_last': 53.9,
                           'CL': 4.27918260238,
                           'Vz': 20.3705870861,
                           'lambda_z': 0.210066729265,
                           't_half': 3.29965236754}],
 'bmed_pk_nth_dose_iv': [{'C': 0.0168988635309, 'C_max_n': 0.0206403185264}, {'C': 0.0166666666667, 'C_max_n': 0.0166666666667}],
 'bmed_pk_oral_concentration': [{'AUC_0_t': 37.7397258999, 'C': 0.00398566157828}, {'AUC_0_t': 1.61184853604, 'C': 4.58093682805e-05}],
 'bmed_pk_oral_peak': [{'C_max': 0.00424569796896, 't_max': 7129.51385719}, {'C_max': 0.00648981224272, 't_max': 10346.0663233}],
 'bmed_pk_short_infusion': [{'C': 0.00331608906589, 'C_end': 0.00604230823073}, {'C': 0.00282698908207, 'C_end': 0.00369640527584}],
 'bmed_pk_steady_state_iv': [{'C_avg_ss': 0.015625, 'C_max_ss': 0.0226995777615, 'C_min_ss': 0.0101995777615, 'R_ac': 1.81596622092},
                             {'C_avg_ss': 0.0144175317186, 'C_max_ss': 0.0199909497585, 'C_min_ss': 0.00999094975854, 'R_ac': 1.99909497585}],
 'bmed_pk_steady_state_oral': [{'C_max_ss': 0.00639474435162, 'C_min_ss': 0.00281433487262, 't_max_ss': 5901.00635948},
                               {'C_max_ss': 0.000674329007686, 'C_min_ss': 7.5014114358e-05, 't_max_ss': 5495.93492785}],
 'bmed_pk_terminal_half_life_fit': [{'C0': 9.73504961651, 'k': 0.174804824351, 't_half': 3.96526344814},
                                    {'C0': 110.831964432, 'k': 0.32542556159, 't_half': 2.12997152766}],
 'bmed_pk_time_to_steady_state_fraction': [{'t': 66314.4506782}, {'t': 124766.492501}],
 'bmed_pk_two_compartment_bolus': [{'A': 0.00729415733871,
                                    'B': 0.00270584266129,
                                    'C': 0.00226643451993,
                                    'alpha': 0.000259969415098,
                                    'beta': 1.78083626794e-05},
                                   {'A': 0.0193088097425,
                                    'B': 0.00569119025748,
                                    'C': 0.0139756237989,
                                    'alpha': 0.000465593217248,
                                    'beta': 6.62900497419e-06}],
 'bmed_pk_two_compartment_micro_constants': [{'CL': 1.60256410256e-06,
                                              'V1': 0.0125,
                                              'Vss': 0.0304733727811,
                                              'k10': 0.000128205128205,
                                              'k12': 0.000194711538462,
                                              'k21': 0.000135416666667},
                                             {'CL': 4.38596491228e-06,
                                              'V1': 0.02,
                                              'Vss': 0.0634349030471,
                                              'k10': 0.000219298245614,
                                              'k12': 0.000458479532164,
                                              'k21': 0.000211111111111}],
 'bmed_poiseuille_vessel_flow': [{'Q': 9.57359160827e-06, 'R_hyd': 55704230.0822, 'tau_wall': 5.3328954966, 'v_mean': 0.7618422138},
                                 {'Q': 9.81747704247e-11, 'R_hyd': 509295817894.0, 'tau_wall': 0.5, 'v_mean': 0.003125}],
 'bmed_postsynaptic_potential': [{'V_peak': 0.000787450656184, 'V_t': 0.00078532562746, 't_peak': 0.00924196240747},
                                 {'V_peak': -0.000802488365972, 'V_t': -0.0007765320908, 't_peak': 0.00402359478109}],
 'bmed_relative_risk_ci': [{'RR': 2.0, 'lower': 0.986563206732, 'upper': 4.0544791988}, {'RR': 2.25, 'lower': 1.62671184861, 'upper': 3.11210618175}],
 'bmed_respiratory_mechanics_fit': [{'C': 0.0216558330482, 'P0': -3.1691943128, 'R': 16.7140600316},
                                    {'C': 0.0182635997346, 'P0': 5.22500197446, 'R': 3.60921416349}],
 'bmed_restricted_mean_survival_time': [{'RMST': 16.1165266106}, {'RMST': 7.175}],
 'bmed_sls_creep': [{'strain': 0.104462491923, 'strain_inf': 0.2}, {'strain': 0.00130753461445, 'strain_inf': 0.005}],
 'bmed_sls_stress_relaxation': [{'stress': 5532.65329856, 'stress_inf': 2500.0}, {'stress': 3082682.26589, 'stress_inf': 2000000.0}],
 'bmed_smr_exact_ci': [{'SMR': 1.35869565217, 'lower': 0.879276187382, 'upper': 2.00570280965},
                       {'SMR': 0.4, 'lower': 0.109025526289, 'upper': 1.03382087039}],
 'bmed_ultrasound_interface': [{'R': 0.428101690303, 'T': 0.571898309697, 'r_p': 0.654294803818, 't_p': 1.65429480382},
                               {'R': 0.998757294498, 'T': 0.00124270550214, 'r_p': -0.999378454089, 't_p': 0.000621545910728}],
 'bmed_ultrasound_matching_layer': [{'T': 0.999983361178, 'kd': 1.57079632679}, {'T': 0.464142184043, 'kd': 1.25663706144}],
 'bmed_windkessel_diastolic_decay': [{'P': 11652.5234102}, {'P': 7272.67708928}],
 'bmed_windkessel_input_impedance': [{'Z_mod': 11742475.2849, 'phase_deg': -84.9470813708}, {'Z_mod': 9965557.37501, 'phase_deg': -45.0521284839}],
 'bmed_xray_attenuation_layers': [{'I': 145.875756856, 'T': 0.145875756856}, {'I': 7.41884218635, 'T': 0.148376843727}]}
# @@EXV-END@@

# the worked examples of the definitional additions (computed by their references on the build machine)
_EXV.update({
    'bmed_snr_averaging': [{"SNR": 40.0, "gain": 4.0, "N": 16.0}, {"SNR": 20.0, "gain": 4.0, "N": 16.0}],
    'bmed_mri_ernst_angle': [{"alpha": 0.141185772249, "alpha_deg": 8.08934887715, "S_rel": 0.0707103834932}, {"alpha": 0.959715014208, "alpha_deg": 54.9876198495, "S_rel": 0.520429744074}],
    'bmed_epi_seir_peak': [{"t_peak": 3177221.4983, "i_max": 0.306250281976, "s_at_peak": 0.122175820982, "e_at_peak": 0.153125140988}, {"t_peak": 3525519.77392, "i_max": 0.110336506958, "s_at_peak": 0.47521087306, "e_at_peak": 0.0331009520874}],
    'bmed_bmi': [{"BMI": 22.8571428571, "BMI_prime": 0.914285714286}, {"BMI": 27.7777777778, "BMI_prime": 1.11111111111}],
    'bmed_bmr': [{"BMR_kcal_day": 1648.75, "BMR_W": 79.8422453704, "BMR_MJ_day": 6.89837}, {"BMR_kcal_day": 1318.733, "BMR_W": 63.8608665741, "BMR_MJ_day": 5.517578872}],
    'bmed_hounsfield_units': [{"HU": 1000.0, "mu_rel": 2.0}, {"HU": 52.6870389884, "mu_rel": 1.05263157895}],
    'bmed_larmor_frequency': [{"f": 63866217.6908, "f_MHz": 63.8662176908, "omega": 401283280.62}, {"f": 51705000.0, "f_MHz": 51.705, "omega": 324872096.308}],
    'bmed_ultrasound_wavelength': [{"wavelength": 0.000308, "wavelength_mm": 0.308, "period": 2e-07, "k": 20399.952296}, {"wavelength": 0.000451428571429, "wavelength_mm": 0.451428571429, "period": 2.85714285714e-07, "k": 13918.4484653}],
})

for _k, _vals in _EXV.items():
    if _k in PROCEDURES:
        for _e, _v in zip(PROCEDURES[_k]["examples"], _vals):
            _e["outputs"].update(_v)
