/**
 * Error messages for Theme functionality
 */

export const themeErrorMap: Record<string, string> = {
  // Theme errors
  GetPreferredThemeCommandError_Get:
    "Failed to load theme preference. Please try again.",
  SetPreferredThemeCommandError_Update:
    "Failed to update theme preference. Please try again.",

  // User errors
  GetUserIdCommandError_GetOrCreateUserId:
    "Failed to initialize user. Please restart the launcher.",
};
