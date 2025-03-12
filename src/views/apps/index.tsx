import { Search } from "lucide-react";

export function AppsHome() {
  return <>
    <div className="flex w-full justify-center text-center items-center">
      <div className="relative w-full max-w-xl">
        <input
          type="text"
          placeholder="Search for apps, games, and more"
          className="w-full py-3 pl-9 pr-3 bg-primary/10 rounded-xl border border-border focus:outline-none focus:ring-1 focus:ring-muted transition duration-300"
        />
        <Search className="absolute left-2.5 top-1/2 -translate-y-1/2 text-muted-foreground h-5 w-5" />
      </div>
    </div>

    <div className="flex flex-col lg:flex-row grow-0 w-full gap-3 animate">
      <div className="hero" style={{ background: `url("https://www.39digits.com/static/5cda054ca018a2f574e6a08a533ff251/2bef9/firefox-logo-banner.png") center/cover` }}>
        <div></div>
        <div className="flex flex-col h-full">
          <h1>Your Next Browser!</h1>
          <h2 className="">Get the browser that puts your privacy first — and always has</h2>

          <div className="w-full">
            <button>Explore Firefox</button>
          </div>

          <h3 className="mt-auto mb-2">&copy; Mozilla Firefox</h3>
        </div>
      </div>

      <div className="cards">
        <div>
          A
        </div>
        <div>
          A
        </div>
        <div>
          A
        </div>
        <div>
          A
        </div>
      </div>
    </div>
  </>;
}