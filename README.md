# MobTime CLI

## Overview

MobTime CLI is a command-line tool designed to facilitate and enhance mob programming sessions. The tool helps teams manage their mob programming sessions efficiently, ensuring a smooth and productive collaboration experience.

## Features

- **Timer Management**: Set up and manage timed rotations for mob programming sessions, allowing team members to take turns as the "driver."
- **Notifications**: Receive notifications when it's time to switch roles, ensuring everyone stays synchronized during the mob programming session.
- **Customizable Settings**: Tailor the tool to your team's preferences by configuring session duration, notification preferences, and other settings.

## Getting Started

### Prerequisites

- Rust: Make sure you have Rust installed on your machine.

### Installation

1. Clone the repository: `git clone https://github.com/rienmack/mobtime-cli.git`
2. Navigate to the project directory: `cd mobtime-cli`
3. Build the project: `cargo build`
4. Run the CLI: `cargo run`

## Usage

```bash
# Display help
cargo run -- --help

# Start a new mob programming session
cargo run start
