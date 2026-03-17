# COMO_INSTALAR_E_USAR

## Objetivo
Reutilizar um kit de pesquisa governada e comparavel sem reescrever a burocracia toda.

## O que este kit faz bem
- abre ciclos governados
- congela manifestos
- gera checklist automatico
- gera canonical completeness check automatico
- gera drafts de comparative/decision note
- sincroniza index e status automaticos
- gera verify trace por ciclo
- define papeis e guardrails para agentes e humanos

## O que precisa existir no repo de destino
Este kit assume um workspace Rust com:
- `cargo`
- pasta `docs/experiments/`
- pasta `benchmarks/manifests/`
- alguma pasta de `scripts/`
- examples Rust ou executor equivalente que execute drafts/checks

## Como instalar no novo repo
1. Copie os documentos de politica para `docs/policies/`.
2. Copie os templates de ciclo para `docs/experiments/TEMPLATE_FAMILY_v1/`.
3. Copie `docs/RUST_EXPERIMENT_LOOP.md` e `docs/INTEROP_MAPPING.md` se quiser o playbook completo.
4. Copie `files/justfile` se quiser os atalhos.
5. Copie `files/codemeta.json` e ajuste os metadados do software.
6. Adapte os comandos e scripts locais para apontar para o crate executor do seu workspace.
7. Garanta que o executor consiga produzir drafts/checks automaticos sem consolidar interpretacao final humana.

## Como abrir um ciclo
Exemplo:
```bash
./scripts/new_cycle_scaffold.sh \
  --cycle-id cycle-1 \
  --title "Short title" \
  --question "What single governed question is this cycle answering?"
```

## Como refrescar e verificar
```bash
./scripts/refresh_cycle_governance.sh \
  --cycle-dir docs/experiments/cycle-1 \
  --manifest benchmarks/manifests/cycle-1-v1.json

./scripts/verify_cycle_progress.sh \
  --cycle-dir docs/experiments/cycle-1 \
  --cycle-id cycle-1 \
  --manifest benchmarks/manifests/cycle-1-v1.json \
  --verify-command "cargo test --workspace"
```

## Regras para LLMs e agentes
- nunca rode benchmark serio sem manifest congelado
- nunca misture resultados de ciclos diferentes
- se mudar metodologia, engine, benchmark spec, input protocol, provider selection, case corpus ou report schema, abra nova revisao/ciclo conforme a politica de mudanca
- mantenha docs automaticos automaticos; mantenha interpretacao humana humana
- trate provider/model como componente intercambiavel
- use session log e patch ledger para manter rastreabilidade de operacao

## Saidas esperadas por ciclo
- `PRE_CYCLE_CHECKLIST.auto.md`
- `CANONICAL_COMPLETENESS_CHECK.auto.md`
- `COMPARATIVE_NOTE.auto.md` quando houver runs suficientes
- `DECISION_NOTE.auto.md` quando houver reports
- `VERIFY_STATUS.auto.md` quando houver verificacao
- `CYCLE_STATUS.auto.md`
- `EXPERIMENT_INDEX.auto.md`
- `IMPLEMENTATION_STATUS.auto.md`

## Regra de ouro
Automatizar estrutura, rastreabilidade e disciplina.
Preservar para humanos a hipotese, a interpretacao, o freeze e a mudanca de direcao.
