#!/usr/bin/env python3
r"""
generate_figures.py — Generate publication-ready vector figures (SVG & TikZ)
for Hadwiger minor contractions and invariant distributions.

Outputs:
  - papers/figures/costa_contractions.svg
  - papers/figures/atlas_n9_distributions.svg
  - papers/figures/costa_contractions.tikz
"""

from __future__ import annotations

import math
from pathlib import Path

REPO = Path(__file__).resolve().parents[1]
FIG_DIR = REPO / "papers" / "figures"
FIG_DIR.mkdir(parents=True, exist_ok=True)


def generate_svg_distributions() -> Path:
    """Generate high-resolution vector SVG for n=9 atlas distributions."""
    width, height = 900, 480
    out_svg = FIG_DIR / "atlas_n9_distributions.svg"

    # Data from results/research/atlas-n9-report.md
    # Hadwiger number counts: h=1..9
    h_data = [
        (1, 1, 0.0004),
        (2, 152, 0.0553),
        (3, 9203, 3.3506),
        (4, 85215, 31.0247),
        (5, 141381, 51.4734),
        (6, 37381, 13.6095),
        (7, 1314, 0.4784),
        (8, 20, 0.0073),
        (9, 1, 0.0004),
    ]

    # Deficit counts: h - chi = 0..3
    margin_data = [
        ("+0 (Tight)", 61287, 22.31, "#3b82f6"),
        ("+1 (Surplus)", 160899, 58.58, "#10b981"),
        ("+2 (Surplus)", 51234, 18.65, "#8b5cf6"),
        ("+3 (Surplus)", 1248, 0.45, "#f59e0b"),
    ]

    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}" style="background-color:#0f172a; font-family:system-ui, -apple-system, sans-serif;">',
        '<defs>',
        '  <linearGradient id="barGrad" x1="0%" y1="0%" x2="0%" y2="100%">',
        '    <stop offset="0%" stop-color="#38bdf8"/>',
        '    <stop offset="100%" stop-color="#0284c7"/>',
        '  </linearGradient>',
        '  <linearGradient id="panelGrad" x1="0%" y1="0%" x2="100%" y2="100%">',
        '    <stop offset="0%" stop-color="#1e293b"/>',
        '    <stop offset="100%" stop-color="#0f172a"/>',
        '  </linearGradient>',
        '</defs>',
        '<!-- Main Title -->',
        '<text x="450" y="36" fill="#f8fafc" font-size="20" font-weight="bold" text-anchor="middle">Complete Invariant Atlas for n = 9 (274,668 Non-Isomorphic Graphs)</text>',
        '<text x="450" y="58" fill="#94a3b8" font-size="13" text-anchor="middle">100% Adherence to Hadwiger &amp; Weak Hadwiger Conjectures (0 Violations)</text>',
    ]

    # Panel 1: Hadwiger Number Distribution
    p1_x, p1_y, p1_w, p1_h = 40, 80, 400, 360
    svg.append(f'<rect x="{p1_x}" y="{p1_y}" width="{p1_w}" height="{p1_h}" rx="12" fill="url(#panelGrad)" stroke="#334155" stroke-width="1"/>')
    svg.append(f'<text x="{p1_x + p1_w//2}" y="{p1_y + 32}" fill="#38bdf8" font-size="15" font-weight="bold" text-anchor="middle">Hadwiger Number h(G) Distribution</text>')

    plot_x, plot_y, plot_w, plot_h = p1_x + 40, p1_y + 60, p1_w - 60, 240
    max_h_count = 145000
    bar_w = plot_w / len(h_data) - 8

    # Grid lines
    for i in range(5):
        val = i * 35000
        gy = plot_y + plot_h - (val / max_h_count) * plot_h
        svg.append(f'<line x1="{plot_x}" y1="{gy}" x2="{plot_x + plot_w}" y2="{gy}" stroke="#334155" stroke-dasharray="3,3" stroke-width="1"/>')
        svg.append(f'<text x="{plot_x - 8}" y="{gy + 4}" fill="#64748b" font-size="10" text-anchor="end">{val//1000}k</text>')

    for idx, (h_val, count, pct) in enumerate(h_data):
        bx = plot_x + idx * (bar_w + 8) + 4
        bh = (count / max_h_count) * plot_h
        by = plot_y + plot_h - bh
        svg.append(f'<rect x="{bx}" y="{by}" width="{bar_w}" height="{bh}" rx="3" fill="url(#barGrad)"/>')
        svg.append(f'<text x="{bx + bar_w/2}" y="{plot_y + plot_h + 18}" fill="#94a3b8" font-size="12" font-weight="bold" text-anchor="middle">{h_val}</text>')
        if pct >= 5.0:
            svg.append(f'<text x="{bx + bar_w/2}" y="{by - 6}" fill="#f8fafc" font-size="10" font-weight="bold" text-anchor="middle">{pct:.1f}%</text>')

    # Panel 2: Hadwiger Deficit (h - chi) Distribution
    p2_x = 460
    svg.append(f'<rect x="{p2_x}" y="{p1_y}" width="{p1_w}" height="{p1_h}" rx="12" fill="url(#panelGrad)" stroke="#334155" stroke-width="1"/>')
    svg.append(f'<text x="{p2_x + p1_w//2}" y="{p1_y + 32}" fill="#10b981" font-size="15" font-weight="bold" text-anchor="middle">Hadwiger Deficit Δ_H = h(G) - χ(G)</text>')

    p2_plot_x = p2_x + 40
    max_m_count = 170000
    m_bar_w = plot_w / len(margin_data) - 16

    for i in range(5):
        val = i * 40000
        gy = plot_y + plot_h - (val / max_m_count) * plot_h
        svg.append(f'<line x1="{p2_plot_x}" y1="{gy}" x2="{p2_plot_x + plot_w}" y2="{gy}" stroke="#334155" stroke-dasharray="3,3" stroke-width="1"/>')
        svg.append(f'<text x="{p2_plot_x - 8}" y="{gy + 4}" fill="#64748b" font-size="10" text-anchor="end">{val//1000}k</text>')

    for idx, (label, count, pct, col) in enumerate(margin_data):
        bx = p2_plot_x + idx * (m_bar_w + 16) + 8
        bh = (count / max_m_count) * plot_h
        by = plot_y + plot_h - bh
        svg.append(f'<rect x="{bx}" y="{by}" width="{m_bar_w}" height="{bh}" rx="4" fill="{col}"/>')
        svg.append(f'<text x="{bx + m_bar_w/2}" y="{plot_y + plot_h + 18}" fill="#94a3b8" font-size="11" font-weight="bold" text-anchor="middle">{label.split()[0]}</text>')
        svg.append(f'<text x="{bx + m_bar_w/2}" y="{by - 6}" fill="#f8fafc" font-size="10" font-weight="bold" text-anchor="middle">{pct:.1f}%</text>')

    svg.append('</svg>')
    out_svg.write_text('\n'.join(svg), encoding='utf-8')
    return out_svg


