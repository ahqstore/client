import { ChevronLeft } from "lucide-react";

export default function BackButton({ backTo, set }: { backTo: number; set: (_: number) => void }) {
  return <button
    className="p-3 border border-border cursor-pointer mr-auto rounded-xl bg-primary/10"
    onClick={() => set(backTo)}
  >
    <ChevronLeft className="h-5 w-5" />
  </button>
}