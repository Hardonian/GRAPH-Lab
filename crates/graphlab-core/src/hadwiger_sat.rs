//! SAT-based exact minor solver for Hadwiger number verification on n > 10.
//!
//! Encodes the complete minor decision problem (G has a K_t minor) into CNF
//! and solves it with the CDCL SAT solver `varisat`.
//!
//! Certified models are independently checked using `check_branch_model`.

use std::collections::HashSet;

use crate::graph::{Graph, GraphError};
use crate::hadwiger::{check_branch_model, DELETED};
use varisat::{ExtendFormula, Solver, Var};

#[derive(Debug, thiserror::Error)]
pub enum HadwigerSatError {
    #[error("SAT solver failed: {0}")]
    Solver(String),
    #[error("Graph error: {0}")]
    Graph(#[from] GraphError),
}

/// Determine whether graph G has a K_t minor using SAT encoding.
pub fn has_kminor_sat(g: &Graph, t: usize) -> Result<bool, HadwigerSatError> {
    Ok(find_branch_model_sat(g, t)?.is_some())
}

/// Compute the exact Hadwiger number h(G) of graph G using SAT minor tests.
/// Searches upwards from t = 1 to the theoretical upper bound; stops at the first UNSAT.
pub fn hadwiger_number_sat(g: &Graph) -> Result<usize, HadwigerSatError> {
    let n = g.num_vertices();
    if n == 0 {
        return Ok(0);
    }
    let m = g.edge_count() as u64;
    let mut ub = 1usize;
    while (((ub + 1) as u64) * (ub as u64) / 2) <= m && ub < n {
        ub += 1;
    }
    ub = ub.min(n);

    let max_deg = (0..n)
        .map(|v| (0..n).filter(|&u| g.has_edge(v, u)).count())
        .max()
        .unwrap_or(0);

    let mut best_t = 1;
    for t in 2..=ub {
        if max_deg + 1 < t && t > n / 2 {
            break;
        }
        if has_kminor_sat(g, t)? {
            best_t = t;
        } else {
            break;
        }
    }
    Ok(best_t)
}

/// Find a constructive branch-set certificate for a K_t minor in G using SAT.
#[allow(clippy::needless_range_loop)]
pub fn find_branch_model_sat(g: &Graph, t: usize) -> Result<Option<Vec<usize>>, HadwigerSatError> {
    let n = g.num_vertices();
    if t == 0 || t > n {
        return Ok(None);
    }
    if t == 1 {
        let mut assign = vec![DELETED; n];
        if n >= 1 {
            assign[0] = 0;
            return Ok(Some(assign));
        }
        return Ok(None);
    }
    let m = g.edge_count();
    if m < t * (t - 1) / 2 {
        return Ok(None);
    }

    // Degree bound pruning:
    // If max_deg < t - 1 and t > n / 2, at least one branch must be a singleton,
    // which has degree <= max_deg < t - 1, unable to connect to all t - 1 other branches.
    let max_deg = (0..n)
        .map(|v| (0..n).filter(|&u| g.has_edge(v, u)).count())
        .max()
        .unwrap_or(0);
    if max_deg + 1 < t && t > n / 2 {
        return Ok(None);
    }

    let mut solver = Solver::new();

    // 1. Allocate branch assignment and root variables
    // x[v][k] for v in 0..n, k in 0..t
    let mut x: Vec<Vec<Var>> = Vec::with_capacity(n);
    let mut root: Vec<Vec<Var>> = Vec::with_capacity(n);
    for _ in 0..n {
        let mut x_v = Vec::with_capacity(t);
        let mut r_v = Vec::with_capacity(t);
        for _ in 0..t {
            x_v.push(solver.new_var());
            r_v.push(solver.new_var());
        }
        x.push(x_v);
        root.push(r_v);
    }

    // Constraint 1.1: Pairwise disjoint branch sets (at most one branch per vertex)
    for v in 0..n {
        for j in 0..t {
            for k in (j + 1)..t {
                solver.add_clause(&[x[v][j].negative(), x[v][k].negative()]);
            }
        }
    }

    // Constraint 2: Roots
    for k in 0..t {
        // At least one root for branch k
        let mut any_root = Vec::with_capacity(n);
        for v in 0..n {
            any_root.push(root[v][k].positive());
            // Root must be in branch k
            solver.add_clause(&[root[v][k].negative(), x[v][k].positive()]);
        }
        solver.add_clause(&any_root);

        // At most one root for branch k
        for u in 0..n {
            for v in (u + 1)..n {
                solver.add_clause(&[root[u][k].negative(), root[v][k].negative()]);
            }
        }
    }

    // Constraint 3: Symmetry breaking on roots (root_0 < root_1 < ... < root_{t-1})
    for k in 0..(t - 1) {
        for v in 0..n {
            for u in 0..=v {
                solver.add_clause(&[root[v][k].negative(), root[u][k + 1].negative()]);
            }
        }
    }
    // Range bounds on roots
    for k in 0..t {
        for v in 0..k {
            solver.add_clause(&[root[v][k].negative()]);
        }
        for v in (n - t + k + 1)..n {
            solver.add_clause(&[root[v][k].negative()]);
        }
    }

    // Edge list
    let mut edges = Vec::with_capacity(m);
    for u in 0..n {
        for v in (u + 1)..n {
            if g.has_edge(u, v) {
                edges.push((u, v));
            }
        }
    }

    // Constraint 4: Connectivity
    let max_depth = n - t;
    if max_depth > 0 {
        // Parent pointers p[v][u] for u in N(v)
        let mut p: Vec<Vec<Option<Var>>> = vec![vec![None; n]; n];
        for &(u, v) in &edges {
            p[u][v] = Some(solver.new_var());
            p[v][u] = Some(solver.new_var());
        }

        // Distance order variables d[v][l] for l in 0..max_depth (representing dist(v) >= l + 1)
        let mut d: Vec<Vec<Var>> = Vec::with_capacity(n);
        for _ in 0..n {
            let mut d_v = Vec::with_capacity(max_depth);
            for _ in 0..max_depth {
                d_v.push(solver.new_var());
            }
            d.push(d_v);
        }

        for v in 0..n {
            let neighbors: Vec<usize> = (0..n).filter(|&u| g.has_edge(v, u)).collect();

            // d[v][l+1] implies d[v][l]
            for l in 0..(max_depth - 1) {
                solver.add_clause(&[d[v][l + 1].negative(), d[v][l].positive()]);
            }

            // Root has distance 0, so d[v][0] (dist >= 1) is false if v is a root
            for k in 0..t {
                solver.add_clause(&[root[v][k].negative(), d[v][0].negative()]);
            }

            // If v in branch k is not root, it must have at least one parent in N(v)
            for k in 0..t {
                let mut clause = vec![x[v][k].negative(), root[v][k].positive()];
                for &u in &neighbors {
                    if let Some(p_vu) = p[v][u] {
                        clause.push(p_vu.positive());
                    }
                }
                solver.add_clause(&clause);
            }

            // If v is root of branch k, it cannot have parents
            for &u in &neighbors {
                if let Some(p_vu) = p[v][u] {
                    for k in 0..t {
                        solver.add_clause(&[root[v][k].negative(), p_vu.negative()]);
                    }

                    // If p_vu is true, v and u must be in the same branch
                    let mut in_branch = vec![p_vu.negative()];
                    for k in 0..t {
                        in_branch.push(x[v][k].positive());
                        solver.add_clause(&[
                            p_vu.negative(),
                            x[v][k].negative(),
                            x[u][k].positive(),
                        ]);
                    }
                    solver.add_clause(&in_branch);

                    // Parent u has strictly smaller distance than v:
                    // dist(v) >= 1 (d[v][0])
                    solver.add_clause(&[p_vu.negative(), d[v][0].positive()]);
                    // dist(u) >= l + 1 implies dist(v) >= l + 2
                    for l in 0..(max_depth - 1) {
                        solver.add_clause(&[
                            p_vu.negative(),
                            d[u][l].negative(),
                            d[v][l + 1].positive(),
                        ]);
                    }
                    // dist(u) cannot be >= max_depth if it is a parent
                    solver.add_clause(&[p_vu.negative(), d[u][max_depth - 1].negative()]);
                }
            }
        }
    }

    // Constraint 5: Mutual cross-edges between branches
    let mut cross: Vec<Vec<Vec<Var>>> = Vec::with_capacity(m);
    for _ in 0..m {
        let mut row_j = Vec::with_capacity(t);
        for _ in 0..t {
            let mut row_k = Vec::with_capacity(t);
            for _ in 0..t {
                row_k.push(solver.new_var());
            }
            row_j.push(row_k);
        }
        cross.push(row_j);
    }

    for j in 0..t {
        for k in (j + 1)..t {
            // At least one edge between branch j and branch k
            let mut cross_clause = Vec::with_capacity(m);
            for e_idx in 0..m {
                cross_clause.push(cross[e_idx][j][k].positive());
                let (u, v) = edges[e_idx];
                let c = cross[e_idx][j][k];

                solver.add_clause(&[c.negative(), x[u][j].positive(), x[u][k].positive()]);
                solver.add_clause(&[c.negative(), x[v][j].positive(), x[v][k].positive()]);
                solver.add_clause(&[c.negative(), x[u][j].negative(), x[v][j].negative()]);
                solver.add_clause(&[c.negative(), x[u][k].negative(), x[v][k].negative()]);
            }
            solver.add_clause(&cross_clause);
        }
    }

    // Solve with SAT solver
    let sat = solver
        .solve()
        .map_err(|e| HadwigerSatError::Solver(format!("{e:?}")))?;

    if !sat {
        return Ok(None);
    }

    let model = solver.model().ok_or_else(|| {
        HadwigerSatError::Solver("satisfiable formula yielded no model".to_string())
    })?;

    let mut true_vars = HashSet::new();
    for lit in model {
        if lit.is_positive() {
            true_vars.insert(lit.var());
        }
    }

    let mut assign = vec![DELETED; n];
    for v in 0..n {
        for k in 0..t {
            if true_vars.contains(&x[v][k]) {
                assign[v] = k;
                break;
            }
        }
    }

    // Validate with independent checker
    if !check_branch_model(g, &assign, t) {
        return Err(HadwigerSatError::Solver(
            "SAT solver returned a model that failed independent validation".to_string(),
        ));
    }

    Ok(Some(assign))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sat_minor_matches_exact_on_petersen() {
        let petersen = Graph::petersen().unwrap();
        // Petersen has K_5 minor (h = 5), but not K_6 minor
        assert!(has_kminor_sat(&petersen, 5).unwrap());
        assert!(!has_kminor_sat(&petersen, 6).unwrap());
        assert_eq!(hadwiger_number_sat(&petersen).unwrap(), 5);

        let model = find_branch_model_sat(&petersen, 5).unwrap().unwrap();
        assert!(check_branch_model(&petersen, &model, 5));
    }

    #[test]
    fn sat_minor_on_groetzsch() {
        let g = Graph::groetzsch().unwrap();
        assert_eq!(g.num_vertices(), 11);
        assert_eq!(g.edge_count(), 20);

        // Grötzsch has chromatic number 4, so h(G) >= 4
        assert!(has_kminor_sat(&g, 4).unwrap());
        let h = hadwiger_number_sat(&g).unwrap();
        assert!(h >= 4);

        if let Some(model) = find_branch_model_sat(&g, h).unwrap() {
            assert!(check_branch_model(&g, &model, h));
        }
    }

    #[test]
    fn sat_minor_on_chvatal() {
        let g = Graph::chvatal().unwrap();
        assert_eq!(g.num_vertices(), 12);
        assert_eq!(g.edge_count(), 24);

        // Chvátal has chi = 4, so h(G) >= 4
        assert!(has_kminor_sat(&g, 4).unwrap());
        let h = hadwiger_number_sat(&g).unwrap();
        assert!(h >= 4);

        if let Some(model) = find_branch_model_sat(&g, h).unwrap() {
            assert!(check_branch_model(&g, &model, h));
        }
    }

    #[test]
    fn sat_minor_on_clebsch() {
        let g = Graph::clebsch().unwrap();
        assert_eq!(g.num_vertices(), 16);
        assert_eq!(g.edge_count(), 40);

        // Clebsch graph has h = 5
        assert!(has_kminor_sat(&g, 5).unwrap());
        let model = find_branch_model_sat(&g, 5).unwrap().unwrap();
        assert!(check_branch_model(&g, &model, 5));
    }

    #[test]
    fn sat_minor_on_hypercube_4() {
        let q4 = Graph::hypercube(4).unwrap();
        assert_eq!(q4.num_vertices(), 16);
        assert_eq!(q4.edge_count(), 32);

        // Q_4 has h >= 4
        assert!(has_kminor_sat(&q4, 4).unwrap());
        let model4 = find_branch_model_sat(&q4, 4).unwrap().unwrap();
        assert!(check_branch_model(&q4, &model4, 4));
    }

    #[test]
    fn exact_benchmarks_report() {
        let groetzsch = Graph::groetzsch().unwrap();
        let h_g = hadwiger_number_sat(&groetzsch).unwrap();
        let m_g = find_branch_model_sat(&groetzsch, h_g).unwrap().unwrap();
        println!("BENCHMARK GROETZSCH: n=11, m=20, h={h_g}, model={m_g:?}");

        let chvatal = Graph::chvatal().unwrap();
        let h_c = hadwiger_number_sat(&chvatal).unwrap();
        let m_c = find_branch_model_sat(&chvatal, h_c).unwrap().unwrap();
        println!("BENCHMARK CHVATAL: n=12, m=24, h={h_c}, model={m_c:?}");

        let clebsch = Graph::clebsch().unwrap();
        let h_cl = hadwiger_number_sat(&clebsch).unwrap();
        let m_cl = find_branch_model_sat(&clebsch, h_cl).unwrap().unwrap();
        println!("BENCHMARK CLEBSCH: n=16, m=40, h={h_cl}, model={m_cl:?}");

        let q4 = Graph::hypercube(4).unwrap();
        let h_q4 = hadwiger_number_sat(&q4).unwrap();
        let m_q4 = find_branch_model_sat(&q4, h_q4).unwrap().unwrap();
        println!("BENCHMARK HYPERCUBE_4: n=16, m=32, h={h_q4}, model={m_q4:?}");
    }
}
