# COMPARISON_POLICY

## Objetivo
Formalizar quando uma comparacao e valida, quando e invalida, e quando exige acao humana.

Comparacao nao pode ser implicita. Ela deve ser parte explicita do metodo.

---

## Principio
Comparar so faz sentido quando os dois lados compartilham contexto suficiente. Sem isso, a comparacao produz numeros, mas nao produz conhecimento.

---

## 1. O que pode ser comparado

Duas runs podem ser comparadas diretamente quando:
- pertencem ao mesmo ciclo
- usam o mesmo manifest congelado (ou revisao compativel)
- usam o mesmo corpus
- usam o mesmo benchmark spec
- diferem em exatamente uma dimensao controlada (ex: provider/model, engine, reasoning mode)

Comparacao entre lanes do mesmo ciclo e o caso natural.

---

## 2. O que nao pode ser comparado diretamente

Duas runs **nao** podem ser comparadas diretamente quando:
- pertencem a ciclos diferentes
- usam benchmark specs diferentes
- usam corpus diferentes sem nota de compatibilidade
- usam versoes de engine que mudaram a pergunta
- tiveram expectation changes intermediarias nao registradas

Nestes casos, a comparacao so e valida com nota metodologica explicita e caveat declarado.

---

## 3. Quando comparacao e invalida

A comparacao e considerada invalida quando:
- as dimensoes comparadas nao estao congeladas de forma compativel
- o baseline foi reinterpretado retroativamente
- houve mudanca de medidor (harness, adapter, benchmark spec) sem registro
- a comparacao mistura resultados de plan e run sem distinguir

---

## 4. Quando mudanca exige novo baseline

Um novo baseline deve ser estabelecido quando:
- houve engine_revision que alterou o que esta sendo avaliado
- houve benchmark_spec_revision
- houve corpus_revision que muda a pergunta efetiva
- o baseline anterior nao e mais reproduzivel
- o baseline anterior foi invalidado por mudanca metodologica

O baseline anterior deve ser preservado para referencia historica, mas nao pode ser usado como ponto de comparacao ativo sem nota.

---

## 5. Quando comparacao deve virar decision note

Uma comparacao deve ser promovida a decision note quando:
- mostra diferenca significativa entre lanes
- sugere mudanca de direcao do ciclo
- revela falha metodologica ou incompletude
- produz evidencia suficiente para fechar o ciclo
- produz evidencia que contradiz a hipotese

Decision notes sao sempre documentos humanos (`.md`), nunca drafts automaticos (`.auto.md`) promovidos silenciosamente.

---

## 6. Campos minimos de comparacao

Toda comparacao estruturada deve incluir:
- `comparison_scope`: quais runs estao sendo comparadas
- `shared_context`: o que e identico entre as runs
- `varying_dimension`: o que difere
- `delta_vs_baseline`: diferenca em relacao ao baseline
- `delta_vs_last_verified`: diferenca em relacao a ultima verificacao
- `significance_threshold`: criterio de relevancia usado
- `noise_note`: se ha ruido ou variabilidade nao controlada
- `validity`: valid | valid_with_caveats | invalid

---

## 7. Regra de ouro
Comparacao sem contexto compartilhado e numerologia.
Comparacao com contexto compartilhado e evidencia.
