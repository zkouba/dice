// Tiny hash-based router. Routes:
//   #/            -> roller (home)
//   #/sets        -> set management list
//   #/sets/new    -> create a set
//   #/sets/<id>   -> edit a set
export type Route =
  | { name: "roller" }
  | { name: "list" }
  | { name: "editor"; id: string | null };

export function parseRoute(): Route {
  const parts = location.hash.replace(/^#/, "").split("/").filter(Boolean);
  if (parts[0] === "sets") {
    if (parts[1] === "new") return { name: "editor", id: null };
    if (parts[1]) return { name: "editor", id: parts[1] };
    return { name: "list" };
  }
  return { name: "roller" };
}

export function navigate(path: string): void {
  location.hash = path;
}

export function onRouteChange(fn: () => void): void {
  window.addEventListener("hashchange", fn);
}
