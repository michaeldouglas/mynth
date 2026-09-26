# Dataset Mynth Literals v0

Este diretório é o primeiro pacote de dataset da Mynth. Ele começa como um
scaffold orientado ao aprendizado: a estrutura está pronta, mas os registros
de treinamento ainda não foram escritos porque a sintaxe oficial dos
literais Mynth ainda precisa ser aprovada.

## Objetivo

Preparar exemplos de:

```text
instrução em linguagem natural -> representação externa Mynth
```

O primeiro recurso será um literal inteiro. Depois que a especificação for
aprovada, poderemos adicionar booleanos, strings e outros valores básicos em
revisões separadas.

## Estado atual

- revisão: `mynth-literals-v0-draft`;
- registros fonte: `0`;
- readiness para treinamento: `blocked`;
- motivo: sintaxe e semântica dos literais ainda não foram aprovadas;
- validação pelo compiler: indisponível;
- fine-tuning: não autorizado nem iniciado.

## Como um registro será criado

1. Definir a regra na especificação da linguagem.
2. Escrever a instrução natural.
3. Escrever a saída Mynth canônica.
4. Informar o comportamento esperado.
5. Salvar o registro em `source/records.jsonl`.
6. Validar contra o contrato do harness.
7. Validar pelo compiler quando ele suportar o recurso.
8. Derivar os splits somente depois da revisão dos registros.

O arquivo fonte deve ser tratado como imutável. Correções devem gerar uma nova
revisão ou um artefato derivado com a transformação documentada.

## Próxima tarefa

Preencher a decisão de `Mynth Literals v0` para o primeiro literal inteiro:

- forma lexical;
- tipo semântico;
- valores permitidos;
- valores inválidos;
- representação canônica;
- comportamento esperado.

Somente depois disso criaremos os primeiros registros reais.
