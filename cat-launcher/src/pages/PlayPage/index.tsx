import {
    DndContext,
    closestCenter,
    KeyboardSensor,
    PointerSensor,
    useSensor,
    useSensors,
    DragEndEvent,
} from "@dnd-kit/core";
import {
    arrayMove,
    SortableContext,
    sortableKeyboardCoordinates,
    verticalListSortingStrategy,
} from "@dnd-kit/sortable";
import { useMutation, useQuery, useQueryClient } from "@tanstack/react-query";
import { useEffect, useState } from "react";

import GameVariantCard from "./GameVariantCard";
import {
    fetchGameVariantsInfo,
    updateGameVariantOrder,
} from "@/lib/commands";
import { GameVariantInfo } from "@/generated-types/GameVariantInfo";

function PlayPage() {
    const queryClient = useQueryClient();
    const {
        data: gameVariantsInfo = [],
        isLoading: gameVariantsLoading,
        isError: gameVariantsError,
        error: gameVariantsErrorObj,
    } = useQuery<GameVariantInfo[]>({
        queryKey: ["gameVariantsInfo"],
        queryFn: fetchGameVariantsInfo,
    });
    const [orderedItems, setOrderedItems] = useState<GameVariantInfo[]>([]);

    useEffect(() => {
        if (gameVariantsInfo) {
            setOrderedItems(gameVariantsInfo);
        }
    }, [gameVariantsInfo]);

    const sensors = useSensors(
        useSensor(PointerSensor),
        useSensor(KeyboardSensor, {
            coordinateGetter: sortableKeyboardCoordinates,
        }),
    );

    const { mutate } = useMutation({
        mutationFn: updateGameVariantOrder,
        onSuccess: () => {
            queryClient.invalidateQueries({ queryKey: ["gameVariantsInfo"] });
        },
    });

    function handleDragEnd(event: DragEndEvent) {
        const { active, over } = event;

        if (over && active.id !== over.id) {
            const oldIndex = orderedItems.findIndex(
                (item) => item.id === active.id,
            );
            const newIndex = orderedItems.findIndex((item) => item.id === over.id);

            const newOrder = arrayMove(orderedItems, oldIndex, newIndex);
            setOrderedItems(newOrder);
            mutate(newOrder.map((item) => item.id));
        }
    }

    if (gameVariantsLoading) {
        return <p>Loading...</p>;
    }

    if (gameVariantsError) {
        return <p>Error: {gameVariantsErrorObj?.message ?? "Unknown error"}</p>;
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
                <main className="grid grid-cols-[repeat(auto-fit,minmax(20rem,1fr))] gap-2 p-2">
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
