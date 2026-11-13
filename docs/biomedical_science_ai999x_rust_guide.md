# biomedical-science: the Biomedical Science program, user guide

**biomedical-science-ai999x 1.6.0**, Rust build: command line, desktop app and browser. **Creator / engineer:** Ibnul Jaif Farabi · **Company:** AI999X · support@ai999x.com

Biomedical Science covers pharmacokinetics and dosing models, epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative only, no clinical advice). This guide is about `biomedical-science`, the model as a self-contained program for Windows and Linux: 96 capabilities in 10 topics, answered on your own computer with no network, no Python and nothing else to install. The same compiled model runs in the AI999X desktop app and in the browser on the model's page (https://www.ai999x.com/models/biomedical-science-ai999x/); the Python package (`pip install biomedical-science-ai999x`) has its own guide. Every command output printed here is what the program printed.

## Contents

- 1. Get it: desktop app, command line, browser, Python
- 2. Quick start
- 3. Writing inputs
- 4. Outcomes and exit codes
- 5. Scripts and agents: batch and JSON
- 6. Figures
- 7. In a web page or Node.js: WebAssembly
- 8. Every capability

## 1. Get it

Version 1.6.0 of the AI999X desktop app and of the command-line bundle. Checksums for every file: https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS; the newest version is always under https://downloads.ai999x.com/desktop/latest/. Full instructions: https://www.ai999x.com/desktop-cli/

| what | system | download |
|---|---|---|
| Desktop app: Biomedical Science | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-biomedical-science-1.6.0-windows-x86_64.zip |
| Desktop app: Biomedical Science | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-biomedical-science-1.6.0-linux-x86_64.tar.gz |
| Desktop app: all models | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-windows-x86_64.zip |
| Desktop app: all models | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz |
| Command line | Windows x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-windows-x86_64.zip |
| Command line | Linux x86-64 | https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz |

### Desktop app

Biomedical Science in its own app: search, questions in words, input forms, live figures and explanations. Windows: Download ai999x-biomedical-science-1.6.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x-biomedical-science.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-biomedical-science-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-biomedical-science-1.6.0-linux-x86_64.tar.gz
./ai999x-biomedical-science-1.6.0-linux-x86_64/ai999x-biomedical-science
```

It opens on this model: type a question or pick a capability, fill in the form and press Compute. The AI999X desktop app with every model works the same way (choose the model first). Windows: Download ai999x-1.6.0-windows-x86_64.zip, right-click it → Extract All, then open the folder and start ai999x.exe. If Windows SmartScreen asks, choose More info → Run anyway. Linux:

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
tar xzf ai999x-1.6.0-linux-x86_64.tar.gz
./ai999x-1.6.0-linux-x86_64/ai999x
```

### Command line: Linux

The bundle holds one program per model; `biomedical-science` is this one. The checksum line should print OK.

```shell
curl -LO https://downloads.ai999x.com/desktop/1.6.0/ai999x-cli-1.6.0-linux-x86_64.tar.gz
curl -LO https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS
sha256sum -c --ignore-missing SHA256SUMS
mkdir -p ~/.local/share && tar xzf ai999x-cli-1.6.0-linux-x86_64.tar.gz -C ~/.local/share
echo 'export PATH="$HOME/.local/share/ai999x-cli-1.6.0-linux-x86_64:$PATH"' >> ~/.bashrc && source ~/.bashrc
biomedical-science --version
```

### Command line: Windows (PowerShell)

The fourth line should print True.

```powershell
$F = "ai999x-cli-1.6.0-windows-x86_64.zip"
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/$F" -OutFile $F
Invoke-WebRequest "https://downloads.ai999x.com/desktop/1.6.0/SHA256SUMS" -OutFile SHA256SUMS
(Get-FileHash $F -Algorithm SHA256).Hash.ToLower() -eq ((Select-String -SimpleMatch $F SHA256SUMS).Line -split ' ')[0]
Expand-Archive $F -DestinationPath "$env:LOCALAPPDATA\AI999X" -Force
$D = "$env:LOCALAPPDATA\AI999X\ai999x-cli-1.6.0-windows-x86_64"; $env:Path += ";$D"
[Environment]::SetEnvironmentVariable("Path", [Environment]::GetEnvironmentVariable("Path", "User") + ";$D", "User")
biomedical-science --version
```

### In your browser

Open https://www.ai999x.com/models/biomedical-science-ai999x/, choose a capability and press Compute in your browser: the same program runs on your device as WebAssembly; nothing is sent anywhere.

### Python

```shell
pip install biomedical-science-ai999x
```

## 2. Quick start

### 2.1 Check the install

Prints the model, its version and that this is the Rust build.

```shell
$ biomedical-science --version
biomedical-science-ai999x 1.6.0 (Rust build, ax-engine 1.6.0)
```

### 2.2 Ask in words

Write the question as you would say it: the program reads the quantities and their units, picks the capability and answers.

```shell
$ biomedical-science ask "IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h"
Read as: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)
C = 0.006860145451 kg/m^3
fraction_remaining = 0.5488116361
  by: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)  (C(t) = (D/V) exp(-k t))
  D = 0.0005 kg
  V = 0.04 m^3
  k = 2.777777778e-05 1/s
  t = 21600 s
  assumes: one compartment, instantaneous mixing; first-order elimination; t >= 0
```

### 2.3 Find a capability

Search the capabilities by what you want to know; the first column is the id to pass to info and solve.

```shell
$ biomedical-science search "one-compartment iv bolus: concentration at a time (a model calculation, not medical advice)"
bmed_pk_iv_bolus_concentration       One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_auc_iv_bolus                 Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_effect_site_concentration    Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_two_compartment_bolus        Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_loading_maintenance          Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation)  (pharmacokinetic models)
bmed_pk_nth_dose_iv                  Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_steady_state_iv              Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice)  (pharmacokinetic models)
bmed_pk_infusion_concentration       Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice)  (pharmacokinetic models)
… 12 more
```

### 2.4 List the capabilities

Every capability the program answers: its id (what you pass to solve) and its name.

```shell
$ biomedical-science list
bmed_bmi                                 Body mass index (a calculation, not medical advice)
bmed_bmr                                 Basal metabolic rate by a stated regression (a calculation, not medical advice)
bmed_cable_steady_state                  Passive cable (dendrite or axon): steady-state voltage along it and input resistance
bmed_cohens_kappa                        Cohen's kappa for agreement between two raters or two tests
bmed_cox_hazard_ratio                    Cox proportional-hazards model with one covariate: hazard ratio (Efron ties)
bmed_dose_liquid_volume                  Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice)
bmed_dose_tablets                        Tablets to give: desired over have (dosage arithmetic, not medical advice)
bmed_dose_weight_based                   Weight-based dose (dosage arithmetic, not medical advice)
… 88 more
```

### 2.5 What a capability needs

The equation or method, every input and output with its SI unit. Read this before solve: it tells you the input names.

```shell
$ biomedical-science info bmed_pk_iv_bolus_concentration
One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)
  C(t) = (D/V) exp(-k t)
  in   D        bolus dose [kg]
  in   V        volume of distribution [m^3]
  in   k        elimination rate constant [1/s]
  in   t        time after the dose [s]
  out  C        concentration at time t [kg/m^3]
  out  fraction_remaining fraction of the dose still in the body [1]
```

### 2.6 Solve

Inputs as name=value, each with any unit of the right kind. The answer lists every output, the method used, each input converted to SI, and the conditions the answer assumes.

```shell
$ biomedical-science solve bmed_pk_iv_bolus_concentration "D=500 mg" "V=40 L" "k=0.1 1/h" "t=6 h"
C = 0.006860145451 kg/m^3
fraction_remaining = 0.5488116361
  by: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)  (C(t) = (D/V) exp(-k t))
  D = 0.0005 kg
  V = 0.04 m^3
  k = 2.777777778e-05 1/s
  t = 21600 s
  assumes: one compartment, instantaneous mixing; first-order elimination; t >= 0
```

### 2.7 Machine-readable answer

--json prints the same answer as one line of JSON (values in SI): for scripts and other programs.

```shell
$ biomedical-science solve bmed_pk_iv_bolus_concentration "D=500 mg" "V=40 L" "k=0.1 1/h" "t=6 h" --json
{"outcome":"SOLVED","message":"","value":0.00686014545118,"unit":"kg/m^3","solved_for":"C","si":[["D",0.0005],["V",0.04],["k",0.00002777777777777778],["t",21600.0]],"values":[["C",0.00686014545118,"kg/m^3"],["fraction_remaining",0.548811636094,""]],"target":"C","phrase":"concentration at time t","assumptions":["one compartment, instantaneous mixing","first-order elimination","t >= 0"],"name":"One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)","equation":"C(t) = (D/V) exp(-k t)"}
```

### 2.8 When an input is missing

No number is guessed: the answer names what is missing, shows a complete call, and the program exits with status 1.

```shell
$ biomedical-science solve bmed_pk_iv_bolus_concentration "D=500 mg" "V=40 L" "k=0.1 1/h"
NEEDS_INPUT: bmed_pk_iv_bolus_concentration needs t (time after the dose). Example: solve('bmed_pk_iv_bolus_concentration', D='0.0005 kg', V='0.04 m^3', k='2.77778e-05 1/s', t='21600 s').
```

### 2.9 Many questions at once

One JSON request per line in, one JSON answer per line out, in order: for scripts, notebooks and agents (the program starts once). In PowerShell: Get-Content cases.jsonl | biomedical-science batch

`cases.jsonl`:

```json
{"key": "bmed_pk_iv_bolus_concentration", "inputs": {"D": "500 mg", "V": "40 L", "k": "0.1 1/h", "t": "6 h"}}
{"key": "bmed_epi_sir_final_size", "inputs": {"R0": 2.5, "s0": 0.99, "i0": 0.01}}
```

```shell
$ biomedical-science batch < cases.jsonl
{"outcome":"SOLVED","message":"","value":0.00686014545118,"unit":"kg/m^3","solved_for":"C","si":[["D",0.0005],["V",0.04],["k",0.00002777777777777778],["t",21600.0]],"values":[["C",0.00686014545118,"kg/m^3"],["fraction_remaining",0.548811636094,""]],"target":"C","phrase":"concentration at time t","assumptions":["one compartment, instantaneous mixing","first-order elimination","t >= 0"],"name":"One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)","equation":"C(t) = (D/V) exp(-k t)"}
{"outcome":"SOLVED","message":"","value":0.105894194336,"unit":"","solved_for":"s_inf","si":[["R0",2.5],["s0",0.99],["i0",0.01]],"values":[["s_inf",0.105894194336,""],["attack",0.884105805664,""]],"target":"s_inf","phrase":"susceptible fraction at the end","assumptions":["closed population, no births or deaths, homogeneous mixing","fractions of the population","the rest of the population (1 - s0 - i0) is immune"],"name":"SIR epidemic model: final size (final susceptible fraction and attack fraction)","equation":"final-size relation ln(s0/s_inf) = R0 (s0 + i0 - s_inf), solved by bisection on (0, min(s0, 1/R0)]"}
```

## 3. Writing inputs

Inputs are written `name=value`, one per argument; `biomedical-science info <capability>` lists the names. Quote an argument that holds a space or brackets (bash, zsh and PowerShell all accept double quotes). The answer always states each input as it was read, converted to SI, so a misread unit is visible at once.

| kind | example | how it is read |
|---|---|---|
| a number with a unit | `"R=0.128 nm"` | any unit of the same kind; converted to SI |
| a number in SI | `R=1.28e-10` | no unit means the SI unit shown by info |
| a choice | `structure=FCC` | one of the values info lists |
| a list or vector | `"v=[1, 2, 3]" or "v=1, 2, 3"` | brackets keep it a list |
| a matrix | `"A=[[1, 2], [3, 4]]"` | one list per row |
| true / false | `inclusive=true` | for yes/no options |

An equation can be solved for any of its quantities: give the others and add `--target <symbol>`. Without `--target`, the program solves for the quantity the capability is named after.

## 4. Outcomes and exit codes

The program never guesses a number: when it cannot answer, it says why and exits with status 1, so a script can tell an answer from a refusal without reading the text.

| outcome | meaning | exit status |
|---|---|---|
| SOLVED | the answer: every output with its unit, the method, the inputs in SI, the conditions assumed | 0 |
| NEEDS_INPUT | an input is missing; the message names it and shows a complete call | 1 |
| INVALID_INPUT | a value or unit of the wrong kind, an unknown input name, or a value outside the conditions | 1 |
| AMBIGUOUS | the inputs allow two different answers; say which one is meant | 1 |
| OUT_OF_SCOPE | the model has no capability for this, or the result would not be meaningful | 1 |
| (usage) | a command the program does not know: it prints its usage | 2 |

## 5. Scripts and agents: batch and JSON

`biomedical-science solve ... --json` prints one answer as one line of JSON. `biomedical-science batch` reads one request per line on standard input and writes one JSON answer per line, in order, starting the program once for any number of requests. A request is `{"key": "<capability>", "inputs": {"name": "value", ...}, "target": null}`. In PowerShell, pipe the file in: `Get-Content cases.jsonl | biomedical-science batch`. `biomedical-science catalog` prints every capability with its inputs as JSON, to build forms or tool definitions.

| answer field | meaning |
|---|---|
| outcome | one of the outcomes above |
| message | why there is no answer (empty when SOLVED) |
| value, unit | the main result in SI units |
| values | every output: [symbol, value, unit] (procedures) |
| target | the quantity solved for |
| si | each numeric input converted to SI: [symbol, value] |
| assumptions | the conditions the answer relies on |
| name, equation | the capability and its equation or method |

```python
import json, subprocess

reqs = [{"key": "<capability>", "inputs": {"name": "value"}}]
out = subprocess.run(["biomedical-science", "batch"], input="\n".join(map(json.dumps, reqs)),
                     capture_output=True, text=True, check=True).stdout
answers = [json.loads(line) for line in out.splitlines()]
```

## 6. Figures

`biomedical-science figures <capability> name=value ... --out DIR` draws the live figures for those inputs and writes each as an SVG file into DIR (an existing folder); without `--out` the SVG text comes back as JSON. The desktop app and the model page draw the same figures as soon as you press Compute.

## 7. In a web page or Node.js: WebAssembly

