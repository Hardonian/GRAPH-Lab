//! Compact undirected simple graph with bitset adjacency.
//!
//! Storage: rows of `n` bits packed into `u64` words. n <= 64 keeps the
//! representation single-word-per-row and cache friendly; larger graphs are
//! a deliberate future extension (see docs/STATUS.md).
//!
//! Invariants enforced by the constructor:
//! - no self-loops
//! - symmetry (adjacency matrix equals its transpose)
//! - vertex count consistent with the bit width used

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Maximum supported vertex count in this engine revision.
pub const MAX_VERTICES: usize = 64;

#[derive(Debug, Clone, PartialEq, Eq, Error)]
pub enum GraphError {
    #[error("vertex count {0} exceeds supported maximum {MAX_VERTICES}")]
    TooManyVertices(usize),
    #[error("vertex count {0} is negative or zero-padded invalid")]
    InvalidVertexCount(i64),
    #[error("edge endpoint {0} out of range for graph with {1} vertices")]
    VertexOutOfRange(usize, usize),
    #[error("self-loop on vertex {0} is not allowed in a simple graph")]
    SelfLoop(usize),
    #[error("invalid graph6 input: {0}")]
    InvalidGraph6(String),
    #[error("edge list has {0} entries but expected {1} for encoded graph")]
    EdgeCountMismatch(usize, usize),
}

#[derive(Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Graph {
    /// Number of vertices.
    n: usize,
    /// Adjacency bits: row-major, `n` bits per row, one u64 per row (n<=64).
    /// Bit k of `rows[i]` = edge (i, k).
    rows: Vec<u64>,
}

impl std::fmt::Debug for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Graph(n={}, m={})", self.n, self.edge_count())
    }
}

impl Graph {
    /// Empty graph on `n` vertices (no edges).
    pub fn empty(n: usize) -> Result<Self, GraphError> {
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        Ok(Self {
            n,
            rows: vec![0u64; n],
        })
    }

    /// Cocktail party graph CP(t) on 2t vertices: complete graph K_{2t} minus
    /// the perfect matching {(2i, 2i+1) : i < t}.
    pub fn cocktail_party(t: usize) -> Result<Self, GraphError> {
        let n = 2 * t;
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        for i in 0..n {
            for j in (i + 1)..n {
                if !(i / 2 == j / 2 && i % 2 != j % 2) {
                    g.add_edge(i, j)?;
                }
            }
        }
        Ok(g)
    }

    /// Complete bipartite graph K_{a, b} on a + b vertices.
    /// Vertices 0..a form part A; vertices a..(a+b) form part B.
    pub fn complete_bipartite(a: usize, b: usize) -> Result<Self, GraphError> {
        let n = a + b;
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        for u in 0..a {
            for v in a..n {
                g.add_edge(u, v)?;
            }
        }
        Ok(g)
    }

    /// Complete multipartite graph K_{n_1, ..., n_k}.
    /// Vertices are indexed sequentially by parts.
    pub fn complete_multipartite(parts: &[usize]) -> Result<Self, GraphError> {
        let n: usize = parts.iter().sum();
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        let mut part_starts = Vec::with_capacity(parts.len());
        let mut curr = 0;
        for &sz in parts {
            part_starts.push(curr);
            curr += sz;
        }
        for (i, &sz_i) in parts.iter().enumerate() {
            let start_i = part_starts[i];
            for (j, &sz_j) in parts.iter().enumerate().skip(i + 1) {
                let start_j = part_starts[j];
                for u in start_i..(start_i + sz_i) {
                    for v in start_j..(start_j + sz_j) {
                        g.add_edge(u, v)?;
                    }
                }
            }
        }
        Ok(g)
    }

    /// Complete tripartite graph K_{a, b, c}.
    pub fn complete_tripartite(a: usize, b: usize, c: usize) -> Result<Self, GraphError> {
        Self::complete_multipartite(&[a, b, c])
    }

