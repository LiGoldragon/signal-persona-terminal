# signal-persona-terminal

Signal contract for Persona terminal transport control.

This crate owns the typed request/event vocabulary used by `persona-harness`,
`terminal-cell` integration callers, and the `persona-terminal` transport
owner.

It carries control records for terminal connection, input, resize, capture,
prompt pattern registration, input gate leases, write injection acknowledgements,
and terminal worker lifecycle observations. Raw PTY/viewer bytes remain on the
terminal data plane and are not Signal-framed by this crate.
