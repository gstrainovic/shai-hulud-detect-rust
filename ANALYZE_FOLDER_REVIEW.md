# 📊 ANALYZE/ FOLDER ANALYSIS

## Status: Scripts in `/analyze` folder

### 1. verify_normal_mode.sh
**Status**: ✅ **DUPLIKAT**  
**Duplikat von**: `dev-rust-scanner-1/scripts/verification/verify_normal_mode.sh`  
**Aktion**: **ARCHIVIEREN** (analyze/ ist veraltet)

### 2. verify_100_percent.sh
**Status**: ⚠️ **SPEZIAL-TOOL**  
**Funktion**: Master verification mit per-testcase logs  
**Benötigt**: `parallel_testcase_scan.sh` output  
**Aktion**: **BEHALTEN & NACH scanner-1 MIGRIEREN**

### 3. parallel_testcase_scan.sh
**Status**: ⚠️ **NÜTZLICH**  
**Funktion**: Parallel Bash scanning aller test-cases mit detailliertem logging  
**Erstellt**: Per-testcase logs in `analyze/per-testcase-logs/`  
**Aktion**: **BEHALTEN & NACH scanner-1 MIGRIEREN**

### 4. generate_exact_test_expectations.sh
**Status**: ❓ **LEGACY**  
**Funktion**: Generiert erwartete Werte für dokumentierte test-cases  
**Anmerkung**: Könnte nützlich sein für Dokumentation  
**Aktion**: **OPTIONAL - kann archiviert werden**

---

## EMPFEHLUNG: MIGRATION PLAN

### Phase 1: Migrate nützliche Scripts
```bash
# Move to scanner-1
mv analyze/parallel_testcase_scan.sh dev-rust-scanner-1/scripts/verification/
mv analyze/verify_100_percent.sh dev-rust-scanner-1/scripts/verification/

# Optional
mv analyze/generate_exact_test_expectations.sh dev-rust-scanner-1/scripts/archive/
```

### Phase 2: Archive Duplikat
```bash
# verify_normal_mode.sh ist Duplikat - löschen oder archivieren
rm analyze/verify_normal_mode.sh
```

### Phase 3: Cleanup analyze/ folder
```bash
# Nach Migration: analyze/ sollte leer sein oder nur logs enthalten
```

---

## NEUE STRUKTUR (nach Migration)

```
dev-rust-scanner-1/scripts/
├── verification/
│   ├── verify_normal_mode.sh          # Existing
│   ├── verify_paranoid_mode.sh        # Existing
│   ├── final_both_modes_check.sh      # Existing
│   ├── parallel_testcase_scan.sh      # ← MIGRATED
│   └── verify_100_percent.sh          # ← MIGRATED
│
├── analysis/      # (existing 6 scripts)
├── debug/         # (existing 7 scripts)
│
└── archive/
    └── generate_exact_test_expectations.sh  # Optional legacy
```

---

## BEGRÜNDUNG

### ✅ BEHALTEN (migrieren):
1. **parallel_testcase_scan.sh**: 
   - Einzigartiges Tool
   - Erstellt detaillierte per-case logs
   - Nützlich für Debugging

2. **verify_100_percent.sh**:
   - Master verification script
   - Nutzt parallel scan results
   - Umfassender als einzelne verify scripts

### ❌ LÖSCHEN:
1. **verify_normal_mode.sh** in analyze/:
   - Exaktes Duplikat
   - Neuere Version in scanner-1/scripts/verification/

### ⚠️ OPTIONAL:
1. **generate_exact_test_expectations.sh**:
   - Legacy tool
   - Könnte für Docs nützlich sein
   - Nicht kritisch

---

## NEXT STEPS

**Soll ich die Migration durchführen?**

1. Migrate `parallel_testcase_scan.sh` → verification/
2. Migrate `verify_100_percent.sh` → verification/
3. Archive `generate_exact_test_expectations.sh` → archive/
4. Delete `verify_normal_mode.sh` (Duplikat)
5. Update `scripts/README.md` mit neuen Scripts
