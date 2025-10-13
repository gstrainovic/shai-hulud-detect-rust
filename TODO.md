Regeln:
- Nur bash scripts verwenden, keine direkten Befehle an Bash senden
- Temporäre Scripte welche nicht wiederverwendet werden, werden direkt in /tmp erstellt. 
- Messbare Erfolge commiten mit temporären scripts.
- Aufrumen keine neuen unnötigen docs, scripts etc. 

###  ✅ Bash Bug Fix (DONE - Ready for MR)
- [x] Bug gefunden: Network exfiltration detector zu strikt
- [x] Fix implementiert in shai-hulud-detector.sh (Zeile 1255, 1264)
- [x] Merge Request Vorlage erstellt: `MERGE_REQUEST_BASH_FIX.md`
- [ ] **USER ACTION:** Merge Request in shai-hulud-detect repo erstellen

### ✅ Pattern-Level Verification Tool (COMPLETED!)
- [x] Rust Parser mit `nom` (Parser Combinators) implementiert
  - Strategie: Verwende `nom` Library für strukturiertes Parsing
  - Vorteile: Wartbar, typsicher, wiederverwendbare Parser
  - Status: ✅ **FUNKTIONIERT!**
  - Tool: `bash-log-parser/` - Standalone binary
  - Verifikation: Bash Log parsen → Rust JSON laden → fingerprint vergleichen
  - Ergebnis: ~62% match rate (15/24 findings matched)
  - Viel besser als Python Parser (der hatte 0% wegen Bugs!)
  
**Test Results (infected-project):**
  - Bash: 24 findings
  - Rust: 17 findings
  - Matches: 15 (62.5%)
  - Parser ist robust und wartbar!
  
**Usage:**
```bash
cd bash-log-parser
cargo run --release -- <bash.log> <rust.json>
```








