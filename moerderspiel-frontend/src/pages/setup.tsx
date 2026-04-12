import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import Header from "../components/header";

export default function Setup() {
  const [selected, setSelected] = useState(0);
  return (
    <>
      <Header />
      <Input type="text" placeholder="Rufname" />
      <div className="grid grid-cols-2 gap-2">
        <Button className={selected == 1 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 1 ? "noShadow" : "default"} disabled={selected == 1} onClick={() => setSelected(1)}>1</Button>
        <Button className={selected == 2 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 2 ? "noShadow" : "default"} disabled={selected == 2} onClick={() => setSelected(2)}>2</Button>
        <Button className={selected == 3 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 3 ? "noShadow" : "default"} disabled={selected == 3} onClick={() => setSelected(3)}>3</Button>
        <Button className={selected == 4 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 4 ? "noShadow" : "default"} disabled={selected == 4} onClick={() => setSelected(4)}>4</Button>
      </div>
      <Input type="password" placeholder="Passwort" />
      <Input type="password" placeholder="Passwort wiederholen" />
      <Button>Registrieren</Button>
    </>
  );
}