The same model is a WebAssembly module, `biomedical_science_ai999x.wasm` (https://www.ai999x.com/compute/rust/biomedical_science_ai999x.wasm), with a small loader, `ax-wasm.js` (https://www.ai999x.com/compute/rust/ax-wasm.js). It runs in a browser, a Web Worker or Node.js, with no server.

```html
<script src="ax-wasm.js"></script>
<script>
  AXWasm.load("biomedical_science_ai999x.wasm").then(function (m) {
    var a = m.solve({key: "<capability>", inputs: {name: "value"}});
    console.log(a.outcome, a.values || a.value);
    console.log(m.catalog().capabilities.length + " capabilities");
  });
</script>
```

## 8. Every capability

By topic. For each: what it does, what it needs and gives, when it applies, and a command with what the program printed for it. `biomedical-science info <id>` shows the same inputs and outputs in the terminal.

### Biomechanics (11)

<a id="bmed_bmr"></a>

#### Basal metabolic rate by a stated regression (a calculation, not medical advice), `bmed_bmr`

The basal metabolic rate estimated by the Mifflin-St Jeor (1990) or the revised Harris-Benedict (Roza and Shizgal 1984) regression from mass, height, age and sex, in kcal/day, W and MJ/day; a model calculation, not dietary or medical advice.

$$
\mathrm{BMR} = 10\,m + 6.25\,h - 5\,a + s
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | body mass | kg |
| `h` | in | height | m |
| `age` | in | age in years (adults) | - |
| `sex` | in | sex (the equations have a male and a female form) | - |
| `equation` | in | regression: mifflin_st_jeor or harris_benedict_revised | - |
| `BMR_kcal_day` | out | basal metabolic rate in kcal/day | - |
| `BMR_W` | out | basal metabolic rate (W) | W |
| `BMR_MJ_day` | out | basal metabolic rate in MJ/day | - |

**Applies when:** a population regression, not a measurement and not advice; adults 18 to 120 years; 1 kcal = 4.184 kJ.

```shell
$ biomedical-science solve bmed_bmr "m=70 kg" "h=175 cm" age=30 sex=male equation=mifflin_st_jeor
BMR_kcal_day = 1648.75
BMR_W = 79.84224537 W
BMR_MJ_day = 6.89837
  by: Basal metabolic rate by a stated regression (a calculation, not medical advice)  (Mifflin-St Jeor: 10 m + 6.25 h_cm - 5 age + 5 (male) or - 161 (female); revised Harris-Benedict: 88.362 + 13.397 m + 4.799 h_cm - 5.677 age (male), 447.593 + 9.247 m + 3.098 h_cm - 4.330 age (female))
  m = 70 kg
  h = 1.75 m
  age = 30
  sex = male
  equation = mifflin_st_jeor
  assumes: a population regression, not a measurement and not advice; adults 18 to 120 years; 1 kcal = 4.184 kJ
```

**What the worked example means:** This calculation estimates the basal metabolic rate using either the Mifflin-St Jeor or revised Harris-Benedict regression, based on body mass of 70 kg, height of 175 cm, age of 30, and sex. The resulting energy expenditure is 1648.75 per day, which converts to 79.8422 W in watts and 6.89837 megajoules per day. This figure represents a statistical population average for resting energy use, not a personalized medical measurement.

<a id="bmed_bmi"></a>

#### Body mass index (a calculation, not medical advice), `bmed_bmi`

The body mass index BMI = m/h^2 (kg/m^2) and BMI prime (BMI/25) as a calculation from mass and height; no category or health advice is given.

$$
\mathrm{BMI} = \frac{m}{h^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | body mass | kg |
| `h` | in | height | m |
| `BMI` | out | body mass index in kg/m^2 | - |
| `BMI_prime` | out | BMI/25 | - |

**Applies when:** a calculation only: not medical advice; 1 <= m <= 700 kg, 0.3 <= h <= 2.8 m.

```shell
$ biomedical-science solve bmed_bmi "m=70 kg" "h=1.75 m"
BMI = 22.85714286
BMI_prime = 0.9142857143
  by: Body mass index (a calculation, not medical advice)  (BMI = m/h^2)
  m = 70 kg
  h = 1.75 m
  assumes: a calculation only: not medical advice; 1 <= m <= 700 kg, 0.3 <= h <= 2.8 m
```

**What the worked example means:** This calculation determines the Body Mass Index using the formula BMI equals mass divided by height squared. With an input mass of 70 kg and a height of 1.75 m, the resulting index is 22.8571. The secondary output, which is the index divided by twenty five, yields 0.914286. These figures are mathematical results derived from physical measurements and do not constitute medical advice or diagnostic categories.

<a id="bmed_indicator_dilution_cardiac_output"></a>

#### Cardiac output by indicator dilution (Stewart-Hamilton), `bmed_indicator_dilution_cardiac_output`

The cardiac output (volume flow) from the mass of an injected indicator and its sampled first-pass concentration curve, by the Stewart-Hamilton principle with a trapezoidal area.

$$
CO = \frac{m}{\int_0^{\infty} C(t)\,dt}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `m` | in | mass of indicator injected | kg |
| `t` | in | sampling times in s, strictly increasing | - |
| `C` | in | indicator concentrations in mg/L (first pass, recirculation excluded) | - |
| `AUC` | out | area under the concentration curve, mg*s/L | - |
| `CO` | out | cardiac output m / AUC | m^3/s |

**Applies when:** complete mixing, no indicator loss; first-pass curve (recirculation removed by the user); times in s, concentrations in mg/L; linear trapezoids.

```shell
$ biomedical-science solve bmed_indicator_dilution_cardiac_output "m=5 mg" "t=[0, 2, 4, 6, 8, 10, 12, 14, 16]" "C=[0, 0.5, 2.8, 4.1, 3.5, 2.2, 1.1, 0.5, 0.2]"
AUC = 29.6
CO = 0.0001689189189 m^3/s
  by: Cardiac output by indicator dilution (Stewart-Hamilton)  (CO = m / integral of C dt (linear trapezoidal rule))
  m = 5e-06 kg
  assumes: complete mixing, no indicator loss; first-pass curve (recirculation removed by the user); times in s, concentrations in mg/L; linear trapezoids
```

**What the worked example means:** This calculation determines the volume of blood pumped by the heart per second using the Stewart-Hamilton principle. By dividing the known mass of the injected indicator, recorded as 5 mg, by the total area under the concentration-time curve, computed here as 29.6, the resulting flow rate is found to be 0.000168919 m^3/s. This value represents the efficiency of the heart's pumping action based on how quickly the tracer clears from the bloodstream.

<a id="bmed_kelvin_voigt_creep"></a>

#### Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress, `bmed_kelvin_voigt_creep`

The strain of a Kelvin-Voigt (spring parallel to dashpot) tissue model a time t after a constant stress is applied.

$$
\varepsilon(t) = \frac{\sigma}{E}\left(1 - e^{-E t/\eta}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | applied constant stress | Pa |
| `E` | in | spring modulus | Pa |
| `eta` | in | dashpot viscosity (Pa*s) | Pa*s |
| `t` | in | time since loading | s |
| `strain` | out | strain at time t | - |

**Applies when:** linear viscoelasticity; stress applied as a step at t = 0, no initial strain.

```shell
$ biomedical-science solve bmed_kelvin_voigt_creep "sigma=10 kPa" "E=100 kPa" "eta=500 kPa*s" "t=3 s"
strain = 0.04511883639
  by: Kelvin-Voigt viscoelastic tissue model: creep strain under constant stress  (eta de/dt + E e = sigma: e = (sigma/E)(1 - exp(-E t/eta)))
  sigma = 10000 Pa
  E = 100000 Pa
  eta = 500000 Pa*s
  t = 3 s
  assumes: linear viscoelasticity; stress applied as a step at t = 0, no initial strain
```

**What the worked example means:** This calculation determines the viscoelastic creep strain of a tissue modeled as a Kelvin-Voigt element. Given a constant applied stress of 10 kPa, a spring modulus of 100 kPa, and a dashpot viscosity of 500 kPa*s, the model predicts that the strain reaches 0.0451188 after 3 s. This result reflects the gradual deformation characteristic of soft tissues under sustained load, where the viscous component limits the rate of extension.

<a id="bmed_poiseuille_vessel_flow"></a>

#### Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance, `bmed_poiseuille_vessel_flow`

Laminar (Hagen-Poiseuille) flow of a Newtonian fluid through a rigid vessel: volume flow rate, mean velocity, wall shear stress and hydraulic resistance from the pressure drop, radius, length and viscosity.

$$
Q = \frac{\pi r^4 \Delta P}{8 \mu L},\quad \tau_w = \frac{4\mu Q}{\pi r^3},\quad R = \frac{8\mu L}{\pi r^4}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dP` | in | pressure drop along the vessel | Pa |
| `r` | in | vessel radius | m |
| `L` | in | vessel length | m |
| `mu` | in | viscosity of the blood (Newtonian) | Pa*s |
| `Q` | out | volume flow rate | m^3/s |
| `v_mean` | out | mean velocity | m/s |
| `tau_wall` | out | wall shear stress | Pa |
| `R_hyd` | out | hydraulic resistance | Pa*s/m^3 |

**Applies when:** steady laminar flow, Newtonian fluid; rigid straight cylindrical vessel, fully developed flow.

```shell
$ biomedical-science solve bmed_poiseuille_vessel_flow "dP=4 mmHg" "r=2 mm" "L=10 cm" "mu=3.5 mPa*s"
Q = 9.573591608e-06 m^3/s
v_mean = 0.7618422138 m/s
tau_wall = 5.332895497 Pa
R_hyd = 55704230.08 Pa*s/m^3
  by: Poiseuille flow in a blood vessel: flow rate, mean velocity, wall shear stress, resistance  (Q = pi r^4 dP / (8 mu L); v = Q / (pi r^2); tau_w = 4 mu Q / (pi r^3); R = 8 mu L / (pi r^4))
  dP = 533.2895497 Pa
  r = 0.002 m
  L = 0.1 m
  mu = 0.0035 Pa*s
  assumes: steady laminar flow, Newtonian fluid; rigid straight cylindrical vessel, fully developed flow
```

**What the worked example means:** This calculation determines the hemodynamic properties of blood flowing through a straight, rigid cylindrical vessel. Using the input pressure drop of 4 mmHg, radius of 2 mm, length of 10 cm, and viscosity of 3.5 mPa*s, it computes the volume flow rate as 9.57359e-06 m^3/s, mean velocity as 0.761842 m/s, wall shear stress as 5.3329 Pa, and hydraulic resistance as 5.57042e+07 Pa*s/m^3. These values describe how easily fluid moves through the tube and the frictional force exerted on the vessel walls.

<a id="bmed_respiratory_mechanics_fit"></a>

#### Single-compartment respiratory mechanics: resistance and compliance by least squares, `bmed_respiratory_mechanics_fit`

Airway resistance, respiratory compliance and the end-expiratory pressure of the single-compartment equation of motion P = R Q + V/C + P0, fitted by least squares to sampled pressure, flow and volume.

$$
P = R\,\dot V + \frac{V}{C} + P_0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | airway pressures in cmH2O | - |
| `Q` | in | flows in L/s at the same instants | - |
| `V` | in | volumes above end-expiration in L at the same instants | - |
| `R` | out | resistance, cmH2O*s/L | - |
| `C` | out | compliance, L/cmH2O | - |
| `P0` | out | pressure at zero flow and volume, cmH2O | - |

**Applies when:** linear single-compartment model; pressures in cmH2O, flows in L/s, volumes in L; ordinary least squares.

```shell
$ biomedical-science solve bmed_respiratory_mechanics_fit "P=[5.2, 9.8, 13.1, 15.9, 18.6, 20.2]" "Q=[0.5, 0.5, 0.45, 0.4, 0.35, 0.3]" "V=[0, 0.1, 0.19, 0.27, 0.34, 0.4]"
R = 16.71406003
C = 0.02165583305
P0 = -3.169194313
  by: Single-compartment respiratory mechanics: resistance and compliance by least squares  (multiple linear regression of P on Q, V and a constant; C = 1/elastance)
  assumes: linear single-compartment model; pressures in cmH2O, flows in L/s, volumes in L; ordinary least squares
```

**What the worked example means:** This calculation determines the mechanical properties of the respiratory system by fitting measured airway pressures, flows, and volumes to a linear single-compartment model. The resulting resistance value of 16.7141 quantifies the pressure required to overcome airflow friction, while the compliance value of 0.0216558 indicates how easily the lung tissue stretches under pressure. A baseline pressure of -3.16919 accounts for any offset in the measurements when flow and volume are zero.

<a id="bmed_sls_creep"></a>

#### Standard linear solid (Zener) tissue model: creep under constant stress, `bmed_sls_creep`

The creep strain of a standard linear solid (a spring E_inf parallel to a Maxwell arm E1-eta) a time t after a constant stress is applied, and the final strain.

$$
\varepsilon(t) = \frac{\sigma}{E_\infty} - \left(\frac{\sigma}{E_\infty} - \frac{\sigma}{E_\infty + E_1}\right)e^{-t/\tau_c},\ \tau_c = \frac{\eta(E_\infty + E_1)}{E_\infty E_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sigma` | in | applied constant stress | Pa |
| `E_inf` | in | equilibrium (parallel) spring modulus | Pa |
| `E1` | in | spring modulus of the Maxwell arm | Pa |
| `eta` | in | dashpot viscosity of the Maxwell arm | Pa*s |
| `t` | in | time since loading | s |
| `strain` | out | strain at time t | - |
| `strain_inf` | out | final strain sigma/E_inf | - |

**Applies when:** linear viscoelasticity; step stress at t = 0.

```shell
$ biomedical-science solve bmed_sls_creep "sigma=10 kPa" "E_inf=50 kPa" "E1=100 kPa" "eta=200 kPa*s" "t=2 s"
strain = 0.1044624919
strain_inf = 0.2
  by: Standard linear solid (Zener) tissue model: creep under constant stress  (e(t) = sigma/E_inf - (sigma/E_inf - sigma/(E_inf + E1)) exp(-t/tau_c), tau_c = eta (E_inf + E1)/(E_inf E1))
  sigma = 10000 Pa
  E_inf = 50000 Pa
  E1 = 100000 Pa
  eta = 200000 Pa*s
  t = 2 s
  assumes: linear viscoelasticity; step stress at t = 0
```

**What the worked example means:** This calculation determines the time-dependent deformation of tissue modeled as a standard linear solid. Given a constant stress of 10 kPa, the strain evolves from an initial elastic response toward a steady state, reaching 0.104462 at 2 s. The long-term limit is governed solely by the parallel spring stiffness, resulting in a final strain of 0.2. This behavior reflects the balance between immediate elasticity and delayed viscous flow within the material structure.

<a id="bmed_sls_stress_relaxation"></a>

#### Standard linear solid (Zener) tissue model: stress relaxation at constant strain, `bmed_sls_stress_relaxation`

The stress of a standard linear solid a time t after a step strain eps0 is imposed, and the relaxed (equilibrium) stress.

$$
\sigma(t) = \varepsilon_0\left(E_\infty + E_1 e^{-E_1 t/\eta}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `eps0` | in | imposed step strain (a pure number, e.g. 0.05) | - |
| `E_inf` | in | equilibrium spring modulus | Pa |
| `E1` | in | spring modulus of the Maxwell arm | Pa |
| `eta` | in | dashpot viscosity of the Maxwell arm | Pa*s |
| `t` | in | time since the strain step | s |
| `stress` | out | stress at time t | Pa |
| `stress_inf` | out | relaxed stress E_inf eps0 | Pa |

**Applies when:** linear viscoelasticity; step strain at t = 0.

```shell
$ biomedical-science solve bmed_sls_stress_relaxation eps0=0.05 "E_inf=50 kPa" "E1=100 kPa" "eta=200 kPa*s" "t=1 s"
stress = 5532.653299 Pa
stress_inf = 2500 Pa
  by: Standard linear solid (Zener) tissue model: stress relaxation at constant strain  (sigma(t) = eps0 (E_inf + E1 exp(-E1 t/eta)))
  eps0 = 0.05
  E_inf = 50000 Pa
  E1 = 100000 Pa
  eta = 200000 Pa*s
  t = 1 s
  assumes: linear viscoelasticity; step strain at t = 0
```

**What the worked example means:** This calculation determines the stress in a viscoelastic tissue model at a specific time after a sudden strain. Using the imposed step strain of 0.05, the equilibrium modulus 50 kPa, and the Maxwell arm parameters 100 kPa and 200 kPa*s, the resulting stress at time 1 s is computed as 5532.65 Pa. This value represents the internal force per area resisting deformation, which decreases over time from its initial peak towards the relaxed equilibrium stress of 2500 Pa, illustrating how biological tissues lose stiffness under sustained load.

<a id="bmed_elbow_lever_muscle_force"></a>

#### Static lever model of the forearm: muscle force and joint reaction, `bmed_elbow_lever_muscle_force`

For a horizontal forearm held in static equilibrium, the elbow-flexor muscle force and the joint reaction force from the hand load, the forearm weight, their distances from the elbow and the muscle's insertion distance and angle.

$$
F_m = \frac{W d_L + W_a d_a}{d_m \sin\theta},\quad R_x = F_m\cos\theta,\quad R_y = W + W_a - F_m\sin\theta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `W` | in | load held in the hand (weight) | N |
| `d_load` | in | distance of the load from the elbow | m |
| `W_arm` | in | weight of the forearm and hand | N |
| `d_arm` | in | distance of the forearm's centre of mass from the elbow | m |
| `d_muscle` | in | distance of the muscle insertion from the elbow | m |
| `theta` | in | angle between the muscle and the forearm | rad |
| `F_muscle` | out | muscle force | N |
| `R_x` | out | joint reaction along the forearm (towards the hand positive) | N |
| `R_y` | out | joint reaction, upward positive | N |
| `R` | out | magnitude of the joint reaction | N |

**Applies when:** static equilibrium in a vertical plane; forearm horizontal from the elbow, loads vertical; one muscle pulling up and back towards the elbow at angle theta; 0 < theta < 180 deg.

```shell
$ biomedical-science solve bmed_elbow_lever_muscle_force "W=50 N" "d_load=35 cm" "W_arm=15 N" "d_arm=15 cm" "d_muscle=4 cm" "theta=80 deg"
F_muscle = 501.3668896 N
R_x = 87.06144672 N
R_y = -428.75 N
R = 437.5000091 N
  by: Static lever model of the forearm: muscle force and joint reaction  (moments about the elbow: F d_m sin(theta) = W d_load + W_arm d_arm; force balances give R_x = F cos(theta), R_y = W + W_arm - F sin(theta))
  W = 50 N
  d_load = 0.35 m
  W_arm = 15 N
  d_arm = 0.15 m
  d_muscle = 0.04 m
  theta = 1.396263402 rad
  assumes: static equilibrium in a vertical plane; forearm horizontal from the elbow, loads vertical; one muscle pulling up and back towards the elbow at angle theta; 0 < theta < 180 deg
```

**What the worked example means:** This model computes the elbow flexor muscle force and joint reaction forces required to maintain static equilibrium. Given a hand load of 50 N at 35 cm meters and forearm weight of 15 N at 15 cm meters, the muscle acts at 4 cm meters with an angle of 80 deg radians. The resulting muscle force is 501.367 N, which generates horizontal and vertical joint reactions of 87.0614 N and -428.75 N respectively.

<a id="bmed_windkessel_diastolic_decay"></a>

#### Two-element Windkessel: diastolic pressure decay, `bmed_windkessel_diastolic_decay`

The arterial pressure of a two-element Windkessel model during diastole (no inflow), decaying from P0 towards the venous pressure with time constant RC.

$$
P(t) = P_v + (P_0 - P_v)\,e^{-t/(RC)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P0` | in | pressure at the start of diastole | Pa |
| `Pv` | in | venous (outflow) pressure | Pa |
| `R` | in | peripheral resistance | Pa*s/m^3 |
| `C` | in | arterial compliance | m^3/Pa |
| `t` | in | time into diastole | s |
| `P` | out | arterial pressure at time t | Pa |

**Applies when:** no inflow during diastole; linear resistance and compliance.

```shell
$ biomedical-science solve bmed_windkessel_diastolic_decay "P0=120 mmHg" "Pv=5 mmHg" "R=1 mmHg*s/mL" "C=1.5 mL/mmHg" "t=0.5 s"
P = 11652.52341 Pa
  by: Two-element Windkessel: diastolic pressure decay  (C dP/dt = -(P - Pv)/R: P = Pv + (P0 - Pv) exp(-t/(R C)))
  P0 = 15998.68649 Pa
  Pv = 666.6119371 Pa
  R = 133322387.4 Pa*s/m^3
  C = 1.125092364e-08 m^3/Pa
  t = 0.5 s
  assumes: no inflow during diastole; linear resistance and compliance
```

**What the worked example means:** This calculation determines the arterial pressure at a specific moment during diastole using a two-element Windkessel model. It assumes no blood enters the arteries, so pressure falls exponentially from the starting value 120 mmHg toward the constant venous pressure 5 mmHg. The result, 11652.5 Pa, reflects how quickly the system relaxes based on peripheral resistance 1 mmHg*s/mL and arterial compliance 1.5 mL/mmHg. A larger product of resistance and compliance means slower decay, while shorter times or lower initial pressures yield lower current values.

<a id="bmed_windkessel_input_impedance"></a>

#### Windkessel models (two- and three-element): input impedance at a frequency, `bmed_windkessel_input_impedance`

The modulus and phase of the input impedance of a two-element (R, C) or three-element (Zc, R, C) Windkessel model of the arterial tree at a stated frequency.

$$
Z(\omega) = Z_c + \frac{R}{1 + j\omega R C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `model` | in | Windkessel model | - |
| `R` | in | peripheral resistance | Pa*s/m^3 |
| `C` | in | arterial compliance | m^3/Pa |
| `f` | in | frequency | Hz |
| `Zc` | in | characteristic impedance of the aorta (three-element model only) (optional) | Pa*s/m^3 |
| `Z_mod` | out | impedance modulus | Pa*s/m^3 |
| `phase_deg` | out | impedance phase in degrees | - |

**Applies when:** linear model, sinusoidal steady state; Zc only for the three-element model.

```shell
$ biomedical-science solve bmed_windkessel_input_impedance model=two_element "R=1 mmHg*s/mL" "C=1.5 mL/mmHg" "f=1.2 Hz"
Z_mod = 11742475.28 Pa*s/m^3
phase_deg = -84.94708137
  by: Windkessel models (two- and three-element): input impedance at a frequency  (Z = Zc + R/(1 + j 2 pi f R C) (Zc = 0 for two elements); modulus and phase)
  R = 133322387.4 Pa*s/m^3
  C = 1.125092364e-08 m^3/Pa
  f = 1.2 Hz
  model = two_element
  assumes: linear model, sinusoidal steady state; Zc only for the three-element model
```

In the worked example, model = two_element, R = 1 mmHg*s/mL, C = 1.5 mL/mmHg, f = 1.2 Hz go through the equation above and give Z_mod = 11742475.28 Pa*s/m^3, phase_deg = -84.94708137. It applies only when linear model, sinusoidal steady state; Zc only for the three-element model: outside that it says what is wrong instead of returning a number.

### Diagnostic test statistics (6)

<a id="bmed_cohens_kappa"></a>

#### Cohen's kappa for agreement between two raters or two tests, `bmed_cohens_kappa`

Cohen's kappa (chance-corrected agreement) and the observed agreement of two raters or two diagnostic tests classifying the same subjects, from their k x k agreement table.

$$
\kappa = \frac{p_o - p_e}{1 - p_e},\quad p_e = \sum_i \frac{n_{i\cdot}\,n_{\cdot i}}{n^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `counts` | in | agreement table: rows = categories given by rater A, columns = categories given by rater B | - |
| `kappa` | out | Cohen's kappa | - |
| `p_observed` | out | observed proportion of agreement | - |

**Applies when:** the same subjects rated by both; unweighted kappa; the same categories in rows and columns.

```shell
$ biomedical-science solve bmed_cohens_kappa "counts=[[40, 10], [5, 45]]"
kappa = 0.7
p_observed = 0.85
  by: Cohen's kappa for agreement between two raters or two tests  (kappa = (p_o - p_e)/(1 - p_e), p_e = sum of products of row and column margins / n^2)
  assumes: the same subjects rated by both; unweighted kappa; the same categories in rows and columns
```

**What the worked example means:** This calculation determines the level of agreement between two raters or diagnostic tests using their classification counts. It yields a Cohen's kappa value of 0.7, which represents the proportion of agreement corrected for chance, alongside an observed agreement rate of 0.85. A positive kappa indicates that the raters agree more often than would be expected by random guessing alone.

<a id="bmed_dx_youden_cutoff"></a>

#### Optimal diagnostic cut-off by the Youden index, `bmed_dx_youden_cutoff`

The score threshold that maximises the Youden index J = sensitivity + specificity - 1 over all observed cut-offs, with the sensitivity and specificity there.

$$
J(c) = \mathrm{Se}(c) + \mathrm{Sp}(c) - 1,\quad c^* = \arg\max_c J(c)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `scores_pos` | in | scores of the diseased subjects | - |
| `scores_neg` | in | scores of the non-diseased subjects | - |
| `threshold` | out | optimal cut-off (score >= threshold is test-positive) | - |
| `sensitivity` | out | sensitivity at the cut-off | - |
| `specificity` | out | specificity at the cut-off | - |
| `J` | out | Youden index at the cut-off | - |

**Applies when:** higher scores indicate disease; cut-offs at the observed scores; ties in J: the highest threshold (most specific); some cut-off has J > 0.

```shell
$ biomedical-science solve bmed_dx_youden_cutoff "scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4]" "scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1]"
threshold = 0.55
sensitivity = 0.8
specificity = 0.8333333333
J = 0.6333333333
  by: Optimal diagnostic cut-off by the Youden index  (for every observed score as threshold: J = TP/n1 - FP/n0 (exact fractions); the maximum, highest threshold first)
  assumes: higher scores indicate disease; cut-offs at the observed scores; ties in J: the highest threshold (most specific); some cut-off has J > 0
```

**What the worked example means:** This calculation determines the score cut-off that maximizes the Youden index, which combines sensitivity and specificity to identify the best balance between detecting disease and avoiding false alarms. Using the scores of diseased subjects [0.9, 0.8, 0.7, 0.55, 0.4] and non-diseased subjects [0.6, 0.45, 0.4, 0.3, 0.2, 0.1], the method evaluates every observed value as a potential threshold. The resulting optimal cut-off is 0.55, where the test correctly identifies 0.8 of diseased cases and correctly excludes 0.833333 of healthy cases, achieving a Youden index of 0.633333. This specific threshold was chosen because it provides the highest combined discriminative power among all possible cut-offs in the data.

<a id="bmed_dx_predictive_values"></a>

#### Predictive values and likelihood ratios of a diagnostic test, `bmed_dx_predictive_values`

The positive and negative predictive values of a test at a stated prevalence (pre-test probability), and its positive and negative likelihood ratios and diagnostic odds ratio.

$$
PPV = \frac{Se\,p}{Se\,p + (1-Sp)(1-p)},\quad NPV = \frac{Sp(1-p)}{Sp(1-p) + (1-Se)p},\quad LR^{+} = \frac{Se}{1-Sp}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `sensitivity` | in | sensitivity P(positive \| disease), a fraction or % | - |
| `specificity` | in | specificity P(negative \| no disease), a fraction or % | - |
| `prevalence` | in | prevalence (pre-test probability of disease), a fraction or % | - |
| `PPV` | out | positive predictive value P(disease \| positive) | - |
| `NPV` | out | negative predictive value P(no disease \| negative) | - |
| `LR_pos` | out | positive likelihood ratio sens/(1 - spec) | - |
| `LR_neg` | out | negative likelihood ratio (1 - sens)/spec | - |
| `DOR` | out | diagnostic odds ratio LR+/LR- | - |

**Applies when:** 0 < sensitivity, specificity, prevalence < 1; the test's accuracy does not depend on prevalence.

```shell
$ biomedical-science solve bmed_dx_predictive_values sensitivity=0.9 specificity=0.95 prevalence=0.02
PPV = 0.2686567164
NPV = 0.9978563773
LR_pos = 18
LR_neg = 0.1052631579
DOR = 171
  by: Predictive values and likelihood ratios of a diagnostic test  (PPV = Se p / (Se p + (1 - Sp)(1 - p)); NPV = Sp (1 - p) / (Sp (1 - p) + (1 - Se) p); LR+ = Se/(1 - Sp); LR- = (1 - Se)/Sp)
  sensitivity = 0.9
  specificity = 0.95
  prevalence = 0.02
  assumes: 0 < sensitivity, specificity, prevalence < 1; the test's accuracy does not depend on prevalence
```

In the worked example, sensitivity = 0.9, specificity = 0.95, prevalence = 0.02 go through the equation above and give PPV = 0.2686567164, NPV = 0.9978563773, LR_pos = 18, LR_neg = 0.1052631579. It applies only when 0 < sensitivity, specificity, prevalence < 1; the test's accuracy does not depend on prevalence: outside that it says what is wrong instead of returning a number.

<a id="bmed_dx_roc_auc"></a>

#### ROC curve area of a diagnostic score (Mann-Whitney form), `bmed_dx_roc_auc`

The area under the ROC curve of a continuous diagnostic score from the scores of diseased and non-diseased subjects: the probability that a diseased subject scores higher (ties count one half), with the Mann-Whitney U.

$$
AUC = \frac{1}{n_1 n_0}\sum_{i,j}\left[\mathbb{1}(x_i > y_j) + \tfrac12\mathbb{1}(x_i = y_j)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `scores_pos` | in | scores of the diseased (condition-positive) subjects | - |
| `scores_neg` | in | scores of the non-diseased subjects | - |
| `AUC` | out | area under the ROC curve | - |
| `U` | out | Mann-Whitney U of the diseased group | - |

**Applies when:** higher scores indicate disease; ties count one half.

```shell
$ biomedical-science solve bmed_dx_roc_auc "scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4]" "scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1]"
AUC = 0.8833333333
U = 26.5
  by: ROC curve area of a diagnostic score (Mann-Whitney form)  (AUC = U/(n1 n0), U = number of (diseased, non-diseased) pairs with the diseased score higher + half the ties)
  assumes: higher scores indicate disease; ties count one half
```

**What the worked example means:** This calculation determines the diagnostic accuracy by comparing scores between diseased and non-diseased subjects. Using the provided positive scores [0.9, 0.8, 0.7, 0.55, 0.4] and negative scores [0.6, 0.45, 0.4, 0.3, 0.2, 0.1] gives the Mann-Whitney U statistic, which results in 26.5. This value is normalized to produce an Area Under the Curve of 0.883333, representing the probability that a randomly selected diseased individual has a higher score than a randomly selected healthy individual.

<a id="bmed_dx_threshold_sensitivity_specificity"></a>

#### Sensitivity and specificity of a diagnostic score at a cut-off, `bmed_dx_threshold_sensitivity_specificity`

The confusion-matrix counts, sensitivity and specificity of a continuous diagnostic score when scores at or above a threshold are called positive.

$$
\mathrm{Se} = \frac{TP}{TP + FN},\quad \mathrm{Sp} = \frac{TN}{TN + FP}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `scores_pos` | in | scores of the diseased subjects | - |
| `scores_neg` | in | scores of the non-diseased subjects | - |
| `threshold` | in | cut-off: a score >= threshold is test-positive | - |
| `sensitivity` | out | TP/(TP + FN) | - |
| `specificity` | out | TN/(TN + FP) | - |
| `TP` | out | true positives | - |
| `FN` | out | false negatives | - |
| `TN` | out | true negatives | - |
| `FP` | out | false positives | - |

**Applies when:** a score at or above the threshold is positive.

```shell
$ biomedical-science solve bmed_dx_threshold_sensitivity_specificity "scores_pos=[0.9, 0.8, 0.7, 0.55, 0.4]" "scores_neg=[0.6, 0.45, 0.4, 0.3, 0.2, 0.1]" threshold=0.5
sensitivity = 0.8
specificity = 0.8333333333
TP = 4
FN = 1
TN = 5
FP = 1
  by: Sensitivity and specificity of a diagnostic score at a cut-off  (count the 2x2 table at the cut-off; sensitivity = TP/(TP + FN), specificity = TN/(TN + FP))
  threshold = 0.5
  assumes: a score at or above the threshold is positive
```

**What the worked example means:** This calculation determines how well a diagnostic score performs at a specific threshold by constructing a confusion matrix. It counts true positives, false negatives, true negatives, and false positives based on whether scores are at or above the value of 0.5. The resulting sensitivity of 0.8 indicates the proportion of diseased subjects correctly identified, while the specificity of 0.833333 shows the proportion of non-diseased subjects correctly excluded.

<a id="bmed_dx_sensitivity_specificity_ci"></a>

#### Sensitivity and specificity with Wilson score intervals, `bmed_dx_sensitivity_specificity_ci`

Sensitivity and specificity of a diagnostic test from its 2x2 counts, each with a Wilson score confidence interval at the stated level.

$$
\frac{\hat p + \frac{z^2}{2n} \pm z\sqrt{\frac{\hat p(1-\hat p)}{n} + \frac{z^2}{4n^2}}}{1 + \frac{z^2}{n}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TP` | in | true positives | - |
| `FN` | in | false negatives | - |
| `TN` | in | true negatives | - |
| `FP` | in | false positives | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `sensitivity` | out | TP/(TP + FN) | - |
| `sens_lower` | out | lower Wilson limit | - |
| `sens_upper` | out | upper Wilson limit | - |
| `specificity` | out | TN/(TN + FP) | - |
| `spec_lower` | out | lower Wilson limit | - |
| `spec_upper` | out | upper Wilson limit | - |

**Applies when:** independent subjects; Wilson score interval (no continuity correction); level stated.

```shell
$ biomedical-science solve bmed_dx_sensitivity_specificity_ci TP=45 FN=5 TN=90 FP=10 level=0.95
sensitivity = 0.9
sens_lower = 0.7863976856
sens_upper = 0.9565242351
specificity = 0.9
spec_lower = 0.8256343385
spec_upper = 0.9447708629
  by: Sensitivity and specificity with Wilson score intervals  (Wilson interval (p + z^2/2n +- z sqrt(p(1 - p)/n + z^2/4n^2)) / (1 + z^2/n))
  level = 0.95
  assumes: independent subjects; Wilson score interval (no continuity correction); level stated
```

**What the worked example means:** Sensitivity and specificity are calculated from the true positive, false negative, true negative, and false positive counts. The point estimate for sensitivity is 0.9, bounded by the Wilson score interval limits of 0.786398 and 0.956524. Similarly, specificity is estimated at 0.9, with its confidence range extending from 0.825634 to 0.944771. These intervals provide a plausible range for the diagnostic performance metrics based on the observed data.

### Dosage calculations (6)

<a id="bmed_iv_rate_from_dose_rate"></a>

#### Infusion pump rate from a dose rate (arithmetic, not medical advice), `bmed_iv_rate_from_dose_rate`

The pump rate in mL/h that delivers an ordered dose rate (mg/h, or ug/kg/min with the body weight) from a bag holding a stated amount of drug in a stated volume; also the drip rate for a drop factor.

$$
\dot V = \frac{\dot D}{C},\qquad C = \frac{m}{V_{bag}},\qquad \dot D = d\,W
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `drug` | in | amount of drug in the bag | kg |
| `bag_volume` | in | volume of the bag | m^3 |
| `dose_rate` | in | ordered dose rate, e.g. mg/h (give this or dose_ug_per_kg_min) (optional) | kg/s |
| `dose_ug_per_kg_min` | in | ordered dose per kilogram per minute, a number in ug/kg/min (optional) (optional) | - |
| `W` | in | body weight (with dose_ug_per_kg_min) (optional) | kg |
| `drop_factor` | in | drop factor of the giving set, drops per mL (optional) (optional) | - |
| `C` | out | concentration in the bag (1 kg/m^3 = 1 mg/mL) | kg/m^3 |
| `dose_rate` | out | dose rate delivered | kg/s |
| `dose_mg_h` | out | dose rate in mg/h | - |
| `flow` | out | pump rate | m^3/s |
| `flow_mL_h` | out | pump rate in mL/h | - |
| `drops_per_min` | out | drip rate in drops per minute (when the drop factor is given) | - |

**Applies when:** arithmetic only: not medical advice; drug > 0, bag volume > 0; exactly one of a dose rate or a dose per kilogram per minute (with 0.3 kg <= W <= 400 kg).

```shell
$ biomedical-science solve bmed_iv_rate_from_dose_rate "drug=400 mg" "bag_volume=250 mL" dose_ug_per_kg_min=5 "W=80 kg"
C = 1.6 kg/m^3
dose_rate = 6.666666667e-09 kg/s
dose_mg_h = 24
flow = 4.166666667e-09 m^3/s
flow_mL_h = 15
  by: Infusion pump rate from a dose rate (arithmetic, not medical advice)  (C = drug/bag volume; dose rate = (ug/kg/min) x W x 60 / 1000 mg/h; mL/h = (mg/h) / (mg/mL))
  drug = 0.0004 kg
  bag_volume = 0.00025 m^3
  dose_ug_per_kg_min = 5
  W = 80 kg
  assumes: arithmetic only: not medical advice; drug > 0, bag volume > 0; exactly one of a dose rate or a dose per kilogram per minute (with 0.3 kg <= W <= 400 kg)
```

In the worked example, drug = 400 mg, bag_volume = 250 mL, dose_ug_per_kg_min = 5, W = 80 kg go through the equation above and give C = 1.6 kg/m^3, dose_rate = 6.666666667e-09 kg/s, dose_mg_h = 24, flow = 4.166666667e-09 m^3/s. It applies only when arithmetic only: not medical advice; drug > 0, bag volume > 0; exactly one of a dose rate or a dose per kilogram per minute (with 0.3 kg <= W <= 400 kg): outside that it says what is wrong instead of returning a number.

<a id="bmed_iv_flow_rate"></a>

#### IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice), `bmed_iv_flow_rate`

The flow rate of an infusion in mL/h from the volume and the time (or the infusion time from the volume and the rate), and the drip rate in drops per minute from the set's drop factor.

$$
\dot V = \frac{V}{T},\qquad \text{gtt/min} = \frac{V\,[\mathrm{mL}]}{T\,[\mathrm{min}]}\times\text{drop factor}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | volume to infuse | m^3 |
| `T` | in | infusion time (give T or rate) (optional) | s |
| `rate` | in | flow rate (give rate or T) (optional) | m^3/s |
| `drop_factor` | in | drop factor of the giving set, drops per mL (optional) (optional) | - |
| `flow` | out | flow rate | m^3/s |
| `flow_mL_h` | out | flow rate in mL/h | - |
| `duration` | out | infusion time | s |
| `duration_h` | out | infusion time in h | - |
| `duration_min` | out | infusion time in min | - |
| `drops_per_min` | out | drip rate in drops per minute (when the drop factor is given) | - |

**Applies when:** arithmetic only: not medical advice; V > 0; exactly one of T > 0 or rate > 0; drop factor a whole number of drops per mL, 1 to 100.

```shell
$ biomedical-science solve bmed_iv_flow_rate "V=1000 mL" "T=8 h" drop_factor=15
flow = 3.472222222e-08 m^3/s
flow_mL_h = 125
duration = 28800 s
duration_h = 8
duration_min = 480
drops_per_min = 31.25
  by: IV flow rate: mL/h, drops per minute and infusion time (arithmetic, not medical advice)  (mL/h = volume / hours; time = volume / rate; drops/min = (mL/h / 60) x drop factor)
  V = 0.001 m^3
  T = 28800 s
  assumes: arithmetic only: not medical advice; V > 0; exactly one of T > 0 or rate > 0; drop factor a whole number of drops per mL, 1 to 100
```

In the worked example, V = 1000 mL, T = 8 h, drop_factor = 15 go through the equation above and give flow = 3.472222222e-08 m^3/s, flow_mL_h = 125, duration = 28800 s, duration_h = 8. It applies only when arithmetic only: not medical advice; V > 0; exactly one of T > 0 or rate > 0; drop factor a whole number of drops per mL, 1 to 100: outside that it says what is wrong instead of returning a number.

<a id="bmed_reconstitution_concentration"></a>

#### Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice), `bmed_reconstitution_concentration`

The concentration of a powder drug after reconstitution, C = drug/(diluent + displacement volume), and the volume to give for an ordered dose, D/C.

$$
C = \frac{m}{V_{dil} + V_{disp}},\qquad V = \frac{D}{C}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `powder` | in | drug amount in the vial | kg |
| `diluent` | in | diluent volume added | m^3 |
| `displacement` | in | displacement volume of the powder (optional, default 0) (optional) | m^3 |
| `D` | in | ordered dose (optional) (optional) | kg |
| `V_total` | out | final volume after reconstitution | m^3 |
| `V_total_mL` | out | final volume in mL | - |
| `C` | out | concentration after reconstitution (1 kg/m^3 = 1 mg/mL) | kg/m^3 |
| `C_mg_mL` | out | concentration in mg/mL | - |
| `V_give` | out | volume to give for the ordered dose (when given) | m^3 |
| `V_give_mL` | out | volume to give in mL | - |

**Applies when:** arithmetic only: not medical advice; drug > 0, diluent > 0, displacement >= 0; ordered dose <= drug in the vial.

```shell
$ biomedical-science solve bmed_reconstitution_concentration "powder=1 g" "diluent=3.4 mL" "displacement=0.6 mL" "D=500 mg"
V_total = 4e-06 m^3
V_total_mL = 4
C = 250 kg/m^3
C_mg_mL = 250
V_give = 2e-06 m^3
V_give_mL = 2
  by: Reconstituted powder: concentration and volume to give (dosage arithmetic, not medical advice)  (final volume = diluent + displacement; C = drug / final volume; volume to give = D / C)
  powder = 0.001 kg
  diluent = 3.4e-06 m^3
  displacement = 6e-07 m^3
  D = 0.0005 kg
  assumes: arithmetic only: not medical advice; drug > 0, diluent > 0, displacement >= 0; ordered dose <= drug in the vial
```

In the worked example, powder = 1 g, diluent = 3.4 mL, displacement = 0.6 mL, D = 500 mg go through the equation above and give V_total = 4e-06 m^3, V_total_mL = 4, C = 250 kg/m^3, C_mg_mL = 250. It applies only when arithmetic only: not medical advice; drug > 0, diluent > 0, displacement >= 0; ordered dose <= drug in the vial: outside that it says what is wrong instead of returning a number.

<a id="bmed_dose_tablets"></a>

#### Tablets to give: desired over have (dosage arithmetic, not medical advice), `bmed_dose_tablets`

The number of tablets (or capsules) for an ordered dose from the strength of one tablet, tablets = D/H; arithmetic only (whether a tablet may be split is not judged).

$$
n = \frac{D}{H}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | ordered (desired) dose | kg |
| `H` | in | strength of one tablet or capsule (have) | kg |
| `tablets` | out | number of tablets to give, D/H | - |

**Applies when:** arithmetic only: not medical advice; D > 0, H > 0.

```shell
$ biomedical-science solve bmed_dose_tablets "D=500 mg" "H=250 mg"
tablets = 2
  by: Tablets to give: desired over have (dosage arithmetic, not medical advice)  (tablets = D/H (desired over have))
  D = 0.0005 kg
  H = 0.00025 kg
  assumes: arithmetic only: not medical advice; D > 0, H > 0
```

In the worked example, D = 500 mg, H = 250 mg go through the equation above and give tablets = 2. It applies only when arithmetic only: not medical advice; D > 0, H > 0: outside that it says what is wrong instead of returning a number.

<a id="bmed_dose_liquid_volume"></a>

#### Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice), `bmed_dose_liquid_volume`

The volume of a liquid or injection stock to give for an ordered dose, V = D/H x Q, where the stock holds H in a volume Q; also the stock concentration H/Q.

$$
V = \frac{D}{H}\,Q
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | ordered (desired) dose | kg |
| `H` | in | amount of drug in the stock volume (have) | kg |
| `Q` | in | stock volume that holds H | m^3 |
| `V` | out | volume to give | m^3 |
| `V_mL` | out | volume to give in mL | - |
| `C` | out | stock concentration H/Q (1 kg/m^3 = 1 mg/mL) | kg/m^3 |

**Applies when:** arithmetic only: not medical advice; D > 0, H > 0, Q > 0.

```shell
$ biomedical-science solve bmed_dose_liquid_volume "D=250 mg" "H=125 mg" "Q=5 mL"
V = 1e-05 m^3
V_mL = 10
C = 25 kg/m^3
  by: Volume of a liquid to give: desired over have (dosage arithmetic, not medical advice)  (V = (D/H) x Q (desired over have times quantity))
  D = 0.00025 kg
  H = 0.000125 kg
  Q = 5e-06 m^3
  assumes: arithmetic only: not medical advice; D > 0, H > 0, Q > 0
```

In the worked example, D = 250 mg, H = 125 mg, Q = 5 mL go through the equation above and give V = 1e-05 m^3, V_mL = 10, C = 25 kg/m^3. It applies only when arithmetic only: not medical advice; D > 0, H > 0, Q > 0: outside that it says what is wrong instead of returning a number.

<a id="bmed_dose_weight_based"></a>

#### Weight-based dose (dosage arithmetic, not medical advice), `bmed_dose_weight_based`

The dose for a body weight from an order in mg/kg, per dose or per day, with the daily total or the single dose when the number of doses per day is given; arithmetic only, no dose is judged safe.

$$
D = d_{kg}\,W,\qquad D_{dose} = \frac{d_{kg,day}\,W}{n}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `dose_mg_per_kg` | in | ordered dose per kilogram of body weight, a number in mg/kg | - |
| `W` | in | body weight | kg |
| `basis` | in | whether the mg/kg figure is per dose or per day | - |
| `doses_per_day` | in | number of doses per day (needed for a per-day order) (optional) | - |
| `dose` | out | single dose | kg |
| `dose_mg` | out | single dose in mg | - |
| `daily_dose` | out | total dose per day (when the doses per day are known) | kg |
| `daily_dose_mg` | out | total dose per day in mg | - |

**Applies when:** arithmetic only: not medical advice; 0 < dose per kilogram <= 1000 mg/kg; 0.3 kg <= body weight <= 400 kg; a per-day order needs the number of doses per day (1 to 24).

```shell
$ biomedical-science solve bmed_dose_weight_based dose_mg_per_kg=15 "W=20 kg" basis=per_dose doses_per_day=4
dose_mg = 300
daily_dose_mg = 1200
dose = 0.0003 kg
daily_dose = 0.0012 kg
  by: Weight-based dose (dosage arithmetic, not medical advice)  (dose = (mg/kg) x weight in kg; per day: single dose = daily dose / doses per day)
  dose_mg_per_kg = 15
  W = 20 kg
  basis = per_dose
  assumes: arithmetic only: not medical advice; 0 < dose per kilogram <= 1000 mg/kg; 0.3 kg <= body weight <= 400 kg; a per-day order needs the number of doses per day (1 to 24)
```

In the worked example, dose_mg_per_kg = 15, W = 20 kg, basis = per_dose, doses_per_day = 4 go through the equation above and give dose_mg = 300, daily_dose_mg = 1200, dose = 0.0003 kg, daily_dose = 0.0012 kg. It applies only when arithmetic only: not medical advice; 0 < dose per kilogram <= 1000 mg/kg; 0.3 kg <= body weight <= 400 kg; a per-day order needs the number of doses per day (1 to 24): outside that it says what is wrong instead of returning a number.

### Epidemic models (9)

<a id="bmed_epi_herd_immunity"></a>

#### Herd-immunity threshold and critical vaccination coverage, `bmed_epi_herd_immunity`

The fraction of a population that must be immune to stop sustained spread, 1 - 1/R0, and, for a vaccine of stated efficacy, the vaccination coverage that achieves it (and whether it is reachable).

$$
H = 1 - \frac{1}{R_0},\quad V_c = \frac{1 - 1/R_0}{E}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | basic reproduction number | - |
| `efficacy` | in | vaccine efficacy against infection, a fraction or % (optional) (optional) | - |
| `threshold` | out | herd-immunity threshold 1 - 1/R0 (immune fraction) | - |
| `threshold_percent` | out | threshold in % | - |
| `coverage` | out | critical vaccination coverage (1 - 1/R0)/E (when the efficacy is given) | - |
| `coverage_percent` | out | critical coverage in % | - |
| `reachable` | out | reachable (coverage <= 100 %) or not reachable with this vaccine | - |

**Applies when:** 1 < R0 <= 100; 0 < efficacy <= 1; homogeneous mixing, lasting all-or-nothing immunity.

```shell
$ biomedical-science solve bmed_epi_herd_immunity R0=4 efficacy=0.9
threshold = 0.75
threshold_percent = 75
coverage = 0.8333333333
coverage_percent = 83.33333333
reachable = reachable
  by: Herd-immunity threshold and critical vaccination coverage  (H = 1 - 1/R0; V_c = H / E)
  R0 = 4
  efficacy = 0.9
  assumes: 1 < R0 <= 100; 0 < efficacy <= 1; homogeneous mixing, lasting all-or-nothing immunity
```

In the worked example, R0 = 4, efficacy = 0.9 go through the equation above and give threshold = 0.75, threshold_percent = 75, coverage = 0.8333333333, coverage_percent = 83.33333333. It applies only when 1 < R0 <= 100; 0 < efficacy <= 1; homogeneous mixing, lasting all-or-nothing immunity: outside that it says what is wrong instead of returning a number.

<a id="bmed_epi_reed_frost_final_size"></a>

#### Reed-Frost chain-binomial epidemic: distribution and mean of the final size, `bmed_epi_reed_frost_final_size`

The probability distribution of the number of susceptibles eventually infected in a Reed-Frost chain-binomial epidemic in a small closed group, its mean and the chance of no spread.

$$
S_{t+1} \sim \mathrm{Bin}\!\left(S_t, (1 - p)^{I_t}\right),\quad I_{t+1} = S_t - S_{t+1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `n` | in | initial number of susceptibles (1 to 30) | - |
| `m` | in | initial number of infectives (1 to 30) | - |
| `p` | in | probability that one infective infects one given susceptible in a generation (0 < p < 1) | - |
| `mean_final` | out | expected number of susceptibles infected | - |
| `P_no_spread` | out | probability that no susceptible is infected | - |
| `distribution` | out | probabilities of 0, 1, ..., n susceptibles infected | - |

**Applies when:** discrete generations, each infective infectious for one generation; independent contacts.

```shell
$ biomedical-science solve bmed_epi_reed_frost_final_size n=4 m=1 p=0.2
mean_final = 1.270137242
P_no_spread = 0.4096
distribution = [0.4096,0.2097152,0.163577856,0.1351614464,0.0819454976]
  by: Reed-Frost chain-binomial epidemic: distribution and mean of the final size  (Markov chain over (susceptibles, infectives): a susceptible escapes i infectives with probability (1 - p)^i; new infections are binomial; probabilities summed over all paths)
  p = 0.2
  assumes: discrete generations, each infective infectious for one generation; independent contacts
```

**What the worked example means:** This calculation determines the final size distribution for a Reed-Frost chain-binomial epidemic model. Starting with 4 susceptible and 1 infective individuals, it computes the probability that each susceptible escapes infection from all current infectives as one minus the transmission probability raised to the power of the number of infectives. The result shows an average of 1.27014 total infections and a chance of zero spread at 0.4096, illustrating how stochasticity affects small closed populations.

<a id="bmed_epi_r_from_growth_rate"></a>

#### Reproduction number from an epidemic growth rate (gamma-distributed generation interval), `bmed_epi_r_from_growth_rate`

The reproduction number implied by an observed exponential growth rate when the generation interval follows a gamma distribution of stated mean and standard deviation (Wallinga-Lipsitch).

$$
R = \left(1 + r\,\frac{s^2}{\mu}\right)^{\mu^2/s^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `r` | in | exponential growth rate (negative for a decline) | 1/s |
| `Tg_mean` | in | mean generation interval | s |
| `Tg_sd` | in | standard deviation of the generation interval | s |
| `R` | out | reproduction number | - |

**Applies when:** exponential growth or decline; gamma-distributed generation interval.

```shell
$ biomedical-science solve bmed_epi_r_from_growth_rate "r=0.1 1/day" "Tg_mean=5 day" "Tg_sd=2 day"
R = 1.617701811
  by: Reproduction number from an epidemic growth rate (gamma-distributed generation interval)  (R = 1 / M(-r), M the moment generating function of the gamma distribution: R = (1 + r theta)^k, k = mean^2/sd^2, theta = sd^2/mean)
  r = 1.157407407e-06 1/s
  Tg_mean = 432000 s
  Tg_sd = 172800 s
  assumes: exponential growth or decline; gamma-distributed generation interval
```

**What the worked example means:** This calculation determines the effective reproduction number, representing the average number of secondary infections produced by each infected individual in a partially immune population. It derives this value from an observed exponential growth rate of 0.1 1/day per unit time, combined with the characteristics of the generation interval, which has a mean of 5 day and a standard deviation of 2 day. The resulting reproduction number of 1.6177 indicates whether the epidemic is expanding or contracting; values above one suggest growth, while values below one indicate decline.

<a id="bmed_epi_seir_final_size"></a>

#### SEIR epidemic model: final size, `bmed_epi_seir_final_size`

The final susceptible fraction and the attack fraction of an SEIR epidemic (the latent stage does not change the final size) from R0 and the initial susceptible, exposed and infectious fractions.

$$
\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + e_0 + i_0 - s_\infty)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | basic reproduction number beta/gamma | - |
| `s0` | in | initial susceptible fraction | - |
| `e0` | in | initial exposed (latent) fraction | - |
| `i0` | in | initial infectious fraction | - |
| `s_inf` | out | susceptible fraction at the end | - |
| `attack` | out | fraction infected during the epidemic, s0 - s_inf | - |

**Applies when:** closed population, homogeneous mixing; exponentially distributed latent and infectious periods; e0 + i0 > 0.

```shell
$ biomedical-science solve bmed_epi_seir_final_size R0=3 s0=0.98 e0=0.01 i0=0.01
s_inf = 0.05807797447
attack = 0.9219220255
  by: SEIR epidemic model: final size  (final-size relation ln(s0/s_inf) = R0 (s0 + e0 + i0 - s_inf), solved by bisection)
  R0 = 3
  s0 = 0.98
  e0 = 0.01
  i0 = 0.01
  assumes: closed population, homogeneous mixing; exponentially distributed latent and infectious periods; e0 + i0 > 0
```

**What the worked example means:** This calculation determines the final state of an SEIR epidemic by solving the final-size relation using bisection. Given a basic reproduction number of 3, initial susceptible fraction of 0.98, exposed fraction of 0.01, and infectious fraction of 0.01, the model computes that the epidemic ends with a susceptible fraction of 0.058078. Consequently, the total fraction of the population infected during the outbreak, known as the attack rate, is 0.921922. This result highlights that the latent stage does not change the final size compared to a standard SIR model when R0 remains constant.

<a id="bmed_epi_seir_growth_rate"></a>

#### SEIR epidemic model: initial exponential growth rate from R0, `bmed_epi_seir_growth_rate`

The early exponential growth (or decline) rate of an SEIR epidemic in a fully susceptible population, from R0, the rate of leaving the latent stage and the recovery rate.

$$
r = \frac{-(\sigma + \gamma) + \sqrt{(\sigma + \gamma)^2 + 4\sigma\gamma(R_0 - 1)}}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | basic reproduction number beta/gamma | - |
| `sigma` | in | rate of leaving the latent stage (1/latent period) | 1/s |
| `gamma` | in | recovery rate (1/infectious period) | 1/s |
| `r` | out | initial growth rate (negative when R0 < 1) | 1/s |

**Applies when:** fully susceptible population at the start; exponential latent and infectious periods.

```shell
$ biomedical-science solve bmed_epi_seir_growth_rate R0=2.5 "sigma=0.2 1/day" "gamma=0.1 1/day"
r = 9.158424161e-07 1/s
  by: SEIR epidemic model: initial exponential growth rate from R0  (largest root of r^2 + (sigma + gamma) r - sigma gamma (R0 - 1) = 0)
  R0 = 2.5
  sigma = 2.314814815e-06 1/s
  gamma = 1.157407407e-06 1/s
  assumes: fully susceptible population at the start; exponential latent and infectious periods
```

**What the worked example means:** This calculation determines the initial exponential growth rate of an SEIR epidemic in a fully susceptible population. Using the provided basic reproduction number, latent stage departure rate, and recovery rate, the model computes that cases will grow or decline at a rate of 9.15842e-07 1/s. This value indicates how quickly new infections accumulate during the early phase before resources or behavioral changes limit spread.

<a id="bmed_epi_seir_peak"></a>

#### SEIR epidemic model: peak prevalence and the time of the peak, `bmed_epi_seir_peak`

The largest infectious fraction of an SEIR epidemic and the time at which it occurs, from the transmission, incubation and recovery rates and the initial fractions.

$$
\dot s = -\beta s i,\ \dot e = \beta s i - \sigma e,\ \dot i = \sigma e - \gamma i;\quad t_{peak}:\ \sigma e = \gamma i
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | transmission rate | 1/s |
| `sigma` | in | rate of leaving the latent stage (1/latent period) | 1/s |
| `gamma` | in | recovery rate | 1/s |
| `s0` | in | initial susceptible fraction | - |
| `e0` | in | initial exposed (latent) fraction | - |
| `i0` | in | initial infectious fraction | - |
| `t_peak` | out | time of the peak of the infectious fraction | s |
| `i_max` | out | peak infectious fraction | - |
| `s_at_peak` | out | susceptible fraction at the peak | - |
| `e_at_peak` | out | exposed fraction at the peak | - |

**Applies when:** closed population, homogeneous mixing, frequency-dependent transmission; R0 s0 > 1, e0 + i0 > 0; the peak is the first maximum of i after t = 0 and exceeds i0.

```shell
$ biomedical-science solve bmed_epi_seir_peak "beta=0.5 1/day" "sigma=0.2 1/day" "gamma=0.1 1/day" s0=0.99 e0=0.008 i0=0.002
t_peak = 3177221.498 s
i_max = 0.306250282
s_at_peak = 0.122175821
e_at_peak = 0.153125141
  by: SEIR epidemic model: peak prevalence and the time of the peak  (Taylor-series integration (order 26, steps set by the series' tail) of the SEIR system to the first sign change of sigma e - gamma i, the root located on the series)
  beta = 5.787037037e-06 1/s
  sigma = 2.314814815e-06 1/s
  gamma = 1.157407407e-06 1/s
  s0 = 0.99
  e0 = 0.008
  i0 = 0.002
  assumes: closed population, homogeneous mixing, frequency-dependent transmission; R0 s0 > 1, e0 + i0 > 0; the peak is the first maximum of i after t = 0 and exceeds i0
```

**What the worked example means:** This calculation determines the time when an SEIR epidemic reaches its highest point and the corresponding peak level of infection. By integrating the differential equations for susceptible, exposed, and infectious populations using a Taylor series method, it identifies the moment when the rate of new recoveries equals the rate of individuals leaving the latent stage. For the given parameters, the peak occurs at 3.17722e+06 s, where the infectious fraction reaches 0.30625. This result indicates the severity and timing of the outbreak's most intense phase.

<a id="bmed_epi_sir_final_size"></a>

#### SIR epidemic model: final size (final susceptible fraction and attack fraction), `bmed_epi_sir_final_size`

The fraction still susceptible when an SIR epidemic ends and the fraction infected during it, from R0 and the initial susceptible and infected fractions (the final-size relation).

$$
\ln\frac{s_0}{s_\infty} = R_0\,(s_0 + i_0 - s_\infty)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | basic reproduction number beta/gamma | - |
| `s0` | in | initial susceptible fraction | - |
| `i0` | in | initial infected fraction (> 0) | - |
| `s_inf` | out | susceptible fraction at the end | - |
| `attack` | out | fraction infected during the epidemic, s0 - s_inf | - |

**Applies when:** closed population, no births or deaths, homogeneous mixing; fractions of the population; the rest of the population (1 - s0 - i0) is immune.

```shell
$ biomedical-science solve bmed_epi_sir_final_size R0=2.5 s0=0.99 i0=0.01
s_inf = 0.1058941943
attack = 0.8841058057
  by: SIR epidemic model: final size (final susceptible fraction and attack fraction)  (final-size relation ln(s0/s_inf) = R0 (s0 + i0 - s_inf), solved by bisection on (0, min(s0, 1/R0)])
  R0 = 2.5
  s0 = 0.99
  i0 = 0.01
  assumes: closed population, no births or deaths, homogeneous mixing; fractions of the population; the rest of the population (1 - s0 - i0) is immune
```

**What the worked example means:** This calculation determines the end state of an SIR epidemic using the final-size relation. Given a basic reproduction number of 2.5, an initial susceptible fraction of 0.99, and an initial infected fraction of 0.01, the model solves for the proportion of the population that remains susceptible, which is 0.105894. Consequently, the total fraction of the population that becomes infected during the outbreak, known as the attack rate, is calculated as 0.884106. This result indicates how much of the susceptible pool was depleted by the time the infection burned out.

<a id="bmed_epi_sir_peak"></a>

#### SIR epidemic model: peak prevalence and the time of the peak, `bmed_epi_sir_peak`

The largest infected fraction of an SIR epidemic and the time at which it occurs, from the transmission and recovery rates and the initial fractions.

$$
i_{max} = s_0 + i_0 - \frac{1 + \ln(R_0 s_0)}{R_0},\quad t_{peak} = \int_{1/R_0}^{s_0}\frac{ds}{\beta s\,i(s)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | transmission rate | 1/s |
| `gamma` | in | recovery rate | 1/s |
| `s0` | in | initial susceptible fraction | - |
| `i0` | in | initial infected fraction (> 0) | - |
| `i_max` | out | peak infected fraction | - |
| `t_peak` | out | time of the peak | s |

**Applies when:** closed population, homogeneous mixing, frequency-dependent transmission; R0 s0 > 1.

```shell
$ biomedical-science solve bmed_epi_sir_peak "beta=0.3 1/day" "gamma=0.1 1/day" s0=0.999 i0=0.001
i_max = 0.3007960706
t_peak = 3313898.583 s
  by: SIR epidemic model: peak prevalence and the time of the peak  (i_max = s0 + i0 - (1 + ln(R0 s0))/R0 from the first integral of the SIR system; t_peak = integral over s from 1/R0 to s0 of ds / (beta s i(s)) (mpmath quadrature))
  beta = 3.472222222e-06 1/s
  gamma = 1.157407407e-06 1/s
  s0 = 0.999
  i0 = 0.001
  assumes: closed population, homogeneous mixing, frequency-dependent transmission; R0 s0 > 1
```

**What the worked example means:** This calculation determines the highest proportion of the population that becomes infected during an SIR epidemic and when this peak occurs. Using the initial susceptible fraction 0.999, initial infected fraction 0.001, transmission rate 0.3 1/day, and recovery rate 0.1 1/day, the model computes a peak prevalence of 0.300796. The time to reach this peak is calculated as 3.3139e+06 s. These values indicate the severity and timing of the outbreak's maximum spread under the assumption of homogeneous mixing and constant parameters.

<a id="bmed_epi_sis_prevalence"></a>

#### SIS epidemic model: prevalence at a time and the endemic level, `bmed_epi_sis_prevalence`

The infected fraction of an SIS model (no immunity after recovery) at time t and the endemic equilibrium it approaches.

$$
i(t) = \frac{i_0\,e^{rt}}{1 + i_0\,\beta\,(e^{rt} - 1)/r},\quad r = \beta - \gamma,\quad i^* = 1 - \gamma/\beta
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `beta` | in | transmission rate | 1/s |
| `gamma` | in | recovery rate | 1/s |
| `i0` | in | initial infected fraction | - |
| `t` | in | time | s |
| `i` | out | infected fraction at time t | - |
| `i_star` | out | endemic infected fraction (0 when beta <= gamma) | - |

**Applies when:** closed population, homogeneous mixing; recovered individuals are susceptible again.

```shell
$ biomedical-science solve bmed_epi_sis_prevalence "beta=0.4 1/day" "gamma=0.1 1/day" i0=0.01 "t=10 day"
i = 0.1601112475
i_star = 0.75
  by: SIS epidemic model: prevalence at a time and the endemic level  (logistic solution of di/dt = beta i (1 - i) - gamma i: i = i0 e^{rt} / (1 + i0 beta (e^{rt} - 1)/r), r = beta - gamma)
  beta = 4.62962963e-06 1/s
  gamma = 1.157407407e-06 1/s
  i0 = 0.01
  t = 864000 s
  assumes: closed population, homogeneous mixing; recovered individuals are susceptible again
```

**What the worked example means:** This calculation determines the proportion of infected individuals in an SIS epidemic model at a specific time point, given the transmission rate 0.4 1/day, recovery rate 0.1 1/day, initial infected fraction 0.01, and elapsed time 10 day. The result, 0.160111, represents the current prevalence of the disease within the closed population. Additionally, the endemic equilibrium 0.75 indicates the stable fraction of infected individuals that the system approaches over the long term when transmission exceeds recovery.

### Epidemiological measures of association (8)

<a id="bmed_nnt_ci"></a>

#### Absolute risk reduction and number needed to treat with Wald intervals, `bmed_nnt_ci`

The absolute risk reduction (control risk minus treated risk) of a two-arm trial with its Wald confidence interval, and the number needed to treat with the interval obtained by inverting it.

$$
ARR = p_c - p_t,\quad NNT = \frac{1}{ARR},\quad \mathrm{SE} = \sqrt{\tfrac{p_t(1-p_t)}{n_t} + \tfrac{p_c(1-p_c)}{n_c}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `events_t` | in | events in the treated group | - |
| `n_t` | in | size of the treated group | - |
| `events_c` | in | events in the control group | - |
| `n_c` | in | size of the control group | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `ARR` | out | absolute risk reduction | - |
| `ARR_lower` | out | lower limit | - |
| `ARR_upper` | out | upper limit | - |
| `NNT` | out | number needed to treat 1/ARR | - |
| `NNT_lower` | out | 1/ARR_upper | - |
| `NNT_upper` | out | 1/ARR_lower | - |

**Applies when:** unpooled Wald interval for the difference of proportions; the interval excludes zero (a finite NNT interval); level stated; NNT not rounded.

```shell
$ biomedical-science solve bmed_nnt_ci events_t=15 n_t=100 events_c=30 n_c=100 level=0.95
ARR = 0.15
ARR_lower = 0.03613638193
ARR_upper = 0.2638636181
NNT = 6.666666667
NNT_lower = 3.789836611
NNT_upper = 27.67294197
  by: Absolute risk reduction and number needed to treat with Wald intervals  (ARR = pc - pt; SE = sqrt(pt(1 - pt)/nt + pc(1 - pc)/nc); NNT = 1/ARR with limits 1/upper, 1/lower)
  level = 0.95
  assumes: unpooled Wald interval for the difference of proportions; the interval excludes zero (a finite NNT interval); level stated; NNT not rounded
```

**What the worked example means:** This calculation determines the absolute difference in event rates between the control and treated groups, defined as the control risk minus the treated risk. The resulting absolute risk reduction is 0.15, bounded by an unpooled Wald confidence interval ranging from 0.0361364 to 0.263864. This metric translates into the number of patients required to be treated to prevent one additional adverse outcome, calculated as the reciprocal of the absolute risk reduction. The estimated number needed to treat is 6.66667, with corresponding limits derived from the inverted confidence interval.

<a id="bmed_epi_basic_rates"></a>

#### Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality, `bmed_epi_basic_rates`

The incidence proportion (attack rate) and, when given, the incidence rate per person-year, the point prevalence, the case-fatality rate and the mortality, from case counts in one population.

$$
IP = \frac{\text{new cases}}{N},\quad P = \frac{\text{existing cases}}{N},\quad CFR = \frac{\text{deaths}}{\text{cases}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cases` | in | new cases during the period | - |
| `population` | in | population at risk at the start of the period | - |
| `prevalent` | in | existing cases at a point in time (optional, for the prevalence) (optional) | - |
| `deaths` | in | deaths from the disease among its cases (optional) (optional) | - |
| `period` | in | length of the observation period (optional, for rates per person-year) (optional) | s |
| `incidence_proportion` | out | incidence proportion (attack rate) = new cases / population | - |
| `incidence_per_100000` | out | incidence proportion per 100,000 | - |
| `incidence_rate` | out | incidence rate per person and unit time (when the period is given) | Hz |
| `incidence_per_100000_person_years` | out | incidence rate per 100,000 person-years (when the period is given) | - |
| `prevalence` | out | point prevalence = existing cases / population (when given) | - |
| `prevalence_per_100000` | out | prevalence per 100,000 | - |
| `case_fatality` | out | case-fatality rate = deaths / cases (when deaths are given) | - |
| `mortality_per_100000` | out | disease mortality per 100,000 population over the period | - |
| `mortality_per_100000_person_years` | out | mortality rate per 100,000 person-years (deaths and period given) | - |

**Applies when:** 0 <= cases <= population; existing cases <= population; deaths <= cases; one closed population, constant over the period.

```shell
$ biomedical-science solve bmed_epi_basic_rates cases=150 population=10000 prevalent=400 deaths=6 "period=1 yr"
incidence_proportion = 0.015
incidence_per_100000 = 1500
incidence_rate = 4.753213172e-10 Hz
incidence_per_100000_person_years = 1500
prevalence = 0.04
prevalence_per_100000 = 4000
case_fatality = 0.04
mortality_per_100000 = 60
mortality_per_100000_person_years = 60
  by: Basic epidemiological rates: incidence, attack rate, prevalence, case fatality and mortality  (incidence proportion = cases/N; incidence rate = cases/(N T); prevalence = existing cases/N; case fatality = deaths/cases; mortality = deaths/N)
  period = 31557600 s
  assumes: 0 <= cases <= population; existing cases <= population; deaths <= cases; one closed population, constant over the period
```

In the worked example, cases = 150, population = 10000, prevalent = 400, deaths = 6, period = 1 yr go through the equation above and give incidence_proportion = 0.015, incidence_per_100000 = 1500, incidence_rate = 4.753213172e-10 Hz, incidence_per_100000_person_years = 1500. It applies only when 0 <= cases <= population; existing cases <= population; deaths <= cases; one closed population, constant over the period: outside that it says what is wrong instead of returning a number.

<a id="bmed_incidence_rate_ratio_ci"></a>

#### Incidence rate ratio of two person-time groups with log-Wald interval, `bmed_incidence_rate_ratio_ci`

The ratio of the incidence rates (events per person-time) of two groups with the log-scale Wald confidence interval at the stated level.

$$
IRR = \frac{e_1/T_1}{e_2/T_2},\quad \mathrm{SE}(\ln IRR) = \sqrt{\tfrac1{e_1} + \tfrac1{e_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `events1` | in | events in group 1 | - |
| `time1` | in | person-time of group 1 (the same unit for both groups) | - |
| `events2` | in | events in group 2 (reference) | - |
| `time2` | in | person-time of group 2 | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `IRR` | out | incidence rate ratio group 1 / group 2 | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |

**Applies when:** Poisson counts; log-Wald interval; the same person-time unit for both groups; level stated.

```shell
$ biomedical-science solve bmed_incidence_rate_ratio_ci events1=30 time1=1000 events2=15 time2=1200 level=0.95
IRR = 2.4
lower = 1.291331304
upper = 4.460512946
  by: Incidence rate ratio of two person-time groups with log-Wald interval  (IRR = (e1/T1)/(e2/T2); SE(ln IRR) = sqrt(1/e1 + 1/e2))
  time1 = 1000
  time2 = 1200
  level = 0.95
  assumes: Poisson counts; log-Wald interval; the same person-time unit for both groups; level stated
```

**What the worked example means:** This calculation determines the ratio of event rates between two groups by dividing the incidence rate of group one by that of group two. The result, 2.4, indicates how much more or less likely events are in the first group compared to the reference group. Precision is estimated using a log-scale Wald method, producing an interval bounded by 1.29133 and 4.46051 at the specified level.

<a id="bmed_mantel_haenszel_odds_ratio"></a>

#### Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval, `bmed_mantel_haenszel_odds_ratio`

The Mantel-Haenszel odds ratio pooled over strata of 2x2 tables (adjusting for a confounder), with the Robins-Breslow-Greenland confidence interval at the stated level.

$$
OR_{MH} = \frac{\sum_i a_i d_i / n_i}{\sum_i b_i c_i / n_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tables` | in | one row [a, b, c, d] per stratum: exposed cases, exposed non-cases, unexposed cases, unexposed non-cases | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `OR_MH` | out | Mantel-Haenszel pooled odds ratio | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |

**Applies when:** a common odds ratio across strata; Robins-Breslow-Greenland variance; level stated.

```shell
$ biomedical-science solve bmed_mantel_haenszel_odds_ratio "tables=[[10, 20, 5, 30], [8, 12, 6, 20]]" level=0.95
OR_MH = 2.607758621
lower = 1.084053438
upper = 6.273127121
  by: Mantel-Haenszel pooled odds ratio over strata with Robins-Breslow-Greenland interval  (OR_MH = sum(a d / n) / sum(b c / n); variance of ln OR_MH by Robins, Breslow and Greenland)
  level = 0.95
  assumes: a common odds ratio across strata; Robins-Breslow-Greenland variance; level stated
```

**What the worked example means:** This calculation determines the Mantel-Haenszel pooled odds ratio by combining counts from multiple strata, yielding a value of 2.60776. The associated confidence interval ranges from 1.08405 to 6.27313, reflecting the precision of this estimate based on the input tables and the specified confidence level. This method adjusts for confounding variables by assuming a common effect across all strata.

<a id="bmed_odds_ratio_ci"></a>

#### Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval, `bmed_odds_ratio_ci`

The odds ratio of an exposure-outcome 2x2 table (case-control or cohort) with the Woolf logit confidence interval at the stated level.

$$
OR = \frac{ad}{bc},\quad \mathrm{SE}(\ln OR) = \sqrt{\tfrac1a + \tfrac1b + \tfrac1c + \tfrac1d}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | exposed with the outcome | - |
| `b` | in | exposed without the outcome | - |
| `c` | in | unexposed with the outcome | - |
| `d` | in | unexposed without the outcome | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `OR` | out | odds ratio ad/(bc) | - |
| `lower` | out | lower confidence limit | - |
| `upper` | out | upper confidence limit | - |

**Applies when:** no zero cell (no continuity correction); Woolf interval exp(ln OR +- z SE); level stated.

```shell
$ biomedical-science solve bmed_odds_ratio_ci a=20 b=80 c=10 d=90 level=0.95
OR = 2.25
lower = 0.9942948923
upper = 5.091547829
  by: Odds ratio of a 2x2 exposure-outcome table with Woolf (logit) interval  (OR = ad/(bc); SE(ln OR) = sqrt(1/a + 1/b + 1/c + 1/d))
  level = 0.95
  assumes: no zero cell (no continuity correction); Woolf interval exp(ln OR +- z SE); level stated
```

**What the worked example means:** This calculation determines the odds ratio for an exposure-outcome association using a two-by-two table, where exposed cases are 20, exposed non-cases are 80, unexposed cases are 10, and unexposed non-cases are 90. The resulting odds ratio of 2.25 indicates how much more likely the outcome is in the exposed group compared to the unexposed group. A confidence level of 0.95 yields an interval between 0.994295 and 5.09155, providing a range of plausible values for the true population odds ratio based on the Woolf logit method.

<a id="bmed_relative_risk_ci"></a>

#### Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval, `bmed_relative_risk_ci`

The risk ratio of exposed to unexposed in a cohort 2x2 table with the log-scale (Katz) confidence interval at the stated level.

$$
RR = \frac{a/(a+b)}{c/(c+d)},\quad \mathrm{SE}(\ln RR) = \sqrt{\tfrac1a - \tfrac1{a+b} + \tfrac1c - \tfrac1{c+d}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | exposed with the outcome | - |
| `b` | in | exposed without the outcome | - |
| `c` | in | unexposed with the outcome | - |
| `d` | in | unexposed without the outcome | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `RR` | out | risk ratio [a/(a + b)]/[c/(c + d)] | - |
| `lower` | out | lower confidence limit | - |
| `upper` | out | upper confidence limit | - |

**Applies when:** cohort (risks estimable); no zero cell; log interval exp(ln RR +- z SE); level stated.

```shell
$ biomedical-science solve bmed_relative_risk_ci a=20 b=80 c=10 d=90 level=0.95
RR = 2
lower = 0.9865632067
upper = 4.054479199
  by: Relative risk (risk ratio) of a cohort 2x2 table with log (Katz) interval  (RR = [a/(a + b)]/[c/(c + d)]; SE(ln RR) = sqrt(1/a - 1/(a + b) + 1/c - 1/(c + d)))
  level = 0.95
  assumes: cohort (risks estimable); no zero cell; log interval exp(ln RR +- z SE); level stated
```

**What the worked example means:** This calculation determines the ratio of outcome risks between exposed and unexposed groups in a cohort study. Using counts for exposed cases, exposed non-cases, unexposed cases, and unexposed non-cases, it computes the point estimate 2. The uncertainty is quantified using the log scale method, yielding limits at 0.986563 and 4.05448 based on the specified confidence level.

<a id="bmed_smr_exact_ci"></a>

#### Standardized mortality (incidence) ratio with exact Poisson interval, `bmed_smr_exact_ci`

The standardized mortality or incidence ratio observed/expected with the exact (Garwood) Poisson confidence interval at the stated level.

$$
\mathrm{SMR} = \frac{O}{E},\quad P(X \ge O;\lambda_L) = \tfrac{\alpha}{2},\ P(X \le O;\lambda_U) = \tfrac{\alpha}{2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `observed` | in | observed number of deaths or cases (0 to 1000) | - |
| `expected` | in | expected number from reference rates | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `SMR` | out | observed / expected | - |
| `lower` | out | lower exact limit | - |
| `upper` | out | upper exact limit | - |

**Applies when:** observed count Poisson; expected count treated as known; exact (Garwood) limits; level stated.

```shell
$ biomedical-science solve bmed_smr_exact_ci observed=25 expected=18.4 level=0.95
SMR = 1.358695652
lower = 0.8792761874
upper = 2.00570281
  by: Standardized mortality (incidence) ratio with exact Poisson interval  (limits lambda with P(X >= O; lambda) = alpha/2 and P(X <= O; lambda) = alpha/2 (bisection on the Poisson sums), divided by E)
  expected = 18.4
  level = 0.95
  assumes: observed count Poisson; expected count treated as known; exact (Garwood) limits; level stated
```

**What the worked example means:** This calculation determines the standardized mortality or incidence ratio by dividing the observed number of events, 25, by the expected number derived from reference rates, 18.4. The result, 1.3587, indicates whether the event rate in the study population is higher or lower than expected. To account for statistical uncertainty, especially in small samples, exact Poisson confidence limits are computed at the specified level of 0.95. These limits, 0.879276 and 2.0057, provide a range within which the true ratio likely falls, assuming the observed count follows a Poisson distribution and the expected count is known precisely.

<a id="bmed_vaccine_efficacy"></a>

#### Vaccine efficacy from attack rates: `bmed_vaccine_efficacy`

Vaccine efficacy VE = (ARU - ARV)/ARU = 1 - RR from the attack rates of the vaccinated and the unvaccinated, with the risk ratio, the absolute risk reduction and the number needed to vaccinate.

$$
VE = \frac{ARU - ARV}{ARU} = 1 - \frac{ARV}{ARU}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `cases_v` | in | cases among the vaccinated | - |
| `n_v` | in | number vaccinated | - |
| `cases_u` | in | cases among the unvaccinated | - |
| `n_u` | in | number unvaccinated | - |
| `ARV` | out | attack rate in the vaccinated | - |
| `ARU` | out | attack rate in the unvaccinated | - |
| `RR` | out | risk ratio ARV/ARU | - |
| `VE` | out | vaccine efficacy 1 - ARV/ARU (fraction) | - |
| `VE_percent` | out | vaccine efficacy in % | - |
| `ARR` | out | absolute risk reduction ARU - ARV | - |
| `NNV` | out | number needed to vaccinate to prevent one case, 1/ARR (when ARR > 0) | - |

**Applies when:** cases <= group size; at least one unvaccinated case; comparable groups, equal follow-up.

```shell
$ biomedical-science solve bmed_vaccine_efficacy cases_v=20 n_v=1000 cases_u=100 n_u=1000
ARV = 0.02
ARU = 0.1
RR = 0.2
VE = 0.8
VE_percent = 80
ARR = 0.08
NNV = 12.5
  by: Vaccine efficacy from attack rates  (ARV = cases_v/n_v; ARU = cases_u/n_u; VE = (ARU - ARV)/ARU = 1 - ARV/ARU)
  assumes: cases <= group size; at least one unvaccinated case; comparable groups, equal follow-up
```

In the worked example, cases_v = 20, n_v = 1000, cases_u = 100, n_u = 1000 go through the equation above and give ARV = 0.02, ARU = 0.1, RR = 0.2, VE = 0.8. It applies only when cases <= group size; at least one unvaccinated case; comparable groups, equal follow-up: outside that it says what is wrong instead of returning a number.

### Medical-imaging physics (14)

<a id="bmed_hounsfield_units"></a>

#### CT number in Hounsfield units: `bmed_hounsfield_units`

The CT number HU = 1000 (mu - mu_water)/(mu_water - mu_air) of a material from its linear attenuation coefficient and those of water and air (air 0 by default, then 1000 (mu - mu_water)/mu_water).

$$
HU = 1000\,\frac{\mu - \mu_{water}}{\mu_{water} - \mu_{air}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `mu` | in | linear attenuation coefficient of the material | 1/m |
| `mu_water` | in | linear attenuation coefficient of water | 1/m |
| `mu_air` | in | linear attenuation coefficient of air (default 0) (optional) | 1/m |
| `HU` | out | CT number in Hounsfield units | - |
| `mu_rel` | out | mu/mu_water | - |

**Applies when:** coefficients at the same effective energy; 0 <= mu_air < mu_water.

```shell
$ biomedical-science solve bmed_hounsfield_units "mu=0.38 1/cm" "mu_water=0.19 1/cm"
HU = 1000
mu_rel = 2
  by: CT number in Hounsfield units  (HU = 1000 (mu - mu_water)/(mu_water - mu_air))
  mu = 38 1/m
  mu_water = 19 1/m
  assumes: coefficients at the same effective energy; 0 <= mu_air < mu_water
```

**What the worked example means:** The computed CT number is 1000 Hounsfield units, derived from the material's attenuation coefficient of 0.38 1/cm per meter.

<a id="bmed_mri_adc_fit"></a>

#### Diffusion MRI: apparent diffusion coefficient from signals at several b-values, `bmed_mri_adc_fit`

The apparent diffusion coefficient and the b = 0 signal from diffusion-weighted signals at two or more b-values, by a least-squares fit of ln S against b (mono-exponential decay).

$$
\ln S = \ln S_0 - b\cdot ADC
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `b` | in | b-values in s/mm^2 | - |
| `S` | in | signals at those b-values (any one unit) | - |
| `ADC` | out | apparent diffusion coefficient, mm^2/s | - |
| `S0` | out | fitted signal at b = 0 (unit of S) | - |

**Applies when:** mono-exponential decay S = S0 exp(-b ADC); unweighted least squares on ln S; b in s/mm^2.

```shell
$ biomedical-science solve bmed_mri_adc_fit "b=[0, 1000]" "S=[1200, 420]"
ADC = 0.001049822125
S0 = 1200
  by: Diffusion MRI: apparent diffusion coefficient from signals at several b-values  (least-squares line ln S = ln S0 - ADC b)
  assumes: mono-exponential decay S = S0 exp(-b ADC); unweighted least squares on ln S; b in s/mm^2
```

**What the worked example means:** This calculation determines the apparent diffusion coefficient and the baseline signal intensity from diffusion-weighted MRI data. By performing a least-squares linear fit of the natural logarithm of the signal against the b-value, the method extracts the slope and intercept of the mono-exponential decay curve. The resulting apparent diffusion coefficient, reported as 0.00104982, quantifies the magnitude of water molecule diffusion within the tissue, while the fitted baseline signal, 1200, represents the theoretical signal intensity at zero diffusion weighting.

<a id="bmed_gaussian_psf_mtf"></a>

#### Imaging resolution: MTF of a Gaussian point-spread function, `bmed_gaussian_psf_mtf`

The modulation transfer function at a spatial frequency of an imaging system with a Gaussian point-spread function of stated FWHM, and the frequency at which the MTF falls to 50 %.

$$
\mathrm{MTF}(f) = e^{-2\pi^2\sigma^2 f^2},\quad \sigma = \frac{\mathrm{FWHM}}{2\sqrt{2\ln 2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `fwhm` | in | full width at half maximum of the point-spread function | m |
| `f` | in | spatial frequency (cycles per length) | 1/m |
| `MTF` | out | modulation transfer at f | - |
| `f50` | out | spatial frequency where the MTF is 0.5 | 1/m |

**Applies when:** Gaussian, shift-invariant point-spread function; one dimension (a line-spread function).

```shell
$ biomedical-science solve bmed_gaussian_psf_mtf "fwhm=0.5 mm" "f=1 1/mm"
MTF = 0.4106858004
f50 = 882.5424006 1/m
  by: Imaging resolution: MTF of a Gaussian point-spread function  (sigma = FWHM/(2 sqrt(2 ln 2)); MTF = exp(-2 pi^2 sigma^2 f^2); f50 = sqrt(ln 2 / 2)/(pi sigma))
  fwhm = 0.0005 m
  f = 1000 1/m
  assumes: Gaussian, shift-invariant point-spread function; one dimension (a line-spread function)
```

**What the worked example means:** This calculation determines the Modulation Transfer Function for an imaging system characterized by a Gaussian point-spread function. Given a full width at half maximum of 0.5 mm and a spatial frequency of 1 1/mm, the resulting modulation transfer value is 0.410686. This dimensionless ratio indicates how well the system preserves contrast at that specific frequency. Additionally, the spatial frequency where the contrast drops to fifty percent is calculated as 882.542 1/m, providing a standard metric for comparing resolution limits across different systems.

<a id="bmed_larmor_frequency"></a>

#### Larmor (precession) frequency in a magnetic field, `bmed_larmor_frequency`

The Larmor frequency f = (gamma/2 pi) B of nuclear spins in a field B: protons by default (42.577478461 MHz/T, CODATA 2022), or another nucleus with its stated gamma/2 pi in MHz/T.

$$
f = \frac{\gamma}{2\pi}\,B_0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `B` | in | magnetic flux density (T) | T |
| `gamma_bar` | in | reduced gyromagnetic ratio gamma/2 pi in MHz/T (default: the proton) (optional) | - |
| `f` | out | Larmor frequency | Hz |
| `f_MHz` | out | Larmor frequency in MHz | - |
| `omega` | out | angular Larmor frequency | rad/s |

**Applies when:** B > 0; bare nucleus (no chemical shielding).

```shell
$ biomedical-science solve bmed_larmor_frequency "B=1.5 T"
f = 63866217.69 Hz
f_MHz = 63.86621769
omega = 401283280.6 rad/s
  by: Larmor (precession) frequency in a magnetic field  (f = (gamma/2 pi) B)
  B = 1.5 T
  assumes: B > 0; bare nucleus (no chemical shielding)
```

**What the worked example means:** The Larmor frequency is 6.38662e+07 Hz for a field of 1.5 T.

<a id="bmed_mri_ernst_angle"></a>

#### MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal), `bmed_mri_ernst_angle`

The Ernst angle alpha_E = arccos(exp(-TR/T1)) that maximises the steady-state signal of a spoiled gradient-echo sequence, and that largest signal as a fraction of M0 (T2* decay excluded).

$$
\alpha_E = \arccos\left(e^{-TR/T_1}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TR` | in | repetition time | s |
| `T1` | in | longitudinal relaxation time | s |
| `alpha` | out | Ernst angle | rad |
| `alpha_deg` | out | Ernst angle in degrees | - |
| `S_rel` | out | signal at the Ernst angle, fraction of M0 (no T2* decay) | - |

**Applies when:** ideal spoiling, steady state; TR, T1 > 0.

```shell
$ biomedical-science solve bmed_mri_ernst_angle "TR=10 ms" "T1=1000 ms"
alpha = 0.1411857722 rad
alpha_deg = 8.089348877
S_rel = 0.07071038349
  by: MRI Ernst angle (the flip angle of the largest spoiled gradient-echo signal)  (cos(alpha_E) = exp(-TR/T1); S = sin(alpha)(1 - E1)/(1 - E1 cos alpha))
  TR = 0.01 s
  T1 = 1 s
  assumes: ideal spoiling, steady state; TR, T1 > 0
```

**What the worked example means:** This calculation determines the optimal flip angle for maximum signal in a spoiled gradient-echo sequence. Using a repetition time of 10 ms and a longitudinal relaxation time of 1000 ms, the resulting Ernst angle is 8.08935 degrees. At this specific angle, the steady-state signal reaches its peak value, expressed as 0.0707104 of the equilibrium magnetization M0.

<a id="bmed_mri_inversion_recovery"></a>

#### MRI inversion recovery: longitudinal magnetisation at TI and the null point, `bmed_mri_inversion_recovery`

The longitudinal magnetisation (fraction of M0, signed) at inversion time TI in a repeated inversion-recovery sequence with repetition time TR, and the inversion time that nulls the tissue.

$$
\frac{M_z(TI)}{M_0} = 1 - 2e^{-TI/T_1} + e^{-TR/T_1},\quad TI_{null} = T_1\ln\frac{2}{1 + e^{-TR/T_1}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T1` | in | longitudinal relaxation time | s |
| `TR` | in | repetition time | s |
| `TI` | in | inversion time | s |
| `Mz_ratio` | out | Mz/M0 at TI (negative before the null) | - |
| `TI_null` | out | inversion time of zero signal | s |

**Applies when:** ideal 180 degree inversion and 90 degree readout at TI; readout leaves Mz = 0; steady state, TI < TR.

```shell
$ biomedical-science solve bmed_mri_inversion_recovery "T1=250 ms" "TR=2000 ms" "TI=150 ms"
Mz_ratio = -0.09728780956
TI_null = 0.1732029435 s
  by: MRI inversion recovery: longitudinal magnetisation at TI and the null point  (Mz(TI)/M0 = 1 - 2 exp(-TI/T1) + exp(-TR/T1); null at TI = T1 ln(2/(1 + exp(-TR/T1))))
  T1 = 0.25 s
  TR = 2 s
  TI = 0.15 s
  assumes: ideal 180 degree inversion and 90 degree readout at TI; readout leaves Mz = 0; steady state, TI < TR
```

**What the worked example means:** This calculation determines the signed fraction of equilibrium longitudinal magnetization present at the inversion time, which is -0.0972878, based on the tissue relaxation time of 250 ms, repetition time of 2000 ms, and inversion time of 150 ms. It also identifies the specific inversion time required to suppress this signal completely, calculated as 0.173203 s. A negative value for the magnetization fraction indicates that the net magnetization vector has not yet crossed zero along the longitudinal axis, while the null point represents the optimal timing for fluid attenuation or tissue suppression in clinical imaging.

<a id="bmed_mri_t2_fit"></a>

#### MRI multi-echo T2 estimate (log-linear fit), `bmed_mri_t2_fit`

The transverse relaxation time T2 and the extrapolated signal at TE = 0 from signals at several echo times, by a least-squares fit of ln S against TE.

$$
\ln S = \ln S_0 - \frac{TE}{T_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `TE` | in | echo times in ms | - |
| `S` | in | signals at those echo times (any one unit) | - |
| `T2` | out | transverse relaxation time, ms | - |
| `S0` | out | fitted signal at TE = 0 (unit of S) | - |

**Applies when:** mono-exponential decay S = S0 exp(-TE/T2); unweighted least squares on ln S; TE in ms.

```shell
$ biomedical-science solve bmed_mri_t2_fit "TE=[10, 30, 50, 80, 120]" "S=[905, 740, 605, 445, 300]"
T2 = 99.445918
S0 = 999.8103891
  by: MRI multi-echo T2 estimate (log-linear fit)  (least-squares line ln S = ln S0 - TE/T2)
  assumes: mono-exponential decay S = S0 exp(-TE/T2); unweighted least squares on ln S; TE in ms
```

**What the worked example means:** This calculation determines the transverse relaxation time T2 and the extrapolated signal at zero echo time S0 from a set of MRI signals measured at various echo times. By taking the natural logarithm of each signal value, the exponential decay relationship becomes linear. A least-squares fit is then applied to these log-transformed values against their corresponding echo times. The slope of this line yields the negative reciprocal of T2, while the intercept provides the estimate for S0. For example, if the measured echo times are [10, 30, 50, 80, 120] and the observed signals are [905, 740, 605, 445, 300], the resulting parameters are 99.4459 for the relaxation time and 999.81 for the baseline signal.

<a id="bmed_mri_spin_echo_signal"></a>

#### MRI spin-echo signal from T1, T2, TR and TE (simplified model), `bmed_mri_spin_echo_signal`

The steady-state spin-echo signal S = M0 (1 - exp(-TR/T1)) exp(-TE/T2) of a tissue with relaxation times T1 and T2, for repetition time TR and echo time TE.

$$
S = M_0\left(1 - e^{-TR/T_1}\right)e^{-TE/T_2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M0` | in | equilibrium magnetisation (relative proton density) | - |
| `TR` | in | repetition time | s |
| `TE` | in | echo time | s |
| `T1` | in | longitudinal relaxation time | s |
| `T2` | in | transverse relaxation time | s |
| `S` | out | signal in the units of M0 | - |

**Applies when:** 90 degree excitations every TR, transverse magnetisation spoiled before each; ideal refocusing without effect on Mz (TE << TR); TE < TR.

```shell
$ biomedical-science solve bmed_mri_spin_echo_signal M0=1 "TR=500 ms" "TE=15 ms" "T1=900 ms" "T2=100 ms"
S = 0.3668738307
  by: MRI spin-echo signal from T1, T2, TR and TE (simplified model)  (S = M0 (1 - exp(-TR/T1)) exp(-TE/T2))
  M0 = 1
  TR = 0.5 s
  TE = 0.015 s
  T1 = 0.9 s
  T2 = 0.1 s
  assumes: 90 degree excitations every TR, transverse magnetisation spoiled before each; ideal refocusing without effect on Mz (TE << TR); TE < TR
```

**What the worked example means:** This calculation determines the steady-state spin-echo MRI signal intensity for a specific tissue. It uses the formula involving exponential decay terms based on the provided repetition time, echo time, longitudinal relaxation time, transverse relaxation time, and relative proton density. The resulting signal value of 0.366874 indicates how bright this tissue appears in an image under these specific timing conditions, reflecting the balance between T1 recovery and T2 decay.

<a id="bmed_mri_spoiled_gradient_echo"></a>

#### MRI spoiled gradient-echo signal (Ernst equation), `bmed_mri_spoiled_gradient_echo`

The steady-state signal of a spoiled gradient-echo sequence with flip angle alpha, repetition time TR and echo time TE for a tissue with T1 and T2*.

$$
S = M_0 \sin\alpha\,\frac{1 - E_1}{1 - E_1\cos\alpha}\,e^{-TE/T_2^*},\quad E_1 = e^{-TR/T_1}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `M0` | in | equilibrium magnetisation (relative proton density) | - |
| `TR` | in | repetition time | s |
| `TE` | in | echo time | s |
| `T1` | in | longitudinal relaxation time | s |
| `T2star` | in | effective transverse relaxation time T2* | s |
| `alpha` | in | flip angle | rad |
| `S` | out | signal in the units of M0 | - |

**Applies when:** ideal spoiling of transverse magnetisation before each pulse; steady state; 0 < alpha <= 90 deg, TE < TR.

```shell
$ biomedical-science solve bmed_mri_spoiled_gradient_echo M0=1 "TR=10 ms" "TE=3 ms" "T1=1000 ms" "T2star=40 ms" "alpha=8 deg"
S = 0.06559702498
  by: MRI spoiled gradient-echo signal (Ernst equation)  (S = M0 sin(alpha) (1 - E1)/(1 - cos(alpha) E1) exp(-TE/T2*), E1 = exp(-TR/T1))
  M0 = 1
  TR = 0.01 s
  TE = 0.003 s
  T1 = 1 s
  T2star = 0.04 s
  alpha = 0.1396263402 rad
  assumes: ideal spoiling of transverse magnetisation before each pulse; steady state; 0 < alpha <= 90 deg, TE < TR
```

**What the worked example means:** This calculation determines the MRI signal intensity using the Ernst equation. It combines the equilibrium magnetization 1, longitudinal relaxation time 1000 ms, and effective transverse relaxation time 40 ms with sequence parameters including repetition time 10 ms, echo time 3 ms, and flip angle 8 deg. The resulting signal value 0.065597 represents the steady-state response under ideal spoiling conditions, indicating how bright the tissue appears in the image relative to its proton density.

<a id="bmed_snr_averaging"></a>

#### Signal-to-noise ratio gained by signal averaging, `bmed_snr_averaging`

The signal-to-noise ratio after averaging N acquisitions with independent noise, SNR_N = SNR_1 sqrt(N), or the smallest number of averages that reaches a target SNR.

$$
SNR_N = SNR_1\sqrt{N}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `SNR1` | in | signal-to-noise ratio of one acquisition | - |
| `N` | in | number of averages (give N or SNR_target) (optional) | - |
| `SNR_target` | in | target SNR (give N or SNR_target) (optional) | - |
| `SNR` | out | SNR after averaging | - |
| `gain` | out | SNR gain sqrt(N) | - |
| `N` | out | number of averages | - |

**Applies when:** independent, zero-mean noise of equal variance in every acquisition; an identical signal.

```shell
$ biomedical-science solve bmed_snr_averaging SNR1=10 N=16
SNR = 40
gain = 4
N = 16
  by: Signal-to-noise ratio gained by signal averaging  (SNR_N = SNR_1 sqrt(N); N needed = ceil((SNR_target/SNR_1)^2))
  SNR1 = 10
  assumes: independent, zero-mean noise of equal variance in every acquisition; an identical signal
```

**What the worked example means:** This calculation determines the signal-to-noise ratio achieved by averaging multiple acquisitions. Starting with a single acquisition signal-to-noise ratio of 10, the method applies the square root relationship to compute the final signal-to-noise ratio of 40. The result indicates that the noise reduction efficiency scales with the square root of the number of averages, yielding a total gain of 4. This metric helps practitioners balance scan duration against image quality requirements.

<a id="bmed_ultrasound_interface"></a>

#### Ultrasound reflection and transmission at a tissue interface (normal incidence), `bmed_ultrasound_interface`

The intensity reflection and transmission coefficients and the pressure amplitude coefficients of an ultrasound wave meeting a flat interface between two media at normal incidence.

$$
R = \left(\frac{Z_2 - Z_1}{Z_2 + Z_1}\right)^2,\quad T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z1` | in | acoustic impedance of the first medium | kg/(m^2*s) |
| `Z2` | in | acoustic impedance of the second medium | kg/(m^2*s) |
| `R` | out | intensity reflection coefficient | - |
| `T` | out | intensity transmission coefficient | - |
| `r_p` | out | pressure reflection coefficient | - |
| `t_p` | out | pressure transmission coefficient | - |

**Applies when:** normal incidence on a flat interface; lossless media, plane waves.

```shell
$ biomedical-science solve bmed_ultrasound_interface "Z1=1630000 kg/(m^2*s)" "Z2=7800000 kg/(m^2*s)"
R = 0.4281016903
T = 0.5718983097
r_p = 0.6542948038
t_p = 1.654294804
  by: Ultrasound reflection and transmission at a tissue interface (normal incidence)  (r = (Z2 - Z1)/(Z2 + Z1), t = 2 Z2/(Z1 + Z2); R = r^2, T = 4 Z1 Z2/(Z1 + Z2)^2)
  Z1 = 1630000 kg/(m^2*s)
  Z2 = 7800000 kg/(m^2*s)
  assumes: normal incidence on a flat interface; lossless media, plane waves
```

**What the worked example means:** This calculation determines the proportion of ultrasound energy that reflects off or transmits through a boundary between two tissues at normal incidence. Using the acoustic impedance of the first medium, 1.63e+06 kg/(m^2*s), and the second medium, 7.8e+06 kg/(m^2*s), gives the intensity reflection coefficient, 0.428102, and the intensity transmission coefficient, 0.571898. These values reveal the relative brightness of tissue boundaries in an image; a higher reflection coefficient typically corresponds to a brighter echo on the scan.

<a id="bmed_ultrasound_matching_layer"></a>

#### Ultrasound transmission through a matching layer (normal incidence), `bmed_ultrasound_matching_layer`

The intensity transmission coefficient from a transducer (impedance Z1) through a matching layer (impedance Zm, thickness d, sound speed c_m) into tissue (Z2) at frequency f, and the layer's phase thickness.

$$
T = \frac{4 Z_1 Z_2}{(Z_1 + Z_2)^2\cos^2 kd + \left(Z_m + \frac{Z_1 Z_2}{Z_m}\right)^2\sin^2 kd}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Z1` | in | impedance of the source medium (transducer) | kg/(m^2*s) |
| `Zm` | in | impedance of the matching layer | kg/(m^2*s) |
| `Z2` | in | impedance of the load medium (tissue) | kg/(m^2*s) |
| `d` | in | layer thickness | m |
| `c_m` | in | sound speed in the layer | m/s |
| `f` | in | frequency | Hz |
| `T` | out | intensity transmission coefficient | - |
| `kd` | out | phase thickness of the layer (rad) | - |

**Applies when:** normal incidence, plane waves, lossless layers; semi-infinite source and load media.

```shell
$ biomedical-science solve bmed_ultrasound_matching_layer "Z1=30000000 kg/(m^2*s)" "Zm=6900000 kg/(m^2*s)" "Z2=1600000 kg/(m^2*s)" "d=0.15 mm" "c_m=2400 m/s" "f=4 MHz"
T = 0.9999833612
kd = 1.570796327
  by: Ultrasound transmission through a matching layer (normal incidence)  (T = 4 Z1 Z2 / [(Z1 + Z2)^2 cos^2(kd) + (Zm + Z1 Z2/Zm)^2 sin^2(kd)], k = 2 pi f / c_m)
  Z1 = 30000000 kg/(m^2*s)
  Zm = 6900000 kg/(m^2*s)
  Z2 = 1600000 kg/(m^2*s)
  d = 0.00015 m
  c_m = 2400 m/s
  f = 4000000 Hz
  assumes: normal incidence, plane waves, lossless layers; semi-infinite source and load media
```

**What the worked example means:** This calculation determines how much ultrasound energy passes from a transducer into tissue through an intermediate matching layer. Using the source impedance 3e+07 kg/(m^2*s), layer impedance 6.9e+06 kg/(m^2*s), and tissue impedance 1.6e+06 kg/(m^2*s), along with the layer thickness 0.15 mm, sound speed 2400 m/s, and frequency 4 MHz, it computes the transmission efficiency. The result, 0.999983, represents the fraction of incident intensity that enters the tissue, while 1.5708 indicates the electrical length of the layer in radians, which influences constructive or destructive interference of reflections.

<a id="bmed_ultrasound_wavelength"></a>

#### Ultrasound wavelength in tissue: `bmed_ultrasound_wavelength`

The wavelength lambda = c/f of ultrasound of frequency f in a medium with speed of sound c (1540 m/s, the soft-tissue convention, by default), with the period and wavenumber.

$$
\lambda = \frac{c}{f}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `f` | in | ultrasound frequency | Hz |
| `c` | in | speed of sound (default 1540 m/s) (optional) | m/s |
| `wavelength` | out | wavelength | m |
| `wavelength_mm` | out | wavelength in mm | - |
| `period` | out | period 1/f | s |
| `k` | out | wavenumber 2 pi/lambda | 1/m |

**Applies when:** f, c > 0; c = 1540 m/s unless stated.

```shell
$ biomedical-science solve bmed_ultrasound_wavelength "f=5 MHz"
wavelength = 0.000308 m
wavelength_mm = 0.308
period = 2e-07 s
k = 20399.9523 1/m
  by: Ultrasound wavelength in tissue  (lambda = c/f)
  f = 5000000 Hz
  assumes: f, c > 0; c = 1540 m/s unless stated
```

In the worked example, f = 5 MHz go through the equation above and give wavelength = 0.000308 m, wavelength_mm = 0.308, period = 2e-07 s, k = 20399.9523 1/m. It applies only when f, c > 0; c = 1540 m/s unless stated: outside that it says what is wrong instead of returning a number.

<a id="bmed_xray_attenuation_layers"></a>

#### X-ray attenuation through tissue layers (narrow monoenergetic beam), `bmed_xray_attenuation_layers`

The transmitted fraction and intensity of a narrow monoenergetic X-ray beam crossing successive tissue layers, from each layer's mass attenuation coefficient, density and thickness.

$$
\frac{I}{I_0} = \exp\left(-\sum_i \left(\frac{\mu}{\rho}\right)_i \rho_i x_i\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `I0` | in | incident intensity | W/m^2 |
| `mu_rho` | in | mass attenuation coefficient of each layer, cm^2/g | - |
| `rho` | in | density of each layer, g/cm^3 | - |
| `x` | in | thickness of each layer, cm | - |
| `T` | out | transmitted fraction | - |
| `I` | out | transmitted intensity | W/m^2 |

**Applies when:** narrow beam (no scatter reaching the detector); one photon energy; units cm^2/g, g/cm^3, cm.

```shell
$ biomedical-science solve bmed_xray_attenuation_layers "I0=1000 W/m^2" "mu_rho=[0.2, 0.5, 0.2]" "rho=[1, 1.85, 1]" "x=[2, 1, 3]"
T = 0.1458757569
I = 145.8757569 W/m^2
  by: X-ray attenuation through tissue layers (narrow monoenergetic beam)  (T = exp(-sum (mu/rho)_i rho_i x_i); I = I0 T)
  I0 = 1000 W/m^2
  assumes: narrow beam (no scatter reaching the detector); one photon energy; units cm^2/g, g/cm^3, cm
```

**What the worked example means:** This calculation determines how much of the initial X-ray beam survives passage through multiple tissue layers. Using the incident intensity 1000 W/m^2, along with the mass attenuation coefficients, densities, and thicknesses for each layer, it computes the total attenuation to find the transmitted fraction 0.145876. The resulting transmitted intensity 145.876 W/m^2 represents the radiation remaining after absorption by the tissues.

### Neuron models (10)

<a id="bmed_fitzhugh_nagumo_fixed_point"></a>

#### FitzHugh-Nagumo neuron model: fixed point and its stability, `bmed_fitzhugh_nagumo_fixed_point`

The resting (fixed) point of the FitzHugh-Nagumo model v' = v - v^3/3 - w + I, w' = eps (v + a - b w), the trace and determinant of its Jacobian and whether it is stable (resting) or unstable (repetitive firing).

$$
v - \frac{v^3}{3} - \frac{v + a}{b} + I = 0,\quad J = \begin{pmatrix}1 - v^2 & -1\\ \varepsilon & -\varepsilon b\end{pmatrix}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `a` | in | parameter a | - |
| `b` | in | parameter b (0 < b < 1) | - |
| `eps` | in | time-scale ratio eps (> 0) | - |
| `I` | in | applied current (dimensionless) | - |
| `v_star` | out | fixed-point v | - |
| `w_star` | out | fixed-point w | - |
| `trace` | out | trace of the Jacobian | - |
| `determinant` | out | determinant of the Jacobian | - |
| `stable` | out | true when the fixed point is stable | - |

**Applies when:** dimensionless FitzHugh-Nagumo model; 0 < b < 1 (a unique fixed point).

```shell
$ biomedical-science solve bmed_fitzhugh_nagumo_fixed_point a=0.7 b=0.8 eps=0.08 I=0
v_star = -1.199408035
w_star = -0.6242600441
trace = -0.502579635
determinant = 0.1080690966
stable = true
  by: FitzHugh-Nagumo neuron model: fixed point and its stability  (v* solves v - v^3/3 - (v + a)/b + I = 0 (bisection); w* = (v* + a)/b; J = [[1 - v*^2, -1], [eps, -eps b]]; stable when trace < 0 (det > 0 here))
  a = 0.7
  b = 0.8
  eps = 0.08
  I = 0
  assumes: dimensionless FitzHugh-Nagumo model; 0 < b < 1 (a unique fixed point)
```

**What the worked example means:** This calculation finds the resting voltage and recovery variable values for the FitzHugh-Nagumo neuron model. The fixed point coordinates are -1.19941 for voltage and -0.62426 for recovery, derived from parameters including current 0. The Jacobian's trace is -0.50258 and determinant is 0.108069. Since the trace is negative, the system remains at rest rather than firing repetitively.

<a id="bmed_ghk_current_density"></a>

#### Goldman-Hodgkin-Katz current equation: ionic current density through a membrane, `bmed_ghk_current_density`

The current density carried by one ion species through a membrane of given permeability at a stated membrane potential, from the GHK current (constant-field) equation; outward current positive.

$$
I = P z^2 \frac{F^2 V}{RT}\,\frac{[C]_i - [C]_o e^{-zFV/RT}}{1 - e^{-zFV/RT}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `P` | in | membrane permeability to the ion | m/s |
| `z` | in | valence of the ion (+1, +2, -1, ...) | - |
| `C_in` | in | inside concentration | mol/m^3 |
| `C_out` | in | outside concentration | mol/m^3 |
| `V` | in | membrane potential, inside relative to outside | V |
| `T` | in | absolute temperature (or degC) | K |
| `I` | out | current density, outward positive | A/m^2 |

**Applies when:** constant field across the membrane; independent ion movement; outward current positive.

```shell
$ biomedical-science solve bmed_ghk_current_density "P=1e-08 m/s" z=1 "C_in=140 mmol/L" "C_out=4 mmol/L" "V=-20 mV" "T=310 K"
I = 0.08528254849 A/m^2
  by: Goldman-Hodgkin-Katz current equation: ionic current density through a membrane  (I = P z^2 F^2 V/(RT) (C_in - C_out exp(-u))/(1 - exp(-u)), u = zFV/(RT) (written stably as P z F [C_in g(u) - C_out g(-u)], g(u) = u/(1 - e^-u)))
  P = 1e-08 m/s
  C_in = 140 mol/m^3
  C_out = 4 mol/m^3
  V = -0.02 V
  T = 310 K
  assumes: constant field across the membrane; independent ion movement; outward current positive
```

**What the worked example means:** This calculation determines the electrical current density carried by a single ion species across a cell membrane using the Goldman-Hodgkin-Katz equation. It takes into account the membrane permeability, the valence of the ion, the concentrations inside and outside the cell, the membrane potential, and the absolute temperature. The result, 0.0852825 A/m^2, represents the net flow of charge per unit area, where a positive value indicates that current is flowing outward from the cell.

<a id="bmed_ghk_voltage"></a>

#### Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl, `bmed_ghk_voltage`

The resting membrane potential of the Goldman-Hodgkin-Katz voltage equation from the relative permeabilities and the inside and outside concentrations of K+, Na+ and Cl-.

$$
V_m = \frac{RT}{F}\ln\frac{P_K[K^+]_o + P_{Na}[Na^+]_o + P_{Cl}[Cl^-]_i}{P_K[K^+]_i + P_{Na}[Na^+]_i + P_{Cl}[Cl^-]_o}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `T` | in | absolute temperature (or degC) | K |
| `PK` | in | relative permeability to K+ (> 0) | - |
| `PNa` | in | relative permeability to Na+ | - |
| `PCl` | in | relative permeability to Cl- | - |
| `K_o` | in | outside K+ | mol/m^3 |
| `K_i` | in | inside K+ | mol/m^3 |
| `Na_o` | in | outside Na+ | mol/m^3 |
| `Na_i` | in | inside Na+ | mol/m^3 |
| `Cl_o` | in | outside Cl- | mol/m^3 |
| `Cl_i` | in | inside Cl- | mol/m^3 |
| `V_m` | out | membrane potential, inside relative to outside | V |

**Applies when:** steady state with zero net current; constant field in the membrane, independent ions; only K+, Na+ and Cl- permeate; only ratios of permeabilities matter.

```shell
$ biomedical-science solve bmed_ghk_voltage "T=310 K" PK=1 PNa=0.04 PCl=0.45 "K_o=4 mmol/L" "K_i=140 mmol/L" "Na_o=145 mmol/L" "Na_i=12 mmol/L" "Cl_o=116 mmol/L" "Cl_i=4 mmol/L"
V_m = -0.07506627615 V
  by: Goldman-Hodgkin-Katz voltage equation: membrane potential from K, Na and Cl  (V = (RT/F) ln((PK K_o + PNa Na_o + PCl Cl_i)/(PK K_i + PNa Na_i + PCl Cl_o)))
  T = 310 K
  PK = 1
  PNa = 0.04
  PCl = 0.45
  K_o = 4 mol/m^3
  K_i = 140 mol/m^3
  Na_o = 145 mol/m^3
  Na_i = 12 mol/m^3
  Cl_o = 116 mol/m^3
  Cl_i = 4 mol/m^3
  assumes: steady state with zero net current; constant field in the membrane, independent ions; only K+, Na+ and Cl- permeate; only ratios of permeabilities matter
```

**What the worked example means:** This calculation determines the resting membrane potential using the Goldman-Hodgkin-Katz voltage equation. It combines the absolute temperature, relative permeabilities, and inside and outside concentrations of potassium, sodium, and chloride ions. The resulting potential of -0.0750663 V represents the electrical difference across the cell membrane when these ions are in steady state with zero net current flow.

<a id="bmed_hh_potassium_conductance_clamp"></a>

#### Hodgkin-Huxley model: potassium conductance after a voltage-clamp step, `bmed_hh_potassium_conductance_clamp`

The potassium conductance gK = gK_max n^4 and the gate n a time t after the membrane is stepped from a holding potential (at steady state) to a new clamped potential, in the Hodgkin-Huxley model.

$$
n(t) = n_\infty - (n_\infty - n_0)e^{-t/\tau_n},\quad g_K = \bar g_K n^4
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gK_max` | in | maximal K conductance | S/m^2 |
| `V_hold` | in | holding potential before the step | V |
| `V_step` | in | clamped potential after the step | V |
| `t` | in | time after the step | s |
| `g_K` | out | K conductance at time t | S/m^2 |
| `n` | out | K gate n at time t | - |

**Applies when:** ideal voltage clamp; gates at steady state at the holding potential; Hodgkin-Huxley 1952 rate functions.

```shell
$ biomedical-science solve bmed_hh_potassium_conductance_clamp "gK_max=36 mS/cm^2" "V_hold=-65 mV" "V_step=0 mV" "t=2 ms"
g_K = 104.1721669 S/m^2
n = 0.7334361287
  by: Hodgkin-Huxley model: potassium conductance after a voltage-clamp step  (n(t) = n_inf(V1) - (n_inf(V1) - n_inf(V0)) exp(-(alpha_n + beta_n) t); g_K = gK_max n^4)
  gK_max = 360 S/m^2
  V_hold = -0.065 V
  V_step = 0 V
  t = 0.002 s
  assumes: ideal voltage clamp; gates at steady state at the holding potential; Hodgkin-Huxley 1952 rate functions
```

**What the worked example means:** This calculation determines the potassium conductance 104.172 S/m^2 and the activation gate value 0.733436 at a time of 2 ms after the membrane potential steps from -65 mV to 0 mV. The result reflects how the gate relaxes exponentially toward its new steady state, scaled by the fourth power for conductance.

<a id="bmed_hh_gating_steady_state"></a>

#### Hodgkin-Huxley model: steady-state gating variables at a membrane potential, `bmed_hh_gating_steady_state`

The steady-state values of the Hodgkin-Huxley gating variables m, h and n at a clamped membrane potential, from the 1952 squid-axon rate functions.

$$
x_\infty(V) = \frac{\alpha_x(V)}{\alpha_x(V) + \beta_x(V)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `V` | in | clamped membrane potential | V |
| `m_inf` | out | Na activation at steady state | - |
| `h_inf` | out | Na inactivation at steady state | - |
| `n_inf` | out | K activation at steady state | - |

**Applies when:** Hodgkin-Huxley 1952 rate functions, modern convention (rest near -65 mV), 6.3 degC; -150 mV <= V <= +100 mV.

```shell
$ biomedical-science solve bmed_hh_gating_steady_state "V=-65 mV"
m_inf = 0.05293248526
h_inf = 0.5961207535
n_inf = 0.3176769141
  by: Hodgkin-Huxley model: steady-state gating variables at a membrane potential  (x_inf = alpha_x / (alpha_x + beta_x) for x = m, h, n)
  V = -0.065 V
  assumes: Hodgkin-Huxley 1952 rate functions, modern convention (rest near -65 mV), 6.3 degC; -150 mV <= V <= +100 mV
```

In the worked example, V = -65 mV go through the equation above and give m_inf = 0.05293248526, h_inf = 0.5961207535, n_inf = 0.3176769141. It applies only when Hodgkin-Huxley 1952 rate functions, modern convention (rest near -65 mV), 6.3 degC; -150 mV <= V <= +100 mV: outside that it says what is wrong instead of returning a number.

<a id="bmed_hh_resting_potential"></a>

#### Hodgkin-Huxley model: zero-current steady (resting) membrane potential, `bmed_hh_resting_potential`

The membrane potential at which the steady-state Na, K and leak currents of the Hodgkin-Huxley model sum to zero (the resting potential), from the maximal conductances and reversal potentials.

$$
\bar g_{Na} m_\infty^3 h_\infty (V - E_{Na}) + \bar g_K n_\infty^4 (V - E_K) + g_L (V - E_L) = 0
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `gNa` | in | maximal Na conductance | S/m^2 |
| `gK` | in | maximal K conductance | S/m^2 |
| `gL` | in | leak conductance | S/m^2 |
| `ENa` | in | Na reversal potential | V |
| `EK` | in | K reversal potential | V |
| `EL` | in | leak reversal potential | V |
| `V_rest` | out | zero-current steady-state potential | V |

**Applies when:** Hodgkin-Huxley 1952 rate functions (modern convention), gates at steady state; a unique zero-current potential between -120 and +60 mV (checked); that state stable (Routh-Hurwitz test of the linearised model; otherwise refused); EK < EL < ENa.

```shell
$ biomedical-science solve bmed_hh_resting_potential "gNa=120 mS/cm^2" "gK=36 mS/cm^2" "gL=0.3 mS/cm^2" "ENa=50 mV" "EK=-77 mV" "EL=-54.387 mV"
V_rest = -0.06499637933 V
  by: Hodgkin-Huxley model: zero-current steady (resting) membrane potential  (root of gNa m_inf^3 h_inf (V - ENa) + gK n_inf^4 (V - EK) + gL (V - EL) = 0 (scan and bisection))
  gNa = 1200 S/m^2
  gK = 360 S/m^2
  gL = 3 S/m^2
  ENa = 0.05 V
  EK = -0.077 V
  EL = -0.054387 V
  assumes: Hodgkin-Huxley 1952 rate functions (modern convention), gates at steady state; a unique zero-current potential between -120 and +60 mV (checked); that state stable (Routh-Hurwitz test of the linearised model; otherwise refused); EK < EL < ENa
```

**What the worked example means:** This calculation determines the resting membrane potential where the sum of steady-state ionic currents equals zero. Using maximal conductances for sodium (120 mS/cm^2), potassium (36 mS/cm^2), and leak (0.3 mS/cm^2) along with their respective reversal potentials (50 mV, -77 mV, -54.387 mV), the method solves for the voltage that satisfies the current balance equation. The resulting potential, -0.0649964 V, represents the stable equilibrium state of the neuron model under these specific conductance conditions.

<a id="bmed_lif_firing_rate"></a>

#### Leaky integrate-and-fire neuron: firing rate under constant current, `bmed_lif_firing_rate`

The interspike interval and firing rate of a leaky integrate-and-fire neuron driven by a constant current, with reset and an absolute refractory period.

$$
T = t_{ref} + \tau_m \ln\frac{V_\infty - V_{reset}}{V_\infty - V_{th}},\quad V_\infty = V_{rest} + R_m I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tau_m` | in | membrane time constant | s |
| `R_m` | in | membrane resistance | ohm |
| `I` | in | constant input current | A |
| `V_rest` | in | resting potential | V |
| `V_th` | in | spike threshold | V |
| `V_reset` | in | reset potential | V |
| `t_ref` | in | absolute refractory period | s |
| `rate` | out | firing rate | Hz |
| `ISI` | out | interspike interval | s |

**Applies when:** V_rest + R I > V_th (otherwise no firing); V_reset < V_th; deterministic constant input.

```shell
$ biomedical-science solve bmed_lif_firing_rate "tau_m=20 ms" "R_m=100 Mohm" "I=0.3 nA" "V_rest=-70 mV" "V_th=-50 mV" "V_reset=-70 mV" "t_ref=2 ms"
rate = 41.71490687 Hz
ISI = 0.02397224577 s
  by: Leaky integrate-and-fire neuron: firing rate under constant current  (ISI = t_ref + tau ln((V_inf - V_reset)/(V_inf - V_th)), V_inf = V_rest + R I; rate = 1/ISI)
  tau_m = 0.02 s
  R_m = 100000000 ohm
  I = 3e-10 A
  V_rest = -0.07 V
  V_th = -0.05 V
  V_reset = -0.07 V
  t_ref = 0.002 s
  assumes: V_rest + R I > V_th (otherwise no firing); V_reset < V_th; deterministic constant input
```

**What the worked example means:** This calculation determines the steady-state firing behavior of a leaky integrate-and-fire neuron driven by a constant current. The membrane potential rises from rest toward an asymptotic value determined by the input current and membrane resistance. Once the threshold is crossed, the neuron spikes and resets after a fixed refractory period. The resulting interspike interval is 0.0239722 s, which corresponds to a firing rate of 41.7149 Hz. This output reflects how the intrinsic electrical properties of the cell, such as time constant and resistance, interact with the external stimulus strength to regulate neural activity.

<a id="bmed_cable_steady_state"></a>

#### Passive cable (dendrite or axon): steady-state voltage along it and input resistance, `bmed_cable_steady_state`

The steady-state voltage at distance x along a passive cylindrical cable held at V0 at x = 0, and the input resistance there, for a sealed end, a killed (short-circuited) end or a semi-infinite cable.

$$
\lambda = \sqrt{\frac{R_m d}{4 R_i}},\quad V(x) = V_0\frac{\cosh((L - x)/\lambda)}{\cosh(L/\lambda)}\ \text{(sealed)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Rm` | in | specific membrane resistance | ohm*m^2 |
| `Ri` | in | axial (cytoplasmic) resistivity | ohm*m |
| `d` | in | cable diameter | m |
| `x` | in | distance from the stimulated end | m |
| `V0` | in | steady voltage at x = 0 (relative to rest) | V |
| `end` | in | far-end condition | - |
| `L` | in | cable length (sealed or killed end only) (optional) | m |
| `V_x` | out | voltage at x (relative to rest) | V |
| `R_in` | out | input resistance at x = 0 | ohm |

**Applies when:** passive membrane, steady state; uniform cylinder; L only for a finite cable.

```shell
$ biomedical-science solve bmed_cable_steady_state "Rm=20000 ohm*cm^2" "Ri=100 ohm*cm" "d=2 um" "x=300 um" "V0=10 mV" end=sealed "L=1 mm"
V_x = 0.008134176383 V
R_in = 417952112.3 ohm
  by: Passive cable (dendrite or axon): steady-state voltage along it and input resistance  (lambda = sqrt(Rm d/(4 Ri)); sealed V0 cosh((L - x)/lambda)/cosh(L/lambda), killed V0 sinh((L - x)/lambda)/sinh(L/lambda), infinite V0 exp(-x/lambda); R_in = r_i lambda times coth, tanh or 1)
  Rm = 2 ohm*m^2
  Ri = 1 ohm*m
  d = 2e-06 m
  x = 0.0003 m
  V0 = 0.01 V
  L = 0.001 m
  end = sealed
  assumes: passive membrane, steady state; uniform cylinder; L only for a finite cable
```

**What the worked example means:** This calculation determines the electrical signal attenuation along a passive cylindrical cable. Given a specific membrane resistance of 20000 ohm*cm^2, axial resistivity of 100 ohm*cm, and diameter of 2 um, the model computes the space constant. For a voltage of 10 mV applied at the start, the resulting potential at position 300 um is 0.00813418 V. The effective resistance seen by the current source at the injection site is 4.17952e+08 ohm, which depends on the boundary condition at the far end.

<a id="bmed_lif_subthreshold_voltage"></a>

#### Passive membrane (leaky integrator): voltage at a time under constant current, `bmed_lif_subthreshold_voltage`

The membrane potential of a passive RC membrane (the subthreshold leaky integrate-and-fire neuron) a time t after a constant current starts, from its initial potential.

$$
V(t) = V_\infty + (V_0 - V_\infty)e^{-t/\tau_m},\quad V_\infty = V_{rest} + R_m I
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tau_m` | in | membrane time constant | s |
| `R_m` | in | membrane resistance | ohm |
| `I` | in | constant input current | A |
| `V_rest` | in | resting potential | V |
| `V0` | in | potential at t = 0 | V |
| `t` | in | time | s |
| `V` | out | membrane potential at time t | V |

**Applies when:** passive membrane, no spike or threshold; constant current from t = 0.

```shell
$ biomedical-science solve bmed_lif_subthreshold_voltage "tau_m=20 ms" "R_m=100 Mohm" "I=0.15 nA" "V_rest=-70 mV" "V0=-70 mV" "t=10 ms"
V = -0.0640979599 V
  by: Passive membrane (leaky integrator): voltage at a time under constant current  (tau dV/dt = -(V - V_rest) + R I: V = V_inf + (V0 - V_inf) exp(-t/tau))
  tau_m = 0.02 s
  R_m = 100000000 ohm
  I = 1.5e-10 A
  V_rest = -0.07 V
  V0 = -0.07 V
  t = 0.01 s
  assumes: passive membrane, no spike or threshold; constant current from t = 0
```

In the worked example, tau_m = 20 ms, R_m = 100 Mohm, I = 0.15 nA, V_rest = -70 mV, V0 = -70 mV, t = 10 ms go through the equation above and give V = -0.0640979599 V. It applies only when passive membrane, no spike or threshold; constant current from t = 0: outside that it says what is wrong instead of returning a number.

<a id="bmed_postsynaptic_potential"></a>

#### Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current, `bmed_postsynaptic_potential`

The postsynaptic potential of a passive RC membrane receiving a synaptic current I0 exp(-t/tau_s) at t = 0: its value at time t, the time of its peak and the peak value.

$$
V(t) = \frac{R I_0 \tau_s}{\tau_s - \tau_m}\left(e^{-t/\tau_s} - e^{-t/\tau_m}\right),\quad t_{peak} = \frac{\tau_m\tau_s}{\tau_s - \tau_m}\ln\frac{\tau_s}{\tau_m}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `tau_m` | in | membrane time constant | s |
| `tau_s` | in | synaptic current decay time constant | s |
| `R_m` | in | membrane resistance | ohm |
| `I0` | in | peak synaptic current (negative for inward) | A |
| `t` | in | time after the synaptic event | s |
| `V_t` | out | potential change at time t | V |
| `t_peak` | out | time of the peak | s |
| `V_peak` | out | peak potential change | V |

**Applies when:** passive membrane starting at rest; current jumps to I0 and decays exponentially; tau_s != tau_m.

```shell
$ biomedical-science solve bmed_postsynaptic_potential "tau_m=20 ms" "tau_s=5 ms" "R_m=100 Mohm" "I0=50 pA" "t=10 ms"
V_t = 0.0007853256275 V
t_peak = 0.009241962407 s
V_peak = 0.0007874506562 V
  by: Postsynaptic potential of a passive membrane driven by an exponentially decaying synaptic current  (tau_m dV/dt = -V + R I0 exp(-t/tau_s): V = R I0 tau_s/(tau_s - tau_m) (exp(-t/tau_s) - exp(-t/tau_m)); t_peak = tau_m tau_s ln(tau_s/tau_m)/(tau_s - tau_m))
  tau_m = 0.02 s
  tau_s = 0.005 s
  R_m = 100000000 ohm
  I0 = 5e-11 A
  t = 0.01 s
  assumes: passive membrane starting at rest; current jumps to I0 and decays exponentially; tau_s != tau_m
```

**What the worked example means:** This calculation determines the voltage response of a passive neuronal membrane to a synaptic current that decays exponentially. Using the membrane resistance, the peak current amplitude, and the distinct time constants for the membrane and the synapse, it computes the potential change at any given time. The result indicates that the potential peaks at 0.000787451 V after 0.00924196 s seconds, showing how quickly the neuron responds to the input signal before it dissipates.

### Pharmacokinetic data analysis (4)

<a id="bmed_pk_bioavailability_from_profiles"></a>

#### Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice), `bmed_pk_bioavailability_from_profiles`

The absolute bioavailability F from the dose-normalised areas under an oral and an IV concentration-time profile (linear trapezoids from the first to the last sample).

$$
F = \frac{AUC_{oral}\,D_{iv}}{AUC_{iv}\,D_{oral}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D_iv` | in | IV dose | kg |
| `t_iv` | in | IV sampling times (one time unit for both profiles) | - |
| `C_iv` | in | IV concentrations (one concentration unit for both profiles) | - |
| `D_oral` | in | oral dose | kg |
| `t_oral` | in | oral sampling times | - |
| `C_oral` | in | oral concentrations | - |
| `AUC_iv` | out | IV area (units of the lists) | - |
| `AUC_oral` | out | oral area (units of the lists) | - |
| `F` | out | absolute bioavailability (AUC_oral/D_oral)/(AUC_iv/D_iv) | - |

**Applies when:** linear pharmacokinetics (dose-proportional areas); both profiles sampled to a comparable end; areas to the last sample, no extrapolation; the same units in both profiles.

```shell
$ biomedical-science solve bmed_pk_bioavailability_from_profiles "D_iv=100 mg" "t_iv=[0, 1, 2, 4, 8, 12]" "C_iv=[10, 8.2, 6.7, 4.5, 2, 0.9]" "D_oral=200 mg" "t_oral=[0, 1, 2, 4, 8, 12]" "C_oral=[0, 6.1, 8, 7.2, 3.9, 1.8]"
AUC_iv = 46.55
AUC_oral = 58.9
F = 0.6326530612
  by: Absolute bioavailability from IV and oral concentration profiles (a model calculation, not medical advice)  (F = (AUC_oral D_iv)/(AUC_iv D_oral) with linear-trapezoid areas)
  D_iv = 0.0001 kg
  D_oral = 0.0002 kg
  assumes: linear pharmacokinetics (dose-proportional areas); both profiles sampled to a comparable end; areas to the last sample, no extrapolation; the same units in both profiles
```

**What the worked example means:** This calculation determines the fraction of an orally administered dose that reaches systemic circulation by comparing exposure metrics. Using linear trapezoidal integration, the area under the concentration-time curve for the intravenous route is computed as 46.55, while the corresponding value for the oral route is 58.9. These areas are normalized by their respective doses, 100 mg for intravenous administration and 200 mg for oral administration, to account for dosage differences. The resulting ratio yields an absolute bioavailability of 0.632653, indicating how much active drug successfully enters the bloodstream after passing through the gastrointestinal tract compared to direct injection.

<a id="bmed_pk_auc_trapezoid"></a>

#### AUC from concentration-time data by the trapezoidal rule, `bmed_pk_auc_trapezoid`

The area under a sampled concentration-time curve from the first to the last sample, by the linear trapezoidal rule or the linear-up/log-down rule (stated).

$$
AUC = \sum_i \frac{(C_i + C_{i+1})\,\Delta t_i}{2}\quad\text{or}\quad \frac{(C_i - C_{i+1})\,\Delta t_i}{\ln(C_i/C_{i+1})}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | sampling times, strictly increasing, in one time unit | - |
| `C` | in | concentrations at those times, in one concentration unit | - |
| `method` | in | linear trapezoids, or linear while rising and logarithmic while falling | - |
| `AUC` | out | area from the first to the last sample, in (unit of C) x (unit of t) | - |

**Applies when:** times strictly increasing; concentrations >= 0; log-down segments need both concentrations positive (otherwise linear).

```shell
$ biomedical-science solve bmed_pk_auc_trapezoid "t=[0, 0.5, 1, 2, 4, 6, 8, 12]" "C=[0, 4.2, 6.8, 7.9, 6.1, 4.3, 3, 1.5]" method=linear
AUC = 51.85
  by: AUC from concentration-time data by the trapezoidal rule  (sum of segment areas: linear (C1 + C2) dt / 2; log-down (C1 - C2) dt / ln(C1/C2))
  method = linear
  assumes: times strictly increasing; concentrations >= 0; log-down segments need both concentrations positive (otherwise linear)
```

**What the worked example means:** This calculation determines the total drug exposure over time by approximating the area under the observed concentration curve. It uses discrete sampling times and measured concentrations, applying either linear trapezoids or a hybrid method that switches to logarithmic interpolation during declining phases. The resulting value represents the cumulative integral of concentration with respect to time, expressed as 51.85, which reflects how much active substance was present in the system across the measured interval.

<a id="bmed_pk_terminal_half_life_fit"></a>

#### Elimination rate constant and half-life from concentration data (log-linear fit), `bmed_pk_terminal_half_life_fit`

The first-order elimination rate constant, half-life and back-extrapolated concentration from a straight-line least-squares fit of ln C against time (mono-exponential decline).

$$
\ln C = \ln C_0 - k t,\quad t_{1/2} = \frac{\ln 2}{k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `t` | in | sampling times, in one time unit (results are per that unit) | - |
| `C` | in | concentrations at those times, in one concentration unit (C0 is in that unit) | - |
| `k` | out | elimination rate constant, per time unit of t | - |
| `t_half` | out | half-life, in the time unit of t | - |
| `C0` | out | back-extrapolated concentration at t = 0, in the unit of C | - |

**Applies when:** mono-exponential decline over the sampled times; all concentrations positive; unweighted least squares on ln C; at least three points.

```shell
$ biomedical-science solve bmed_pk_terminal_half_life_fit "t=[1, 2, 4, 6, 8, 12]" "C=[8.2, 6.9, 4.8, 3.4, 2.4, 1.2]"
k = 0.1748048244
t_half = 3.965263448
C0 = 9.735049617
  by: Elimination rate constant and half-life from concentration data (log-linear fit)  (least-squares line ln C = ln C0 - k t; t_half = ln 2 / k)
  assumes: mono-exponential decline over the sampled times; all concentrations positive; unweighted least squares on ln C; at least three points
```

**What the worked example means:** This calculation determines the first-order elimination rate constant, half-life, and back-extrapolated initial concentration from measured plasma levels. By fitting a straight line to the natural logarithm of concentration against time, the slope provides the rate constant 0.174805, which directly dictates how quickly the substance clears from the body. The derived half-life 3.96526 represents the duration required for the concentration to decrease by fifty percent, while the intercept estimates the theoretical starting concentration 9.73505 at zero time.

<a id="bmed_pk_nca_iv_bolus"></a>

#### Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz), `bmed_pk_nca_iv_bolus`

Non-compartmental parameters from a concentration-time profile after an IV bolus: AUC to the last sample (linear trapezoids), terminal rate constant from the last n points, half-life, AUC to infinity, clearance and terminal volume.

$$
AUC_{0-\infty} = AUC_{0-t_{last}} + \frac{C_{last}}{\lambda_z},\quad CL = \frac{D}{AUC_{0-\infty}},\quad V_z = \frac{CL}{\lambda_z}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | IV bolus dose | kg |
| `t` | in | sampling times in h, the first at 0 (measured or back-extrapolated C0) | - |
| `C` | in | concentrations in mg/L at those times | - |
| `n_terminal` | in | number of last points used for the terminal slope (3 to n - 1) | - |
| `AUC_last` | out | AUC from 0 to the last sample, mg*h/L | - |
| `lambda_z` | out | terminal rate constant, 1/h | - |
| `t_half` | out | terminal half-life, h | - |
| `AUC_inf` | out | AUC from 0 to infinity (observed C_last / lambda_z added), mg*h/L | - |
| `CL` | out | clearance D / AUC_inf, L/h | - |
| `Vz` | out | terminal volume CL / lambda_z, L | - |

**Applies when:** linear trapezoidal rule; lambda_z by unweighted least squares on ln C of the last n points; extrapolation with the observed last concentration; times in h, concentrations in mg/L.

```shell
$ biomedical-science solve bmed_pk_nca_iv_bolus "D=100 mg" "t=[0, 0.25, 0.5, 1, 2, 4, 6, 8, 12, 24]" "C=[5, 4, 3.3, 2.44, 1.72, 1.25, 0.974, 0.766, 0.474, 0.112]" n_terminal=4
AUC_last = 18.4825
lambda_z = 0.1201659481
t_half = 5.768249588
AUC_inf = 19.41454441
CL = 5.150777577
Vz = 42.86386999
  by: Non-compartmental analysis of an IV bolus profile (AUC, lambda_z, half-life, clearance, Vz)  (AUC by trapezoids; ln C = a - lambda_z t on the last points; AUC_inf = AUC_last + C_last/lambda_z; CL = D/AUC_inf; Vz = CL/lambda_z)
  D = 0.0001 kg
  assumes: linear trapezoidal rule; lambda_z by unweighted least squares on ln C of the last n points; extrapolation with the observed last concentration; times in h, concentrations in mg/L
```

**What the worked example means:** This calculation derives pharmacokinetic parameters from concentration-time data following an intravenous bolus. It computes the area under the curve to the last sample using the linear trapezoidal rule, resulting in 18.4825. The terminal elimination rate constant is estimated by fitting a line to the logarithm of concentrations at the final time points, giving 0.120166. From this, the terminal half-life is calculated as 5.76825. The total exposure is extrapolated to infinity by adding the tail area, yielding 19.4145. Finally, systemic clearance is determined by dividing the dose by the total exposure, resulting in 5.15078, while the volume of distribution during the terminal phase is derived from clearance and the elimination rate constant, giving 42.8639.

### Pharmacokinetic models (20)

<a id="bmed_pk_auc_iv_bolus"></a>

#### Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice), `bmed_pk_auc_iv_bolus`

The area under the plasma concentration-time curve from 0 to infinity and from 0 to t after an IV bolus in a one-compartment model.

$$
AUC_{0-\infty} = \frac{D}{CL},\quad AUC_{0-t} = \frac{D}{CL}\left(1 - e^{-CL\,t/V}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | bolus dose | kg |
| `CL` | in | clearance | m^3/s |
| `V` | in | volume of distribution | m^3 |
| `t` | in | end of the partial area | s |
| `AUC_inf` | out | area from 0 to infinity | kg*s/m^3 |
| `AUC_0_t` | out | area from 0 to t | kg*s/m^3 |

**Applies when:** one compartment; first-order elimination k = CL/V.

```shell
$ biomedical-science solve bmed_pk_auc_iv_bolus "D=500 mg" "CL=5 L/h" "V=40 L" "t=8 h"
AUC_inf = 360 kg*s/m^3
AUC_0_t = 227.5634012 kg*s/m^3
  by: Area under the concentration curve of a one-compartment IV bolus (a model calculation, not medical advice)  (AUC(0-inf) = D/CL; AUC(0-t) = (D/CL)(1 - exp(-CL t / V)))
  D = 0.0005 kg
  CL = 1.388888889e-06 m^3/s
  V = 0.04 m^3
  t = 28800 s
  assumes: one compartment; first-order elimination k = CL/V
```

**What the worked example means:** This calculation determines the area under the plasma concentration-time curve for a one-compartment model following an intravenous bolus. Using the dose of 500 mg, clearance of 5 L/h, volume of distribution of 40 L, and observation time of 8 h, it computes the total systemic exposure as 360 kg*s/m^3 and the cumulative exposure up to the specified time point as 227.563 kg*s/m^3. These values represent the integral of concentration over time, providing a measure of the body's overall exposure to the drug.

<a id="bmed_pk_time_to_steady_state_fraction"></a>

#### Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice), `bmed_pk_time_to_steady_state_fraction`

The time after the start of a constant-rate infusion at which a one-compartment model reaches a stated fraction of its steady-state concentration.

$$
t_f = -\frac{\ln(1 - f)}{CL/V}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `CL` | in | clearance | m^3/s |
| `V` | in | volume of distribution | m^3 |
| `f` | in | fraction of steady state (90% or 0.9) | - |
| `t` | out | time to reach f of steady state | s |

**Applies when:** one compartment, first-order elimination; no drug at the start; 0 < f < 1.

```shell
$ biomedical-science solve bmed_pk_time_to_steady_state_fraction "CL=5 L/h" "V=40 L" f=0.9
t = 66314.45068 s
  by: Constant infusion: time to reach a fraction of steady state (a model calculation, not medical advice)  (1 - exp(-k t) = f: t = -ln(1 - f)/k, k = CL/V)
  CL = 1.388888889e-06 m^3/s
  V = 0.04 m^3
  f = 0.9
  assumes: one compartment, first-order elimination; no drug at the start; 0 < f < 1
```

**What the worked example means:** This calculation determines the duration required for a drug concentration to attain 0.9 of its steady-state level during constant infusion. Using clearance 5 L/h and volume of distribution 40 L, the result indicates that it takes 66314.5 s to achieve this fraction. This helps clinicians understand when therapeutic effects stabilize and plan appropriate timing for blood level measurements.

<a id="bmed_pk_infusion_concentration"></a>

#### Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice), `bmed_pk_infusion_concentration`

The plasma concentration of a one-compartment model during a constant-rate intravenous infusion started at t = 0, and the steady-state concentration it approaches.

$$
C(t) = \frac{R_0}{CL}\left(1 - e^{-CL\,t/V}\right),\quad C_{ss} = \frac{R_0}{CL}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | infusion rate | kg/s |
| `CL` | in | clearance | m^3/s |
| `V` | in | volume of distribution | m^3 |
| `t` | in | time since the start of the infusion | s |
| `C` | out | concentration at time t | kg/m^3 |
| `C_ss` | out | steady-state concentration R0/CL | kg/m^3 |

**Applies when:** one compartment, first-order elimination; infusion still running at t; no drug at t = 0.

```shell
$ biomedical-science solve bmed_pk_infusion_concentration "R0=50 mg/h" "CL=5 L/h" "V=40 L" "t=6 h"
C = 0.005276334473 kg/m^3
C_ss = 0.01 kg/m^3
  by: Constant-rate IV infusion: concentration at a time and steady state (a model calculation, not medical advice)  (C = (R0/CL)(1 - exp(-CL t / V)))
  R0 = 1.388888889e-08 kg/s
  CL = 1.388888889e-06 m^3/s
  V = 0.04 m^3
  t = 21600 s
  assumes: one compartment, first-order elimination; infusion still running at t; no drug at t = 0
```

**What the worked example means:** This calculation determines the drug concentration in plasma during a constant intravenous infusion. Using an infusion rate of 50 mg/h, clearance of 5 L/h, and volume of distribution of 40 L, the model predicts a concentration of 0.00527633 kg/m^3 after 6 h seconds have elapsed. This value rises asymptotically toward the theoretical steady-state concentration of 0.01 kg/m^3, which represents the plateau level where the rate of drug input equals the rate of elimination.

<a id="bmed_pk_effect_site_concentration"></a>

#### Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice), `bmed_pk_effect_site_concentration`

The effect-site concentration of a one-compartment model linked to an effect compartment (rate ke0) after an IV bolus, the time of its peak and the peak value.

$$
C_e(t) = \frac{D}{V}\,\frac{k_{e0}}{k_{e0} - k}\left(e^{-k t} - e^{-k_{e0} t}\right),\quad t_{peak} = \frac{\ln(k_{e0}/k)}{k_{e0} - k}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | bolus dose | kg |
| `V` | in | volume of distribution | m^3 |
| `k` | in | elimination rate constant | 1/s |
| `ke0` | in | effect-site equilibration rate constant | 1/s |
| `t` | in | time after the dose | s |
| `Ce` | out | effect-site concentration at time t | kg/m^3 |
| `t_peak` | out | time of the effect-site peak | s |
| `Ce_peak` | out | peak effect-site concentration | kg/m^3 |

**Applies when:** one compartment plus an effect compartment of negligible volume; ke0 != k.

```shell
$ biomedical-science solve bmed_pk_effect_site_concentration "D=10 mg" "V=20 L" "k=0.3 1/h" "ke0=2 1/h" "t=30 min"
Ce = 0.0002898991384 kg/m^3
t_peak = 4017.430556 s
Ce_peak = 0.0003577456356 kg/m^3
  by: Effect-compartment model: effect-site concentration and its peak after an IV bolus (a model calculation, not medical advice)  (dCe/dt = ke0 (C - Ce): Ce = (D/V) ke0/(ke0 - k) (exp(-k t) - exp(-ke0 t)); peak where Ce = C)
  D = 1e-05 kg
  V = 0.02 m^3
  k = 8.333333333e-05 1/s
  ke0 = 0.0005555555556 1/s
  t = 1800 s
  assumes: one compartment plus an effect compartment of negligible volume; ke0 != k
```

**What the worked example means:** This calculation determines the drug concentration at the site of action after an intravenous bolus, accounting for equilibration delay. Using a dose of 10 mg, volume of distribution 20 L, elimination rate 0.3 1/h, and equilibration rate 2 1/h, the model predicts an effect-site concentration of 0.000289899 kg/m^3 at time 30 min. The peak effect-site concentration is calculated as 0.000357746 kg/m^3, which occurs at time 4017.43 s. This helps explain hysteresis by showing how the effect lags behind plasma levels.

<a id="bmed_pk_hepatic_clearance_parallel_tube"></a>

#### Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice), `bmed_pk_hepatic_clearance_parallel_tube`

The hepatic clearance and extraction ratio of the parallel-tube liver model, in which the concentration falls along the sinusoids, from blood flow, unbound fraction and intrinsic clearance.

$$
CL_H = Q\left(1 - e^{-f_u CL_{int}/Q}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | hepatic blood flow | m^3/s |
| `fu` | in | unbound fraction in blood (0 < fu <= 1) | - |
| `CL_int` | in | intrinsic clearance of unbound drug | m^3/s |
| `CL_H` | out | hepatic clearance | m^3/s |
| `E` | out | extraction ratio | - |

**Applies when:** plug flow along identical sinusoids, enzymes evenly distributed; linear elimination.

```shell
$ biomedical-science solve bmed_pk_hepatic_clearance_parallel_tube "Q=90 L/h" fu=0.1 "CL_int=500 L/h"
CL_H = 1.065616448e-05 m^3/s
E = 0.4262465793
  by: Hepatic clearance: parallel-tube (sinusoidal) liver model (a model calculation, not medical advice)  (E = 1 - exp(-fu CL_int / Q); CL_H = Q E)
  Q = 2.5e-05 m^3/s
  fu = 0.1
  CL_int = 0.0001388888889 m^3/s
  assumes: plug flow along identical sinusoids, enzymes evenly distributed; linear elimination
```

**What the worked example means:** This calculation determines the liver's ability to remove drug from the blood using the parallel tube model. Combining hepatic blood flow 90 L/h, the unbound fraction of the drug in blood 0.1, and intrinsic clearance 500 L/h gives the total hepatic clearance 1.06562e-05 m^3/s and the extraction ratio 0.426247. The result indicates how efficiently the liver extracts the drug as it flows through the sinusoids, assuming plug flow conditions.

<a id="bmed_pk_hepatic_clearance_well_stirred"></a>

#### Hepatic clearance: well-stirred liver model (a model calculation, not medical advice), `bmed_pk_hepatic_clearance_well_stirred`

The hepatic clearance and extraction ratio of the well-stirred (venous equilibrium) liver model from hepatic blood flow, unbound fraction and intrinsic clearance.

$$
CL_H = \frac{Q\,f_u\,CL_{int}}{Q + f_u\,CL_{int}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `Q` | in | hepatic blood flow | m^3/s |
| `fu` | in | unbound fraction in blood (0 < fu <= 1) | - |
| `CL_int` | in | intrinsic clearance of unbound drug | m^3/s |
| `CL_H` | out | hepatic clearance | m^3/s |
| `E` | out | extraction ratio | - |

**Applies when:** well-stirred liver: outflow concentration equals the liver concentration; linear elimination.

```shell
$ biomedical-science solve bmed_pk_hepatic_clearance_well_stirred "Q=90 L/h" fu=0.1 "CL_int=500 L/h"
CL_H = 8.928571429e-06 m^3/s
E = 0.3571428571
  by: Hepatic clearance: well-stirred liver model (a model calculation, not medical advice)  (CL_H = Q fu CL_int / (Q + fu CL_int); E = CL_H / Q)
  Q = 2.5e-05 m^3/s
  fu = 0.1
  CL_int = 0.0001388888889 m^3/s
  assumes: well-stirred liver: outflow concentration equals the liver concentration; linear elimination
```

**What the worked example means:** The well-stirred liver model calculates hepatic clearance as 8.92857e-06 m^3/s using hepatic blood flow of 90 L/h, unbound fraction of 0.1, and intrinsic clearance of 500 L/h. The resulting extraction ratio is 0.357143, indicating how efficiently the liver removes the drug from the blood.

<a id="bmed_pk_short_infusion"></a>

#### Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice), `bmed_pk_short_infusion`

The plasma concentration of a one-compartment model at time t after the start of a constant-rate infusion lasting T_inf (during or after the infusion), and the concentration at the end of the infusion.

$$
C(t) = \begin{cases}\frac{R_0}{CL}(1 - e^{-kt}) & t \le T\\ \frac{R_0}{CL}(1 - e^{-kT})\,e^{-k(t - T)} & t > T\end{cases}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | infusion rate | kg/s |
| `T_inf` | in | duration of the infusion | s |
| `CL` | in | clearance | m^3/s |
| `V` | in | volume of distribution | m^3 |
| `t` | in | time since the start of the infusion | s |
| `C` | out | concentration at time t | kg/m^3 |
| `C_end` | out | concentration at the end of the infusion | kg/m^3 |

**Applies when:** one compartment, first-order elimination; single infusion, no drug before it.

```shell
$ biomedical-science solve bmed_pk_short_infusion "R0=200 mg/h" "T_inf=1 h" "CL=6 L/h" "V=30 L" "t=4 h"
C = 0.003316089066 kg/m^3
C_end = 0.006042308231 kg/m^3
  by: Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice)  (during: (R0/CL)(1 - exp(-k t)); after: C_end exp(-k (t - T_inf)), k = CL/V)
  R0 = 5.555555556e-08 kg/s
  T_inf = 3600 s
  CL = 1.666666667e-06 m^3/s
  V = 0.03 m^3
  t = 14400 s
  assumes: one compartment, first-order elimination; single infusion, no drug before it
```

**What the worked example means:** This calculation determines the plasma drug concentration at a specific time point relative to a constant-rate infusion. Using the infusion rate 200 mg/h, clearance 6 L/h, volume of distribution 30 L, and infusion duration 1 h, it computes the current concentration 0.00331609 kg/m^3 and the maximum concentration achieved at the end of the infusion, denoted as 0.00604231 kg/m^3. The result reflects how the drug accumulates during the infusion phase and subsequently declines due to first-order elimination after the infusion stops.

<a id="bmed_pk_loading_maintenance"></a>

#### Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation), `bmed_pk_loading_maintenance`

For a one-compartment model, the bolus and the constant infusion rate that together hold the model concentration at a target value from the start (model calculation only, not a dosing recommendation).

$$
LD = C_{target} V,\quad R_0 = C_{target}\,CL
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C_target` | in | target model concentration | kg/m^3 |
| `V` | in | volume of distribution | m^3 |
| `CL` | in | clearance | m^3/s |
| `LD` | out | loading bolus of the model | kg |
| `R0` | out | maintenance infusion rate of the model | kg/s |

**Applies when:** one compartment, first-order elimination; bolus and infusion start together.

```shell
$ biomedical-science solve bmed_pk_loading_maintenance "C_target=15 mg/L" "V=50 L" "CL=4 L/h"
LD = 0.00075 kg
R0 = 1.666666667e-08 kg/s
  by: Loading dose and maintenance infusion rate of a one-compartment model (model calculation, not a dosing recommendation)  (LD = C V fills the volume; R0 = C CL replaces what is cleared)
  C_target = 0.015 kg/m^3
  V = 0.05 m^3
  CL = 1.111111111e-06 m^3/s
  assumes: one compartment, first-order elimination; bolus and infusion start together
```

**What the worked example means:** This calculation determines the specific bolus amount and continuous infusion rate needed to immediately achieve and sustain a target concentration within a one-compartment pharmacokinetic model. By using the provided volume of distribution and clearance values, the system computes a loading dose of 0.00075 kg to fill the compartment and an infusion rate of 1.66667e-08 kg/s to replace what is cleared per unit time, ensuring the concentration remains at the desired level from the start.

<a id="bmed_pk_michaelis_menten_concentration"></a>

#### Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice), `bmed_pk_michaelis_menten_concentration`

The concentration a time t after an IV bolus in a one-compartment model with saturable Michaelis-Menten elimination, from the closed form with the Lambert W function.

$$
C(t) = K_m\,W\!\left(\frac{C_0}{K_m}\exp\frac{C_0 - V_{max} t / V}{K_m}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | kg/m^3 |
| `Vmax` | in | maximum elimination rate (amount per time) | kg/s |
| `Km` | in | Michaelis constant | kg/m^3 |
| `V` | in | volume of distribution | m^3 |
| `t` | in | time after the dose | s |
| `C` | out | concentration at time t | kg/m^3 |

**Applies when:** one compartment; elimination rate Vmax C/(Km + C); no other elimination.

```shell
$ biomedical-science solve bmed_pk_michaelis_menten_concentration "C0=20 mg/L" "Vmax=500 mg/day" "Km=4 mg/L" "V=50 L" "t=2 day"
C = 0.005306898661 kg/m^3
  by: Michaelis-Menten (saturable) elimination: concentration at a time (a model calculation, not medical advice)  (V dC/dt = -Vmax C/(Km + C) integrates to C = Km W((C0/Km) exp((C0 - Vmax t/V)/Km)); W found from w + ln w = L by Newton steps)
  C0 = 0.02 kg/m^3
  Vmax = 5.787037037e-09 kg/s
  Km = 0.004 kg/m^3
  V = 0.05 m^3
  t = 172800 s
  assumes: one compartment; elimination rate Vmax C/(Km + C); no other elimination
```

**What the worked example means:** This calculation determines the drug concentration in the body at a specific time after an intravenous dose, assuming elimination follows saturable Michaelis Menten kinetics. The result 0.0053069 kg/m^3 represents the mass of drug per unit volume remaining in the central compartment, derived from the initial concentration 20 mg/L, the maximum elimination rate 500 mg/day, the Michaelis constant 4 mg/L, and the volume of distribution 50 L.

<a id="bmed_pk_michaelis_menten_steady_state"></a>

#### Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice), `bmed_pk_michaelis_menten_steady_state`

The steady-state concentration of a one-compartment model with Michaelis-Menten elimination given a constant dosing rate below Vmax.

$$
C_{ss} = \frac{K_m R_0}{V_{max} - R_0}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | constant dosing rate (amount per time) | kg/s |
| `Vmax` | in | maximum elimination rate | kg/s |
| `Km` | in | Michaelis constant | kg/m^3 |
| `C_ss` | out | steady-state concentration | kg/m^3 |

**Applies when:** Michaelis-Menten elimination only; R0 < Vmax (otherwise no steady state).

```shell
$ biomedical-science solve bmed_pk_michaelis_menten_steady_state "R0=300 mg/day" "Vmax=500 mg/day" "Km=4 mg/L"
C_ss = 0.006 kg/m^3
  by: Michaelis-Menten elimination: steady-state concentration under a constant dosing rate (a model calculation, not medical advice)  (R0 = Vmax C/(Km + C): C_ss = Km R0/(Vmax - R0))
  R0 = 3.472222222e-09 kg/s
  Vmax = 5.787037037e-09 kg/s
  Km = 0.004 kg/m^3
  assumes: Michaelis-Menten elimination only; R0 < Vmax (otherwise no steady state)
```

**What the worked example means:** This calculation determines the stable drug concentration in a one-compartment model with Michaelis-Menten elimination, given a constant dosing rate of 300 mg/day and maximum elimination capacity of 500 mg/day. The result, 0.006 kg/m^3, represents the equilibrium level where the rate of drug administration equals the rate of saturable elimination. This value is valid only when the dosing rate remains below the maximum elimination capacity.

<a id="bmed_pk_michaelis_menten_time"></a>

#### Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice), `bmed_pk_michaelis_menten_time`

The time for the concentration of a one-compartment model with Michaelis-Menten elimination to fall from C0 to C.

$$
t = \frac{V}{V_{max}}\left[(C_0 - C) + K_m \ln\frac{C_0}{C}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `C0` | in | initial concentration | kg/m^3 |
| `C` | in | target concentration (< C0) | kg/m^3 |
| `Vmax` | in | maximum elimination rate | kg/s |
| `Km` | in | Michaelis constant | kg/m^3 |
| `V` | in | volume of distribution | m^3 |
| `t` | out | time to fall from C0 to C | s |

**Applies when:** one compartment; Michaelis-Menten elimination only; C < C0.

```shell
$ biomedical-science solve bmed_pk_michaelis_menten_time "C0=20 mg/L" "C=10 mg/L" "Vmax=500 mg/day" "Km=4 mg/L" "V=50 L"
t = 110355.1666 s
  by: Michaelis-Menten elimination: time to fall to a concentration (a model calculation, not medical advice)  (t = V [(C0 - C) + Km ln(C0/C)] / Vmax)
  C0 = 0.02 kg/m^3
  C = 0.01 kg/m^3
  Vmax = 5.787037037e-09 kg/s
  Km = 0.004 kg/m^3
  V = 0.05 m^3
  assumes: one compartment; Michaelis-Menten elimination only; C < C0
```

**What the worked example means:** This calculation determines the elapsed time required for a drug concentration in a one-compartment model to decrease from an initial value of 20 mg/L to a target value of 10 mg/L, assuming elimination follows Michaelis-Menten kinetics. The result, 110355 s, represents the duration needed for this reduction given the volume of distribution 50 L, maximum elimination rate 500 mg/day, and Michaelis constant 4 mg/L. This approach is particularly relevant when enzyme saturation prevents simple first-order decay, meaning the clearance rate changes significantly with concentration.

<a id="bmed_pk_iv_bolus_concentration"></a>

#### One-compartment IV bolus: concentration at a time (a model calculation, not medical advice), `bmed_pk_iv_bolus_concentration`

The plasma concentration of a one-compartment model a time t after an intravenous bolus dose, with first-order elimination; also the fraction of the dose remaining.

$$
C(t) = \frac{D}{V}\,e^{-k t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | bolus dose | kg |
| `V` | in | volume of distribution | m^3 |
| `k` | in | elimination rate constant | 1/s |
| `t` | in | time after the dose | s |
| `C` | out | concentration at time t | kg/m^3 |
| `fraction_remaining` | out | fraction of the dose still in the body | - |

**Applies when:** one compartment, instantaneous mixing; first-order elimination; t >= 0.

```shell
$ biomedical-science solve bmed_pk_iv_bolus_concentration "D=500 mg" "V=40 L" "k=0.1 1/h" "t=6 h"
C = 0.006860145451 kg/m^3
fraction_remaining = 0.5488116361
  by: One-compartment IV bolus: concentration at a time (a model calculation, not medical advice)  (C(t) = (D/V) exp(-k t))
  D = 0.0005 kg
  V = 0.04 m^3
  k = 2.777777778e-05 1/s
  t = 21600 s
  assumes: one compartment, instantaneous mixing; first-order elimination; t >= 0
```

**What the worked example means:** This calculation determines the plasma concentration of a drug at time 6 h after an intravenous bolus dose of 500 mg, assuming the body acts as a single well-mixed compartment with first-order elimination. Using a volume of distribution of 40 L and an elimination rate constant of 0.1 1/h, the resulting concentration is 0.00686015 kg/m^3. Additionally, the fraction of the original dose still present in the body is calculated to be 0.548812, which indicates how much drug remains relative to the initial administration.

<a id="bmed_pk_oral_concentration"></a>

#### Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice), `bmed_pk_oral_concentration`

The plasma concentration and partial AUC of a one-compartment model with first-order absorption (rate ka) and elimination (rate k) after a single oral dose with bioavailability F.

$$
C(t) = \frac{F D k_a}{V (k_a - k)}\left(e^{-k t} - e^{-k_a t}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | bioavailable fraction (0 < F <= 1; 80% or 0.8) | - |
| `D` | in | oral dose | kg |
| `V` | in | volume of distribution | m^3 |
| `ka` | in | absorption rate constant | 1/s |
| `k` | in | elimination rate constant | 1/s |
| `t` | in | time after the dose | s |
| `C` | out | concentration at time t | kg/m^3 |
| `AUC_0_t` | out | area under the curve from 0 to t | kg*s/m^3 |

**Applies when:** one compartment; first-order absorption and elimination; ka != k; no lag time.

```shell
$ biomedical-science solve bmed_pk_oral_concentration F=0.8 "D=250 mg" "V=35 L" "ka=1.2 1/h" "k=0.15 1/h" "t=3 h"
C = 0.003985661578 kg/m^3
AUC_0_t = 37.7397259 kg*s/m^3
  by: Oral dose, first-order absorption: concentration at a time (Bateman function) (a model calculation, not medical advice)  (Bateman function C = F D ka / (V (ka - k)) (exp(-k t) - exp(-ka t)) and its integral)
  F = 0.8
  D = 0.00025 kg
  V = 0.035 m^3
  ka = 0.0003333333333 1/s
  k = 4.166666667e-05 1/s
  t = 10800 s
  assumes: one compartment; first-order absorption and elimination; ka != k; no lag time
```

**What the worked example means:** This calculation determines the drug concentration in the blood at a specific time after an oral dose, using the Bateman function for a one-compartment model. It accounts for how much of the dose actually reaches circulation, the speed at which it enters the body, and the rate at which the body removes it. The result shows that at 3 h, the concentration stands at 0.00398566 kg/m^3, while the total area under the curve from administration to that moment is 37.7397 kg*s/m^3, representing the overall systemic exposure during this period.

<a id="bmed_pk_oral_peak"></a>

#### Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice), `bmed_pk_oral_peak`

The time of the peak plasma concentration and the peak concentration of a one-compartment model with first-order absorption after a single oral dose.

$$
t_{max} = \frac{\ln(k_a/k)}{k_a - k},\quad C_{max} = \frac{F D}{V} e^{-k t_{max}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | bioavailable fraction (0 < F <= 1) | - |
| `D` | in | oral dose | kg |
| `V` | in | volume of distribution | m^3 |
| `ka` | in | absorption rate constant | 1/s |
| `k` | in | elimination rate constant | 1/s |
| `t_max` | out | time of the peak | s |
| `C_max` | out | peak concentration | kg/m^3 |

**Applies when:** one compartment; first-order absorption and elimination; ka != k.

```shell
$ biomedical-science solve bmed_pk_oral_peak F=0.8 "D=250 mg" "V=35 L" "ka=1.2 1/h" "k=0.15 1/h"
t_max = 7129.513857 s
C_max = 0.004245697969 kg/m^3
  by: Oral dose: time and height of the peak concentration (tmax, Cmax) (a model calculation, not medical advice)  (dC/dt = 0: t_max = ln(ka/k)/(ka - k); C_max = (F D / V) exp(-k t_max))
  F = 0.8
  D = 0.00025 kg
  V = 0.035 m^3
  ka = 0.0003333333333 1/s
  k = 4.166666667e-05 1/s
  assumes: one compartment; first-order absorption and elimination; ka != k
```

**What the worked example means:** This calculation determines the maximum plasma concentration and its timing for a single oral dose using a one-compartment model. The result, a peak of 0.0042457 kg/m^3 reached at 7129.51 s, indicates when drug exposure is highest. This value depends on the bioavailable fraction 0.8, dose 250 mg, volume of distribution 35 L, absorption rate constant 1.2 1/h, and elimination rate constant 0.15 1/h. It assumes first-order kinetics where absorption and elimination rates differ.

<a id="bmed_pk_nth_dose_iv"></a>

#### Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice), `bmed_pk_nth_dose_iv`

The peak concentration just after the nth of equal IV bolus doses given every tau, and the concentration a time t later, in a one-compartment model (superposition).

$$
C_n(t) = \frac{D}{V}\,\frac{1 - e^{-n k \tau}}{1 - e^{-k \tau}}\,e^{-k t}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | each bolus dose | kg |
| `V` | in | volume of distribution | m^3 |
| `k` | in | elimination rate constant | 1/s |
| `tau` | in | dosing interval | s |
| `n` | in | number of the dose (1 = first) | - |
| `t` | in | time after the nth dose (0 to tau) | s |
| `C_max_n` | out | concentration just after the nth dose | kg/m^3 |
| `C` | out | concentration a time t after the nth dose | kg/m^3 |

**Applies when:** equal doses at equal intervals; linear one-compartment model; 0 <= t <= tau.

```shell
$ biomedical-science solve bmed_pk_nth_dose_iv "D=500 mg" "V=40 L" "k=0.1 1/h" "tau=8 h" n=3 "t=2 h"
C_max_n = 0.02064031853 kg/m^3
C = 0.01689886353 kg/m^3
  by: Repeated IV bolus doses: concentration after the nth dose (a model calculation, not medical advice)  (superposition: C = (D/V) (1 - r^n)/(1 - r) exp(-k t), r = exp(-k tau))
  D = 0.0005 kg
  V = 0.04 m^3
  k = 2.777777778e-05 1/s
  tau = 28800 s
  t = 7200 s
  assumes: equal doses at equal intervals; linear one-compartment model; 0 <= t <= tau
```

**What the worked example means:** This calculation determines the drug concentration in a one-compartment model following repeated intravenous bolus doses. Using the principle of superposition, it computes the peak level immediately after the nth dose, which is 0.0206403 kg/m^3, and the level a time t later, which is 0.0168989 kg/m^3. The result reflects how the body accumulates the drug when equal doses are given at fixed intervals, assuming linear kinetics and instantaneous distribution.

<a id="bmed_pk_steady_state_iv"></a>

#### Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice), `bmed_pk_steady_state_iv`

The steady-state peak, trough and average concentrations and the accumulation factor of a one-compartment model given equal IV bolus doses every tau.

$$
C_{max,ss} = \frac{D/V}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k\tau},\quad \bar C_{ss} = \frac{D}{V k \tau}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | each bolus dose | kg |
| `V` | in | volume of distribution | m^3 |
| `k` | in | elimination rate constant | 1/s |
| `tau` | in | dosing interval | s |
| `C_max_ss` | out | steady-state peak (just after a dose) | kg/m^3 |
| `C_min_ss` | out | steady-state trough (just before a dose) | kg/m^3 |
| `C_avg_ss` | out | average concentration over an interval at steady state | kg/m^3 |
| `R_ac` | out | accumulation factor: steady-state peak over first-dose peak | - |

**Applies when:** equal doses at equal intervals; linear one-compartment model.

```shell
$ biomedical-science solve bmed_pk_steady_state_iv "D=500 mg" "V=40 L" "k=0.1 1/h" "tau=8 h"
C_max_ss = 0.02269957776 kg/m^3
C_min_ss = 0.01019957776 kg/m^3
C_avg_ss = 0.015625 kg/m^3
R_ac = 1.815966221
  by: Repeated IV bolus dosing: steady-state peak, trough, average and accumulation factor (a model calculation, not medical advice)  (geometric series: C_max,ss = (D/V)/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k tau); C_avg = D/(V k tau))
  D = 0.0005 kg
  V = 0.04 m^3
  k = 2.777777778e-05 1/s
  tau = 28800 s
  assumes: equal doses at equal intervals; linear one-compartment model
```

**What the worked example means:** This calculation determines the drug concentrations in plasma once a steady state is reached during repeated intravenous bolus dosing. Given each dose of 500 mg administered every 8 h to a patient with volume of distribution 40 L and elimination rate constant 0.1 1/h, the model computes the steady-state peak, trough, and average levels. The resulting peak concentration is 0.0226996 kg/m^3, which represents the highest level immediately after a dose, while the trough is 0.0101996 kg/m^3, representing the lowest level just before the next dose. The average concentration over the interval is 0.015625 kg/m^3, indicating the overall exposure. Additionally, the accumulation factor is 1.81597, showing how much higher these steady-state levels are compared to the first dose.

<a id="bmed_pk_steady_state_oral"></a>

#### Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice), `bmed_pk_steady_state_oral`

The steady-state peak concentration, the time of the peak after a dose and the trough of a one-compartment model with first-order absorption given equal oral doses every tau.

$$
C_{ss}(t) = \frac{F D k_a}{V (k_a - k)}\left[\frac{e^{-k t}}{1 - e^{-k\tau}} - \frac{e^{-k_a t}}{1 - e^{-k_a \tau}}\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `F` | in | bioavailable fraction (0 < F <= 1) | - |
| `D` | in | each oral dose | kg |
| `V` | in | volume of distribution | m^3 |
| `ka` | in | absorption rate constant | 1/s |
| `k` | in | elimination rate constant | 1/s |
| `tau` | in | dosing interval | s |
| `C_max_ss` | out | steady-state peak | kg/m^3 |
| `t_max_ss` | out | time of the peak after a dose | s |
| `C_min_ss` | out | steady-state trough (at the dosing times) | kg/m^3 |

**Applies when:** equal doses at equal intervals; linear one-compartment model, first-order absorption; ka != k; the steady-state peak lies inside the interval.

```shell
$ biomedical-science solve bmed_pk_steady_state_oral F=0.8 "D=250 mg" "V=35 L" "ka=1.2 1/h" "k=0.15 1/h" "tau=8 h"
C_max_ss = 0.006394744352 kg/m^3
t_max_ss = 5901.006359 s
C_min_ss = 0.002814334873 kg/m^3
  by: Repeated oral dosing: steady-state peak, its time and the trough (a model calculation, not medical advice)  (superposition to steady state: C_ss(t) = F D ka/(V(ka - k)) [exp(-k t)/(1 - exp(-k tau)) - exp(-ka t)/(1 - exp(-ka tau))]; dC/dt = 0 for the peak)
  F = 0.8
  D = 0.00025 kg
  V = 0.035 m^3
  ka = 0.0003333333333 1/s
  k = 4.166666667e-05 1/s
  tau = 28800 s
  assumes: equal doses at equal intervals; linear one-compartment model, first-order absorption; ka != k; the steady-state peak lies inside the interval
```

**What the worked example means:** This calculation determines the maximum and minimum drug concentrations at steady state for repeated oral dosing. Using the bioavailable fraction 0.8, dose 250 mg, volume of distribution 35 L, absorption rate constant 1.2 1/h, elimination rate constant 0.15 1/h, and dosing interval 8 h, it finds that the peak concentration reaches 0.00639474 kg/m^3 at time 5901.01 s after a dose, while the lowest concentration before the next dose is 0.00281433 kg/m^3. This reflects how first-order absorption delays and flattens the concentration curve compared to intravenous administration.

<a id="bmed_pk_intermittent_infusion_steady_state"></a>

#### Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice), `bmed_pk_intermittent_infusion_steady_state`

The steady-state peak (end of infusion) and trough (end of interval) of a one-compartment model given infusions of duration T_inf at rate R0 every tau.

$$
C_{max,ss} = \frac{R_0}{CL}\,\frac{1 - e^{-kT}}{1 - e^{-k\tau}},\quad C_{min,ss} = C_{max,ss}\,e^{-k(\tau - T)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `R0` | in | infusion rate | kg/s |
| `T_inf` | in | duration of each infusion | s |
| `tau` | in | dosing interval | s |
| `CL` | in | clearance | m^3/s |
| `V` | in | volume of distribution | m^3 |
| `C_max_ss` | out | steady-state concentration at the end of an infusion | kg/m^3 |
| `C_min_ss` | out | steady-state concentration at the end of an interval | kg/m^3 |

**Applies when:** one compartment, first-order elimination; T_inf < tau; identical infusions.

```shell
$ biomedical-science solve bmed_pk_intermittent_infusion_steady_state "R0=1000 mg/h" "T_inf=1 h" "tau=8 h" "CL=6 L/h" "V=30 L"
C_max_ss = 0.03785416532 kg/m^3
C_min_ss = 0.009334722242 kg/m^3
  by: Repeated short infusions: steady-state peak and trough (a model calculation, not medical advice)  (C_max,ss = (R0/CL)(1 - exp(-k T))/(1 - exp(-k tau)); C_min,ss = C_max,ss exp(-k(tau - T)), k = CL/V)
  R0 = 2.777777778e-07 kg/s
  T_inf = 3600 s
  tau = 28800 s
  CL = 1.666666667e-06 m^3/s
  V = 0.03 m^3
  assumes: one compartment, first-order elimination; T_inf < tau; identical infusions
```

**What the worked example means:** This calculation determines the highest drug level reached at the end of an infusion, which is 0.0378542 kg/m^3, and the lowest level remaining just before the next dose begins, known as 0.00933472 kg/m^3. These values are derived from the infusion rate of 1000 mg/h, the duration of each infusion set to 1 h, and the dosing interval of 8 h, alongside the patient's clearance of 6 L/h and volume of distribution of 30 L. Understanding these peaks and troughs helps assess whether the drug remains within a therapeutic window throughout the dosing cycle.

<a id="bmed_pk_two_compartment_bolus"></a>

#### Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice), `bmed_pk_two_compartment_bolus`

The hybrid rate constants alpha and beta, the coefficients A and B and the central concentration at time t of a two-compartment model after an IV bolus, from the volume V1 and the micro-constants k10, k12, k21.

$$
C(t) = A e^{-\alpha t} + B e^{-\beta t},\quad \alpha\beta = k_{10}k_{21},\ \alpha + \beta = k_{10} + k_{12} + k_{21}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | bolus dose | kg |
| `V1` | in | central volume | m^3 |
| `k10` | in | elimination rate constant from the central compartment | 1/s |
| `k12` | in | transfer rate constant central to peripheral | 1/s |
| `k21` | in | transfer rate constant peripheral to central | 1/s |
| `t` | in | time after the dose | s |
| `alpha` | out | fast (distribution) rate constant | 1/s |
| `beta` | out | slow (terminal) rate constant | 1/s |
| `A` | out | coefficient of exp(-alpha t) | kg/m^3 |
| `B` | out | coefficient of exp(-beta t) | kg/m^3 |
| `C` | out | central concentration at time t | kg/m^3 |

**Applies when:** linear two-compartment model, elimination from the central compartment; alpha > beta.

```shell
$ biomedical-science solve bmed_pk_two_compartment_bolus "D=100 mg" "V1=10 L" "k10=0.2 1/h" "k12=0.5 1/h" "k21=0.3 1/h" "t=4 h"
alpha = 0.0002599694151 1/s
beta = 1.780836268e-05 1/s
A = 0.007294157339 kg/m^3
B = 0.002705842661 kg/m^3
C = 0.00226643452 kg/m^3
  by: Two-compartment IV bolus: macro-constants and concentration from micro-constants (a model calculation, not medical advice)  (alpha, beta roots of x^2 - (k10 + k12 + k21) x + k10 k21 = 0; A = (D/V1)(alpha - k21)/(alpha - beta), B = (D/V1)(k21 - beta)/(alpha - beta))
  D = 0.0001 kg
  V1 = 0.01 m^3
  k10 = 5.555555556e-05 1/s
  k12 = 0.0001388888889 1/s
  k21 = 8.333333333e-05 1/s
  t = 14400 s
  assumes: linear two-compartment model, elimination from the central compartment; alpha > beta
```

**What the worked example means:** This calculation determines the drug concentration in the central compartment at time t following an intravenous bolus. It uses the dose, central volume, and micro-rate constants to compute the hybrid rate constants alpha and beta, along with their corresponding coefficients A and B. The resulting concentration 0.00226643 kg/m^3 reflects the biexponential decay characteristic of a two-compartment model, where distribution and elimination processes occur simultaneously but at different rates.

<a id="bmed_pk_two_compartment_micro_constants"></a>

#### Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice), `bmed_pk_two_compartment_micro_constants`

The central volume, the micro rate constants k10, k12, k21, the clearance and the steady-state volume of a two-compartment model from the biexponential fit C = A exp(-alpha t) + B exp(-beta t) of an IV bolus.

$$
k_{21} = \frac{A\beta + B\alpha}{A + B},\quad k_{10} = \frac{\alpha\beta}{k_{21}},\quad k_{12} = \alpha + \beta - k_{21} - k_{10}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `D` | in | bolus dose | kg |
| `A` | in | coefficient of the fast exponential | kg/m^3 |
| `alpha` | in | fast rate constant | 1/s |
| `B` | in | coefficient of the slow exponential | kg/m^3 |
| `beta` | in | slow rate constant | 1/s |
| `V1` | out | central volume | m^3 |
| `k10` | out | elimination rate constant | 1/s |
| `k12` | out | central to peripheral | 1/s |
| `k21` | out | peripheral to central | 1/s |
| `CL` | out | clearance | m^3/s |
| `Vss` | out | volume of distribution at steady state | m^3 |

**Applies when:** IV bolus, linear two-compartment model, elimination from the central compartment; alpha > beta > 0, A > 0, B > 0.

```shell
$ biomedical-science solve bmed_pk_two_compartment_micro_constants "D=100 mg" "A=6 mg/L" "alpha=1.5 1/h" "B=2 mg/L" "beta=0.15 1/h"
V1 = 0.0125 m^3
k10 = 0.0001282051282 1/s
k12 = 0.0001947115385 1/s
k21 = 0.0001354166667 1/s
CL = 1.602564103e-06 m^3/s
Vss = 0.03047337278 m^3
  by: Two-compartment model: micro-constants, clearance and Vss from a biexponential curve (a model calculation, not medical advice)  (V1 = D/(A + B); k21 = (A beta + B alpha)/(A + B); k10 = alpha beta / k21; k12 = alpha + beta - k21 - k10; CL = k10 V1; Vss = V1 (1 + k12/k21))
  D = 0.0001 kg
  A = 0.006 kg/m^3
  alpha = 0.0004166666667 1/s
  B = 0.002 kg/m^3
  beta = 4.166666667e-05 1/s
  assumes: IV bolus, linear two-compartment model, elimination from the central compartment; alpha > beta > 0, A > 0, B > 0
```

**What the worked example means:** This calculation derives physiological parameters from the biexponential fit of plasma concentration data after an intravenous bolus. The central volume, denoted as V1, is calculated by dividing the dose D by the sum of the fast coefficient A and slow coefficient B, resulting in 0.0125 m^3. The micro rate constants are then determined: k21, representing transfer from peripheral to central compartments, is 0.000135417 1/s; k10, the elimination rate constant, is 0.000128205 1/s; and k12, for transfer from central to peripheral, is 0.000194712 1/s. These values allow computation of total clearance CL as 1.60256e-06 m^3/s and the volume of distribution at steady state Vss as 0.0304734 m^3, providing insight into how the drug distributes and is removed from the body.

### Survival analysis (8)

<a id="bmed_cox_hazard_ratio"></a>

#### Cox proportional-hazards model with one covariate: hazard ratio (Efron ties), `bmed_cox_hazard_ratio`

The log hazard ratio, hazard ratio, its standard error and Wald confidence interval of a Cox proportional-hazards model with a single covariate, by maximum partial likelihood with Efron's handling of ties.

$$
\ell(\beta) = \sum_j\left[\beta\sum_{i\in D_j}x_i - \sum_{l=0}^{d_j-1}\ln\left(\sum_{R_j}e^{\beta x} - \tfrac{l}{d_j}\sum_{D_j}e^{\beta x}\right)\right]
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times | - |
| `events` | in | 1 = event, 0 = censored | - |
| `x` | in | covariate value of each subject (e.g. 1 = treated, 0 = control) | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `beta` | out | log hazard ratio per unit of x | - |
| `HR` | out | hazard ratio exp(beta) | - |
| `se` | out | standard error of beta | - |
| `HR_lower` | out | lower Wald limit of the hazard ratio | - |
| `HR_upper` | out | upper Wald limit of the hazard ratio | - |

**Applies when:** proportional hazards; Efron approximation for tied event times; a finite maximum (no monotone likelihood); level stated.

```shell
$ biomedical-science solve bmed_cox_hazard_ratio "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" "x=[1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1]" level=0.95
beta = 0.04732279296
HR = 1.04846039
se = 0.6738763833
HR_lower = 0.2798642406
HR_upper = 3.927865837
  by: Cox proportional-hazards model with one covariate: hazard ratio (Efron ties)  (Newton-Raphson on the Efron partial log-likelihood (step halving); SE from the observed information)
  level = 0.95
  assumes: proportional hazards; Efron approximation for tied event times; a finite maximum (no monotone likelihood); level stated
```

**What the worked example means:** This calculation determines the effect of the covariate on the event rate using a Cox proportional-hazards model. The log hazard ratio, or beta, was estimated as 0.0473228, which translates to a hazard ratio of 1.04846. This means that for each unit increase in the covariate, the instantaneous risk of the event changes by this factor. The standard error of the estimate is 0.673876, and the Wald confidence interval at the specified level ranges from 0.279864 to 3.92787. If the interval excludes one, the association is statistically significant.

<a id="bmed_exponential_survival_fit"></a>

#### Exponential survival model: hazard rate, mean and median survival (maximum likelihood), `bmed_exponential_survival_fit`

The maximum-likelihood constant hazard of an exponential survival model from censored follow-up data, with the implied mean and median survival times.

$$
\hat\lambda = \frac{d}{\sum_i t_i},\quad \tilde t = \frac{\ln 2}{\hat\lambda}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times, all positive (one time unit) | - |
| `events` | in | 1 = event, 0 = censored | - |
| `hazard` | out | events per unit time | - |
| `mean` | out | mean survival 1/hazard | - |
| `median` | out | median survival ln 2 / hazard | - |

**Applies when:** constant hazard; non-informative censoring; at least one event.

```shell
$ biomedical-science solve bmed_exponential_survival_fit "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]"
hazard = 0.02506963788
mean = 39.88888889
median = 27.64887087
  by: Exponential survival model: hazard rate, mean and median survival (maximum likelihood)  (hazard = (number of events) / (total follow-up time))
  assumes: constant hazard; non-informative censoring; at least one event
```

**What the worked example means:** This calculation determines the maximum likelihood estimate for the hazard rate in an exponential survival model using observed follow-up times and event indicators. The hazard rate, defined as the number of events divided by the total accumulated follow-up time, is found to be 0.0250696. Based on this constant hazard assumption, the expected mean survival time is calculated as the reciprocal of the hazard, resulting in 39.8889, while the median survival time is derived from the natural logarithm of two divided by the hazard, yielding 27.6489. These values summarize the central tendency and spread of the survival distribution under the specified model.

<a id="bmed_exponential_hazard_ratio"></a>

#### Hazard ratio of two exponential survival groups with Wald interval, `bmed_exponential_hazard_ratio`

The ratio of the maximum-likelihood constant hazards of two groups (group 1 relative to group 2) with the log-scale Wald confidence interval.

$$
HR = \frac{d_1/\sum t_{1}}{d_2/\sum t_{2}},\quad \mathrm{SE}(\ln HR) = \sqrt{\tfrac1{d_1} + \tfrac1{d_2}}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times1` | in | follow-up times, group 1 (positive) | - |
| `events1` | in | 1 = event, 0 = censored, group 1 | - |
| `times2` | in | follow-up times, group 2 (reference) | - |
| `events2` | in | 1 = event, 0 = censored, group 2 | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `HR` | out | hazard ratio group 1 / group 2 | - |
| `lower` | out | lower limit | - |
| `upper` | out | upper limit | - |

**Applies when:** constant hazard in each group; at least one event per group; level stated.

```shell
$ biomedical-science solve bmed_exponential_hazard_ratio "times1=[6, 6, 6, 7, 10, 13, 16, 22, 23]" "events1=[1, 1, 1, 1, 1, 1, 1, 1, 1]" "times2=[6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events2=[1, 0, 1, 0, 0, 1, 0, 0, 1, 0, 0, 0]" level=0.95
HR = 5.160550459
lower = 1.589232949
upper = 16.75731746
  by: Hazard ratio of two exponential survival groups with Wald interval  (hazard_g = d_g / sum t_g; SE(ln HR) = sqrt(1/d1 + 1/d2))
  level = 0.95
  assumes: constant hazard in each group; at least one event per group; level stated
```

**What the worked example means:** This calculation determines the relative risk of an event occurring in group one compared to group two, assuming a constant hazard rate over time. The result is derived by dividing the number of events in the first group divided by their total follow-up time by the equivalent ratio for the second group. For the example provided, the computed hazard ratio is 5.16055, indicating that the event rate in the first group is this multiple of the rate in the reference group. A value near one suggests similar risks, while values significantly different from one indicate a disparity in survival outcomes between the two cohorts.

<a id="bmed_km_survival_median"></a>

#### Kaplan-Meier survival estimate at a time and median survival, `bmed_km_survival_median`

The Kaplan-Meier (product-limit) estimate of the survival probability at time t and the median survival time from follow-up times with event or censoring indicators.

$$
\hat S(t) = \prod_{u_i \le t}\left(1 - \frac{d_i}{n_i}\right)
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times (one time unit; results in that unit) | - |
| `events` | in | 1 = event observed, 0 = censored, one per time | - |
| `t` | in | time at which S(t) is reported | - |
| `S_t` | out | Kaplan-Meier survival probability at t | - |
| `median` | out | median survival: first time with S <= 0.5 (none when not reached) | - |

**Applies when:** non-informative censoring; a censoring at an event time counts as at risk at that time.

```shell
$ biomedical-science solve bmed_km_survival_median "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" t=12
S_t = 0.7529411765
median = 23
  by: Kaplan-Meier survival estimate at a time and median survival  (S(t) = product over event times u <= t of (1 - d/n), exact fractions; median = first event time with S <= 1/2)
  t = 12
  assumes: non-informative censoring; a censoring at an event time counts as at risk at that time
```

**What the worked example means:** This calculation determines the Kaplan-Meier survival estimate for a specific time point based on the provided follow-up data. Multiplying the conditional probabilities of surviving each event interval up to time 12 gives a survival probability of 0.752941. Additionally, the median survival time, defined as the first time point where the survival curve drops to or below one half, is identified as 23. These values summarize the proportion of subjects remaining free of the event at the queried time and the central tendency of survival duration.

<a id="bmed_km_confidence_interval"></a>

#### Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval, `bmed_km_confidence_interval`

The Kaplan-Meier survival estimate at time t with its confidence interval on the log(-log) scale (Greenwood variance; the exponential Greenwood interval) at the stated level.

$$
\hat S^{\exp\left(\pm z\,\sqrt{V}/|\ln\hat S|\right)},\quad V = \sum_{u_i \le t}\frac{d_i}{n_i(n_i - d_i)}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times (one time unit) | - |
| `events` | in | 1 = event, 0 = censored | - |
| `t` | in | time at which S(t) is reported | - |
| `level` | in | confidence level (95% or 0.95) | - |
| `S_t` | out | Kaplan-Meier estimate at t | - |
| `lower` | out | lower confidence limit | - |
| `upper` | out | upper confidence limit | - |

**Applies when:** non-informative censoring; an event at or before t and S(t) > 0; level stated.

```shell
$ biomedical-science solve bmed_km_confidence_interval "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" t=12 level=0.95
S_t = 0.7529411765
lower = 0.5031995108
upper = 0.8893618352
  by: Kaplan-Meier survival with log(-log) (exponential Greenwood) confidence interval  (Greenwood sum V = sum d/(n(n - d)); limits S^exp(+-z sqrt(V)/|ln S|))
  t = 12
  level = 0.95
  assumes: non-informative censoring; an event at or before t and S(t) > 0; level stated
```

**What the worked example means:** This calculation determines the Kaplan-Meier survival estimate at time 12, which is 0.752941. The confidence interval, ranging from 0.5032 to 0.889362, is constructed using the exponential Greenwood method. This approach applies a log-minus-log transformation to the variance of the survival estimate, ensuring that the resulting limits remain within the valid probability range of zero to one while reflecting the uncertainty associated with the observed events and censoring.

<a id="bmed_logrank_test"></a>

#### Log-rank test comparing two survival curves, `bmed_logrank_test`

The log-rank (Mantel-Cox) chi-square statistic and p-value for equal survival in two groups from follow-up times and event indicators.

$$
\chi^2 = \frac{\left(\sum_i (d_{1i} - e_{1i})\right)^2}{\sum_i v_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times1` | in | follow-up times, group 1 | - |
| `events1` | in | 1 = event, 0 = censored, group 1 | - |
| `times2` | in | follow-up times, group 2 | - |
| `events2` | in | 1 = event, 0 = censored, group 2 | - |
| `chi_square` | out | log-rank chi-square (1 degree of freedom) | - |
| `p_value` | out | two-sided p-value | - |

**Applies when:** non-informative censoring; hypergeometric variance at each event time.

```shell
$ biomedical-science solve bmed_logrank_test "times1=[6, 6, 6, 7, 10, 13, 16, 22, 23]" "events1=[1, 1, 1, 1, 1, 1, 1, 1, 1]" "times2=[1, 1, 2, 2, 3, 4, 4, 5, 5, 8]" "events2=[1, 1, 1, 1, 1, 1, 1, 1, 1, 1]"
chi_square = 13.84998162
p_value = 0.0001979986732
  by: Log-rank test comparing two survival curves  (at each event time: expected events of group 1 and the hypergeometric variance; chi2 = (O1 - E1)^2 / V)
  assumes: non-informative censoring; hypergeometric variance at each event time
```

**What the worked example means:** This calculation compares the observed number of events against those expected under the null hypothesis of equal survival for each time point where an event occurs. By aggregating these differences across all event times, the method produces a chi-square statistic of 13.85 and a corresponding p-value of 0.000197999. A low p-value suggests that the survival curves differ significantly, implying that the timing of events is not random with respect to group membership, while a high p-value indicates no statistically significant difference in survival distributions between the two groups based on the provided follow-up data.

<a id="bmed_nelson_aalen_hazard"></a>

#### Nelson-Aalen cumulative hazard at a time: `bmed_nelson_aalen_hazard`

The Nelson-Aalen estimate of the cumulative hazard at time t from follow-up times with event or censoring indicators.

$$
\hat H(t) = \sum_{u_i \le t}\frac{d_i}{n_i}
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times (one time unit) | - |
| `events` | in | 1 = event, 0 = censored | - |
| `t` | in | time at which H(t) is reported | - |
| `H_t` | out | cumulative hazard at t | - |

**Applies when:** non-informative censoring; tied events contribute d/n (no tie smoothing).

```shell
$ biomedical-science solve bmed_nelson_aalen_hazard "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" t=12
H_t = 0.2683473389
  by: Nelson-Aalen cumulative hazard at a time  (H(t) = sum over event times u <= t of d/n)
  t = 12
  assumes: non-informative censoring; tied events contribute d/n (no tie smoothing)
```

**What the worked example means:** This calculation determines the cumulative hazard function value at a specific time point using the Nelson-Aalen estimator. It sums the ratio of observed events to the number of individuals at risk for every event time up to and including 12. The resulting value, 0.268347, represents the total accumulated risk experienced by the cohort until that moment in survival analysis.

<a id="bmed_restricted_mean_survival_time"></a>

#### Restricted mean survival time from the Kaplan-Meier curve, `bmed_restricted_mean_survival_time`

The restricted mean survival time up to tau: the area under the Kaplan-Meier survival curve from 0 to tau.

$$
\mathrm{RMST}(\tau) = \int_0^{\tau}\hat S(t)\,dt
$$

| symbol |  | meaning | SI unit |
|---|---|---|---|
| `times` | in | follow-up times (one time unit) | - |
| `events` | in | 1 = event, 0 = censored | - |
| `tau` | in | restriction time (in the unit of the times) | - |
| `RMST` | out | restricted mean survival time, in the time unit | - |

**Applies when:** non-informative censoring; the Kaplan-Meier step function integrated exactly.

```shell
$ biomedical-science solve bmed_restricted_mean_survival_time "times=[6, 6, 6, 7, 10, 13, 16, 22, 23, 6, 9, 10, 11, 17, 19, 20, 25, 32, 32, 34, 35]" "events=[1, 1, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]" tau=20
RMST = 16.11652661
  by: Restricted mean survival time from the Kaplan-Meier curve  (RMST = integral from 0 to tau of the Kaplan-Meier step function)
  tau = 20
  assumes: non-informative censoring; the Kaplan-Meier step function integrated exactly
```

**What the worked example means:** This calculation determines the average survival time up to a specific cutoff point, known as tau. It represents the area under the Kaplan-Meier curve from time zero to tau, providing a clinically interpretable measure of survival benefit that does not rely on proportional hazards assumptions. The resulting value of 16.1165 indicates the expected time lived within the restriction window defined by 20.

Questions and reports: support@ai999x.com. AI999X, The Future Is Self-Improving.
