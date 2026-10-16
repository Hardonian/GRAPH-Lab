# Security Policy

This project runs locally on a shared research host that also carries
production services.

- No inbound network services. The CLI and Python control plane are
  local-only; nothing listens on a port.
- No secrets in the repo. Campaign configs may reference env var *names*
  but never values.
- Solver and external-binary execution is via pinned, provenance-recorded
  versions invoked with argv arrays (no shell interpolation).
- Report issues privately to the repository owner via GitHub security
  advisories.
