# OpenSpec Zed Extension

This Zed extension provides OpenSpec slash commands for the Zed Assistant, allowing you to use OpenSpec workflows directly within the Zed editor.

## Features

This extension supports all OpenSpec slash commands:

- `/openspec propose` - Create a new change and generate planning artifacts
- `/openspec explore` - Explore ideas and investigate problems  
- `/openspec new` - Start a new change scaffold
- `/openspec continue` - Create the next artifact in the dependency chain
- `/openspec ff` - Fast-forward: create all planning artifacts at once
- `/openspec verify` - Validate implementation matches artifacts
- `/openspec sync` - Merge delta specs into main specs
- `/openspec apply` - Implement tasks from the active change
- `/openspec archive` - Archive a completed change
- `/openspec bulk-archive` - Archive multiple completed changes at once
- `/openspec onboard` - Guided tutorial through OpenSpec workflow

## Prerequisites

- [Node.js 20.19.0 or higher](https://nodejs.org/)
- OpenSpec installed globally: `npm install -g @fission-ai/openspec@latest`

## Installation

1. Clone this repository to your Zed extensions directory:
   ```bash
   git clone https://github.com/OpenSpec/OpenSpec-Zed ~/.config/zed/extensions/openspec
   ```

2. (Optional) Install OpenSpec globally for manual use:
   ```bash
   npm install -g @fission-ai/openspec@latest
   ```

3. Restart Zed

## Usage

1. Open the Zed Assistant panel
2. Type `/` to see available slash commands
3. Select one of the OpenSpec commands (e.g., `/openspec propose`)
4. Add any required arguments (change names, descriptions, etc.)
5. Press Enter to get AI assistance with your OpenSpec workflow

**Note**: The extension provides AI prompts to help you with OpenSpec workflows. For actual OpenSpec CLI execution, you'll need to run the commands manually in your terminal.

## Development

### Building

```bash
cargo build --target wasm32-wasi
```

### Testing

To test the extension locally:

1. Copy the extension to Zed's dev extensions directory:
   ```bash
   cp -r ~/.config/zed/extensions/openspec ~/.config/zed/dev/extensions/
   ```

2. Restart Zed

3. Use the extension in the Assistant panel

## License

This extension is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request