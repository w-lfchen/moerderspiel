import { useState, type JSX } from "react";
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from '@/components/ui/button';
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from '@/components/ui/input';
import { Label } from "@/components/ui/label";

import Header from "@/components/header";
import { fetch_post, digestMessage } from "@/lib/utils";

export default function Setup() {
  const [registerMessage, setRegisterMessage] = useState<JSX.Element | null>(null);
  const [isLoading, setIsLoading] = useState(false);

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
    setRegisterMessage(null);
    setIsLoading(true);
    try {
      await fetch_post("/game/player/add", {
        "name": name,
        "hashed_password": await digestMessage(password),
      }).then(async response =>
        setRegisterMessage(response.ok ? <Label className="m-4">Spieler mit Name {(await response.json()).name} erfolgreich registriert</Label> : <Label className="text-red-500 m-4">Fehler: {response.text()}</Label>)
      );
    }
    finally {
      setIsLoading(false);
    }
  }

  return (
    <>
      <Header />
      <Card className="">
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
                      <Input type="text" placeholder="Rufname" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="password"
                render={({ field }) => (
                  <FormItem className="my-2">
                    <FormLabel>Passwort</FormLabel>
                    <FormControl>
                      <Input type="password" placeholder="Passwort" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <FormField
                control={form.control}
                name="confirm"
                render={({ field }) => (
                  <FormItem className="my-2">
                    <FormLabel>Passwort bestätigen</FormLabel>
                    <FormControl>
                      <Input type="password" placeholder="Passwort" {...field} />
                    </FormControl>
                    <FormMessage />
                  </FormItem>
                )}
              />
              <div className="flex-col">
                <Button type="submit">Bestätigen</Button>
                {isLoading && <Label className="m-4">Wird verarbeitet...</Label>}
                {registerMessage}
              </div>
            </form>
          </Form>
        </CardContent>
      </Card>
    </>
  );
}
