---
postId: 811deccfdddd
topics:
  - Rust
  - Retro Computing
  - Game Development
  - Emulation
  - Commodore 64
---

# Porting C64 Games to Rust
### a subtitle

In 1985 Daniele Piccoli wrote a football management game in Commodore 64 BASIC and mailed it, presumably on cassette, into the small universe of Italian home computing. Forty years later the game still runs, but the machine it was written for exists mostly in emulators and in the memories of people who once waited through its loading screen. This particular game was never memorable for its looks; the graphics are character-cell boxes and the audio simply does not exist. What made it stick, for me, was that a teenager could LIST it and peek into code that was genuinely complex for its time, a whole league simulation laid bare in a few hundred numbered lines. That is the real reason this game, of all games, seemed worth preserving. The interesting question is not whether you can rewrite a game like this in Rust. Of course you can; it's a few hundred lines of arithmetic and PRINT statements. The interesting question is what "porting" should mean when the source material is a piece of history.

There are two honest answers. You can translate the game logic into idiomatic Rust and build a modern terminal UI around it, which preserves the rules but discards the artifact. Or you can preserve the artifact itself: write an interpreter that runs the original BASIC listing, character graphics and all, inside a modern terminal. This project ended up doing both, and the second path turned out to be where all the hard lessons live.

<img src="./splash-comparison.png" alt="Original C64 splash screen in VICE on the left and the same BASIC listing rendered by the Rust interpreter in a macOS terminal on the right" width="100%">

The screenshot above is the payoff. On the left, the original PRG running in VICE. On the right, the same unmodified listing interpreted by a Rust program drawing into an ordinary terminal with Unicode and ANSI colors. Same title box, same pitch, same little players. Getting those two images to agree took more archaeology than engineering.

## Two ports, one game

