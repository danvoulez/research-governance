# TEMPLATE_STATUS_MATRIX

> Este template gera `STATUS_MATRIX.auto.md` dentro de cada ciclo.
> O arquivo gerado deve ser automatico e nao conter interpretacao humana.

---

## Formato

| lane | corpus | provider/model | reasoning | engine_revision | status | lifecycle_state | last_verified | freeze_state | delta_vs_baseline | delta_vs_last_verified | significance | noise_note | comparison_scope |
|---|---|---|---|---|---|---|---|---|---|---|---|---|---|
| _lane-id_ | _corpus-id_ | _provider/model_ | _mode_ | _revision_ | _pass/fail/partial_ | _collected/validated/compared/triaged/frozen/superseded_ | _timestamp_ | _frozen/open/pending_ | _value or n/a_ | _value or n/a_ | _above/below/within threshold_ | _note_ | _ref_ |

---

## Colunas

### Identificacao
- **lane**: identificador da lane
- **corpus**: identificador do corpus usado
- **provider/model**: provider e modelo
- **reasoning**: modo de reasoning se aplicavel
- **engine_revision**: revisao da engine conforme version registry

### Estado
- **status**: resultado operacional da run (pass / fail / partial)
- **lifecycle_state**: estado no RUN_LIFECYCLE (collected / validated / compared / triaged / frozen / superseded)
- **last_verified**: timestamp da ultima verificacao
- **freeze_state**: estado de congelamento (frozen / open / pending)

### Comparacao (delta)
- **delta_vs_baseline**: diferenca em relacao ao baseline definido
- **delta_vs_last_verified**: diferenca em relacao a ultima verificacao
- **significance**: acima / abaixo / dentro do threshold definido
- **noise_note**: observacao sobre variabilidade ou ruido
- **comparison_scope**: referencia ao par de comparacao

---

## Regras de geracao
- gerado automaticamente como `.auto.md`
- uma linha por combinacao lane x run mais recente
- atualizado pelo refresh do ciclo
- nao contem interpretacao humana
- colunas de delta preenchidas apenas quando comparacao formal existir
