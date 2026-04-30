# kommit

> AI-powered git commit messages — fully local, zero network, free forever.

kommit reads your staged diff, learns your commit style from your git history, and suggests a conventional commit message in under a second. No API keys. No internet. No cost per commit.

---

## Why kommit

Every cloud-based commit message tool has the same problems:

- Sends your code to someone else's server
- Costs money at scale (20 commits/day adds up)
- Too slow to use as a pre-commit hook

kommit runs entirely on your machine using a small local model via llama.cpp.

---

## Features

- Reads your actual staged diff via `git diff --cached`
- Learns your personal style from your last 50 commits
- Installs as a `prepare-commit-msg` hook — runs automatically on every commit
- Conventional commits format out of the box
- Cross-platform: Windows, Mac, Linux
- Zero telemetry, zero network calls, zero API keys

---

## Installation

### Prerequisites

- [Rust](https://rustup.rs/) 1.75+
- Git
- [Ollama](https://ollama.com) — for local model inference

### Set up ollama

```bash
# After installing ollama, pull the model (one-time, ~1GB)
ollama pull qwen2.5-coder:1.5b

# Start ollama (needs to be running when you use kommit)
ollama serve
```

### Download binary (no Rust needed)

Download the latest binary for your platform from the
[releases page](https://github.com/SnehaDeshmukh28/kommit/releases/latest):

- **Windows:** `kommit-windows-x86_64.exe`
- **Mac:** `kommit-macos-x86_64`
- **Linux:** `kommit-linux-x86_64`

Add it to your PATH and you're done.

### Build from source

```bash
git clone https://github.com/SnehaDeshmukh28/kommit.git
cd kommit
cargo build --release
```

Add the binary to your PATH:

```bash
# Linux / Mac
cp target/release/kommit ~/.local/bin/

# Windows — add target\release\ to your PATH in System Settings
```

---

## Usage

### Set up in a repo

```bash
cd your-project
kommit init
```

This installs the git hook. From now on, every `git commit` will automatically suggest a message based on your staged changes.

### Generate manually

```bash
git add .
kommit generate
```

Example output:

```
Learning from your last commits...
  Preferred type : feat
  Uses scope     : true
  Avg length     : 51 chars

Suggested commit message:

  feat(auth): add JWT token refresh logic

Accept? [y/n/e to edit]:
```

---

### Generate with reasoning

```bash
kommit generate --body
```

Example output:

```
  Suggested message:

    feat(auth): replace session tokens with JWT

  Reasoning:

    Switched to JWT because session storage was becoming
    a bottleneck at scale. JWT allows stateless auth
    across multiple service instances without shared state.

  y  accept    e  edit    r  regenerate    n  cancel
```

## Configuration

Create `~/.kommit/config.toml` to customize behaviour:

```toml
model = "llama3.2:3b"
ollama_url = "http://localhost:11434"
max_diff_chars = 4000
```

View your current config:

```bash
kommit config
```

All settings are optional — defaults work out of the box.

---

## How it works

```
git add .  →  kommit reads staged diff
           →  parses changed files
           →  reads your last 50 commits for style
           →  builds a prompt with diff + style hint
           →  runs local model inference (llama.cpp)
           →  suggests conventional commit message
           →  you accept, edit, or reject
```

---

## Status

Active development — v0.4.0 released.

| Feature | Status |
|---|---|
| Staged diff reader | ✅ Done |
| Style learning from git log | ✅ Done |
| Git hook installer | ✅ Done |
| Prompt builder | ✅ Done |
| Local model inference (ollama) | ✅ Done |
| Interactive accept / edit / reject | ✅ Done |
| Junk file exclusion | ✅ Done |
| Configurable model via config file | ✅ Done |
| Cross-platform release binaries | ✅ Done |
| Native llama.cpp backend | 🔄 In progress |
| Commit message body with reasoning | ✅ Done |
| Auto-download model on first run | 📋 Planned |
| Homebrew tap (Mac) | 📋 Planned |
| Scoop bucket (Windows) | 📋 Planned |

---

## Project structure

```
src/
  main.rs      — CLI entry point, argument parsing
  diff.rs      — reads staged diff via git diff --cached
  model.rs     — prompt builder and model inference
  style.rs     — learns commit style from git log
  hook.rs      — installs prepare-commit-msg git hook
```

---

## Contributing

PRs welcome. This repo uses conventional commits and CI must pass before merging.

```bash
git checkout -b feat/your-feature
# make your changes
cargo fmt
cargo clippy
cargo test
git commit -m "feat: your feature"
git push origin feat/your-feature
# open a PR on GitHub
```

---

## License

MIT — see [LICENSE](LICENSE)