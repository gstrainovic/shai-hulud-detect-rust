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

Aktueller Status:
- Parser: aktualisiert (paranoid-block parsing, path handling, message normalisation)
- Rust network detector: eingeschränktes btoa()-Kontext-Scanning implementiert (3-line window)
- Testlauf PARANOID: 24/25 Testfälle matchen; einzig `infected-project` hat H/M/L diff: Bash=8/18/2 vs Rust=8/19/2

Nächste Schritte (geplant und ausgeführt):
- WIP-Note im Parser-Ausgang (bei webhook.site) eingefügt.
- Detaillierte Per-Case-Diagnosen erzeugt (scripts unter /tmp/*.sh)
- Warte-/Follow-up: PR #50 beobachten; nach Merge re-run und ggf. Rust-PR vorbereiten.
