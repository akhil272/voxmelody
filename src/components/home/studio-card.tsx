import type { LucideIcon } from "lucide-react";
import type { ReactNode } from "react";
import { Waveform } from "@/components/brand/waveform";
import { Card } from "@/components/ui/card";
import { cn } from "@/lib/utils";

interface StudioCardProps {
  title: string;
  description: string;
  icon: LucideIcon;
  tone: "voice" | "music";
  children: ReactNode;
}

export function StudioCard({ title, description, icon: Icon, tone, children }: StudioCardProps) {
  return (
    <Card>
      <div className="flex items-center justify-between gap-4">
        <div>
          <h2 className="flex items-center gap-3 font-semibold">
            <Icon className={cn("size-5", tone === "voice" ? "text-primary" : "text-secondary")} />
            {title}
          </h2>
          <p className="mt-1 text-sm text-muted-foreground">{description}</p>
        </div>
        <Waveform tone={tone} />
      </div>
      {children}
    </Card>
  );
}
