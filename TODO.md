Regeln:
- Nur scripts verwenden, keine direkten Befehle an 

### 1. ✅ Bash Bug Fix (DONE - Ready for MR)
- [x] Bug gefunden: Network exfiltration detector zu strikt
- [x] Fix implementiert in shai-hulud-detector.sh (Zeile 1255, 1264)
- [ ] **USER ACTION:** Merge Request erstellen für shai-hulud-detect repo

### 2. ⏳ Cleanup scan_results.json Artifacts
- [ ] Verify kein scan_results.json bleibt im test-case directory

### 3. Analyze scripts:
- [] JSON output hinzugefügt

### 4. 100% pattern-level match zwischen Bash und Rust
- [] welche patterns hat rust alles?
- [] welche patterns hat bash alles?
- [] Erstelle einen Bash Log Parser mit Rust
   - Benutze die beste Lib
   - Wartbarkeit, Einfachheit ist wichtig, Performance, Lernkurve zweitrangig
- [] Dieser vergleicht dan es mit dem Rust Json

### Known Issues
1. **Bash Scanner Bug (Network Exfiltration):**
   - Regex pattern too strict: `[[:space:]]domain[[:space:]/\"\']`
   - Misses: `hostname: 'webhook.site'` style patterns
   - Fixed in line 1255, 1264
   - **Pending:** MR to shai-hulud-detect repo

2. **scan_results.json Artifacts:**
   - Old test runs leave JSON in test directories
   - Rust scanner picks them up
   - **Solution:** Cleanup nach jedem test (implemented, pending test)

3. **LOW RISK Findings:**
   - Bash shows only count, not individual findings
   - This is expected behavior

4. **Test Zeiten**
- Parallel normal: ~4m 4s for 26 test cases
- Parallel paranoid: ~3m 19s for 26 test cases
- [ ] Check why Parallel paranoid test is faster as Parallel normal
- Sequential: 4m 59s 
- Seq. Paranoid: 5m 17s
- [ ] Warum ist der Parallel test nicht viel schneller?

