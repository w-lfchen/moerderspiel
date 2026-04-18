import {
  Accordion, AccordionContent, AccordionItem, AccordionTrigger,
} from "@/components/ui/accordion";


export default function Header() {
  return (
    <div className="flex-col items-center justify-center text-3xl">
      <h1>Mörderspiel</h1>
      <Accordion type="single" collapsible className="my-4 w-full">
        <AccordionItem value="item-1">
          <AccordionTrigger>Regeln</AccordionTrigger>
          <AccordionContent>
            <p>Alle Mitspielenden erhalten zum Start zufällig drei Mordaufträge; z.B. "Erledige Bertram".
              Man zieht also los und sucht Bertram, den man erst mal erkennen und dann durch Übergabe eines beliebigen Gegenstandes umbringen soll.
              Es muss schon eine freiwillige, unbeobachtete Übergabe sein, andernfalls gilt der Mord nicht.
              Ziel ist es, das Opfer in eine Situation zu bringen, bei der er nicht aufs Spiel achtet und einfach einen Gegenstand annimmt.
              Zuwerfen gilt nicht, aufzwängen auch nicht.</p>
            <br />
            <p>Hat man jemanden erfolgreich ermordet, muss man ihm dies sofort sagen, auf der Webite eintragen und erhält vom Opfer dessen Mordauftrag dort, mit dem man weiter spielen kann.
              Jeder Mordauftrag symbolisiert ein Leben, wenn man drei mal umgebracht wurde, scheidet man aus.</p>
            <br />
            <p>Die Person mit den meisten abgeschlossenen Aufträgen gewinnt.</p>
            <br />
            <p>Das Spiel beginnt am Samstag um 14:00 Uhr und endet am Sonntag um 12:00 Uhr.</p>
          </AccordionContent>
        </AccordionItem>
      </Accordion>
    </div>
  );
}
