import React from 'react';
import DocsLayout from '../components/DocsLayout';
import ContributingDocs from '../docs/contributing.mdx';

export default function Contributing() {
    return (
        <DocsLayout activePage="contributing" headingLevels="h2, h3">
            <div className="prose prose-invert prose-headings:scroll-mt-32 prose-pre:bg-[#0A0A0A] prose-pre:border prose-pre:border-[#333] prose-headings:font-sans prose-headings:text-background prose-a:text-accent prose-p:font-mono prose-p:text-background/80 prose-li:font-mono prose-li:text-background/80 max-w-none">
                <ContributingDocs />
            </div>
        </DocsLayout>
    );
}
