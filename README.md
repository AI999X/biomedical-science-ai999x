# Biomedical Science (biomedical-science-ai999x)

[![tests](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/biomedical-science-ai999x)](https://pypi.org/project/biomedical-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/biomedical-science-ai999x)

Biomedical Science covers pharmacokinetics and dosing models, epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative only, no clinical advice): 96 calculations in 10 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Biomedical Science desktop app: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice), with its worked example and live scene](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/hero.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-biomedical-science-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-biomedical-science-1.6.7-linux-x86_64.tar.gz) |
| **Command line** `biomedical-science` (Windows, Linux) | [Windows](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-windows-x86_64.zip) · [Linux](https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.7/ai999x-cli-1.6.7-linux-x86_64.tar.gz) |
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

## Guides

- Desktop app guide: the window, every calculation, scenes and charts: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_desktop_guide.md)
- Python guide: every calculation with a Python example: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_capabilities.md)
- Command-line guide: every calculation with the command and what it printed: [PDF](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.pdf) · [HTML](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.html) · [Markdown](https://huggingface.co/AI999X/biomedical-science-ai999x/blob/main/docs/biomedical_science_ai999x_rust_guide.md)

Every calculation, with its formula, a worked example and its figures: **[Biomedical Science on ai999x.com](https://www.ai999x.com/models/biomedical-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.7 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/biomedical-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.7`; the [GitHub release](https://github.com/AI999X/biomedical-science-ai999x/releases/tag/v1.6.7) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `biomedical-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/biomedical-science`) and its data (`rust/models/biomedical_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p biomedical-science-ai999x
./target/release/biomedical-science --help
cargo test -p biomedical-science-ai999x
```
