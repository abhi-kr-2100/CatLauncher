import { ReactNode } from "react";

import {
  Accordion,
  AccordionContent,
  AccordionItem,
  AccordionTrigger,
} from "@/components/ui/accordion";
import { WorldOptionMetadata } from "@/generated-types/WorldOptionMetadata";

interface GroupOptionProps {
  metadata: WorldOptionMetadata;
  children: ReactNode;
}

export function GroupOption({
  metadata,
  children,
}: GroupOptionProps) {
  return (
    <Accordion type="single" collapsible className="w-full">
      <AccordionItem value={metadata.id} className="border-b-0">
        <AccordionTrigger className="hover:no-underline py-2">
          <div className="text-left">
            <div className="text-sm font-bold uppercase tracking-wider text-muted-foreground">
              {metadata.name}
            </div>
            <div className="text-xs font-normal text-muted-foreground/70">
              {metadata.description}
            </div>
          </div>
        </AccordionTrigger>
        <AccordionContent className="pt-4 pb-2 border-l-2 ml-2 border-muted">
          {children}
        </AccordionContent>
      </AccordionItem>
    </Accordion>
  );
}
