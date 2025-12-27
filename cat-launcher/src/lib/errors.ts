/**
 * Shared error utilities for translating backend errors to user-friendly messages
 */

export interface BackendError {
  type: string;
  message: string;
}

/**
 * Checks if an error is a backend error object with type and message
 */
export function isBackendError(
  error: unknown,
): error is BackendError {
  return (
    typeof error === "object" &&
    error !== null &&
    "type" in error &&
    "message" in error &&
    typeof (error as Record<string, unknown>).type === "string" &&
    typeof (error as Record<string, unknown>).message === "string"
  );
}

/**
 * Generic error message constructor that can be extended by feature-specific modules
 */
export function constructErrorMessage(
  errorType: string,
  message: string,
  errorMap: Record<string, string>,
): string {
  const userFriendlyMessage = errorMap[errorType];

  if (userFriendlyMessage) {
    return userFriendlyMessage;
  }

  return `An error occurred: ${message}`;
}

/**
 * Extracts a user-friendly error message from various error types
 * Features should override this with their own error maps
 */
export function getUserFriendlyErrorMessage(
  error: unknown,
  errorMap?: Record<string, string>,
): string {
  if (isBackendError(error)) {
    if (errorMap) {
      return constructErrorMessage(
        error.type,
        error.message,
        errorMap,
      );
    }
    return error.message;
  }

  // Handle standard Error objects
  if (error instanceof Error) {
    return error.message;
  }

  // Handle string errors
  if (typeof error === "string") {
    return error;
  }

  // Fallback for unknown error types
  return "An unexpected error occurred. Please try again.";
}
