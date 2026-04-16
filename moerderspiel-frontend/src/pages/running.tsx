import { useState } from "react";
import { zodResolver } from '@hookform/resolvers/zod';
import { useForm } from 'react-hook-form';
import { z } from 'zod';

import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Form, FormControl, FormField, FormItem, FormLabel, FormMessage } from '@/components/ui/form';
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";

import Header from "@/components/header";
import Selector from "@/components/selector";
import { fetch_post } from "@/lib/utils";
import type { PlayerState } from "@/lib/types";
import { PlayerStatePage } from "@/components/player_state";

export default function Running() {
  const [value, setValue] = useState(null);
  const [statusError, setStatusError] = useState(null);
  const [playerState, setPlayerState] = useState(null);


  // slop doc example code
  const formSchema = z.object({
    name: z.string().min(1, {
      message: "grr empty string",
    }),
    password: z.string(),
  });

  const form = useForm<z.infer<typeof formSchema>>({
    resolver: zodResolver(formSchema),
    defaultValues: {
      name: "",
      password: "",
    },
  });

  async function onSubmit({ name, password }: z.infer<typeof formSchema>) {
    // TODO: hash password before sending
    if (value === null) {
      setStatusError(<Label className="text-red-500">Bitte wähle eine Variante aus.</Label>);
      return;
    }
    // TODO: get requests can't have a body, fix this in both front- and backend
    await fetch_post("/game/player/status", {
      "name": name,
      "variant": value,
      "hashed_password": password,
    }).then(async response => {
      if (response.ok) {
        // TODO: display card
        setStatusError(null);
        const state: PlayerState = await response.json();
        setPlayerState(state);
      } else {
        setStatusError(<Label className="text-red-500">Fehler: {await response.text()}</Label>);
      }
    });
  }

  // TODO: fix grid layout
  return (
    <>
      <Header />
      {(playerState === null) ?
        <>
          <Card>
            <CardHeader>
              <CardTitle>Status abfragen</CardTitle>
            </CardHeader>
            <CardContent>
              <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)}>
                  <div className="flex flex-row gap-2 items-baseline">
                    <FormItem>
                      <FormLabel>Variante</FormLabel>
                      <FormControl>
                        <Selector onValueChange={setValue} />
                      </FormControl>
                      <FormMessage />
                    </FormItem>
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
                  </div>
                  <FormField
                    control={form.control}
                    name="password"
                    render={({ field }) => (
                      <FormItem>
                        <FormLabel>Passwort</FormLabel>
                        <FormControl>
                          <Input type="password" placeholder="Passwort" {...field} />
                        </FormControl>
                        <FormMessage />
                      </FormItem>
                    )}
                  />
                  <Button type="submit">Status anfragen</Button>
                  {statusError}
                </form>
              </Form>
            </CardContent>
          </Card>
        </>
        :
        <>
          <PlayerStatePage state={playerState} />
        </>
      }
    </>
  );
  // TODO: finish
}
