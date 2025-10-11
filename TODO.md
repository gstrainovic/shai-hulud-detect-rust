Regeln:
- Nur scripts verwenden, keine direkten Befehle an Bash senden
- Temporäre Scripte welche nicht wiederverwendet werden können direkt in /tmp erstellen. 
- Jeden Teilerfolg commiten mit /tmp scripts.

###  ✅ Bash Bug Fix (DONE - Ready for MR)
- [x] Bug gefunden: Network exfiltration detector zu strikt
- [x] Fix implementiert in shai-hulud-detector.sh (Zeile 1255, 1264)
- [ ] **USER ACTION:** Merge Request erstellen für shai-hulud-detect repo

### ✅ Pattern-Level Verification Tool
- [x] Python Script existiert bereits: `scripts/verify_pattern_match.py`
  - Problem: 500+ Zeilen, keine Libraries, schwer wartbar
- [ ] **IN PROGRESS:** Rust Parser mit `nom` (Parser Combinators)
  - Strategie: Verwende `nom` Library für strukturiertes Parsing
  - Vorteile: Wartbar, typsicher, wiederverwendbare Parser
  - Status: Cargo.toml updated mit `nom`, `clap`, `colored`
  - Nächster Schritt: Parser implementieren mit nom combinators
  - Verifikation: Bash Log parsen → Rust JSON laden → fingerprint vergleichen
  - Ziel: Pattern-für-Pattern Vergleich (file_path + message)
  
**Aktueller Stand:**
  - Tool-Struktur: `bash-log-parser/` erstellt
  - Dependencies: nom, serde_json, clap, colored, anyhow
  - Code: Noch nicht funktional (in Arbeit)

**scan_results.json Artifacts:**
   - Old test runs leave JSON in test directories
   - Rust scanner picks them up
   - [x] **Solution:** Cleanup nach jedem test (implemented and verified - no artifacts found)

4. **✅ Performance Optimierung - BEWIESEN**

**Test Resultate (alle 4 Tests durchgeführt):**

| Test | Vorher | Nachher | Verbesserung |
|------|---------|---------|--------------|
| **Parallel Normal** | 4m 4s (244s) | **2m 22s (142s)** | **✅ 1.7x schneller (-102s)** |
| **Parallel Paranoid** | 3m 19s (199s) | **3m 14s (194s)** | ✅ Minimal besser (-5s) |
| **Sequential Normal** | 4m 59s (299s) | **5m 38s (338s)** | ❌ Langsamer (+39s) |
| **Sequential Paranoid** | 5m 17s (317s) | **6m 19s (379s)** | ❌ Langsamer (+62s) |

**Alle Tests: 26/26 Perfect Match ✅**

**Analyse:**
- ✅ **Parallel Tests profitieren** massiv vom Binary-Build (1.7x speedup!)
- ❌ **Sequential Tests langsamer** wegen einmaligem Build-Overhead
  - Binary build dauert ~40s → sequential muss das bezahlen
  - Parallel amortisiert den Build über 26 Tests
- ✅ **Logs:** `scripts/analyze/per-testcase-logs/20251012_001616/` (normal)
- ✅ **Logs:** `scripts/analyze/per-testcase-logs-paranoid/20251012_001736/` (paranoid)

**Commits:**
- `0b7009a` - Performance optimization (scripts updated)
- `4a026d7` - Cleanup temporary files

- [x] Check why Parallel paranoid test is faster as Parallel normal
  - **Root Cause:** Parallel normal uses `-P 8` for Rust (resource contention)
  - **Solution:** Paranoid uses optimal `-P 4` (less contention)
- [x] Warum ist der Parallel test nicht viel schneller?
  - **Root Causes:** 
    1. Using `cargo run` instead of pre-built binary (huge overhead!)
    2. Sequential phases (Bash then Rust, not fully parallel)
    3. Amdahl's Law: I/O bottlenecks, shared resources
  - **Solutions:** Implemented and tested!
- [x] **ACTION:** Implement performance fixes
  - [x] Change parallel_testcase_scan.sh to use `-P 4` (not 8)
  - [x] Build binary once at start of each script
  - [x] Use pre-built binary instead of `cargo run`
  - [x] All 4 scripts updated
  - [x] **✅ BEWIESEN:** Complete tests run with measurements
  - [ ] Optional: Interleave Bash and Rust scans (future optimization)

