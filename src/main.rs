use leptos::prelude::*;
use std::time::Duration;

fn main() {
    leptos::mount::mount_to_body(|| {
        let (text, set_text) = create_signal("".to_string());
        let full_bio = "> init --user bishal\n[+] Role: Linux Admin @ Dheyo.ai\n[+] Env: Nix-on-Droid + Termux\n[+] Virt: QEMU/Alpine (Docker on Android)\n[+] Projects: Slurm Cluster, Google LDAP/SSSD\n[+] Status: Mastering Rust/Leptos";

        // Typing Effect Logic
        create_effect(move |_| {
            let chars: Vec<char> = full_bio.chars().collect();
            for i in 0..chars.len() {
                let c = chars[i];
                set_timeout(
                    move || { 
                        set_text.update(|current| current.push(c)); 
                    },
                    Duration::from_millis(i as u64 * 35),
                );
            }
        });

        view! {
            <nav class="navbar">
                <div class="nav-container">
                    <div class="logo">"BISHAL"<span>"SINGH"</span></div>
                    <ul class="nav-links">
                        <li><a href="#projects">"ADMIN_LOG"</a></li>
                        <li><a href="#stack">"SYS_INFO"</a></li>
                        <li><a href="mailto:bishals@dheyo.ai" class="nav-btn">"PING_ME"</a></li>
                    </ul>
                </div>
            </nav>

            <main>
                <section class="hero">
                    <div class="glow-sphere"></div>
                    <div class="hero-content">
                        <h1 class="glitch-title">"SYSTEMS ARCHITECT"</h1>

                        <div class="terminal-container">
                            <div class="terminal-bar">
                                <div class="dots">
                                    <span class="d1"></span>
                                    <span class="d2"></span>
                                    <span class="d3"></span>
                                </div>
                                <span class="term-name">"nix-on-droid — 80x24"</span>
                            </div>
                            <pre class="terminal-output">
                                {move || text.get()}
                                <span class="cursor-block">"█"</span>
                            </pre>
                        </div>
                    </div>
                </section>

                <section id="projects" class="section">
                    <h2 class="label">"// EXPERTISE_MANIFEST"</h2>
                    <div class="card-grid">
                        <div class="card">
                            <span style="color: var(--accent); font-size: 0.7rem;">"01"</span>
                            <h3>"Authentication"</h3>
                            <p>"Synced Raspberry Pi with Google LDAP via SSSD. Pure infrastructure magic."</p>
                        </div>
                        <div class="card">
                            <span style="color: var(--accent); font-size: 0.7rem;">"02"</span>
                            <h3>"HPC Clusters"</h3>
                            <p>"Setup Slurm clusters and web agents on RPi ARM64 nodes."</p>
                        </div>
                        <div class="card">
                            <span style="color: var(--accent); font-size: 0.7rem;">"03"</span>
                            <h3>"Virtualization"</h3>
                            <p>"Learned Docker on Android by virtualizing Alpine via QEMU (Non-root)."</p>
                        </div>
                    </div>
                </section>

                <section id="stack" class="section">
                    <h2 class="label">"// TOOLCHAIN"</h2>
                    <div class="pill-container">
                        <div class="pill">"Bash Automation"</div>
                        <div class="pill">"Nix Flakes"</div>
                        <div class="pill">"Rust/Leptos"</div>
                        <div class="pill">"QEMU/KVM"</div>
                        <div class="pill">"Slurm"</div>
                        <div class="pill">"SSSD/LDAP"</div>
                    </div>
                </section>
            </main>

            <footer>
                <p>"bishals@dheyo.ai | Uptime: 100% | Built in a Shell"</p>
            </footer>
        }
    })
}

