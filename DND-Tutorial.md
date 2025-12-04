# DND-Kit Tutorial

This document explains how `dnd-kit` is used in this project to enable drag-and-drop functionality for reordering game variant cards.

## `PlayPage/index.tsx`

This file contains the main logic for the drag-and-drop functionality.

```tsx
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
```

### Line-by-Line Explanation

-   **`import { ... } from "@dnd-kit/core";`**: Imports the necessary components from the `@dnd-kit/core` library.
    -   **`DndContext`**: The main provider component that enables drag-and-drop functionality.
    -   **`closestCenter`**: A collision detection algorithm that determines which droppable element is closest to the draggable element.
    -   **`KeyboardSensor` and `PointerSensor`**: Sensors that allow users to interact with draggable elements using a keyboard or a mouse/pointer.
    -   **`useSensor` and `useSensors`**: Hooks used to create and configure the sensors.
    -   **`DragEndEvent`**: The type of the event that is fired when a drag operation ends.
-   **`import { ... } from "@dnd-kit/sortable";`**: Imports the necessary components from the `@dnd-kit/sortable` library.
    -   **`arrayMove`**: A utility function that moves an element in an array from one position to another.
    -   **`SortableContext`**: A component that provides the context for sortable elements.
    -   **`sortableKeyboardCoordinates`**: A utility function that provides the coordinates for keyboard sorting.
    -   **`verticalListSortingStrategy`**: A sorting strategy that is optimized for vertical lists.
-   **`const [orderedItems, setOrderedItems] = useState<GameVariantInfo[]>([]);`**: A state variable that stores the ordered list of game variants.
-   **`useEffect(() => { ... }, [gameVariantsInfo]);`**: An effect that updates the `orderedItems` state when the `gameVariantsInfo` data is fetched.
-   **`const sensors = useSensors(...)`**: Creates and configures the sensors that will be used to detect drag operations.
-   **`const { mutate } = useMutation(...)`**: A mutation from `react-query` that calls the `updateGameVariantOrder` command to update the order of the game variants in the database.
-   **`function handleDragEnd(event: DragEndEvent) { ... }`**: The event handler that is called when a drag operation ends.
    -   **`const { active, over } = event;`**: Destructures the `active` and `over` properties from the `DragEndEvent` object.
    -   **`if (over && active.id !== over.id) { ... }`**: Checks if the draggable element was dropped over a different droppable element.
    -   **`const oldIndex = ...` and `const newIndex = ...`**: Finds the old and new indexes of the dragged element.
    -   **`const newOrder = arrayMove(orderedItems, oldIndex, newIndex);`**: Creates a new array with the updated order.
    -   **`setOrderedItems(newOrder);`**: Updates the `orderedItems` state with the new order.
    -   **`mutate(newOrder.map((item) => item.id));`**: Calls the `updateGameVariantOrder` mutation to update the order in the database.
-   **`<DndContext ...>`**: The main provider component for `dnd-kit`.
-   **`<SortableContext ...>`**: Provides the context for the sortable elements.
-   **`{orderedItems.map((variantInfo) => (...))}`**: Renders the list of game variant cards.

## `GameVariantCard.tsx`

This file contains the component for the game variant card.

```tsx
import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { GripVertical } from "lucide-react";
import { useState } from "react";

import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
} from "@/components/ui/card";
import { ExternalLink } from "@/components/ui/ExternalLink";
import { TipOfTheDay } from "@/game-tips/TipOfTheDay";
import type { GameVariantInfo } from "@/generated-types/GameVariantInfo";
import InteractionButton from "./InteractionButton";
import { PlayTime } from "./PlayTime";
import ReleaseSelector from "./ReleaseSelector";

export interface GameVariantProps {
    variantInfo: GameVariantInfo;
}

export default function GameVariantCard({ variantInfo }: GameVariantProps) {
    const [selectedReleaseId, setSelectedReleaseId] = useState<
        string | undefined
    >();
    const {
        attributes,
        listeners,
        setNodeRef,
        transform,
        transition,
    } = useSortable({ id: variantInfo.id });

    const style = {
        transform: CSS.Transform.toString(transform),
        transition,
    };

    return (
        <Card ref={setNodeRef} style={style} {...attributes}>
            <CardHeader>
                <div className="flex justify-between items-start">
                    <div>
                        <CardTitle>{variantInfo.name}</CardTitle>
                        <CardDescription>
                            <div className="flex gap-5">
                                {variantInfo.links.map((link) => (
                                    <ExternalLink key={link.href} href={link.href}>
                                        {link.label}
                                    </ExternalLink>
                                ))}
                            </div>
                        </CardDescription>
                    </div>
                    <div {...listeners} className="cursor-grab">
                        <GripVertical />
                    </div>
                </div>
            </CardHeader>
            <CardContent className="flex flex-col gap-4">
                <TipOfTheDay variant={variantInfo.id} />
                <ReleaseSelector
                    variant={variantInfo.id}
                    selectedReleaseId={selectedReleaseId}
                    setSelectedReleaseId={setSelectedReleaseId}
                />
            </CardContent>
            <CardFooter className="flex flex-col gap-4 items-stretch">
                <InteractionButton
                    variant={variantInfo.id}
                    selectedReleaseId={selectedReleaseId}
                />
                <PlayTime variant={variantInfo.id} releaseId={selectedReleaseId} />
            </CardFooter>
        </Card>
    );
}
```

### Line-by-Line Explanation

-   **`import { useSortable } from "@dnd-kit/sortable";`**: Imports the `useSortable` hook from the `@dnd-kit/sortable` library.
-   **`import { CSS } from "@dnd-kit/utilities";`**: Imports the `CSS` utility from the `@dnd-kit/utilities` library.
-   **`const { ... } = useSortable({ id: variantInfo.id });`**: The `useSortable` hook that makes the component draggable.
    -   **`attributes`**: The attributes that need to be applied to the draggable element.
    -   **`listeners`**: The event listeners that need to be applied to the drag handle.
    -   **`setNodeRef`**: A function that is used to set the ref of the draggable element.
    -   **`transform` and `transition`**: The transform and transition styles that are applied to the draggable element.
-   **`const style = { ... };`**: The style that is applied to the draggable element.
-   **`<Card ref={setNodeRef} style={style} {...attributes}>`**: The main card component.
    -   **`ref={setNodeRef}`**: Sets the ref of the draggable element.
    -   **`style={style}`**: Applies the transform and transition styles.
    -   **`{...attributes}`**: Applies the draggable attributes.
-   **`<div {...listeners} className="cursor-grab">`**: The drag handle.
    -   **`{...listeners}`**: Applies the event listeners.
    -   **`className="cursor-grab"`**: Sets the cursor to a grab icon.
-   **`<GripVertical />`**: The icon for the drag handle.
