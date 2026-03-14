# Refactoring Report

This report identifies inconsistencies between the current codebase and the standards defined in `AGENTS.md`.

## 1. Frontend Hooks - Error Handling Pattern

The `AGENTS.md` specifies that custom hooks wrapping `useQuery` and `useMutation` should take error callbacks and use `useRef`/`useEffect` to handle them. Several hooks in the project do not follow this pattern or follow it only partially (missing `useRef` or `useEffect` sync).

### Identified Inconsistencies:

- **Global Hooks (`cat-launcher/src/hooks/`):**
    - `useDeleteBackup.ts`: Takes `onError` but doesn't use `useRef`/`useEffect` pattern.
    - `useRestoreBackup.ts`: Takes `onError` but doesn't use `useRef`/`useEffect` pattern.
    - `useManualBackups.ts`: Missing error callbacks and `useEffect` sync.
    - `useDeleteManualBackup.ts`: Takes `onError` but doesn't use `useRef`/`useEffect` pattern.
    - `useBackups.ts`: Missing error callbacks and `useEffect` sync.
    - `useRestoreManualBackup.ts`: Takes `onError` but doesn't use `useRef`/`useEffect` pattern.
    - `useCreateManualBackup.ts`: Takes `onError` but doesn't use `useRef`/`useEffect` pattern.

- **Page-specific Hooks:**
    - `cat-launcher/src/pages/game-tips/hooks/useGetTips.ts`: Missing error callbacks and `useEffect` sync.
    - `cat-launcher/src/pages/TilesetsPage/hooks.ts`: `useGetThirdPartyTilesetInstallationStatus` and `useListAllTilesets` missing callbacks. `useUninstallThirdPartyTileset` doesn't use `useRef`/`useEffect`.
    - `cat-launcher/src/pages/ModsPage/hooks/useUninstallThirdPartyMod.ts`: Doesn't use `useRef`/`useEffect`.
    - `cat-launcher/src/pages/ModsPage/hooks/useGetThirdPartyModInstallationStatus.ts`: Missing error callbacks.
    - `cat-launcher/src/pages/PlayPage/hooks/useLaunchGame.ts`: Takes `onError` but uses it directly in `useMutation` without `useRef`.
    - `cat-launcher/src/pages/SoundpacksPage/hooks.ts`: `useGetThirdPartySoundpackInstallationStatus` and `useListAllSoundpacks` missing callbacks. `useUninstallThirdPartySoundpack` doesn't use `useRef`/`useEffect`.

### Suggested Fixes:
Refactor these hooks to:
1. Accept optional error callback(s).
2. Store callbacks in `useRef`.
3. Update `useRef` in `useEffect` when callbacks change.
4. Use `useEffect` to trigger the callback when the `error` state from `useQuery` or `useMutation` changes.

---

## 2. Raw `useQuery`/`useMutation` Usage

`AGENTS.md` states: "Raw `useQuery` and `useMutation` hooks are not used. Instead create custom hooks that wrap `useQuery` and `useMutation`."

### Identified Inconsistencies:
- `cat-launcher/src/providers/PostHogProviderWithIdentifiedUser.tsx`: Uses `useQuery` directly to fetch `userId`.

### Suggested Fixes:
- Create a new hook `useUserId` in `cat-launcher/src/hooks/useUserId.ts` and use it in `PostHogProviderWithIdentifiedUser.tsx`.

---

## 3. Directory Structure

`AGENTS.md` specifies a directory structure for features:
`cat-launcher/src/pages/{feature_name}/hooks`: Hooks specific to this feature.

### Identified Inconsistencies:
- `cat-launcher/src/pages/SoundpacksPage/hooks.ts`: Should be in a `hooks/` directory and potentially split if it contains multiple hooks.
- `cat-launcher/src/pages/TilesetsPage/hooks.ts`: Should be in a `hooks/` directory and potentially split.

### Suggested Fixes:
- Move `cat-launcher/src/pages/SoundpacksPage/hooks.ts` to `cat-launcher/src/pages/SoundpacksPage/hooks/index.ts` (or individual files).
- Move `cat-launcher/src/pages/TilesetsPage/hooks.ts` to `cat-launcher/src/pages/TilesetsPage/hooks/index.ts` (or individual files).
