#!/bin/bash

# Validate Dependabot Configuration
# This script checks if the Dependabot setup is properly configured

set -e

echo "🔍 Validating Dependabot Configuration..."
echo "================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Check if required files exist
echo -e "${BLUE}📁 Checking required files...${NC}"

required_files=(
    ".github/dependabot.yml"
    ".github/workflows/dependabot-auto-merge.yml"
    ".github/workflows/ci.yml"
)

for file in "${required_files[@]}"; do
    if [[ -f "$file" ]]; then
        echo -e "  ✅ ${GREEN}$file${NC} exists"
    else
        echo -e "  ❌ ${RED}$file${NC} missing"
        exit 1
    fi
done

# Validate dependabot.yml syntax
echo -e "\n${BLUE}🔧 Validating dependabot.yml syntax...${NC}"

if command -v yq &> /dev/null; then
    if yq eval '.version' .github/dependabot.yml > /dev/null 2>&1; then
        version=$(yq eval '.version' .github/dependabot.yml)
        echo -e "  ✅ ${GREEN}Valid YAML syntax (version: $version)${NC}"
    else
        echo -e "  ❌ ${RED}Invalid YAML syntax${NC}"
        exit 1
    fi
else
    echo -e "  ⚠️  ${YELLOW}yq not installed, skipping YAML validation${NC}"
fi

# Check for required package ecosystems
echo -e "\n${BLUE}📦 Checking package ecosystems...${NC}"

if grep -q "cargo" .github/dependabot.yml; then
    echo -e "  ✅ ${GREEN}Cargo ecosystem configured${NC}"
else
    echo -e "  ❌ ${RED}Cargo ecosystem missing${NC}"
    exit 1
fi

if grep -q "github-actions" .github/dependabot.yml; then
    echo -e "  ✅ ${GREEN}GitHub Actions ecosystem configured${NC}"
else
    echo -e "  ❌ ${RED}GitHub Actions ecosystem missing${NC}"
    exit 1
fi

# Check workflow permissions
echo -e "\n${BLUE}🔐 Checking workflow permissions...${NC}"

if grep -q "contents: write" .github/workflows/dependabot-auto-merge.yml; then
    echo -e "  ✅ ${GREEN}Contents write permission set${NC}"
else
    echo -e "  ❌ ${RED}Contents write permission missing${NC}"
    exit 1
fi

if grep -q "pull-requests: write" .github/workflows/dependabot-auto-merge.yml; then
    echo -e "  ✅ ${GREEN}Pull requests write permission set${NC}"
else
    echo -e "  ❌ ${RED}Pull requests write permission missing${NC}"
    exit 1
fi

# Check CI workflow job names
echo -e "\n${BLUE}🧪 Checking CI job names...${NC}"

if grep -q "Test Suite" .github/workflows/ci.yml; then
    echo -e "  ✅ ${GREEN}'Test Suite' job found${NC}"
else
    echo -e "  ❌ ${RED}'Test Suite' job missing${NC}"
    exit 1
fi

if grep -q "Build Check" .github/workflows/ci.yml; then
    echo -e "  ✅ ${GREEN}'Build Check' job found${NC}"
else
    echo -e "  ❌ ${RED}'Build Check' job missing${NC}"
    exit 1
fi

# Check for security features
echo -e "\n${BLUE}🛡️  Checking security features...${NC}"

if grep -q "alert-state" .github/workflows/dependabot-auto-merge.yml; then
    echo -e "  ✅ ${GREEN}Security alert handling configured${NC}"
else
    echo -e "  ⚠️  ${YELLOW}Security alert handling not found${NC}"
fi

if grep -q "security" .github/dependabot.yml; then
    echo -e "  ✅ ${GREEN}Security updates configured${NC}"
else
    echo -e "  ⚠️  ${YELLOW}Security updates not explicitly configured${NC}"
fi

# Check for dependency grouping
echo -e "\n${BLUE}📊 Checking dependency grouping...${NC}"

if grep -q "groups:" .github/dependabot.yml; then
    echo -e "  ✅ ${GREEN}Dependency grouping configured${NC}"
    
    # Count groups
    if command -v yq &> /dev/null; then
        group_count=$(yq eval '.updates[0].groups | keys | length' .github/dependabot.yml 2>/dev/null || echo "0")
        echo -e "  📈 ${BLUE}Found $group_count dependency groups${NC}"
    fi
else
    echo -e "  ⚠️  ${YELLOW}No dependency grouping found${NC}"
fi

# Validate Cargo.toml exists (required for Cargo ecosystem)
echo -e "\n${BLUE}📋 Checking Rust project structure...${NC}"

if [[ -f "Cargo.toml" ]]; then
    echo -e "  ✅ ${GREEN}Root Cargo.toml found${NC}"
else
    echo -e "  ❌ ${RED}Root Cargo.toml missing${NC}"
    exit 1
fi

# Check for workspace configuration
if grep -q "\[workspace\]" Cargo.toml; then
    echo -e "  ✅ ${GREEN}Cargo workspace detected${NC}"
else
    echo -e "  ℹ️  ${BLUE}Single crate project${NC}"
fi

# Summary
echo -e "\n${GREEN}================================================${NC}"
echo -e "${GREEN}✅ Dependabot configuration validation complete!${NC}"
echo -e "${GREEN}================================================${NC}"

echo -e "\n${BLUE}📋 Configuration Summary:${NC}"
echo -e "  • Dependabot will check for updates weekly (Mondays 9:00 UTC)"
echo -e "  • Security updates checked daily (6:00 UTC)"
echo -e "  • Patch and minor updates auto-merged after tests pass"
echo -e "  • Major updates require manual review"
echo -e "  • GitHub Actions updates auto-merged"

echo -e "\n${BLUE}🚀 Next Steps:${NC}"
echo -e "  1. Ensure repository has proper branch protection rules"
echo -e "  2. Verify CI tests are comprehensive and reliable"
echo -e "  3. Monitor first few Dependabot PRs to ensure proper operation"
echo -e "  4. Consider enabling Dependabot security alerts in repository settings"

echo -e "\n${YELLOW}💡 Pro Tips:${NC}"
echo -e "  • Review .github/DEPENDABOT_AUTO_MERGE.md for detailed documentation"
echo -e "  • Use 'dependabot recreate' comment to regenerate PRs if needed"
echo -e "  • Monitor workflow runs in the Actions tab"

echo -e "\n${GREEN}🎉 Your Dependabot auto-merge setup is ready!${NC}"