import { SideBar } from '@/components/layouts/SideBar';
import { createFileRoute } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Home,
  ssr: false,
});

function Home() {
  return (
    <>
      <SideBar />
    </>
  );
}
