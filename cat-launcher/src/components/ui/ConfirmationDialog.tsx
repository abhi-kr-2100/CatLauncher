import { ReactNode } from "react";

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

/**
 * Props for the {@link ConfirmationDialog} component.
 * @public
 */
interface ConfirmationDialogProps {
  /** Whether the dialog is currently open. */
  open: boolean;
  /** Callback function called when the dialog's open state changes. */
  onOpenChange: (_open: boolean) => void;
  /** Callback function executed when the user confirms the action. */
  onConfirm: () => void;
  /** The title of the confirmation dialog. */
  title: string;
  /** A description providing more context about the action being confirmed. */
  description: string;
  /** The text to display on the confirmation button. Defaults to "Confirm". */
  confirmText?: string;
  /** The text to display on the cancel button. Defaults to "Cancel". */
  cancelText?: string;
  /** Optional additional content to display within the dialog. */
  children?: ReactNode;
}

/**
 * A reusable dialog component for confirming user actions.
 *
 * @param props - The component props.
 * @returns A React element representing the confirmation dialog.
 * @public
 */
export function ConfirmationDialog({
  open,
  onOpenChange,
  onConfirm,
  title,
  description,
  confirmText = "Confirm",
  cancelText = "Cancel",
  children,
}: ConfirmationDialogProps) {
  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>{title}</DialogTitle>
          <DialogDescription>{description}</DialogDescription>
        </DialogHeader>
        {children}
        <DialogFooter>
          <DialogClose asChild>
            <Button variant="outline">{cancelText}</Button>
          </DialogClose>
          <DialogClose asChild>
            <Button onClick={onConfirm}>{confirmText}</Button>
          </DialogClose>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
