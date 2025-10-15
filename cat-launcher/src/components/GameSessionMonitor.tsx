import { useEffect, useState } from "react";

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
import { listenToGameEvent } from "@/lib/commands";
import { copyToClipboard, toastCL } from "@/lib/utils";
import { Copy } from "lucide-react";

const GameSessionMonitor = () => {
  const [isCrashDialogOpen, setIsCrashDialogOpen] = useState(false);
  const [logs, setLogs] = useState<string[]>([]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;

    listenToGameEvent((event) => {
      switch (event.type) {
        case "Log":
          setLogs((prev) => [...prev, event.payload]);
          break;
        case "Exit":
          // code is null if the process was terminated by a signal
          if (event.payload.code !== 0) {
            setIsCrashDialogOpen(true);
          } else {
            // Game exited successfully, clear logs
            setLogs([]);
          }
          break;
        case "Error":
          toastCL("error", "Game error", event.payload.message);
          setLogs((prev) => [...prev, `ERROR: ${event.payload.message}`]);
          setIsCrashDialogOpen(true);
          break;
      }
    })
      .then((fn) => {
        unlisten = fn;
      })
      .catch((error) => {
        toastCL("error", "Error listening to game events", error);
      });

    return () => {
      unlisten?.();
    };
  }, []);

  const onOpenChange = (open: boolean) => {
    setIsCrashDialogOpen(open);
    if (!open) {
      setLogs([]);
    }
  };

  const logsText = logs.join("\n");

  return (
    <Dialog open={isCrashDialogOpen} onOpenChange={onOpenChange}>
      <DialogContent className="max-w-4xl">
        <DialogHeader>
          <DialogTitle>Game exited unexpectedly</DialogTitle>
          <DialogDescription>
            The game may have crashed or exited with an error. Here are the
            logs:
          </DialogDescription>
        </DialogHeader>
        <div className="max-h-96 overflow-y-auto bg-muted p-4 rounded-md">
          <pre className="text-sm">{logsText}</pre>
        </div>
        <DialogFooter>
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
          <DialogClose asChild>
            <Button>Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default GameSessionMonitor;
