#![allow(dead_code)]

use crate::{
    DataBreakpoint, ExceptionFilterOptions, ExceptionOptions, FunctionBreakpoint,
    InstructionBreakpoint, Source, SourceBreakpoint, StackFrameFormat, SteppingGranularity,
    ValueFormat,
};

/// The ‘initialize’ request is sent as the first request from the client to the debug adapter
///
/// in order to configure it with client capabilities and to retrieve capabilities from the debug adapter.
///
/// Until the debug adapter has responded to with an ‘initialize’ response,
/// the client must not send any additional requests or events to the debug adapter.
///
/// In addition the debug adapter is not allowed to send any requests or events to the client until it has responded
/// with an ‘initialize’ response.
///
/// The ‘initialize’ request may only be sent once.
pub struct InitializeRequest(InitializeArguments);

pub struct InitializeArguments {
    /**
     * The ID of the (frontend) client using this adapter.
     */
    client_id: Option<String>,

    /**
     * The human readable name of the (frontend) client using this adapter.
     */
    client_name: Option<String>,

    /**
     * The ID of the debug adapter.
     */
    adapter_id: String,

    /**
     * The ISO-639 locale of the (frontend) client using this adapter, e.g. en-US
     * or de-CH.
     */
    locale: Option<String>,

    /**
     * If true all line numbers are 1-based (default).
     */
    lines_start_at1: Option<bool>,

    /**
     * If true all column numbers are 1-based (default).
     */
    columns_start_at1: Option<bool>,

    /**
     * Determines in what format paths are specified. The default is 'path', which
     * is the native format.
     * Values: 'path', 'uri', etc.
     */
    path_format: Option<InitializeArgumentsPathFormat>,

    /**
     * Client supports the optional type attribute for variables.
     */
    supports_variable_type: Option<bool>,

    /**
     * Client supports the paging of variables.
     */
    supports_variable_paging: Option<bool>,

    /**
     * Client supports the runInTerminal request.
     */
    supports_run_in_terminal_request: Option<bool>,

    /**
     * Client supports memory references.
     */
    supports_memory_references: Option<bool>,

    /**
     * Client supports progress reporting.
     */
    supports_progress_reporting: Option<bool>,

    /**
     * Client supports the invalidated event.
     */
    supports_invalidated_event: Option<bool>,
}

pub enum InitializeArgumentsPathFormat {
    Path,
    Uri,
    Other(String),
}

/// This optional request indicates that the client has finished initialization of the debug adapter.
///
/// So it is the last request in the sequence of configuration requests (which was started by the ‘initialized’ event).
///
/// Clients should only call this request if the capability ‘supportsConfigurationDoneRequest’ is true.
pub struct ConfigurationDoneRequest;

/// This launch request is sent from the client to the debug adapter to start the debuggee with or without debugging
/// (if ‘noDebug’ is true).
///
/// Since launching is debugger/runtime specific, the arguments for this request are not part of this specification.
pub struct LaunchRequest(LaunchArguments);

pub struct LaunchArguments {
    /**
     * If noDebug is true the launch request should launch the program without
     * enabling debugging.
     */
    no_debug: Option<bool>,

    /**
     * Optional data from the previous, restarted session.
     * The data is sent as the 'restart' attribute of the 'terminated' event.
     * The client should leave the data intact.
     */
    restart: Option<serde_json::Value>,
}

/// The attach request is sent from the client to the debug adapter to attach to a debuggee that is already running.
///
/// Since attaching is debugger/runtime specific, the arguments for this request are not part of this specification
pub struct AttachRequest(AttachArguments);

pub struct AttachArguments {
    /**
     * Optional data from the previous, restarted session.
     * The data is sent as the 'restart' attribute of the 'terminated' event.
     * The client should leave the data intact.
     */
    restart: Option<serde_json::Value>,
}

/// Restarts a debug session. Clients should only call this request if the capability ‘supportsRestartRequest’ is true.
///
/// If the capability is missing or has the value false,
/// a typical client will emulate ‘restart’ by terminating the debug adapter first and then launching it anew.
pub struct RestartRequest(Option<RestartArguments>);

pub enum RestartArguments {
    Launch(LaunchArguments),
    Attach(AttachArguments),
}

