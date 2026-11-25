import { cn } from "@/lib/utils";

interface WordmarkProps {
  className?: string;
}

export function Wordmark({ className }: WordmarkProps) {
  return (
    <span className={cn("font-heading font-bold tracking-tight", className)}>
      Vox
      <span className="bg-linear-to-r from-secondary to-primary to-35% bg-clip-text text-transparent">Melody</span>
    </span>
  );
}
