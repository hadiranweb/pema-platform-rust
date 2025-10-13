# 🤖 Dependabot Auto-Merge Configuration

This repository is configured with automated dependency management using GitHub's Dependabot and custom auto-merge workflows.

## 📋 Overview

The auto-merge system automatically handles dependency updates while maintaining code quality and security:

- ✅ **Patch & Minor Updates**: Automatically merged after tests pass
- ✅ **Security Updates**: Immediately merged after tests pass
- ✅ **GitHub Actions**: Automatically merged after tests pass
- ⚠️ **Major Updates**: Require manual review due to potential breaking changes

## 🔧 Configuration Files

### 1. Dependabot Configuration (`.github/dependabot.yml`)

**Weekly Updates (Mondays 9:00 UTC):**
- Rust/Cargo dependencies
- GitHub Actions

**Daily Security Checks (6:00 UTC):**
- Security vulnerability fixes

**Dependency Grouping:**
- Related dependencies are grouped together (e.g., `tokio*`, `actix*`, `serde*`)
- Reduces PR noise and ensures compatibility

### 2. Auto-Merge Workflow (`.github/workflows/dependabot-auto-merge.yml`)

**Triggers:**
- Pull request opened/updated by Dependabot
- Only runs on non-draft PRs

**Safety Checks:**
- Waits for all CI tests to pass (`Test Suite` and `Build Check`)
- Validates dependency metadata
- Logs detailed information about updates

## 🚦 Auto-Merge Rules

### ✅ Automatically Merged

1. **Patch Updates** (`1.0.0` → `1.0.1`)
   - Bug fixes and security patches
   - Low risk of breaking changes

2. **Minor Updates** (`1.0.0` → `1.1.0`)
   - New features with backward compatibility
   - Generally safe to auto-merge

3. **Security Updates**
   - Critical security vulnerability fixes
   - Merged immediately after tests pass

4. **GitHub Actions Updates**
   - CI/CD workflow improvements
   - Generally safe and isolated

### ⚠️ Manual Review Required

1. **Major Updates** (`1.0.0` → `2.0.0`)
   - Potential breaking changes
   - Requires human review and testing
   - Auto-commented with review checklist

## 🔍 Workflow Steps

1. **Dependabot creates PR** with dependency update
2. **Metadata extraction** - Analyzes update type and dependencies
3. **CI tests run** - Full test suite including:
   - Code formatting (`cargo fmt`)
   - Linting (`cargo clippy`)
   - Unit and integration tests
   - Build verification
4. **Wait for CI completion** - Workflow waits for all checks to pass
5. **Auto-approval** - Safe updates are automatically approved
6. **Auto-merge** - Approved updates are merged using squash strategy
7. **Success notification** - Comment added with update details

## 📊 Monitoring and Notifications

### Success Notifications
- Detailed comment on successful auto-merge
- Includes dependency names and version changes
- Confirms all tests passed

### Manual Review Notifications
- Major updates receive detailed review instructions
- Includes changelog review checklist
- Highlights potential breaking changes

### Failure Handling
- Failed CI checks prevent auto-merge
- Manual intervention required for test failures
- Detailed logs available in workflow runs

## 🛡️ Security Features

### Security Updates
- **Priority handling** for vulnerability fixes
- **Daily scanning** for new security issues
- **Immediate merge** after tests pass
- **Special labeling** for security PRs

### Safety Measures
- **Draft PR exclusion** - Won't merge draft PRs
- **CI requirement** - All tests must pass
- **Regex validation** - Ensures correct CI check names
- **Verbose logging** - Detailed workflow execution logs

## 🏷️ Labels and Organization

### Automatic Labels
- `dependencies` - All dependency updates
- `rust` - Rust/Cargo dependencies
- `github-actions` - GitHub Actions updates
- `security` - Security vulnerability fixes
- `auto-merge-candidate` - Eligible for auto-merge

### Grouping Strategy
- **Related dependencies** grouped together
- **Reduces PR volume** and merge conflicts
- **Ensures compatibility** between related packages

## 🔧 Customization

### Adjusting Auto-Merge Rules

To modify which updates are auto-merged, edit the conditions in `.github/workflows/dependabot-auto-merge.yml`:

```yaml
# Example: Also auto-merge pre-release updates
if: |
  steps.metadata.outputs.update-type == 'version-update:semver-patch' || 
  steps.metadata.outputs.update-type == 'version-update:semver-minor' ||
  steps.metadata.outputs.update-type == 'version-update:semver-prerelease' ||
  contains(steps.metadata.outputs.dependency-names, 'github-actions')
```

### Adding Dependency Groups

Add new groups to `.github/dependabot.yml`:

```yaml
groups:
  web-framework:
    patterns:
      - "yew*"
      - "wasm-*"
```

### Changing Schedule

Modify the schedule in `.github/dependabot.yml`:

```yaml
schedule:
  interval: "daily"  # or "weekly", "monthly"
  time: "06:00"
  timezone: "UTC"
```

## 📈 Benefits

### Development Efficiency
- **Reduced manual work** - No need to manually merge safe updates
- **Faster security fixes** - Critical updates merged immediately
- **Consistent updates** - Regular, predictable dependency maintenance

### Code Quality
- **Always tested** - No updates merged without passing tests
- **Grouped updates** - Related dependencies updated together
- **Detailed logging** - Full audit trail of all changes

### Security
- **Rapid response** - Security fixes applied quickly
- **Automated scanning** - Daily security vulnerability checks
- **Safe defaults** - Conservative auto-merge rules

## 🚨 Troubleshooting

### Auto-Merge Not Working

1. **Check CI status** - Ensure all tests are passing
2. **Verify PR author** - Must be `dependabot[bot]`
3. **Check update type** - Major updates require manual review
4. **Review workflow logs** - Check for errors in auto-merge workflow

### False Positives

If safe updates aren't being auto-merged:

1. **Check dependency metadata** - Ensure proper semver classification
2. **Review CI check names** - Verify they match workflow expectations
3. **Check PR labels** - Ensure proper labeling by Dependabot

### Manual Override

To manually merge a Dependabot PR:

1. **Review the changes** thoroughly
2. **Run additional tests** if needed
3. **Approve the PR** manually
4. **Merge using squash** to maintain clean history

## 📚 Resources

- [Dependabot Documentation](https://docs.github.com/en/code-security/dependabot)
- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Semantic Versioning](https://semver.org/)
- [Rust Security Advisory Database](https://rustsec.org/)

---

*This auto-merge system is designed to balance automation with safety, ensuring dependencies stay up-to-date while maintaining code quality and security.*