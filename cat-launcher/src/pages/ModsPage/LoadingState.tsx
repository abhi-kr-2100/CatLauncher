export default function LoadingState() {
  return (
    <div
      className="flex flex-col items-center justify-center h-full"
      role="status"
      aria-live="polite"
      aria-busy="true"
    >
      <div className="flex items-center gap-2">
        <div className="animate-spin rounded-full h-4 w-4 border-b-2 border-primary" aria-hidden="true" />
        <p className="text-lg">Loading mods...</p>
      </div>
    </div>
  );
}
