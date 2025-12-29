import { useEffect, useMemo, useRef, useState } from "react";

import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import type { ReleaseNotesUpdatePayload } from "@/generated-types/ReleaseNotesUpdatePayload";
import {
  listenToReleaseNotesUpdate,
  triggerFetchReleaseNotesForVariant,
} from "@/lib/commands";
import { cn, setupEventListener, toastCL } from "@/lib/utils";
import { useInstallAndMonitorRelease } from "./hooks";
import ReleaseLabel from "./ReleaseLabel";

const NO_RELEASE_NOTES_TEXT = "No release notes.";
const RELEASE_NOTES_LOAD_ERROR_TEXT = "Failed to load release notes.";

type CompareKind = "upgrade" | "downgrade" | "same";

export function ReleaseNotesDialog({
  open,
  onOpenChange,
  variant,
  releases,
  currentActiveRelease,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseNotesDialogProps) {
  const normalizedCurrentActive =
    currentActiveRelease && currentActiveRelease !== ""
      ? currentActiveRelease
      : undefined;

  const [compareFrom, setCompareFrom] = useState<
    string | undefined
  >();
  const [compareTo, setCompareTo] = useState<string | undefined>();

  useEffect(() => {
    if (!open) {
      return;
    }

    const initialFrom = normalizedCurrentActive;
    const initialTo = selectedReleaseId ?? normalizedCurrentActive;

    setCompareFrom(initialFrom);
    setCompareTo(initialTo);
  }, [open, normalizedCurrentActive, selectedReleaseId]);

  const { compareKind, versionsToShow } = useMemo(() => {
    return getVersionsToShow(releases, compareFrom, compareTo);
  }, [releases, compareFrom, compareTo]);

  const confirmText =
    compareKind === "same"
      ? "Already Installed"
      : compareKind === "downgrade"
        ? "Downgrade"
        : "Upgrade";

  const confirmVariant =
    compareKind === "downgrade" ? "destructive" : "default";

  const confirmDisabled = compareKind === "same" || !compareTo;

  const [notesByVersion, setNotesByVersion] = useState<
    Record<
      string,
      {
        status: "loading" | "loaded" | "error";
        notes?: string | null;
      }
    >
  >({});

  const requestIdRef = useRef<string | null>(null);

  useEffect(() => {
    const handler = (payload: ReleaseNotesUpdatePayload) => {
      if (payload.variant !== variant) {
        return;
      }

      if (payload.request_id !== requestIdRef.current) {
        return;
      }

      if (payload.version === null) {
        return;
      }

      if (
        payload.status === "Cached" ||
        payload.status === "Fetched"
      ) {
        setNotesByVersion((prev) => ({
          ...prev,
          [payload.version!]: {
            status: "loaded",
            notes: payload.notes ?? "",
          },
        }));
        return;
      }

      if (payload.status === "Error") {
        setNotesByVersion((prev) => ({
          ...prev,
          [payload.version!]: {
            status: "error",
          },
        }));
      }
    };

    const cleanup = setupEventListener(
      listenToReleaseNotesUpdate,
      handler,
      "Failed to subscribe to release notes updates.",
    );

    return cleanup;
  }, [variant]);

  useEffect(() => {
    if (!open) {
      return;
    }

    if (!compareTo || versionsToShow.length === 0) {
      return;
    }

    const requestId = crypto.randomUUID();
    requestIdRef.current = requestId;

    setNotesByVersion(() =>
      versionsToShow.reduce(
        (acc, v) => {
          acc[v] = { status: "loading" };
          return acc;
        },
        {} as Record<
          string,
          {
            status: "loading" | "loaded" | "error";
            notes?: string | null;
          }
        >,
      ),
    );

    triggerFetchReleaseNotesForVariant(
      variant,
      requestId,
      versionsToShow,
    ).catch((error) => {
      toastCL("error", "Failed to fetch release notes.", error);
    });
  }, [open, variant, compareTo, versionsToShow]);

  const { install } = useInstallAndMonitorRelease(variant, compareTo);

  const onConfirm = () => {
    if (!compareTo) {
      return;
    }

    if (compareKind === "same") {
      return;
    }

    setSelectedReleaseId(compareTo);
    install(compareTo);
  };

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    const latestRelease = releases.reduce(
      (prev: undefined | GameRelease, curr) => {
        if (!prev) {
          return curr;
        }

        if (curr.created_at > prev.created_at) {
          return curr;
        }

        return prev;
      },
      undefined,
    );

    return releases.map((r) => {
      const isActive = r.version === normalizedCurrentActive;
      const isLatest = r.version === latestRelease?.version;

      return {
        value: r.version,
        label: (
          <ReleaseLabel
            variant={variant}
            version={r.version}
            isActive={isActive}
            isLatest={isLatest}
          />
        ),
      };
    });
  }, [releases, normalizedCurrentActive, variant]);

  const isSelectorsDisabled = comboboxItems.length === 0;

  return (
    <ConfirmationDialog
      open={open}
      onOpenChange={onOpenChange}
      onConfirm={onConfirm}
      title="What's new?"
      description="Review release notes before changing versions."
      confirmText={confirmText}
      cancelText="Cancel"
      confirmDisabled={confirmDisabled}
      confirmVariant={confirmVariant}
    >
      <div className="flex flex-col gap-3">
        <div className="flex items-end gap-2">
          <VirtualizedCombobox
            label="Active Release"
            items={comboboxItems}
            value={compareFrom}
            onChange={setCompareFrom}
            disabled={isSelectorsDisabled}
            placeholder="Select a release"
            className="grow"
          />
          <div className="pb-2 text-muted-foreground">→</div>
          <VirtualizedCombobox
            label="Selected Release"
            items={comboboxItems}
            value={compareTo}
            onChange={setCompareTo}
            disabled={isSelectorsDisabled}
            placeholder="Select a release"
            className="grow"
          />
        </div>

        <div
          className={cn(
            "rounded-md border overflow-hidden",
            compareKind === "downgrade" &&
              "bg-destructive/10 border-destructive/30",
          )}
        >
          <div className="max-h-[60vh] overflow-auto">
            {versionsToShow.length === 0 ? (
              <div className="p-4 text-sm text-muted-foreground">
                {"No releases selected."}
              </div>
            ) : (
              <div className="divide-y">
                {versionsToShow.map((version) => {
                  const notesState = notesByVersion[version];
                  const status = notesState?.status ?? "loading";

                  const content =
                    status === "error"
                      ? RELEASE_NOTES_LOAD_ERROR_TEXT
                      : (notesState?.notes ?? "") === ""
                        ? NO_RELEASE_NOTES_TEXT
                        : (notesState?.notes ?? "");

                  return (
                    <div key={version} className="bg-background/40">
                      <div className="px-3 py-2 text-xs font-mono bg-muted/40">
                        {version}
                      </div>
                      <pre className="px-3 py-2 text-sm font-mono whitespace-pre-wrap">
                        {status === "loading"
                          ? "Loading..."
                          : content}
                      </pre>
                    </div>
                  );
                })}
              </div>
            )}
          </div>
        </div>
      </div>
    </ConfirmationDialog>
  );
}

