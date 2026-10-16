//! graphlab-core — exact, deterministic graph computation for research.
//!
//! Layering (see docs/ARCHITECTURE.md):
//! - `graph`   : compact undirected simple graph on bitset adjacency
//! - `graph6`  : lossless graph6 encode/decode with strict validation
//! - `canon`   : canonical labeling / isomorphism
//! - `invariants` : exact graph invariants
//!
//! Every public routine is exact integer/combinatorial logic. Floating point
//! is confined to heuristic candidate scoring and must be labelled as such.

pub mod graph;

pub use graph::{Graph, GraphError, MAX_VERTICES};
