//! Signal contract - Persona terminal transport control plane.
//!
//! Read this file as the public interface of the terminal control plane. The
//! harness requests terminal connection, input, resize, detachment, and
//! capture. Persona-terminal also uses this vocabulary to drive terminal-cell's
//! control plane: prompt-pattern registration, input-gate leases, programmatic
//! injection, and worker lifecycle observation.
//!
//! Raw attached-viewer bytes are not Signal frames. They stay on the
//! terminal-cell data plane.
//!
//! See `ARCHITECTURE.md` for the channel's role and boundaries.

use rkyv::{Archive, Deserialize as RkyvDeserialize, Serialize as RkyvSerialize};
use signal_core::signal_channel;

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalName(String);

impl TerminalName {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalGeneration(u64);

impl TerminalGeneration {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalSequence(u64);

impl TerminalSequence {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalInputBytes(Vec<u8>);

impl TerminalInputBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct TerminalTranscriptBytes(Vec<u8>);

impl TerminalTranscriptBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalRows(u16);

impl TerminalRows {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalColumns(u16);

impl TerminalColumns {
    pub const fn new(value: u16) -> Self {
        Self(value)
    }

    pub const fn into_u16(self) -> u16 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TerminalByteCount(u64);

impl TerminalByteCount {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromptPatternId(String);

impl PromptPatternId {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PromptPatternBytes(Vec<u8>);

impl PromptPatternBytes {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self(bytes)
    }

    pub fn as_slice(&self) -> &[u8] {
        self.0.as_slice()
    }

    pub fn into_vec(self) -> Vec<u8> {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptPattern {
    LiteralSuffix(PromptPatternBytes),
    RegexSuffix { pattern: PromptPatternBytes },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct RegisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern: PromptPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct UnregisterPromptPattern {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ListPromptPatterns {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternEntry {
    pub pattern_id: PromptPatternId,
    pub pattern: PromptPattern,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternRegistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternUnregistered {
    pub terminal: TerminalName,
    pub pattern_id: PromptPatternId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct PromptPatternList {
    pub terminal: TerminalName,
    pub entries: Vec<PromptPatternEntry>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InputGateReason(String);

impl InputGateReason {
    pub fn new(value: impl Into<String>) -> Self {
        Self(value.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct InputGateLeaseId(u64);

impl InputGateLeaseId {
    pub const fn new(value: u64) -> Self {
        Self(value)
    }

    pub const fn into_u64(self) -> u64 {
        self.0
    }
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InputGateLease {
    pub id: InputGateLeaseId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum PromptState {
    NotChecked,
    Clean,
    Dirty { trailing_count: TerminalByteCount },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct AcquireInputGate {
    pub terminal: TerminalName,
    pub reason: InputGateReason,
    pub prompt_pattern_id: Option<PromptPatternId>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct ReleaseInputGate {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct WriteInjection {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub bytes: TerminalInputBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct GateAcquired {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub prompt_state: PromptState,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct GateBusy {
    pub terminal: TerminalName,
    pub current_holder: InputGateLeaseId,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct GateReleased {
    pub terminal: TerminalName,
    pub lease: InputGateLease,
    pub cached_human_bytes: TerminalByteCount,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InjectionAck {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub sequence: TerminalSequence,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct InjectionRejected {
    pub terminal: TerminalName,
    pub reason: InjectionRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum InjectionRejectionReason {
    UnknownTerminal,
    UnknownLease,
    GateNotHeld,
    DirtyPrompt,
    TransportFailed,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct SubscribeTerminalWorkerLifecycle {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TerminalWorkerKind {
    InputWriter,
    OutputFanout,
    OutputReader,
    ChildExitWatcher,
    SocketAcceptLoop,
    AttachConnectionPump,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalWorkerStopReason {
    InputCommandChannelClosed,
    InputWriteFailed(String),
    OutputCommandChannelClosed,
    OutputReaderFinished,
    OutputReadFailed(String),
    OutputPortClosed,
    ChildExited(String),
    ChildWaitFailed(String),
    SocketAcceptFailed(String),
    AttachConnectionClosed,
    AttachConnectionFailed(String),
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalWorkerLifecycle {
    Started(TerminalWorkerKind),
    Stopped {
        worker: TerminalWorkerKind,
        reason: TerminalWorkerStopReason,
    },
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalWorkerLifecycleSnapshot {
    pub terminal: TerminalName,
    pub observations: Vec<TerminalWorkerLifecycle>,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalWorkerLifecycleEvent {
    pub terminal: TerminalName,
    pub observation: TerminalWorkerLifecycle,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalConnection {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalInput {
    pub terminal: TerminalName,
    pub bytes: TerminalInputBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResize {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetachment {
    pub terminal: TerminalName,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalDetachmentReason {
    HumanRequested,
    HarnessStopped,
    ViewerReplaced,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCapture {
    pub terminal: TerminalName,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalReady {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalInputAccepted {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TranscriptDelta {
    pub terminal: TerminalName,
    pub sequence: TerminalSequence,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalResized {
    pub terminal: TerminalName,
    pub rows: TerminalRows,
    pub columns: TerminalColumns,
    pub generation: TerminalGeneration,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalCaptured {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub bytes: TerminalTranscriptBytes,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalDetached {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub reason: TerminalDetachmentReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalExited {
    pub terminal: TerminalName,
    pub generation: TerminalGeneration,
    pub status: TerminalExitStatus,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalExitStatus {
    Exited { code: i32 },
    Signaled { signal: i32 },
    StatusUnavailable,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub struct TerminalRejected {
    pub terminal: TerminalName,
    pub reason: TerminalRejectionReason,
}

#[derive(Archive, RkyvSerialize, RkyvDeserialize, Debug, Clone, PartialEq, Eq)]
pub enum TerminalRejectionReason {
    NotConnected,
    InputRejected,
    ResizeRejected,
    CaptureRejected,
    TransportFailed,
}

signal_channel! {
    request TerminalRequest {
        TerminalConnection(TerminalConnection),
        TerminalInput(TerminalInput),
        TerminalResize(TerminalResize),
        TerminalDetachment(TerminalDetachment),
        TerminalCapture(TerminalCapture),
        RegisterPromptPattern(RegisterPromptPattern),
        UnregisterPromptPattern(UnregisterPromptPattern),
        ListPromptPatterns(ListPromptPatterns),
        AcquireInputGate(AcquireInputGate),
        ReleaseInputGate(ReleaseInputGate),
        WriteInjection(WriteInjection),
        SubscribeTerminalWorkerLifecycle(SubscribeTerminalWorkerLifecycle),
    }
    reply TerminalEvent {
        TerminalReady(TerminalReady),
        TerminalInputAccepted(TerminalInputAccepted),
        TranscriptDelta(TranscriptDelta),
        TerminalResized(TerminalResized),
        TerminalCaptured(TerminalCaptured),
        TerminalDetached(TerminalDetached),
        TerminalExited(TerminalExited),
        TerminalRejected(TerminalRejected),
        PromptPatternRegistered(PromptPatternRegistered),
        PromptPatternUnregistered(PromptPatternUnregistered),
        PromptPatternList(PromptPatternList),
        GateAcquired(GateAcquired),
        GateBusy(GateBusy),
        GateReleased(GateReleased),
        InjectionAck(InjectionAck),
        InjectionRejected(InjectionRejected),
        TerminalWorkerLifecycleSnapshot(TerminalWorkerLifecycleSnapshot),
        TerminalWorkerLifecycleEvent(TerminalWorkerLifecycleEvent),
    }
}
