# Mynth Implementation Root

This directory is the reserved implementation root for Mynth. The foundation
currently defines ownership boundaries and contracts only; it does not yet
contain language source code, a tokenizer, a model, or a Cargo workspace.

- `core/` owns the future Qwen2.5-Coder-1.5B fine-tuning subsystem.
- `compiler/` owns the future Mynth language implementation.

The model emits external Mynth Representation. The compiler remains the source
of truth for syntax, semantics, types, Mynth IR, interpretation, and backend
lowering.
