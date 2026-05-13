import { createFileRoute, Link } from '@tanstack/react-router';

export const Route = createFileRoute('/')({
  component: Home,
});

function Home() {
  return (
    <>
      <Link to="/game/multiple-choice" className="border border-foreground p-2">
        Go to /game/multiple-choice
      </Link>
    </>
  );
}
