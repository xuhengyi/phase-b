# Capability: task-switching

- Introduced in: ch3
- Applies to: ch3, ch4, ch5, ch6, ch7, ch8

## Contract

The chapter can run multiple runnable user tasks or equivalent conformance scenarios so that all selected base success markers appear. Exact scheduling order is allowed to vary.

## Observable Requirements

- Multiple write tests can all complete.
- Later chapters can run many independent process/thread scenarios and eventually reach checker success markers.

## Out Of Scope

Fairness, timing guarantees, and exercise trace counters are not claimed by L1.
