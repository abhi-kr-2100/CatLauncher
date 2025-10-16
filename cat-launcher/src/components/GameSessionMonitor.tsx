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

  return (
    <Dialog
      open={gameStatus === GameStatus.CRASHED}
      onOpenChange={resetGameSessionMonitor}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Game exited unexpectedly</DialogTitle>
          <DialogDescription>
            The game may have crashed or exited with an error.
          </DialogDescription>
        </DialogHeader>

        <div className="flex flex-col gap-2">
          <h3 className="font-semibold">Exit Status</h3>
          <pre className="text-sm bg-muted p-4 rounded-md">
            {exitCode ?? "Unknown"}
          </pre>
        </div>

        {logsText && (
          <div className="flex flex-col gap-2">
            <h3 className="font-semibold">Logs</h3>
            <pre className="text-sm bg-muted p-4 rounded-md whitespace-pre-wrap max-h-[200px] overflow-auto">
              {logsText}
            </pre>
          </div>
        )}

        <DialogFooter>
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
