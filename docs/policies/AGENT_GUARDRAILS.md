# AGENT_GUARDRAILS

## Objetivo
Definir limites operacionais rigidos para agentes dentro do kit de governanca de pesquisa.

Guardrails nao existem para reduzir produtividade. Eles existem para impedir que velocidade destrua comparabilidade, rastreabilidade e sentido metodologico.

---

## Agent must not
O agente **nao deve**:

1. reinterpretar baseline retroativamente;
2. alterar expectation sem nota explicita;
3. abrir novo ciclo sem decision note ou aprovacao humana equivalente;
4. promover draft automatico a documento final sem revisao humana;
5. misturar mudancas de engine e corpus sem registrar isso explicitamente;
6. comparar runs de ciclos diferentes como se fossem diretamente equivalentes sem nota metodologica;
7. substituir ausencia de evidência por texto confiante;
8. esconder falha de verificacao ou run incompleta;
9. apagar rastros de comandos, arquivos tocados ou artefatos relevantes;
10. tratar output gerado como interpretacao humana consolidada.

---

## Agent must escalate to human
O agente **deve escalar para humano** quando houver:

1. mudanca de hipotese;
2. mudanca de baseline;
3. mudanca de expectation;
4. mudanca de corpus que afete comparabilidade;
5. mudanca de engine, harness, adapter ou benchmark spec com impacto metodologico;
6. necessidade de abrir novo ciclo ou nova revisao sem regra ja definida;
7. conflito entre resultados e narrativa anterior;
8. falta de clareza sobre se um documento automatico pode ser consolidado;
9. run relevante com artefatos incompletos;
10. qualquer situacao em que haja mais de uma interpretacao plausivel com impacto decisorio.

---

## Regras de operacao segura

### 1. Freeze first
Nenhum benchmark serio deve ser executado sem manifest congelado e identificacao clara de ciclo, lane e revisao.

### 2. Separate automatic from final
- `.auto.md` e derivado automatico.
- `.md` e documento humano consolidado.

O agente pode escrever o primeiro. O segundo exige revisao humana.

### 3. No silent methodology drift
Se mudar metodologia, benchmark spec, input protocol, report schema, corpus, engine ou criterio de avaliacao, isso deve gerar nota, impacto de versao ou novo ciclo conforme a politica de mudanca.

### 4. No hidden patching
Toda tranche de codigo com relevancia experimental deve ter session log e patch ledger suficientes para auditoria.

### 5. No fake completeness
Run parcial, run quebrada ou run sem artefatos minimos nao pode ser tratada como run completa.

---

## Politica de confianca
O agente deve preferir:
- declarar incerteza a inventar certeza;
- registrar caveat a omitir caveat;
- escalar cedo a consolidar errado.

---

## Separacao entre plano e execucao

O agente deve respeitar a fronteira entre plan e run:

- **plan** (hipotese, expectation, baseline, manifest, corpus) pertence ao enquadramento do ciclo;
- **run** (outputs, metrics, reports, traces) pertence a execucao;
- resultados de run nao podem reescrever artefatos de plan sem nota explicita e aprovacao humana;
- hipotese nao pode ser ajustada para "encaixar" nos resultados obtidos;
- expectation nao pode ser redefinida retroativamente para transformar falha em sucesso.

---

## Conteudo permitido por tipo de arquivo

### `.auto.md` (draft automatico)
- fatos extraidos de artefatos
- resumos estruturais
- status operacional
- lacunas declaradas

### `.md` (documento humano consolidado)
- interpretacao
- conclusao
- mudanca de direcao
- decisao metodologica

### Regras
- draft automatico nao pode inferir causalidade
- draft automatico nao pode promover recomendacao final sem revisao humana
- promover `.auto.md` para `.md` exige aprovacao humana explicita

---

## Frase operacional
Se a acao muda o significado do experimento, escale.
Se a acao apenas mantem o ritual, automatize.
