import { useEffect, useState } from "react";
import Running from "./pages/running";
import Setup from "./pages/setup";
import { fetch_get } from "./lib/utils";
import { Label } from "@/components/ui/label";
import Finished from "./pages/finished";

function App() {
  const [page, setPage] = useState(<Label>loading...</Label>);
  useEffect(() => {
    fetch_get("/game").then(async response => {
      if (response.ok) {
        const s = await response.json();
        switch (s.phase) {
          case "Setup":
            setPage(<Setup />);
            break;
          case "Running":
            setPage(<Running />);
            break;
          case "Finished":
            setPage(<Finished />);
            break;
        }
      } else {
        const text = await response.text();
        setPage(<Label>Fehler: {text}</Label>);
      }
    });
  }, []);

  return (
    <div className="text-sm">
      {page}
    </div>
  );
}

export default App
