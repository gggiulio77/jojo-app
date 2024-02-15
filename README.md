# jojo-app

This desktop application facilitates users in managing and configuring [jojo-client](https://github.com/gggiulio77/jojo-client). It provides a user-friendly interface for initiating the [jojo](https://github.com/gggiulio77/jojo) system's process.

### Quick Links

- [Getting Started](#getting-started)
- [Prerequisites](#prerequisites)
- [Installation](#installation)
- [Usage](#usage)
- [Roadmap](#roadmap)
- [License](#license)

## Getting Started

In addition to a graphical user interface, this project serves to deploy [jojo-server](https://github.com/gggiulio77/jojo-server) and [jojo-discovery](https://github.com/gggiulio77/jojo-discovery) on the host PC. It involves spawning threads to handle various services and utilizing channels for inter-service communication.

The workflow begins with [jojo-discovery](https://github.com/gggiulio77/jojo-discovery) providing network information to [jojo-client](https://github.com/gggiulio77/jojo-client), followed by [jojo-server](https://github.com/gggiulio77/jojo-server) managing the websocket server and interfacing with [jojo-app](https://github.com/gggiulio77/jojo-app) to showcase all clients in the UI.

Network-related operations are centralized in [jojo-server](https://github.com/gggiulio77/jojo-server), thus [jojo-app](https://github.com/gggiulio77/jojo-app) communicates with this service through events.

<br>

Developed using [Tauri](https://tauri.app/) and incorporating [Leptos](https://leptos.dev/) along with [DaisyUI](https://daisyui.com/) as the frontend framework.

For now the project only runs on Windows.

### Prerequisites

Before proceeding, ensure you have [Rust](https://www.rust-lang.org/tools/install) installed on your system.

If you're new to [Tauri](https://tauri.app/), I recommend reading the [getting started guide](https://tauri.app/v1/guides/getting-started/prerequisites).

Once you've met all the prerequisites for [Tauri](https://tauri.app/), install [Tauri-CLI](https://tauri.app/v1/api/cli/) by running 

`cargo install tauri-cli`

### Installation

`git clone https://github.com/gggiulio77/jojo-app.git`

## Usage

To initiate a development environment, execute `cargo tauri dev`. This command compiles and runs the backend in a shell while opening a [WebView](https://tauri.app/v1/references/webview-versions/) window for the frontend.

For creating an installer, use `cargo tauri build`.

Installers will be stored using a release tag.

## Roadmap

- [ ] Enhance error handling
- [ ] Make it Linux and MacOs compatible
- [ ] Improve documentation

## License