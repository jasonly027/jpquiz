import { GenericLayout } from '@/components/layouts/GenericLayout';

export function NotFound() {
  return (
    <GenericLayout className="m-6 flex grow items-center">
      <span className="text-lg font-semibold">Page not found</span>
    </GenericLayout>
  );
}
