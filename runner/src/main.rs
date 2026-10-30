//! Runner for the "100 exercises to learn Rust" course.
//!
//! Replaces `wr`: discovers every exercise crate under `exercises/`, runs
//! `cargo test` for the current one, and offers a rustlings-style interactive
//! watch mode.
//!
//! Watch mode keys (single keypresses, no Enter needed):
//!   r / Enter  re-run the current exercise
//!   n          move to the next exercise (only once the current one passes)
//!   h          show the current exercise's notes (leading source comments)
//!   l          list all exercises with progress
//!   q / Ctrl-C quit
//!   (saving the current exercise's file also re-runs it)
//!
//! Usage:
//!   cargo run -p runner              # watch mode (default)
//!   cargo run -p runner -- run [name]
//!   cargo run -p runner -- list
//!   cargo run -p runner -- reset [name]
//!   cargo run -p runner -- check-all

use anyhow::{bail, Context, Result};
use notify::{Config, EventKind, RecommendedWatcher, RecursiveMode, Watcher};
use std::{
    collections::HashSet,
    env, fs,
    io::{self, IsTerminal, Read, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::mpsc,
    thread,
    time::{Duration, Instant},
};

const EXERCISES_DIR: &str = "exercises";
const STATE_FILE: &str = ".runner-state";

const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn main() -> Result<()> {
    let root = find_repo_root()?;
    let exercises = discover_exercises(&root);
    if exercises.is_empty() {
        bail!("no exercise crates found under `{EXERCISES_DIR}/`");
    }

    let mut args = env::args().skip(1);
    let cmd = args.next();
    let rest: Vec<String> = args.collect();

    match cmd.as_deref() {
        None | Some("watch") => watch(&root, &exercises),
        Some("run") => run_one(&root, &exercises, &rest),
        Some("list") | Some("ls") => {
            list(&root, &exercises);
            Ok(())
        }
        Some("reset") => reset(&root, &exercises, &rest),
        Some("check-all") => check_all(&root, &exercises),
        Some("help") | Some("-h") | Some("--help") => {
            print_help();
            Ok(())
        }
        Some(other) => bail!("unknown command `{other}` — run `runner help`"),
    }
}

// --- raw terminal mode (unix) ------------------------------------------------
//
// Disables canonical mode + echo so each keypress is read immediately, without
// waiting for Enter and without being echoed. ISIG stays on so Ctrl-C still
// delivers SIGINT; a handler restores the terminal before exiting so the user's
// shell is never left in raw mode.

#[cfg(unix)]
mod raw {
    use std::io;
    use std::os::unix::io::AsRawFd;

    static mut SAVED: Option<(i32, libc::termios)> = None;

    extern "C" fn on_sigint(_sig: libc::c_int) {
        unsafe {
            if let Some((fd, term)) = SAVED {
                libc::tcsetattr(fd, libc::TCSANOW, &term);
            }
            libc::_exit(130);
        }
    }

    pub struct Guard {
        fd: i32,
        original: libc::termios,
    }

    impl Guard {
        pub fn enable() -> io::Result<Self> {
            let fd = io::stdin().as_raw_fd();
            // SAFETY: `tcgetattr` only writes into the provided `termios`.
            let mut original: libc::termios = unsafe { std::mem::zeroed() };
            if unsafe { libc::tcgetattr(fd, &mut original) } != 0 {
                return Err(io::Error::last_os_error());
            }
            let mut raw = original;
            raw.c_lflag &= !(libc::ICANON | libc::ECHO);

            let guard = Guard { fd, original };
            // SAFETY: SAVED is written once on the main thread before the key
            // reader is spawned and before any signal can be delivered; the
            // handler only reads it.
            unsafe {
                SAVED = Some((fd, original));
                let mut sa: libc::sigaction = std::mem::zeroed();
                sa.sa_sigaction = on_sigint as *const () as usize;
                sa.sa_flags = 0;
                libc::sigemptyset(&mut sa.sa_mask);
                libc::sigaction(libc::SIGINT, &sa, std::ptr::null_mut());
            }
            if unsafe { libc::tcsetattr(fd, libc::TCSANOW, &raw) } != 0 {
                return Err(io::Error::last_os_error());
            }
            Ok(guard)
        }
    }

    impl Drop for Guard {
        fn drop(&mut self) {
            // Restore on normal exit / panic.
            unsafe {
                libc::tcsetattr(self.fd, libc::TCSANOW, &self.original);
            }
        }
    }
}

// --- discovery ---------------------------------------------------------------

fn find_repo_root() -> Result<PathBuf> {
    let cwd = env::current_dir().context("couldn't read current directory")?;
    let mut p: &Path = &cwd;
    loop {
        if p.join(EXERCISES_DIR).is_dir() {
            return Ok(p.to_path_buf());
        }
        match p.parent() {
            Some(parent) => p = parent,
            None => bail!(
                "not inside the course repo (no `{EXERCISES_DIR}/` found upward from {})",
                cwd.display()
            ),
        }
    }
}

/// Sorted list of exercise directories: `exercises/<chapter>/<exercise>` each
/// containing a `Cargo.toml`. Sorted == intended learning order (dirs are
/// zero-padded).
fn discover_exercises(root: &Path) -> Vec<PathBuf> {
    let base = root.join(EXERCISES_DIR);
    let mut dirs = Vec::new();
    if let Ok(chapters) = fs::read_dir(&base) {
        for chapter in chapters.flatten() {
            let cp = chapter.path();
            if !cp.is_dir() {
                continue;
            }
            if let Ok(exs) = fs::read_dir(&cp) {
                for ex in exs.flatten() {
                    let ep = ex.path();
                    if ep.is_dir() && ep.join("Cargo.toml").exists() {
                        dirs.push(ep);
                    }
                }
            }
        }
    }
    dirs.sort();
    dirs
}

// --- state -------------------------------------------------------------------

fn state_path(root: &Path) -> PathBuf {
    root.join(STATE_FILE)
}

fn key_of(root: &Path, dir: &Path) -> String {
    dir.strip_prefix(root)
        .unwrap_or(dir)
        .to_string_lossy()
        .into_owned()
}

fn load_done(root: &Path) -> HashSet<String> {
    fs::read_to_string(state_path(root))
        .map(|s| {
            s.lines()
                .map(|l| l.trim().to_owned())
                .filter(|l| !l.is_empty())
                .collect()
        })
        .unwrap_or_default()
}

fn save_done(root: &Path, done: &HashSet<String>) -> Result<()> {
    let mut lines: Vec<&str> = done.iter().map(String::as_str).collect();
    lines.sort_unstable();
    let mut body = lines.join("\n");
    if !body.is_empty() {
        body.push('\n');
    }
    fs::write(state_path(root), body).context("couldn't write state file")
}

fn first_pending_idx(
    root: &Path,
    exercises: &[PathBuf],
    done: &HashSet<String>,
) -> Option<usize> {
    exercises.iter().position(|d| !done.contains(&key_of(root, d)))
}

fn find_exercise<'a>(
    root: &Path,
    exercises: &'a [PathBuf],
    name: &str,
) -> Result<&'a PathBuf> {
    let matches: Vec<_> = exercises
        .iter()
        .filter(|d| key_of(root, d).contains(name))
        .collect();
    match matches.len() {
        0 => bail!("no exercise matches `{name}`"),
        1 => Ok(matches[0]),
        _ => {
            let names = matches
                .iter()
                .map(|m| key_of(root, m))
                .collect::<Vec<_>>()
                .join(", ");
            bail!("`{name}` is ambiguous, matches: {names}");
        }
    }
}

