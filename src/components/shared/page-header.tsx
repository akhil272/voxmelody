interface PageHeaderProps {
  title: string;
}

export function PageHeader({ title }: PageHeaderProps) {
  return <h1 className="text-2xl font-semibold">{title}</h1>;
}
