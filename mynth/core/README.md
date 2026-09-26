# Mynth Core: Fine-Tuning Subsystem

`core/` is the ownership boundary for the Mynth model experiment. It will
eventually contain datasets, configurations, training, evaluation, and artifact
manifests for Qwen2.5-Coder-1.5B.

This foundation does not create datasets, model weights, checkpoints, training
code, or a training run. Heavy artifacts must remain outside source control.

Planned areas:

- `datasets/`: immutable sources, derived revisions, split and lineage records;
- `configs/`: reviewed LLaMA-Factory configurations;
- `training/`: authorized run orchestration;
- `evaluation/`: compiler-backed model evaluation;
- `artifacts/`: metadata and pointers, not model weights.
