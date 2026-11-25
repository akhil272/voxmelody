import type { ComponentProps } from "react";
import { cn } from "@/lib/utils";

function Card({ className, ...props }: ComponentProps<"section">) {
  return (
    <section
      data-slot="card"
      className={cn("flex flex-col gap-3 rounded-xl border bg-card p-4 text-card-foreground shadow-xs", className)}
      {...props}
    />
  );
}

export { Card };
