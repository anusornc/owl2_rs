# Development Workflow Guidelines

This document outlines the proper development workflow for contributing to the owl2_rs project.

## Branching Strategy

### Main Branch
- `main` - The stable, production-ready branch
- Only receives merges from completed, tested, and reviewed feature branches
- Always builds successfully and passes all tests

### Feature Branches
- Create a new branch from `main` for each feature or bug fix
- Use descriptive names: `feature/descriptive-name` or `fix/issue-description`
- Keep branches focused on specific, well-defined goals
- Delete branches after merging to keep the repository clean

### Example:
```bash
# Start a new feature
git checkout main
git pull origin main
git checkout -b feature/add-rdf-xml-support

# Work on the feature
# ...make changes...
git add .
git commit -m "Add RDF/XML parser support"

# Keep branch up to date with main
git checkout main
git pull origin main
git checkout feature/add-rdf-xml-support
git merge main

# Push and create PR
git push origin feature/add-rdf-xml-support
```

## Development Process

### Before Starting Work
1. Ensure you're on the latest `main` branch
2. Create a new feature branch with a descriptive name
3. Understand the feature requirements and scope

### During Development
1. Make frequent, small commits with clear messages
2. Run tests regularly to catch issues early
3. Keep dependencies updated and documented
4. Write clear documentation for new features
5. Follow existing code style and conventions

### Before Merging
1. Ensure all tests pass
2. Update documentation if needed
3. Squash related commits for cleaner history
4. Rebase on latest `main` to resolve conflicts
5. Create a Pull Request with clear description

### After Merging
1. Delete the feature branch
2. Sync local `main` branch
3. Start next feature from clean `main`

## Code Quality Standards

### Compilation
- All code must compile without errors
- No warnings in production code
- Proper error handling throughout

### Testing
- All new features must include unit tests
- Existing tests must continue to pass
- Integration tests for complex features
- Performance benchmarks for optimization work

### Documentation
- Public APIs must be documented
- Complex algorithms should include comments
- Examples for new features
- README updates for major changes

## Conflict Resolution

### Prevention
1. Regularly merge `main` into feature branches
2. Communicate with team about overlapping work
3. Keep feature branches short-lived
4. Break large features into smaller, incremental changes

### Resolution
1. Address conflicts immediately when they arise
2. Test thoroughly after resolving conflicts
3. Ask for help when conflicts are complex
4. Consider breaking changes into smaller commits

## Best Practices

1. **Small, Focused Commits**: Each commit should represent a single logical change
2. **Descriptive Commit Messages**: Use present tense, be clear about what changed
3. **Regular Testing**: Run `cargo test` frequently during development
4. **Code Reviews**: All changes should be reviewed before merging
5. **Performance Awareness**: Monitor performance impacts of changes
6. **Backwards Compatibility**: Maintain compatibility when possible
7. **Security Consciousness**: Follow security best practices
8. **Documentation First**: Document features as you develop them

## Emergency Procedures

### Broken Main Branch
1. Identify the problematic commit
2. Revert the commit if necessary
3. Fix the issue in a new branch
4. Thoroughly test before merging back

### Critical Bug Found
1. Create hotfix branch from `main`
2. Fix the bug with minimal changes
3. Thoroughly test the fix
4. Merge hotfix with urgency but care
5. Delete hotfix branch