    /// Turán graph T(n, r): complete r-partite graph on n vertices whose partition
    /// sizes are as equal as possible (differing by at most 1).
    pub fn turan(n: usize, r: usize) -> Result<Self, GraphError> {
        if r == 0 {
            return Err(GraphError::InvalidVertexCount(0));
        }
        let n_tot = n;
        if n_tot > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n_tot));
        }
        let q = n / r;
        let s = n % r;
        let mut parts = Vec::with_capacity(r);
        for i in 0..r {
            parts.push(if i < s { q + 1 } else { q });
        }
        Self::complete_multipartite(&parts)
    }

    /// 2D Grid graph P_a □ P_b on a * b vertices (Cartesian product of paths P_a and P_b).
    pub fn grid_2d(a: usize, b: usize) -> Result<Self, GraphError> {
        let n = a
            .checked_mul(b)
            .ok_or(GraphError::TooManyVertices(usize::MAX))?;
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        for i in 0..a {
            for j in 0..b {
                let u = i * b + j;
                if j + 1 < b {
                    g.add_edge(u, u + 1)?;
                }
                if i + 1 < a {
                    g.add_edge(u, u + b)?;
                }
            }
        }
        Ok(g)
    }

    /// Cylinder graph C_a □ P_b on a * b vertices (Cartesian product of cycle C_a and path P_b).
    pub fn cylinder(a: usize, b: usize) -> Result<Self, GraphError> {
        if a < 3 {
            return Err(GraphError::InvalidVertexCount(a as i64));
        }
        let n = a
            .checked_mul(b)
            .ok_or(GraphError::TooManyVertices(usize::MAX))?;
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        for i in 0..a {
            for j in 0..b {
                let u = i * b + j;
                if j + 1 < b {
                    g.add_edge(u, u + 1)?;
                }
                let next_i = (i + 1) % a;
                let v = next_i * b + j;
                g.add_edge(u, v)?;
            }
        }
        Ok(g)
    }

    /// Torus Grid graph C_a □ C_b on a * b vertices (Cartesian product of cycles C_a and C_b, where a, b >= 3).
    pub fn torus_grid(a: usize, b: usize) -> Result<Self, GraphError> {
        if a < 3 || b < 3 {
            return Err(GraphError::InvalidVertexCount(a.min(b) as i64));
        }
        let n = a
            .checked_mul(b)
            .ok_or(GraphError::TooManyVertices(usize::MAX))?;
        if n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(n));
        }
        let mut g = Self::empty(n)?;
        for i in 0..a {
            for j in 0..b {
                let u = i * b + j;
                let next_j = (j + 1) % b;
                let v_j = i * b + next_j;
                if u < v_j {
                    g.add_edge(u, v_j)?;
                }
                let next_i = (i + 1) % a;
                let v_i = next_i * b + j;
                if u < v_i {
                    g.add_edge(u, v_i)?;
                }
            }
        }
        Ok(g)
    }

    /// Hypercube graph Q_d on 2^d vertices (Hamming cube of dimension d).
    pub fn hypercube(d: usize) -> Result<Self, GraphError> {
        if d > 6 {
            return Err(GraphError::TooManyVertices(1 << d));
        }
        let n = 1 << d;
        let mut g = Self::empty(n)?;
        for u in 0..n {
            for bit in 0..d {
                let v = u ^ (1 << bit);
                if u < v {
                    g.add_edge(u, v)?;
                }
            }
        }
        Ok(g)
    }

    /// Apex graph G * K_1: adds a universal vertex adjacent to all existing vertices.
    pub fn apex(&self) -> Result<Self, GraphError> {
        let n = self.n;
        let new_n = n + 1;
        if new_n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(new_n));
        }
        let mut g = Self::empty(new_n)?;
        for u in 0..n {
            for v in (u + 1)..n {
                if self.has_edge(u, v) {
                    g.add_edge(u, v)?;
                }
            }
            g.add_edge(u, n)?;
        }
        Ok(g)
    }

    /// Construct a new graph on n + 1 vertices by adding vertex n with the given neighbor mask.
    pub fn extend_vertex(&self, neighbor_mask: u64) -> Result<Self, GraphError> {
        let n = self.n;
        let new_n = n + 1;
        if new_n > MAX_VERTICES {
            return Err(GraphError::TooManyVertices(new_n));
        }
        let mut rows = Vec::with_capacity(new_n);
        for u in 0..n {
            let bit = ((neighbor_mask >> u) & 1) << n;
            rows.push(self.rows[u] | bit);
        }
        rows.push(neighbor_mask & ((1u64 << n) - 1));
        Ok(Self { n: new_n, rows })
    }

    /// Build from an edge list. Edges are validated; duplicates are merged.
    pub fn from_edges(n: usize, edges: &[(usize, usize)]) -> Result<Self, GraphError> {
        let mut g = Self::empty(n)?;
        for &(a, b) in edges {
            g.add_edge(a, b)?;
        }
        Ok(g)
    }

    /// Build from a full n x n adjacency matrix (must be symmetric, zero diagonal).
    pub fn from_adjacency(matrix: &[Vec<bool>]) -> Result<Self, GraphError> {
        let n = matrix.len();
        let mut g = Self::empty(n)?;
        for (i, row) in matrix.iter().enumerate() {
            if row.len() != n {
                return Err(GraphError::InvalidGraph6(format!(
                    "adjacency row {i} has width {}, expected {n}",
                    row.len()
                )));
            }
            for (j, &bit) in row.iter().enumerate() {
                if bit {
                    if i == j {
                        return Err(GraphError::SelfLoop(i));
                    }
                    if !matrix[j][i] {
                        return Err(GraphError::InvalidGraph6(format!(
                            "asymmetric adjacency at ({i},{j})"
                        )));
                    }
                    if i < j {
                        // set once; the mirror will be skipped by i<j filter
                        let _ = g.set_bit(i, j);
                    }
                }
            }
        }
        Ok(g)
    }

    #[inline]
    fn set_bit(&mut self, i: usize, j: usize) -> Result<(), GraphError> {
        if i >= self.n || j >= self.n {
            return Err(GraphError::VertexOutOfRange(i.max(j), self.n));
        }
        if i == j {
            return Err(GraphError::SelfLoop(i));
        }
        self.rows[i] |= 1u64 << j;
        self.rows[j] |= 1u64 << i;
        Ok(())
    }

    pub fn add_edge(&mut self, a: usize, b: usize) -> Result<(), GraphError> {
        self.set_bit(a, b)
    }

    #[inline]
    pub fn has_edge(&self, a: usize, b: usize) -> bool {
        if a >= self.n || b >= self.n {
            return false;
        }
        (self.rows[a] >> b) & 1 == 1
    }

    #[inline]
    pub fn num_vertices(&self) -> usize {
        self.n
    }

    #[inline]
    pub fn adjacency_row(&self, v: usize) -> u64 {
        self.rows[v]
    }

    pub fn edge_count(&self) -> usize {
        self.rows
            .iter()
            .map(|r| r.count_ones() as usize)
            .sum::<usize>()
            / 2
    }

    /// Neighbors of `v` in ascending order.
    pub fn neighbors(&self, v: usize) -> Vec<usize> {
        let mut out = Vec::new();
        let mut row = self.rows[v];
        while row != 0 {
            out.push(row.trailing_zeros() as usize);
            row &= row - 1;
        }
        out
    }

    pub fn degree(&self, v: usize) -> usize {
        self.rows[v].count_ones() as usize
    }

    /// Degree sequence, non-increasing.
    pub fn degree_sequence(&self) -> Vec<usize> {
        let mut d: Vec<usize> = (0..self.n).map(|v| self.degree(v)).collect();
        d.sort_unstable_by(|a, b| b.cmp(a));
        d
    }

    /// Complement of the graph (same vertex set, inverted edges).
    pub fn complement(&self) -> Graph {
        let mut c = self.clone();
        for i in 0..self.n {
            let mask = if self.n == 64 {
                u64::MAX
            } else {
                (1u64 << self.n) - 1
            };
            c.rows[i] = (!self.rows[i]) & mask & !(1u64 << i);
        }
        c
    }

    /// Deterministic canonical-ish serialization used for content hashing.
    /// NOT an isomorphism certificate — see `canon` module (Milestone 1).
    pub fn to_adjacency_vec(&self) -> Vec<Vec<bool>> {
        (0..self.n)
            .map(|i| (0..self.n).map(|j| (self.rows[i] >> j) & 1 == 1).collect())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_graph_zero_edges() {
        let g = Graph::empty(6).unwrap();
        assert_eq!(g.edge_count(), 0);
        assert_eq!(g.num_vertices(), 6);
    }

    #[test]
    fn edges_are_symmetric_and_counted_once() {
        let g = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 0)]).unwrap();
        assert!(g.has_edge(1, 0));
        assert!(g.has_edge(0, 1));
        assert_eq!(g.edge_count(), 3);
        assert_eq!(g.degree_sequence(), vec![2, 2, 2, 0]);
    }

    #[test]
    fn duplicate_edges_merge() {
        let g = Graph::from_edges(3, &[(0, 1), (1, 0)]).unwrap();
        assert_eq!(g.edge_count(), 1);
    }

    #[test]
    fn self_loop_rejected() {
        assert!(matches!(
            Graph::from_edges(3, &[(1, 1)]),
            Err(GraphError::SelfLoop(1))
        ));
    }

    #[test]
    fn out_of_range_rejected() {
        assert!(matches!(
            Graph::from_edges(3, &[(0, 5)]),
            Err(GraphError::VertexOutOfRange(5, 3))
        ));
    }

    #[test]
    fn too_many_vertices_rejected() {
        assert!(matches!(
            Graph::empty(MAX_VERTICES + 1),
            Err(GraphError::TooManyVertices(_))
        ));
    }

    #[test]
    fn complement_is_involutive() {
        let g = Graph::from_edges(5, &[(0, 1), (2, 3)]).unwrap();
        let cc = g.complement().complement();
        assert_eq!(g, cc);
    }

    #[test]
    fn adjacency_roundtrip() {
        let g = Graph::from_edges(4, &[(0, 1), (2, 3)]).unwrap();
        let h = Graph::from_adjacency(&g.to_adjacency_vec()).unwrap();
        assert_eq!(g, h);
    }
}
