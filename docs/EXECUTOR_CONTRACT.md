# EXECUTOR_CONTRACT

## Objetivo
Definir o que qualquer executor deve fazer para plugar no kit de governanca, independente de linguagem, framework ou ferramenta.

O kit nao impoe um executor. O kit impoe um **contrato** que o executor deve cumprir.

---

## O que e um executor

O executor e o componente que realmente produz artefatos do ritual:
- drafts automaticos (.auto.md)
- checks de completude
- comparative notes
- decision notes
- status sync
- pre-cycle checklists
- canonical completeness checks

No workspace original, o executor sao dois examples Rust em `crates/manager-plane/examples/`:
- `draft_governance_notes.rs`
- `validate_experiment_governance.rs`

Em outro workspace, o executor pode ser:
- outro crate Rust
- um script Python
- um script shell
- um CLI em qualquer linguagem
- um LLM agent que gera os artefatos diretamente

---

## Contrato do executor

### 1. Deve gerar artefatos como `.auto.md`

Toda saida do executor e draft automatico. Nunca documento final.
- usar extensao `.auto.md`
- nao conter interpretacao, causalidade ou recomendacao final
- declarar lacunas quando informacao estiver faltando

### 2. Deve aceitar inputs estruturados

O executor deve receber:
- path do ciclo
- path do manifest
- paths de reports ou artefatos de run
- cycle-id

O formato exato dos argumentos depende da implementacao, mas os scripts do kit (`scripts/refresh_cycle_governance.sh`, etc.) assumem CLI com flags.

### 3. Deve produzir saidas previsiveis

| Operacao | Saida esperada |
|---|---|
| pre-cycle check | `PRE_CYCLE_CHECKLIST.auto.md` |
| canonical completeness check | `CANONICAL_COMPLETENESS_CHECK.auto.md` |
| comparative note | `COMPARATIVE_NOTE.auto.md` |
| decision note | `DECISION_NOTE.auto.md` |
| cycle status | `CYCLE_STATUS.auto.md` |
| experiment index | `EXPERIMENT_INDEX.auto.md` |
| implementation status | `IMPLEMENTATION_STATUS.auto.md` |
| verify status | `VERIFY_STATUS.auto.md` |
| status matrix | `STATUS_MATRIX.auto.md` |

### 4. Deve respeitar o naming do kit

- `.auto.md` para outputs automaticos
- nomes de arquivo conforme a tabela acima
- localizacao dentro de `docs/experiments/<cycle>/`

### 5. Deve ser invocavel pelos scripts

Os scripts do kit chamam o executor via:
```bash
cargo run -p <crate> --example <example> -- <subcommand> [args]
```

Se o seu executor nao for um example Rust, adapte os scripts para chamar o seu equivalente. O contrato e sobre o que entra e o que sai, nao sobre como o executor e implementado.

---

## Como escrever um executor minimo

### Opcao A — Rust example (como o original)

1. Criar um crate ou usar um existente
2. Adicionar examples que aceitem subcomandos (comparison, decision, preflight, canonical)
3. Cada subcomando recebe paths e produz `.auto.md`
4. Adaptar os scripts em `scripts/` para apontar para o crate

### Opcao B — Script Python/shell

1. Criar um script que aceite subcomandos e paths como argumentos
2. Produzir `.auto.md` no local esperado
3. Adaptar os scripts em `scripts/` para chamar o script

### Opcao C — LLM agent como executor

1. O agente le os artefatos de entrada (manifest, reports, charter, methodology)
2. Gera os `.auto.md` seguindo os templates do kit
3. Salva nos paths esperados
4. Registra a geracao no session log

Neste caso, o agente **e** o executor. Os scripts podem ser simplificados ou substituidos por instrucoes no `AGENTS.md`.

---

## Checklist de conformidade do executor

- [ ] aceita cycle-dir, manifest e reports como entrada
- [ ] produz `.auto.md` nos paths corretos
- [ ] nao produz interpretacao humana
- [ ] declara lacunas quando informacao falta
- [ ] pode ser chamado pelos scripts do kit ou por alternativa documentada
- [ ] respeita naming do kit
- [ ] outputs sao suficientes para o check de completude (COMPLETENESS_LEVELS)

---

## O que o executor nao faz

- nao decide se a run e boa ou ruim
- nao congela o ciclo
- nao muda o baseline
- nao aprova comparacoes
- nao promove `.auto.md` para `.md`

Essas acoes pertencem ao humano (ver `docs/policies/AGENT_HUMAN_ROLES.md`).

---

## Regra de ouro
O kit governa. O executor executa. O humano decide.
Qualquer componente que cumpra o contrato pode ser o executor.