/// The ‘disconnect’ request is sent from the client to the debug adapter in order to stop debugging.
///
/// It asks the debug adapter to disconnect from the debuggee and to terminate the debug adapter.
///
/// If the debuggee has been started with the ‘launch’ request, the ‘disconnect’ request terminates the debuggee.
///
/// If the ‘attach’ request was used to connect to the debuggee, ‘disconnect’ does not terminate the debuggee.
///
/// This behavior can be controlled with the ‘terminateDebuggee’ argument (if supported by the debug adapter).
pub struct DisconnectRequest(Option<DisconnectArguments>);

pub struct DisconnectArguments {
    /**
     * A value of true indicates that this 'disconnect' request is part of a
     * restart sequence.
     */
    restart: Option<bool>,

    /**
     * Indicates whether the debuggee should be terminated when the debugger is
     * disconnected.
     * If unspecified, the debug adapter is free to do whatever it thinks is best.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportTerminateDebuggee' is true.
     */
    terminate_debuggee: Option<bool>,

    /**
     * Indicates whether the debuggee should stay suspended when the debugger is
     * disconnected.
     * If unspecified, the debuggee should resume execution.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportSuspendDebuggee' is true.
     */
    suspend_debuggee: Option<bool>,
}

/// The ‘terminate’ request is sent from the client to the debug adapter in order to give the debuggee a chance for terminating itself.
///
/// Clients should only call this request if the capability ‘supportsTerminateRequest’ is true.
pub struct TerminateRequest(Option<TerminateArguments>);

pub struct TerminateArguments {
    /**
     * A value of true indicates that this 'terminate' request is part of a
     * restart sequence.
     */
    restart: Option<bool>,
}

/// The ‘breakpointLocations’ request returns all possible locations for source breakpoints in a given range.
///
/// Clients should only call this request if the capability ‘supportsBreakpointLocationsRequest’ is true.
pub struct BreakpointLocationsRequest(Option<BreakpointLocationsArguments>);

pub struct BreakpointLocationsArguments {
    /**
     * The source location of the breakpoints; either 'source.path' or
     * 'source.reference' must be specified.
     */
    source: Source,

    /**
     * Start line of range to search possible breakpoint locations in. If only the
     * line is specified, the request returns all possible locations in that line.
     */
    line: usize,

    /**
     * Optional start column of range to search possible breakpoint locations in.
     * If no start column is given, the first column in the start line is assumed.
     */
    column: Option<usize>,

    /**
     * Optional end line of range to search possible breakpoint locations in. If
     * no end line is given, then the end line is assumed to be the start line.
     */
    end_line: Option<usize>,

    /**
     * Optional end column of range to search possible breakpoint locations in. If
     * no end column is given, then it is assumed to be in the last column of the
     * end line.
     */
    end_column: Option<usize>,
}

/// Sets multiple breakpoints for a single source and clears all previous breakpoints in that source.
///
/// To clear all breakpoint for a source, specify an empty array.
///
/// When a breakpoint is hit, a ‘stopped’ event (with reason ‘breakpoint’) is generated.
pub struct SetBreakpointsRequest(SetBreakpointsArguments);

pub struct SetBreakpointsArguments {
    /**
     * The source location of the breakpoints; either 'source.path' or
     * 'source.reference' must be specified.
     */
    source: Source,

    /**
     * The code locations of the breakpoints.
     */
    breakpoints: Option<Vec<SourceBreakpoint>>,

    /**
     * Deprecated: The code locations of the breakpoints.
     */
    lines: Option<Vec<usize>>,

    /**
     * A value of true indicates that the underlying source has been modified
     * which results in new breakpoint locations.
     */
    source_modified: Option<bool>,
}

/// Replaces all existing function breakpoints with new function breakpoints.
///
/// To clear all function breakpoints, specify an empty array.
///
/// When a function breakpoint is hit, a ‘stopped’ event (with reason ‘function breakpoint’) is generated.
///
/// Clients should only call this request if the capability ‘supportsFunctionBreakpoints’ is true.
pub struct SetFunctionBreakpointsRequest(SetFunctionBreakpointsArguments);

pub struct SetFunctionBreakpointsArguments {
    /**
     * The function names of the breakpoints.
     */
    breakpoints: Vec<FunctionBreakpoint>,
}

/// The request configures the debuggers response to thrown exceptions.
///
/// If an exception is configured to break, a ‘stopped’ event is fired (with reason ‘exception’).
///
/// Clients should only call this request if the capability ‘exceptionBreakpointFilters’ returns one or more filters.
pub struct SetExceptionBreakpointsRequest(SetExceptionBreakpointsArguments);

