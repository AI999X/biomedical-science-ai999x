# Biomedical Science: the desktop app, user guide

**Biomedical Science**, desktop app 1.6.5 for Windows and Linux. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Biomedical Science covers pharmacokinetics and dosing models, epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative only, no clinical advice). Its desktop app holds all 96 calculations in 10 topics: pick one, change its inputs and the answer, a live scene and charts follow at once. It runs on your own computer with no network connection and nothing else to install. The pictures in this guide were taken in this app.

## Contents

- 1. Get it and start it
- 2. The window
- 3. Compute a calculation
- 4. Ask in words
- 5. The live scene and the charts
- 6. Light and dark
- 7. Every calculation
- 8. Other ways to use it

## 1. Get it and start it

| system | download |
|---|---|
| Windows x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-biomedical-science-1.6.5-windows-x86_64.zip |
| Linux x86-64 | https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-biomedical-science-1.6.5-linux-x86_64.tar.gz |

Windows: Download ai999x-biomedical-science-1.6.5-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-biomedical-science.exe. If Windows SmartScreen asks, choose More info → Run anyway.

Linux:

```shell
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/ai999x-biomedical-science-1.6.5-linux-x86_64.tar.gz
curl -LO https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-biomedical-science-1.6.5-linux-x86_64.tar.gz
./ai999x-biomedical-science-1.6.5-linux-x86_64/ai999x-biomedical-science
```

Checksums of every download: https://github.com/AI999X/ai999x-desktop/releases/download/v1.6.5/SHA256SUMS. Install steps for every app: https://www.ai999x.com/desktop-cli/

## 2. The window

![The Biomedical Science app, opened on One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) with its worked example.](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/hero.webp)

*The Biomedical Science app, opened on One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) with its worked example.*

On the left, a search box and the topics with their calculations. In the middle, the chosen calculation: its answer at the top, then the live scene (or its charts), what the answer means and the formula. On the right, its inputs, each with a slider where that makes sense, and the Compute and Reset buttons. At the bottom, a box for a question in words.

## 3. Compute a calculation

Open Pharmacokinetic models › One-compartment IV bolus: concentration at a time (a model calculation, not medical advice) (or type part of its name in the search box). It opens with a worked example filled in: “IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h” The answer: C = 0.006860145451 kg/m^3, fraction_remaining = 0.5488116361.

Change any input (type a number with its unit, or drag its slider) and the answer, the scene and the charts follow; Compute recomputes after typing, Reset brings back the worked example. An input outside the range the formula holds for is refused with the reason, never answered with a guessed number.

![One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): the live scene of the worked example.](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-1.webp)

*One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): the live scene of the worked example.*

## 4. Ask in words

Type a question as you would say it, for example “IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h”. The app reads the quantities and their units, picks the calculation and answers; the command-line program reads it the same way and printed:

```text
Read as: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)
C = 0.006860145451 kg/m^3
fraction_remaining = 0.5488116361
```

When a question names a quantity the app cannot place, it says what is missing instead of guessing.

## 5. The live scene and the charts

Every calculation draws a scene from its answer: it moves as an input changes. The Charts button shows charts that vary one input around your values; Enlarge opens one in a large view. Scenes from other topics:

![SIR epidemic model: final size (final susceptible fraction and attack fraction)](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-2.webp)

*SIR epidemic model: final size (final susceptible fraction and attack fraction)*

![Kaplan-Meier survival estimate at a time and median survival](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-3.webp)

*Kaplan-Meier survival estimate at a time and median survival*

![Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-4.webp)

*Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl*

![Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-5.webp)

*Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval*

![Body mass index (a calculation, not medical advice)](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-6.webp)

*Body mass index (a calculation, not medical advice)*

![Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice)](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-7.webp)

*Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice)*

![CT number in Hounsfield units](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/scene-8.webp)

*CT number in Hounsfield units*

![One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): a chart, enlarged.](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-1.webp)

*One-compartment IV bolus: concentration at a time (a model calculation, not medical advice): a chart, enlarged.*

![SIR epidemic model: final size (final susceptible fraction and attack fraction): a chart, enlarged.](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/chart-2.webp)

*SIR epidemic model: final size (final susceptible fraction and attack fraction): a chart, enlarged.*

## 6. Light and dark

The app follows a light or a dark theme (the switch is in its top bar).

![The Biomedical Science app in the dark theme.](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/hero-dark.webp)

*The Biomedical Science app in the dark theme.*

## 7. Every calculation

96 calculations in 10 topics; each has its own entry on the model's page (https://www.ai999x.com/models/biomedical-science-ai999x/) with its formula, worked example and charts.

