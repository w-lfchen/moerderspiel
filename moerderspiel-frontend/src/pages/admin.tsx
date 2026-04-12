import { Button } from "@/components/ui/button";
import Header from "../components/header";
import { Input } from "@/components/ui/input";
import { Dialog, DialogClose, DialogContent, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from "@/components/ui/dialog";
import { Label } from "@/components/ui/label";
import Selector from "@/components/selector";

export default function Admin() {
  return (
    <>
      <Header />
      <Button>Start</Button>
      <Button>Finish</Button>
      <div className="flex">
        <Selector />
        <Input type="text" placeholder="Rufname" />
        <Dialog>
          <form>
            <DialogTrigger asChild>
              <Button>edit user</Button>
            </DialogTrigger>
            <DialogContent>
              <DialogHeader>
                <DialogTitle>edit user</DialogTitle>
              </DialogHeader>
              <div className="grid gap-4">
                <div className="grid gap-3">
                  <Label>Rufname</Label>
                  <Input defaultValue="Rufname" />
                </div>
                <div className="grid gap-3">
                  <Label>TODO: auswahl einfügen</Label>
                </div>
              </div>
              <DialogFooter>
                <DialogClose asChild>
                  <Button variant="neutral">Cancel</Button>
                </DialogClose>
                <Button type="submit">Save changes</Button>
              </DialogFooter>
            </DialogContent>
          </form>
        </Dialog>
      </div>
    </>
  );
}
// TODO: improve on copied example
