import { useState } from "react";
import pkg from "../../package.json";
import { openLink } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { createDebugReport } from "@/lib/commands";
import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";

/**
 * External links for the CatLauncher project.
 * Includes GitHub repository, issue reporting, and feature requests.
 */
const LINKS = [
  {
    label: "⭐ Star CatLauncher on GitHub",
    url: "https://github.com/abhi-kr-2100/CatLauncher",
    variant: "outline" as const,
  },
  {
    label: "🐛 Report an issue",
    url: "https://github.com/abhi-kr-2100/CatLauncher/issues/new",
    variant: "outline" as const,
  },
  {
    label: "🚀 Request a new feature",
    url: "https://github.com/abhi-kr-2100/CatLauncher/issues/new",
    variant: "outline" as const,
  },
];

/**
 * AboutPage component that displays information about CatLauncher.
 * It shows the application name, description, version, and helpful links.
 *
 * @returns A React component that renders the About page.
 */
export default function AboutPage() {
  const [isReportDialogOpen, setIsReportDialogOpen] = useState(false);
  const [reportStep, setReportStep] = useState<
    "idle" | "creating" | "created"
  >("idle");
  const [zipPath, setZipPath] = useState<string | null>(null);

  const handleReportIssueClick = async () => {
    setIsReportDialogOpen(true);
    setReportStep("creating");
    try {
      const path = await createDebugReport();
      setZipPath(path);
      setReportStep("created");
    } catch (error) {
      console.error("Failed to create debug report:", error);
      setIsReportDialogOpen(false);
      // Fallback to just opening the link if report creation fails
      openLink(LINKS[1].url);
    }
  };

  return (
    <div className="flex flex-col items-center gap-4 py-4 max-w-lg mx-auto">
      <div className="flex flex-col items-center gap-2">
        <h1 className="text-2xl font-bold">CatLauncher</h1>
        <p className="text-center">
          An opinionated cross-platform launcher for Cataclysm games
          with modern social features.
        </p>
        <p className="text-muted-foreground text-sm">
          v{pkg.version}
        </p>
      </div>

      <div className="flex flex-col gap-2">
        {LINKS.map((link) => (
          <Button
            key={link.label}
            variant={link.variant}
            onClick={() => {
              if (link.label === "🐛 Report an issue") {
                handleReportIssueClick();
              } else {
                openLink(link.url);
              }
            }}
          >
            {link.label}
          </Button>
        ))}
      </div>

      <ConfirmationDialog
        open={isReportDialogOpen}
        onOpenChange={setIsReportDialogOpen}
        title="Report an Issue"
        description={
          reportStep === "creating"
            ? "Creating a .zip archive of CatLauncher's current state to help us debug the issue. Please wait..."
            : `A debug report has been created at: ${zipPath}. Please attach this file when reporting the issue on GitHub.`
        }
        confirmText={
          reportStep === "creating" ? "Creating..." : "Proceed to GitHub"
        }
        cancelText="Close"
        onConfirm={() => {
          if (reportStep === "created") {
            openLink(LINKS[1].url);
          }
        }}
        closeOnConfirm={false}
      />
    </div>
  );
}
