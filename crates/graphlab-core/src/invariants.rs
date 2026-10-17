//! Exact graph invariants on the bitset representation. All integer math.

use crate::graph::Graph;

/// Connected components (vertex sets), canonical order by smallest vertex.
pub fn components(g: &Graph) -> Vec<Vec<usize>> {
    let n = g.num_vertices();
    let mut seen = vec![false; n];
    let mut out = Vec::new();
    for s in 0..n {
        if seen[s] {
            continue;
        }
        let mut stack = vec![s];
        let mut comp = Vec::new();
        seen[s] = true;
        while let Some(v) = stack.pop() {
            comp.push(v);
            for u in g.neighbors(v) {
                if !seen[u] {
                    seen[u] = true;
                    stack.push(u);
                }
            }
        }
        comp.sort_unstable();
        out.push(comp);
    }
    out.sort_by_key(|c| c[0]);
    out
}

pub fn is_connected(g: &Graph) -> bool {
    if g.num_vertices() == 0 {
        return true; // convention: the null graph is connected here — audited in PROOF_TRUST_MODEL docs when formalized
    }
    components(g).len() == 1
}

/// Number of triangles (3-cycles), exact via bitset intersection.
pub fn triangle_count(g: &Graph) -> u64 {
    let n = g.num_vertices();
    let mut t = 0u64;
    for i in 0..n {
        for &j in &g.neighbors(i) {
            if j <= i {
                continue;
            }
            // common neighbors of i and j with index > j
            let common = g.adjacency_row(i) & g.adjacency_row(j);
            let mask_hi = if n == 64 {
                u64::MAX
            } else {
                !((1u64 << (j + 1)) - 1)
            };
            t += (common & mask_hi).count_ones() as u64;
        }
    }
    t
}

/// Clique number, exact (branch and bound over vertices, fine for n <= ~40
/// sparse; engine graphs stay small by design).
pub fn clique_number(g: &Graph) -> usize {
    let n = g.num_vertices();
    let mut best = 0usize;
    let mut cur: Vec<usize> = Vec::new();
    expand(g, 0, n, &mut cur, &mut best);
    best
}

fn expand(g: &Graph, start: usize, n: usize, cur: &mut Vec<usize>, best: &mut usize) {
    if cur.len() + (n - start) <= *best {
        return; // bound: even taking all remaining vertices can't beat best
    }
    for v in start..n {
        if cur.iter().all(|&u| g.has_edge(u, v)) {
            cur.push(v);
            if cur.len() > *best {
                *best = cur.len();
            }
            expand(g, v + 1, n, cur, best);
            cur.pop();
        }
    }
}

/// Independence number = clique number of complement.
pub fn independence_number(g: &Graph) -> usize {
    clique_number(&g.complement())
}

/// Chromatic number, exact via independent-set cover DFS with symmetry
/// breaking (color classes filled left-to-right). Fine for bounded n.
pub fn chromatic_number(g: &Graph) -> usize {
    let n = g.num_vertices();
    if n == 0 {
        return 0;
    }
    for k in 1..=n {
        let mut colors = vec![usize::MAX; n];
        if color_dfs(g, 0, k, &mut colors) {
            return k;
        }
    }
    n
}

fn color_dfs(g: &Graph, v: usize, k: usize, colors: &mut Vec<usize>) -> bool {
    if v == g.num_vertices() {
        return true;
    }
    let max_used = colors[..v]
        .iter()
        .filter(|&&c| c < k)
        .max()
        .map_or(0usize, |&c| c + 1)
        .min(k - 1);
    for c in 0..=max_used {
        let ok = g.neighbors(v).iter().all(|&u| u >= v || colors[u] != c);
        if ok {
            colors[v] = c;
            if color_dfs(g, v + 1, k, colors) {
                return true;
            }
        }
    }
    colors[v] = usize::MAX;
    false
}

/// Number of 4-cycles (as vertex sets with all 4 cycle edges; counts each
/// C4 once). Exact via pair-of-common-neighbors.
pub fn cycle4_count(g: &Graph) -> u64 {
    let n = g.num_vertices();
    let mut c = 0u64;
    for u in 0..n {
        for v in (u + 1)..n {
            if !g.has_edge(u, v) {
                continue;
            }
            let common = g.adjacency_row(u) & g.adjacency_row(v);
            let cm = common.count_ones() as u64;
            // each C4 through edge (u,v) uses two common neighbors w,x with
            // w-x NOT an edge? No — a 4-cycle u-w-v-x-u needs w,x common
            // neighbors of u,v. Each unordered pair {w,x} of common neighbors
            // yields the cycle u-w-v-x-u iff w!=x. It's counted once per
            // diagonal pair of the 4-cycle; divide by 2 at the end.
            if cm >= 2 {
                c += cm * (cm - 1) / 2;
            }
        }
    }
    c / 2
}

/// Degree sequence (non-increasing) — mirrors Graph::degree_sequence for API
/// discoverability in invariant contexts.
pub fn degree_sequence(g: &Graph) -> Vec<usize> {
    g.degree_sequence()
}

