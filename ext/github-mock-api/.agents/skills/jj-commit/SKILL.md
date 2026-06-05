---
name: jj-commit
description: Commit changes using Jujutsu VCS with Conventional Commits format.
license: CC BY-NC-ND 4.0
metadata:
  author: abhi-kr-2100
  version: "1.1"
---

# Jujutsu Commit Skill

Use this skill when
- you need to commit changes
- the user requests you to commit changes

## Allowed Commands

You're allowed to run only the following commands:

- jj status --no-pager # to list changed files
- jj diff --git --no-pager [optional-file-path]
- jj desc -m <commit-message>

You must not run any other `jj` command. You must not run `git`.

## Commit Message Format

Commit messages should follow the Conventional Commits format:

- **Header**: `type(scope): description`
  - **Type**: One of `feat`, `fix`, `docs`, `refactor`, `perf`, `style`, `test`, `chore`, `ci`, `revert`, `build`.
  - **Scope** (optional): The name of the feature or module being modified.
  - **Description**: A brief summary of the change.
- **Body** (optional): A detailed description of the change. Start with the motivation for the change and then list the changes made.
- **Footer** (optional): Any additional information about the change, like `BREAKING CHANGE` notices or issue references (e.g., `Closes #123`).
- **Co-Authored-By:**: Name of the agent with an optional email.

Example:

```
feat(user): add user authentication

Motivation:
- To secure user accounts and provide personalized experiences.

Changes:
- Add a new user model.
- Add a new user repository.

BREAKING CHANGE: Authentication is now required for all API endpoints.

Co-Authored-By: Agent0 <agent@zero.com>
```
