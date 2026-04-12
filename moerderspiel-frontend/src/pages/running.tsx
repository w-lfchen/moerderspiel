import { Button } from "@/components/ui/button";
import { Input } from "@/components/ui/input";
import Header from "../components/header";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import Selector from "@/components/selector";

export default function Setup() {
  return (
    <>
      <Header />
      <div className="flex">
        <Selector />
        <Input type="text" placeholder="Rufname" />
      </div>
      <div className="flex">
        <Input type="password" placeholder="Passwort" />
        <Button>Status anfragen</Button>
      </div>
      <Card>
        <CardHeader>
          <CardTitle>Status</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2">
            <p>Leben</p>
            <p>Kills</p>
          </div>
        </CardContent>
      </Card>
      <div className="grid grid-cols-1">
        <Button>kill order placehoder</Button>
        <Button>kill order placehoder</Button>
        <Button>kill order placehoder</Button>
      </div>
    </>
  );
  // TODO: finish
}
