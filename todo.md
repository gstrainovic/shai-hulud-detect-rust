# Todo List - Shai-Hulud Rust Implementation Improvements

## Ziel: Bash-Feature-Parität erreichen

### 1. ✅ Strukturierte Ausgabe-Kategorien implementieren
- [ ] Implementiere die kategorisierte Ausgabe wie in Bash mit klaren Trennungen
- [ ] Kategorien: Malicious workflow files, Suspicious package versions, Cryptocurrency theft patterns, etc.

### 2. ⏳ ASCII-Box Kontextdarstellung hinzufügen  
- [ ] Füge die strukturierte Darstellung mit ASCII-Boxen hinzu
- [ ] Format: ┌─ File: /path │ Context: HIGH RISK: [description] └─

### 3. ⏳ Spezifische Pattern-Erkennung erweitern
- [ ] Füge fehlende Pattern hinzu: 'Known crypto theft function names'
- [ ] 'Cryptocurrency regex patterns'
- [ ] Detaillierte Trufflehog-Klassifizierung

### 4. ⏳ Package-spezifische Details verbessern
- [ ] Verbessere Package-Darstellung
- [ ] Format: "Package: name@version Found in: path"

### 5. ⏳ Semver-Range Handling implementieren
- [ ] Implementiere Semver-Range Erkennung für @operato/board@~9.0.35
- [ ] Statt einzelner Versionen

### 6. ⏳ Worm-Activity Detection hinzufügen
- [ ] Füge spezifische Erkennung hinzu
- [ ] "Recently modified lockfile contains @ctrl packages (potential worm activity)"

### 7. ⏳ Output-Formatierung optimieren
- [ ] Optimiere die Gesamtstruktur der Ausgabe
- [ ] Bessere Lesbarkeit und Kategorisierung wie in der Bash-Version

## Status Legend
- ✅ Completed
- ⏳ In Progress  
- ❌ Blocked
- 🔄 Testing