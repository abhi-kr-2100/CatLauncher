import { FC } from "react";
import { Page } from "@/components/Page";
import { GeneralSettings } from "./General";
import { FontSettings } from "./Fonts";
import { ThemeSettings } from "./Themes";

export const Settings: FC = () => {
  return (
    <Page title="Settings">
      <div className="space-y-4">
        <GeneralSettings />
        <FontSettings />
        <ThemeSettings />
      </div>
    </Page>
  );
};
