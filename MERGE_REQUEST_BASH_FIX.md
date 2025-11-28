# Bash Scanner Bugs & Fixes

## Aktueller Status (Stand: 2025-11-28)

### ✅ Gelöste Probleme
- **PR #50 (Network Exfiltration)**: Wurde gemerged und ist im Bash-Scanner enthalten. Rust-Scanner ist angepasst.
- **Parser-Updates**: `bash-log-parser` wurde aktualisiert für korrekte Pfad- und Message-Normalisierung.
- **November 2025 Attack**: Alle 9 neuen Detektoren sind in Rust implementiert und 100% kompatibel (nach Message-Anpassungen).

---

## 🐛 Offener Bash-Bug: pnpm-lock.yaml Timestamp-Check

**Status**: ⏳ Fix implementiert in `shai-hulud-detect-gs`, PR pending upstream.

### Problembeschreibung
In `shai-hulud-detector.sh` (Funktion `check_package_integrity`) wird für `pnpm-lock.yaml` Dateien eine temporäre Datei erstellt, um das Format zu normalisieren.
Der Check auf "Recently modified lockfile" (Wurm-Aktivität) prüft jedoch fälschlicherweise den Timestamp dieser **temporären Datei** statt der Originaldatei.

**Code-Stelle (Bash):**
```bash
# $lockfile ist hier die temporäre Datei (gerade erstellt)
file_age=$(date -r "$lockfile" +%s 2>/dev/null || echo "0")
# ...
if [[ $age_diff -lt 2592000 ]]; then # < 30 Tage
    # Feuert IMMER, da Temp-File 0 Sekunden alt ist
    echo "...Recently modified..."
fi
```

### Beweis
Test-Case: `infected-lockfile-pnpm`
- `pnpm-lock.yaml` Timestamp: 1760434682 (ca. 45 Tage alt)
- Erwartung: Keine Warnung (da > 30 Tage)
- Bash-Ergebnis: Warnung "Recently modified lockfile..." (falsch positiv)
- Rust-Ergebnis: Keine Warnung (korrekt)

### Fix (in shai-hulud-detect-gs implementiert)
```bash
# Verwende $org_file statt $lockfile für den Timestamp-Check
file_age=$(date -r "$org_file" +%s 2>/dev/null || echo "0")
```

### Auswirkung auf Tests
Dies verursacht die einzige verbleibende Diskrepanz im Test-Lauf:
- `infected-lockfile-pnpm`: Bash findet 1 Medium Issue, Rust findet 0.
- Dies ist ein **bestätigter Bash-Bug** und kein Fehler im Rust-Scanner.
