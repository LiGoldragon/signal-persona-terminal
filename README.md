# signal-terminal

Signal contract for Persona terminal transport control.

This crate owns the typed request/event vocabulary used by Persona components
that need terminal transport and by the `terminal` transport owner.
`terminal` is the Persona-facing endpoint. `terminal-cell` remains the
low-level PTY primitive behind that owner; it is not a separate production
Signal endpoint.

It carries control records for terminal connection, input, resize, capture,
prompt pattern registration, input gate leases, write injection acknowledgements,
terminal worker lifecycle observations, and read-only session registry lookup.
Raw PTY/viewer bytes remain on the terminal data plane and are not
Signal-framed by this crate. Owner-only session lifecycle commands such as
creating or retiring sessions live in `owner-signal-terminal`, not in
this ordinary communication contract.

Each `TerminalRequest` declares its Signal root through
`TerminalRequest::signal_verb()`: write/lease creation requests use `Assert`,
resize uses `Mutate`, detach/remove/release requests use `Retract`,
capture/list reads use `Match`, and worker lifecycle streams use `Subscribe`.
