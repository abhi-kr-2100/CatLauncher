import { Copy } from "lucide-react";

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
  const { gameStatus, logsText, exitCode, resetGameSessionMonitor } =
    useGameSessionEvents();

  const title =
    gameStatus === GameStatus.CRASHED
      ? "Game crashed"
      : gameStatus === GameStatus.TERMINATED
        ? "Game was terminated by an external source"
        : gameStatus === GameStatus.ERROR
          ? "Game exited unexpectedly or failed to start"
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
          <DialogDescription>
            The following information may help you diagnose the issue.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-2">
          <h3 className="font-semibold">Exit Code</h3>
          <pre className="text-sm bg-muted p-4 rounded-md">
            {exitCode ?? "Unknown"}
          </pre>
        </div>

        {logsText && (
          <div className="flex flex-col gap-2">
            <h3 className="font-semibold">Logs</h3>
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
                    toastCL("success", "Logs copied to clipboard");
                  })
                  .catch((error) => {
                    toastCL("error", "Error copying logs", error);
                  });
              }}
              variant={"ghost"}
            >
              <Copy />
              Copy Logs
            </Button>
          )}
          <DialogClose asChild>
            <Button>Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default GameSessionMonitor;