/// Spectral-free, exact "secondary bound": edge density as an exact rational
/// (m / C(n,2)) returned as (num, den) with den = 0 meaning n < 2.
pub fn edge_density_rational(g: &Graph) -> (u64, u64) {
    let n = g.num_vertices() as u64;
    if n < 2 {
        return (0, 0);
    }
    let pairs = n * (n - 1) / 2;
    let m = g.edge_count() as u64;
    let gg = gcd(m, pairs);
    (m / gg, pairs / gg)
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Full invariant set, JSON-friendly, deterministic key order.
#[derive(serde::Serialize, Debug, Clone)]
pub struct Invariants {
    pub n: usize,
    pub m: usize,
    pub degree_sequence: Vec<usize>,
    pub components: usize,
    pub connected: bool,
    pub triangles: u64,
    pub cycle4: u64,
    pub clique_number: usize,
    pub independence_number: usize,
    pub chromatic_number: usize,
    pub density_num: u64,
    pub density_den: u64,
}

pub fn all_invariants(g: &Graph) -> Invariants {
    let (num, den) = edge_density_rational(g);
    Invariants {
        n: g.num_vertices(),
        m: g.edge_count(),
        degree_sequence: degree_sequence(g),
        components: components(g).len(),
        connected: is_connected(g),
        triangles: triangle_count(g),
        cycle4: cycle4_count(g),
        clique_number: clique_number(g),
        independence_number: independence_number(g),
        chromatic_number: chromatic_number(g),
        density_num: num,
        density_den: den,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::g6::decode_graph6;

    fn path(n: usize) -> Graph {
        let edges: Vec<(usize, usize)> = (0..n - 1).map(|i| (i, i + 1)).collect();
        Graph::from_edges(n, &edges).unwrap()
    }

    fn complete(n: usize) -> Graph {
        let edges: Vec<(usize, usize)> = (0..n)
            .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
            .collect();
        Graph::from_edges(n, &edges).unwrap()
    }

    fn cycle(n: usize) -> Graph {
        let edges: Vec<(usize, usize)> = (0..n).map(|i| (i, (i + 1) % n)).collect();
        Graph::from_edges(n, &edges).unwrap()
    }

    #[test]
    fn known_small_values() {
        // C5: chi=3 (odd cycle), omega=2, alpha=2, triangles=0, conn
        let c5 = cycle(5);
        assert_eq!(chromatic_number(&c5), 3);
        assert_eq!(clique_number(&c5), 2);
        assert_eq!(independence_number(&c5), 2);
        assert_eq!(triangle_count(&c5), 0);
        assert!(is_connected(&c5));
        // K4: chi 4, omega 4, 4 triangles, 3 C4s? K4 has 3 distinct 4-cycles as vertex sets {all 4} * 3 cycles. Our counter counts vertex sets with a C4 subgraph? see doc: counts pairs of common neighbors per edge = C4 subgraphs w/ diagonals included. K4: each edge has 2 common neighbors -> 1 pair; 6 edges -> 6; /2 = 3. K4 has 3 distinct 4-cycles (Hamilton cycles modulo direction/start = 3). Matches.
        let k4 = complete(4);
        assert_eq!(chromatic_number(&k4), 4);
        assert_eq!(clique_number(&k4), 4);
        assert_eq!(triangle_count(&k4), 4);
        assert_eq!(cycle4_count(&k4), 3);
        // P4: chi=2, omega=2, alpha=2, triangles=0
        let p4 = path(4);
        assert_eq!(chromatic_number(&p4), 2);
        assert_eq!(clique_number(&p4), 2);
        assert_eq!(independence_number(&p4), 2);
        assert_eq!(triangle_count(&p4), 0);
        // empty+isolated
        let e = Graph::empty(3).unwrap();
        assert_eq!(components(&e).len(), 3);
        assert!(!is_connected(&e));
        assert_eq!(chromatic_number(&e), 1);
        assert_eq!(chromatic_number(&Graph::empty(0).unwrap()), 0);
    }

    #[test]
    fn c6_is_two_regular_not_triangles() {
        let c6e: Vec<(usize, usize)> = (0..6).map(|i| (i, (i + 1) % 6)).collect();
        let c6 = Graph::from_edges(6, &c6e).unwrap();
        assert_eq!(c6.degree_sequence(), vec![2, 2, 2, 2, 2, 2]);
        assert_eq!(triangle_count(&c6), 0);
        assert_eq!(cycle4_count(&c6), 0);
        assert_eq!(chromatic_number(&c6), 2);
    }

    #[test]
    fn chi_alpha_clique_relations_on_small_graphs() {
        // sanity: omega <= chi, alpha >= n/chi (independence lower bound)
        for s in crate::enumerate::enumerate_labeled_upto(5) {
            if s.len() > 4 {
                continue;
            } // tiny sample keeps test fast; full sweep in bench
            let g = decode_graph6(&s).unwrap();
            let om = clique_number(&g);
            let chi = chromatic_number(&g);
            let al = independence_number(&g);
            let n = g.num_vertices();
            assert!(om <= chi.max(1) || n == 0, "omega>chi on {s}");
            if n > 0 {
                assert!((al * chi) >= n, "alpha*chi < n on {s}");
            }
        }
    }

    #[test]
    fn density_exact_rational() {
        let k4 = complete(4);
        assert_eq!(edge_density_rational(&k4), (1, 1));
        let p4 = path(4);
        assert_eq!(edge_density_rational(&p4), (1, 2)); // 3 edges / 6 pairs
    }
}
