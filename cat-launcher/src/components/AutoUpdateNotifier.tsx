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
import { UPDATE_LINK } from "@/lib/constants";
import {
  AutoUpdateStatus,
  useAutoUpdateEvents,
} from "@/providers/hooks";
import { ExternalLink } from "./ui/ExternalLink";

const AutoUpdateNotifier = () => {
  const { t } = useTranslation();
  const { autoUpdateStatus, resetAutoUpdateStatus } =
    useAutoUpdateEvents();

  return (
    <Dialog
      open={autoUpdateStatus === AutoUpdateStatus.FAILURE}
      onOpenChange={resetAutoUpdateStatus}
    >
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{t("autoUpdateFailed")}</DialogTitle>
          <DialogDescription>
            {t("description")}
            <ExternalLink href={UPDATE_LINK}>
              {UPDATE_LINK}
            </ExternalLink>
          </DialogDescription>
        </DialogHeader>
        <DialogFooter>
          <DialogClose asChild>
            <Button>{t("close")}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
};

export default AutoUpdateNotifier;
