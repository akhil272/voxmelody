import { SettingsIcon } from "lucide-react";
import { EmptyPage } from "@/components/shared/empty-page";
import { COPY } from "@/lib/constants";

export default function SettingsPage() {
  return <EmptyPage title={COPY.nav.settings} {...COPY.pages.settings} icon={SettingsIcon} />;
}
