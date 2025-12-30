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

import { useGameVariants } from "@/hooks/useGameVariants";
import { t } from "@/i18n";
import { toastCL } from "@/lib/utils";
import GameVariantCard from "./GameVariantCard";

function PlayPage() {
  const {
    gameVariants: orderedItems,
    updateOrder,
    isLoading: gameVariantsLoading,
    isError: gameVariantsError,
    error: gameVariantsErrorObj,
  } = useGameVariants({
    onOrderUpdateError: (error) => {
      toastCL(
        "error",
        t("i18n.error.failedToUpdateGameVariantsOrder"),
        error,
      );
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
        (item) => item.id === active.id,
      );
      const newIndex = orderedItems.findIndex(
        (item) => item.id === over.id,
      );

      const newOrder = arrayMove(orderedItems, oldIndex, newIndex);
      updateOrder(newOrder);
    }
  }

  if (gameVariantsLoading) {
    return <p>{t("i18n.loading")}</p>;
  }

  if (gameVariantsError) {
    return (
      <p>
        {t("i18n.error")}:{" "}
        {gameVariantsErrorObj?.message ?? t("i18n.error.unknown")}
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
        items={orderedItems.map((item) => item.id)}
        strategy={verticalListSortingStrategy}
      >
        <main className="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-2">
          {orderedItems.map((variantInfo) => (
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
