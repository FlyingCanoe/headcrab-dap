//! This module contain the type corresponding to the event specify in the DAP 1.48 standard.
//! The documentation in this module is adapted from the DAP 1.48 spec licence under the Creative Commons Attribution 3.0 United States License.
//! The DAP specification is available at [here](https://microsoft.github.io/debug-adapter-protocol/specification)

#![allow(dead_code)]

use crate::{Breakpoint, Capabilities, InvalidatedAreas, Module, Source};

pub enum Event {
    Initialized,
    Stopped(StoppedEvent),
    Continued(ContinuedEvent),
    Exited(ExitedEvent),
    Terminated(ThreadEvent),
    Thread(ThreadEvent),
    Output(OutputEvent),
    Breakpoint(BreakpointEvent),
    Module(ModuleEvent),
    LoadedSource(LoadedSourceEvent),
    Process(ProcessEvent),
    Capabilities(CapabilitiesEvent),
    ProgressStart(ProgressStartEvent),
    ProgressUpdate(ProgressUpdateEvent),
    ProgressEnd(ProgressUpdateEvent),
    Invalidated(InvalidatedEvent),
    Other {
        event: String,
        body: Option<serde_json::Value>,
    },
}

/// The event indicates that the execution of the debuggee has stopped due to some condition.
/// This can be caused by a break point previously set, a stepping request has completed,
/// by executing a debugger statement etc.
pub struct StoppedEvent {
    /// The reason for the event.
    /// For backward compatibility this string is shown in the UI if the
    /// 'description' attribute is missing (but it must not be translated).
    /// Values: 'step', 'breakpoint', 'exception', 'pause', 'entry', 'goto',
    /// 'function breakpoint', 'data breakpoint', 'instruction breakpoint', etc.
    reason: StoppedEventRaison,

    /// The full reason for the event, e.g. 'Paused on exception'. This string is
    /// shown in the UI as is and must be translated.
    description: Option<String>,
    /// The thread which was stopped.
    thread_id: Option<usize>,
    /// A value of true hints to the frontend that this event should not change
    /// the focus.
    preserve_focus_hint: Option<bool>,
    /// Additional information. E.g. if reason is 'exception', text contains the
    /// exception name. This string is shown in the UI.
    text: Option<String>,
    /// If 'allThreadsStopped' is true, a debug adapter can announce that all
    /// threads have stopped.
    /// - The client should use this information to enable that all threads can
    /// be expanded to access their stacktraces.
    /// - If the attribute is missing or false, only the thread with the given
    /// threadId can be expanded.
    all_threads_stopped: Option<bool>,
    /// Ids of the breakpoints that triggered the event. In most cases there will
    /// be only a single breakpoint but here are some examples for multiple
    /// breakpoints:
    /// - Different types of breakpoints map to the same location.
    /// - Multiple source breakpoints get collapsed to the same instruction by
    /// the compiler/runtime.s
    /// - Multiple function breakpoints with different function names map to the
    /// same location.
    hit_breakpoint_ids: Option<Vec<usize>>,
}

pub enum StoppedEventRaison {
    Step,
    Breakpoint,
    Exception,
    Pause,
    Entry,
    Goto,
    FunctionBreakpoint,
    DataBreakpoint,
    InstructionBreakpoint,
    Other(String),
}

/// The event indicates that the execution of the debuggee has continued.
/// Please note: a debug adapter is not expected to send this event in response
/// to a request that implies that execution continues, e.g. ‘launch’ or ‘continue’.
/// It is only necessary to send a ‘continued’ event if there was no previous request that implied this.
pub struct ContinuedEvent {
    /// The thread which was continued.
    thread_id: usize,
    /// If 'allThreadsContinued' is true, a debug adapter can announce that all
    /// threads have continued.
    all_threads_continued: Option<bool>,
}

/// The event indicates that the debuggee has exited and returns its exit code.
pub struct ExitedEvent {
    /// The exit code returned from the debuggee.
    exit_code: usize,
}

/// The event indicates that debugging of the debuggee has terminated.
/// This does not mean that the debuggee itself has exited.
pub struct TerminatedEvent {
    /// A debug adapter may set 'restart' to true (or to an arbitrary object) to
    /// request that the front end restarts the session.
    /// The value is not interpreted by the client and passed unmodified as an
    /// attribute '__restart' to the 'launch' and 'attach' requests.
    restart: Option<serde_json::Value>,
}

