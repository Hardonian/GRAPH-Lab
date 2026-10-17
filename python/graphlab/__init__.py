"""graphlab Python control plane.

Independent oracles + orchestration. The Rust engine is never trusted on
its own: EXACT_FINITE claims must agree with networkx AND igraph on small
instances before the label is earned.
"""

__version__ = "0.1.0"
