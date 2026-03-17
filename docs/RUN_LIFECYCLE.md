# RUN_LIFECYCLE

## Objetivo
Definir os estados formais de uma run e as transicoes validas entre eles.

Uma run nao pode ficar "solta". Ela deve ter lifecycle rastreavel.

---

## Estados

### collected
A run foi executada e os artefatos brutos existem.
Ainda nao passou por validacao de completude.

### validated
A run passou pelo check de completude (RUN_CONTRACT).
Artefatos minimos confirmados: manifest, raw output, report, timestamps, provider/model, version registry.

### compared
A run foi incluida em pelo menos uma comparacao estruturada.
Existe referencia a ela em comparative note ou status matrix.

### triaged
A run foi avaliada metodologicamente.
Humano ou agente determinou se ela e relevante, redundante, ou problematica.
Caveats foram registrados se houver.

### frozen
A run foi congelada como evidencia oficial do ciclo.
Nao pode ser alterada sem abrir nova revisao.
Faz parte do pacote auditavel do ciclo.

### superseded
A run foi substituida por uma run mais recente ou mais completa.
Permanece no historico para referencia, mas nao e mais a referencia ativa.

---

## Transicoes validas

```
collected --> validated        (check de completude passa)
collected --> superseded       (run abandonada antes de validar)
validated --> compared         (incluida em comparacao)
validated --> superseded       (substituida antes de comparar)
compared  --> triaged          (avaliacao metodologica)
compared  --> superseded       (substituida por comparacao mais recente)
triaged   --> frozen           (aprovacao humana para freeze)
triaged   --> superseded       (substituida por run melhor)
frozen    --> superseded       (nova run congela e substitui)
```

---

## Quem pode promover cada estado

| Transicao | Quem pode fazer |
|---|---|
| collected → validated | automacao (validate_run_completeness) |
| validated → compared | automacao (ao gerar comparative note / status matrix) |
| compared → triaged | agente (com registro) ou humano |
| triaged → frozen | **humano apenas** |
| qualquer → superseded | agente (com nota) ou humano |

---

## O que exige revisao humana

- promover de triaged para frozen
- reverter de frozen para qualquer estado anterior
- declarar que uma run superseded deve voltar a ser ativa
- resolver conflito entre runs no mesmo estado

---

## Regras operacionais

### 1. Nenhuma run nasce frozen
Toda run comeca como collected. Freeze exige passar por todos os estados intermediarios.

### 2. Superseded nao e deletado
Uma run superseded permanece no historico. Ela nao perde seus artefatos.

### 3. Estado deve ser rastreavel
O estado atual da run deve aparecer em:
- TEMPLATE_CYCLE_README (campo lifecycle_state)
- STATUS_MATRIX.auto.md (coluna status)
- validate_run_completeness (output de verificacao)

### 4. Transicoes invalidas
Nao e permitido:
- pular de collected para frozen
- pular de collected para compared
- reverter de frozen sem nota humana
- promover run sem artefatos minimos

---

## Regra de ouro
Run sem lifecycle e dado solto.
Run com lifecycle e evidencia governada.
