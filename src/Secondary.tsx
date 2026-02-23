export function Secondary() {
  return (
    <div
      data-tauri-drag-region
      className="bg-base-100 text-base-content border-base-content w-screen h-screen flex flex-col"
    >
      <div
        data-tauri-drag-region
        className="bg-base-300 py-2 flex text-neutral-content w-full items-center text-center justify-center mb-auto"
      >
        <img data-tauri-drag-region src="/icon.png" width={20} height={20} />
        <span data-tauri-drag-region className="ml-1 font-sans font-extrabold">
          AHQ Store
        </span>
      </div>
      <div className="mb-auto flex flex-col justify-center items-center text-center">
        <h1 data-tauri-drag-region>Enter this code</h1>
        <h1 data-tauri-drag-region className="font-extrabold text-2xl">
          {window.location.pathname.replace("/", "")}
        </h1>
      </div>
    </div>
  );
}