| topic | calculations |
|---|---|
| Biomechanics | Basal metabolic rate by a stated regression (a calculation, not medical advice), Body mass index (a calculation, not medical advice), Cardiac output by indicator dilution (Stewart-Hamilton), Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress, Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance, Single-compartment respiratory mechanics: resistance and compliance by least squares, Standard linear solid (Zener) tissue model: creep under constant stress, Standard linear solid (Zener) tissue model: stress relaxation at constant strain, Static lever model of the forearm: muscle force and joint reaction, Two-element Windkessel: diastolic pressure decay, Windkessel models (two- and three-element): input impedance at a frequency |
| Diagnostic test statistics | Cohen's kappa for agreement between two raters or two tests, Optimal diagnostic cut-off by the Youden index, Predictive values and likelihood ratios of a diagnostic test, ROC curve area of a diagnostic score (Mann-Whitney form), Sensitivity and specificity of a diagnostic score at a cut-off, Sensitivity and specificity with Wilson score intervals |
| Dosage calculations | Infusion pump rate from a dose rate (arithmetic, not medical advice), IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice), Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice), Tablets to give: desired over have (dosage arithmetic, not medical advice), Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice), Weight-based dose (dosage arithmetic, not medical advice) |
| Epidemic models | Herd-immunity threshold and critical vaccination coverage, Reed-Frost chain-binomial epidemic: distribution and mean of the final size, Reproduction number from an epidemic growth rate (gamma-distributed generation interval), SEIR epidemic model: final size, SEIR epidemic model: initial exponential growth rate from R0, SEIR epidemic model: peak prevalence and the time of the peak, SIR epidemic model: final size (final susceptible fraction and attack fraction), SIR epidemic model: peak prevalence and the time of the peak, SIS epidemic model: prevalence at a time and the endemic level |
| Epidemiological measures of association | Absolute risk reduction and number needed to treat with Wald intervals, Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality, Incidence rate ratio of two person-time groups with log-Wald interval, Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval, Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval, Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval, Standardized mortality (incidence) ratio with exact Poisson interval, Vaccine efficacy from attack rates |
| Medical-imaging physics | CT number in Hounsfield units, Diffusion MRI: apparent diffusion coefficient from signals at several b-values, Imaging resolution: MTF of a Gaussian point-spread function, Larmor (precession) frequency in a magnetic field, MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal), MRI inversion recovery: longitudinal magnetisation at TI and the null point, MRI multi-echo T2 estimate (log-linear fit), MRI spin-echo signal from T1, T2, TR and TE (simplified model), MRI spoiled gradient-echo signal (Ernst equation), Signal-to-noise ratio gained by signal averaging, Ultrasound reflection and transmission at a tissue interface (normal incidence), Ultrasound transmission through a matching layer (normal incidence), Ultrasound wavelength in tissue, X-ray attenuation through tissue layers (narrow monoenergetic beam) |
| Neuron models | FitzHugh-Nagumo neuron model: fixed point and its stability, Goldman-Hodgkin-Katz current equation: ionic current density through a membrane, Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl, Hodgkin-Huxley model: potassium conductance after a voltage-clamp step, Hodgkin-Huxley model: steady-state gating variables at a membrane potential, Hodgkin-Huxley model: zero-current steady (resting) membrane potential, Leaky integrate-and-fire neuron: firing rate under constant current, Passive cable (dendrite or axon): steady-state voltage along it and input resistance, Passive membrane (leaky integrator): voltage at a time under constant current, Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current |
| Pharmacokinetic data analysis | Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice), AUC from concentration-time data by the trapezoidal rule, Elimination rate constant and half-life from concentration data (log-linear fit), Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz) |
| Pharmacokinetic models | Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice), Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice), Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice), Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice), Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice), Hepatic clearance: well-stirred liver model (a model calculation, not medical advice), Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice), Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation), Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice), Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice), Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice), One-compartment IV bolus: concentration at a time (a model calculation, not medical advice), Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice), Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice), Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice), Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice), Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice), Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice), Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice), Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice) |
| Survival analysis | Cox proportional-hazards model with one covariate: hazard ratio (Efron ties), Exponential survival model: hazard rate, mean and median survival (maximum likelihood), Hazard ratio of two exponential survival groups with Wald interval, Kaplan-Meier survival estimate at a time and median survival, Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval, Log-rank test comparing two survival curves, Nelson-Aalen cumulative hazard at a time, Restricted mean survival time from the Kaplan-Meier curve |

## 8. Other ways to use it

The same engine runs on the command line (`biomedical-science`, its own guide), in Python (`pip install biomedical-science-ai999x`, `import biomedical_science_ai999x`, its own guide) and in the browser on https://www.ai999x.com/models/biomedical-science-ai999x/.

Licence: Apache-2.0 · AI999X · https://www.ai999x.com/models/biomedical-science-ai999x/
