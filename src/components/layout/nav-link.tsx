"use client";

import Link from "next/link";
import { usePathname } from "next/navigation";
import type { ReactNode } from "react";
import { cn, isActiveRoute } from "@/lib/utils";

interface NavLinkProps {
  href: string;
  children: ReactNode;
}

export function NavLink({ href, children }: NavLinkProps) {
  const active = isActiveRoute(usePathname(), href);

  return (
    <Link
      href={href}
      draggable={false}
      aria-current={active ? "page" : undefined}
      className={cn(
        "flex h-10 items-center gap-4 rounded-md px-3 text-sm transition-colors outline-none hover:bg-accent focus-visible:ring-3 focus-visible:ring-ring/40 [&_svg]:size-5",
        active && "bg-primary text-primary-foreground hover:bg-primary",
      )}
    >
      {children}
    </Link>
  );
}
