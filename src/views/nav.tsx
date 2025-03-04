import { items } from "./view";

function NavigationItem({ name, icon, i, iconFilled, id, item, setItem }: {
  name: string;
  id: number;
  i: number;
  icon: JSX.Element,
  iconFilled: JSX.Element,
  item: number,
  setItem: (_: number) => void
}) {
  return <div className={`flex flex-col transition-all items-center justify-center text-center size-18 rounded-lg cursor-pointer nav_item ${item == id ? "act_nav_item" : ""} ${i == 2 ? "mt-auto" : ""}`} onClick={() => setItem(id)}>
    {item == id ? iconFilled : icon}
    {item != id && <span>{name}</span>}
  </div>;
}

export default function NavigationSidebar({ item, setItem }: { item: number; setItem: (_: number) => void }) {
  return (
    <>
      {items
        .filter((s) => !(s.hidden && s.hidden()))
        .map((s, i) =>
          <NavigationItem key={s.name} {...s} item={item} setItem={setItem} i={i} />
        )
      }
    </>
  );
}