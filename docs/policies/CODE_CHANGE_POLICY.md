# CODE_CHANGE_POLICY

## Objetivo
Mapear tipos de mudanca de codigo e documentacao para seus impactos obrigatorios de versao, run, lane, revisao ou ciclo.

A pergunta central deste documento e simples:
**quando uma mudanca ainda pertence ao mesmo contexto comparavel, e quando ela exige uma borda metodologica nova?**

---

## Principio: separar "mudou o sistema" de "mudou o medidor"

Mudancas no sistema avaliado (engine) e mudancas no aparato de medicao (harness, adapter, benchmark spec) tem impactos metodologicos diferentes. Confundir os dois invalida comparacoes silenciosamente.

---

## Dimensoes de impacto

- **nova run**: repetir execucao dentro do mesmo contexto congelado;
- **nova lane**: criar trilha paralela comparavel quando ha variante relevante;
- **nova revisao (`v2`, `v3`, ...)**: atualizar manifest ou artefato versionado dentro do mesmo ciclo;
- **novo ciclo**: quando a pergunta, metodologia ou base comparativa mudou o bastante para exigir novo enquadramento.

---

## Tabela de politica de mudanca

| Revision type | Exemplos | Impacto minimo | Nova run? | Nova lane? | Novo ciclo? | Invalida comparacoes? | Nota humana? |
|---|---|---|---|---|---|---|---|
| `engine_revision` | logica central, algoritmo, runtime, componente avaliado | nova lane ou novo ciclo | sim | sim, se pergunta continua | sim, se pergunta muda | sim, contra runs anteriores da mesma lane | sim |
| `harness_revision` | pipeline de execucao, flags, orchestration, timeout, coleta | nova revisao + nova run | sim | se comprometer comparabilidade | se romper comparabilidade | possivel | sim, quando afetar coleta/reproducibilidade |
| `adapter_revision` | parsing, normalizacao, schema bridge, transformacao de entrada/saida | nova revisao + nova run | sim | se mudar semantica observada | raro | possivel se mudar semantica | sim, quando afetar semantica |
| `benchmark_spec_revision` | rubrica, criterio, protocolo, scoring, schema de report | novo ciclo | sim | n/a | sim | sim, por padrao | sim |
| `corpus_revision` | adiciona/remove casos, reclassifica, troca dataset | nova revisao + nova lane | sim | sim | se invalidar pergunta | depende do grau | sim |
| `docs_revision` | ajuste textual, typo, reformulacao narrativa | nenhum, se apenas textual | nao | nao | nao | nao | sim, se consolidar interpretacao/freeze |
| `tests_revision` | novos asserts, cobertura, refactor de teste | nova run de verificacao | sim | nao | nao | nao | sim, se redefinir expectativa |
| `expectation_revision` | muda o que conta como sucesso/falha/caveat | nova revisao ou novo ciclo | sim | possivel | se romper leitura do ciclo | sim | sim, sempre |
| `provider_model_revision` | trocar fornecedor, modelo, modo de reasoning | nova lane + nova run | sim | sim | nao | nao, se lane separada | nao |
| `report_schema_revision` | muda campos, estrutura, metricas minimas | nova revisao + nova run | sim | nao | se quebrar historico | possivel | sim |
| `freeze_revision` | alterar o que esta congelado | nova revisao; possivel novo ciclo | depende | depende | se quebra metodologica | depende | sim |

---

## Regras de decisao rapida

### 1. Quando gerar nova run
Quando a mudanca nao muda a pergunta, nao redefine metodologia e apenas exige nova evidencia executada.

### 2. Quando gerar nova lane
Quando a mudanca introduz variante comparavel: novo provider/model, nova engine candidata, nova configuracao de reasoning, nova estrategia operacional dentro da mesma pergunta.

### 3. Quando gerar nova revisao
Quando artefato governado foi alterado mas o ciclo continua: manifest ajustado, expectation explicitada, freeze refinado, schema de suporte atualizado.

### 4. Quando abrir novo ciclo
Quando houver: nova pergunta, mudanca metodologica relevante, benchmark spec nova, baseline incomparavel, corpus ou expectation alterados a ponto de romper leitura continua.

---

## Regras especificas por classe

### engine_revision
- **Impacto minimo:** nova lane ou novo ciclo.
- **Exige:** patch ledger, session log, nova run.
- **Escalar:** sempre que a mudanca alterar a interpretacao do que esta sendo avaliado.
- **Invalida:** comparacoes diretas contra runs anteriores na mesma lane sem nota.

### harness_revision
- **Impacto minimo:** nova revisao e nova run.
- **Exige:** nota de impacto quando afetar coleta, timeout, orchestration ou reproducibilidade.

### adapter_revision
- **Impacto minimo:** nova revisao e nova run.
- **Exige:** registrar se a mudanca altera apenas formato ou tambem semantica.

### benchmark_spec_revision
- **Impacto minimo:** novo ciclo.
- **Exige:** evitar comparar diretamente com runs de spec anterior.

### corpus_revision
- **Impacto minimo:** nova revisao e nova lane.
- **Escalar para novo ciclo:** quando mudar a pergunta efetiva ou invalidar comparacoes anteriores.

### docs_revision
- **Impacto minimo:** nenhum, se apenas textual.
- **Exige revisao humana:** quando consolidar interpretacao, freeze ou decisao.

### tests_revision
- **Impacto minimo:** nova run de verificacao.
- **Escalar:** quando os testes redefinem o criterio do que conta como correto.

### expectation_revision
- **Impacto minimo:** nova revisao ou novo ciclo.
- **Exige:** nota explicita sempre. Nunca ajustar retroativamente sem registro.

### provider_model_revision
- **Impacto minimo:** nova lane + nova run.
- **Tratar:** como dimensao intercambiavel, mantendo rastreio em lane separada.

---

## Regra de ouro
Se a mudanca altera codigo, isso nao basta para dizer o impacto.
O impacto real e definido por **quanto a mudanca altera comparabilidade, interpretacao e sentido metodologico**.