pub struct SetExceptionBreakpointsArguments {
    /**
     * Set of exception filters specified by their ID. The set of all possible
     * exception filters is defined by the 'exceptionBreakpointFilters'
     * capability. The 'filter' and 'filterOptions' sets are additive.
     */
    filters: Vec<String>,

    /**
     * Set of exception filters and their options. The set of all possible
     * exception filters is defined by the 'exceptionBreakpointFilters'
     * capability. This attribute is only honored by a debug adapter if the
     * capability 'supportsExceptionFilterOptions' is true. The 'filter' and
     * 'filterOptions' sets are additive.
     */
    filter_options: Option<Vec<ExceptionFilterOptions>>,

    /**
     * Configuration options for selected exceptions.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsExceptionOptions' is true.
     */
    exception_options: Option<Vec<ExceptionOptions>>,
}

/// Obtains information on a possible data breakpoint that could be set on an expression or variable.
///
/// Clients should only call this request if the capability ‘supportsDataBreakpoints’ is true.
pub struct DataBreakpointInfoRequest(DataBreakpointInfoArguments);

pub struct DataBreakpointInfoArguments {
    /**
     * Reference to the Variable container if the data breakpoint is requested for
     * a child of the container.
     */
    variables_reference: Option<usize>,

    /**
     * The name of the Variable's child to obtain data breakpoint information for.
     * If variablesReference isn’t provided, this can be an expression.
     */
    name: String,
}

/// Replaces all existing data breakpoints with new data breakpoints.
///
/// To clear all data breakpoints, specify an empty array.
///
/// When a data breakpoint is hit, a ‘stopped’ event (with reason ‘data breakpoint’) is generated.
///
/// Clients should only call this request if the capability ‘supportsDataBreakpoints’ is true.
pub struct SetDataBreakpointsRequest(SetDataBreakpointsArguments);

pub struct SetDataBreakpointsArguments {
    /**
     * The contents of this array replaces all existing data breakpoints. An empty
     * array clears all data breakpoints.
     */
    breakpoints: Vec<DataBreakpoint>,
}

/// Replaces all existing instruction breakpoints. Typically, instruction breakpoints would be set from a diassembly window.
///
/// To clear all instruction breakpoints, specify an empty array.
///
/// When an instruction breakpoint is hit, a ‘stopped’ event (with reason ‘instruction breakpoint’) is generated.
///
/// Clients should only call this request if the capability ‘supportsInstructionBreakpoints’ is true.
pub struct SetInstructionBreakpointsRequest(SetInstructionBreakpointsArguments);

pub struct SetInstructionBreakpointsArguments {
    /**
     * The instruction references of the breakpoints
     */
    breakpoints: Vec<InstructionBreakpoint>,
}

/// The request starts the debuggee to run again.
pub struct ContinueRequest(ContinueArguments);

pub struct ContinueArguments {
    /**
     * Continue execution for the specified thread (if possible).
     * If the backend cannot continue on a single thread but will continue on all
     * threads, it should set the 'allThreadsContinued' attribute in the response
     * to true.
     */
    thread_id: usize,
}

/// The request starts the debuggee to run again for one step.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘step’) after the step has completed.
pub struct NextRequest(NextArguments);

pub struct NextArguments {
    /**
     * Execute 'next' for this thread.
     */
    thread_id: usize,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    granularity: Option<SteppingGranularity>,
}

/// The request starts the debuggee to step into a function/method if possible.
///
/// If it cannot step into a target, ‘stepIn’ behaves like ‘next’.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘step’) after the step has completed.
///
/// If there are multiple function/method calls (or other targets) on the source line,
///
/// the optional argument ‘targetId’ can be used to control into which target the ‘stepIn’ should occur.
///
/// The list of possible targets for a given source line can be retrieved via the ‘stepInTargets’ request.
pub struct StepInRequest(StepInArguments);

pub struct StepInArguments {
    /**
     * Execute 'stepIn' for this thread.
     */
    thread_id: usize,

    /**
     * Optional id of the target to step into.
     */
    target_id: Option<usize>,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    granularity: Option<SteppingGranularity>,
}

/// The request starts the debuggee to run again for one step.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘step’) after the step has completed.
pub struct StepOutRequest(StepOutArguments);

