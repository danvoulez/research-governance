# COMPLETENESS_LEVELS

## Objetivo
Substituir o check binario de completude por tres niveis que refletem o estado real de uma run ou ciclo.

---

## Os tres niveis

### Level 1 — Artifact completeness
Todos os artefatos minimos exigidos pelo RUN_CONTRACT existem.

**Checklist:**
- [ ] manifest existe e esta congelado
- [ ] raw output existe e esta persistido
- [ ] report existe com metricas minimas
- [ ] timestamps de inicio e fim existem
- [ ] provider/model estao identificados
- [ ] version registry foi atualizado

**Resultado:**
- PASS: todos os artefatos presentes
- FAIL: algum artefato faltando

Uma run que falha Level 1 nao pode avancar no lifecycle.

---

### Level 2 — Method completeness
O contexto metodologico esta registrado de forma suficiente para auditoria.

**Checklist:**
- [ ] session log existe para a sessao que produziu a run
- [ ] patch ledger existe se houve mudanca de codigo relevante
- [ ] cycle README esta atualizado
- [ ] manifest referencia as revisoes corretas no version registry
- [ ] expectation esta documentada e nao foi alterada silenciosamente
- [ ] baseline esta identificado se houver comparacao
- [ ] nao ha mudanca de dimensao nao registrada (CODE_CHANGE_POLICY)

**Resultado:**
- PASS: contexto completo
- PASS WITH CAVEATS: contexto parcial, caveats declarados
- FAIL: contexto insuficiente para auditoria

Uma run que falha Level 2 pode existir como collected/validated, mas nao pode ser comparada formalmente.

---

### Level 3 — Comparison readiness
A run esta pronta para entrar em comparacao formal governada.

**Checklist:**
- [ ] Level 1 PASS
- [ ] Level 2 PASS ou PASS WITH CAVEATS
- [ ] baseline de comparacao esta identificado
- [ ] dimensao variante esta isolada e registrada
- [ ] comparacao e valida conforme COMPARISON_POLICY
- [ ] nenhuma invalidacao pendente
- [ ] freeze state do ciclo permite comparacao

**Resultado:**
- PASS: pronta para comparacao
- PASS WITH CAVEATS: comparavel com ressalvas declaradas
- FAIL: nao comparavel no estado atual

---

## Como os niveis se aplicam ao ciclo

| Nivel | Aplica-se a | Quem verifica |
|---|---|---|
| Level 1 — Artifact | cada run individual | automacao (validate_run_completeness) |
| Level 2 — Method | cada run + contexto do ciclo | automacao + agente |
| Level 3 — Comparison | par de runs ou lane | agente + humano |

---

## Formato de saida do validador

O validador (validate_run_completeness) deve produzir saida estruturada:

```
## Run completeness check

### Level 1 — Artifact completeness
- status: PASS | FAIL
- missing: [lista de artefatos faltantes]

### Level 2 — Method completeness
- status: PASS | PASS WITH CAVEATS | FAIL
- caveats: [lista de caveats]
- missing: [lista de itens faltantes]

### Level 3 — Comparison readiness
- status: PASS | PASS WITH CAVEATS | FAIL
- blocking: [lista de bloqueios]
- caveats: [lista de ressalvas]
```

---

## Regra de ouro
Completude nao e so "arquivo existe".
Completude e "arquivo existe, contexto esta registrado, e a run pode ser comparada com confianca".