def generate_svg_contractions() -> Path:
    """Generate high-resolution vector SVG demonstrating Costa minor contractions."""
    width, height = 960, 520
    out_svg = FIG_DIR / "costa_contractions.svg"

    svg: list[str] = [
        f'<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {width} {height}" width="{width}" height="{height}" style="background-color:#0b0f19; font-family:system-ui, -apple-system, sans-serif;">',
        '<defs>',
        '  <filter id="glow" x="-20%" y="-20%" width="140%" height="140%">',
        '    <feGaussianBlur stdDeviation="3" result="blur"/>',
        '    <feComposite in="SourceGraphic" in2="blur" operator="over"/>',
        '  </filter>',
        '</defs>',
        '<text x="480" y="38" fill="#f8fafc" font-size="21" font-weight="bold" text-anchor="middle">Branch-Set Minor Contractions in Sum-Free Cayley Complements</text>',
        '<text x="480" y="60" fill="#94a3b8" font-size="13" text-anchor="middle">Resolution of Costa, Luu, Wood, and Yip (2025) Question 20 (HC for α=2)</text>',
    ]

    # Branch set color palette
    colors = ["#ef4444", "#3b82f6", "#10b981", "#f59e0b", "#8b5cf6", "#ec4899", "#14b8a6", "#f97316", "#6366f1"]

    # Diagram 1: F_5 (n=5, C_5 complement = C_5, contracting to K_3)
    c1_x, c1_y, c1_r = 170, 200, 75
    svg.append('<rect x="25" y="85" width="290" height="400" rx="14" fill="#111827" stroke="#1f2937" stroke-width="1.5"/>')
    svg.append('<text x="170" y="118" fill="#38bdf8" font-size="16" font-weight="bold" text-anchor="middle">F_5 (n=5, χ=3, h=3)</text>')
    svg.append('<text x="170" y="138" fill="#94a3b8" font-size="12" text-anchor="middle">S = {1, 4} ⟹ G ≅ C_5 ⟹ K_3 minor</text>')

    # 5 vertices on circle
    f5_coords = []
    for i in range(5):
        ang = -math.pi / 2 + 2 * math.pi * i / 5
        f5_coords.append((c1_x + c1_r * math.cos(ang), c1_y + c1_r * math.sin(ang)))

    # Edges of C_5 complement: in F_5, non-edges are {1,4}, so edges are diff in {2, 3}: (0,2), (0,3), (1,3), (1,4), (2,4)
    f5_edges = [(0, 2), (0, 3), (1, 3), (1, 4), (2, 4)]
    for u, v in f5_edges:
        x1, y1 = f5_coords[u]
        x2, y2 = f5_coords[v]
        svg.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="#374151" stroke-width="2"/>')

    # Branch sets: B0={0}, B1={1,3}, B2={2,4}
    f5_branches = [0, 1, 2, 1, 2]
    # Highlight matching contraction edges (1,3) and (2,4)
    for u, v, col in [(1, 3, colors[1]), (2, 4, colors[2])]:
        x1, y1 = f5_coords[u]
        x2, y2 = f5_coords[v]
        svg.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{col}" stroke-width="4" stroke-linecap="round"/>')

    for i, (x, y) in enumerate(f5_coords):
        b = f5_branches[i]
        col = colors[b]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="14" fill="{col}" stroke="#f8fafc" stroke-width="2" filter="url(#glow)"/>')
        svg.append(f'<text x="{x:.1f}" y="{y + 4:.1f}" fill="#ffffff" font-size="11" font-weight="bold" text-anchor="middle">{i}</text>')

    svg.append('<text x="170" y="325" fill="#e2e8f0" font-size="13" font-weight="bold" text-anchor="middle">Contraction Quotients:</text>')
    svg.append(f'<text x="170" y="348" fill="{colors[0]}" font-size="12" text-anchor="middle">Branch 0: {{0}} (singleton)</text>')
    svg.append(f'<text x="170" y="368" fill="{colors[1]}" font-size="12" text-anchor="middle">Branch 1: {{1, 3}} (contracted edge)</text>')
    svg.append(f'<text x="170" y="388" fill="{colors[2]}" font-size="12" text-anchor="middle">Branch 2: {{2, 4}} (contracted edge)</text>')
    svg.append('<rect x="70" y="415" width="200" height="42" rx="8" fill="#1e293b" stroke="#38bdf8" stroke-width="1"/>')
    svg.append('<text x="170" y="441" fill="#38bdf8" font-size="13" font-weight="bold" text-anchor="middle">K_3 Minor Certified (h=3)</text>')

    # Diagram 2: F_7 (n=7, S={1,6}, χ=4, h=5, Margin +1)
    c2_x, c2_y, c2_r = 480, 200, 78
    svg.append('<rect x="335" y="85" width="290" height="400" rx="14" fill="#111827" stroke="#1f2937" stroke-width="1.5"/>')
    svg.append('<text x="480" y="118" fill="#10b981" font-size="16" font-weight="bold" text-anchor="middle">F_7 (n=7, χ=4, h=5)</text>')
    svg.append('<text x="480" y="138" fill="#94a3b8" font-size="12" text-anchor="middle">S = {1, 6} ⟹ m=14 ⟹ Surplus Margin +1</text>')

    f7_coords = []
    for i in range(7):
        ang = -math.pi / 2 + 2 * math.pi * i / 7
        f7_coords.append((c2_x + c2_r * math.cos(ang), c2_y + c2_r * math.sin(ang)))

    f7_branches = [0, 1, 2, 3, 4, 1, 3]  # Branch 1:{1,5}, Branch 3:{3,6}, 0:{0}, 2:{2}, 4:{4}
    # Edges in F_7: diffs in {2, 3, 4, 5}
    for u in range(7):
        for v in range(u + 1, 7):
            d = min((v - u) % 7, (u - v) % 7)
            if d in [2, 3]:
                svg.append(f'<line x1="{f7_coords[u][0]:.1f}" y1="{f7_coords[u][1]:.1f}" x2="{f7_coords[v][0]:.1f}" y2="{f7_coords[v][1]:.1f}" stroke="#374151" stroke-width="1.5"/>')

    # Highlight matching contractions: (1,5) and (3,6)
    for u, v, col in [(1, 5, colors[1]), (3, 6, colors[3])]:
        x1, y1 = f7_coords[u]
        x2, y2 = f7_coords[v]
        svg.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{col}" stroke-width="3.5" stroke-linecap="round"/>')

    for i, (x, y) in enumerate(f7_coords):
        b = f7_branches[i]
        col = colors[b]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="13" fill="{col}" stroke="#f8fafc" stroke-width="2" filter="url(#glow)"/>')
        svg.append(f'<text x="{x:.1f}" y="{y + 4:.1f}" fill="#ffffff" font-size="10" font-weight="bold" text-anchor="middle">{i}</text>')

    svg.append('<text x="480" y="325" fill="#e2e8f0" font-size="13" font-weight="bold" text-anchor="middle">5 Contracted Branch Sets:</text>')
    svg.append(f'<text x="480" y="348" fill="{colors[1]}" font-size="12" text-anchor="middle">Branch 1: {{1, 5}} | Branch 3: {{3, 6}}</text>')
    svg.append(f'<text x="480" y="368" fill="{colors[0]}" font-size="12" text-anchor="middle">Singletons: {{0}}, {{2}}, {{4}}</text>')
    svg.append('<text x="480" y="388" fill="#10b981" font-size="12" font-weight="bold" text-anchor="middle">All 10 Inter-Branch Pairs Adjacent</text>')
    svg.append('<rect x="380" y="415" width="200" height="42" rx="8" fill="#1e293b" stroke="#10b981" stroke-width="1"/>')
    svg.append('<text x="480" y="441" fill="#10b981" font-size="13" font-weight="bold" text-anchor="middle">K_5 Minor Certified (h=5 > χ=4)</text>')

    # Diagram 3: Non-Cyclic Z_2 x Z_6 (n=12, connected, χ=6, h=7, Margin +1)
    c3_x, c3_y, c3_r = 790, 200, 80
    svg.append('<rect x="645" y="85" width="290" height="400" rx="14" fill="#111827" stroke="#1f2937" stroke-width="1.5"/>')
    svg.append('<text x="790" y="118" fill="#8b5cf6" font-size="16" font-weight="bold" text-anchor="middle">Z_2 × Z_6 (n=12, χ=6, h=7)</text>')
    svg.append('<text x="790" y="138" fill="#94a3b8" font-size="12" text-anchor="middle">Connected Class 2 (m=42) ⟹ Margin +1</text>')

    z_coords = []
    for i in range(12):
        ang = -math.pi / 2 + 2 * math.pi * i / 12
        z_coords.append((c3_x + c3_r * math.cos(ang), c3_y + c3_r * math.sin(ang)))

    # Connected class 2 branch model: 7 branches (5 pairs + 2 singletons)
    z_branches = [0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4]
    for i in range(5):
        u, v = i, i + 7
        x1, y1 = z_coords[u]
        x2, y2 = z_coords[v]
        svg.append(f'<line x1="{x1:.1f}" y1="{y1:.1f}" x2="{x2:.1f}" y2="{y2:.1f}" stroke="{colors[i]}" stroke-width="2.5" stroke-linecap="round"/>')

    for i, (x, y) in enumerate(z_coords):
        b = z_branches[i]
        col = colors[b]
        svg.append(f'<circle cx="{x:.1f}" cy="{y:.1f}" r="10" fill="{col}" stroke="#f8fafc" stroke-width="1.5"/>')
        svg.append(f'<text x="{x:.1f}" y="{y + 3:.1f}" fill="#ffffff" font-size="8" font-weight="bold" text-anchor="middle">{i}</text>')

    svg.append('<text x="790" y="325" fill="#e2e8f0" font-size="13" font-weight="bold" text-anchor="middle">Non-Cyclic Contraction Model:</text>')
    svg.append('<text x="790" y="348" fill="#8b5cf6" font-size="12" text-anchor="middle">5 Mixed Pairs: {0,7}, {1,8}, {2,9}...</text>')
    svg.append('<text x="790" y="368" fill="#94a3b8" font-size="12" text-anchor="middle">2 Singletons: {5}, {6}</text>')
    svg.append('<text x="790" y="388" fill="#e2e8f0" font-size="12" text-anchor="middle">Total Branch Sets: 7 (Complete K_7)</text>')
    svg.append('<rect x="690" y="415" width="200" height="42" rx="8" fill="#1e293b" stroke="#8b5cf6" stroke-width="1"/>')
    svg.append('<text x="790" y="441" fill="#8b5cf6" font-size="13" font-weight="bold" text-anchor="middle">K_7 Minor Certified (h=7 > χ=6)</text>')

    svg.append('</svg>')
    out_svg.write_text('\n'.join(svg), encoding='utf-8')
    return out_svg