// --- running one exercise ----------------------------------------------------

/// Run an exercise's tests, returning (success, captured output).
// stdout and stderr are captured separately and concatenated (no shell);
// `--color always` keeps the output coloured when it's repainted.
const BUILD_TIMEOUT: Duration = Duration::from_secs(300);
const RUN_TIMEOUT: Duration = Duration::from_secs(10);

/// Run `cmd` in its own process group, capturing merged output, and SIGKILL the
/// whole group if it runs past `timeout`. Returns `(killed, exit_code, output)`.
/// Splitting the group matters: `cargo test` spawns the test binary as a child,
/// so killing only `cargo` would orphan a hung test (e.g. the `05_blocking` deadlock).
fn run_timed(
    mut cmd: Command,
    timeout: Duration,
    label: &str,
) -> Result<(bool, Option<i32>, Vec<u8>)> {
    cmd.stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // 0 = child becomes leader of a new process group (pgid == child pid).
        cmd.process_group(0);
    }
    let mut child = cmd
        .spawn()
        .with_context(|| format!("failed to spawn {label}"))?;
    let mut stdout = child.stdout.take().expect("piped stdout");
    let mut stderr = child.stderr.take().expect("piped stderr");
    let out_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stdout.read_to_end(&mut buf);
        buf
    });
    let err_handle = thread::spawn(move || {
        let mut buf = Vec::new();
        let _ = stderr.read_to_end(&mut buf);
        buf
    });

    let start = Instant::now();
    let mut killed = false;
    let code = loop {
        if let Some(status) = child.try_wait()? {
            break status.code();
        }
        if start.elapsed() >= timeout {
            #[cfg(unix)]
            unsafe {
                // Kill the whole process group, not just the leader.
                libc::kill(-(child.id() as libc::pid_t), libc::SIGKILL);
            }
            #[cfg(not(unix))]
            {
                let _ = child.kill();
            }
            killed = true;
            break child.wait().ok().and_then(|s| s.code());
        }
        thread::sleep(Duration::from_millis(50));
    };

    // Cargo writes build progress/errors to stderr and test results to stdout;
    // join both and put stderr first so build context precedes test output.
    let mut merged = err_handle.join().unwrap_or_default();
    merged.append(&mut out_handle.join().unwrap_or_default());
    Ok((killed, code, merged))
}

