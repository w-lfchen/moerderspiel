import { Button } from "@/components/ui/button";
import { DropdownMenu, DropdownMenuContent, DropdownMenuGroup, DropdownMenuTrigger, DropdownMenuRadioGroup, DropdownMenuRadioItem } from "@/components/ui/dropdown-menu";
import { useState } from "react";

export default function Selector() {
  // TODO: improve
  const [position, setPosition] = useState("0")
  return (
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
  );
}
