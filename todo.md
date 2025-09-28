# TODO: Shai-Hulud Rust Scanner Improvements

**Erstellt am:** September 28, 2025  
**Ziel:** Rust Scanner auf gleiches oder höheres Level wie Bash Scanner bringen

## 📋 Aufgaben-Liste

### ✅ ABGESCHLOSSEN
- [ ] Git Branch Analysis implementiert (Commit: 4852892)
- [ ] Detailed Typosquatting Analysis implementiert (Commit: e361578) 
- [ ] Specialized Network Exfiltration Checks implementiert (Commit: 578c335)

---

### 🔄 IN BEARBEITUNG

[ ] npm run test soll die tests vom ordner hai-hulud-detect\test-cases\ ausfürhen und mit test_verification_detailed.json, so war es mal, prüfe die git history.

[ ] finde fehler in C:\Users\gstra\Code\shai-hulud-detect-rust\test_verification_detailed.json die eindeutig sind und fixe es wie z.b postinstall? C:\Users\gstra\Code\shai-hulud-detect\test-cases\infected-project\package-with-postinstall.json + hätte als postinstall erkannt werden soll gemäss dateiname und dan gäbe es nicht C:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases\postinstall-hooks-test


#### 1. Log-Vergleich Test-Cases Rust vs Bash
- [ ] **Status:** ✅ ABGESCHLOSSEN - Kritische Lücken gefunde13. finde fehler in C:\Users\gstra\Code\shai-hulud-detect-rust\test_verification_detailed.json die eindeutig sind und fixe es wie z.b postinstall? C:\Users\gstra\Code\shai-hulud-detect\test-cases\infected-project\package-with-postinstall.json + hätte als postinstall erkannt werden soll gemäss dateiname und dan gäbe es nicht C:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases\postinstall-hooks-testn
- [ ] **Aufgabe:** Rust test-cases log mit `logs\bash\bash-testcase.log` vergleichen
- [ ] **Hinweis:** Bash scan NICHT ausführen (sehr langsam), nur bestehende Logs verwenden
- [ ] **Suchen:** Unterschiede in Erkennung, Formatierung, Details
- [ ] **Deliverable:** Liste der gefundenen Unterschiede

**🚨 ERGEBNIS:** 
- **?? weniger Critical Issues** im Rust Scanner (-??!)
- **Fehlende Scan-Module:** ??
- **Nächster Schritt:** TODO #2 - Diese Lücken dringend beheben

---

#### 2. Gefundene Unterschiede beheben
- [ ] **Status:** 🔥 DRINGEND - Kritische Lücken beheben
- [ ] **Aufgabe:** Alle kritischen Unterschiede aus #1 implementieren
- [ ] **FEHLENDE MODULE zu implementieren:**
  - [ ] **Postinstall Hooks Scanner** ✅ IMPLEMENTIERT - 🐛 Minor bug mit existierendem Test-Case
  - [ ] **Cryptocurrency Theft Patterns** (dedicated crypto theft detection) - 🔄 IN PROGRESS
  - [ ] **Trufflehog Activity Scanner** (secret scanning activity)
  - [ ] **Shai-Hulud Migration Patterns** (repository migration detection)
  - [ ] **Lockfile Integrity Checker** (package lock file integrity)
- [ ] **Vorgehen:** Teilerfolge → Testen → Commiten → Pushen
- [ ] **Deliverable:** Commits für jede behobene Diskrepanz

**🎯 FORTSCHRITT:** 1/5 Module implementiert (+4 Critical Issues)

---

#### 3. Report-Cleanup
- [ ] **Status:** 🟡 Abhängig von #1-2
- [ ] **Aufgabe:** `TESTCASE_COMPARISON_REPORT.md` löschen und pushen
- [ ] **Bedingung:** Nur wenn alles aus #1-2 erledigt ist
- [ ] **Deliverable:** Sauberes Repository

---

#### 4. Hardcoded Test-Bypasses prüfen
- [ ] **Status:** 🟡 Nicht gestartet  
- [ ] **Aufgabe:** Rust Code auf hardcodierte Sachen durchsuchen
- [ ] **Ziel:** Vermeiden, dass Tests nur durch Hardcoding bestehen
- [ ] **Scope:** Besonders Pattern-Matching, Compromised-Package-Lists
- [ ] **Deliverable:** Liste verdächtiger hardcodierter Werte + Fixes

