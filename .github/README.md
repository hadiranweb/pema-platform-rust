# GitHub Actions & Dependabot Configuration

This directory contains GitHub Actions workflows and Dependabot configuration for the PEMA Platform Rust project.

## Workflows

### 1. CI Workflow (`ci.yml`)
- **Triggers**: Push to `main`/`mine` branches, Pull Requests
- **Jobs**:
  - **Test Suite**: Runs formatting checks, clippy linting, and all tests with PostgreSQL database
  - **Build Check**: Verifies that the entire workspace builds successfully
- **Features**:
  - Caches Cargo dependencies for faster builds
  - Sets up PostgreSQL service for database tests
  - Runs database migrations before tests
  - Validates code formatting and linting

### 2. Dependabot Auto-Merge (`dependabot-auto-merge.yml`)
- **Triggers**: Pull Requests from Dependabot
- **Behavior**:
  - **Patch & Minor Updates**: Automatically approved and merged after CI passes
  - **Major Updates**: Commented for manual review (not auto-merged)
- **Safety**: Only runs after both Test Suite and Build Check pass
- **Permissions**: Requires `contents: write` and `pull-requests: write`

## Dependabot Configuration (`dependabot.yml`)

Automatically creates pull requests for:
- **Cargo dependencies**: Weekly on Mondays at 9:00 AM
- **GitHub Actions**: Weekly on Mondays at 9:00 AM

### Configuration Details:
- **Review Limits**: Max 10 Cargo PRs, 5 GitHub Actions PRs
- **Labels**: Automatically tagged with `dependencies` and ecosystem-specific labels
- **Assignees/Reviewers**: Set to repository owner
- **Commit Messages**: Prefixed with `deps:` for Cargo, `ci:` for GitHub Actions

## Security Considerations

1. **Auto-merge is limited to patch and minor updates** - Major version updates require manual review
2. **All tests must pass** before auto-merge occurs
3. **Uses GitHub's built-in GITHUB_TOKEN** - no additional secrets required
4. **Dependabot metadata validation** ensures only legitimate dependency updates are processed

## Setup Requirements

To enable this configuration:

1. **Repository Settings**: Ensure "Allow auto-merge" is enabled in repository settings
2. **Branch Protection**: Consider setting up branch protection rules for `main` branch
3. **Permissions**: The workflows use standard GitHub permissions - no additional setup needed

## Customization

To modify the auto-merge behavior:
- Edit `dependabot-auto-merge.yml` to change which update types are auto-merged
- Modify `dependabot.yml` to adjust update frequency or add/remove package ecosystems
- Update `ci.yml` to add additional checks or modify test configuration