import React, { useEffect } from 'react';
import { useLocation, Link } from 'react-router-dom';

// Import MDX files globally
import SetupDocs from '../docs/setup.mdx';
import ModerationDocs from '../docs/moderation.mdx';
import CaseMgmtDocs from '../docs/case-management.mdx';
import ReversalsDocs from '../docs/reversals.mdx';
import UtilityDocs from '../docs/utility.mdx';

export default function Docs() {
    const { hash } = useLocation();

    useEffect(() => {
        if (hash) {
            const element = document.getElementById(hash.replace('#', ''));
            if (element) {
                setTimeout(() => element.scrollIntoView({ behavior: 'smooth' }), 100);
            }
        } else {
            window.scrollTo(0, 0);
        }
    }, [hash]);

    const sections = [
        { id: 'setup', title: 'Setup & Config', Component: SetupDocs },
        { id: 'core-mod', title: 'Core Moderation', Component: ModerationDocs },
        { id: 'case-mgmt', title: 'Case Management', Component: CaseMgmtDocs },
        { id: 'reversals', title: 'Reversals', Component: ReversalsDocs },
        { id: 'utility', title: 'Utility', Component: UtilityDocs }
    ];

    return (
        <div className="w-full flex">
            {/* Sidebar Navigation */}
            <aside className="hidden lg:block w-64 shrink-0 pt-32 pb-24 border-r border-[#1A1A1A] sticky top-0 h-screen overflow-y-auto">
                <div className="px-6 flex flex-col gap-6">
                    <div className="flex flex-col gap-2">
                        <Link to="/" className="font-mono text-sm text-background/60 hover:text-accent transition-colors">Home</Link>
                        <Link to="/docs" className="font-mono text-sm text-accent transition-colors font-bold">Commands</Link>
                        <Link to="/docs/contributing" className="font-mono text-sm text-background/60 hover:text-accent transition-colors">Contributing</Link>
                    </div>

                    <div className="h-px bg-[#1A1A1A]"></div>

                    <div className="flex flex-col gap-3">
                        <span className="font-mono text-xs text-background/40 uppercase tracking-widest">On this page</span>
                        {sections.map(sec => (
                            <a
                                key={sec.id}
                                href={`#${sec.id}`}
                                className="font-mono text-xs text-background/60 hover:text-background transition-colors"
                            >
                                {sec.title}
                            </a>
                        ))}
                    </div>
                </div>
            </aside>

            {/* Main Content */}
            <div className="flex-1 pt-32 pb-24 px-6 lg:px-16 max-w-4xl mx-auto">
                <h1 className="font-sans text-4xl font-bold text-background mb-4">Command Reference</h1>
                <p className="font-mono text-background/60 leading-relaxed mb-12">
                    Autumn is designed as a prefix-first bot (`!`), but natively supports Discord Slash Commands for all inputs. The documentation below covers standard usage patterns.
                </p>

                <div className="flex flex-col gap-16">
                    {sections.map(sec => (
                        <section key={sec.id} id={sec.id} className="scroll-mt-32">
                            <div className="prose prose-invert prose-pre:bg-[#0A0A0A] prose-pre:border prose-pre:border-[#333] prose-headings:font-sans prose-headings:text-background prose-a:text-accent max-w-none">
                                <sec.Component />
                            </div>
                        </section>
                    ))}
                </div>
            </div>

            {/* Balancing Spacer for Global Centering */}
            <div className="hidden lg:block w-64 shrink-0"></div>
        </div>
    );
}
