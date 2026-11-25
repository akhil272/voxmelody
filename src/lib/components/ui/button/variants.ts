import { tv, type VariantProps } from "tailwind-variants";

export const buttonVariants = tv({
  base: "inline-flex shrink-0 items-center justify-center rounded-md border border-transparent font-medium whitespace-nowrap transition-colors outline-none select-none focus-visible:ring-3 focus-visible:ring-ring/40 disabled:pointer-events-none disabled:opacity-60 [&_svg]:pointer-events-none [&_svg]:size-4 [&_svg]:shrink-0",
  variants: {
    variant: {
      default: "bg-primary text-primary-foreground hover:bg-primary-hover",
      secondary: "bg-secondary text-secondary-foreground hover:bg-secondary-hover focus-visible:ring-secondary/40",
      outline: "border-border bg-card text-foreground hover:bg-accent hover:text-accent-foreground",
      ghost: "text-foreground hover:bg-muted",
      close: "text-foreground hover:bg-destructive hover:text-primary-foreground",
      link: "text-secondary hover:text-secondary-hover",
    },
    size: {
      default: "h-10 gap-2 px-4 text-sm",
      lg: "h-11 gap-3 px-5 text-sm",
      inline: "gap-1.5 text-xs",
      window: "h-full w-12 rounded-none",
    },
  },
  defaultVariants: { variant: "default", size: "default" },
});

export type ButtonVariants = VariantProps<typeof buttonVariants>;
