# Possible Bugs in the BASIC Listing

Static analysis of `football-manager-intl.bas` (line numbers are BASIC line
numbers, identical in `footballmanager.txt` / the original PRG unless noted).
All of these are inherited from the original 1980s listing except where
flagged; none have been fixed.

## High impact

### 1. Goal animation hangs from the second goal onward (line 44000)

```basic
44000 GIR=GIR+1:IFGIR=20THEN44020
44010 GOTO 40500
```

`GIR` is never reset anywhere in the program (this is its only assignment).
The first goal animation counts 1→20 and returns with `GIR=20`. On the next
goal `GIR` becomes 21, 22, … and the equality test `GIR=20` can never be true
again, so the ball-bounce loop at 40500–44010 never exits. Any match (or
session — `GIR` survives across matches too) with more than one animated goal
should lock up in the animation. Fix would be `GIR=0` on entry at 40000 or a
`>=` test.

### 2. Out-of-bounds array index crashes on key 'Y'/'Z' (lines 20440, 25285)

```basic
20440 PZ=ASC(A$)-64:IF(PZ<0ORPZ>25)THEN20430
20450 IFC(PZ)<>4THEN20430
```

Arrays are dimensioned `DIMC(24)` (indices 0–24), but the guard admits
`PZ=25` (key `Y`). `C(25)` raises `BAD SUBSCRIPT` and kills the program at
the half-time substitution prompt. Same off-by-one at 25285 in the in-match
substitution routine (`HZ>25` allows 25). The equivalent check at 1320
(`IFA$<"A"ORA$>"X"`) is correct — these two are not.

### 3. Player team's ATTACK skill is ignored (line 20190)

```basic
20180 D(6)=INT(D(3)+D(4)/2+D(1)/2+D(2)/2)
20190 D(7)=INT(D(7)+(D(6))/2+(D(3))/2+(D(4))/2)
```

`D(7)` (your attack rating) is zeroed at 20130 and then computed from itself,
so it reduces to `defense_rating/2 + defense/2 + midfield/2`. Your forwards'
skill `D(5)` is never used. Compare the opponent's version at 3260, which
correctly starts from the attack stat: `D(14)=INT(D(12)+D(11)/2+D(8)/2+D(9)/2)`.
Almost certainly `D(7)` was meant to be `D(5)+…`. Net effect: buying strikers
barely helps; the AI gets the intended formula, you don't.

### 4. Morale formulas are shifted one result over (lines 20510–20520)

```basic
20510 IFU1=P1THENK=INT(20-K)/2+K:GOTO20530
20520 IFU1<P1THENK=INT(K/2)+1
```

Documented intent (CONVERSION_NOTES.md): win → `(20-K)/2+K`, draw → `K/2+1`.
The listing applies the win formula to a **draw**, applies the draw/loss
formula to a **loss** (arguably right), and applies **nothing** on a win —
morale never rises when you win. Note also the precedence quirk: `INT(20-K)/2+K`
halves outside the `INT`, so morale becomes fractional until `K=INT(K)` at
20560 truncates it.

## Medium impact

### 5. Stadium rent is inflated by a stale variable (line 3670)

```basic
3670 FORPZ=1TO24:IFC(PZ)>0THENXZ=XZ+70000+(5-N)*10000
```

`XZ` is not reset to 0 before the loop; it still holds the opponent's league
index from line 3485 (`XZ=A1`). Rent is overstated by 1–16 currency units per
week (observable in-game as totals like `960009`). The overstated `XZ` also
feeds `HZ` (total expenses).

### 6. `GOTO3805` is trapped inside a string literal (line 3800)

```basic
3800 IFW<0THENPRINT"{down}{down}{rght}{rght}YOU HAVE \ "W":GOTO3805
```

The quote after `W` opens a second string literal whose content is
`:GOTO3805` (unterminated, closed by end of line). When you are broke, the
program prints the text `:GOTO3805` on screen and falls through to 3802,
printing "YOU HAVE" twice. Line 3802 has the same stray trailing quote
(harmless there — the literal is empty).

### 7. Self-match guard only works in the bottom division (lines 2965, 2975)