pub struct StepOutArguments {
    /**
     * Execute 'stepOut' for this thread.
     */
    thread_id: usize,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    granularity: Option<SteppingGranularity>,
}

/// The request starts the debuggee to run one step backwards.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘step’) after the step has completed.
///
/// Clients should only call this request if the capability ‘supportsStepBack’ is true.
pub struct StepBackRequest(StepBackArguments);

pub struct StepBackArguments {
    /**
     * Execute 'stepBack' for this thread.
     */
    thread_id: usize,

    /**
     * Optional granularity to step. If no granularity is specified, a granularity
     * of 'statement' is assumed.
     */
    granularity: Option<SteppingGranularity>,
}

/// The request starts the debuggee to run backward.
///
/// Clients should only call this request if the capability ‘supportsStepBack’ is true.
pub struct ReverseContinueRequest(ReverseContinueArguments);

pub struct ReverseContinueArguments {
    /**
     * Execute 'reverseContinue' for this thread.
     */
    thread_id: usize,
}

/// The request restarts execution of the specified stackframe.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘restart’) after the restart has completed.
///
/// Clients should only call this request if the capability ‘supportsRestartFrame’ is true.
pub struct RestartFrameRequest(RestartFrameArguments);

pub struct RestartFrameArguments {
    /**
     * Restart this stackframe.
     */
    frame_id: usize,
}

/// he request sets the location where the debuggee will continue to run.
///
/// This makes it possible to skip the execution of code or to executed code again.
///
/// The code between the current location and the goto target is not executed but skipped.
///
/// The debug adapter first sends the response and then a ‘stopped’ event with reason ‘goto’.
///
/// Clients should only call this request if the capability ‘supportsGotoTargetsRequest’ is true (because only then goto targets exist that can be passed as arguments).
pub struct GotoRequest(GotoArguments);

pub struct GotoArguments {
    /**
     * Set the goto target for this thread.
     */
    thread_id: usize,

    /**
     * The location where the debuggee will continue to run.
     */
    target_id: usize,
}

/// The request suspends the debuggee.
///
/// The debug adapter first sends the response and then a ‘stopped’ event (with reason ‘pause’) after the thread has been paused successfully.
pub struct PauseRequest(PauseArguments);

pub struct PauseArguments {
    /**
     * Pause execution for this thread.
     */
    thread_id: usize,
}

/// The request returns a stacktrace from the current execution state of a given thread.
///
/// A client can request all stack frames by omitting the startFrame and levels arguments.
/// For performance conscious clients and if the debug adapter’s ‘supportsDelayedStackTraceLoading’ capability is true,
/// stack frames can be retrieved in a piecemeal way with the startFrame and levels arguments.
/// The response of the stackTrace request may contain a totalFrames property that hints at the total number of frames in the stack.
/// If a client needs this total number upfront,
/// it can issue a request for a single (first) frame and depending on the value of totalFrames decide how to proceed.
/// In any case a client should be prepared to receive less frames than requested, which is an indication that the end of the stack has been reached.
pub struct StackTraceRequest(StackTraceArguments);

pub struct StackTraceArguments {
    /**
     * Retrieve the stacktrace for this thread.
     */
    thread_id: usize,

    /**
     * The index of the first frame to return; if omitted frames start at 0.
     */
    start_frame: Option<usize>,

    /**
     * The maximum number of frames to return. If levels is not specified or 0,
     * all frames are returned.
     */
    levels: Option<usize>,

    /**
     * Specifies details on how to format the stack frames.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    format: Option<StackFrameFormat>,
}

/// The request returns the variable scopes for a given stackframe ID.
pub struct ScopesRequest(ScopesArguments);

pub struct ScopesArguments {
    /**
     * Retrieve the scopes for this stackframe.
     */
    frame_id: usize,
}

/// Retrieves all child variables for the given variable reference.
///
/// An optional filter can be used to limit the fetched children to either named or indexed children.
pub struct VariablesRequest(VariablesArguments);

pub struct VariablesArguments {
    /**
     * The Variable reference.
     */
    variables_reference: usize,

    /**
     * Optional filter to limit the child variables to either named or indexed. If
     * omitted, both types are fetched.
     * Values: 'indexed', 'named', etc.
     */
    filter: Option<VariablesArgumentsFilter>,

    /**
     * The index of the first variable to return; if omitted children start at 0.
     */
    start: Option<usize>,

    /**
     * The number of variables to return. If count is missing or 0, all variables
     * are returned.
     */
    count: Option<usize>,

