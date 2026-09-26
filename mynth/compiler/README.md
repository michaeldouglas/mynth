# Mynth Compiler Subsystem

`compiler/` is independent from model training. It will eventually own parsing,
verification, type checking, semantic modeling, Mynth IR, reference
interpretation, and LLVM/backend lowering.

The compiler consumes external Mynth Representation and does not treat model
output, LLVM IR, or OpenVINO behavior as the definition of Mynth semantics.

Planned areas:

- `representation/`; `parser/`; `verifier/`; `type_checker/`;
- `semantic_model/`; `ir/`; `interpreter/`; `backend/`.
