export default function ShowSpinner() {
  return (
    <div className="flex flex-col">
      <span className="loading loading-spinner loading-lg mx-auto"></span>
      <span className="text-lg">Almost There</span>
    </div>
  );
}