def generate_tikz_figure() -> Path:
    """Generate standalone LaTeX TikZ figure for research paper incorporation."""
    out_tikz = FIG_DIR / "costa_contractions.tikz"
    tikz = r"""\begin{tikzpicture}[scale=0.9, every node/.style={circle, draw=black!80, inner sep=1.5pt, font=\scriptsize, font=\bfseries}]
  % F_5 matching contraction into K_3
  \begin{scope}[shift={(0,0)}]
    \node[fill=red!40] (v0) at (90:1.6cm) {0};
    \node[fill=blue!40] (v1) at (18:1.6cm) {1};
    \node[fill=green!40] (v2) at (306:1.6cm) {2};
    \node[fill=blue!40] (v3) at (234:1.6cm) {3};
    \node[fill=green!40] (v4) at (162:1.6cm) {4};

    \draw[thick, blue!80, line width=1.5pt] (v1) -- (v3);
    \draw[thick, green!80, line width=1.5pt] (v2) -- (v4);
    \draw[gray!70] (v0) -- (v2);
    \draw[gray!70] (v0) -- (v3);
    \draw[gray!70] (v1) -- (v4);

    \node[draw=none, below] at (0,-2.0) {$\mathbb{F}_5$ Complement: $K_3$ minor};
  \end{scope}

  % F_7 matching contraction into K_5
  \begin{scope}[shift={(5.5,0)}]
    \node[fill=red!40] (u0) at (90:1.6cm) {0};
    \node[fill=blue!40] (u1) at (38.5:1.6cm) {1};
    \node[fill=orange!40] (u2) at (347:1.6cm) {2};
    \node[fill=purple!40] (u3) at (295.5:1.6cm) {3};
    \node[fill=teal!40] (u4) at (244:1.6cm) {4};
    \node[fill=blue!40] (u5) at (193:1.6cm) {5};
    \node[fill=purple!40] (u6) at (141.5:1.6cm) {6};

    \draw[thick, blue!80, line width=1.5pt] (u1) -- (u5);
    \draw[thick, purple!80, line width=1.5pt] (u3) -- (u6);

    \node[draw=none, below] at (0,-2.0) {$\mathbb{F}_7$ Complement: $K_5$ minor ($h=5 > \chi=4$)};
  \end{scope}
\end{tikzpicture}
"""
    out_tikz.write_text(tikz.strip(), encoding="utf-8")
    return out_tikz


def main() -> None:
    print("=" * 60)
    print("GENERATING PUBLICATION-READY VECTOR FIGURES")
    print("=" * 60)

    f1 = generate_svg_distributions()
    print(f"Generated: {f1}")

    f2 = generate_svg_contractions()
    print(f"Generated: {f2}")

    f3 = generate_tikz_figure()
    print(f"Generated: {f3}")
    print("All vector figures generated successfully.")


if __name__ == "__main__":
    main()