    /**
     * Specifies details on how to format the Variable values.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    format: Option<ValueFormat>,
}

pub enum VariablesArgumentsFilter {
    Indexed,
    Named,
}

/// Set the variable with the given name in the variable container to a new value.
/// Clients should only call this request if the capability ‘supportsSetVariable’ is true.
/// 
/// If a debug adapter implements both setVariable and setExpression, a client will only use setExpression if the variable has an evaluateName property.
pub struct SetVariableRequest(SetVariableArguments);

pub struct SetVariableArguments {
    /**
     * The reference of the variable container.
     */
    variables_reference: usize,

    /**
     * The name of the variable in the container.
     */
    name: String,

    /**
     * The value of the variable.
     */
    value: String,

    /**
     * Specifies details on how to format the response value.
     */
    format: Option<ValueFormat>,
}

pub struct SourceRequest(SourceArguments);

/// The request retrieves the source code for a given source reference.
pub struct SourceArguments {
    /**
     * Specifies the source content to load. Either source.path or
     * source.sourceReference must be specified.
     */
    source: Option<Source>,

    /**
     * The reference to the source. This is the same as source.sourceReference.
     * This is provided for backward compatibility since old backends do not
     * understand the 'source' attribute.
     */
    source_reference: Option<usize>,
}

/// The request terminates the threads with the given ids.
/// 
/// Clients should only call this request if the capability ‘supportsTerminateThreadsRequest’ is true.
pub struct TerminateThreadsRequest(TerminateThreadsArguments);

pub struct TerminateThreadsArguments {
    /**
     * Ids of threads to be terminated.
     */
    thread_ids: Option<Vec<usize>>,
}

/// Modules can be retrieved from the debug adapter with this request which can either return all modules or a range of modules to support paging.
/// 
/// Clients should only call this request if the capability ‘supportsModulesRequest’ is true.
pub struct ModulesRequest(ModulesArguments);

pub struct ModulesArguments {
    /**
     * The index of the first module to return; if omitted modules start at 0.
     */
    start_module: Option<usize>,

    /**
     * The number of modules to return. If moduleCount is not specified or 0, all
     * modules are returned.
     */
    module_count: Option<usize>,
}

///Evaluates the given expression in the context of the top most stack frame.
///
///The expression has access to any variables and arguments that are in scope.
pub struct EvaluateRequest(EvaluateArguments);

pub struct EvaluateArguments {
    /**
     * The expression to evaluate.
     */
    expression: String,

    /**
     * Evaluate the expression in the scope of this stack frame. If not specified,
     * the expression is evaluated in the global scope.
     */
    frame_id: Option<usize>,

    /**
     * The context in which the evaluate request is run.
     * Values:
     * 'watch': evaluate is run in a watch.
     * 'repl': evaluate is run from REPL console.
     * 'hover': evaluate is run from a data hover.
     * 'clipboard': evaluate is run to generate the value that will be stored in
     * the clipboard.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsClipboardContext' is true.
     * etc.
     */
    context: Option<EvaluateArgumentsContext>,

    /**
     * Specifies details on how to format the Evaluate result.
     * The attribute is only honored by a debug adapter if the capability
     * 'supportsValueFormattingOptions' is true.
     */
    format: Option<ValueFormat>,
}

pub enum EvaluateArgumentsContext {
    Watch,
    Repl,
    Hover,
    Clipboard,
    Other(String),
}

/// Evaluates the given ‘value’ expression and assigns it to the ‘expression’ which must be a modifiable l-value.
/// 
/// The expressions have access to any variables and arguments that are in scope of the specified frame.
/// 
/// Clients should only call this request if the capability ‘supportsSetExpression’ is true.
/// 
/// If a debug adapter implements both setExpression and setVariable, a client will only use setExpression if the variable has an evaluateName property.
pub struct SetExpressionRequest(SetExpressionArguments);

pub struct SetExpressionArguments {
    /**
     * The l-value expression to assign to.
     */
    expression: String,

    /**
     * The value expression to assign to the l-value expression.
     */
    value: String,

    /**
     * Evaluate the expressions in the scope of this stack frame. If not
     * specified, the expressions are evaluated in the global scope.
     */
    frame_id: Option<usize>,

    /**
     * Specifies how the resulting value should be formatted.
     */
    format: Option<ValueFormat>,
}

