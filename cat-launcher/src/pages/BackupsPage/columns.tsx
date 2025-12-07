import { ColumnDef } from "@tanstack/react-table";
import { ArrowUpDown } from "lucide-react";

import { Badge } from "@/components/ui/badge";
import { Button } from "@/components/ui/button";
import { BackupEntry } from "@/generated-types/BackupEntry";

type ColumnsFactory = (options: {
  openDeleteDialog: (backup: BackupEntry) => void;
  openRestoreDialog: (backup: BackupEntry) => void;
}) => ColumnDef<BackupEntry>[];

export const columns: ColumnsFactory = ({
  openDeleteDialog,
  openRestoreDialog,
}) => [
  {
    accessorKey: "id",
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
    cell: ({ row }) => {
      const backup = row.original;
      return <div>Automatic-{backup.id}</div>;
    },
  },
  {
    header: "Type",
    cell: () => {
      return <Badge variant="outline">AUTO</Badge>;
    },
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
      const timestamp = row.getValue("timestamp") as number;
      const date = new Date(timestamp * 1000);

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
