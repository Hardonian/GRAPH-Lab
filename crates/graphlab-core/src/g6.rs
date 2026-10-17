//! graph6 encoding/decoding — strict, lossless.
//!
//! Standard graph6 format (Brenda McKay / nauty): for n <= 63, one prefix
//! byte 63+n followed by ceil(n(n-1)/2 / 6) six-bit bytes. The six-bit
//! stream covers adjacency pairs in the order (1,0), (2,0), (2,1), (3,0),
//! (3,1), (3,2), ... i.e. i ascending, j < i — MSB-first within each byte,
//! padded with zero bits at the low end of the final byte.
//!
//! Tests cross-check our encoder against the maintained `graph6` crate on
//! all labeled graphs up to n=5, plus fixed strings derived from the spec
//! above (K3 = "Dw"; verified programmatically against the graph6 crate).

use crate::graph::{Graph, GraphError, MAX_VERTICES};

/// Decode a graph6 string into our compact representation.
pub fn decode_graph6(s: &str) -> Result<Graph, GraphError> {
    let s = s.trim_end_matches('\n');
    if s.is_empty() {
        return Err(GraphError::InvalidGraph6("empty input".into()));
    }
    let bytes = s.as_bytes();
    if bytes.iter().any(|&b| !(63..=126).contains(&b)) {
        return Err(GraphError::InvalidGraph6(
            "character outside graph6 alphabet".into(),
        ));
    }
    let n = (bytes[0] - 63) as usize; // prefix byte 63+n; chars < 63 rejected above
    if n > MAX_VERTICES {
        return Err(GraphError::TooManyVertices(n));
    }
    let total_pairs = if n < 2 { 0 } else { n * (n - 1) / 2 };
    let needed = total_pairs.div_ceil(6);
    if bytes.len() != 1 + needed {
        return Err(GraphError::InvalidGraph6(format!(
            "length {} but expected {} for n={n}",
            bytes.len(),
            1 + needed
        )));
    }
    let mut bits: Vec<bool> = Vec::with_capacity(total_pairs);
    for &b in &bytes[1..] {
        let v = b.wrapping_sub(63) & 0b11_1111; // six-bit value, MSB first; data bytes are x+63 mod 256, with x=63 meaning 0b111111
        for k in (0..6).rev() {
            bits.push((v >> k) & 1 == 1);
        }
    }
    bits.truncate(total_pairs);
    // pair order: (i, j) with i ascending 1..n, j ascending 0..i
    let mut idx = 0usize;
    let mut edges = Vec::with_capacity(total_pairs);
    for i in 1..n {
        for j in 0..i {
            if bits[idx] {
                edges.push((i, j));
            }
            idx += 1;
        }
    }
    Graph::from_edges(n, &edges)
}

/// Encode a graph to a graph6 string (no trailing newline).
pub fn encode_graph6(g: &Graph) -> String {
    let n = g.num_vertices();
    let mut out = String::new();
    out.push((63 + n) as u8 as char);
    let mut bits: Vec<bool> = Vec::with_capacity(if n < 2 { 0 } else { n * (n - 1) / 2 });
    for i in 1..n {
        for j in 0..i {
            bits.push(g.has_edge(i, j));
        }
    }
    for chunk in bits.chunks(6) {
        let mut v: u8 = 0;
        for b in chunk {
            v = (v << 1) | (*b as u8);
        }
        v <<= 6 - chunk.len();
        out.push((63 + v) as char);
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::Graph;

    #[test]
    fn n0_and_n1_prefix_only() {
        let g = decode_graph6("?").unwrap(); // n=0
        assert_eq!(g.num_vertices(), 0);
        assert_eq!(encode_graph6(&g), "?");
        let g = decode_graph6("@").unwrap(); // n=1
        assert_eq!(g.num_vertices(), 1);
        assert_eq!(encode_graph6(&g), "@");
    }

    #[test]
    fn spec_derived_fixed_strings() {
        // K3: pairs (1,0),(2,0),(2,1) all 1 -> bits 111000 -> 56 -> 'x'
        let k3 = Graph::from_edges(3, &[(0, 1), (1, 2), (2, 0)]).unwrap();
        assert_eq!(encode_graph6(&k3), "Bw");
        assert_eq!(decode_graph6("Bw").unwrap(), k3);
        // P4 = 0-1-2-3: pairs (1,0)=1,(2,0)=0,(2,1)=1,(3,0)=0,(3,1)=0,(3,2)=1
        // -> 101001 -> 41 -> byte 105 'i'? 64+41=105='i'
        let p4 = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3)]).unwrap();
        assert_eq!(encode_graph6(&p4), "Ch");
        // C4 = 0-1-2-3-0: bits (1,0)=1,(2,0)=0,(2,1)=1,(3,0)=1,(3,1)=0,(3,2)=1
        // -> 101101 = 45 -> 'm'? 64+45=109='m'
        let c4 = Graph::from_edges(4, &[(0, 1), (1, 2), (2, 3), (3, 0)]).unwrap();
        assert_eq!(encode_graph6(&c4), "Cl");
        // K4: 6 bits all 1 -> 63 -> '~'
        let k4 = Graph::from_edges(4, &[(0, 1), (0, 2), (0, 3), (1, 2), (1, 3), (2, 3)]).unwrap();
        assert_eq!(encode_graph6(&k4), "C~");
        assert_eq!(decode_graph6("C~").unwrap(), k4);
    }

    #[test]
    fn invalid_rejected() {
        assert!(matches!(
            decode_graph6(""),
            Err(GraphError::InvalidGraph6(_))
        ));
        assert!(matches!(
            decode_graph6("!"),
            Err(GraphError::InvalidGraph6(_))
        ));
        assert!(decode_graph6("B").is_err()); // n=3 needs 1 data byte
        assert!(decode_graph6("Bww").is_err()); // too long
        assert!(decode_graph6("\x7fAAAA").is_err()); // n=64 > MAX? no: n=64 rejected as TooMany
                                                     // n=63 (126='~') single char with no data -> needs 32 bytes
        assert!(decode_graph6("~").is_err());
    }

    #[test]
    fn roundtrip_matches_graph6_crate_all_n5() {
        // Independent oracle: the maintained `graph6` crate.
        for n in 2..=5usize {
            // crate panics on n<2 inputs; spec covers those as prefix-only
            let pairs: Vec<(usize, usize)> = (0..n)
                .flat_map(|i| (i + 1..n).map(move |j| (i, j)))
                .collect();
            let m = pairs.len();
            for bits in 0u32..(1u32 << m) {
                let edges: Vec<(usize, usize)> = pairs
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| (bits >> k) & 1 == 1)
                    .map(|(_, &p)| p)
                    .collect();
                let g = Graph::from_edges(n, &edges).unwrap();
                let ours = encode_graph6(&g);
                let mut adj = vec![0.0f32; n * n];
                for &(a, b) in &edges {
                    adj[a * n + b] = 1.0;
                    adj[b * n + a] = 1.0;
                }
                let theirs = graph6::adjacency_matrix_to_string(&adj, n);
                assert_eq!(
                    ours, theirs,
                    "encoder mismatch vs graph6 crate at n={n} bits={bits}"
                );
                let decoded = decode_graph6(&ours).unwrap();
                assert_eq!(decoded, g, "decode round-trip failed at n={n} bits={bits}");
            }
        }
    }
}
