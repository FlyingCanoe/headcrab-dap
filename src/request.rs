#![allow(dead_code)]

use crate::{
    DataBreakpoint, ExceptionFilterOptions, ExceptionOptions, FunctionBreakpoint,
    InstructionBreakpoint, Source, SourceBreakpoint, StackFrameFormat, SteppingGranularity,
    ValueFormat,
};

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

pub struct AttachArguments {
    /**
     * Optional data from the previous, restarted session.
     * The data is sent as the 'restart' attribute of the 'terminated' event.
     * The client should leave the data intact.
     */
    restart: Option<serde_json::Value>,
}

pub enum RestartArguments {
    Launch(LaunchArguments),
    Attach(AttachArguments),
}

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

pub struct TerminateArguments {
    /**
     * A value of true indicates that this 'terminate' request is part of a
     * restart sequence.
     */
    restart: Option<bool>,
}

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

pub struct SetFunctionBreakpointsArguments {
    /**
     * The function names of the breakpoints.
     */
    breakpoints: Vec<FunctionBreakpoint>,
}

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

pub struct SetDataBreakpointsArguments {
    /**
     * The contents of this array replaces all existing data breakpoints. An empty
     * array clears all data breakpoints.
     */
    breakpoints: Vec<DataBreakpoint>,
}

pub struct SetInstructionBreakpointsArguments {
    /**
     * The instruction references of the breakpoints
     */
    breakpoints: Vec<InstructionBreakpoint>,
}

pub struct ContinueArguments {
    /**
     * Continue execution for the specified thread (if possible).
     * If the backend cannot continue on a single thread but will continue on all
     * threads, it should set the 'allThreadsContinued' attribute in the response
     * to true.
     */
    thread_id: usize,
}

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

pub struct ReverseContinueArguments {
    /**
     * Execute 'reverseContinue' for this thread.
     */
    thread_id: usize,
}

pub struct RestartFrameArguments {
    /**
     * Restart this stackframe.
     */
    frame_id: usize,
}

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

pub struct PauseArguments {
    /**
     * Pause execution for this thread.
     */
    thread_id: usize,
}

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

pub struct ScopesArguments {
    /**
     * Retrieve the scopes for this stackframe.
     */
    frame_id: usize,
}

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

pub struct TerminateThreadsArguments {
    /**
     * Ids of threads to be terminated.
     */
    thread_ids: Option<Vec<usize>>,
}

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

pub struct StepInTargetsArguments {
    /**
     * The stack frame for which to retrieve the possible stepIn targets.
     */
    frame_id: usize,
}

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
