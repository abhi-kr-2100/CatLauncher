import { ReactNode } from "react";

import { Button } from "@/components/ui/button";
import { openLink } from "@/lib/utils";
import { ExternalLinkIcon } from "lucide-react";

/**
 * Props for the {@link ExternalLink} component.
 * @public
 */
interface ExternalLinkProps {
  /** The URL to open when the link is clicked. */
  href: string;
  /** The content to be displayed within the link. */
  children: ReactNode;
}

/**
 * A link component that opens an external URL using the system's default browser.
 *
 * @param props - The component props.
 * @returns A React element representing the external link button.
 * @public
 */
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