The repository holds both approaches side by side. The first is a conventional rewrite: `GameState`, `Player`, and `Team` structs, a match engine that reproduces the original formulas, and a ratatui interface. It plays well, but every screen is an interpretation. The second is [`c64basic`](https://github.com/aovestdipaperino/c64basic), a small interpreter crate that consumes the original listing in petcat format, the textual encoding VICE uses where control characters appear as `{clr}`, `{down}`, or `{$dd}`.

The interpreter pipeline is deliberately boring:

```mermaid
flowchart TB
    A["footballmanager.txt<br/>(petcat-format BASIC)"] --> B["Lexer<br/>tokens + PETSCII escapes"]
    B --> C["Parser<br/>AST per line"]
    C --> D["Interpreter<br/>token-bucket paced"]
    D --> E["Screen buffer<br/>40x25 PETSCII bytes"]
    E --> F["Renderer<br/>Unicode + ANSI colors"]
    style A fill:#e8f5e9,stroke:#2e7d32,color:#111
    style E fill:#e3f2fd,stroke:#1565c0,color:#111
    style F fill:#e3f2fd,stroke:#1565c0,color:#111
```

The screen buffer is the load-bearing decision. The interpreter does not print anything; it pokes PETSCII bytes into a 40 by 25 grid, exactly like the C64 wrote screen codes into memory at $0400. A separate renderer walks the grid at 50 Hz and translates each byte to a Unicode glyph and a terminal color. Keeping the buffer in PETSCII means the translation problem stays in one function, and that function is where this port nearly died.

## The character ROM is the only source of truth

CBM BASIC strings are full of graphics characters. The pitch in Football Manager is drawn from bytes like `{$dd}` (a vertical bar), `{$c0}` (a horizontal bar), and `{$b0}`/`{$ae}` (corners). Map those bytes to the wrong Unicode glyphs and the pitch dissolves into the abstract art you may remember from bad ROM dumps: slashes where corners should be, checkmarks in the penalty area.

The first version of the mapping table was written from memory and from charts found online, and it was wrong in a dozen places. Reference charts disagree with each other, because PETSCII has two character sets, because print codes differ from screen codes, and because half the internet confuses the CBM-key graphics block with the shifted-letter block. The fix was to stop trusting charts entirely. VICE ships the actual character generator ROM, `chargen-901225-01.bin`, 8 bytes per glyph, one bit per pixel. A ten-line Python script renders any byte as ASCII art:

```text
$B0 (screen 70):          $D5 (SHIFT-U):
........                  ........
........                  ........
........                  ........
...#####                  .....###
...#####                  ....####
...##...                  ...###..
...##...                  ...##...
...##...                  ...##...
```

Now the mapping is a matter of looking: `$B0` is a square top-left corner, so it becomes `┌`; `$D5` is rounded, so it becomes `╭`. The final table in the interpreter reads like an inventory of small certainties:

```rust
0xC9 => '╮', // SHIFT-I, top-right rounded corner
0xCA => '╰', // SHIFT-J, bottom-left rounded corner
0xCD => '╲', // SHIFT-M, diagonal
0xCE => '╱', // SHIFT-N, diagonal
0xD1 => '●', // SHIFT-Q, filled circle
0xD7 => '○', // SHIFT-W, hollow circle
```

One structural fact fell out for free: bytes `$60` to `$7F` map to the same ROM glyphs as `$C0` to `$DF`, so the second range is defined as a one-line delegation to the first. That is not an optimization; it is what the hardware does, and encoding hardware facts as code structure is the closest thing this kind of project has to a design principle.

## Borrowing the real font

Unicode gets you a recognizable pitch, but it is still an impersonation. `┌` in a modern terminal font is a thin elegant line; the C64 drew its corners two pixels thick, and its rounded corners have a chunky charm no coding font reproduces. There is a way to stop impersonating: Style64 publishes C64 Pro Mono, a free TrueType font built from the actual character ROM, with every glyph of both PETSCII banks exposed through Unicode private use area codepoints, screen codes mapped at U+EE00 for the uppercase bank and U+EF00 for the lowercase one. Since the interpreter already stores the screen as PETSCII bytes, a `--c64-font` flag that emits those codepoints instead of the standard mapping is almost embarrassingly small: convert the print code to a screen code, add the base, done. Set the terminal font to C64 Pro Mono and every cell is the exact 8x8 bitmap the VIC-II would have fetched.

Almost. The first run produced skulls and smileys where the letters F, G, and I should have been. The private use area is legally a no man's land, and modern terminals have quietly colonized it: Warp and Ghostty ship built-in Nerd Font symbols and override parts of that range no matter which font you chose, and the Nerd Font "progress" icons happen to sit at U+EE00 through U+EE0B, exactly where the screen codes for those letters land. Ghostty adds a second insult by treating private use glyphs as icons and stretching them to double width when the neighboring cell has room, which turned the box corners of the team-selection screen into giant blots.

The fix was to invert the strategy. Parsing the font's character map showed that C64 Pro Mono also covers nearly all of the standard Unicode codepoints the renderer already used, box drawing, block elements, card suits, and draws them with the same authentic bitmaps. So `--c64-font` now emits standard Unicode wherever the font covers it, and reserves the private use area for exactly eight screen codes that have no faithful Unicode equivalent, things like the half-cell checkerboards and the thin vertical bars. Letters stay letters, terminals keep their hands off them, and fidelity survives.

One caveat remains, and it is instructive. Ghostty, Kitty, and WezTerm draw box-drawing characters themselves, bypassing the font entirely so that lines join seamlessly, which silently reintroduces the thin modern corners. Plain Terminal.app, with no such cleverness, renders the font's own glyphs and produces the most faithful picture of the three. The most advanced terminals are the worst at getting out of the way, and the fifty-year-old default is the best.

<img src="./font-comparison.png" alt="The team selection screen rendered side by side: Terminal.app on the left with the C64 font's chunky two-pixel box lines and rounded corners, Ghostty on the right drawing the same box with its own thin box-drawing lines" width="100%">

The text is identical on both sides, pixel for pixel, because letters come straight from the font. The frame gives the game away: on the left, Terminal.app uses the font's glyphs and the box has the thick rounded corners of a real C64; on the right, Ghostty substitutes its own hairline box drawing and the eighties evaporate from everything but the type.

## Emulating slowness on purpose

The first time the interpreter ran the full game, the splash screen was invisible. Not broken: too fast. The original paces itself with empty delay loops, `FOR TR=1 TO 500: NEXT`, which burn real time at 1 MHz and evaporate in native code. A Rust interpreter executes a couple of million BASIC statements per second; the goal animation, twenty ball movements with delay loops between them, completed between two frames of the renderer.

So the interpreter throttles itself with a token bucket. An empty FOR/NEXT loop on real hardware runs at roughly 600 iterations per second, so the bucket earns 600 statements per second and the main loop spends them:

```rust
earned = (earned + now.duration_since(last_exec).as_secs_f64() * rate).min(rate);
let budget = earned as u32;
earned -= budget as f64;
```

The cap at one second of backlog matters: without it, any stall (a window resize, a debugger pause) would bank thousands of statements and release them as a burst, skipping the very animations the throttle exists to preserve. A `--speed` flag scales the rate, and `--speed max` removes it for testing. Emulating a slow machine turns out to mean emulating its slowness.

<img src="./goal-comparison.gif" alt="Animated comparison of the goal celebration: the original game flashing GOAL! in VICE on the left, and the same listing rendered by the Rust interpreter in a terminal on the right" width="100%">

This is what the throttle buys. On the left, the original PRG in VICE flashes its reverse-video `GOAL !` at the pace 1985 intended; on the right, the Rust interpreter runs the same animation loop at the same rhythm, ten flashes with a `FOR TY=1 TO 100` delay between each. The two matches are different because each side rolled its own random numbers, but the celebration is beat for beat the same animation. At native speed it would be over in less than a millisecond.

## Let the original testify

How do you know a port is faithful? You ask the original. VICE, the venerable Commodore emulator, now exists in a build with an embedded MCP server, which means an AI agent (or any HTTP client) can load the original PRG, press keys, read memory, and take screenshots of the genuine article. The comparison image at the top of this post came from exactly that workflow.

Screenshots are for humans, though. The durable version reads screen memory. On the C64 the visible screen lives as 1000 bytes at $0400, one screen code per cell, so a fixture is just a hex dump of that region captured from VICE at a chosen moment. The interpreter stores its own screen the same way, in PETSCII, and a golden test translates both sides to Unicode through the same glyph table before comparing all 25 rows:

```rust
assert_screens_match(
    &interp_rows(&i),                 // headless run of footballmanager.txt
    &fixture_text("team-selection.hex"), // VICE $0400, hand-captured once
    "team selection screen",
);
```

The team-selection screen and the main menu matched VICE byte for byte on the first run, which means the character-ROM work is now pinned by tests instead of by my memory of what looked right.

Reaching deeper screens needs one more thing: determinism. The interpreter seeded its RNG from the OS, so anything past the menus was different every run and impossible to assert against. A `--seed` flag and a seedable generator fix that. Now a scripted playthrough is reproducible, which is what lets a test play an entire fifteen-match season and check that the game reaches the promotion screen rather than hanging or crashing. That season test is also how the bug fixes earn their keep: the original's goal-animation counter never resets, so a faithful run of the unpatched listing would loop forever on the second goal, and the test would sit there until the step budget ran out.

Driving all of this is a second small feature with outsized returns: a `--keyport` flag that opens a localhost TCP socket and injects received bytes as keypresses. Suddenly the whole game is scriptable from the shell:

```bash
c64basic --speed max --keyport 6464 footballmanager.txt &
printf '15\r' | nc 127.0.0.1 6464   # choose team 15
printf 'G'    | nc 127.0.0.1 6464   # play the match
```

Automated play at maximum speed is how the financial formulas, the league table sorting, and the three-points patch for the international edition were verified without a human mashing the space bar.

## Inside the match engine

Fidelity only matters if you know what you are being faithful to, so it is worth unpacking how the game actually decides that Juventus beat you 1-0. The whole simulation fits in perhaps forty lines of BASIC, and it is a nice specimen of 8-bit game design: no ball physics, no player positions, just a handful of numbers colliding through `RND`.

Your side is summarized by five values. Energy is the average power of your eleven fielded players, power being a per-player stat from 1 to 20 that drops by 1 each week a player is fielded and recovers by 10 when he rests. Morale is a club-wide number from 1 to 20 that the results of previous matches push around. Defense, midfield, and attack are the summed style ratings (1 to 5, the player's innate quality) of the players you fielded in each third of the squad list; the first eight players in the roster are defenders, the next eight midfielders, the last eight forwards, and where you field them is your formation. These five collapse into two composite ratings:

```basic
20180 D(6)=INT(D(3)+D(4)/2+D(1)/2+D(2)/2)      : REM defense rating
20190 D(7)=INT(D(5)+(D(6))/2+(D(3))/2+(D(4))/2): REM attack rating
```

Defense is defense plus half of each of midfield, energy, and morale. Attack is your attack skill plus half your own defense rating and half of defense and midfield again. Everything feeds everything; a demoralized, exhausted team attacks worse even with Mbappé up front. (The line above shows the repaired formula. As shipped in 1985, `D(7)` started from itself, freshly zeroed, instead of `D(5)`, so your forwards' skill never reached the pitch. More on that in a moment.)

The opponent is cheaper to build. The AI has no roster at all: its five stats are generated on match day from its league points, `D(PZ)=INT(RND(1)*(G(A1)/I*3)+10)`, which reads as "a random number scaled by points-per-match, plus a floor of 10, capped at 20." A team at the top of the table shows up strong, a struggling one shows up weak, and nobody pays its wage bill. The same two composite formulas then produce the opponent's defense and attack ratings.

The match itself is a loop over chances, and the number of chances is governed by a tempo variable that measures how open the game is:

```basic
20200 HZ=D(7)-D(13)+D(14)-D(6):IFHZ<10THENHZ=15
3430 IFINT(RND(1)*HZ)+1>9THENGOSUB20270 : REM a chance happens
3440 IFINT(RND(1)*HZ)+1>3THEN3430       : REM ...and the half goes on
```

`HZ` is both attacks minus both defenses: two attack-heavy sides produce a large `HZ`, which makes the inner rolls exceed their thresholds more often, which means more chances and a longer half. Two defensive sides strangle the loop early. Each chance flips a coin to pick the attacking side, then resolves with the game's one essential formula:

```basic
20290 IFINT(RND(1)*D(14)-RND(1)*D(6))>0THENH(A4)=H(A4)+1 : REM they score
20310 IFINT(RND(1)*D(7)-RND(1)*D(13))>0THENH(A3)=H(A3)+1 : REM you score
```

A goal is a random draw from the attacker's attack rating beating a random draw from the defender's defense rating. That is the entire theory of football on offer: your rating buys you a bigger die, not a guaranteed win, and a 20-versus-10 mismatch still ships the occasional 0-0. It is crude and it works, which is why the Rust rewrite reproduces it digit for digit rather than replacing it with something respectable.

After the final whistle the numbers loop back into next week. Winning lifts morale by `(20-K)/2`, drawing resets it toward the middle, losing knocks it down; gate receipts scale with your league position; fielded players lose power, resting players regain it, and a tired player with power below 12 risks injury on a roll of `RND(1)*B(HZ)<=2`. Every input of the next simulation is an output of this one, which is what makes an eight-match season feel like a campaign instead of a slot machine.

## The listing pushes back

Run a 1985 listing under a magnifying glass and it confesses. Static analysis plus scripted play turned up real defects in the original. The goal animation counter `GIR` is never reset, so the exact-equality exit test `IF GIR=20` can only succeed once per session; every goal after the first should loop forever. The substitution prompt accepts the key Y, index 25, into arrays dimensioned to 24, a guaranteed `BAD SUBSCRIPT` crash. Your team's attack rating is computed from itself (freshly zeroed) instead of from the attack skill, so your strikers never mattered; the opponent's formula is correct. And one line hides `GOTO3805` inside a string literal, so when you go broke the game literally prints `:GOTO3805` on screen instead of jumping.

None of this is mockery. It is what shipping looked like when your debugger was a television set. But it sharpens the porting question: the faithful interpreter reproduces the bugs, the fixed edition (`football-manager-ita-fixed.bas`) repairs them, and both are legitimate ports of different things, one of the artifact and one of the intent.

## Back to the metal

The fixed and [international](https://gist.github.com/aovestdipaperino/b614dbf5a0fcf00fc70540e710ae55ef) editions are just text files the Rust interpreter reads. Turning them into something a real C64 can boot means tokenizing them into a PRG, which VICE's `petcat` tool does. The first attempt produced `?SYNTAX ERROR IN 1` the instant the machine tried to run it, on a line that reads simply `GOTO 6`.

The cause is a convention collision. In petcat's text format, unshifted letters are written lowercase; an uppercase letter means a shifted PETSCII character. The editions were written in tidy all-caps, so `petcat` dutifully tokenized `GOTO` as six shifted graphics characters rather than the GOTO keyword. The fix is counterintuitive: lowercase the entire source before tokenizing. On the C64's default uppercase/graphics character set, those lowercase PETSCII codes render as capital letters anyway, so the game still looks all-caps on screen. Lowercase to the tool, uppercase to the eye. With that, both editions boot on the genuine machine and land on their team-selection screens, closing the loop from original listing to modern interpreter and back to a bootable binary.

## And now, the browser

The screen-buffer decision paid one more dividend: the interpreter core compiles to WebAssembly unchanged. Because it never talks to a terminal, only pokes PETSCII bytes into a 40 by 25 grid, nothing in it cares whether the host is crossterm or a browser. The web front-end skips fonts and terminal quirks entirely and blits 8x8 glyphs straight from the C64 character generator ROM into a canvas, so every cell is pixel-exact by construction, with no clever terminal to substitute its own hairline corners.

The rest of the machinery carried over just as cleanly. The same 600-statements-per-second token bucket runs on requestAnimationFrame instead of a native loop, so the goal celebration flashes at the pace 1985 intended, and keyboard events map to PETSCII bytes exactly as the TCP keyport does. You can play the international edition live at [fm.enzolombardi.net](https://fm.enzolombardi.net), and the interpreter source lives at [github.com/aovestdipaperino/c64basic](https://github.com/aovestdipaperino/c64basic).

## What the pitch taught about preservation

The Rust rewrite took an afternoon. The interpreter that renders the original listing took character ROM archaeology, a deliberate slowness budget, and an emulator acting as an expert witness. That asymmetry is the lesson. Game logic is portable almost by accident; it is arithmetic, and arithmetic doesn't age. What ages is everything around it: the character set, the timing assumptions, the 40-column screen, the habit of using the machine's quirks as free features. Porting the logic gives you a new game with an old rulebook. Porting the artifact means treating the original binary, ROM, and timing as specifications, and those specifications are checkable, byte by byte, against a running original.

If you try this yourself, start with the screen buffer in the native encoding and translate at the last possible moment, get the real character ROM before writing a single glyph mapping, and give your interpreter a speed limit before you conclude that animations are missing. Make the run deterministic early, with a seed, so you can capture a screen once from the emulator and assert against it forever after. The original is not just the thing you are porting. It is the best test oracle you will ever have.

[Rust](https://medium.com/tag/rust), [Retro Gaming](https://medium.com/tag/retro-gaming), [Emulation](https://medium.com/tag/emulation), [Commodore 64](https://medium.com/tag/commodore-64), [Programming](https://medium.com/tag/programming)

---

## Want more like this?
I write regularly about Rust, design patterns, and performance tips.
Follow me here [on Medium](https://enzolombardi.net) to stay updated.
