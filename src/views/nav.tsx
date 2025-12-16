import { ReactNode } from "react";
import { items } from "./view";

function NavigationItem({
  name,
  icon,
  i,
  iconFilled,
  active,
  id,
  item,
  setItem,
}: {
  name: string;
  id: number;
  i: number;
  icon: ReactNode;
  iconFilled: ReactNode;
  active?: number[];
  item: number;
  setItem: (_: number) => void;
}) {
  const isActive = id == item || (active && active.includes(item));

  return (
    <div
      className={`flex transition-all size-20 rounded-md cursor-pointer nav_item ${isActive ? "act_nav_item" : ""} ${i == 2 ? "mt-auto" : ""}`}
      onClick={() => setItem(id)}
    >
      <div
        className={`bg-[var(--win32-accent)] h-[40%] w-2 rounded-lg my-auto ${isActive ? "" : "hidden"}`}
      ></div>
      <div
        className={`w-full h-full flex flex-col items-center justify-center text-center ${isActive ? "mr-2" : ""}`}
      >
        {isActive ? iconFilled : icon}
        {!isActive && <span>{name}</span>}
      </div>
    </div>
  );
}

export default function NavigationSidebar({
  item,
  setItem,
}: {
  item: number;
  setItem: (_: number) => void;
}) {
  return (
    <>
      {items
        .filter((s) => !(s.hidden && s.hidden()))
        .map((s, i) => (
          <NavigationItem
            key={s.name}
            {...s}
            item={item}
            setItem={setItem}
            i={i}
          />
        ))}
    </>
  );
}