interface ReleaseNotesDialogProps {
  open: boolean;
  onOpenChange: (_open: boolean) => void;
  variant: GameVariant;
  releases: GameRelease[];
  currentActiveRelease: string | undefined;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
}

function getVersionsToShow(
  releases: GameRelease[],
  fromVersion: string | undefined,
  toVersion: string | undefined,
): { compareKind: CompareKind; versionsToShow: string[] } {
  if (!toVersion) {
    return { compareKind: "upgrade", versionsToShow: [] };
  }

  if (!fromVersion) {
    return { compareKind: "upgrade", versionsToShow: [toVersion] };
  }

  if (fromVersion === toVersion) {
    return { compareKind: "same", versionsToShow: [toVersion] };
  }

  const fromIndex = releases.findIndex(
    (r) => r.version === fromVersion,
  );
  const toIndex = releases.findIndex((r) => r.version === toVersion);

  if (fromIndex === -1 || toIndex === -1) {
    return { compareKind: "upgrade", versionsToShow: [toVersion] };
  }

  if (fromIndex === toIndex) {
    return { compareKind: "same", versionsToShow: [toVersion] };
  }

  // releases are sorted newest -> oldest
  if (fromIndex > toIndex) {
    const versions = [] as string[];
    for (let i = fromIndex - 1; i >= toIndex; i -= 1) {
      versions.push(releases[i]!.version);
    }
    return { compareKind: "upgrade", versionsToShow: versions };
  }

  const versions = [] as string[];
  for (let i = fromIndex; i < toIndex; i += 1) {
    versions.push(releases[i]!.version);
  }
  return { compareKind: "downgrade", versionsToShow: versions };
}
