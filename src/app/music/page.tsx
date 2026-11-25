import { MusicIcon } from "lucide-react";
import { EmptyPage } from "@/components/shared/empty-page";
import { COPY } from "@/lib/constants";

export default function MusicPage() {
  return <EmptyPage title={COPY.nav.music} {...COPY.pages.music} icon={MusicIcon} />;
}
