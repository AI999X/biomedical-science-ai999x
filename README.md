# Biomedical Science (biomedical-science-ai999x)

[![tests](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/biomedical-science-ai999x)](https://pypi.org/project/biomedical-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/biomedical-science-ai999x)

Biomedical Science covers pharmacokinetics and dosing models, epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative only, no clinical advice): 96 calculations in 10 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Biomedical Science desktop app: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice), with its worked example and live scene](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/hero.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-biomedical-science-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-biomedical-science-1.6.6-linux-x86_64.tar.gz) |
| **Command line** `biomedical-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.6/ai999x-cli-1.6.6-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/biomedical-science-ai999x/#capabilities) |
| **Python** 3.10+ | `pip install biomedical-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## See it

From the Biomedical Science desktop app: every scene and chart is drawn from the computed answer.

[![Biomedical Science desktop app: a slider moves an input and the answer, the scene and the charts follow](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/clip-loop.webp)](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/clip.mp4)

*[Watch the full clip (MP4)](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/clip.mp4)*

| | |
|---|---|
| [![One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-1.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_pk_iv_bolus_concentration)<br>One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) | [![SIR epidemic model: final size (final susceptible fraction and attack fraction): the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-2.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_epi_sir_final_size)<br>SIR epidemic model: final size (final susceptible fraction and attack fraction) |
| [![Kaplan-Meier survival estimate at a time and median survival: the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-3.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_km_survival_median)<br>Kaplan-Meier survival estimate at a time and median survival | [![Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl: the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-4.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_ghk_voltage)<br>Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl |
| [![Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval: the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-5.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_odds_ratio_ci)<br>Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval | [![Body mass index (a calculation, not medical advice): the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-6.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_bmi)<br>Body mass index (a calculation, not medical advice) |
| [![Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice): the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-7.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_dose_liquid_volume)<br>Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice) | [![CT number in Hounsfield units: the live scene in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-8.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_hounsfield_units)<br>CT number in Hounsfield units |

Charts:

|   |   |   |   |
|---|---|---|---|
| [![One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): a chart in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-1.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_pk_iv_bolus_concentration)<br>One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) | [![SIR epidemic model: final size (final susceptible fraction and attack fraction): a chart in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-2.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_epi_sir_final_size)<br>SIR epidemic model: final size (final susceptible fraction and attack fraction) | [![Kaplan-Meier survival estimate at a time and median survival: a chart in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-3.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_km_survival_median)<br>Kaplan-Meier survival estimate at a time and median survival | [![Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl: a chart in the Biomedical Science desktop app](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-4.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/#cap-bmed_ghk_voltage)<br>Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl |

## Ask in words: 15 examples

Each question below was asked on this release; the comment under it is the line it printed.

```python
import biomedical_science_ai999x as biomedical_science

a = biomedical_science.ask(
    "Cohen's kappa: counts = [[40, 10], [5, 45]]"
)
print(a.outcome, a.values)
# SOLVED {'kappa': 0.7, 'p_observed': 0.85}

a = biomedical_science.ask(
    "Hounsfield units: mu = 0.38 1/cm, mu_water = 0.19 1/cm"
)
print(a.outcome, a.values)
# SOLVED {'HU': 1000.0, 'mu_rel': 2.0}

a = biomedical_science.ask(
    "Bioavailability from profiles: D_iv = 100 mg, t_iv = [0, 1, 2, 4, "
    "8, 12], C_iv = [10, 8.2, 6.7, 4.5, 2, 0.9], D_oral = 200 mg, "
    "t_oral = [0, 1, 2, 4, 8, 12], C_oral = [0, 6.1, 8, 7.2, 3.9, 1.8]"
)
print(a.outcome, a.values)
# SOLVED {'AUC_iv': 46.55, 'AUC_oral': 58.9, 'F': 0.632653061224}

a = biomedical_science.ask(
    "AUC of an IV bolus: D = 500 mg, CL = 5 L/h, V = 40 L, t = 8 h"
)
print(a.outcome, a.values)
# SOLVED {'AUC_inf': 360.0, 'AUC_0_t': 227.563401178}

a = biomedical_science.ask("Body mass index: m = 70 kg, h = 1.75 m")
print(a.outcome, a.values)
# SOLVED {'BMI': 22.8571428571, 'BMI_prime': 0.914285714286}

a = biomedical_science.ask("ADC fit: b = [0, 1000], S = [1200, 420]")
print(a.outcome, a.values)
# SOLVED {'ADC': 0.0010498221245, 'S0': 1200.0}

a = biomedical_science.ask(
    "GHK current density: P = 1e-8 m/s, z = 1, C_in = 140 mmol/L, "
    "C_out = 4 mmol/L, V = -20 mV, T = 310 K"
)
print(a.outcome, a.values)
# SOLVED {'I': 0.085282548488}

a = biomedical_science.ask(
    "AUC by the trapezoidal rule: t = [0, 0.5, 1, 2, 4, 6, 8, 12], C = "
    "[0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5], method = linear"
)
print(a.outcome, a.values)
# SOLVED {'AUC': 51.85}

a = biomedical_science.ask(
    "Time to fraction of steady state: CL = 5 L/h, V = 40 L, f = 90%"
)
print(a.outcome, a.values)
# SOLVED {'t': 66314.4506782}

a = biomedical_science.ask(
    "Indicator dilution cardiac output: m = 5 mg, t = [0, 2, 4, 6, 8, "
    "10, 12, 14, 16], C = [0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2]"
)
print(a.outcome, a.values)
# SOLVED {'AUC': 29.6, 'CO': 0.000168918918919}

a = biomedical_science.ask(
    "Reproduction number from growth rate: r = 0.1 1/day, Tg_mean = 5 "
    "day, Tg_sd = 2 day"
)
print(a.outcome, a.values)
# SOLVED {'R': 1.61770181142}

a = biomedical_science.ask(
    "Incidence rate ratio: events1 = 30, time1 = 1000, events2 = 15, "
    "time2 = 1200, level = 95%"
)
print(a.outcome, a.values)
# SOLVED {'IRR': 2.4, 'lower': 1.29133130431, 'upper': 4.46051294567}

a = biomedical_science.ask(
    "Gaussian PSF MTF: fwhm = 0.5 mm, f = 1 1/mm"
)
print(a.outcome, a.values)
# SOLVED {'MTF': 0.410685800378, 'f50': 882.542400611}

a = biomedical_science.ask(
    "GHK voltage: T = 310 K, PK = 1, PNa = 0.04, PCl = 0.45, K_o = 4 "
    "mmol/L, K_i = 140 mmol/L, Na_o = 145 mmol/L, Na_i = 12 mmol/L, "
    "Cl_o = 116 mmol/L, Cl_i = 4 mmol/L"
)
print(a.outcome, a.values)
# SOLVED {'V_m': -0.0750662761527}

a = biomedical_science.ask(
    "Infusion concentration: R0 = 50 mg/h, CL = 5 L/h, V = 40 L, t = 6 "
    "h"
)
print(a.outcome, a.values)
# SOLVED {'C': 0.00527633447259, 'C_ss': 0.01}
```

## Command line: 15 examples

Each command below was run with `biomedical-science` of this release; the lines under it are the answer it printed (the full answer goes on with the method, every input in SI units and the conditions it assumes).

```text
$ biomedical-science solve bmed_bmr "m=70 kg" "h=175 cm" age=30 sex=male equation=mifflin_st_jeor
BMR_kcal_day = 1648.75
BMR_W = 79.84224537 W
BMR_MJ_day = 6.89837

$ biomedical-science solve bmed_cohens_kappa "counts=[[40, 10], [5, 45]]"
kappa = 0.7
p_observed = 0.85

$ biomedical-science solve bmed_iv_rate_from_dose_rate "drug=400 mg" "bag_volume=250 mL" dose_ug_per_kg_min=5 "W=80 kg"
C = 1.6 kg/m^3
dose_rate = 6.666666667e-09 kg/s
dose_mg_h = 24
flow = 4.166666667e-09 m^3/s
flow_mL_h = 15

$ biomedical-science solve bmed_epi_herd_immunity R0=4 efficacy=0.9
threshold = 0.75
threshold_percent = 75
coverage = 0.8333333333
coverage_percent = 83.33333333
reachable = reachable

$ biomedical-science solve bmed_nnt_ci events_t=15 n_t=100 events_c=30 n_c=100 level=0.95
ARR = 0.15
ARR_lower = 0.03613638193
ARR_upper = 0.2638636181
NNT = 6.666666667
NNT_lower = 3.789836611
NNT_upper = 27.67294197

$ biomedical-science solve bmed_hounsfield_units "mu=0.38 1/cm" "mu_water=0.19 1/cm"
HU = 1000
mu_rel = 2

$ biomedical-science solve bmed_fitzhugh_nagumo_fixed_point a=0.7 b=0.8 eps=0.08 I=0
v_star = -1.199408035
w_star = -0.6242600441
trace = -0.502579635
determinant = 0.1080690966
stable = true

$ biomedical-science solve bmed_pk_bioavailability_from_profiles "D_iv=100 mg" "t_iv=[0, 1, 2, 4, 8, 12]" "C_iv=[10, 8.2, 6.7, 4.5, 2, 0.9]" "D_oral=200 mg" "t_oral=[0, 1, 2, 4, 8, 12]" "C_oral=[0, 6.1, 8, 7.2, 3.9, 1.8]"
AUC_iv = 46.55
AUC_oral = 58.9
F = 0.6326530612

$ biomedical-science solve bmed_pk_auc_iv_bolus "D=500 mg" "CL=5 L/h" "V=40 L" "t=8 h"
AUC_inf = 360 kg*s/m^3
AUC_0_t = 227.5634012 kg*s/m^3

$ biomedical-science solve bmed_cox_hazard_ratio "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" "x=[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]" level=0.95
beta = 0.04732279296
HR = 1.04846039
se = 0.6738763833
HR_lower = 0.2798642406
HR_upper = 3.927865837

$ biomedical-science solve bmed_bmi "m=70 kg" "h=1.75 m"
BMI = 22.85714286
BMI_prime = 0.9142857143

$ biomedical-science solve bmed_dx_youden_cutoff "scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4]" "scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1]"
threshold = 0.55
sensitivity = 0.8
specificity = 0.8333333333
J = 0.6333333333

$ biomedical-science solve bmed_iv_flow_rate "V=1000 mL" "T=8 h" drop_factor=15
flow = 3.472222222e-08 m^3/s
flow_mL_h = 125
duration = 28800 s
duration_h = 8
duration_min = 480
drops_per_min = 31.25

$ biomedical-science solve bmed_epi_reed_frost_final_size n=4 m=1 p=0.2
mean_final = 1.270137242
P_no_spread = 0.4096
distribution = [0.4096,0.2097152,0.163577856,0.1351614464,0.0819454976]

$ biomedical-science solve bmed_epi_basic_rates cases=150 population=10000 prevalent=400 deaths=6 "period=1 yr"
incidence_proportion = 0.015
incidence_per_100000 = 1500
incidence_rate = 4.753213172e-10 Hz
incidence_per_100000_person_years = 1500
prevalence = 0.04
prevalence_per_100000 = 4000
```

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Biomedical Science on ai999x.com](https://www.ai999x.com/models/biomedical-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.6 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/biomedical-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.6`; the [GitHub release](https://github.com/AI999X/biomedical-science-ai999x/releases/tag/v1.6.6) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `biomedical-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/biomedical-science`) and its data (`rust/models/biomedical_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p biomedical-science-ai999x
./target/release/biomedical-science --help
cargo test -p biomedical-science-ai999x
```
