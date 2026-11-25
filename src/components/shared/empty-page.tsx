import type { LucideIcon } from "lucide-react";
import { PageHeader } from "@/components/shared/page-header";
import { Card } from "@/components/ui/card";

interface EmptyPageProps {
  title: string;
  empty: string;
  description: string;
  icon: LucideIcon;
}

export function EmptyPage({ title, empty, description, icon: Icon }: EmptyPageProps) {
  return (
    <div className="flex h-full flex-col gap-6 p-8">
      <PageHeader title={title} />
      <Card className="flex-1 items-center justify-center text-center">
        <span className="grid size-14 place-items-center rounded-full bg-accent text-primary">
          <Icon className="size-6" />
        </span>
        <div>
          <h2 className="text-lg font-semibold">{empty}</h2>
          <p className="mt-1 text-sm text-muted-foreground">{description}</p>
        </div>
      </Card>
    </div>
  );
}
