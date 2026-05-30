import peepoNotes from '../../peeponotes.webp';
import { useTheme } from '../hooks/ThemeProvider';
import { Button } from '../ui/button';
import {
  Moon02Icon,
  Settings03Icon,
  Sun03Icon,
} from '@hugeicons/core-free-icons';
import { HugeiconsIcon, type IconSvgElement } from '@hugeicons/react';
import { Link } from '@tanstack/react-router';

export function SideBar() {
  return (
    <aside className="w-full border-foreground/15 bg-card text-card-foreground max-sm:border-b sm:w-60 sm:border-r">
      <div className="sticky top-0 flex flex-col gap-6 p-6 sm:h-screen">
        <div className="text-center font-game-title text-xl tracking-tight">
          StudyWard
        </div>

        <nav className="flex flex-col gap-1.5 rounded-lg bg-background p-3 ring ring-foreground/15 **:data-[status=active]:font-semibold **:data-[status=active]:text-primary [&_a]:hover:underline">
          <Link to="/">Home</Link>

          <Link to="/about">About</Link>

          <div>
            <div className="font-semibold">Modes</div>

            <div className="ml-3 flex flex-col">
              <Link to="/game/multiple-choice">Multiple Choice</Link>
              <Link to="/game/free-response">Free Response</Link>
            </div>
          </div>
        </nav>

        <div className="mt-auto flex justify-end">
          <img
            src={peepoNotes}
            alt="peepoNotes"
            className="mr-auto size-10 animate-spin"
          />
          <ThemeToggleButton />
        </div>
      </div>
    </aside>
  );
}

function ThemeToggleButton() {
  const { theme, setTheme } = useTheme();

  const onClick = () => {
    const next = theme !== 'dark' ? 'dark' : 'light';
    setTheme(next);
  };

  let icon: IconSvgElement;
  switch (theme) {
    case 'system':
      icon = Settings03Icon;
      break;
    case 'dark':
      icon = Moon02Icon;
      break;
    case 'light':
      icon = Sun03Icon;
      break;
  }

  return (
    <Button onClick={onClick} variant="outline" size="icon-sm">
      <HugeiconsIcon icon={icon} color="currentColor" strokeWidth={1.5} />
    </Button>
  );
}
