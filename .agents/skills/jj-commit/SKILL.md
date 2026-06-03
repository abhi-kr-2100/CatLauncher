---
name: jj-commit
description: Commit changes using Jujutsu VCS with Conventional Commits format.
license: CC BY-NC-ND 4.0
metadata:
  author: abhi-kr-2100
  version: "1.0"
---

# Jujutsu Commit Skill

Use this skill to commit changes in a Jujutsu (jj) version-controlled repository following the [Conventional Commits](https://www.conventionalcommits.org/) specification.

## When to Use

- When you need to commit changes in a Jujutsu repository
- When the user requests a commit with a specific message
- When you need to review uncommitted changes before committing

## Workflow

### 1. Review Uncommitted Changes

Always check the uncommitted changes first using:

```bash
jj diff --git --no-pager
```

### 2. Write the Commit Message

Follow the **Conventional Commits** format strictly:

#### Header (Required)

```
type(scope): description
```

- **type** (required): One of:
  - `feat` - A new feature
  - `fix` - A bug fix
  - `docs` - Documentation only changes
  - `refactor` - A code change that neither fixes a bug nor adds a feature
  - `perf` - A code change that improves performance
  - `style` - Changes that do not affect the meaning of the code (white-space, formatting, missing semi-colons, etc)
  - `test` - Adding missing tests or correcting existing tests
  - `chore` - Other changes that don't modify src or test files
  - `ci` - Changes to CI configuration files and scripts
  - `revert` - Reverts a previous commit
  - `build` - Changes that affect the build system or external dependencies

- **scope** (optional): The name of the feature or module being modified (e.g., `user`, `auth`, `api`)

- **description** (required): A brief summary of the change in lowercase, imperative mood (e.g., "add user authentication" not "added user authentication")

#### Body (Optional)

Include when the change needs more explanation. Structure it as:

```
Motivation:
- [Reason for the change]

Changes:
- [List of specific changes made]
```

#### Footer (Optional)

Include for:
- Breaking changes: `BREAKING CHANGE: [description]`
- Issue references: `Closes #123`, `Fixes #456`, `Resolves #789`
- Related PRs: `See also: #123`

### 3. Commit the Changes

Use the following command to commit:

```bash
jj desc -m "{commit_message}"
```

Where `{commit_message}` is your full Conventional Commits formatted message.

## Examples

```bash
jj desc -m "fix(api): validate input parameters

Motivation:
- Prevent SQL injection vulnerabilities.

Changes:
- Add input validation middleware.
- Sanitize all user inputs.

BREAKING CHANGE: API now rejects requests with invalid parameters."
```
