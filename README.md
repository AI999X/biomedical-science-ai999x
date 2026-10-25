# biomedical-science-ai999x — offline, verified biomedical science solver with step-by-step answers

[![tests](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/biomedical-science-ai999x)](https://pypi.org/project/biomedical-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/biomedical-science-ai999x)

**[Install](https://pypi.org/project/biomedical-science-ai999x/)** · **[Capability guide](https://huggingface.co/AI999X/biomedical-science-ai999x/tree/main/docs)** · **[Evaluation](https://huggingface.co/AI999X/biomedical-science-ai999x/tree/main/evaluation)** · **[AI999X on Hugging Face](https://huggingface.co/AI999X)** · **[Website](https://ai999x.com)** · **[Support: Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi)**

License: Apache-2.0 · Author: Ibnul Jaif Farabi (creator and engineer) · Organization: AI999X · Version 1.0.1

Field: **Life Sciences** · Tasks: Question Answering, Token Classification, Text Classification, Sentence Similarity, Feature Extraction

biomedical-science-ai999x is an offline, deterministic Python solver that answers biomedical science questions with verified results from 86 certified capabilities, gives 561 documented answers identically on Windows and Linux, and refuses rather than guesses when an answer cannot be verified.

**biomedical-science-ai999x** is a compact, deterministic biomedical science engine for Python. It reads a question in plain English (or a typed call), picks a certified biomedical science capability, computes the answer exactly, checks it, and returns the result with its steps, units, LaTeX and assumptions — or an honest refusal that says what is missing. It runs fully offline on any CPU, needs no GPU, no API key and no internet, and is built to give **zero wrong answers**: when it is not sure, it refuses instead of guessing.

[![Buy me a coffee](https://img.shields.io/badge/Buy%20me%20a%20coffee-support%20development-FFDD00?logo=buymeacoffee&logoColor=black)](https://buymeacoffee.com/ibnuljaiffarabi) ![License](https://img.shields.io/badge/license-Apache--2.0-blue) ![Python](https://img.shields.io/badge/python-3.10%20%7C%203.11%20%7C%203.12%20%7C%203.13%20%7C%203.14-blue) ![Offline](https://img.shields.io/badge/runs-offline%20on%20CPU-green)

## Highlights

- **Verified, not generated.** Every answer is computed by a certified biomedical science capability and checked (substituted back, dimension-checked or compared with an independent computation) before it is returned.
- **Zero wrong answers by design.** Seven explicit outcomes — SOLVED, NEEDS_INPUT, AMBIGUOUS, INVALID_INPUT, OUT_OF_SCOPE, UNRECOGNIZED_FORMAT, INTERNAL_ERROR — never a silent guess. Every one of its 561 documented answers is reproduced exactly on every tested platform.
- **Plain-English questions and a guaranteed typed interface.** `ask()` reads documented question forms; `solve(capability_id, **inputs)` always solves a supported task with valid inputs.
- **Explains itself.** Steps, formulas in LaTeX, units, assumptions and the model used, ready for notebooks, tutoring apps and homework checkers.
- **Small and portable.** Pure Python (sympy, mpmath); identical answers on Windows and Linux (Python 3.10, 3.11, 3.12, 3.13, 3.14).
- **Coverage:** pharmacokinetic and epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative model calculations only, not medical advice).

## Model overview

| Property | Value |
|---|---|
| Package | `biomedical-science-ai999x` 1.0.1 (import `biomedical_science_ai999x`) |
| Type | Deterministic neuro-symbolic biomedical science engine (rule-based reading + certified symbolic capabilities) |
| Capabilities | 86 certified procedures |
| Knowledge pack | version 1.0.0 |
| Golden answers | 561 documented answers, identical on every tested platform |
| Platforms tested | Windows (Python 3.13), Linux x86-64 (Python 3.10, 3.11, 3.12, 3.13, 3.14) |
| Runtime | CPU only, offline; no GPU, no network, no API key |
| License | Apache-2.0 |
| Creator / engineer | Ibnul Jaif Farabi |
| Organization | AI999X |

## Quickstart

```bash
pip install biomedical-science-ai999x
```

```python
import biomedical_science_ai999x as biomedical_science

biomedical_science.ask("IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h").show()
# -> SOLVED — C = 0.00686015, fraction_remaining = 0.548812  (bmed_pk_iv_bolus_concentration)
biomedical_science.ask("SIR final size: R0 = 2.5, s0 = 0.99, i0 = 0.01").show()
# -> SOLVED — s_inf = 0.105894, attack = 0.884106  (bmed_epi_sir_final_size)
biomedical_science.ask("Kaplan-Meier survival: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], t = 6").show()
# -> SOLVED — S_t = 0.6, median = 9  (bmed_km_survival_median)
biomedical_science.ask("GHK voltage: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, Cl_o = 116 mmol/L, Cl_i = 4 mmol/L").show()
# -> SOLVED — V_m = -0.0750663  (bmed_ghk_voltage)
biomedical_science.solve("bmed_odds_ratio_ci", a=20, b=80, c=10, d=90, level=0.95).show()
# -> SOLVED — OR = 2.25, lower = 0.994295, upper = 5.09155  (bmed_odds_ratio_ci)
biomedical_science.ask("Who created you?").message
# -> biomedical-science-ai999x was created and engineered by Ibnul Jaif Farabi at AI999X.
```

Command line: `biomedical-science-ai999x "<question>"`, `biomedical-science-ai999x --capabilities`. The outputs above were produced by running this exact code on biomedical-science-ai999x 1.0.1.

## How every answer is verified

1. **Reading** — the question is turned into typed quantities (value, unit, kind, role) and one asked quantity; every stated number must be used or the question is refused.
2. **Planning** — only certified capabilities may answer; a capability applies only under the conditions the wording states (for example a pendulum, a liquid at rest, boiling).
3. **Computing** — exact symbolic or high-precision numeric evaluation.
4. **Checking** — dimensions, substitution back into the defining equation, and agreement between independent readings; any disagreement becomes a refusal, never a number.

Capabilities are certified before they ship: each must reproduce hand-worked reference cases, reject a deliberately wrong variant (a control), and leave every benchmark answer unchanged.

## Capabilities

86 certified procedures. A few of them:

- **Procedures:** AUC from concentration-time data by the trapezoidal rule; Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice); Absolute risk reduction and number needed to treat with Wald intervals; Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice); Basal metabolic rate by a stated regression (a calculation, not medical advice); Body mass index (a calculation, not medical advice); CT number in Hounsfield units; Cardiac output by indicator dilution (Stewart-Hamilton); Cohen's kappa for agreement between two raters or two tests; Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice); Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice); Cox proportional-hazards model with one covariate: hazard ratio (Efron ties); Diffusion MRI: apparent diffusion coefficient from signals at several b-values; Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice); Elimination rate constant and half-life from concentration data (log-linear fit); Exponential survival model: hazard rate, mean and median survival (maximum likelihood); FitzHugh-Nagumo neuron model: fixed point and its stability; Goldman-Hodgkin-Katz current equation: ionic current density through a membrane; Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl; Hazard ratio of two exponential survival groups with Wald interval; Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice); Hepatic clearance: well-stirred liver model (a model calculation, not medical advice); Hodgkin-Huxley model: potassium conductance after a voltage-clamp step; Hodgkin-Huxley model: steady-state gating variables at a membrane potential …

`biomedical_science_ai999x.templates()` lists every question form `ask()` reads; the capability guide (PDF, every example executed) is included in this repository.

## Out of scope

- Open-ended explanation, essay writing or tutoring dialogue: it is a solver, not a chatbot.
- Questions outside its certified biomedical science capabilities, and any question whose stated numbers it cannot all use — it refuses with the reason instead.
- High-stakes engineering, medical or legal decisions without independent review.

## Limitations

- It answers what its certified capabilities cover (pharmacokinetic and epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative model calculations only, not medical advice)); anything else is refused with a reason.
- `ask()` reads documented question forms and common phrasings; unusual wording may be refused — use `solve()` for guaranteed results.
- Results are as correct as the certified relations and their stated conditions; a certified relation is a checked model, not a claim about every physical situation.

## Citation

```bibtex
@software{biomedical_science_ai999x_1_0_1,
  title  = {biomedical-science-ai999x: an offline, verified biomedical science solver},
  author = {Farabi, Ibnul Jaif}, organization = {AI999X},
  year   = {2026}, version = {1.0.1},
  url    = {https://huggingface.co/AI999X/biomedical-science-ai999x}
}
```

## Contact

Questions, bug reports and requests: open a discussion on https://huggingface.co/AI999X/biomedical-science-ai999x/discussions or visit https://ai999x.com.

## Support the project

biomedical-science-ai999x is free and open source. If it helps you, you can support further development on [Buy Me a Coffee](https://buymeacoffee.com/ibnuljaiffarabi).

Created and engineered by **Ibnul Jaif Farabi** · **[AI999X](https://huggingface.co/AI999X)** · [ai999x.com](https://ai999x.com)

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/biomedical-science-ai999x/) (`pip install .` builds it). Releases are tagged `v<version>`; the wheel is attached to each [GitHub release](https://github.com/AI999X/biomedical-science-ai999x/releases). The optional ONNX models (formula graphs and, where measured, a decision model) are on [Hugging Face](https://huggingface.co/AI999X/biomedical-science-ai999x/tree/main/onnx).
