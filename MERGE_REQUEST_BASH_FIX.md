MERGE_REQUEST_BASH_FIX.md

Kurzfassung:
- Problem: Abweichungen zwischen Bash- und Rust-Scanner beim Erkennen von `webhook.site`-Mustern (Paranoid-Mode). Der Bash-Scanner verpasst/formatier Probleme inkonsistent (Zeilennummern / Message-Formulierungen).
- Relevanter Bash-PR: https://github.com/Cobenian/shai-hulud-detect/pull/50

Analyse:
- Parser-Bug (Rust bash-log-parser) behoben: risk-marker, Pfad-Konkatenation, Paranoid-Block-Parsing.
- Nach Fixes bleiben Abweichungen auf Testfall `infected-project` (1 Medium-Differenz) hauptsächlich durch `webhook.site`-Message/Zeilennummer-Differenzen.

Vorgehen (gemäß Projektregeln):
1. Nicht automatisch Rust-Scanner verändern, solange Bash-PR (#50) noch nicht gemerged ist.
2. Wenn PR #50 gemerged ist: Re-run tests; falls Bash dann korrekte Findings liefert, entfernen wir die WIP-Note in `bash-log-parser` und re-run Vergleich.
3. Falls PR #50 nicht merged oder Bash bewusst anderes Verhalten bleiben soll, wird ein gezielter Rust-PR vorbereitet, der:
   - die genaue Formulierungen mit Bash angleicht (same message text and counts),
   - Unit-Tests ergänzt, die den infected-project Fall absichern,
   - die Änderung dokumentiert (🐛, MERGE_REQUEST_BASH_FIX.md aktualisieren).

Aktueller Status (Stand: 2025-11-28):
- ✅ **PR #50 MERGED** - Network exfiltration hostname pattern fix ist im Bash-Scanner
- ✅ Parser: aktualisiert (paranoid-block parsing, path handling, message normalisation)
- ✅ Rust network detector: eingeschränktes btoa()-Kontext-Scanning implementiert (3-line window)
- ✅ November 2025 "The Second Coming" Attack: Alle 9 neuen Detektoren implementiert

---

## Offener Bash-Bug: pnpm-lock.yaml Timestamp-Check (PR pending)

**Datei**: `shai-hulud-detector.sh`, Zeile ~1446
**Funktion**: `check_package_integrity()`

### Problem
Bei pnpm-lock.yaml wird eine temporäre Datei erstellt (`$lockfile = mktemp`), aber der Timestamp-Check verwendet `date -r "$lockfile"` statt `date -r "$org_file"`. 

Die Temp-Datei ist immer "gerade erstellt", daher ist `age_diff` immer < 30 Tage, und das Finding wird IMMER erstellt - unabhängig vom tatsächlichen Alter der pnpm-lock.yaml.

### Beweis
```bash
# Test-Case: infected-lockfile-pnpm
# pnpm-lock.yaml Timestamp: 1760434682 (45 Tage alt)
# 30 Tage in Sekunden: 2592000

# Erwartetes Verhalten: Kein Finding (File älter als 30 Tage)
# Tatsächliches Verhalten: Finding wird erstellt (Temp-File ist 0 Sekunden alt)
```

### Fix (in shai-hulud-detect-gs)
```bash
# Zeile 1446: Verwende $org_file statt $lockfile
file_age=$(date -r "$org_file" +%s 2>/dev/null || echo "0")
```

### Betroffene Test-Cases
- `infected-lockfile-pnpm`: Bash=0/1/0, Rust (korrekt)=0/0/0

### Status
- ✅ Fix implementiert in `shai-hulud-detect-gs/shai-hulud-detector.sh`
- ⏳ PR an upstream Cobenian/shai-hulud-detect noch zu erstellen
- ✅ Rust-Scanner hat korrekte Implementierung (prüft Original-Datei)

---

Abgeschlossen:
- Alle Bash-Bugs aus PR #50 sind behoben
- Rust-Scanner ist auf Version 2.7.6 aktualisiert
- Keine offenen Diskrepanzen mehr zwischen Bash und Rust
- WIP-Notes entfernt - Produktion-ready
