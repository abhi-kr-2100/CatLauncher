# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared against the coding standards and advice mentioned in `AGENTS.md`. It also includes the suggested fixes for each inconsistency.

## Frontend

### 1. Raw `useQuery` Hook Usage

- **File**: `cat-launcher/src/hooks/useGameVariants.ts`
- **Inconsistency**: The `useQuery` hook is used directly, which goes against the guideline of wrapping `useQuery` and `useMutation` in custom hooks.
- **Suggested Fix**: Abstract the `useQuery` call into a custom hook that accepts an `onError` callback for improved error handling.

### 2. Missing Error Handling Callback in Custom Hook

- **File**: `cat-launcher/src/hooks/useGameVariants.ts`
- **Inconsistency**: The `useGameVariants` custom hook does not accept an `onError` callback, which is recommended for handling different error scenarios.
- **Suggested Fix**: Modify the `useGameVariants` hook to accept an `onError` callback and pass it to the underlying `useQuery` hook.

## Backend

### 1. Missing `CommandErrorSerialize` Trait

- **File**: `cat-launcher/src-tauri/src/variants/commands.rs`
- **Inconsistency**: The `GetGameVariantsError` enum does not derive the `CommandErrorSerialize` trait, which is required for all error enums in Tauri commands.
- **Suggested Fix**: Add the `CommandErrorSerialize` trait to the `GetGameVariantsError` enum to ensure proper error serialization.
