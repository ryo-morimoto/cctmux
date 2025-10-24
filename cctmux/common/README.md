# cctmux-common

**Shared utilities**

## Design Philosophy

Following OpenAI Codex's `codex-common` principles:

1. **Shared code that doesn't belong in core**: Utilities used by multiple crates but not business logic
2. **Fine-grained feature flags**: Enable selective compilation of only needed functionality
3. **Minimal dependencies**: Only essential external dependencies

## Placement Guidelines

Code suitable for this crate includes:

- Config management (when needed in the future)
- Logger initialization utilities
- Path management (database paths, config file paths, etc.)
- Time utilities (timestamp formatting, etc.)

## Current Status

Currently an empty module. Add functionality as needed following the feature flag pattern.
