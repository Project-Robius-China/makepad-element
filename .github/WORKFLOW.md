# Self-Managed Fork Workflow

> Project-Robius-China/makepad-element is a self-managed fork of mofa-org/makepad-element.
> We iterate independently and batch-contribute upstream when ready.

## Remotes

| Name | Repo | Purpose |
|------|------|---------|
| `origin` | Project-Robius-China/makepad-element | Primary development target |
| `upstream` | mofa-org/makepad-element | Upstream source, sync periodically |

## Daily Development

```bash
# Start a new feature
git checkout main && git pull origin main
git checkout -b feature/my-feature

# ... develop, commit ...

# Push and create PR
git push -u origin feature/my-feature
gh pr create --repo Project-Robius-China/makepad-element \
  --base main --head feature/my-feature \
  --title "feat: description"

# Self-merge when ready
gh pr merge <PR_NUMBER> --repo Project-Robius-China/makepad-element --merge
git checkout main && git pull origin main
```

## Branch Naming

- `feature/<name>` — new components/features
- `fix/<name>` — bug fixes
- `refactor/<name>` — code improvements
- `docs/<name>` — documentation

## Syncing with Upstream (Weekly)

```bash
git checkout main
git fetch upstream
git merge upstream/main
# Resolve conflicts if any
git push origin main
```

## Contributing Back to Upstream

```bash
# List origin-only commits
git fetch upstream
git log --oneline upstream/main..origin/main

# Create topic branch from upstream/main
git checkout upstream/main
git checkout -b upstream-contrib/<topic>
git cherry-pick <commits>
git push origin upstream-contrib/<topic>

# Submit PR to upstream
gh pr create --repo mofa-org/makepad-element \
  --base main \
  --head Project-Robius-China:upstream-contrib/<topic> \
  --title "feat: topic description"
```