---

#### 5. SCAN_COMPARISON_REPORT.md Verbesserungen  
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgabe:** Report analysieren für Rust Scanner Verbesserungen
- [ ] **Focus:** NUR Rust verbessern, Bash NICHT anfassen
- [ ] **Vorgehen:** Teilerfolge → Testen → Pushen
- [ ] **Deliverable:** Verbesserte Rust Scanner Features

---

#### 6. Bash Script Funktionen analysieren
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgabe:** Bash Script (`../shai-hulud-detect/shai-hulud-detector.sh`) durchgehen
- [ ] **Suchen:** Features die Bash besser macht als Rust
- [ ] **Implementieren:** Fehlende Funktionen in Rust
- [ ] **Vorgehen:** Teilschritte → Testen → Pushen
- [ ] **Deliverable:** Rust Scanner mit allen Bash-Features

---

#### 7. Rust-Dev Funktionen analysieren  
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgabe:** `dev-rust-scanner` Code durchgehen
- [ ] **Suchen:** Features die dort besser implementiert sind
- [ ] **Implementieren:** Überlegene Funktionen übernehmen
- [ ] **Vorgehen:** Teilschritte → Testen → Volltest nach jedem Schritt → Pushen
- [ ] **Hinweis:** Speed-Optimierungen nur zuletzt, Qualität hat Vorrang
- [ ] **Deliverable:** Rust Scanner mit besten Features beider Versionen

---

#### 8. Code-Qualität & Formatting
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgaben:** 
  - [ ] `cargo fmt` ausführen
  - [ ] Lint-Fixes anwenden  
  - [ ] `cargo check` ohne Warnungen
- [ ] **Deliverable:** Sauberer, formatierter Code

---

#### 9. Bash Script Repo Updates
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgabe:** Original Bash Repo auf Updates prüfen
- [ ] **Vorgehen:** 
  - [ ] Bash Repo pullen
  - [ ] Neue Features identifizieren
  - [ ] In Rust einbauen
  - [ ] Volltests durchführen
  - [ ] Erweitern wo sinnvoll
- [ ] **Deliverable:** Aktueller Rust Scanner mit neuesten Features

---

#### 10. Barcode-Scanner-v2 Shai-Hulud Fixes
- [ ] **Status:** 🟡 Nicht gestartet
- [ ] **Aufgabe:** Shai-Hulud Issues in `../barcode-scanner-v2` beheben
- [ ] **Scope:** Alle vom Scanner gefundenen Issues
- [ ] **Deliverable:** Sauberes barcode-scanner-v2 Projekt

#### 11. Vergleiche vet.exe results von test-cases und ../barcode-scanner-v2 mit rust und bash scanner

#### 12. Vergleiche die rust-dev outputs mit rust scanner, was macht rust-dev besser? dies übernehmen





## 📊 Fortschritts-Tracking

### Commits Timeline
- `4852892` - ✅ Git Branch Analysis 
- `e361578` - ✅ Enhanced Typosquatting Analysis
- `578c335` - ✅ Specialized Network Exfiltration Patterns

### Erfolgs-Metriken
- **Features implementiert:** 3/3 Bash Scanner Exclusive Features ✅
- **Test-Cases erstellt:** 3 (Git, Typosquatting, Network) ✅
- **Code-Coverage:** TBD
- **Performance:** TBD

---

## 🎯 Nächste Schritte

1. **SOFORT:** Log-Vergleich Rust vs Bash (#1)
2. **DANN:** Gefundene Unterschiede beheben (#2) 
3. **PARALLEL:** Hardcoded Sachen prüfen (#4)

---

## 📝 Notizen

- **Teilerfolg-Prinzip:** Jeder Schritt muss getestet und committed werden
- **Qualität vor Speed:** Optimierungen erst nach Funktionalität
- **Bash nicht ändern:** Nur Rust Scanner verbessern
- **Vollständige Tests:** Nach jedem größeren Change

---

*Letzte Aktualisierung: September 28, 2025*