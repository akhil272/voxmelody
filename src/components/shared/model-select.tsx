import type { ReactNode } from "react";
import { Select, SelectContent, SelectItem, SelectTrigger, SelectValue } from "@/components/ui/select";

interface ModelSelectProps {
  label: string;
  model: string;
  icon: ReactNode;
  disabled?: boolean;
}

export function ModelSelect({ label, model, icon, disabled }: ModelSelectProps) {
  return (
    <Select defaultValue={model} disabled={disabled}>
      <SelectTrigger aria-label={label}>
        {icon}
        <SelectValue />
      </SelectTrigger>
      <SelectContent>
        <SelectItem value={model}>{model}</SelectItem>
      </SelectContent>
    </Select>
  );
}
