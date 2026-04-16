import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectLabel,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";

export default function Selector(props: { onValueChange: (_: number) => void }) {
  return (
    <Select onValueChange={x => props.onValueChange(parseInt(x))}>
      <SelectTrigger>
        <SelectValue placeholder="Variante" />
      </SelectTrigger>
      <SelectContent>
        <SelectGroup>
          <SelectLabel>Variante</SelectLabel>
          <SelectItem value="1">1</SelectItem>
          <SelectItem value="2">2</SelectItem>
          <SelectItem value="3">3</SelectItem>
          <SelectItem value="4">4</SelectItem>
        </SelectGroup>
      </SelectContent>
    </Select>
  );
}
