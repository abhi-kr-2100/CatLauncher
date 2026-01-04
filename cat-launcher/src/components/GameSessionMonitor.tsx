import { Copy } from "lucide-react";
import { useTranslation } from "react-i18next";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogClose,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import { copyToClipboard, toastCL } from "@/lib/utils";
import { GameStatus, useGameSessionEvents } from "@/providers/hooks";

const GameSessionMonitor = () => {
  const { t } = useTranslation();
  const { gameStatus, logsText, exitCode, resetGameSessionMonitor } =
    useGameSessionEvents();

  const title =
    gameStatus === GameStatus.CRASHED
      ? t("gameCrashed")
      : gameStatus === GameStatus.TERMINATED
        ? t("gameTerminated")
        : gameStatus === GameStatus.ERROR
          ? t("gameExitedUnexpectedly")
          : null;

  return (
    <Dialog
      open={[
        GameStatus.CRASHED,
        GameStatus.ERROR,
        GameStatus.TERMINATED,
      ].includes(gameStatus)}
      onOpenChange={resetGameSessionMonitor}
    >
      <DialogContent className="flex flex-col gap-4 max-h-[90vh]">
        <DialogHeader className="flex flex-col gap-2">
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{t("diagnoseIssue")}</DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-2">
          <h3 className="font-semibold">{t("exitCode")}</h3>
          <pre className="text-sm bg-muted p-4 rounded-md">
            {exitCode ?? t("unknown")}
          </pre>
        </div>

        {logsText && (
          <div className="flex flex-col gap-2">
            <h3 className="font-semibold">{t("logs")}</h3>
            <pre className="text-sm bg-muted p-4 rounded-md whitespace-pre-wrap h-[30vh] overflow-auto">
              {logsText}
            </pre>
          </div>
        )}

        <DialogFooter className="mt-auto">
          {logsText && (
            <Button
              onClick={() => {
                copyToClipboard(logsText)
                  .then(() => {
                    toastCL("success", t("logsCopied"));
                  })
                  .catch((error) => {
                    toastCL("error", t("errorCopyingLogs"), error);
                  });
              }}
              variant={"ghost"}
            >
              <Copy />
              {t("copyLogs")}
            </Button>
          )}
          <DialogClose asChild>
            <Button>{t("close")}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default GameSessionMonitor;
