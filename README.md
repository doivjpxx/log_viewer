# Advanced Log Viewer

A powerful, cross-platform log analysis tool built with modern technologies. Designed to compete with commercial solutions like LogViewPlus while remaining completely open source.

## 🚀 Features

### Current Implementation
- ✅ **Multi-format Support**: Parse various log formats including JSON, XML, and plain text
- ✅ **Advanced Filtering**: Complex filter combinations with AND/OR logic
- ✅ **Fast Search**: Full-text search with regex support
- ✅ **Virtual Scrolling**: Handle large files efficiently
- ✅ **Export Capabilities**: Export filtered results to various formats
- ✅ **Parser Configuration**: Customize parsing rules for different log formats
- ✅ **Real-time Monitoring**: Watch files for live updates

### Planned Features (See [Roadmap](docs/ROADMAP.md))
- 🔄 **Large File Handling**: Open 1GB+ files with chunked processing
- 🔄 **Analytics Dashboard**: Built-in reporting and visualization
- 🔄 **Remote Log Access**: SFTP, cloud integration, and streaming
- 🔄 **Custom Dashboards**: SQL-like queries and custom reports
- 🔄 **Plugin System**: Extensible architecture for custom features

## 🛠 Technology Stack

- **Frontend**: Vue 3 + TypeScript + Tailwind CSS
- **Backend**: Rust + Tauri
- **Search Engine**: Tantivy (planned)
- **State Management**: Pinia
- **Build Tool**: Vite

## 🎯 Why Another Log Viewer?

Log files are often viewed as plain text, but they contain valuable data waiting to be analyzed. This log viewer transforms raw log data into actionable insights through:

- **Performance**: Native Rust backend with async processing
- **Modern UI**: Responsive, intuitive interface built with Vue 3
- **Cross-platform**: Works on Windows, macOS, and Linux
- **Open Source**: No licensing costs, full customization freedom
- **Extensible**: Plugin system for custom parsers and visualizations

## 📊 Competitive Advantages

| Feature | Our Solution | Commercial Tools |
|---------|-------------|------------------|
| Cost | 🆓 Free & Open Source | 💰 Expensive licenses |
| Performance | ⚡ Rust + Modern Web | 🐌 Legacy architectures |
| UI/UX | 🎨 Modern, responsive | 📟 Dated interfaces |
| Extensibility | 🔧 Full source access | 🔒 Limited customization |
| Cross-platform | ✅ Native everywhere | ❌ Often Windows-only |

## 🚀 Quick Start

### Prerequisites
- [Node.js](https://nodejs.org/) (v18 or higher)
- [Rust](https://rustup.rs/) (latest stable)
- [Git](https://git-scm.com/)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/your-username/log_viewer.git
cd log_viewer

# Install dependencies
npm install

# Start development server
npm run tauri dev
```

### Building for Production

```bash
# Build for current platform
npm run tauri build

# The built application will be in src-tauri/target/release/
```

## 📖 Documentation

- [📋 Development Roadmap](docs/ROADMAP.md) - Detailed feature development plan
- [⚙️ Technical Specifications](docs/TECHNICAL_SPECS.md) - Architecture and implementation details
- [📊 Competitive Analysis](docs/COMPETITIVE_ANALYSIS.md) - How we compare to commercial solutions

## 🧪 Sample Log Files

The repository includes sample log files for testing:
- `sample.log` - Basic application logs
- `sample_json.log` - JSON-formatted logs
- `sample_multiline.log` - Multi-line log entries
- `test_large.log` - Large file for performance testing

## 🤝 Contributing

We welcome contributions! Whether you're fixing bugs, adding features, or improving documentation, your help makes this project better.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📋 Development Status

This project is in active development. Current focus:
- **Phase 1**: Core log viewing functionality ✅
- **Phase 2**: Advanced filtering and search (In Progress)
- **Phase 3**: Analytics and dashboards (Planned)

See our [roadmap](docs/ROADMAP.md) for detailed progress tracking.

## 🛡 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🙏 Acknowledgments

- Built with [Tauri](https://tauri.app/) for native performance
- UI powered by [Vue 3](https://vuejs.org/) and [Tailwind CSS](https://tailwindcss.com/)
- Inspired by commercial tools like LogViewPlus and Splunk
- Thanks to the open source community for making this possible

## 📞 Support

- 🐛 [Report Issues](https://github.com/your-username/log_viewer/issues)
- 💬 [Discussions](https://github.com/your-username/log_viewer/discussions)
- 📧 [Email Support](mailto:support@your-domain.com)

---

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