```basic
2965 IFA$(49)=A$(SZ)THEN2940
2970 J(SZ)=J(SZ)*PZ:A1=SZ:SZ=(N-1)*16+SZ
2975 IFSZ=49ORSZ>64THEN2900
```

The name comparison at 2965 checks `A$(SZ)` with `SZ` in 1–16 — i.e. against
*division 1* team names regardless of the division you are in — so it never
matches once you are outside division 1. The backstop at 2975 only catches
slot 49 (your fixed slot in division 4/D). After a promotion your team sits in
a different slot and neither guard fires, so the game can draw *you* as your
own opponent. Additionally, when 2975 does fire it jumps back to 2900 after
`W(SZ)=1` was already set at 2967, permanently marking an innocent team as
"already played" for scheduling.

### 8. FOR/NEXT inside IF…THEN leaves the loop unfinished (line 20170)

```basic
20170 NEXT:D(1)=INT(D(1)/11):FORPZ=2TO5:IFD(PZ)>20THEND(PZ)=20:NEXT
```

In CBM BASIC everything after `THEN` is conditional, including the `NEXT`.
If `D(2)<=20` the `NEXT` never executes: `D(3)`–`D(5)` are never clamped and a
dangling FOR frame is left on the stack (later `NEXT`s can bind to the wrong
loop). The same clamp is done correctly at 3275–3277 for the opponent, on
separate lines. Also note `D(1)=INT(D(1)/11)` divides total energy by a fixed
11 even when fewer than 11 players are fielded.

## Low impact / quirks

### 9. `IFI=Z` compares match count to loan interest (line 2930)

```basic
2930 L=1::IFI=ZTHENWW=INT(RND(1)*2)+1
```

`Z` is the weekly debt-interest variable; the test only behaves as intended
("first match of the season", i.e. `I=0`) while you happen to have no debt.
With debt, the home/away seed can re-randomize whenever interest happens to
equal the number of matches played. Likely meant `IFI=0`. (Note also the
stray double colon.)

### 10. Dead opponent-eligibility test (lines 2950–2960)

```basic
2950 XJ=J(SZ):YJ=PZ:GOSUB20000   (computes KJ=INT(XJ/YJ))
2960 IFJ(SZ)/PZ=0THEN2940
```

The integer-division subroutine result `KJ` is computed and then ignored;
line 2960 tests floating-point `J(SZ)/PZ`, which is zero only when `J(SZ)=0`
(never — `J` starts at 1 and is only multiplied). Probably meant `IFKJ=0`.
As written the check never rejects anything.

### 11. `T` is used but never assigned (line 4110)

```basic
4110 ... R=R+(8-V(1))*5+T*5 ...
```

`T` is 0 forever (no assignment anywhere), so the `T*5` term — presumably a
trophy/title bonus to management level — is dead. Also `R` can go negative if
you keep finishing below 8th, driving "MANAGEMENT LEVEL" negative.

### 12. Gate receipts use an invented position before any table exists (line 20530)

```basic
20530 IFV(1)=0THENV(1)=INT(RND(1)*16)+1
```

Before the first standings are computed your league position — and therefore
the gate money `(17-V(1))*RND(1)*400000…` — is simply a random number 1–16.
Possibly intentional, but worth knowing.

### 13. Dead code around `L` (lines 2930, 3190, 20570)

`L` is set to 1 at 2930 and never changed, so the `IFL<>1` branch at 3190
(alternative opponent-stat generator) is unreachable, and the match-log
string building at 20570 (`IFL=1THENC$(I+1)=…`) always runs but the log is
never read back.

### 14. Result-header prints over the previous screen (line 20595)

`20595 PRINT"...HERE ARE THE OTHER RESULTS"` executes before the framing
subroutine `GOSUB430` runs (that happens inside `GOSUB1620` afterwards), so
the header is drawn onto the final-score screen and immediately overwritten.
Cosmetic.

---

*Generated by static inspection plus runtime observation in the c64basic
emulator; items 1–4 are reproducible, items 7, 9, 10 are inferred from
control-flow analysis of the original listing.*
