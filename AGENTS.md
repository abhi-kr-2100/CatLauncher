# Frontend

* Tools: `pnpm`, `shadcn/ui`, `lucide`, `tailwind`, `tanstack-query`, `redux-toolkit`

## Verification

At the end of every task, run the following commands in order:

* `rm -rf cat-launcher/src/generated-types && cargo test --manifest-path cat-launcher/src-tauri/Cargo.toml`: This will generate TypeScript types from Rust types using `ts-rs`.
* `pnpm --prefix cat-launcher format && pnpm --prefix cat-launcher lint:fix`
* `pnpm --prefix cat-launcher lint`: To ensure there are no errors.

You should not run any other build, test, or dev commands.

## UI

`shadcn/ui` is used for UI components. To add a new component, run:

* `pnpm --prefix cat-launcher dlx shadcn@latest add {component_name}`: This will install the component in the `cat-launcher/src/components/ui` directory.

You can browse the documentation of a component by visiting `https://ui.shadcn.com/docs/components/{component_name}`

You can combine shadcn/ui components to create helper components. Keep these helper components in the `cat-launcher/src/components` directory, separate from the shadcn/ui components.

`lucide` is used for icons, and `tailwind` is used for styling. Never write manual CSS; always prefer `tailwind`.

## Data Fetching and Mutations

`tanstack-query` is used for data fetching and mutations.

* Raw `useQuery` and `useMutation` hooks are not used. Instead create custom hooks that wrap `useQuery` and `useMutation`.
* All query keys must be stored in the `cat-launcher/src/lib/queryKeys.ts` file.

## Strings

* All strings displayed to the user should be internationalization-ready.
* Avoid string construction in parts to avoid problems in non-English languages where grammar and word order differ.
* Avoid string manipulation to format a string value into a user-displayable value as this might not work during translation. Instead, use a mapping function or constant that maps values to displayable labels.

## Directory Structure

* `cat-launcher/src/components`: General and reusable helper components used by many frontend features.
* `cat-launcher/src/hooks`: General and reusable hooks used by many frontend features.
* `cat-launcher/src/lib`: General and reusable utilities used by many frontend features.
* `cat-launcher/src/store`: General and reusable Redux store used by many frontend features.
* `cat-launcher/src/pages/{feature_name}`: A single self-contained frontend feature.
  - `cat-launcher/src/pages/{feature_name}/index.tsx`: The main page component.
  - `cat-launcher/src/pages/{feature_name}/components`: Components specific to this feature.
  - `cat-launcher/src/pages/{feature_name}/hooks`: Hooks specific to this feature.
  - `cat-launcher/src/pages/{feature_name}/lib`: Utilities specific to this feature.
  - `cat-launcher/src/pages/{feature_name}/store`: Redux store specific to this feature.
  
Always follow the directory structure above when creating a new feature.

# Backend

* Tools: `tauri`, `thiserror`, `tokio`

## Commands

* The backend implements Tauri commands which are used by the frontend.
* Commands should be straightforward and should not perform business logic.
* Commands should prepare arguments to pass to business logic functions. The entire context or framework-dependent data should not be passed to business logic functions, in particular, the Tauri `AppHandle` and `App` objects.
* Error thrown by command should derive `thiserror::Error`, `Debug`, `IntoStaticStr`, `CommandErrorSerialize`.

```rust
// Good commands

#[command]
pub async fn get_active_release(
    variant: GameVariant, active_release_repo: State<'_, SqliteActiveReleaseRepository>,
) -> Result<Option<String>, ActiveReleaseCommandError> {
  // Collect all arguments to pass to the business logic function
  let repo = active_release_repo.inner();

  // Call the business logic function
  let active_release = variant.get_active_release(repo).await?;

  // Return the results
  Ok(active_release)
}

// Error Handling
#[derive(thiserror::Error, Debug, IntoStaticStr, CommandErrorSerialize)]
pub enum ActiveReleaseCommandError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseError),

  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] tauri::Error),
} 
```

## Error Handling

* Use `thiserror::Error`, `#[error]` and `#[from]` from the `thiserror` crate to define errors.
* Define one error enum for every function. The error name should be `{FunctionName}Error`
* Don't convert errors to strings. Use `#[from]` to compose errors.

```rust
// Good error handling
#[derive(thiserror::Error, Debug)]
pub enum GetAllTipsError {
  #[error("failed to get active release: {0}")]
  GetActiveRelease(#[from] ActiveReleaseError),

  #[error("failed to get system directory: {0}")]
  SystemDirectory(#[from] io::Error),
}

pub fn get_all_tips() -> Result<(), GetAllTipsError> {
  let active_release = get_active_release()?;
  let system_dir = get_system_directory()?;
  
  Ok(())
}
```

## Repository

* Data is stored in a SQLite database.
* However, business logic functions don't depend on the SQLite database directly.
* Define a Repository trait independent of SQLite that business logic functions depend on.
* Implement the Repository trait for SQLite.

## Directory Structure

* `cat-launcher/src-tauri/`: All Rust code. No Rust code should be outside this directory.
* `cat-launcher/src-tauri/src/{feature_name}`: A single self-contained backend feature.
  - `cat-launcher/src-tauri/src/{feature_name}/mod.rs`: Module declaration for this feature. This file should only contain `mod` and `use` statements.
  - `cat-launcher/src-tauri/src/{feature_name}/commands.rs`: Tauri commands defined for this feature.
  - `cat-launcher/src-tauri/src/{feature_name}/lib.rs`: Utilities specific to this feature.
  - `cat-launcher/src-tauri/src/{feature_name}/types.rs`: Enums, structs, and other data types specific to this feature.
  - `cat-launcher/src-tauri/src/{feature_name}/repository`: Repository specific to this feature. It should contain an abstract repository trait and its implementation in two separate files.
  - `cat-launcher/src-tauri/src/{feature_name}/{business_logic_files}`: Files implementing the business logic used in `commands.rs`.

# Agent Responsibility

* At the end of each task, run the Verification commands to ensure correctness.
* Run `git branch --show-current` to find the current branch name. If current branch is not `main`, `release`, or `gitbutler/workspace`, commit the changes with an appropriate commit message.
