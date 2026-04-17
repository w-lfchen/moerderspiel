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
import { digestMessage, fetch_post } from "@/lib/utils";
import type { PlayerState } from "@/lib/types";
import { PlayerStatePage } from "@/components/player_state";

export default function Running() {
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
    await fetch_post("/game/player/status", {
      "name": name,
      "hashed_password": await digestMessage(password),
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

  return (
    <>
      <Header />
      {(playerState === null) ?
        <>
          <Card className="">
            <CardHeader>
              <CardTitle>Status abfragen</CardTitle>
            </CardHeader>
            <CardContent>
              <Form {...form}>
                <form onSubmit={form.handleSubmit(onSubmit)}>
                  <FormField
                    control={form.control}
                    name="name"
                    render={({ field }) => (
                      <FormItem className="my-2">
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
                  <Button className="my-2" type="submit">Status anfragen</Button>
                  <br />
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
}
