# Fonte imutável

`records.jsonl` será a fonte manual deste dataset. Cada linha deverá ser um
objeto JSON válido conforme o contrato `mynth-dataset-record.schema.json`.

Não adicionar registros antes de a revisão de linguagem
`mynth-literals-v0-draft` ser aprovada. Não preencher `mynth_output` com
placeholders, pseudocódigo ou sintaxe presumida: o campo precisa conter a
representação externa Mynth real quando o registro for criado.

Os registros rejeitados não devem ser apagados. Devem permanecer identificados
com `status: rejected` ou ser movidos para um artefato derivado documentado.
