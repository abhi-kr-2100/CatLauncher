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

/**
 * A dialog component that notifies the user when an automatic update has failed.
 * It provides a link to manually update the application.
 *
 * @returns A React element representing the auto-update notifier dialog, or null if no failure state is present.
 *
 * @public
 */
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
