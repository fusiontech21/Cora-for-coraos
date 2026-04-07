use colored::Colorize;
use std::process::Command;
use update::VERSION;

mod update;

fn run(sudo: bool, args: Vec<&str>) {
    let mut cmd = if sudo {
        let mut c = Command::new("sudo");
        c.args(&args);
        c
    } else {
        let mut c = Command::new(&args[0]);
        c.args(&args[1..]);
        c
    };

    let sts = cmd.status().unwrap_or_else(|_| {
        eprintln!("{}", "Failed to execute command".red());
        std::process::exit(1);
    });

    if sts.success() {
        println!("{}", "Done.".green())
    }
}

fn require_pkg(pkg: Option<&String>) -> &str {
    pkg.map(|s| s.as_str()).unwrap_or_else(|| {
        eprintln!("{}", "Missing package name".red());
        std::process::exit(1);
    })
}

fn main() {
    colored::control::set_override(true);

    let args: Vec<String> = std::env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("{}", "No command provided. Use: cora <command>".red());
        std::process::exit(1);
    }

    let cmd = args[0].as_str();
    let pkg = args.get(1);

    match cmd {
        "install" => {
            let mut a = vec!["pacman", "-S"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a)
        }

        "remove" => {
            let mut a = vec!["pacman", "-Rns"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a);
        }

        "softremove" => {
            let mut a = vec!["pacman", "-R"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a);
        }

        "history" => run(false, vec!["tail", "-n", "20", "/var/log/pacman.log"]),

        "search" => run(false, vec!["pacman", "-Ss", require_pkg(pkg)]),

        "update" => run(true, vec!["pacman", "-Syu"]),

        "forceupdate" => run(true, vec!["pacman", "-Syyu"]),

        "upgrade" => {
            let mut a = vec!["pacman", "-S"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a);
        }

        "downgrade" => run(true, vec!["pacman", "-U", require_pkg(pkg)]),

        "info" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        "check" => run(false, vec!["pacman", "-Dk"]),

        "verify" => run(false, vec!["pacman", "-Qk", require_pkg(pkg)]),

        "cache" => run(false, vec!["du", "-sh", "/var/cache/pacman/pkg"]),

        "cleancache" => run(true, vec!["pacman", "-Sc"]),

        "leaves" => run(false, vec!["pacman", "-Qdtt"]),

        "explicit" => run(false, vec!["pacman", "-Qe"]),

        "reinstall" => {
            let mut a = vec!["pacman", "-S"];
            for p in &args[1..] { a.push(p.as_str()); }
            run(true, a);
        }

        "installed" => run(false, vec!["pacman", "-Qs", require_pkg(pkg)]),

        "list" => run(false, vec!["pacman", "-Qe"]),

        "listall" => run(false, vec!["pacman", "-Q"]),

        "files" => run(false, vec!["pacman", "-Ql", require_pkg(pkg)]),

        "owner" => run(false, vec!["pacman", "-Qo", require_pkg(pkg)]),

        "deps" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        "size" => run(false, vec!["pacman", "-Qi", require_pkg(pkg)]),

        "backup" => run(false, vec!["bash", "-c", "pacman -Qe > ~/cora-backup.txt && echo 'Backup saved to ~/cora-backup.txt'"]),

        "restore" => run(true, vec!["bash", "-c", "pacman -S --needed $(cat ~/cora-backup.txt | awk '{print $1}')"]),

        "dependencies" => run(false, vec!["pacman", "-Si", require_pkg(pkg)]),

        "log" => run(false, vec!["cat", "/var/log/pacman.log"]),

        "mirrors" => run(true, vec!["reflector"]),

        "unlock" => run(true, vec!["rm", "/var/lib/pacman/db.lck"]),

        "stats" => run(false, vec!["pacman", "-Qq"]),

        "news" => run(false, vec!["bash", "-c", "curl -s https://archlinux.org/feeds/news/ | grep -oP '(?<=<title>)[^<]+' | head -10"]),

        "autoremove" => {
            let output = Command::new("pacman")
                .args(["-Qdtq"])
                .output()
                .expect("Failed to get orphaned packages");

            if output.stdout.is_empty() {
                println!("{}", "No orphaned packages found!".green());
            } else {
                let pkgs = String::from_utf8(output.stdout).unwrap();
                let pkgls: Vec<&str> = pkgs.lines().collect();
                let mut cmdar = vec!["pacman", "-Rns"];
                cmdar.extend(pkgls.iter().map(|s| *s));
                run(true, cmdar);
            }
        }

        "sync" => {
            println!("{}", "Syncing CoraOS...".cyan().bold());

            if !update::latest() {
                println!("  → Updating Cora...");
                run(false, vec!["bash", "-c",
                    "curl -s https://raw.githubusercontent.com/fusiontech21/Cora-for-coraos/main/Update/update.sh | bash"
                ]);
            }

            println!("  → Syncing distro files...");
            run(true, vec!["bash", "-c", r#"
                TMP=$(mktemp -d)
                git clone --depth 1 https://github.com/fusiontech21/CoraOS.git "$TMP" 2>/dev/null
                if [ -d "$TMP/releng/airootfs" ]; then
                    rsync -a --ignore-existing "$TMP/releng/airootfs/etc/skel/" /etc/skel/ 2>/dev/null
                    rsync -a --ignore-existing "$TMP/releng/airootfs/usr/local/bin/" /usr/local/bin/ 2>/dev/null
                    rsync -a --ignore-existing "$TMP/releng/airootfs/etc/systemd/" /etc/systemd/ 2>/dev/null
                    rsync -a --ignore-existing "$TMP/releng/airootfs/usr/share/" /usr/share/ 2>/dev/null
                    for f in coraos-repair postinstall; do
                        [ -f "$TMP/releng/airootfs/usr/local/bin/$f" ] && \
                            cp "$TMP/releng/airootfs/usr/local/bin/$f" "/usr/local/bin/$f" && \
                            chmod 755 "/usr/local/bin/$f"
                    done
                fi
                rm -rf "$TMP"
            "#]);

            println!("  → Updating packages...");
            run(true, vec!["pacman", "-Syu"]);

            println!("{}", "CoraOS fully synced.".green().bold());
        }

        "self-update" => {
            if update::latest() {
                println!("{}", "You are already on the latest version!".green());
            } else {
                run(false, vec!["bash", "-c",
                    "curl -s https://raw.githubusercontent.com/fusiontech21/Cora-for-coraos/main/Update/update.sh | bash"
                ]);
            }
            std::process::exit(0);
        }

        "secret" => {
            let txt = "You are secretly a Femboy";
            secrething(&txt);
        }

        "details" => {
            println!("{}", r#"
                ██████╗ ██████╗ ██████╗  █████╗
               ██╔════╝██╔═══██╗██╔══██╗██╔══██╗
               ██║     ██║   ██║██████╔╝███████║
               ██║     ██║   ██║██╔══██╗██╔══██║
               ╚██████╗╚██████╔╝██║  ██║██║  ██║
                ╚═════╝ ╚═════╝ ╚═╝  ╚═╝╚═╝  ╚═╝
            "#.cyan().bold());
            println!("{}", "A tool to help beginners use the terminal on Arch-based distros".white());
            println!("{}", format!("Version: {}", VERSION).white());
            println!("{}", "© 2026 fusiontech21 — GPL-3.0".white());
        }

        "version" => {
            println!("{}", format!("  Cora Version : {}", VERSION).cyan());
            println!("{}", "© 2026 fusiontech21 — GPL-3.0".cyan().bold());
        }

        "help" => {
            println!("{}", "Cora - Available Commands".cyan().bold());
            println!("{}", "─────────────────────────────────────────".cyan());
            println!("{} {}", "cora install <pkg>".green().bold(),      "→ Install a package");
            println!("{} {}", "cora remove <pkg>".green().bold(),       "→ Remove a package (full cleanup)");
            println!("{} {}", "cora softremove <pkg>".green().bold(),   "→ Remove just the package");
            println!("{} {}", "cora reinstall <pkg>".green().bold(),    "→ Reinstall a package");
            println!("{} {}", "cora search <pkg>".green().bold(),       "→ Search for a package");
            println!("{} {}", "cora update".green().bold(),             "→ Update the entire system");
            println!("{} {}", "cora upgrade <pkg>".green().bold(),      "→ Upgrade a specific package");
            println!("{} {}", "cora downgrade <pkg>".green().bold(),    "→ Downgrade a package");
            println!("{} {}", "cora info <pkg>".green().bold(),         "→ Show info about a package");
            println!("{} {}", "cora installed <pkg>".green().bold(),    "→ Check if a package is installed");
            println!("{} {}", "cora list".green().bold(),               "→ List explicitly installed packages");
            println!("{} {}", "cora listall".green().bold(),            "→ List all installed packages");
            println!("{} {}", "cora explicit".green().bold(),           "→ List manually installed packages");
            println!("{} {}", "cora files <pkg>".green().bold(),        "→ Show files owned by a package");
            println!("{} {}", "cora size <pkg>".green().bold(),         "→ Show how much disk space a package uses");
            println!("{} {}", "cora owner <file>".green().bold(),       "→ Show which package owns a file");
            println!("{} {}", "cora deps <pkg>".green().bold(),         "→ Show dependencies of a package");
            println!("{} {}", "cora verify <pkg>".green().bold(),       "→ Verify package files aren't corrupted");
            println!("{} {}", "cora check".green().bold(),              "→ Check for broken dependencies");
            println!("{} {}", "cora news".green().bold(),               "→ Show latest Arch Linux news");
            println!("{} {}", "cora history".green().bold(),            "→ Show last 20 pacman installs");
            println!("{} {}", "cora log".green().bold(),                "→ Show full pacman install history");
            println!("{} {}", "cora stats".green().bold(),              "→ List all installed package names");
            println!("{} {}", "cora cache".green().bold(),              "→ Show pacman cache size");
            println!("{} {}", "cora cleancache".green().bold(),         "→ Clean old package cache");
            println!("{} {}", "cora autoremove".green().bold(),         "→ Remove orphaned packages");
            println!("{} {}", "cora backup".green().bold(),             "→ Backup installed packages to a file");
            println!("{} {}", "cora restore".green().bold(),            "→ Restore packages from backup");
            println!("{} {}", "cora mirrors".green().bold(),            "→ Update your mirrorlist");
            println!("{} {}", "cora unlock".green().bold(),             "→ Remove pacman lock file");
            println!("{} {}", "cora sync".green().bold(),               "→ Update everything (distro + packages + cora)");
            println!("{} {}", "cora self-update".green().bold(),        "→ Update cora to the latest version");
            println!("{} {}", "cora details".green().bold(),            "→ Show info about cora");
            println!("{}", "─────────────────────────────────────────".cyan());
            println!("{}", "© 2026 fusiontech21 — GPL-3.0".white());
        }

        _ => {
            println!("{}", format!("Unknown command ({}) — type 'cora help' for options", cmd).yellow());
        }
    }

    update::checkupdate();
    std::process::exit(0);
}

fn secrething(txt: &str) {
    let colors = ["rd", "ylw", "grn", "cyn", "blue", "mgnt"];
    for (i, ch) in txt.chars().enumerate() {
        let clrs = match colors[i % colors.len()] {
            "rd"   => ch.to_string().red().bold(),
            "ylw"  => ch.to_string().yellow().bold(),
            "grn"  => ch.to_string().green().bold(),
            "cyn"  => ch.to_string().cyan().bold(),
            "blue" => ch.to_string().blue().bold(),
            "mgnt" => ch.to_string().magenta().bold(),
            _      => ch.to_string().white().bold(),
        };
        print!("{}", clrs);
    }
    println!();
}
