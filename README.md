# PESQUISA-PROFISSIONAL-2026-03-17

**This package is not a benchmark. It is the governance layer around benchmarks.**

Pacote congelado com o bloco reutilizavel mais maduro da infraestrutura de pesquisa governada construida no workspace.

Este pacote **nao** substitui harness, **nao** impoe um unico executor, e **nao** automatiza decisao metodologica final.

Ele organiza:
- **freeze** de manifests e dimensoes
- **comparacao** governada entre runs
- **rastreabilidade** de sessoes, patches e artefatos
- **disciplina** de ciclo com papeis explicitos

---

## Regra fundamental de naming

- `.auto.md` = gerado automaticamente. So pode conter fatos extraidos de artefatos, resumos estruturais e status operacional. Nao pode inferir causalidade nem promover recomendacao final.
- `.md` = consolidado por humano. Pode conter interpretacao, conclusao, mudanca de direcao e decisao metodologica.

Draft automatico nao substitui revisao humana. Promover `.auto.md` para `.md` final exige aprovacao humana explicita.

---

## O que este pacote contem
- documentacao de uso e reuso
- politicas de papeis, guardrails e mudanca de codigo
- templates de sessao, patch e ciclo
- contrato de execucao de run
- politica de comparacao
- lifecycle de run
- metadados de software
- atalhos de operacao via `just`

## Arquivos principais
- `AGENTS.md` — **entry point para LLM agents** (leia primeiro)
- `COMO_INSTALAR_E_USAR.md`
- `STATUS_DE_MODULARIDADE.md`
- `KIT_CONTENTS.md`
- `KIT_ROADMAP.md`
- `install.sh`
- `docs/policies/`
- `docs/experiments/TEMPLATE_FAMILY_v1/`
- `docs/RUST_EXPERIMENT_LOOP.md`
- `docs/INTEROP_MAPPING.md`
- `docs/RUN_CONTRACT.md`
- `docs/COMPARISON_POLICY.md`
- `docs/RUN_LIFECYCLE.md`
- `docs/COMPLETENESS_LEVELS.md`
- `docs/EXECUTOR_CONTRACT.md`
- `files/`

## Posicionamento
O valor principal deste pacote nao esta em executar benchmarks sozinho.
O valor principal esta em tornar o ritual de experimentacao mais auditavel, comparavel e governado.

Este kit e modular o bastante para reuso forte em workspaces Rust parecidos, especialmente quando voce quer:
- ciclos versionados
- manifests congelados
- drafts de governance notes
- checklist e canonical check automaticos
- sync de status/index
- verify trace por ciclo
- operacao com agentes dentro de guardrails explicitos
