import { ReactNode } from "react";

import { Button } from "@/components/ui/button";
import { openLink } from "@/lib/utils";
import { ExternalLinkIcon } from "lucide-react";

interface ExternalLinkProps {
  href: string;
  children: ReactNode;
}

export function ExternalLink({ href, children }: ExternalLinkProps) {
  return (
    <Button
      variant="link"
      onClick={() => openLink(href)}
      size={null}
      className="underline p-0"
    >
      {children}
      <ExternalLinkIcon />
    </Button>
  );
}
