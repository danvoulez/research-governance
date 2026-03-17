# RUN_CONTRACT

## Objetivo
Definir o contrato minimo que qualquer run deve cumprir para ser considerada artefato completo dentro do kit de governanca.

O contrato existe para permitir varios executores (Criterion, cargo bench, nextest, custom) sem perder consistencia.

---

## Principio
Uma run so e valida se cumprir o contrato. Run parcial, run sem artefatos ou run sem rastreio nao entra no regime de comparacao.

---

## Artefatos minimos obrigatorios por run

| Artefato | Descricao | Obrigatorio |
|---|---|---|
| manifest | manifest congelado com dimensoes versionadas | sim |
| raw_output | output bruto do executor, sem pos-processamento | sim |
| report | report estruturado com metricas minimas | sim |
| timestamps | inicio e fim da execucao | sim |
| provider_model | identificacao do provider e modelo usados | sim |
| version_registry_update | registro de que version_registry foi atualizado | sim |
| ro_crate | pacote RO-Crate ou referencia a empacotamento futuro | recomendado |

---

## Campos minimos do manifest

- `cycle_id`
- `lane_id`
- `revision`
- `engine_revision`
- `harness_revision`
- `adapter_revision`
- `benchmark_spec_revision`
- `corpus_revision`
- `provider_model`
- `frozen_at` (timestamp)
- `frozen_by` (human | automation)

---

## Requisitos de identificacao

### Provider / model
- nome do provider
- nome do modelo
- versao ou data do modelo se disponivel
- modo de reasoning se aplicavel

### Timestamps
- `run_started_at`
- `run_finished_at`
- timezone ou formato UTC

### Executor
- tipo de executor (cargo bench, criterion, nextest, custom)
- versao do executor se relevante
- flags ou configuracao relevantes

---

## Outputs brutos
- devem ser persistidos sem edicao pos-execucao
- devem ser referenciados no report
- devem incluir stderr/stdout se aplicavel
- devem ser armazenados em local rastreavel por ciclo e lane

---

## Atualizacao de version registry
- o registry (`benchmarks/version_registry.json`) deve ser atualizado apos a run
- fingerprints de dimensoes devem refletir o estado real dos artefatos usados
- se houve mudanca de dimensao, o impacto deve estar registrado conforme CODE_CHANGE_POLICY

---

## Metricas minimas do report
Cada report deve incluir pelo menos:
- identificacao do caso / corpus
- verdict ou resultado por caso
- metricas de legitimidade definidas no BENCHMARK_PROTOCOL
- referencia ao manifest usado
- referencia ao raw output

---

## Pacote RO-Crate
Recomendado mas nao obrigatorio nesta fase:
- agrupar manifest, raw output, report e metadata em pacote auditavel
- seguir estrutura descrita em INTEROP_MAPPING.md

---

## O que invalida uma run

Uma run e invalida quando:
- nao tem manifest congelado
- nao tem raw output persistido
- nao tem report com metricas minimas
- nao tem timestamps
- nao identifica provider/model
- nao atualiza version registry
- mistura dimensoes de ciclos diferentes sem nota

---

## Regra de ouro
O contrato nao exige um executor especifico.
O contrato exige que qualquer executor produza artefatos suficientes para auditoria e comparacao.
