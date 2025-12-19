# Refactoring Report

This document outlines the inconsistencies found in the codebase when compared to the standards defined in `AGENTS.md`.

## Frontend

### Directory Structure

- **`AboutPage.tsx`:** This file is a standalone component in the `pages` directory. It should be moved to its own directory `pages/AboutPage/index.tsx`.
- **`BackupsPage`:**
    - Components are not in a `components` subdirectory. All `.tsx` files except `index.tsx` should be moved to `pages/BackupsPage/components/`.
    - The `types` directory is not specified in the `AGENTS.md` file. Its contents should be moved to a more appropriate location, such as `lib` or colocated with the components that use them.
- **`ModsPage`:**
    - Components are not in a `components` subdirectory. All `.tsx` files except `index.tsx` should be moved to `pages/ModsPage/components/`.
    - `hooks.ts` is not in a `hooks` subdirectory. It should be moved to `pages/ModsPage/hooks/hooks.ts`.
- **`PlayPage`:**
    - Components are not in a `components` subdirectory. All `.tsx` files except `index.tsx` should be moved to `pages/PlayPage/components/`.
    - `hooks.ts` is not in a `hooks` subdirectory. It should be moved to `pages/PlayPage/hooks/hooks.ts`.
- **`SoundpacksPage`:**
    - Components are not in a `components` subdirectory. All `.tsx` files except `index.tsx` should be moved to `pages/SoundpacksPage/components/`.
    - `hooks.ts` is not in a `hooks` subdirectory. It should be moved to `pages/SoundpacksPage/hooks/hooks.ts`.
- **`TilesetsPage`:**
    - Components are not in a `components` subdirectory. All `.tsx` files except `index.tsx` should be moved to `pages/TilesetsPage/components/`.
    - `hooks.ts` is not in a `hooks` subdirectory. It should be moved to `pages/TilesetsPage/hooks/hooks.ts`.

### Data Fetching

- **Raw `useQuery` and `useMutation` hooks are used.** `AGENTS.md` requires that all `useQuery` and `useMutation` hooks are wrapped in custom hooks. The following files contain raw `useQuery` or `useMutation` calls:
    - `cat-launcher/src/pages/ModsPage/hooks.ts`
    - `cat-launcher/src/pages/ModsPage/ModsList.tsx`
    - `cat-launcher/src/pages/PlayPage/ReleaseSelector.tsx`
    - `cat-launcher/src/pages/PlayPage/hooks.ts`
    - `cat-launcher/src/pages/TilesetsPage/hooks.ts`
    - `cat-launcher/src/pages/SoundpacksPage/hooks.ts`
    - `cat-launcher/src/providers/PostHogProviderWithIdentifiedUser.tsx`
    - `cat-launcher/src/hooks/useDeleteManualBackup.ts`
    - `cat-launcher/src/hooks/useBackups.ts`
    - `cat-launcher/src/hooks/useCreateManualBackup.ts`
    - `cat-launcher/src/hooks/useGameVariants.ts`
    - `cat-launcher/src/hooks/useDeleteBackup.ts`
    - `cat-launcher/src/hooks/useManualBackups.ts`
    - `cat-launcher/src/game-tips/TipOfTheDay.tsx`
    - `cat-launcher/src/theme/useTheme.ts`

### UI

I will analyze the UI implementation next.

### Strings

I will analyze the usage of strings next.

## Backend

I will analyze the backend implementation next.
