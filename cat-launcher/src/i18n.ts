import i18n from "i18next";
import { initReactI18next } from "react-i18next";

const resources = {
  en: {
    translation: {
      "Select a Release to Play": "Select a Release to Play",
      "Running...": "Running...",
      "Downloading...": "Downloading...",
      "Installing...": "Installing...",
      "Loading...": "Loading...",
      Play: "Play",
      "Not Available": "Not Available",
      Install: "Install",
      "Resume Last World": "Resume Last World",
      Upgrade: "Upgrade",
      "Install and switch to the latest experimental version.":
        "Install and switch to the latest experimental version.",
      "This release is not yet available. Try again in a couple of hours.":
        "This release is not yet available. Try again in a couple of hours.",
      played_time_in_hours: "{{hours}}h",
      played_time_in_minutes: "< 1m",
      played_time_in_hours_and_minutes: "{{hours}}h {{minutes}}m",
      "Version playtime": "Version playtime",
      "Total playtime": "Total playtime",
    },
  },
};

i18n.use(initReactI18next).init({
  resources,
  lng: "en",
  interpolation: {
    escapeValue: false,
  },
});

export default i18n;
