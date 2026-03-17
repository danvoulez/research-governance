# RUST_EXPERIMENT_LOOP

## Objetivo
Definir um playbook simples para operar experimentos governados em workspaces Rust sem confundir loop de desenvolvimento, loop de verificacao e loop de benchmark.

---

## Principio central
Nem todo comando em Rust e benchmark.
Nem todo benchmark e experimento valido.
Nem toda execucao valida produz run comparavel.

O loop bom separa:
- iteracao local rapida;
- verificacao tecnica;
- execucao governada de experimento.

---

## 1. Quando usar `cargo test -p <crate>`
Use quando voce quer um loop curto, local e focado.

### Bom para:
- iterar em uma crate especifica;
- validar um patch pequeno;
- testar rapidamente um adapter, parser, rule set ou modulo isolado;
- reduzir latencia dentro da IDE.

### Nao substitui:
- verificacao de workspace inteiro;
- benchmark governado;
- evidencia comparativa oficial do ciclo.

---

## 2. Quando usar `cargo test --workspace`
Use quando voce precisa de verificacao tecnica mais ampla antes de aceitar uma mudanca como candidata a continuar no ciclo.

### Bom para:
- checar regressao global;
- validar consistencia entre crates;
- rodar verificacao antes de refresh/verify do ciclo;
- CI e ritos de consolidacao tecnica.

### Nao significa automaticamente:
- que houve run experimental completa;
- que o benchmark esta atualizado;
- que a comparabilidade metodologica foi preservada.

---

## 3. Quando usar `cargo run --example ...`
Use quando a execucao depende de examples do workspace para produzir drafts, checks ou artefatos de governanca.

### Bom para:
- `draft_governance_notes`;
- `validate_experiment_governance`;
- geradores de checklist;
- validadores canonicos;
- outros utilitarios de ritual do ciclo.

### Cuidado:
Examples podem ser infraestrutura do ritual, nao o benchmark em si. Nao confunda "rodou um example" com "executou a evidência principal do ciclo".

---

## 4. Como agentes devem operar na IDE

### Agente deve preferir loops curtos quando:
- estiver explorando patch local;
- estiver confirmando erro de compilacao;
- estiver validando mudanca pequena e isolada.

### Agente deve subir para verificacao ampla quando:
- tocar arquivos centrais;
- alterar harness, adapter ou engine;
- preparar candidate patch para consolidacao;
- encerrar sessao com trabalho relevante.

### Agente deve acionar ritual governado quando:
- houver run comparativa;
- houver refresh de ciclo;
- houver verify trace;
- houver geracao de docs automaticos oficiais do ciclo.

---

## 5. Loop recomendado de trabalho

### Loop curto
1. abrir session log;
2. editar localmente;
3. usar `cargo test -p <crate>` ou comando alvo;
4. registrar arquivos tocados e resultado.

### Loop de verificacao
1. rodar `cargo test --workspace` ou checks equivalentes;
2. registrar sucesso/falha/caveats;
3. atualizar patch ledger se houve tranche relevante.

### Loop governado do ciclo
1. confirmar manifest congelado;
2. executar run relevante;
3. rodar refresh do ciclo;
4. rodar verify do ciclo;
5. validar completude;
6. gerar ou atualizar docs automaticos.

---

## 6. Regra pratica para nao confundir benchmark com runtime
Pergunte sempre:
- este comando apenas valida software?
- este comando produz evidencia do experimento?
- esta evidencia pertence a um ciclo congelado e rastreado?

Se a resposta final for "nao", nao trate a execucao como run experimental oficial.

---

## 7. Heuristica operacional
- **Loop curto:** rapidez.
- **Workspace test:** seguranca tecnica.
- **Example governado:** infraestrutura do ritual.
- **Run do ciclo:** evidencia auditavel.
