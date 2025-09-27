import { ChevronLeft } from "lucide-react";

export default function BackButton({ backTo, set, noRightRound = false }: { backTo: number; set: (_: number) => void, noRightRound: boolean }) {
  return <button
    className={`p-3 border border-border cursor-pointer ${noRightRound ? "mr-0 md:mr-auto" : "mr-auto"} rounded-xl ${noRightRound ? "rounded-r-none md:rounded-r-xl" : ""} bg-primary/10`}
    onClick={() => set(backTo)}
  >
    <ChevronLeft className="h-5 w-5" />
  </button>
}