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

    <div className="flex flex-col mt-3 lg:flex-row grow-0 w-full gap-5 animate">
      <div className="hero" style={{ background: `url("https://www.39digits.com/static/5cda054ca018a2f574e6a08a533ff251/2bef9/firefox-logo-banner.png") center/cover` }}>
        <div></div>
        <div className="flex flex-col h-full">
          <h1>Your Next Browser!</h1>
          <h2 className="">Get the browser that puts your privacy first — and always has</h2>

          <div className="w-full">
            <button>Explore Firefox</button>
          </div>

          <h3 className="mt-auto mb-2 hidden md:block">Mozilla Firefox</h3>
        </div>
      </div>

      <div className="cards">
        <div>
          <div>
            VLC Media Player
          </div>
          <img src="https://image.winudf.com/v2/image1/b3JnLnZpZGVvbGFuLnZsY19iYW5uZXJfMTU1NTA2ODYzMl8wNDA/banner.jpg?fakeurl=1&w=600" />
        </div>

        <div>
          <div></div>
          <img src="https://img-c.udemycdn.com/course/750x422/4466386_ab00_3.jpg" />
        </div>

        <div>
          <div></div>
          <img src="https://cdn.mos.cms.futurecdn.net/K9WURpCKWvEZH6L4Cg428g.jpg" />
        </div>
      </div>
    </div>
  </>;
}