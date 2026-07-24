import pkg from "../../package.json";
import { openLink } from "@/lib/utils";
import { Button } from "@/components/ui/button";
import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import { useDebugReport } from "./AboutPage/hooks/useDebugReport";

/**
 * External links for the CatLauncher project.
 * Includes GitHub repository, issue reporting, and feature requests.
 */
const LINKS = [
  {
    id: "star",
    label: "⭐ Star CatLauncher on GitHub",
    url: "https://github.com/abhi-kr-2100/CatLauncher",
    variant: "outline" as const,
  },
  {
    id: "report-issue",
    label: "🐛 Report an issue",
    url: "https://github.com/abhi-kr-2100/CatLauncher/issues/new",
    variant: "outline" as const,
  },
  {
    id: "request-feature",
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
  const reportIssueUrl =
    LINKS.find((l) => l.id === "report-issue")?.url || "";

  const {
    isDialogOpen,
    setIsDialogOpen,
    reportStep,
    zipPath,
    isCreating,
    handleReportIssueClick,
    onConfirm,
  } = useDebugReport(reportIssueUrl);

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
            key={link.id}
            variant={link.variant}
            disabled={link.id === "report-issue" && isCreating}
            onClick={() => {
              if (link.id === "report-issue") {
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
        open={isDialogOpen}
        onOpenChange={setIsDialogOpen}
        title="Report an Issue"
        description={
          reportStep === "creating"
            ? "Creating a .zip archive of CatLauncher's current state to help us debug the issue. Please wait..."
            : `A debug report has been created at: ${zipPath}. Please attach this file when reporting the issue on GitHub.`
        }
        confirmText={
          reportStep === "creating"
            ? "Creating..."
            : "Proceed to GitHub"
        }
        confirmDisabled={reportStep === "creating"}
        cancelText="Close"
        onConfirm={onConfirm}
        closeOnConfirm={false}
      />
    </div>
  );
}