/// Compile (generous timeout) then run the tests (10s kill timeout). Splitting
/// the two means a cold compile (e.g. tokio) is never killed, while a hung
/// program is. Returns `(success, merged output)`.
fn run_exercise(root: &Path, dir: &Path) -> Result<(bool, Vec<u8>)> {
    let manifest = dir.join("Cargo.toml");
    let key = key_of(root, dir);

    // 1. Compile the tests.
    let mut build = Command::new("cargo");
    build
        .arg("test")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--no-run")
        .arg("--color")
        .arg("always");
    let (bkilled, bcode, mut bout) =
        run_timed(build, BUILD_TIMEOUT, &format!("`cargo test` build for {key}"))?;
    if bkilled {
        bout.extend_from_slice(b"\n\ncompile timed out\n");
        return Ok((false, bout));
    }
    if bcode != Some(0) {
        // Compile failed (or couldn't compile) — surface the compiler errors.
        return Ok((false, bout));
    }

    // 2. Run the tests, bounded by RUN_TIMEOUT.
    let mut run = Command::new("cargo");
    run.arg("test")
        .arg("--manifest-path")
        .arg(&manifest)
        .arg("--color")
        .arg("always");
    let (rkilled, rcode, mut rout) =
        run_timed(run, RUN_TIMEOUT, &format!("`cargo test` run for {key}"))?;
    let mut success = rcode == Some(0);
    if rkilled {
        success = false;
        rout.extend_from_slice(
            format!("\n\n{RED}{BOLD}timed out after 10s — killed (the program didn't finish){RESET}\n")
                .as_bytes(),
        );
    }
    Ok((success, rout))
}

/// Clear the screen and move the cursor to the top-left.
fn clear_screen() {
    // [H = cursor home, [2J = clear screen, [3J = clear scrollback.
    let _ = io::stdout().write_all(b"\x1b[H\x1b[2J\x1b[3J");
    let _ = io::stdout().flush();
}

/// The primary source file to open in an editor / show notes from.
fn source_file(dir: &Path) -> PathBuf {
    let lib = dir.join("src/lib.rs");
    if lib.exists() {
        lib
    } else {
        dir.join("src/main.rs")
    }
}

// --- editor auto-open --------------------------------------------------------

/// Decide which editor to launch, if any. Priority:
/// VS Code (when run inside its terminal) → `$VISUAL` → `$EDITOR` → none.
/// The command must return immediately; a blocking TUI editor (vim, etc.) will
/// fight this program for the terminal.
fn editor_cmd() -> Option<(String, Vec<String>)> {
    let in_vscode = env::var_os("TERM_PROGRAM").is_some_and(|v| v == "vscode");
    if in_vscode {
        for prog in ["code", "codium"] {
            if command_exists(prog) {
                return Some((prog.into(), Vec::new()));
            }
        }
    }
    for var in ["VISUAL", "EDITOR"] {
        if let Some(val) = env::var(var).ok().filter(|v| !v.is_empty()) {
            let mut parts = val.split_whitespace();
            if let Some(program) = parts.next() {
                let args = parts.map(String::from).collect();
                return Some((program.into(), args));
            }
        }
    }
    None
}

