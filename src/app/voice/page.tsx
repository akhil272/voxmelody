import { MicIcon } from "lucide-react";
import { EmptyPage } from "@/components/shared/empty-page";
import { COPY } from "@/lib/constants";

export default function VoicePage() {
  return <EmptyPage title={COPY.nav.voice} {...COPY.pages.voice} icon={MicIcon} />;
}
