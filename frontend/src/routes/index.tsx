import { createFileRoute, Link } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Home,
});

function Home() {
  return (
    <>
      <Link to="/game/multi_choice" className="border border-foreground p-2">
        Go to /game/multi_choice
      </Link>
    </>
  );
}