fn command_exists(prog: &str) -> bool {
    Command::new(prog)
        .arg("--version")
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .is_ok_and(|s| s.success())
}

/// Open an exercise's source file in the editor, without blocking. Best-effort:
/// errors are ignored (the runner works fine without an editor).
fn open_editor(path: &Path) {
    let Some((program, args)) = editor_cmd() else {
        return;
    };
    let path = path.to_path_buf();
    thread::spawn(move || {
        let _ = Command::new(&program)
            .args(&args)
            .arg(&path)
            .stdin(std::process::Stdio::null())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status();
    });
}

// --- watch events ------------------------------------------------------------

enum WatchEvent {
    Key(u8),
    File(PathBuf),
}

/// Blocking reader: forwards every keypress as a raw byte, so the watch loop
/// can react to arbitrary keys (e.g. "press any key to return" from the list).
/// Runs on its own thread so it never blocks while `cargo test` is running.
fn key_reader(sender: mpsc::Sender<WatchEvent>) {
    let stdin = io::stdin();
    let mut lock = stdin.lock();
    let mut buf = [0u8; 1];
    loop {
        match lock.read(&mut buf) {
            Ok(0) => return,
            Ok(_) => {
                if sender.send(WatchEvent::Key(buf[0])).is_err() {
                    return;
                }
            }
            Err(_) => return,
        }
    }
}

// --- watch mode --------------------------------------------------------------

fn watch(root: &Path, exercises: &[PathBuf]) -> Result<()> {
    if !io::stdin().is_terminal() {
        bail!("watch mode needs an interactive terminal; use `runner run` instead");
    }

    #[cfg(unix)]
    let _raw = raw::Guard::enable().context("couldn't enable raw terminal mode")?;
    #[cfg(not(unix))]
    bail!("interactive watch mode is only supported on unix; use `runner run` instead");

    let total = exercises.len();
    let mut done = load_done(root);

    let mut cur_idx = match first_pending_idx(root, exercises, &done) {
        Some(i) => i,
        None => {
            celebrate(total);
            return Ok(());
        }
    };

    let (tx, rx) = mpsc::channel();
    // File watcher: forward modify/create events for any exercise.
    let tx_watch = tx.clone();
    let mut watcher = RecommendedWatcher::new(
        move |res: notify::Result<notify::Event>| {
            if let Ok(ev) = res {
                if matches!(ev.kind, EventKind::Modify(_) | EventKind::Create(_)) {
                    for p in ev.paths {
                        let _ = tx_watch.send(WatchEvent::File(p));
                    }
                }
            }
        },
        Config::default(),
    )
    .context("couldn't create file watcher")?;
    watcher
        .watch(&root.join(EXERCISES_DIR), RecursiveMode::Recursive)
        .context("couldn't watch `exercises/`")?;

    // Key reader thread. `tx_self` lets the loop re-queue key events that
    // arrive during a file-change coalesce (so a quick `q`/`r` isn't dropped).
    let tx_self = tx.clone();
    thread::spawn(move || key_reader(tx));

    open_editor(&source_file(&exercises[cur_idx]));
    let mut output: Vec<u8> = Vec::new();
    let mut passed = false;
    let mut show_notes = false;
    run_and_render(
        root,
        exercises,
        &done,
        cur_idx,
        &mut output,
        &mut passed,
        &mut show_notes,
    )?;

    while let Ok(ev) = rx.recv() {
        match ev {
            WatchEvent::Key(b) => match b {
                b'q' | b'Q' => {
                    clear_screen();
                    println!("{DIM}bye 👋 (progress saved){RESET}");
                    break;
                }
                b'r' | b'R' | b'\r' | b'\n' => {
                    run_and_render(
                        root,
                        exercises,
                        &done,
                        cur_idx,
                        &mut output,
                        &mut passed,
                        &mut show_notes,
                    )?;
                }
                b'n' | b'N' => {
                    if !passed {
                        render(root, exercises, &done, cur_idx, passed, &output, show_notes);
                        continue;
                    }
                    done.insert(key_of(root, &exercises[cur_idx]));
                    save_done(root, &done)?;
                    match first_pending_idx(root, exercises, &done) {
                        None => {
                            celebrate(total);
                            return Ok(());
                        }
                        Some(i) => cur_idx = i,
                    }
                    open_editor(&source_file(&exercises[cur_idx]));
                    run_and_render(
                        root,
                        exercises,
                        &done,
                        cur_idx,
                        &mut output,
                        &mut passed,
                        &mut show_notes,
                    )?;
                }
                b'h' | b'H' => {
                    show_notes = !show_notes;
                    render(root, exercises, &done, cur_idx, passed, &output, show_notes);
                }
                b'l' | b'L' => {
                    clear_screen();
                    list(root, exercises);
                    println!("\n{DIM}press any key to return{RESET}");
                    let _ = io::stdout().flush();
                    // Any key returns to the exercise view.
                    while let Ok(next) = rx.recv() {
                        if matches!(next, WatchEvent::Key(_)) {
                            break;
                        }
                        // Ignore file events while the list is shown.
                    }
                    render(root, exercises, &done, cur_idx, passed, &output, show_notes);
                }
                _ => {} // other keys are ignored in the exercise view
            },
            WatchEvent::File(p) => {
                if !p.starts_with(&exercises[cur_idx]) {
                    continue;
                }
                // Let a burst of editor write events settle, then run once.
                thread::sleep(Duration::from_millis(60));
                // Coalesce further file events, but preserve queued key events
                // by re-queueing them (don't drain the whole channel).
                let mut queued_keys = Vec::new();
                while let Ok(e) = rx.try_recv() {
                    if let k @ WatchEvent::Key(_) = e {
                        queued_keys.push(k);
                    }
                }
                for k in queued_keys {
                    let _ = tx_self.send(k);
                }
                run_and_render(
                    root,
                    exercises,
                    &done,
                    cur_idx,
                    &mut output,
                    &mut passed,
                    &mut show_notes,
                )?;
            }
        }
    }

    Ok(())
}

