import { useSortable } from "@dnd-kit/sortable";
import { CSS } from "@dnd-kit/utilities";
import { CSSProperties } from "react";

/**
 * A custom hook that provides sortable functionality for an item using `@dnd-kit/sortable`.
 *
 * @param id - The unique identifier of the sortable item.
 * @returns An object containing dnd-kit attributes, listeners, refs, styles, and dragging state.
 */
export function useSortableItem(id: string) {
  const {
    attributes,
    listeners,
    setNodeRef,
    transform,
    transition,
    isDragging,
  } = useSortable({ id });

  const style: CSSProperties = {
    transform: CSS.Transform.toString(transform),
    transition,
    zIndex: isDragging ? 1 : undefined,
  };

  return {
    attributes,
    listeners,
    setNodeRef,
    style,
    isDragging,
  };
}
