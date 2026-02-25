import React, { useEffect, useState, useRef, useCallback } from 'react';
import { useLocation, Link } from 'react-router-dom';

function slugify(text) {
    return text
        .toLowerCase()
        .replace(/[^a-z0-9]+/g, '-')
        .replace(/(^-|-$)/g, '');
}

export default function DocsLayout({ activePage, headingLevels = 'h1, h2, h3', groupByH1 = false, children }) {
    const { hash } = useLocation();
    const [activeId, setActiveId] = useState('');
    const [tocEntries, setTocEntries] = useState([]);
    const [mobileNavOpen, setMobileNavOpen] = useState(false);
    const contentRef = useRef(null);

    // After content renders, scan for headings and build TOC
    useEffect(() => {
        const timer = setTimeout(() => {
            if (!contentRef.current) return;

            const headings = contentRef.current.querySelectorAll(headingLevels);
            const entries = [];

            headings.forEach((heading) => {
                const text = heading.textContent?.trim();
                if (!text) return;

                if (!heading.id) {
                    heading.id = slugify(text);
                }

                const level = heading.tagName === 'H1' ? 1 : heading.tagName === 'H2' ? 2 : 3;
                entries.push({ id: heading.id, title: text, level });
            });

            setTocEntries(entries);
        }, 150);

        return () => clearTimeout(timer);
    }, [headingLevels]);

    // Scroll-spy: observe all heading elements
    useEffect(() => {
        if (tocEntries.length === 0) return;

        const observer = new IntersectionObserver(
            (entries) => {
                const visible = entries
                    .filter((entry) => entry.isIntersecting)
                    .sort((a, b) => a.boundingClientRect.top - b.boundingClientRect.top);

                if (visible.length > 0) {
                    setActiveId(visible[0].target.id);
                }
            },
            { rootMargin: '-20% 0px -60% 0px', threshold: 0 }
        );

        tocEntries.forEach(({ id }) => {
            const el = document.getElementById(id);
            if (el) observer.observe(el);
        });

        return () => observer.disconnect();
    }, [tocEntries]);

    // Find which top-level group (h1) the active heading belongs to
    const activeSectionId = (() => {
        if (!activeId || !groupByH1) return '';
        const idx = tocEntries.findIndex((e) => e.id === activeId);
        if (idx < 0) return '';
        for (let i = idx; i >= 0; i--) {
            if (tocEntries[i].level === 1) return tocEntries[i].id;
        }
        return '';
    })();

    const scrollIntoViewOffset = useCallback((element) => {
        const targetTop = window.scrollY + element.getBoundingClientRect().top;
        const offset = window.innerHeight * 0.35;
        window.scrollTo({ top: Math.max(0, targetTop - offset), behavior: 'auto' });
    }, []);

    const jumpToHeading = useCallback((event, id) => {
        event.preventDefault();
        const element = document.getElementById(id);
        if (!element) return;
        scrollIntoViewOffset(element);
        window.history.replaceState(null, '', `#${id}`);
    }, [scrollIntoViewOffset]);

    useEffect(() => {
        if (hash) {
            const targetId = hash.replace('#', '');
            const element = document.getElementById(targetId);
            if (element) {
                setTimeout(() => scrollIntoViewOffset(element), 100);
            }
        } else {
            window.scrollTo(0, 0);
        }
    }, [hash]);

    // Group TOC: h1 entries are parents, h2/h3 are children (when groupByH1 is set)
    const tocGroups = (() => {
        if (!groupByH1) return null;

        const groups = [];
        let current = null;

        for (const entry of tocEntries) {
            if (entry.level === 1) {
                current = { ...entry, children: [] };
                groups.push(current);
            } else if (current) {
                current.children.push(entry);
            } else {
                groups.push({ ...entry, children: [] });
            }
        }

        return groups;
    })();

    const navLinks = [
        { to: '/', label: 'Home' },
        { to: '/docs', label: 'Commands' },
        { to: '/docs/contributing', label: 'Contributing' },
    ];

    const renderTocDesktop = () => {
        if (groupByH1 && tocGroups) {
            return tocGroups.map(group => (
                <div key={group.id} className="flex flex-col gap-1">
                    <a
                        href={`#${group.id}`}
                        onClick={(event) => jumpToHeading(event, group.id)}
                        className={`font-mono text-[11px] font-semibold transition-colors ${
                            activeSectionId === group.id || activeId === group.id
                                ? 'text-accent'
                                : 'text-background/50 hover:text-background'
                        }`}
                    >
                        {group.title}
                    </a>
                    {group.children.length > 0 && (
                        <div className="ml-2 flex flex-col gap-0.5 border-l border-[#1A1A1A] pl-2">
                            {group.children.map(child => (
                                <a
                                    key={child.id}
                                    href={`#${child.id}`}
                                    onClick={(event) => jumpToHeading(event, child.id)}
                                    className={`font-mono text-[11px] transition-colors ${
                                        activeId === child.id
                                            ? 'text-accent'
                                            : 'text-background/40 hover:text-accent'
                                    }`}
                                >
                                    {child.title}
                                </a>
                            ))}
                        </div>
                    )}
                </div>
            ));
        }

        return tocEntries.map(entry => (
            <a
                key={entry.id}
                href={`#${entry.id}`}
                onClick={(event) => jumpToHeading(event, entry.id)}
                className={`font-mono text-[11px] transition-colors ${
                    entry.level === 3 ? 'ml-3' : ''
                } ${
                    activeId === entry.id
                        ? 'text-accent font-semibold'
                        : 'text-background/50 hover:text-background'
                }`}
            >
                {entry.title}
            </a>
        ));
    };

    const renderTocMobile = () => {
        if (groupByH1 && tocGroups) {
            return tocGroups.map(group => (
                <div key={group.id} className="flex flex-col gap-1">
                    <a
                        href={`#${group.id}`}
                        onClick={(event) => { jumpToHeading(event, group.id); setMobileNavOpen(false); }}
                        className={`font-mono text-sm font-semibold transition-colors ${
                            activeSectionId === group.id || activeId === group.id
                                ? 'text-accent'
                                : 'text-background/60 hover:text-background'
                        }`}
                    >
                        {group.title}
                    </a>
                    {group.children.length > 0 && (
                        <div className="ml-3 flex flex-col gap-1 border-l border-[#1A1A1A] pl-3">
                            {group.children.map(child => (
                                <a
                                    key={child.id}
                                    href={`#${child.id}`}
                                    onClick={(event) => { jumpToHeading(event, child.id); setMobileNavOpen(false); }}
                                    className={`font-mono text-xs transition-colors ${
                                        activeId === child.id
                                            ? 'text-accent'
                                            : 'text-background/45 hover:text-accent'
                                    }`}
                                >
                                    {child.title}
                                </a>
                            ))}
                        </div>
                    )}
                </div>
            ));
        }

        return tocEntries.map(entry => (
            <a
                key={entry.id}
                href={`#${entry.id}`}
                onClick={(event) => { jumpToHeading(event, entry.id); setMobileNavOpen(false); }}
                className={`font-mono text-sm transition-colors ${
                    entry.level === 3 ? 'ml-4' : ''
                } ${
                    activeId === entry.id
                        ? 'text-accent font-semibold'
                        : 'text-background/60 hover:text-background'
                }`}
            >
                {entry.title}
            </a>
        ));
    };

    return (
        <div className="w-full flex relative">
            {/* Mobile doc nav toggle */}
            <button
                className="lg:hidden fixed bottom-6 right-6 z-50 bg-accent text-primary px-4 py-2 rounded-full font-mono text-sm font-semibold shadow-lg"
                onClick={() => setMobileNavOpen(!mobileNavOpen)}
            >
                {mobileNavOpen ? 'Close' : 'Navigate'}
            </button>

            {/* Mobile doc nav overlay */}
            {mobileNavOpen && (
                <div className="lg:hidden fixed inset-0 z-40 bg-primary/95 backdrop-blur-xl overflow-y-auto pt-24 pb-24 px-6">
                    <div className="flex flex-col gap-4 mb-8">
                        {navLinks.map(link => (
                            <Link
                                key={link.to}
                                to={link.to}
                                onClick={() => setMobileNavOpen(false)}
                                className={`font-mono text-sm transition-colors ${
                                    activePage === link.label.toLowerCase()
                                        ? 'text-accent font-bold'
                                        : 'text-background/60 hover:text-accent'
                                }`}
                            >
                                {link.label}
                            </Link>
                        ))}
                    </div>

                    <div className="h-px bg-[#1A1A1A] mb-6"></div>

                    <span className="font-mono text-xs text-background/40 uppercase tracking-widest mb-4 block">On this page</span>
                    <div className="flex flex-col gap-3">
                        {renderTocMobile()}
                    </div>
                </div>
            )}

            {/* Sidebar Navigation */}
            <aside className="hidden lg:block w-64 shrink-0 pt-32 pb-24 border-r border-[#1A1A1A] sticky top-0 h-screen overflow-y-auto">
                <div className="px-6 flex flex-col gap-6">
                    <div className="flex flex-col gap-2">
                        {navLinks.map(link => (
                            <Link
                                key={link.to}
                                to={link.to}
                                className={`font-mono text-sm transition-colors ${
                                    activePage === link.label.toLowerCase()
                                        ? 'text-accent font-bold'
                                        : 'text-background/60 hover:text-accent'
                                }`}
                            >
                                {link.label}
                            </Link>
                        ))}
                    </div>
                </div>
            </aside>

            {/* Content + TOC wrapper – centers both as a unit */}
            <div className="flex-1 min-w-0 flex justify-center">
                {/* Main Content */}
                <div ref={contentRef} className="w-full max-w-4xl pt-32 pb-24 px-6 lg:px-16">
                    {children}
                </div>

                {/* On This Page – right-side TOC (dynamic from headings) */}
                <aside className="hidden xl:block w-56 shrink-0 pt-32 pb-24 sticky top-0 h-screen overflow-y-auto">
                    <div className="px-4 flex flex-col gap-4">
                        <span className="font-mono text-xs text-background/40 uppercase tracking-widest">On this page</span>
                        {renderTocDesktop()}
                    </div>
                </aside>
            </div>
        </div>
    );
}
