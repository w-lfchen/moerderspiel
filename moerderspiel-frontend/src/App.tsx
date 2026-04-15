import { useState } from "react";
import Admin from "./pages/admin";
import Running from "./pages/running";
import Setup from "./pages/setup";
import { Button } from "./components/ui/button";
import type { JSX } from "react/jsx-runtime";

// TODO: adjust
export const backend_url = "https://localhost:3000";

function App() {
  const [page, setPage] = useState(0);
  let p: JSX.Element;
  switch (page) {
    case 1:
      p = <Running />;
      break;
    case 2:
      p = <Admin />;
      break;
    default:
      p = <Setup />;
      break;
  };

  return (
    <>
      {p}
      <br />
      <br />
      <Button onClick={() => setPage((page + 1) % 3)}>switch page</Button>
    </>
  );
}

export default App
