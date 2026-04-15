import { useState } from "react";
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";

import {
  Form,
  FormControl,
  FormField,
  FormItem,
  FormLabel,
  FormMessage,
} from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import Header from "@/components/header";
import { backend_url } from "@/App";
import { Label } from "@/components/ui/label";

export default function Setup() {
  const [selected, setSelected] = useState(null);
  const [registerMessage, setRegisterMessage] = useState(null);

  // slop doc example code
  const formSchema = z.object({
    name: z.string().min(1, {
      message: "grr empty string",
    }),
    password: z.string(),
    confirm: z.string(),
  }).refine(({ password, confirm }) => password === confirm, {
    message: "passwords don't match",
    path: ["confirm"],
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      password: "",
      confirm: "",
    },
  });

  async function onSubmit({ name, password }: z.infer<typeof formSchema>) {
    // TODO: hash password before sending
    if (selected === null) {
      setRegisterMessage(<Label className="text-red-500">Bitte wähle eine Variante aus.</Label>);
      return;
    }
    await fetch(backend_url + "/game/player", {
      method: "POST",
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({
        "name": name,
        "variant": 0,
        "hashed_password": password,
      }),
    }).then(response =>
      setRegisterMessage(response.ok ? <Label>Registrierung erfolgreich</Label> : <Label className="text-red-500">Fehler: {response.text()}</Label>)
    );
  }

  return (
    <>
      <Header />
      <Card>
        <CardHeader>
          <CardTitle>Registrieren</CardTitle>
        </CardHeader>
        <CardContent>
          <Form {...form}>
            <form onSubmit={form.handleSubmit(onSubmit)}>
              <FormField
                control={form.control}
                name="name"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Rufname</FormLabel>
                    <FormControl>
                      <Input placeholder="Rufname" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="grid grid-cols-2 gap-2">
                <Button className={selected == 1 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 1 ? "noShadow" : "default"} disabled={selected == 1} onClick={() => setSelected(1)}>1</Button>
                <Button className={selected == 2 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 2 ? "noShadow" : "default"} disabled={selected == 2} onClick={() => setSelected(2)}>2</Button>
                <Button className={selected == 3 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 3 ? "noShadow" : "default"} disabled={selected == 3} onClick={() => setSelected(3)}>3</Button>
                <Button className={selected == 4 ? "translate-x-boxShadowX translate-y-boxShadowY" : ""} variant={selected == 4 ? "noShadow" : "default"} disabled={selected == 4} onClick={() => setSelected(4)}>4</Button>
              </div>
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Passwort</FormLabel>
                    <FormControl>
                      <Input placeholder="Passwort" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="confirm"
                render={({ field }) => (
                  <FormItem>
                    <FormLabel>Passwort bestätigen</FormLabel>
                    <FormControl>
                      <Input placeholder="Passwort" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <Button type="submit">Bestätigen</Button>
              {registerMessage}
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}
