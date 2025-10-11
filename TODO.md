Regeln:
- Nur scripts verwenden, keine direkten Befehle an Bash senden
- Temporäre Scripte welche nicht wiederverwendet werden können direkt in /tmp erstellen. 
- Jeden Teilerfolg commiten mit /tmp scripts.

###  ✅ Bash Bug Fix (DONE - Ready for MR)
- [x] Bug gefunden: Network exfiltration detector zu strikt
- [x] Fix implementiert in shai-hulud-detector.sh (Zeile 1255, 1264)
- [ ] **USER ACTION:** Merge Request erstellen für shai-hulud-detect repo

### 100% pattern-level match zwischen Bash und Rust
- [x] Erstelle einen Bash Log Parser mit Rust
   - Tool erstellt in `bash-log-parser/`
   - Wartbar und einfach mit regex + serde_json
   - Parst Bash Logs und vergleicht mit Rust JSON
- [ ] Dieser vergleicht dan es mit dem Rust Json
   - ⚠️ Aktuell einige Mismatches gefunden:
     - Compromised Packages: Bash 1, Rust 2 (Bash parsed nur 1 statt 2)
     - Trufflehog: Bash 7, Rust 0 (Rust JSON hat andere Struktur)
   - TODO: Parser verbessern für 100% accuracy

**scan_results.json Artifacts:**
   - Old test runs leave JSON in test directories
   - Rust scanner picks them up
   - [x] **Solution:** Cleanup nach jedem test (implemented and verified - no artifacts found)

4. **Test Zeiten**
- Parallel normal: ~4m 4s for 26 test cases
- Parallel paranoid: ~3m 19s for 26 test cases
- Sequential: 4m 59s 
- Seq. Paranoid: 5m 17s
- [x] Check why Parallel paranoid test is faster as Parallel normal
  - **Root Cause:** Parallel normal uses `-P 8` for Rust (resource contention)
  - **Solution:** Paranoid uses optimal `-P 4` (less contention)
- [x] Warum ist der Parallel test nicht viel schneller?
  - **Root Causes:** 
    1. Using `cargo run` instead of pre-built binary (huge overhead!)
    2. Sequential phases (Bash then Rust, not fully parallel)
    3. Amdahl's Law: I/O bottlenecks, shared resources
  - **Solutions:** See `/tmp/performance_analysis.md`
    - Use pre-built binary → 2-3x faster
    - Keep `-P 4` (already optimal)
    - Interleave Bash/Rust → further speedup
- [x] **ACTION:** Implement performance fixes
  - [x] Change parallel_testcase_scan.sh to use `-P 4` (not 8)
  - [x] Build binary once, use in scripts instead of `cargo run`
  - [x] All 4 scripts now build binary at start and use pre-built binary
  - [ ] Optional: Interleave Bash and Rust scans (future optimization)

