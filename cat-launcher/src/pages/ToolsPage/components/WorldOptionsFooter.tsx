import { Button } from "@/components/ui/button";

interface WorldOptionsFooterProps {
  isDirty: boolean;
  isUpdating: boolean;
  apply: () => void;
  cancel: () => void;
}

export function WorldOptionsFooter({
  isDirty,
  isUpdating,
  apply,
  cancel,
}: WorldOptionsFooterProps) {
  return (
    <div className="fixed bottom-0 left-0 right-0 bg-background border-t border-border overflow-x-auto z-10">
      <div className="container mx-auto max-w-2xl px-4 py-4">
        <div className="flex justify-end gap-4">
          <Button
            type="button"
            variant="ghost"
            onClick={cancel}
            disabled={!isDirty || isUpdating}
          >
            Cancel
          </Button>
          <Button
            type="button"
            onClick={apply}
            disabled={!isDirty || isUpdating}
          >
            {isUpdating ? "Applying..." : "Apply"}
          </Button>
        </div>
      </div>
    </div>
  );
}
