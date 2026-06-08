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

  // Track if dialog is open to avoid race conditions
  const isDialogOpenRef = useRef(isDialogOpen);
  useEffect(() => {
    isDialogOpenRef.current = isDialogOpen;
  }, [isDialogOpen]);

  const handleReportIssueClick = useCallback(async () => {
    setIsDialogOpen(true);
    setReportStep("creating");
    setZipPath(null);

    try {
      const path = await createDebugReport();
      // Only update state if the dialog is still open
      if (isDialogOpenRef.current) {
        setZipPath(path);
        setReportStep("created");
      }
    } catch (error) {
      console.error("Failed to create debug report:", error);
      if (isDialogOpenRef.current) {
        setIsDialogOpen(false);
        // Fallback to just opening the link if report creation fails
        openLink(issueUrl);
      }
    }
  }, [issueUrl]);

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
    handleReportIssueClick,
    onConfirm,
  };
}
