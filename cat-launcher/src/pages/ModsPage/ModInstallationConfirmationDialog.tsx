import { ConfirmationDialog } from "@/components/ui/ConfirmationDialog";
import { Alert, AlertDescription } from "@/components/ui/alert";
import { GameVariant } from "@/generated-types/GameVariant";
import { useGetLastModActivity } from "./hooks";
import { toastCL } from "@/lib/utils";
import {
  getStabilityRating,
  getStabilityLevelLabel,
  type StabilityRating,
} from "./lib/stabilityRating";
import { getRelativeTimeDisplay } from "./lib/timeFormatting";
import { modsPageErrorMap } from "./lib/errors";

interface ModInstallationConfirmationDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onConfirm: () => void;
  modId: string;
  variant: GameVariant;
}

const getStabilityColors = (level: StabilityRating["level"]) => {
  switch (level) {
    case "high":
      return {
        color: "text-green-600 dark:text-green-400",
        bgColor:
          "bg-green-50 dark:bg-green-950 border-green-200 dark:border-green-800",
        barColor: "bg-green-500",
      };
    case "medium":
      return {
        color: "text-yellow-600 dark:text-yellow-400",
        bgColor:
          "bg-yellow-50 dark:bg-yellow-950 border-yellow-200 dark:border-yellow-800",
        barColor: "bg-yellow-500",
      };
    case "low":
      return {
        color: "text-red-600 dark:text-red-400",
        bgColor:
          "bg-red-50 dark:bg-red-950 border-red-200 dark:border-red-800",
        barColor: "bg-red-500",
      };
  }
};

export function ModInstallationConfirmationDialog({
  open,
  onOpenChange,
  onConfirm,
  modId,
  variant,
}: ModInstallationConfirmationDialogProps) {
  const { lastActivity, isLoading } = useGetLastModActivity(
    open,
    modId,
    variant,
    (error) =>
      toastCL(
        "error",
        "Failed to fetch mod stability rating.",
        error,
        modsPageErrorMap,
      ),
  );

  const stability = lastActivity
    ? getStabilityRating(lastActivity.timestamp)
    : null;

  const colors = stability
    ? getStabilityColors(stability.level)
    : null;

  return (
    <ConfirmationDialog
      open={open}
      onOpenChange={onOpenChange}
      onConfirm={onConfirm}
      title="Install Third-Party Mod"
      description="Third-party mods can break your game!"
      confirmText="Accept Risk & Install"
      cancelText="Cancel"
    >
      {lastActivity && stability && colors && (
        <Alert className={`border ${colors.bgColor}`}>
          <AlertDescription>
            <div className="space-y-3">
              <div>
                <p className="text-xs font-semibold text-gray-600 dark:text-gray-400 uppercase tracking-wide">
                  Mod Stability
                </p>
                <div className="flex items-center justify-between mt-2">
                  <div
                    className={`text-lg font-bold ${colors.color}`}
                  >
                    {getStabilityLevelLabel(stability.level)}
                  </div>
                  <div className="w-24 h-2 bg-gray-200 dark:bg-gray-700 rounded-full overflow-hidden">
                    <div
                      className={`h-full rounded-full transition-all ${colors.barColor}`}
                      style={{ width: `${stability.score}%` }}
                    />
                  </div>
                </div>
              </div>
              <div className="text-sm text-gray-700 dark:text-gray-300">
                <p className="font-medium">
                  Last updated{" "}
                  {getRelativeTimeDisplay(lastActivity.timestamp)}
                </p>
              </div>
            </div>
          </AlertDescription>
        </Alert>
      )}
      {isLoading && (
        <Alert className="bg-slate-100 dark:bg-slate-900 border-slate-200 dark:border-slate-700">
          <AlertDescription className="text-sm text-slate-600 dark:text-slate-400">
            Loading stability information...
          </AlertDescription>
        </Alert>
      )}
    </ConfirmationDialog>
  );
}