/// The exercise's leading source comments, as a reminder string.
fn notes_for(root: &Path, dir: &Path) -> String {
    let path = source_file(dir);
    let Ok(content) = fs::read_to_string(&path) else {
        return format!("{DIM}(couldn't read {}){RESET}", path.display());
    };
    let mut lines = Vec::new();
    for line in content.lines() {
        let t = line.trim_start();
        if t.starts_with("//") || t.is_empty() {
            if lines.len() >= 50 {
                break;
            }
            lines.push(line);
        } else {
            break;
        }
    }
    let label = path.strip_prefix(root).unwrap_or(&path).display();
    if lines.is_empty() {
        format!("{BOLD}Notes — {label}{RESET}\n{DIM}(no leading notes in this file){RESET}")
    } else {
        format!(
            "{BOLD}Notes — {label}{RESET}\n{DIM}{}{RESET}",
            lines.join("\n")
        )
    }
}

/// Clear the screen and repaint the full current state from the top.
fn render(
    root: &Path,
    exercises: &[PathBuf],
    done: &HashSet<String>,
    cur_idx: usize,
    passed: bool,
    output: &[u8],
    show_notes: bool,
) {
    clear_screen();
    let total = exercises.len();
    let key = key_of(root, &exercises[cur_idx]);
    println!("{BOLD}exercise {}/{total}: {key}{RESET}\n", cur_idx + 1);
    let _ = io::stdout().write_all(output);
    let _ = io::stdout().write_all(b"\n\n");
    if show_notes {
        println!("{}\n", notes_for(root, &exercises[cur_idx]));
    }
    if passed {
        println!("{GREEN}{BOLD}✓ passed{RESET} — press {BOLD}n{RESET} for the next exercise\n");
    } else {
        println!("{RED}{BOLD}✗ failed{RESET} — edit & save, or press {BOLD}r{RESET} to re-run\n");
    }
    println!(
        "{DIM}progress {}/{}   {BOLD}r{RESET}{DIM}:run  {BOLD}n{RESET}:next  {BOLD}h{RESET}:notes  {BOLD}l{RESET}:list  {BOLD}q{RESET}:quit{RESET}",
        done.len(),
        total
    );
    let _ = io::stdout().flush();
}

/// Clear, show a "checking" line, run the current exercise, then render.
fn run_and_render(
    root: &Path,
    exercises: &[PathBuf],
    done: &HashSet<String>,
    cur_idx: usize,
    output: &mut Vec<u8>,
    passed: &mut bool,
    show_notes: &mut bool,
) -> Result<()> {
    clear_screen();
    println!(
        "{DIM}Checking {}…{RESET}",
        key_of(root, &exercises[cur_idx])
    );
    let _ = io::stdout().flush();
    let (ok, out) = run_exercise(root, &exercises[cur_idx])?;
    *passed = ok;
    *output = out;
    *show_notes = false;
    render(root, exercises, done, cur_idx, *passed, output, *show_notes);
    Ok(())
}

