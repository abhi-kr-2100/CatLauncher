# Frontend Architecture

The frontend is a standard React application and follows a similar vertical slice structure as the backend. It is organized by screens (e.g., `PlayPage`).

## Frontend-Backend Interaction

The frontend communicates with the backend by invoking the commands defined in the Rust `commands.rs` files. To keep the UI components decoupled from the backend implementation:
- All Tauri `invoke` calls are wrapped in async functions inside `src/lib/commands.ts`.
- UI components call these wrapper functions, remaining unaware of the underlying Tauri API. This makes the components more reusable and easier to test.
- TanStack Query keys are collected into constants in `src/lib/queryKeys.ts`.

## Error Handling

The frontend uses `@tanstack/react-query` to manage API state and a centralized utility function to display toast notifications.

1.  **API State Management**: Components use the `useQuery` hook from `react-query` to call backend commands. This hook automatically provides `data`, `isLoading`, and `error` states.

2.  **Error Handling and Notification**: A `useEffect` hook monitors the `error` object from `useQuery`. If an error exists, it calls the `toastCL` utility to show a user-friendly message. This keeps the error-handling logic separate from the main component rendering.

3.  **Graceful Degradation**: The component uses the loading and error states from `useQuery` to render a responsive UI, such as showing a loading message or disabling elements when an error occurs.
