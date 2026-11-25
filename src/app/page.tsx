import Link from "next/link";
import { ArrowRightIcon, AudioLinesIcon, InfoIcon, MicIcon, MusicIcon, PlayIcon } from "lucide-react";
import { HeroWave } from "@/components/brand/hero-wave";
import { Wordmark } from "@/components/brand/wordmark";
import { StudioCard } from "@/components/home/studio-card";
import { ModelSelect } from "@/components/shared/model-select";
import { Button } from "@/components/ui/button";
import { buttonVariants } from "@/components/ui/button-variants";
import { Textarea } from "@/components/ui/textarea";
import { COPY, ROUTES } from "@/lib/constants";

export default function HomePage() {
  const copy = COPY.home;

  return (
    <div className="flex min-h-full flex-col gap-4 px-5 pb-5">
      <section className="relative flex min-h-64 flex-1 items-center overflow-hidden px-4">
        <HeroWave />
        <div className="relative">
          <h1 className="text-2xl font-semibold">
            {copy.welcome}
            <Wordmark className="block text-[42px] leading-tight" />
          </h1>
          <p className="mt-1 max-w-72 text-muted-foreground">{copy.description}</p>
          <Link href={ROUTES.voice} className={buttonVariants({ size: "lg", className: "mt-5" })}>
            {copy.start}
            <ArrowRightIcon />
          </Link>
        </div>
      </section>

      <div className="grid grid-cols-2 gap-4">
        <StudioCard title={copy.voiceTitle} description={copy.voiceDescription} icon={MicIcon} tone="voice">
          <Textarea rows={1} aria-label={copy.voiceTitle} placeholder={copy.voicePlaceholder} disabled />
          <div className="flex gap-2">
            <ModelSelect label={copy.voiceModel} model={copy.voiceEngine} icon={<AudioLinesIcon />} disabled />
            <Button variant="outline" disabled>
              <PlayIcon />
              {copy.preview}
            </Button>
            <Button className="flex-1" disabled>
              {copy.generate}
            </Button>
          </div>
        </StudioCard>

        <StudioCard title={copy.musicTitle} description={copy.musicDescription} icon={MusicIcon} tone="music">
          <ModelSelect label={copy.musicModel} model={copy.musicEngine} icon={<MusicIcon />} disabled />
          <Button variant="secondary" className="mt-auto w-full" disabled>
            {copy.generateMusic}
          </Button>
        </StudioCard>
      </div>

      <aside className="flex items-center justify-between rounded-lg border bg-muted px-4 py-2.5 text-xs">
        <span className="flex items-center gap-2.5">
          <InfoIcon className="size-4 text-primary" />
          {copy.privacy}
        </span>
        <Link href={ROUTES.about} className={buttonVariants({ variant: "link", size: "inline" })}>
          {copy.learnMore}
          <ArrowRightIcon />
        </Link>
      </aside>
    </div>
  );
}
