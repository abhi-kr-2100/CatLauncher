import { useQuery } from "@tanstack/react-query";
import { useCallback, useEffect, useMemo, useState } from "react";

import {
  VirtualizedCombobox,
  type ComboboxItem,
} from "@/components/virtualized-combobox";
import type { GameRelease } from "@/generated-types/GameRelease";
import type { GameVariant } from "@/generated-types/GameVariant";
import { getActiveRelease } from "@/lib/commands";
import { queryKeys } from "@/lib/queryKeys";
import { toastCL } from "@/lib/utils";
import { useAppSelector } from "@/store/hooks";
import { useReleases } from "./hooks";
import ReleaseFilter, { FilterFn } from "./ReleaseFilter";
import ReleaseLabel from "./ReleaseLabel";
import ReleaseNotesButton from "./ReleaseNotesButton";
import { Command } from '@tauri-apps/plugin-shell';
import { join, appLocalDataDir } from '@tauri-apps/api/path';
import { platform } from '@tauri-apps/plugin-os';

export default function ReleaseSelector({
  variant,
  selectedReleaseId,
  setSelectedReleaseId,
}: ReleaseSelectorProps) {
  const { releases, isLoading: isReleasesLoading } = useReleases(
    variant,
    (error) =>
      toastCL(
        "error",
        `Failed to load releases for ${variant}.`,
        error,
      ),
    (error) =>
      toastCL(
        "error",
        `Failed to fetch releases for ${variant}.`,
        error,
      ),
  );

  const selectedRelease = useMemo(() => {
    return releases.find((r) => r.version === selectedReleaseId);
  }, [releases, selectedReleaseId]);

  const {
    data: activeRelease,
    isLoading: isActiveReleaseLoading,
    error: activeReleaseError,
  } = useQuery<string | undefined>({
    queryKey: queryKeys.activeRelease(variant),
    queryFn: () => getActiveRelease(variant),
  });

  useEffect(() => {
    if (!activeReleaseError) {
      return;
    }

    toastCL(
      "warning",
      `Failed to get active release of ${variant}.`,
      activeReleaseError,
    );
  }, [activeReleaseError, variant]);

  const [appliedFilter, setAppliedFilter] = useState<FilterFn>(
    () => (_r: GameRelease) => true,
  );

  const latestRelease = useMemo(() => {
    return releases?.[0];
  }, [releases]);

  const comboboxItems = useMemo<ComboboxItem[]>(() => {
    return (
      releases.filter(appliedFilter).map((r) => {
        const isActive = r.version === activeRelease;

        return {
          value: r.version,
          label: (
            <ReleaseLabel
              variant={variant}
              version={r.version}
              isActive={isActive}
            />
          ),
        };
      }) ?? []
    );
  }, [releases, activeRelease, variant, appliedFilter]);

  useEffect(() => {
    // Selected release may become unavailable after filtering
    if (
      selectedReleaseId &&
      !comboboxItems.find((item) => item.value === selectedReleaseId)
    ) {
      setSelectedReleaseId(undefined);
    }
  }, [comboboxItems, selectedReleaseId, setSelectedReleaseId]);

  const installationStatusByVersion = useAppSelector(
    (state) =>
      state.installationProgress.installationStatusByVariant.release[
      variant
      ],
  );

  const autoselect = useCallback(
    (items: ComboboxItem[]) => {
      const installingOrDownloadingRelease = items.find((item) => {
        const status = installationStatusByVersion?.[item.value];
        return status === "Downloading" || status === "Installing";
      });

      if (installingOrDownloadingRelease) {
        return installingOrDownloadingRelease;
      }

      if (isActiveReleaseLoading) {
        return;
      }

      if (activeReleaseError || activeRelease === "") {
        return (
          items.find((i) => i.value === latestRelease?.version) ??
          items?.[0]
        );
      }

      return (
        items.find((i) => i.value === activeRelease) ?? items?.[0]
      );
    },
    [
      activeRelease,
      isActiveReleaseLoading,
      activeReleaseError,
      installationStatusByVersion,
      latestRelease,
    ],
  );

  const isInstalling = Object.values(
    installationStatusByVersion ?? {},
  ).some(
    (status) => status === "Downloading" || status === "Installing",
  );

  const placeholderText = isReleasesLoading
    ? "Loading..."
    : comboboxItems.length === 0
      ? "No releases available."
      : "Select a release";

  return (
    <div className="flex flex-col gap-2">
      <ReleaseFilter
        variant={variant}
        onChange={(filter) =>
          setAppliedFilter((_prev: FilterFn) => filter)
        }
      />
      <div className="flex items-end gap-2">
        <div className="grow">
          <VirtualizedCombobox
            label="Version"
            items={comboboxItems}
            value={selectedReleaseId}
            onChange={setSelectedReleaseId}
            autoselect={autoselect}
            placeholder={placeholderText}
            disabled={isReleasesLoading || isInstalling}
          />
        </div>

        {selectedRelease && (
          <button
            className="inline-flex h-10 w-10 items-center justify-center rounded-md border border-input bg-background text-sm font-medium hover:bg-accent hover:text-accent-foreground"
            title="Open Installation Folder"
            onClick={async () => {
              if (!selectedReleaseId) return;

              try {
                const baseDir = await appLocalDataDir();
                const safeVersion = selectedReleaseId.replace(/[.-]/g, '_');
                const gamePath = await join(baseDir, 'Assets', variant, safeVersion);

                const currentPlatform = platform();
                let commandName = '';

                if (currentPlatform === 'windows') {
                  commandName = 'open-win';
                } else if (currentPlatform === 'macos') {
                  commandName = 'open-mac';
                } else if (currentPlatform === 'linux') {
                  commandName = 'open-linux';
                } else {
                  console.warn("Unknown platform:", currentPlatform);
                  return;
                }

                console.log(`Opening path on ${currentPlatform}:`, gamePath);

                const command = Command.create(commandName, [gamePath]);
                await command.execute();

              } catch (e) {
                console.error("Failed to open folder:", e);
              }
            }}
          >
            📂
          </button>
        )}

        {selectedRelease && (
          <ReleaseNotesButton release={selectedRelease} />
        )}
      </div>
    </div>
  );
}

interface ReleaseSelectorProps {
  variant: GameVariant;
  selectedReleaseId: string | undefined;
  setSelectedReleaseId: (value: string | undefined) => void;
}
