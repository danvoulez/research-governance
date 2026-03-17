# KIT_CONTENTS

## O que vem no pacote
Este pacote contem a camada de governanca de pesquisa reutilizavel mais madura extraida do workspace atual.

### Conteudo principal

**Raiz**
- `README.md`
- `AGENTS.md` (entry point para LLM agents)
- `COMO_INSTALAR_E_USAR.md`
- `STATUS_DE_MODULARIDADE.md`
- `KIT_ROADMAP.md`
- `KIT_TASKLIST.md`
- `KIT_CONTENTS.md`
- `install.sh`

**Politicas** (`docs/policies/`)
- `AGENT_HUMAN_ROLES.md`
- `AGENT_GUARDRAILS.md`
- `CODE_CHANGE_POLICY.md`

**Documentos de metodo** (`docs/`)
- `RUST_EXPERIMENT_LOOP.md`
- `INTEROP_MAPPING.md`
- `RUN_CONTRACT.md`
- `COMPARISON_POLICY.md`
- `RUN_LIFECYCLE.md`
- `COMPLETENESS_LEVELS.md`
- `EXECUTOR_CONTRACT.md`

**Templates** (`docs/experiments/TEMPLATE_FAMILY_v1/`)
- `TEMPLATE_AGENT_SESSION_LOG.md`
- `TEMPLATE_PATCH_LEDGER.md`
- `TEMPLATE_CYCLE_README.md`

**Kit instalavel** (`files/`)
- `justfile`
- `codemeta.json`
- `scripts/` (4 scripts de ciclo)
- `crates/manager-plane/examples/` (2 examples Rust)
- `benchmarks/` (version registry + manifest exemplo)
- `docs/` (templates originais + novos)

## O que e obrigatorio
Para o kit ser util de forma real, o repo de destino deve ter:
- workspace Rust com `cargo`;
- uma estrategia de scripts locais;
- estrutura de docs para experimentos;
- algum executor que gere drafts/checks automaticos;
- disciplina de manifests e ciclos versionados.

## O que e opcional
- usar exatamente o mesmo nome de crate executor;
- usar exatamente os mesmos atalhos do `justfile`;
- preservar toda a estrutura original do repo fonte;
- adotar todos os templates de uma vez.

## O que pressupoe Rust / cargo / IDE
Este pacote pressupoe:
- operacao local em ambiente tecnico;
- `cargo` como base de execucao/verificacao;
- IDE ou shell como interface principal de iteracao;
- possibilidade de rodar examples, scripts e checks localmente.

## O que ainda esta acoplado ao repo original
Ainda ha acoplamento em:
- examples Rust que no repo original vivem em crate especifica;
- convencoes de pasta e nomes de comandos do workspace de origem;
- o `install.sh` copia estrutura mas nao adapta nomes de crate automaticamente.

## O que este pacote entrega de verdade
Este pacote **nao** e um benchmark.
Este pacote e a **camada de governanca ao redor de benchmarks**.

Ele entrega principalmente:
- politicas de papeis, guardrails e mudanca de codigo;
- templates de sessao, patch, ciclo e status matrix;
- contrato de execucao e politica de comparacao;
- lifecycle formal de run;
- tres niveis de completude;
- naming e convencoes de ritual;
- metadados de software;
- script de instalacao;
- atalhos de operacao via `just`.

## O que este pacote nao deve prometer
Nao prometer:
- plug-and-play universal;
- suporte agnostico a qualquer linguagem;
- instalacao completa sem adaptar executor e scripts.
