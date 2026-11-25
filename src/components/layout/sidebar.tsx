import { NavLink } from "@/components/layout/nav-link";
import { COPY, FOOTER_NAV, MAIN_NAV } from "@/lib/constants";

export function Sidebar() {
  return (
    <nav aria-label={COPY.navigation} className="flex w-44 shrink-0 flex-col justify-between border-r bg-card/60 p-3 pt-6">
      <div className="flex flex-col gap-1.5">
        {MAIN_NAV.map(({ href, label, icon: Icon }) => (
          <NavLink key={href} href={href}>
            <Icon />
            {label}
          </NavLink>
        ))}
      </div>
      <div className="border-t pt-3">
        {FOOTER_NAV.map(({ href, label, icon: Icon }) => (
          <NavLink key={href} href={href}>
            <Icon />
            {label}
          </NavLink>
        ))}
      </div>
    </nav>
  );
}