/// The event indicates that a thread has started or exited.
pub struct ThreadEvent {
    /// The reason for the event.
    /// Values: 'started', 'exited', etc.
    raison: ThreadEventRaison,
    /// The identifier of the thread.
    thread_id: usize,
}

pub enum ThreadEventRaison {
    Started,
    Exited,
    Other(String),
}

/// The event indicates that the target has produced some output.
pub struct OutputEvent {
    ///The output category. If not specified, 'console' is assumed.
    ///Values: 'console', 'stdout', 'stderr', 'telemetry', etc.
    category: Option<OutputEventCategory>,

    /// The output to report.
    output: String,

    /// Support for keeping an output log organized by grouping related messages.
    /// Values:
    /// 'start': Start a new group in expanded mode. Subsequent output events are
    /// members of the group and should be shown indented.
    /// The 'output' attribute becomes the name of the group and is not indented.
    /// 'startCollapsed': Start a new group in collapsed mode. Subsequent output
    /// events are members of the group and should be shown indented (as soon as
    /// the group is expanded).
    /// The 'output' attribute becomes the name of the group and is not indented.
    /// 'end': End the current group and decreases the indentation of subsequent
    /// output events.
    /// A non empty 'output' attribute is shown as the unindented end of the
    /// group.
    /// etc.
    group: Option<OutputEventGroup>,

    /// If an attribute 'variablesReference' exists and its value is > 0, the
    /// output contains objects which can be retrieved by passing
    /// 'variablesReference' to the 'variables' request. The value should be less
    /// than or equal to 2147483647 (2^31-1).
    variables_reference: Option<usize>,

    /// An optional source location where the output was produced.
    source: Option<Source>,

    /// An optional source location line where the output was produced.
    line: Option<usize>,

    /// An optional source location column where the output was produced.
    column: Option<usize>,

    /// Optional data to report. For the 'telemetry' category the data will be
    /// sent to telemetry, for the other categories the data is shown in JSON
    /// format.
    data: Option<serde_json::Value>,
}

pub enum OutputEventCategory {
    Console,
    Stdout,
    Stderr,
    Telemetry,
    Other(String),
}

pub enum OutputEventGroup {
    Start,
    StartCollapsed,
    End,
}

/// The event indicates that some information about a breakpoint has changed.
pub struct BreakpointEvent {
    /// The reason for the event.
    /// Values: 'changed', 'new', 'removed', etc.
    reason: BreakpointEventReason,

    /// The 'id' attribute is used to find the target breakpoint and the other
    /// attributes are used as the new values.
    breakpoint: Breakpoint,
}

pub enum BreakpointEventReason {
    Changed,
    New,
    Removed,
    Other(String),
}

/// The event indicates that some information about a module has changed.
pub struct ModuleEvent {
    /// The reason for the event.
    /// Values: 'new', 'changed', 'removed', etc.
    reason: ModuleEventReason,

    /// The new, changed, or removed module. In case of 'removed' only the module
    /// id is used.
    module: Module,
}

pub enum ModuleEventReason {
    New,
    Changed,
    Removed,
}

/// The event indicates that some source has been added, changed, or removed from the set of all loaded sources.
pub struct LoadedSourceEvent {
    /// The reason for the event.
    /// Values: 'new', 'changed', 'removed', etc.
    reason: LoadedSourceEventReason,

    /// The new, changed, or removed source.
    source: Source,
}

pub enum LoadedSourceEventReason {
    New,
    Changed,
    Removed,
}

/// The event indicates that some information about a breakpoint has changed.
pub struct ProcessEvent {
    /// The logical name of the process. This is usually the full path to
    /// process's executable file. Example: /home/example/myproj/program.js.
    name: String,

    /// The system process id of the debugged process. This property will be
    /// missing for non-system processes.
    system_process_id: Option<usize>,

    /// If true, the process is running on the same computer as the debug
    /// adapter.
    is_local_process: Option<bool>,

    /// Describes how the debug engine started debugging this process.
    /// Values:
    /// 'launch': Process was launched under the debugger.
    /// 'attach': Debugger attached to an existing process.
    /// 'attachForSuspendedLaunch': A project launcher component has launched a
    /// new process in a suspended state and then asked the debugger to attach.
    /// etc.
    start_method: Option<ProcessEventStartMethod>,

    /// The size of a pointer or address for this process, in bits. This value
    /// may be used by clients when formatting addresses for display.
    pointer_size: Option<usize>,
}

