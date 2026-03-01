import React, { useEffect, useState, useRef } from 'react';
import gsap from 'gsap';

// --- Card 1: Event Dispatch Feed ---
const DiagnosticShuffler = () => {
    const eventPool = [
        { type: "message_create",     time: "0.2ms" },
        { type: "interaction_create", time: "0.1ms" },
        { type: "guild_member_add",   time: "0.3ms" },
        { type: "message_delete",     time: "0.1ms" },
        { type: "guild_member_update",time: "0.4ms" },
        { type: "reaction_add",       time: "0.2ms" },
        { type: "message_update",     time: "0.1ms" },
    ];

    const [log, setLog] = useState(() => eventPool.slice(0, 4));
    const poolRef = useRef(0);

    useEffect(() => {
        const interval = setInterval(() => {
            poolRef.current = (poolRef.current + 1) % eventPool.length;
            const next = eventPool[poolRef.current];
            setLog(prev => [next, ...prev.slice(0, 3)]);
        }, 900);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="w-full h-64 flex flex-col font-mono text-xs px-5 pt-5 overflow-hidden">
            <div className="flex justify-between items-center mb-5 text-[10px]">
                <span className="text-background/40 uppercase tracking-widest">Event Dispatch</span>
                <span className="text-[#27c93f] flex items-center gap-1.5">
                    <span className="inline-block w-1.5 h-1.5 rounded-full bg-[#27c93f] animate-pulse"></span>
                    live
                </span>
            </div>
            <div className="flex flex-col gap-2.5 overflow-hidden">
                {log.map((ev, i) => (
                    <div
                        key={`${ev.type}-${i}`}
                        className="flex justify-between items-center transition-all duration-400"
                        style={{ opacity: i === 0 ? 1 : i === 1 ? 0.55 : i === 2 ? 0.25 : 0.1 }}
                    >
                        <span className="text-background">{ev.type}</span>
                        <span className="text-[#27c93f]/80 tabular-nums">{ev.time}</span>
                    </div>
                ))}
            </div>
        </div>
    );
};

// Telemetry card removed

// --- Card 3: Open Source Commit ---
const OpenSourceAction = () => {
    const containerRef = useRef(null);
    const cursorRef = useRef(null);
    const btnRef = useRef(null);

    useEffect(() => {
        const ctx = gsap.context(() => {
            const tl = gsap.timeline({ repeat: -1, repeatDelay: 1.5 });

            tl.set(cursorRef.current, { x: 0, y: 150, opacity: 0, scale: 1 })
                .to(cursorRef.current, { opacity: 1, y: 60, duration: 0.4 })
                .to(cursorRef.current, { x: 70, y: 0, duration: 0.8, ease: "power2.inOut" })
                .to(cursorRef.current, { scale: 0.85, duration: 0.1 })
                .to(btnRef.current, { backgroundColor: '#B7410E', color: '#FAF8F5', borderColor: '#B7410E', duration: 0.1 }, "<")
                .to(cursorRef.current, { scale: 1, duration: 0.1 })
                .to(cursorRef.current, { y: 150, opacity: 0, duration: 0.6, delay: 0.4 })
                .set(btnRef.current, { backgroundColor: 'transparent', color: '#FAF8F5', borderColor: 'rgba(255,255,255,0.1)' });

        }, containerRef);
        return () => ctx.revert();
    }, []);

    return (
        <div ref={containerRef} className="relative h-64 w-full bg-[#050505] p-5 flex flex-col items-center justify-center overflow-hidden">
            <div ref={btnRef} className="px-6 py-2 rounded bg-white/5 border border-white/10 font-sans text-sm text-background transition-colors flex items-center gap-2">
                <svg className="w-4 h-4" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round"><path d="M15 22v-4a4.8 4.8 0 0 0-1-3.5c3 0 6-2 6-5.5.08-1.25-.27-2.48-1-3.5.28-1.15.28-2.35 0-3.5 0 0-1 0-3 1.5-2.64-.5-5.36-.5-8 0C6 2 5 2 5 2c-.3 1.15-.3 2.35 0 3.5A5.403 5.403 0 0 0 4 9c0 3.5 3 5.5 6 5.5-.39.49-.68 1.05-.85 1.65-.17.6-.22 1.23-.15 1.85v4" /><path d="M9 18c-4.51 2-5-2-7-2" /></svg>
                Submit Pull Request
            </div>

            <svg
                ref={cursorRef}
                className="absolute top-1/2 left-1/2 w-6 h-6 text-background drop-shadow-md z-10"
                style={{ filter: 'drop-shadow(0 4px 4px rgba(0,0,0,0.5))', pointerEvents: 'none' }}
                viewBox="0 0 24 24"
                fill="currentColor"
                stroke="white"
                strokeWidth="1.5"
            >
                <path d="M4 2l6.1 19 2.5-7.5L20 11z" />
            </svg>
        </div>
    );
};

// --- Card 4: Ollama Integration ---
const OllamaIntegration = () => {
    const [messages, setMessages] = useState([]);
    const sequence = [
        { sender: "Autumn", text: "How can I assist you!" },
        { sender: "User", text: "What is rust?" },
        { sender: "Autumn", text: "That's a great question! Rust is a systems programming language that focuses on speed, memory safety, and safe concurrency." }
    ];

    useEffect(() => {
        let isCancelled = false;

        const runSequence = async () => {
            setMessages([]);

            for (let i = 0; i < sequence.length; i++) {
                if (isCancelled) return;

                await new Promise(r => setTimeout(r, i === 0 ? 500 : 1500));

                if (!isCancelled) {
                    setMessages(prev => [...prev, sequence[i]]);
                }
            }

            if (!isCancelled) {
                setTimeout(() => {
                    if (!isCancelled) {
                        setMessages([]);
                        runSequence();
                    }
                }, 4000);
            }
        };

        runSequence();
        return () => { isCancelled = true; };
    }, []);

    return (
        <div className="h-64 w-full bg-[#050505] p-5 flex flex-col relative overflow-hidden">
            <div className="flex items-center gap-2 mb-4">
                <span className="relative flex h-2 w-2">
                    <span className="animate-ping absolute inline-flex h-full w-full rounded-full bg-[#ffbd2e] opacity-75"></span>
                    <span className="relative inline-flex rounded-full h-2 w-2 bg-[#ffbd2e]"></span>
                </span>
                <span className="font-mono text-[10px] text-background/50">Ollama Link Active</span>
            </div>

            <div className="flex flex-col gap-3 font-sans text-xs">
                {messages.map((msg, idx) => (
                    <div key={idx} className={`flex flex-col ${msg.sender === "User" ? "items-end" : "items-start"} opacity-0 animate-[fadeIn_0.3s_ease-out_forwards]`}>
                        <span className={`text-[10px] font-mono mb-1 ${msg.sender === "User" ? "text-background/50" : "text-accent"}`}>{msg.sender}</span>
                        <div className={`px-3 py-2 rounded max-w-[85%] leading-relaxed ${msg.sender === "User" ? "bg-[#1A1A1A] text-background border border-[#333]" : "bg-[#B7410E]/10 text-background border border-[#B7410E]/30"}`}>
                            {msg.text}
                        </div>
                    </div>
                ))}
            </div>
            <style>{`
                @keyframes fadeIn { from { opacity: 0; transform: translateY(5px); } to { opacity: 1; transform: translateY(0); } }
            `}</style>
        </div>
    );
};

// --- Card 5: Postgres + SQLx Storage ---
const PostgresStorage = () => {
    const [queries, setQueries] = useState([
        "SELECT id, reason FROM mutes WHERE active = true;",
        "CACHE GET modlog:guild:123456 -> HIT",
        "UPDATE users SET warns = warns + 1 WHERE id = $1;",
        "INSERT INTO audit_log (action, node) VALUES ($1, $2);",
    ]);

    useEffect(() => {
        const interval = setInterval(() => {
            setQueries(prev => {
                const current = [...prev];
                const first = current.shift();
                current.push(first);
                return current;
            });
        }, 1500);
        return () => clearInterval(interval);
    }, []);

    return (
        <div className="relative h-64 w-full flex flex-col justify-center items-center bg-[#050505] overflow-hidden p-5">
            <div className="absolute top-4 right-4 flex gap-2 items-center bg-[#111] border border-[#333] px-2 py-1 rounded text-[10px] font-mono text-background/60">
                <svg className="w-3 h-3 text-[#336791]" viewBox="0 0 24 24" fill="currentColor"><path d="M12 2C6.48 2 2 6.48 2 12s4.48 10 10 10 10-4.48 10-10S17.52 2 12 2zm-1 14.5v-9c0-.28.22-.5.5-.5h2c.28 0 .5.22.5.5v9c0 .28-.22.5-.5.5h-2c-.28 0-.5-.22-.5-.5z" /></svg>
                sqlx + postgres + redis
            </div>
            <div className="w-full max-w-[90%] flex flex-col gap-3">
                {queries.map((q, i) => (
                    <div
                        key={i}
                        className={"font-mono text-[10px] md:text-xs truncate transition-all duration-500 " + (i === 1 ? 'text-accent scale-105 opacity-100' : 'text-background/40 opacity-50 scale-100')}
                    >
                        {q}
                    </div>
                ))}
            </div>
        </div>
    );
};


// --- Main Features Component ---
export default function Features() {
    const sectionRef = useRef(null);

    useEffect(() => {
        const ctx = gsap.context(() => {
            gsap.fromTo('.feature-card',
                { autoAlpha: 0, y: 40 },
                {
                    autoAlpha: 1,
                    y: 0,
                    duration: 0.8,
                    stagger: 0.1,
                    ease: "power3.out",
                    scrollTrigger: {
                        trigger: sectionRef.current,
                        start: "top 75%",
                    }
                }
            );
        }, sectionRef);
        return () => ctx.revert();
    }, []);

    const features = [
        {
            title: "Memory-Safe Core",
            desc: "Written in Rust. Handles concurrent Discord events with no GC pauses and no runtime overhead. Fast and reliable under real server load.",
            comp: <DiagnosticShuffler />
        },
        {
            title: "Postgres + Redis Cache",
            desc: "SQLx + PostgreSQL as the source of truth. Redis cache layer for fast audit and modlog lookups.",
            comp: <PostgresStorage />
        },
        {
            title: "Ollama (Optional AI)",
            desc: "Local, private AI mention replies via Ollama. Your own model, your own hardware. No required tokens.",
            comp: <OllamaIntegration />
        },
        {
            title: "Open Source",
            desc: "Public code. Fork it, adjust the rules, compile it yourself. Control your tools.",
            comp: <OpenSourceAction />
        }
    ];

    return (
        <section id="features" ref={sectionRef} className="py-24 w-full max-w-[96rem] mx-auto px-4 md:px-6 relative z-10">
            <div className="mb-16 flex flex-col items-center text-center">
                <h2 className="font-mono font-bold text-sm text-accent tracking-widest uppercase mb-2">&gt;_ THE STACK</h2>
                <h3 className="font-sans text-3xl font-medium text-background">Honest infrastructure.</h3>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
                {features.map((f, i) => (
                    <div key={i} className="feature-card flex flex-col bg-[#0A0A0A] rounded-lg border border-[#333] hover:border-accent/50 transition-colors overflow-hidden">

                        {/* Terminal Header Bar */}
                        <div className="bg-[#151515] border-b border-[#333] px-4 py-2 flex items-center gap-2">
                            <div className="w-2.5 h-2.5 rounded-full bg-[#ff5f56]"></div>
                            <div className="w-2.5 h-2.5 rounded-full bg-[#ffbd2e]"></div>
                            <div className="w-2.5 h-2.5 rounded-full bg-[#27c93f]"></div>
                            <span className="font-mono text-[10px] text-background/40 ml-2">autumn_mod_{i + 1}.rs</span>
                        </div>

                        {/* Interactive Component Block */}
                        <div className="w-full flex justify-center items-center p-1 bg-[#050505]">
                            {f.comp}
                        </div>

                        {/* Description Block */}
                        <div className="p-6 bg-[#0E0E0E] border-t border-[#333] flex-1">
                            <h4 className="font-sans font-medium text-base xl:text-lg whitespace-nowrap tracking-tight text-background mb-2">{f.title}</h4>
                            <p className="font-mono text-xs text-background/50 leading-relaxed">
                                {f.desc}
                            </p>
                        </div>
                    </div>
                ))}
            </div>
        </section>
    );
}
