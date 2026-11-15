# Biomedical Science (biomedical-science-ai999x)

[![tests](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml/badge.svg)](https://github.com/AI999X/biomedical-science-ai999x/actions/workflows/ci.yml) [![PyPI](https://img.shields.io/pypi/v/biomedical-science-ai999x)](https://pypi.org/project/biomedical-science-ai999x/) [![Hugging Face](https://img.shields.io/badge/%F0%9F%A4%97-Hugging%20Face-yellow)](https://huggingface.co/AI999X/biomedical-science-ai999x)

Biomedical Science covers pharmacokinetics and dosing models, epidemic models, diagnostic test statistics, survival analysis, biomechanics, neuron models and medical-imaging physics (quantitative only, no clinical advice): 96 calculations in 10 topics, each with its formula, its steps and its units, and a plain reason instead of a number when an input is missing or out of range. One compiled engine, four ways to use it: the desktop app, the command line, the browser and Python. Everything runs on your own computer, offline.

[![The Biomedical Science desktop app: Infusion of fixed duration: concentration during and after it (a model calculation, not medical advice), with its worked example and live scene](https://www.ai999x.com/?ax_shot=biomedical-science-ai999x/hero.webp)](https://www.ai999x.com/models/biomedical-science-ai999x/)

## Four ways to use it

| | |
|---|---|
| **Desktop app** (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-linux-x86_64.tar.gz) |
| **Command line** `biomedical-science` (Windows, Linux) | [Windows](https://downloads.ai999x.com/desktop/latest/ai999x-cli-windows-x86_64.zip) · [Linux](https://downloads.ai999x.com/desktop/latest/ai999x-cli-linux-x86_64.tar.gz) |
| **Browser** (any device) | [Try it on the model page](https://www.ai999x.com/models/biomedical-science-ai999x/#try) |
| **Python** 3.10+ | `pip install biomedical-science-ai999x`: on Windows and Linux (x86-64) `pip` installs the same compiled engine; on other systems (macOS, ARM) it installs a pure-Python build that gives the same answers |

## Example

```python
import biomedical_science_ai999x as biomedical_science
a = biomedical_science.ask("IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h")
print(a.outcome, a.value, a.unit)
```

Output:

```text
SOLVED 0.00686014545118 kg/m^3
```

Every calculation, with its formula, a worked example and its figures: **[Biomedical Science on ai999x.com](https://www.ai999x.com/models/biomedical-science-ai999x/)**. Install guides for the app and the command line: [https://www.ai999x.com/desktop-cli/](https://www.ai999x.com/desktop-cli/).

Version 1.6.3 · License: Apache-2.0 · Author: Ibnul Jaif Farabi · Organization: AI999X · Contact: support@ai999x.com · [Support the project](https://buymeacoffee.com/ibnuljaiffarabi)

*The Future Is Self-Improving.*

## Source and releases

This repository holds the installable source of each release, exactly as published on [PyPI](https://pypi.org/project/biomedical-science-ai999x/) (`pip install .` builds it). Releases are tagged `v1.6.3`; the [GitHub release](https://github.com/AI999X/biomedical-science-ai999x/releases/tag/v1.6.3) carries the wheel, the desktop app and the command-line program for Windows and Linux, and their checksums.

## Build the program from source

`rust/` holds the Rust source of the `biomedical-science` command-line program and browser module: the AI999X engine (`rust/ax-engine`), this model (`rust/biomedical-science`) and its data (`rust/models/biomedical_science/pack.json`). With Rust 1.80 or later:

```bash
cd rust
cargo build --release -p biomedical-science-ai999x
./target/release/biomedical-science --help
cargo test -p biomedical-science-ai999x
```
