const translations = {
  en: {
    "i18n.loading": "Loading...",
    "i18n.error": "Error",
    "i18n.error.unknown": "Unknown error",
    "i18n.error.failedToUpdateGameVariantsOrder":
      "Failed to update game variants order",
  },
};

export const t = (key: keyof (typeof translations)["en"]) => {
  return translations["en"][key] || key;
};
