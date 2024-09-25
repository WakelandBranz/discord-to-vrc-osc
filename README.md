# discord-to-vrc-osc

A highly performant Discord bot that leverages VRChat's Open Sound Control (OSC) protocol to enable control of VRChat actions through simple Discord commands.

## Table of Contents
1. [Features](#features)
2. [Prerequisites](#prerequisites)
3. [Installation](#installation)
4. [Configuration](#configuration)
5. [Available Commands](#available-commands)
6. [Usage Examples](#usage-examples)
7. [Security Considerations](#security-considerations)
8. [Contributing](#contributing)
9. [License](#license)

## Features

- Customizable through configuration files
- Supports all official VRChat desktop OSC input implementations (VR inputs coming soon)
- Multithreaded tasks allow for multiple actions to be performed concurrently
- Automated world joining (WIP)
- Fast and efficient Rust-based implementation

## Prerequisites

- Rust (latest stable version)
- Discord Bot Token
- VRChat account with OSC enabled

## Installation

1. Clone the repository:
   ```
   git clone https://github.com/WakelandBranz/discord-to-vrc-osc.git
   cd discord-to-vrc-osc
   ```

2. Build the project:
   ```
   cargo build --release
   ```

3. Set up your configuration file (see [Configuration](#configuration) section).

4. Run the bot:
   ```
   cargo run --release
   ```

## Configuration

1. Copy the `config.example.toml` to `config.toml`.
2. Edit `config.toml` and fill in your settings. Here's an explanation of the configuration options:

```toml
[auth]
token = ""  # Your Discord bot token
owners = ["239561212818489344", ""]  # Array of user IDs with owner privileges

[options]
prefixes = ["!", ".", "?", "~"]  # Prefixes for text commands
mention_as_prefix = true  # Allow mentioning the bot as a command prefix
message = ""  # Custom message (used for development)

[system]
ephemeral_admin_commands = true  # Whether admin commands are visible only to the user
vrc_client_logging_channel = ""  # Discord channel ID for VRChat client logging (WIP)

[vrc_client]
localhost = "127.0.0.1"  # IP address for VRChat client communication
receiver_port = 9001  # Port for receiving OSC messages (default: 9001)
transmitter_port = 9000  # Port for sending OSC messages (default: 9000)
```

3. Ensure you've set your Discord bot token in the `[auth]` section.
4. (Optional) Add owner user IDs to the `owners` array for additional privileges.
5. Customize the command prefixes in the `[options]` section if desired.
6. Adjust the VRChat client settings in the `[vrc_client]` section if necessary.

Note: Some features (marked as WIP) are still in development and may not be fully functional.

## Available Commands

All commands can be used with either a slash command (`/`) or a prefix command (e.g., `!`), depending on your preference and configuration settings.

Parameters not marked as optional are required, commands executed without required parameters will fail.

### 1. Move Horizontally
- Command: `/move_horizontal` or `!move_horizontal`
- Description: Moves the character horizontally in the specified direction.
- Parameters:
  - `direction`: The direction to move (Forward, Backward, Left, Right)
  - `duration`: How long to move in the specified direction (in seconds)

### 2. Look
- Command: `/look` or `!look`
- Description: Changes the character's view angle.
- Parameters:
  - `direction`: The direction to look (Left, Right)
  - `duration`: How long to maintain the look direction (in seconds)

### 3. Run
- Command: `/run` or `!run`
- Description: Makes the character run.
- Parameters:
  - `duration`: How long to run (in seconds)

### 4. Jump
- Command: `/jump` or `!jump`
- Description: Makes the character jump.
- Parameters: None

### 5. Combined Action
- Command: `/action_combined` or `!action_combined`
- Description: Performs a combination of actions simultaneously.
- Parameters:
  - `movement` (optional): Direction to move (Forward, Backward, Left, Right)
  - `look` (optional): Direction to look (Left, Right)
  - `run` (optional): Whether to run (true/false)
  - `jump` (optional): Whether to jump (true/false)
  - `duration`: Duration of the combined action (in seconds)

## Usage Examples

1. Move forward for 5 seconds:
   ```
   /move_horizontal direction:Forward duration:5
   ```

2. Look left for 2 seconds:
   ```
   /look direction:Left duration:2
   ```

3. Run for 10 seconds:
   ```
   /run duration:10
   ```

4. Jump:
   ```
   /jump
   ```

5. Move forward, look right, and run for 3 seconds:
   ```
   /action_combined movement:Forward look:Right run:true duration:3
   ```

## Security Considerations

- Ensure that only trusted users have access to the Discord channel where the bot is active.
- Regularly review the bot's audit logs for any suspicious activity.
- Keep your Discord bot token secure and never share it publicly.

## License

This project is licensed under the GPL v3 License - see the [LICENSE](LICENSE) file for details.
