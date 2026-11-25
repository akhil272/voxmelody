import { Logo } from "@/components/brand/logo";
import { Wordmark } from "@/components/brand/wordmark";
import { PageHeader } from "@/components/shared/page-header";
import { Card } from "@/components/ui/card";
import { COPY } from "@/lib/constants";

export default function AboutPage() {
  return (
    <div className="flex h-full flex-col gap-6 p-8">
      <PageHeader title={COPY.nav.about} />
      <Card className="flex-1 items-center justify-center text-center">
        <Logo decorative className="h-16" />
        <Wordmark className="text-3xl" />
        <p className="text-muted-foreground">{COPY.tagline}</p>
        <p className="text-sm">{COPY.about.privacy}</p>
        <p className="text-sm text-muted-foreground">{COPY.about.licence}</p>
      </Card>
    </div>
  );
}
