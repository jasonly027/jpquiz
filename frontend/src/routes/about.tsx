import { SideBar } from '@/components/layouts/SideBar';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/about')({
  component: RouteComponent,
});

function RouteComponent() {
  return (
    <>
      <SideBar />
    </>
  );
}
