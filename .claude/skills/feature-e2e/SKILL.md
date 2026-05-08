---
name: feature-e2e
description: Run an end-to-end test of a Product CLI feature in a fresh tmp directory — build the release binary, init a clean .product/ project, seed it with a feature/ADR/TC, then exercise the feature's CLI and MCP surface. Use when the user asks to "do an end-to-end test of FT-XXX", "verify FT-XXX works in a fresh repo", "test FT-XXX end to end", or "shake out FT-XXX from scratch".
---

# Feature End-to-End Test

This skill runs a real-world end-to-end shake-out of a Product CLI feature against a fresh, isolated repo. Use it when you want to confirm that a feature actually works for a brand-new user — not just that the unit tests pass.

## When to use

- The user asks to do a "thorough end-to-end test" of a specific feature ID
- A feature has just been implemented and needs human-style validation
- You suspect drift between PRD/ADR text and the actual binary behaviour

## What to do

### 1. Read the feature spec first

Open `docs/features/FT-XXX-*.md` (or `.product/features/...` if working under the canonical layout). Note:

- The acceptance criteria list — these become the test plan
- The CLI surface section — every flag and subcommand the feature adds
- The MCP tool changes — input schema and output envelope
- The error codes — each EXX has a path that should fire

### 2. Build a release binary

```bash
cargo build --release
PRODUCT=/abs/path/to/repo/target/release/product
```

Always use the release binary — debug builds are slow and the bottleneck of an e2e run is the round-trip through `assert_cmd`-style fixtures, not compilation.

### 3. Create a clean tmp scratch directory

```bash
rm -rf /tmp/<feature-slug>-e2e-test
mkdir -p /tmp/<feature-slug>-e2e-test
cd /tmp/<feature-slug>-e2e-test
```

Always start clean — the resolution order for templates / config / .product directory walks up the filesystem, so a stale parent state can leak into the test.

### 4. Initialise a fresh project

```bash
$PRODUCT init -y \
    --name "FT-XXX E2E Test" \
    --description "End-to-end test of <feature title>" \
    --domain core="<one-sentence domain>"
```

Defaults to the canonical `.product/` layout (ADR-048). Pass `--legacy-layout` only if the feature specifically needs the pre-FT-057 root-based layout.

### 5. Seed the graph with linked artifacts

A bundle/render-style feature needs at least one feature, one ADR, and one TC, linked together:

```bash
$PRODUCT feature new "Hello World"
$PRODUCT adr new "Use TOML for config"
$PRODUCT test new "feature renders"          # NOTE: subcommand is `test`, NOT `tc`
$PRODUCT feature link FT-001 --adr ADR-001 --yes
$PRODUCT feature link FT-001 --test TC-001 --yes
```

The `--yes` flag on `feature link` is mandatory in non-TTY use to accept inferred transitive TC links.

### 6. Exercise the feature

Walk every acceptance criterion. Capture stdout / stderr / exit code separately — many features emit warnings on stderr while keeping stdout pipe-friendly:

```bash
$PRODUCT <command> --flag value > /tmp/out.txt 2> /tmp/err.txt
echo "exit=$? stdout=$(wc -c </tmp/out.txt) stderr=$(wc -c </tmp/err.txt)"
```

For each rendering target / format / mode the feature exposes, validate the output is structurally well-formed:

```bash
python3 -c "import json; json.load(open('/tmp/out.txt')); print('JSON ok')"
python3 -c "import yaml; yaml.safe_load(open('/tmp/out.txt')); print('YAML ok')"
python3 -c "import xml.etree.ElementTree as ET; ET.parse('/tmp/out.txt'); print('XML ok')"
```

For every error code the spec promises, intentionally trigger it and confirm the exit code, stderr message, and hint text.

### 7. Test the MCP surface

The MCP server reads JSON-RPC over stdio. The minimum exchange is `initialize` → `notifications/initialized` → `tools/call`:

```bash
{
  printf '%s\n' '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2024-11-05","capabilities":{},"clientInfo":{"name":"e2e","version":"0"}}}'
  printf '%s\n' '{"jsonrpc":"2.0","method":"notifications/initialized"}'
  printf '%s\n' '{"jsonrpc":"2.0","id":2,"method":"tools/call","params":{"name":"product_context","arguments":{"id":"FT-001","depth":2,"target":"claude-opus"}}}'
  sleep 0.3
} | $PRODUCT mcp 2>/tmp/mcp.err
```

The response wraps the tool output in `{"content":[{"text":"<json-encoded-result>","type":"text"}]}` — parse it twice to get at the inner envelope. Verify all keys the spec promises are present.

### 8. Test repo-local / user-level overrides

Many features (templates, config, prompts) resolve in repo → user → built-in order. Test all three by:

- Dropping a file under `.product/<thing>/` and confirming it shadows the built-in
- Dropping the same name under `~/.product/<thing>/` and confirming the repo file wins
- Removing the repo override and confirming the user file wins

### 9. Report back

For every acceptance criterion in the spec, mark pass/fail. Call out any drift between spec and behaviour — these are the highest-value findings of an e2e run.

## Gotchas captured from prior runs

- **Subcommand naming.** The CLI is `product test new`, not `product tc new`. The internal IDs use `TC-XXX` but the subcommand family is `test`.
- **MCP arg name.** The `product_context` tool reads its feature id from the `id` property, not `feature_id`. Older PRD examples may show `feature_id` — that is doc drift, not the canonical name.
- **MCP envelope is double-wrapped.** Tool results come as `{"result":{"content":[{"text":"<inner json>","type":"text"}]}}`. You must `json.loads()` twice.
- **Stderr deprecation notes.** Several flags (`--for-llm`) emit deprecation notes to stderr. Always capture stderr separately or you will miss them.
- **No-flag default vs `--target human`.** Per FT-063 these should produce identical Markdown. If they don't, that is the drift to flag.
- **Built-in templates ship embedded.** They live at `src/context/template/builtin/*.toml` but are pulled in via `include_str!` — there is no `templates/` directory at the repo root or anywhere on disk in a fresh install.
- **Resolution paths in templates --where.** Built-ins show as `(built-in)` (no path), repo/user templates show their absolute path.
- **`feature link` in non-TTY needs `--yes`.** Without it, the inferred-transitive-TC prompt blocks waiting on stdin.
- **`product init` is idempotent only with `--force`.** Re-running over an existing config without `--force` errors out — clean the tmp dir first.

## Skill output

End each run with:

1. A pass/fail table covering every acceptance criterion in the spec
2. A drift list — every place spec says X but the binary does Y
3. The path to the scratch directory so the user can poke around (e.g. `/tmp/ft063-e2e-test/`)
