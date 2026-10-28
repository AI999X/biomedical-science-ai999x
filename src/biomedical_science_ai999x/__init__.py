"""biomedical-science-ai999x — offline biomedical science model calculations with their formula, steps and verification.

NOT MEDICAL ADVICE. This package performs quantitative textbook model calculations only. It gives no diagnosis, no
treatment advice and no dosing recommendation for any real patient, and must not be used for clinical decisions.

    import biomedical_science_ai999x as m
    m.ask("IV bolus concentration: D = 500 mg, V = 40 L, k = 0.1 1/h, t = 6 h").show()
    m.ask("Kaplan-Meier survival: times = [2, 3, 3, 5, 8, 9, 12, 15], events = [1, 0, 1, 1, 0, 1, 1, 0], t = 6").show()
    m.solve("bmed_epi_sir_final_size", R0=2.5, s0=0.99, i0=0.01)
    m.capabilities()                   # every certified capability

Covers pharmacokinetic models (one-compartment bolus, infusion and oral absorption, repeated dosing to steady state,
Michaelis-Menten elimination, two-compartment models, hepatic clearance and effect-site models, non-compartmental
analysis), epidemic models (SIR/SEIR final size, SIR peak, SEIR growth rate, R from a growth rate, SIS, Reed-Frost),
diagnostic and epidemiological statistics (ROC area, Youden cut-off, sensitivity and specificity with Wilson
intervals, odds ratio, relative risk, NNT, rate ratio, SMR, Mantel-Haenszel, Cohen's kappa), survival analysis
(Kaplan-Meier with median and interval, Nelson-Aalen, log-rank, Cox and exponential hazard ratios, RMST),
biomechanics (elbow lever model, Poiseuille flow, Windkessel models, viscoelastic tissue, indicator dilution,
respiratory mechanics), neuron models (GHK voltage and current, integrate-and-fire, cable, Hodgkin-Huxley,
postsynaptic potential, FitzHugh-Nagumo) and medical-imaging physics (X-ray attenuation, MRI signals, ultrasound
interfaces and matching layers, Gaussian PSF/MTF, ADC and T2 fits). Questions use one documented form,
"<capability phrase>: name = value, ...", with units on physical values; a question that is not fully determined is
refused rather than guessed. Built by the AI999X model machine from the same engine as every domain; every capability
was checked against independent computations before release.

Creator / engineer: Ibnul Jaif Farabi. Company: AI999X.
"""
from __future__ import annotations

from pathlib import Path

try:                                       # the installed package carries its own copy of the engine
    from ._engine import Answer
    from ._engine.product import Product
except ImportError:                        # the factory's source tree, during development
    from ai999x_factory.runtime import Answer
    from ai999x_factory.runtime.product import Product

__version__ = "1.0.2"
_P = Product("biomedical_science", [Path(__file__).resolve().parent / "pack", Path(__file__).resolve().parents[3]])
pack, ask, solve, compile_problem = _P.pack, _P.ask, _P.solve, _P.compile_problem
formula, formulas, quantities, conditions, capabilities = _P.formula, _P.formulas, _P.quantities, _P.conditions, _P.capabilities
examples, family_examples, structured_example, to_si = _P.examples, _P.family_examples, _P.structured_example, _P.to_si
templates, procedure_examples = _P.templates, _P.procedure_examples
prove, derivations, about = _P.prove, _P.derivations, (lambda: {**_P.about(), "version": __version__})
knowledge, lookup, related = _P.knowledge, _P.lookup, _P.related
plot, plot_relation, search = _P.plot, _P.plot_relation, _P.search   # figures (SVG) and capability search ([search])
__all__ = ["ask", "solve", "prove", "derivations", "about", "formula", "formulas", "quantities", "conditions", "capabilities",
           "compile_problem", "pack", "Answer", "plot", "plot_relation", "search", "__version__"]
