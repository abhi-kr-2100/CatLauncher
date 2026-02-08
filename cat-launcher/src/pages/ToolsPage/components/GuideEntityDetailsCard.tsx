import { Loader2, X } from "lucide-react";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import type { GameVariant } from "@/generated-types/GameVariant";
import useGuideEntity from "../hooks/useGuideEntity";
import { cn } from "@/lib/utils";

interface GuideEntityDetailsCardProps {
  variant: GameVariant;
  entityId: string | null;
  isActive: boolean;
  onActivate: () => void;
  onClose?: () => void;
  showClose: boolean;
}

export default function GuideEntityDetailsCard({
  variant,
  entityId,
  isActive,
  onActivate,
  onClose,
  showClose,
}: GuideEntityDetailsCardProps) {
  const { data: entityDetail, isLoading: isLoadingEntity } =
    useGuideEntity(variant, entityId);

  return (
    <Card
      className={cn(
        "flex flex-col min-h-0 transition-colors border-2",
        isActive ? "border-primary" : "border-transparent",
      )}
      onClick={onActivate}
    >
      <CardHeader className="flex flex-row items-center justify-between space-y-0 pb-2">
        <CardTitle className="text-sm font-medium italic">
          {isActive ? "Active Slot" : "Detail Slot"}
        </CardTitle>
        {showClose && (
          <Button
            variant="ghost"
            size="icon"
            className="h-8 w-8"
            onClick={(e) => {
              e.stopPropagation();
              onClose?.();
            }}
          >
            <X className="h-4 w-4" />
          </Button>
        )}
      </CardHeader>
      <CardContent className="flex-1 p-0 min-h-0">
        <ScrollArea className="h-full">
          <div className="p-4">
            {isLoadingEntity ? (
              <div className="flex items-center justify-center h-32">
                <Loader2 className="h-8 w-8 animate-spin text-muted-foreground" />
              </div>
            ) : entityDetail ? (
              <pre className="text-[10px] md:text-xs font-mono p-4 bg-muted rounded overflow-auto whitespace-pre-wrap">
                {JSON.stringify(entityDetail.raw_json, null, 2)}
              </pre>
            ) : (
              <div className="h-32 flex items-center justify-center border border-dashed rounded-lg text-muted-foreground text-sm text-center px-4">
                {entityId
                  ? "Entity not found"
                  : "Select an item to view its details"}
              </div>
            )}
          </div>
        </ScrollArea>
      </CardContent>
    </Card>
  );
}
