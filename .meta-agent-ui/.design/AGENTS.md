# AGENTS.md

## Canonical Entrypoint

`meta-agent` is the canonical CLI entrypoint for this repository.

```bash
# One-command workflow: generate PRD, decompose, and run
meta-agent kickoff "Brief description of the feature"

# Or import an existing PRD
meta-agent kickoff --mode prd --prd-path path/to/PRD.md

# Check status
meta-agent status
```

## Read Priority

1. `design/CHANGELOG.md` — latest changes and active plans
2. `design/plans/` — current execution plans (PRDs)
3. `design/history/` — completed / archived plans
4. `design/wiki/` — detailed design knowledge
5. `README.md` — project overview and setup

## Current Active Plans

_No active plans yet. Add plans to `design/plans/`._

## Notes

- Design assets are version-controlled under `design/`.
- Runtime state (`.state/`) is local-only and git-ignored.
