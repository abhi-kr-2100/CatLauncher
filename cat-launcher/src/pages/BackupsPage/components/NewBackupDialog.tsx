import { zodResolver } from "@hookform/resolvers/zod";
import { useEffect } from "react";
import { useForm } from "react-hook-form";
import * as z from "zod";

import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from "@/components/ui/dialog";
import {
  Field,
  FieldContent,
  FieldError,
  FieldLabel,
} from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { Textarea } from "@/components/ui/textarea";
import { GameVariant } from "@/generated-types/GameVariant";

const formSchema = z.object({
  name: z.string().min(1, "Name is required"),
  notes: z.string().optional(),
});

interface NewBackupDialogProps {
  open: boolean;
  onOpenChange: (open: boolean) => void;
  onSave: (values: z.infer<typeof formSchema>) => void;
  variant: GameVariant;
  isCreating: boolean;
}

export function NewBackupDialog({
  open,
  onOpenChange,
  onSave,
  variant,
  isCreating,
}: NewBackupDialogProps) {
  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
  });

  useEffect(() => {
    if (open) {
      form.reset({
        name: `${variant}_${Date.now()}`,
        notes: "",
      });
    }
  }, [open, form, variant]);

  return (
    <Dialog open={open} onOpenChange={onOpenChange}>
      <DialogContent>
        <DialogHeader>
          <DialogTitle>New Backup</DialogTitle>
          <DialogDescription>
            Create a new manual backup.
          </DialogDescription>
        </DialogHeader>
        <form
          onSubmit={form.handleSubmit(onSave)}
          className="space-y-4"
        >
          <Field
            data-invalid={!!form.formState.errors.name}
            aria-invalid={!!form.formState.errors.name}
          >
            <FieldLabel htmlFor="name">Name</FieldLabel>
            <FieldContent>
              <Input id="name" {...form.register("name")} />
              <FieldError
                errors={
                  form.formState.errors.name
                    ? [form.formState.errors.name]
                    : undefined
                }
              />
            </FieldContent>
          </Field>
          <Field
            data-invalid={!!form.formState.errors.notes}
            aria-invalid={!!form.formState.errors.notes}
          >
            <FieldLabel htmlFor="notes">Notes</FieldLabel>
            <FieldContent>
              <Textarea id="notes" {...form.register("notes")} />
              <FieldError
                errors={
                  form.formState.errors.notes
                    ? [form.formState.errors.notes]
                    : undefined
                }
              />
            </FieldContent>
          </Field>
          <DialogFooter>
            <Button type="submit" disabled={isCreating}>
              Save
            </Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}