/// This request retrieves the possible stepIn targets for the specified stack frame.
/// 
/// These targets can be used in the ‘stepIn’ request.
/// 
/// The StepInTargets may only be called if the ‘supportsStepInTargetsRequest’ capability exists and is true.
/// 
/// Clients should only call this request if the capability ‘supportsStepInTargetsRequest’ is true.
pub struct StepInTargetsRequest(StepInTargetsArguments);

pub struct StepInTargetsArguments {
    /**
     * The stack frame for which to retrieve the possible stepIn targets.
     */
    frame_id: usize,
}

/// This request retrieves the possible goto targets for the specified source location.
/// 
/// These targets can be used in the ‘goto’ request.
/// 
/// Clients should only call this request if the capability ‘supportsGotoTargetsRequest’ is true.
pub struct GotoTargetsRequest(GotoTargetsArguments);

pub struct GotoTargetsArguments {
    /**
     * The source location for which the goto targets are determined.
     */
    source: Source,

    /**
     * The line location for which the goto targets are determined.
     */
    line: usize,

    /**
     * An optional column location for which the goto targets are determined.
     */
    column: Option<usize>,
}

/// Returns a list of possible completions for a given caret position and text.
/// 
/// Clients should only call this request if the capability ‘supportsCompletionsRequest’ is true.
pub struct CompletionsRequest(CompletionsArguments);

pub struct CompletionsArguments {
    /**
     * Returns completions in the scope of this stack frame. If not specified, the
     * completions are returned for the global scope.
     */
    frame_id: Option<usize>,

    /**
     * One or more source lines. Typically this is the text a user has typed into
     * the debug console before he asked for completion.
     */
    text: String,

    /**
     * The character position for which to determine the completion proposals.
     */
    column: usize,

    /**
     * An optional line for which to determine the completion proposals. If
     * missing the first line of the text is assumed.
     */
    line: Option<usize>,
}

/// Reads bytes from memory at the provided location.
/// 
/// Clients should only call this request if the capability ‘supportsReadMemoryRequest’ is true.
pub struct ReadMemoryRequest(ReadMemoryArguments);

pub struct ReadMemoryArguments {
    /**
     * Memory reference to the base location from which data should be read.
     */
    memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * reading data. Can be negative.
     */
    offset: Option<usize>,

    /**
     * Number of bytes to read at the specified location and offset.
     */
    count: usize,
}

pub struct WriteMemoryRequest(WriteMemoryArguments);

/// Writes bytes to memory at the provided location.
/// 
/// Clients should only call this request if the capability ‘supportsWriteMemoryRequest’ is true.
pub struct WriteMemoryArguments {
    /**
     * Memory reference to the base location to which data should be written.
     */
    memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * writing data. Can be negative.
     */
    offset: Option<usize>,

    /**
     * Optional property to control partial writes. If true, the debug adapter
     * should attempt to write memory even if the entire memory region is not
     * writable. In such a case the debug adapter should stop after hitting the
     * first byte of memory that cannot be written and return the number of bytes
     * written in the response via the 'offset' and 'bytesWritten' properties.
     * If false or missing, a debug adapter should attempt to verify the region is
     * writable before writing, and fail the response if it is not.
     */
    allow_partial: Option<bool>,

    /**
     * Bytes to write, encoded using base64.
     */
    data: String,
}

/// Disassembles code stored at the provided location.
/// 
/// Clients should only call this request if the capability ‘supportsDisassembleRequest’ is true.
pub struct DisassembleRequest(DisassembleArguments);

pub struct DisassembleArguments {
    /**
     * Memory reference to the base location containing the instructions to
     * disassemble.
     */
    memory_reference: String,

    /**
     * Optional offset (in bytes) to be applied to the reference location before
     * disassembling. Can be negative.
     */
    offset: Option<usize>,

    /**
     * Optional offset (in instructions) to be applied after the byte offset (if
     * any) before disassembling. Can be negative.
     */
    instruction_offset: Option<usize>,

    /**
     * Number of instructions to disassemble starting at the specified location
     * and offset.
     * An adapter must return exactly this number of instructions - any
     * unavailable instructions should be replaced with an implementation-defined
     * 'invalid instruction' value.
     */
    instruction_count: Option<usize>,

    /**
     * If true, the adapter should attempt to resolve memory addresses and other
     * values to symbolic names.
     */
    resolve_symbols: Option<bool>,
}
