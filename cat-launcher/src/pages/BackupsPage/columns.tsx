import { ColumnDef } from "@tanstack/react-table";
import { ArrowUpDown } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { CombinedBackup } from "./types/backups";

type ColumnsFactory = (options: {
  openDeleteDialog: (backup: CombinedBackup) => void;
  openRestoreDialog: (backup: CombinedBackup) => void;
}) => ColumnDef<CombinedBackup>[];

export const columns: ColumnsFactory = ({
  openDeleteDialog,
  openRestoreDialog,
}) => [
  {
    accessorKey: "name",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Name
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
  },
  {
    header: "Type",
    accessorKey: "type",
    cell: ({ row }) => {
      const type = row.getValue("type") as string;
      return (
        <Badge variant={type === "manual" ? "default" : "outline"}>
          {type.toUpperCase()}
        </Badge>
      );
    },
  },
  {
    accessorKey: "notes",
    header: "Notes",
  },
  {
    accessorKey: "timestamp",
    header: ({ column }) => {
      return (
        <Button
          variant="ghost"
          onClick={() => column.toggleSorting(column.getIsSorted() === "asc")}
        >
          Date
          <ArrowUpDown className="ml-2 h-4 w-4" />
        </Button>
      );
    },
    cell: ({ row }) => {
      const timestamp = row.getValue("timestamp") as bigint;
      const date = new Date(Number(timestamp) * 1000);

      const day = date.getDate().toString().padStart(2, "0");
      const month = date.toLocaleString("default", { month: "long" });
      const year = date.getFullYear();
      const hours = date.getHours().toString().padStart(2, "0");
      const minutes = date.getMinutes().toString().padStart(2, "0");
      const seconds = date.getSeconds().toString().padStart(2, "0");

      const formattedDate = `${day} ${month}, ${year}, ${hours}:${minutes}:${seconds}`;

      return <div>{formattedDate}</div>;
    },
  },
  {
    id: "actions",
    cell: ({ row }) => {
      const backup = row.original;

      return (
        <div className="flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            onClick={() => openRestoreDialog(backup)}
          >
            Restore
          </Button>
          <Button
            variant="destructive"
            size="sm"
            onClick={() => openDeleteDialog(backup)}
          >
            Delete
          </Button>
        </div>
      );
    },
  },
];
