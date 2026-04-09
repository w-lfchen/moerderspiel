import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuTrigger, DropdownMenuRadioGroup, DropdownMenuRadioItem } from "@/components/ui/dropdown-menu";
import { Input } from "@/components/ui/input";
import { useState } from "react";
import Header from "./header";

export default function Setup() {
  const [position, setPosition] = useState("0")
  return (
    <>
      <Header />
      <div className="flex">
        <DropdownMenu>
          <DropdownMenuTrigger asChild>
            <Button variant="noShadow">Auswahl</Button>
          </DropdownMenuTrigger>
          <DropdownMenuContent>
            <DropdownMenuGroup>
              <DropdownMenuRadioGroup value={position} onValueChange={setPosition}>
                <DropdownMenuRadioItem value="1">1</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="2">2</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="3">3</DropdownMenuRadioItem>
                <DropdownMenuRadioItem value="4">4</DropdownMenuRadioItem>
              </DropdownMenuRadioGroup>
            </DropdownMenuGroup>
          </DropdownMenuContent>
        </DropdownMenu>
        <Input type="text" placeholder="Rufname" />
      </div>
      <div className="flex">
        <Input type="password" placeholder="Passwort" />
        <Button>Status anfragen</Button>
      </div>
    </>
  );
  // TODO: finish
}