// --- non-watch subcommands ---------------------------------------------------

fn run_one(root: &Path, exercises: &[PathBuf], rest: &[String]) -> Result<()> {
    let mut done = load_done(root);
    let dir = match rest.first() {
        Some(name) => find_exercise(root, exercises, name)?.clone(),
        None => match first_pending_idx(root, exercises, &done) {
            Some(i) => exercises[i].clone(),
            None => {
                celebrate(exercises.len());
                return Ok(());
            }
        },
    };

    println!("{BOLD}{DIM}▶ {}{RESET}", key_of(root, &dir));
    let (success, out) = run_exercise(root, &dir)?;
    let _ = io::stdout().write_all(&out);
    let _ = io::stdout().flush();
    if success {
        done.insert(key_of(root, &dir));
        save_done(root, &done)?;
        println!("{GREEN}{BOLD}✓ passed{RESET}");
        if let Some(i) = first_pending_idx(root, exercises, &done) {
            println!("{DIM}next: {}{RESET}", key_of(root, &exercises[i]));
        }
    } else {
        println!("{RED}{BOLD}✗ failed{RESET}");
        std::process::exit(1);
    }
    Ok(())
}

fn check_all(root: &Path, exercises: &[PathBuf]) -> Result<()> {
    let mut done = HashSet::new();
    for dir in exercises {
        println!("{BOLD}{DIM}▶ {}{RESET}", key_of(root, dir));
        let (success, out) = run_exercise(root, dir)?;
        let _ = io::stdout().write_all(&out);
        let _ = io::stdout().flush();
        if success {
            done.insert(key_of(root, dir));
            save_done(root, &done)?;
        } else {
            println!("{RED}{BOLD}✗ stopped at {}{RESET}", key_of(root, dir));
            std::process::exit(1);
        }
    }
    celebrate(exercises.len());
    Ok(())
}

fn reset(root: &Path, exercises: &[PathBuf], rest: &[String]) -> Result<()> {
    match rest.first() {
        Some(name) => {
            let dir = find_exercise(root, exercises, name)?;
            let key = key_of(root, dir);
            let mut done = load_done(root);
            if done.remove(&key) {
                save_done(root, &done)?;
                println!("reset {key}");
            } else {
                println!("{DIM}{key} wasn't marked done{RESET}");
            }
        }
        None => {
            let _ = fs::remove_file(state_path(root));
            println!("cleared all progress");
        }
    }
    Ok(())
}

fn list(root: &Path, exercises: &[PathBuf]) {
    let done = load_done(root);
    let n_done = exercises
        .iter()
        .filter(|d| done.contains(&key_of(root, d)))
        .count();
    for (i, dir) in exercises.iter().enumerate() {
        let key = key_of(root, dir);
        let (mark, color) = if done.contains(&key) {
            ("✓", GREEN)
        } else {
            ("✗", RED)
        };
        println!("{DIM}{:>3}.{RESET} {color}{mark}{RESET} {key}", i + 1);
    }
    println!("\n{BOLD}{n_done}/{} done{RESET}", exercises.len());
}

fn celebrate(total: usize) {
    println!("\n{GREEN}{BOLD}🎉 All {total} exercises complete! Well done.{RESET}\n");
}

fn print_help() {
    println!("runner — proceed through the 100 Rust exercises one by one\n");
    println!("USAGE:");
    println!("  cargo run -p runner [-- <COMMAND> [ARGS]]\n");
    println!("COMMANDS:");
    println!("  (default) / watch   interactive: run current, re-run on save, n=next, q=quit");
    println!("  run [name]          run one exercise once (first pending if no name); advance on pass");
    println!("  list                show all exercises with pass/fail status");
    println!("  reset [name]        clear progress (one exercise if name given)");
    println!("  check-all           re-run every exercise, stop at first failure");
    println!("  help                show this message\n");
    println!("WATCH KEYS:  r run · n next · h notes · l list · q quit");
    println!("`name` matches any substring of the exercise path, e.g. `01_syntax`.");
    println!(
        "Editor auto-open: VS Code terminal → `code`; otherwise `$VISUAL`/`$EDITOR` (must be non-blocking)."
    );
    println!("Done state is stored in {STATE_FILE} (gitignored).");
}
