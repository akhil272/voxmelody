/** The developer's Buy Me a Coffee page, shared with Akhil's other apps. */
export const SUPPORT_URL = "https://buymeacoffee.com/builtbyakhil";

export const SOCIAL_LINKS = [
  { label: "Instagram", href: "https://www.instagram.com/builtby.akhil", icon: "instagram" },
  { label: "X", href: "https://x.com/builtbyakhil", icon: "x" },
  { label: "LinkedIn", href: "https://www.linkedin.com/in/akhil272/", icon: "linkedin" },
] as const;

export type SocialIcon = (typeof SOCIAL_LINKS)[number]["icon"];

export const ABOUT_PHOTO = "/about/akhil.webp";
