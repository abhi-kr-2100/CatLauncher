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
import { UPDATE_LINK } from "@/lib/constants";
import {
  AutoUpdateStatus,
  useAutoUpdateEvents,
} from "@/providers/hooks";
import { ExternalLink } from "./ui/ExternalLink";

const AutoUpdateNotifier = () => {
  const { autoUpdateStatus, resetAutoUpdateStatus } =
    useAutoUpdateEvents();

  return (
    <Dialog
      open={autoUpdateStatus === AutoUpdateStatus.FAILURE}
      onOpenChange={resetAutoUpdateStatus}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>Autoupdate Failed</DialogTitle>
          <DialogDescription>
            Please manually update the app by visiting
            <ExternalLink href={UPDATE_LINK}>
              {UPDATE_LINK}
            </ExternalLink>
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button>Close</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default AutoUpdateNotifier;
