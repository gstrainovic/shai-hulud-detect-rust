# Todo List - Shai-Hulud Rust Implementation Improvements

## Ziel: Bash-Feature-Parität erreichen

### 1. ✅ COMPLETED: Strukturierte Ausgabe-Kategorien implementieren
- [x] Implementiere die kategorisierte Ausgabe wie in Bash mit klaren Trennungen
- [x] Kategorien: Malicious workflow files, Suspicious package versions, Cryptocurrency theft patterns, etc.

### 2. ✅ COMPLETED: ASCII-Box Kontextdarstellung hinzufügen  
- [x] Füge die strukturierte Darstellung mit ASCII-Boxen hinzu
- [x] Format: ┌─ File: /path │ Context: HIGH RISK: [description] └─

### 3. ✅ COMPLETED: Spezifische Pattern-Erkennung erweitern
- [x] Füge fehlende Pattern hinzu: 'Known crypto theft function names'
- [x] 'Cryptocurrency regex patterns'
- [x] Detaillierte Trufflehog-Klassifizierung

### 4. ✅ COMPLETED: Package-spezifische Details verbessern
- [x] Verbessere Package-Darstellung
- [x] Format: "Package: name@version Found in: path"

### 5. ✅ COMPLETED: Semver-Range Handling implementieren
- [x] Implementiere Semver-Range Erkennung für @operato/board@~9.0.35
- [x] Statt einzelner Versionen

### 6. ⏳ Worm-Activity Detection hinzufügen
- [ ] Füge spezifische Erkennung hinzu
- [ ] "Recently modified lockfile contains @ctrl packages (potential worm activity)"

### 7. ⏳ Output-Formatierung optimieren
- [ ] Optimiere die Gesamtstruktur der Ausgabe
- [ ] Bessere Lesbarkeit und Kategorisierung wie in der Bash-Version

### 8 weitere:
[ ] es fehlten patterns, welcher der test nicht merkte, erweitere die C:\Users\gstra\Code\shai-hulud-detect-rust\test_verification_detailed.json. 
[ ] prüfe nach weiteren tests welche in C:\Users\gstra\Code\shai-hulud-detect\shai-hulud-detector.sh drin ist aber nicht in C:\Users\gstra\Code\shai-hulud-detect-rust\test_verification_detailed.json

## Status Legend
- ✅ Completed
- ⏳ In Progress  
- ❌ Blocked
- 🔄 Testing