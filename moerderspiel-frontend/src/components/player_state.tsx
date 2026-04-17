import { useState } from "react";

import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger, } from '@/components/ui/alert-dialog';
import { Button } from "@/components/ui/button";
import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Label } from "@/components/ui/label";

import type { Order, PlayerState } from "@/lib/types";
import { fetch_post } from "@/lib/utils";

async function complete_order(order_id: number, errorConsumer: (_: string) => void, orderConsumer: (_: Order) => void, increaseKills: () => void) {
  await fetch_post(`/game/order/${order_id}`).then(async response => {
    if (response.ok) {
      const { new_order } = await response.json();
      errorConsumer(null);
      orderConsumer(new_order);
      increaseKills();
    } else {
      errorConsumer(await response.text());
    }
  });
}

export function PlayerStatePage(props: { state: PlayerState }) {
  const [killError, setKillError] = useState(null);
  const state = props.state;
  // might be a better way to do this
  const [kills, setKills] = useState(state.num_kills);
  const [orders, setOrders] = useState(state.orders.sort((x, y) => x.circle - y.circle));
  // TODO: there must be a better way to do this
  const orderConsumer = (o: Order) => setOrders(orders.map(x => x.circle == o.circle ? o : x));
  return (
    <>
      <Card className="my-2">
        <CardHeader>
          <CardTitle>Status von {state.name}</CardTitle>
        </CardHeader>
        <CardContent>
          <div className="grid grid-cols-2">
            <p>Verbleibende Leben: {orders.length}</p>
            <br />
            <p>Registrierte Morde: {kills}</p>
          </div>
        </CardContent>
      </Card>
      <div className="grid grid-cols-1">
        <Label className="text-2xl">Aufträge:</Label>
        {orders.map(order =>
          order.target_id === state.id ?
            <Label className="m-4">Kein Auftrag für Kreis {order.circle} verfügbar</Label>
            :
            <AlertDialog>
              <AlertDialogTrigger asChild>
                <Button className="my-1">
                  <Label>{order.circle}: {order.target_name}</Label>
                </Button>
              </AlertDialogTrigger>
              <AlertDialogContent>
                <AlertDialogHeader>
                  <AlertDialogTitle>Bestätigung</AlertDialogTitle>
                  <AlertDialogDescription>
                    Bitte bestätige, dass du {order.target_name} umgebracht hast.
                  </AlertDialogDescription>
                </AlertDialogHeader>
                <AlertDialogFooter>
                  <AlertDialogCancel>Abbrechen</AlertDialogCancel>
                  <AlertDialogAction
                    onClick={() => complete_order(order.id, setKillError, orderConsumer, () => setKills(kills + 1))}
                  >Bestätigen</AlertDialogAction>
                </AlertDialogFooter>
              </AlertDialogContent>
            </AlertDialog>
        )}
      </div>
      {killError && <Label> Fehler beim Verarbeiten: {killError}</Label>}
    </>
  );

}
