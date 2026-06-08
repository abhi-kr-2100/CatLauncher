import { useState, useCallback, useRef, useEffect } from "react";
import { createDebugReport } from "@/lib/commands";
import { openLink } from "@/lib/utils";

export type ReportStep = "idle" | "creating" | "created";

/**
 * Custom hook to handle the diagnostic report creation and dialog state.
 *
 * @param issueUrl - The URL to open for reporting an issue.
 * @returns An object containing the dialog state and handlers.
 */
export function useDebugReport(issueUrl: string) {
  const [isDialogOpen, setIsDialogOpen] = useState(false);
  const [reportStep, setReportStep] = useState<ReportStep>("idle");
  const [zipPath, setZipPath] = useState<string | null>(null);

  // Track the ID of the current active request to ignore outdated promises
  const latestRequestIdRef = useRef(0);
  // Track if a report is currently being created to prevent concurrent starts
  const [isCreating, setIsCreating] = useState(false);

  // Track if dialog is open to avoid updating state for a closed dialog
  const isDialogOpenRef = useRef(isDialogOpen);
  useEffect(() => {
    isDialogOpenRef.current = isDialogOpen;
  }, [isDialogOpen]);

  const handleReportIssueClick = useCallback(async () => {
    // Prevent starting a new report if one is already in progress
    if (isCreating) return;

    const requestId = ++latestRequestIdRef.current;

    setIsDialogOpen(true);
    setReportStep("creating");
    setZipPath(null);
    setIsCreating(true);

    try {
      const path = await createDebugReport();

      // Only update state if this is the latest request AND the dialog is still open
      if (
        requestId === latestRequestIdRef.current &&
        isDialogOpenRef.current
      ) {
        setZipPath(path);
        setReportStep("created");
      }
    } catch (error) {
      console.error("Failed to create debug report:", error);

      // Only update state if this is the latest request AND the dialog is still open
      if (
        requestId === latestRequestIdRef.current &&
        isDialogOpenRef.current
      ) {
        setIsDialogOpen(false);
        // Fallback to just opening the link if report creation fails
        openLink(issueUrl);
      }
    } finally {
      // Only reset isCreating if this is the latest request
      if (requestId === latestRequestIdRef.current) {
        setIsCreating(false);
      }
    }
  }, [issueUrl, isCreating]);

  const onConfirm = useCallback(() => {
    if (reportStep === "created") {
      openLink(issueUrl);
    }
  }, [reportStep, issueUrl]);

  return {
    isDialogOpen,
    setIsDialogOpen,
    reportStep,
    zipPath,
    isCreating,
    handleReportIssueClick,
    onConfirm,
  };
}
