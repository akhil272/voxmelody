import { FolderIcon } from "lucide-react";
import { EmptyPage } from "@/components/shared/empty-page";
import { COPY } from "@/lib/constants";

export default function LibraryPage() {
  return <EmptyPage title={COPY.nav.library} {...COPY.pages.library} icon={FolderIcon} />;
}
