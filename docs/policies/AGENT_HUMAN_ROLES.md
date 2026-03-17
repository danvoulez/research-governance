# AGENT_HUMAN_ROLES

## Objetivo
Definir a separacao operacional entre agente, automacao e humano dentro de um ciclo de experimento governado.

Este documento existe para evitar duas falhas comuns:
1. dar autonomia demais ao agente em pontos metodologicos;
2. empurrar para humanos tarefas mecanicas que deveriam ser automaticas.

## Principio central
O agente pode operar o ritual.
O humano decide a direcao.
A automacao garante estrutura, rastreabilidade e consistencia.

---

## 1. O que o agente pode fazer
O agente pode executar trabalho operacional dentro de um ciclo ja aberto e com manifest congelado, desde que respeite guardrails e registre o que fez.

### O agente pode:
- preencher drafts estruturados a partir de artefatos existentes;
- gerar rascunhos de comparative note e decision note como `.auto.md`;
- preparar patch proposal e registrar patch ledger;
- atualizar status automaticos do ciclo;
- executar comandos previstos no protocolo do workspace;
- rodar verificacoes tecnicas e checks de completude;
- abrir session log e registrar arquivos tocados, comandos, outputs e escalacoes;
- propor, mas nao consolidar, interpretacoes sobre resultados.

### O agente pode fazer patch generation quando:
- o objetivo do patch estiver ligado a uma hipotese ou tarefa explicita;
- os arquivos alterados forem registrados;
- a dimensao afetada for classificada;
- a mudanca nao reescrever baseline silenciosamente;
- houver trilha metodologica suficiente para auditoria.

### O agente pode fazer benchmark execution quando:
- existir manifest congelado para a run;
- provider/model estiverem registrados;
- o comando estiver alinhado com o protocolo do ciclo;
- a run gerar artefatos completos ou falhar explicitamente.

### O agente pode fazer docs draft quando:
- o documento for explicitamente marcado como automatico;
- o texto for derivado de artefatos existentes;
- lacunas e incertezas forem declaradas;
- a saida nao for promovida a documento final sem revisao humana.

---

## 2. O que o agente nao pode fazer
O agente nao pode tomar decisoes metodologicas finais, reinterpretar historico, nem consolidar mudancas de sentido experimental sem rastro humano.

### O agente nao pode:
- alterar baseline retroativamente;
- mudar expectation sem nota explicita;
- abrir novo ciclo sem decisao registrada;
- promover `.auto.md` para `.md` final por conta propria;
- misturar mudanca de engine com mudanca de corpus sem registro;
- tratar comparacao parcial como conclusao final;
- apagar contexto de falha metodologica para "limpar" a historia;
- declarar freeze sem aprovacao humana.

---

## 3. O que a automacao faz
A automacao e a camada mecanica do ritual. Ela nao interpreta resultados; ela organiza e verifica.

### A automacao faz:
- scaffold de ciclo;
- refresh de status e indices;
- validacao de completude canonica;
- verificacao de existencia de manifests, reports e traces;
- geracao de arquivos `.auto.md` previstos pelo kit;
- consolidacao de matrizes, ledgers e checks estruturais;
- enforcement de naming e convencoes de pasta;
- atualizacao de registries e rastros derivados.

### A automacao nao faz:
- decidir se um resultado "vale a pena";
- aprovar mudanca de metodologia;
- interpretar trade-offs como conclusao oficial;
- substituir nota decisoria humana.

---

## 4. O que e responsabilidade humana
A responsabilidade humana cobre tudo que muda o significado do experimento, o enquadramento metodologico ou a leitura oficial dos resultados.

### E responsabilidade humana:
- formular pergunta e hipotese do ciclo;
- aprovar freeze inicial e refreezes;
- decidir abertura de novo ciclo ou nova revisao;
- aprovar expectation changes;
- aprovar corpus changes relevantes;
- interpretar resultados comparativos;
- transformar draft automatico em documento final;
- aceitar, rejeitar ou redirecionar patches propostos;
- decidir quando uma evidência e suficiente para mudar direcao.

---

## 5. Regras explicitas por area critica

### Patch generation
- **Agente:** pode propor e implementar patch dentro do escopo autorizado e com ledger.
- **Automacao:** pode validar se ha session log, patch ledger e referencias minimas.
- **Humano:** aprova patches que mudam comportamento metodologico, baseline ou direcao do ciclo.

### Benchmark execution
- **Agente:** pode rodar benchmarks previstos no protocolo e registrar a run.
- **Automacao:** valida completude da run e atualiza status derivados.
- **Humano:** decide se a run entra na narrativa comparativa oficial.

### Docs draft
- **Agente:** pode gerar `.auto.md` a partir de dados existentes.
- **Automacao:** garante naming e localizacao corretos.
- **Humano:** revisa, interpreta e consolida em `.md` final quando aplicavel.

### Freeze
- **Agente:** pode preparar checklist e proposta de freeze.
- **Automacao:** pode verificar se os artefatos necessarios existem.
- **Humano:** declara freeze, aprova quebra de freeze e define proximo estado legitimo.

### Corpus change
- **Agente:** pode detectar necessidade e preparar nota de mudanca.
- **Automacao:** pode apontar divergencia entre corpus esperado e corpus atual.
- **Humano:** aprova a mudanca e decide seu impacto metodologico.

### Expectation change
- **Agente:** pode sugerir revisao ou abrir draft de nota.
- **Automacao:** pode detectar ausencia de registro.
- **Humano:** aprova a nova expectation e decide se a comparabilidade foi preservada.

---

## 6. Separacao entre plano e execucao

O kit distingue formalmente duas fases:

### Plan / freeze contract
- pergunta do ciclo
- hipotese
- expectation
- baseline congelado
- manifest congelado
- corpus congelado

Estes artefatos pertencem ao **plano**. Eles definem o que esperamos e contra o que comparamos.

### Run / execution record
- outputs brutos
- metrics coletadas
- reports gerados
- verify traces
- session logs
- patch ledgers

Estes artefatos pertencem a **execucao**. Eles registram o que aconteceu.

### Regras de fronteira
- a hipotese nao pode ser reescrita pela execucao;
- a expectation nao pode ser ajustada retroativamente sem nota explicita e aprovacao humana;
- o baseline congelado nao pode ser reinterpretado sem abrir nova revisao ou novo ciclo;
- resultados da run nao podem alterar o plan silenciosamente.

Confundir plan com run e a origem dos piores erros metodologicos.

---

## 7. Regra operacional de ouro
O agente acelera execucao.
O humano preserva o sentido.
A automacao preserva a forma.
