# aihub 🤖

A fast, lightweight CLI for interacting with AI language models. Built with Rust.

[![Crates.io](https://img.shields.io/crates/v/aihub.svg)](https://crates.io/crates/aihub)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## 📦 Installation

### Via Cargo
```bash
cargo install aihub
```

### Via APT (Debian/Ubuntu)
```bash
curl -fsSL https://your-repo-url/gpg-key | sudo gpg --dearmor -o /usr/share/keyrings/aihub.gpg
echo "deb [signed-by=/usr/share/keyrings/aihub.gpg] https://your-repo-url stable main" | sudo tee /etc/apt/sources.list.d/aihub.list
sudo apt update
sudo apt install aihub
```

### From Source
```bash
git clone https://github.com/beingglitch/aihub.git
cd aihub
cargo install --path .
```

## 🚀 Quick Start

1. **Set your API key:**
```bash
aihub config set-key --key YOUR_OPENAI_API_KEY
```

2. **Start asking:**
```bash
aihub ask --prompt "Explain quantum computing in simple terms"
```

## 💡 Usage

### Basic Commands

```bash
# Simple question
aihub ask -p "What is Rust?"

# With context/instructions
aihub ask -p "Write a REST API" -i "Use Python Flask with error handling"

# With reasoning effort (low, medium, high)
aihub ask -p "Debug this algorithm" -e high

# Save API key
aihub config set-key -k YOUR_KEY
```

### Examples

```bash
# Code generation
aihub ask -p "Create a binary search tree" -i "Use Rust with comments"

# Problem solving
aihub ask -p "Optimize this SQL query: SELECT * FROM users WHERE..." -e high

# Explanation
aihub ask -p "How does TCP handshake work?"
```

## 📋 Commands

| Command | Options | Description |
|---------|---------|-------------|
| `ask` | `-p, --prompt` | Your question or prompt (required) |
|       | `-i, --instructions` | Optional context or instructions |
|       | `-e, --effort` | Reasoning effort: low/medium/high |
| `config set-key` | `-k, --key` | Save your API key locally |

## ⚙️ Configuration

**Config file:** `~/.config/aihub/config.toml`

**Environment variable:** `OPENAI_API_KEY`

Priority: Environment variable > Config file

## 🛣️ Roadmap

### Near Term
- [ ] Model selection (GPT-4, GPT-4o, GPT-3.5)
- [ ] Multi-provider support
  - [ ] Anthropic (Claude)
  - [ ] xAI (Grok)
  - [ ] Google (Gemini)
  - [ ] Local models (Ollama)
- [ ] Personalization & context management
- [ ] Conversation history
- [ ] Interactive mode

### Future
- [ ] Streaming responses
- [ ] Token usage tracking
- [ ] Custom system prompts
- [ ] Plugin system
- [ ] Rich TUI interface

## 🏗️ Built With

- **Rust** - Systems programming language
- **Tokio** - Async runtime
- **Clap** - Command-line argument parsing
- **Reqwest** - HTTP client

## 🤝 Contributing

Contributions welcome! Please:

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Commit changes: `git commit -m "feat: add amazing feature"`
4. Push to branch: `git push origin feature/amazing-feature`
5. Open a Pull Request

## 📝 License

MIT License - see [LICENSE](LICENSE) file for details.

## 📧 Contact

- **Issues:** [GitHub Issues](https://github.com/beingglitch/aihub/issues)
- **Repository:** [github.com/beingglitch/aihub](https://github.com/beingglitch/aihub)

---

Built with ❤️ and Rust 🦀