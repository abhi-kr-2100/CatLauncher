# Refactoring Report

This report details the inconsistencies found in the codebase compared to the standards outlined in `AGENTS.md`.

## Frontend

### Internationalization

The following files contained hardcoded strings that were displayed to the user. This violated the internationalization guidelines, which state that all user-facing strings should be internationalization-ready.

**`cat-launcher/src/components/AutoUpdateNotifier.tsx`**

- **Inconsistency:** Hardcoded strings "Autoupdate Failed" and "Close".
- **Fix:** Replaced these strings with the `useTranslation` hook and translation keys.

```tsx
<DialogTitle>{t("autoUpdateFailed")}</DialogTitle>
...
<Button>{t("close")}</Button>
```

**`cat-launcher/src/components/DataTable.tsx`**

- **Inconsistency:** Hardcoded string "No results.".
- **Fix:** Replaced with the `useTranslation` hook and a translation key.

```tsx
<TableCell
  colSpan={columns.length}
  className="h-24 text-center"
>
  {t("noResults")}
</TableCell>
```

**`cat-launcher/src/components/DownloadProgress.tsx`**

- **Inconsistency:** Hardcoded string "Downloading...".
- **Fix:** Replaced with the `useTranslation` hook and a translation key.

```tsx
{isIndeterminate
    ? `${t("downloading")} ${formatBytes(downloaded).join(" ")}`
    : t("downloading")}
```

**`cat-launcher/src/components/GameSessionMonitor.tsx`**

- **Inconsistency:** Multiple hardcoded strings.
- **Fix:** Replaced all hardcoded strings with the `useTranslation` hook and translation keys.

```tsx
const title =
  gameStatus === GameStatus.CRASHED
    ? t("gameCrashed")
    : gameStatus === GameStatus.TERMINATED
      ? t("gameTerminated")
      : gameStatus === GameStatus.ERROR
        ? t("gameExitedUnexpectedly")
        : null;
...
<DialogDescription>{t("diagnoseIssue")}</DialogDescription>
...
<h3 className="font-semibold">{t("exitCode")}</h3>
...
{exitCode ?? t("unknown")}
...
<h3 className="font-semibold">{t("logs")}</h3>
...
toastCL("success", t("logsCopied"));
...
toastCL("error", t("errorCopyingLogs"), error);
...
<Copy />
{t("copyLogs")}
...
<Button>{t("close")}</Button>
```
