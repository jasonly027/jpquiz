import { Title } from '@/components/Title';
import { GenericLayout } from '@/components/layouts/GenericLayout';
import { Card, CardContent } from '@/components/ui/card';
import { cn } from '@/lib/utils';
import { createFileRoute } from '@tanstack/react-router';
import type { ComponentProps } from 'react';

export const Route = createFileRoute('/about')({
  component: RouteComponent,
});

const JASON_EMAIL = 'jasonly.code@gmail.com';

function RouteComponent() {
  return (
    <GenericLayout className="items-center">
      <Title>About</Title>

      <div className="mx-4">
        <Card className="w-full max-w-3xl text-base">
          <CardContent>
            <article className="[&_p]:mb-4">
              <section>
                <p>
                  StudyWard is a Japanese language studying tool. There are a
                  variety of game modes each with their own settings meant to
                  test a user&apos;s comprehension. The site is best used as a
                  supplement to your other learning materials to gauge how well
                  you&apos;ve absorbed the information.
                </p>

                <p>
                  StudyWard is primarily maintained by{' '}
                  <Anchor href={`mailto:${JASON_EMAIL}`}>Jason</Anchor> with
                  ideas and consultation from Legs.
                </p>
              </section>

              <section>
                <h2 className="mb-1 text-xl">Sources</h2>

                <h3 className="text-lg">
                  <Anchor href="http://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project">
                    JMDict
                  </Anchor>
                </h3>
                <p>
                  Used for translations, definitions, part-of-speech
                  categorization, and as a general source of truth. A huge
                  thanks to{' '}
                  <Anchor href="https://github.com/scriptin/jmdict-simplified">
                    Dmitry Shpika
                  </Anchor>{' '}
                  for simplifying its usage.
                </p>

                <h3 className="text-lg">
                  <Anchor href="https://www.tanos.co.uk/jlpt/">
                    Jonathan Waller&apos;s JLPT Vocabulary Lists
                  </Anchor>
                </h3>
                <p>
                  Used for creating word pairings and JLPT NLevel
                  categorization. Thank you to{' '}
                  <Anchor href="https://github.com/Bluskyo/JLPT_Vocabulary">
                    Isak Mikalsen
                  </Anchor>{' '}
                  for reformatting.
                </p>
              </section>
            </article>
          </CardContent>
        </Card>
      </div>
    </GenericLayout>
  );
}

function Anchor({ className, ...props }: ComponentProps<'a'>) {
  return <a className={cn('text-primary underline', className)} {...props} />;
}
