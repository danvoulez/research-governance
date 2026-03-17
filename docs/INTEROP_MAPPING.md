# INTEROP_MAPPING

## Objetivo
Mostrar como o kit se organiza por dentro em funcoes canonicas e, por fora, conversa com padroes mais amplos de interoperabilidade.

---

## Visao geral
O kit e **canonico por dentro** e **interoperavel por fora**.

Isso significa:
- internamente, ele organiza o ritual em artefatos com papeis claros;
- externamente, ele pode ser descrito e exportado com estruturas reconheciveis.

---

## Mapping principal

| Elemento do kit | Funcao interna | Correspondencia externa |
|---|---|---|
| Template family | define funcoes documentais estaveis do ritual | funcao canonica de documentacao governada |
| Run manifest | congela contexto de execucao e dimensoes relevantes | campos do tipo PROV-like para entidade, atividade e contexto |
| Research package do ciclo | agrupa artefatos e evidencias | RO-Crate |
| Software metadata | descreve software, linguagem, licenca e requisitos | CodeMeta |
| Session log | registra sessao operacional do agente | log de atividade / provenance operacional |
| Patch ledger | liga evidência, mudanca e status | trilha de mudanca com proveniencia |
| Cycle README | pagina de entrada do ciclo | overview operacional do research object |

---

## 1. Template family -> funcoes canonicas
A familia de templates nao existe para padronizar texto por estetica. Ela existe para garantir que cada funcao do ritual tenha um recipiente estavel.

Exemplos:
- checklist pre-ciclo;
- canonical completeness check;
- comparative note;
- decision note;
- verify trace;
- session log;
- patch ledger;
- cycle README.

A funcao canonica e mais importante do que o phrasing local de cada arquivo.

---

## 2. Run manifest -> campos do tipo PROV
O manifest de run descreve o contexto congelado da execucao. Conceitualmente, ele se aproxima de estruturas de proveniencia:
- **entity:** inputs, corpus, engine revision, benchmark spec;
- **activity:** execucao da run;
- **agent:** humano, automacao, provider/model, executor.

O kit nao precisa implementar PROV formal completo para se beneficiar da ideia. O importante e preservar:
- quem executou;
- o que foi executado;
- com que configuracao;
- em qual ciclo/revisao;
- com quais artefatos resultantes.

---

## 3. Research package -> RO-Crate
Quando um ciclo agrega manifests, outputs, reports, notas e verificacoes, ele se aproxima naturalmente de um research object empacotavel.

RO-Crate entra aqui como formato externo util para:
- empacotamento auditavel;
- troca de artefatos;
- descricao de relacoes entre arquivos;
- reuso e arquivamento.

O kit nao substitui RO-Crate. Ele prepara o terreno para uma exportacao mais limpa em estilo RO-Crate.

---

## 4. Software metadata -> CodeMeta
`codemeta.json` cobre a camada de metadata de software:
- nome;
- descricao;
- linguagem;
- licenca;
- requisitos;
- status;
- categoria.

Isso torna o kit e seu executor mais legiveis fora do repo original.

---

## 5. Consequencia pratica
Com esse mapping, o kit ganha duas propriedades valiosas:
1. disciplina interna forte para experimentos com agentes;
2. melhor capacidade de exportacao, descricao e interoperabilidade fora do repo de origem.

---

## Regra de desenho
Por dentro, o kit deve continuar simples, auditavel e ritualizado.
Por fora, ele deve conseguir apontar para padroes reconheciveis sem virar refem deles.
