import {
  closestCenter,
  DndContext,
  DragEndEvent,
  KeyboardSensor,
  PointerSensor,
  useSensor,
  useSensors,
} from "@dnd-kit/core";
import {
  arrayMove,
  SortableContext,
  sortableKeyboardCoordinates,
  verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";

import { useGameVariants } from "@/hooks/useGameVariants";
import { toastCL } from "@/lib/utils";
import GameVariantCard from "./GameVariantCard";

function PlayPage() {
  const {
    gameVariants: orderedItems = [],
    updateOrder,
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useGameVariants<GameVariantInfo[], Error, GameVariantInfo[]>({
    mutationOptions: {
      onError: (error: Error) => {
        toastCL(
          "error",
          "Failed to update game variants order",
          error,
        );
      },
    },
    queryOptions: {
      onError: (error: Error) => {
        toastCL("error", "Failed to fetch game variants", error);
      },
    },
  });

  const sensors = useSensors(
    useSensor(PointerSensor),
    useSensor(KeyboardSensor, {
      coordinateGetter: sortableKeyboardCoordinates,
    }),
  );

  function handleDragEnd(event: DragEndEvent) {
    const { active, over } = event;

    if (over && active.id !== over.id) {
      const oldIndex = orderedItems.findIndex(
        (item: GameVariantInfo) => item.id === active.id,
      );
      const newIndex = orderedItems.findIndex(
        (item: GameVariantInfo) => item.id === over.id,
      );

      const newOrder = arrayMove(orderedItems, oldIndex, newIndex);
      updateOrder(newOrder);
    }
  }

  if (gameVariantsLoading) {
    return <p>{"Loading..."}</p>;
  }

  if (gameVariantsError) {
    return (
      <p>
        {"Error:"} {gameVariantsErrorObj?.message ?? "Unknown error"}
      </p>
    );
  }

  return (
    <DndContext
      sensors={sensors}
      collisionDetection={closestCenter}
      onDragEnd={handleDragEnd}
    >
      <SortableContext
        items={orderedItems.map((item: GameVariantInfo) => item.id)}
        strategy={verticalListSortingStrategy}
      >
        <main className="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-2">
          {orderedItems.map((variantInfo: GameVariantInfo) => (
            <GameVariantCard
              key={variantInfo.id}
              variantInfo={variantInfo}
            />
          ))}
        </main>
      </SortableContext>
    </DndContext>
  );
}

export default PlayPage;
