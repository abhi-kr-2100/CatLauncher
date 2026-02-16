import pkg from "../../../package.json";
import { openLink } from "@/lib/utils";
import { Button } from "@/components/ui/button";

const TEXT = {
  title: "CatLauncher",
  description:
    "An opinionated cross-platform launcher for Cataclysm games with modern social features.",
  version: (version: string) => `v${version}`,
  links: {
    github: "⭐ Star CatLauncher on GitHub",
    issue: "🐛 Report an issue",
    feature: "🚀 Request a new feature",
  },
} as const;

const LINKS = [
  {
    label: TEXT.links.github,
    url: "https://github.com/abhi-kr-2100/CatLauncher",
    variant: "outline" as const,
  },
  {
    label: TEXT.links.issue,
    url: "https://github.com/abhi-kr-2100/CatLauncher/issues/new",
    variant: "outline" as const,
  },
  {
    label: TEXT.links.feature,
    url: "https://github.com/abhi-kr-2100/CatLauncher/issues/new",
    variant: "outline" as const,
  },
];

export default function AboutPage() {
  return (
    <div className="flex flex-col items-center gap-4 py-4 max-w-lg mx-auto">
      <div className="flex flex-col items-center gap-2">
        <h1 className="text-2xl font-bold">{TEXT.title}</h1>
        <p className="text-center">{TEXT.description}</p>
        <p className="text-muted-foreground text-sm">
          {TEXT.version(pkg.version)}
        </p>
      </div>

      <div className="flex flex-col gap-2">
        {LINKS.map((link) => (
          <Button
            key={link.label}
            variant={link.variant}
            onClick={() => openLink(link.url)}
          >
            {link.label}
          </Button>
        ))}
      </div>
    </div>
  );
}
