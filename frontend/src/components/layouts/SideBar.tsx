import { Link } from '@tanstack/react-router';

export function SideBar() {
  return (
    <aside className="flex w-full flex-col gap-6 border-foreground/15 bg-card p-6 text-card-foreground max-sm:border-b sm:min-h-screen sm:w-60 sm:border-r">
      <div className="text-center font-game-title text-xl tracking-tight">
        StudyWard
      </div>

      <nav className="flex flex-col gap-2 rounded-lg bg-background p-3 [&_[data-status=active]]:font-semibold [&_[data-status=active]]:text-primary [&_a]:hover:underline">
        <Link to="/">Home</Link>

        <Link to="/about">About</Link>

        <div>
          <div className="font-semibold">Modes</div>

          <div className="ml-3 flex flex-col gap-2">
            <Link to="/game/multiple-choice">Multiple Choice</Link>
          </div>
        </div>
      </nav>
    </aside>
  );
}
