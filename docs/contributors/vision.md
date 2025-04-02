# Project Vision

Hibiscus Commit CLI addresses the friction in conventional commit workflows by learning from repository history to provide intelligent, context-aware suggestions. While existing commit tools enforce structure, they rarely adapt to team-specific patterns, leading to repetitive input and inconsistent messages. Our tool analyses previous commits to auto-suggest scopes, co-authors, and issue references while maintaining a configurable template system that works with existing standards like commitlint. We prioritize developer experience through smart defaults, cross-platform reliability, and progressive enhancement—starting with local git analysis and expanding to third-party integrations. The project scope focuses on conventional commit standardization with contextual intelligence, excluding general git workflow management beyond the commit process.

## Core Objectives

| Priority | Objective | Success Metric |
|----------|-----------|----------------|
| P0 | Create conventional-commit compliant messages | 100% compliance with conventional-commit spec |
| P0 | Allow multi-line input for commit body | Successful parsing of multi-paragraph bodies |
| P0 | Support multiple lines in the footer | Proper formatting of multiple footer entries |
| P0 | Remember previous commits and offer defaults | Accurate suggestion of previously used values |
| P1 | Support configuring body options via templates | Template system can handle 95% of common commit formats |
| P1 | Support configuring footer options via templates | Same as above |
| P1 | Support configuring scope lists (mandatory or suggestions) | Configuration options validated against conventional commit spec |
| P1 | Support configuring type lists (feat and fix always mandatory) | Same as above |
| P1 | Support reading commitlint config | Commitlint configs properly imported and respected |
| P2 | Integration with Github and Gitlab to retrieve users and issues | Successful API connections to all three platforms |
| P2 | Integration with Jira to retrieve issues | Issue retrieval in under 1 second for standard queries |
| P2 | Configurable integrations (e.g., for self-hosted instances) | Support for custom domains and authentication methods |
| P3 | Integration with other VCS providers as requested by the community | Extensible plugin architecture that allows community contributions |
| P3 | Integration with other issue tracking systems as requested by the community | Documented process for requesting new integrations |
| P3 | Support importing configuration from other tools | Compatible with 90%+ of existing conventional commit tools' configurations |

## Governance

This project adheres to the Hibiscus Collective organization governance model and contribution guidelines.

### Decision Making

The project follows the Hibiscus Collective decision-making process as outlined in [POL-0001](https://github.com/HibiscusCollective/.github/wiki/POL%E2%80%900001%3A-Decision-Making-Process).

### Contribution Process

Contributions to this project are welcome and should follow:

- Project-specific guidelines in [CONTRIBUTING.md](../CONTRIBUTING.md)
- Organization-wide contribution standards at [Hibiscus Collective Contributing](https://github.com/HibiscusCollective/.github/wiki/Contributing)

### Governance Model

The project currently follows the Benevolent Dictator For Life (BDFL) model as described in the [Hibiscus Collective Governance](https://github.com/HibiscusCollective/.github/wiki/Governance) guidelines, with plans to transition to a committee-based governance model when the community reaches critical mass.

## Out of scope

The following list of features is explicitly out of scope of the project.
Community forks are absolutely welcome and encouraged to provide them, but it would take a very compelling argument to add them to the project.

Note that this is not an exhaustive list. Items may be added or removed as the project evolves.

### Git Hook Management

While Hibiscus Commit CLI can integrate with git hooks (e.g., pre-commit hooks might invoke our tool), the management, installation, and configuration of git hooks themselves is out of scope. Our tool focuses exclusively on generating conventional commit messages, not on enforcing them through hooks or managing the git hook lifecycle.

Users who wish to enforce usage of this tool through git hooks will need to configure those hooks separately or use third-party hook management solutions.

### General Git Operations

Hibiscus Commit CLI is deliberately focused on the commit message authoring experience. General git operations such as branching strategies, merging, rebasing, or repository management are explicitly out of scope. The tool assumes git is already installed and configured on the user's system.

### IDE/Editor Integration

While we provide a command-line interface that can be used anywhere, specific IDE or editor integrations (VS Code extensions, JetBrains plugins, etc.) are out of scope for the core project. The community is welcome to build these integrations using our library.
