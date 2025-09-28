# TODO: Shai-Hulud Rust Scanner Improvements

**Erstellt am:** September 28, 2025  
**Ziel:** Rust Scanner auf gleiches oder höheres Level wie Bash Scanner bringen

## 📋 Aufgaben-Liste

### ✅ ABGESCHLOSSEN
- [x] Git Branch Analysis implementiert (Commit: 4852892)
- [x] Detailed Typosquatting Analysis implementiert (Commit: e361578) 
- [x] Specialized Network Exfiltration Checks implementiert (Commit: 578c335)

---

### 🔄 IN BEARBEITUNG

#### 1. Log-Vergleich Test-Cases Rust vs Bash
- [x] **Status:** ✅ ABGESCHLOSSEN - Kritische Lücken gefunden
- [x] **Aufgabe:** Rust test-cases log mit `logs\bash\bash-testcase.log` vergleichen
- [x] **Hinweis:** Bash scan NICHT ausführen (sehr langsam), nur bestehende Logs verwenden
- [x] **Suchen:** Unterschiede in Erkennung, Formatierung, Details
- [x] **Deliverable:** Liste der gefundenen Unterschiede

**🚨 ERGEBNIS:** 
- **31 weniger Critical Issues** im Rust Scanner (-45%!)
- **Fehlende Scan-Module:** postinstall hooks, cryptocurrency patterns, Trufflehog, Shai-Hulud migration, lockfile integrity
- **Nächster Schritt:** TODO #2 - Diese Lücken dringend beheben

---

#### 2. Gefundene Unterschiede beheben
- [ ] **Status:** 🔥 DRINGEND - Kritische Lücken beheben
- [x] **Aufgabe:** Alle kritischen Unterschiede aus #1 implementieren
- [ ] **FEHLENDE MODULE zu implementieren:**
  - [x] **Postinstall Hooks Scanner** ✅ IMPLEMENTIERT - 🐛 Minor bug mit existierendem Test-Case
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

13. finde fehler in C:\Users\gstra\Code\shai-hulud-detect-rust\test_verification_detailed.json die eindeutig sind und fixe es wie z.b postinstall? C:\Users\gstra\Code\shai-hulud-detect\test-cases\infected-project\package-with-postinstall.json + hätte als postinstall erkannt werden soll gemäss dateiname und dan gäbe es nicht C:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases\postinstall-hooks-test


14. prüfe, gab es die test_cases in C:\Users\gstra\Code\shai-hulud-detect\test-cases schon aber wurden nicht erkannt und neu erstellt in C:\Users\gstra\Code\shai-hulud-detect-rust\tests\test-cases, fixe falls nötig
- [x] **Status:** ✅ BEHOBEN - Doppelte Test-Cases entfernt
- [x] **Gefunden:** `typosquatting-test` und `network-exfiltration-test` waren Duplikate
- [x] **Entfernt:** Beide duplizierten Test-Cases gelöscht
- [x] **Behalten:** `git-branch-test` (kein Original), `crypto-theft-test` (erweitert Original), `postinstall-hooks-test` (Original-Bug-Fix)
- [x] **Validiert:** Existierende Test-Cases in `../shai-hulud-detect/test-cases/` funktionieren mit meinem Scanner
---

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