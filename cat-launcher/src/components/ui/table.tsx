import * as React from "react";

import { cn } from "@/lib/utils";

/**
 * Main container for a data table.
 *
 * @param props - Table element props.
 * @returns A React element representing the table.
 * @public
 */
function Table({
  className,
  ...props
}: React.ComponentProps<"table">) {
  return (
    <div
      data-slot="table-container"
      className="relative w-full overflow-x-auto"
    >
      <table
        data-slot="table"
        className={cn("w-full caption-bottom text-sm", className)}
        {...props}
      />
    </div>
  );
}

/**
 * The header section of a {@link Table}.
 *
 * @param props - Thead element props.
 * @returns A React element representing the table header.
 * @public
 */
function TableHeader({
  className,
  ...props
}: React.ComponentProps<"thead">) {
  return (
    <thead
      data-slot="table-header"
      className={cn("[&_tr]:border-b", className)}
      {...props}
    />
  );
}

/**
 * The body section of a {@link Table}.
 *
 * @param props - Tbody element props.
 * @returns A React element representing the table body.
 * @public
 */
function TableBody({
  className,
  ...props
}: React.ComponentProps<"tbody">) {
  return (
    <tbody
      data-slot="table-body"
      className={cn("[&_tr:last-child]:border-0", className)}
      {...props}
    />
  );
}

/**
 * The footer section of a {@link Table}.
 *
 * @param props - Tfoot element props.
 * @returns A React element representing the table footer.
 * @public
 */
function TableFooter({
  className,
  ...props
}: React.ComponentProps<"tfoot">) {
  return (
    <tfoot
      data-slot="table-footer"
      className={cn(
        "bg-muted/50 border-t font-medium [&>tr]:last:border-b-0",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A row within a {@link Table}.
 *
 * @param props - Tr element props.
 * @returns A React element representing a table row.
 * @public
 */
function TableRow({
  className,
  ...props
}: React.ComponentProps<"tr">) {
  return (
    <tr
      data-slot="table-row"
      className={cn(
        "hover:bg-muted/50 data-[state=selected]:bg-muted border-b transition-colors",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A header cell within a {@link TableHeader}.
 *
 * @param props - Th element props.
 * @returns A React element representing a table head cell.
 * @public
 */
function TableHead({
  className,
  ...props
}: React.ComponentProps<"th">) {
  return (
    <th
      data-slot="table-head"
      className={cn(
        "text-foreground h-10 px-2 text-left align-middle font-medium whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A data cell within a {@link TableRow}.
 *
 * @param props - Td element props.
 * @returns A React element representing a table cell.
 * @public
 */
function TableCell({
  className,
  ...props
}: React.ComponentProps<"td">) {
  return (
    <td
      data-slot="table-cell"
      className={cn(
        "p-2 align-middle whitespace-nowrap [&:has([role=checkbox])]:pr-0 [&>[role=checkbox]]:translate-y-[2px]",
        className,
      )}
      {...props}
    />
  );
}

/**
 * A caption for a {@link Table}.
 *
 * @param props - Caption element props.
 * @returns A React element representing the table caption.
 * @public
 */
function TableCaption({
  className,
  ...props
}: React.ComponentProps<"caption">) {
  return (
    <caption
      data-slot="table-caption"
      className={cn("text-muted-foreground mt-4 text-sm", className)}
      {...props}
    />
  );
}

export {
  Table,
  TableHeader,
  TableBody,
  TableFooter,
  TableHead,
  TableRow,
  TableCell,
  TableCaption,
};
