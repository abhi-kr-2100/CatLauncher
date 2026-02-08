import { useState } from "react";
import { Loader2, Plus } from "lucide-react";

import { GameVariant } from "@/generated-types/GameVariant";
import VariantSelector from "@/components/VariantSelector";
import { useGameVariants } from "@/hooks/useGameVariants";
import { SearchInput } from "@/components/SearchInput";
import { useDebounce } from "@/hooks/useDebounce";
import useGuideSearch from "../hooks/useGuideSearch";
import {
  Card,
  CardContent,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Button } from "@/components/ui/button";
import {
  ResizableHandle,
  ResizablePanel,
  ResizablePanelGroup,
} from "@/components/ui/resizable";
import GuideEntityDetailsCard from "./GuideEntityDetailsCard";

interface TiledLayoutProps {
  indices: number[];
  detailSlots: (string | null)[];
  activeSlotIndex: number;
  onActivate: (index: number) => void;
  onClose: (index: number) => void;
  variant: GameVariant;
  depth?: number;
}

function TiledLayout({
  indices,
  detailSlots,
  activeSlotIndex,
  onActivate,
  onClose,
  variant,
  depth = 0,
}: TiledLayoutProps) {
  if (indices.length === 1) {
    const index = indices[0];
    return (
      <GuideEntityDetailsCard
        variant={variant}
        entityId={detailSlots[index]}
        isActive={activeSlotIndex === index}
        onActivate={() => onActivate(index)}
        onClose={() => onClose(index)}
        showClose={detailSlots.length > 1}
      />
    );
  }

  const mid = Math.ceil(indices.length / 2);
  const leftIndices = indices.slice(0, mid);
  const rightIndices = indices.slice(mid);
  const direction: "horizontal" | "vertical" =
    depth % 2 === 0 ? "horizontal" : "vertical";

  return (
    <ResizablePanelGroup
      orientation={direction}
      className="h-full w-full rounded-lg border min-h-0"
    >
      <ResizablePanel defaultSize={50} minSize={20}>
        <TiledLayout
          indices={leftIndices}
          detailSlots={detailSlots}
          activeSlotIndex={activeSlotIndex}
          onActivate={onActivate}
          onClose={onClose}
          variant={variant}
          depth={depth + 1}
        />
      </ResizablePanel>
      <ResizableHandle withHandle />
      <ResizablePanel defaultSize={50} minSize={20}>
        <TiledLayout
          indices={rightIndices}
          detailSlots={detailSlots}
          activeSlotIndex={activeSlotIndex}
          onActivate={onActivate}
          onClose={onClose}
          variant={variant}
          depth={depth + 1}
        />
      </ResizablePanel>
    </ResizablePanelGroup>
  );
}

export default function Guide() {
  const { gameVariants, isLoading: isLoadingVariants } =
    useGameVariants();
  const [selectedVariant, setSelectedVariant] =
    useState<GameVariant>("DarkDaysAhead");
  const [searchQuery, setSearchQuery] = useState("");
  const debouncedSearchQuery = useDebounce(searchQuery, 500);

  const [detailSlots, setDetailSlots] = useState<(string | null)[]>([
    null,
  ]);
  const [activeSlotIndex, setActiveSlotIndex] = useState(0);

  const { data: searchResults, isLoading: isSearching } =
    useGuideSearch(selectedVariant, debouncedSearchQuery);

  const handleSelectEntity = (id: string) => {
    setDetailSlots((prev) => {
      const newSlots = [...prev];
      newSlots[activeSlotIndex] = id;
      return newSlots;
    });
  };

  const addSlot = () => {
    setDetailSlots((prev) => [...prev, null]);
    setActiveSlotIndex(detailSlots.length);
  };

  const removeSlot = (index: number) => {
    if (detailSlots.length <= 1) return;
    setDetailSlots((prev) => prev.filter((_, i) => i !== index));
    if (activeSlotIndex >= index && activeSlotIndex > 0) {
      setActiveSlotIndex((prev) => prev - 1);
    }
  };

  return (
    <div className="space-y-4 flex flex-col h-[calc(100vh-8rem)]">
      <div className="flex flex-col gap-4 sm:flex-row sm:items-center">
        <h2 className="text-2xl font-bold">Guide</h2>
        <div className="flex gap-4 items-center flex-1">
          <VariantSelector
            gameVariants={gameVariants}
            selectedVariant={selectedVariant}
            onVariantChange={setSelectedVariant}
            isLoading={isLoadingVariants}
          />
          <SearchInput
            value={searchQuery}
            onChange={setSearchQuery}
            placeholder="Search items, monsters..."
            className="w-64"
          />
          {isSearching && (
            <Loader2 className="h-4 w-4 animate-spin text-muted-foreground" />
          )}
          <div className="flex-1" />
          <Button variant="outline" size="sm" onClick={addSlot}>
            <Plus className="h-4 w-4 mr-2" />
            Add Comparison
          </Button>
        </div>
      </div>

      <div className="grid grid-cols-1 md:grid-cols-4 gap-4 flex-1 min-h-0">
        <Card className="md:col-span-1 flex flex-col min-h-0">
          <CardHeader>
            <CardTitle>Search Results</CardTitle>
          </CardHeader>
          <CardContent className="flex-1 p-0 min-h-0">
            <ScrollArea className="h-full">
              <div className="p-4 space-y-2">
                {!debouncedSearchQuery && (
                  <p className="text-sm text-muted-foreground">
                    Type at least 2 characters to search.
                  </p>
                )}
                {debouncedSearchQuery &&
                  searchResults?.length === 0 &&
                  !isSearching && (
                    <p className="text-sm text-muted-foreground">
                      No results found.
                    </p>
                  )}
                {searchResults?.map((entry) => (
                  <button
                    key={entry.id}
                    onClick={() => handleSelectEntity(entry.id)}
                    className={`w-full text-left p-2 rounded hover:bg-accent transition-colors ${
                      detailSlots[activeSlotIndex] === entry.id
                        ? "bg-accent"
                        : ""
                    }`}
                  >
                    <div className="font-medium">
                      {entry.name || entry.id}
                    </div>
                    <div className="text-xs text-muted-foreground uppercase">
                      {entry.entry_type}
                    </div>
                  </button>
                ))}
              </div>
            </ScrollArea>
          </CardContent>
        </Card>

        <div className="md:col-span-3 min-h-0">
          <TiledLayout
            indices={Array.from(
              { length: detailSlots.length },
              (_, i) => i,
            )}
            detailSlots={detailSlots}
            activeSlotIndex={activeSlotIndex}
            onActivate={setActiveSlotIndex}
            onClose={removeSlot}
            variant={selectedVariant}
          />
        </div>
      </div>
    </div>
  );
}
