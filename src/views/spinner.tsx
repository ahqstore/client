export default function ShowSpinner() {
  return (
    <div className="flex flex-col">
      <span className="loading loading-spinner w-[calc(var(--size-selector,0.25rem)*8)] mx-auto"></span>
      {/* <span className="text-lg">Almost There</span> */}
    </div>
  );
}