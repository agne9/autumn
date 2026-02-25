import React from 'react';
import DocsLayout from '../components/DocsLayout';

import SetupDocs from '../docs/setup.mdx';
import ModerationDocs from '../docs/moderation.mdx';
import CaseMgmtDocs from '../docs/case-management.mdx';
import ReversalsDocs from '../docs/reversals.mdx';
import UtilityDocs from '../docs/utility.mdx';

const docSections = [
    { id: 'setup', title: 'Setup & Config', Component: SetupDocs },
    { id: 'core-mod', title: 'Core Moderation', Component: ModerationDocs },
    { id: 'case-mgmt', title: 'Case Management', Component: CaseMgmtDocs },
    { id: 'reversals', title: 'Reversals', Component: ReversalsDocs },
    { id: 'utility', title: 'Utility', Component: UtilityDocs },
];

export default function Docs() {
    return (
        <DocsLayout activePage="commands" headingLevels="h1, h2, h3" groupByH1>
            <h1 className="font-sans text-4xl font-bold text-background mb-4">Command Reference</h1>
            <p className="font-mono text-background/60 leading-relaxed mb-12">
                Autumn is designed as a prefix-first bot (`!`), but natively supports Discord Slash Commands for all inputs. The documentation below covers standard usage patterns.
            </p>

            <div className="flex flex-col gap-16">
                {docSections.map(sec => (
                    <section key={sec.id} id={sec.id} className="scroll-mt-32">
                        <div className="prose prose-invert prose-pre:bg-[#0A0A0A] prose-pre:border prose-pre:border-[#333] prose-headings:font-sans prose-headings:text-background prose-a:text-accent max-w-none">
                            <sec.Component />
                        </div>
                    </section>
                ))}
            </div>
        </DocsLayout>
    );
}