pub enum ProcessEventStartMethod {
    Launch,
    Attach,
    AttachForSuspendedLaunch,
}

/// The event indicates that one or more capabilities have changed.
///
/// Since the capabilities are dependent on the frontend and its UI,
/// it might not be possible to change that at random times (or too late).
///
/// Consequently this event has a hint characteristic: a frontend can only be expected to make a ‘best effort’
/// in honouring individual capabilities but there are no guarantees.
///
/// Only changed capabilities need to be included, all other capabilities keep their values.
pub struct CapabilitiesEvent {
    /// The set of updated capabilities.
    capabilities: Capabilities,
}

/// The event signals that a long running operation is about to start and
///
/// provides additional information for the client to set up a corresponding progress and cancellation UI.
///
/// The client is free to delay the showing of the UI in order to reduce flicker.
///
/// This event should only be sent if the client has passed the value true for the ‘supportsProgressReporting’ capability
/// of the ‘initialize’ request.
pub struct ProgressStartEvent {
    /// An ID that must be used in subsequent 'progressUpdate' and 'progressEnd'
    /// events to make them refer to the same progress reporting.
    /// IDs must be unique within a debug session.
    progress_id: String,

    /// Mandatory (short) title of the progress reporting. Shown in the UI to
    /// describe the long running operation.
    title: String,

    /// The request ID that this progress report is related to. If specified a
    /// debug adapter is expected to emit
    /// progress events for the long running request until the request has been
    /// either completed or cancelled.
    /// If the request ID is omitted, the progress report is assumed to be
    /// related to some general activity of the debug adapter.
    request_id: Option<usize>,

    /// If true, the request that reports progress may be canceled with a
    /// 'cancel' request.
    /// So this property basically controls whether the client should use UX that
    /// supports cancellation.
    /// Clients that don't support cancellation are allowed to ignore the
    /// setting.
    cancellable: Option<bool>,

    /// Optional, more detailed progress message.
    message: Option<String>,

    /// Optional progress percentage to display (value range: 0 to 100). If
    /// omitted no percentage will be shown.
    percentage: Option<usize>,
}

/// the event signals that the progress reporting needs to updated with a new message and/or percentage.
///
/// The client does not have to update the UI immediately,
/// but the clients needs to keep track of the message and/or percentage values.
///
/// This event should only be sent if the client has passed the value true for the ‘supportsProgressReporting’ capability
/// of the ‘initialize’ request.
pub struct ProgressUpdateEvent {
    /// The ID that was introduced in the initial 'progressStart' event.
    progress_id: String,

    /// Optional, more detailed progress message. If omitted, the previous
    /// message (if any) is used.
    message: Option<String>,

    /// Optional progress percentage to display (value range: 0 to 100). If
    /// omitted no percentage will be shown.
    percentage: Option<usize>,
}

/// The event signals the end of the progress reporting with an optional final message.
///
/// This event should only be sent if the client has passed the value true for the ‘supportsProgressReporting’ capability
/// of the ‘initialize’ request.
pub struct ProgressEndEvent {
    /// The ID that was introduced in the initial 'ProgressStartEvent'.
    progress_id: String,

    /// Optional, more detailed progress message. If omitted, the previous
    /// message (if any) is used.
    message: Option<String>,
}

///This event signals that some state in the debug adapter has changed and requires
/// that the client needs to re-render the data snapshot previously requested.
///
///Debug adapters do not have to emit this event for runtime changes
/// like stopped or thread events because in that case the client refetches the new state anyway.
/// But the event can be used for example to refresh the UI after rendering formatting has changed in the debug adapter.
///
///This event should only be sent if the debug adapter has received a value true for the ‘supportsInvalidatedEvent’
/// capability of the ‘initialize’ request.
pub struct InvalidatedEvent {
    /// Optional set of logical areas that got invalidated. This property has a
    /// hint characteristic: a client can only be expected to make a 'best
    /// effort' in honouring the areas but there are no guarantees. If this
    /// property is missing, empty, or if values are not understand the client
    /// should assume a single value 'all'.
    areas: Option<Vec<InvalidatedAreas>>,

    /// If specified, the client only needs to refetch data related to this
    /// thread.
    thread_id: Option<usize>,

    /// If specified, the client only needs to refetch data related to this stack
    /// frame (and the 'threadId' is ignored).
    stack_frame_id: Option<usize>,
}
