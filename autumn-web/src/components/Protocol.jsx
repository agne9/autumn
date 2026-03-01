import React, { useEffect, useRef } from 'react';
import gsap from 'gsap';
import { ScrollTrigger } from 'gsap/ScrollTrigger';

export default function Protocol() {
    const containerRef = useRef(null);
    const cardsRef = useRef([]);

    const steps = [
        {
            num: "01",
            title: "Act in seconds",
            desc: "Full moderation surface — bans, kicks, timeouts, warns, and bulk purges — from slash commands or prefix. Both supported natively.",
            Animation: () => (
                <div className="w-full font-mono text-xs md:text-sm flex flex-col gap-3">
                    <div className="flex flex-col gap-1">
                        <div className="text-background/40 text-[10px]"># moderation transcript</div>
                        <div><span className="text-accent">$</span> <span className="text-background/90">!warn @rebeluser posting invite links</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">Warning recorded.</span> Case W14 opened.</div>
                    </div>
                    <div className="flex flex-col gap-1">
                        <div><span className="text-accent">$</span> <span className="text-background/90">!timeout @rebeluser 30m repeated spam</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">Timeout applied.</span> Case T8 opened.</div>
                    </div>
                    <div className="flex flex-col gap-1">
                        <div><span className="text-accent">$</span> <span className="text-background/90">!purge 50</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">50 messages deleted.</span></div>
                    </div>
                </div>
            )
        },
        {
            num: "02",
            title: "Everything becomes a case",
            desc: "Every action opens a numbered, auditable case. Add notes, update reasons, and query history across any moderator or target.",
            Animation: () => (
                <div className="w-full font-mono text-xs md:text-sm flex flex-col gap-3">
                    <div className="flex flex-col gap-1">
                        <div className="text-background/40 text-[10px]"># audit log session</div>
                        <div><span className="text-accent">$</span> <span className="text-background/90">!case W14</span></div>
                        <div className="text-background/50 pl-2">↳ W14 · warn · @rebeluser · <span className="text-[#ffbd2e]">posting invite links</span></div>
                    </div>
                    <div className="flex flex-col gap-1">
                        <div><span className="text-accent">$</span> <span className="text-background/90">!case W14 note Escalated to admin team</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">Note added.</span></div>
                    </div>
                    <div className="flex flex-col gap-1">
                        <div><span className="text-accent">$</span> <span className="text-background/90">!warnings @rebeluser 7</span></div>
                        <div className="text-background/50 pl-2">↳ 3 warnings in the last 7 days. <span className="text-background/30">// Redis cache hit</span></div>
                    </div>
                </div>
            )
        },
        {
            num: "03",
            title: "Automate the boring parts",
            desc: "Word filter detects and acts on violations automatically. Warn thresholds trigger escalating timeouts — no manual follow-up needed.",
            Animation: () => (
                <div className="w-full font-mono text-xs md:text-sm flex flex-col gap-3">
                    <div className="flex flex-col gap-1">
                        <div className="text-background/40 text-[10px]"># server configuration</div>
                        <div><span className="text-accent">$</span> <span className="text-background/90">!wordfilter enable</span></div>
                        <div><span className="text-accent">$</span> <span className="text-background/90">!wordfilter action timeout</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">Filter active.</span> Violations → 5 min timeout.</div>
                    </div>
                    <div className="flex flex-col gap-1">
                        <div><span className="text-accent">$</span> <span className="text-background/90">!escalation enable</span></div>
                        <div><span className="text-accent">$</span> <span className="text-background/90">!escalation set warns 3</span></div>
                        <div className="text-background/50 pl-2">↳ <span className="text-[#27c93f]">Escalation active.</span> 3 warns/24h → auto-timeout.</div>
                    </div>
                </div>
            )
        }
    ];

    useEffect(() => {
        const ctx = gsap.context(() => {
            // New sticky-based stacking logic (no GSAP pin)
            cardsRef.current.forEach((card, index) => {
                if (index === cardsRef.current.length - 1) return;

                const innerCard = card.querySelector('.protocol-card-inner');
                const nextCard = cardsRef.current[index + 1];

                if (innerCard && nextCard) {
                    gsap.to(innerCard, {
                        scale: 0.9,
                        opacity: 0.3,
                        filter: "blur(4px)",
                        ease: "none",
                        scrollTrigger: {
                            trigger: nextCard,
                            start: "top bottom",
                            end: "top top",
                            scrub: true,
                        }
                    });
                }
            });
        }, containerRef);
        return () => ctx.revert();
    }, []);

    return (
        <section id="protocol" className="w-full bg-primary relative" ref={containerRef}>
            {steps.map((step, index) => (
                <div
                    key={index}
                    ref={el => cardsRef.current[index] = el}
                    className="h-screen w-full flex items-center justify-center sticky top-0 overflow-hidden bg-primary"
                    style={{ zIndex: index }}
                >
                    <div className="protocol-card-inner h-auto md:h-[330px] w-[90vw] max-w-4xl bg-[#0A0A0A] rounded-lg border border-[#333] flex flex-col md:flex-row shadow-[0_0_30px_rgba(0,0,0,0.8)] overflow-hidden">

                        {/* Terminal Window Decoration (mobile) */}
                        <div className="w-full h-8 bg-[#151515] border-b border-[#333] px-4 flex items-center gap-2 md:hidden shrink-0">
                            <div className="w-2.5 h-2.5 rounded-full bg-[#ff5f56]"></div>
                            <div className="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]"></div>
                            <div className="w-2.5 h-2.5 rounded-full bg-[#27c93f]"></div>
                            <span className="font-mono text-[10px] text-background/40 ml-2">step_0{index + 1}.sh</span>
                        </div>

                        {/* Content Panel */}
                        <div className="flex-1 md:basis-1/2 flex flex-col justify-center p-8 md:p-16 pt-8 md:pt-16 border-b md:border-b-0 md:border-r border-[#333] bg-[#0E0E0E]">
                            <span className="font-mono text-sm text-accent mb-4 font-bold">[{step.num}]</span>
                            <h3 className="font-sans font-medium text-2xl md:text-3xl text-background mb-4">
                                {step.title}
                            </h3>
                            <p className="font-mono text-sm text-background/60 leading-relaxed">
                                {step.desc}
                            </p>
                        </div>

                        {/* Visual / Code Panel */}
                        <div className="flex-1 md:basis-1/2 flex flex-col bg-[#050505] relative">
                            {/* Desktop Terminal Header */}
                            <div className="hidden md:flex w-full h-8 bg-[#151515] border-b border-[#333] px-4 items-center gap-2">
                                <div className="w-2.5 h-2.5 rounded-full bg-[#ff5f56]"></div>
                                <div className="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]"></div>
                                <div className="w-2.5 h-2.5 rounded-full bg-[#27c93f]"></div>
                                <span className="font-mono text-[10px] text-background/40 ml-2">step_0{index + 1}.sh</span>
                            </div>
                            <div className="p-8 flex-1 flex items-center">
                                <step.Animation />
                            </div>
                        </div>

                    </div>
                </div>
            ))}
        </section>
    );
}
