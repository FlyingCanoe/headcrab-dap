use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
/// Information about the capabilities of a debug adapter.
pub struct Capabilities {
    /// The debug adapter supports the 'configurationDone' request.
    #[serde(rename = "supportsConfigurationDoneRequest")]
    supports_configuration_done_request: Option<bool>,

    /// The debug adapter supports function breakpoints.
    #[serde(rename = "supportsFunctionBreakpoints")]
    supports_function_breakpoints: Option<bool>,

    /// The debug adapter supports conditional breakpoints.
    #[serde(rename = "supportsConditionalBreakpoints")]
    supports_conditional_breakpoints: Option<bool>,

    /// The debug adapter supports breakpoints that break execution after a
    /// specified number of hits.
    #[serde(rename = "supportsHitConditionalBreakpoints")]
    supports_hit_conditional_breakpoints: Option<bool>,

    /// The debug adapter supports a (side effect free) evaluate request for data
    /// hovers.
    #[serde(rename = "supportsEvaluateForHovers")]
    supports_evaluate_for_hovers: Option<bool>,

    /// Available exception filter options for the 'setExceptionBreakpoints'
    /// request.
    #[serde(rename = "exceptionBreakpointFilters")]
    exception_breakpoint_filters: Option<Vec<ExceptionBreakpointsFilter>>,

    /// The debug adapter supports stepping back via the 'stepBack' and
    /// 'reverseContinue' requests.
    #[serde(rename = "supportsStepBack")]
    supports_step_back: Option<bool>,

    /// The debug adapter supports setting a variable to a value.
    #[serde(rename = "supportsSetVariable")]
    supports_set_variable: Option<bool>,

    /// The debug adapter supports restarting a frame.
    #[serde(rename = "supportsRestartFrame")]
    supports_restart_frame: Option<bool>,

    /// The debug adapter supports the 'gotoTargets' request.
    #[serde(rename = "supportsGotoTargetsRequest")]
    supports_goto_targets_request: Option<bool>,

    /// The debug adapter supports the 'stepInTargets' request.
    #[serde(rename = "supportsStepInTargetsRequest")]
    supports_step_in_targets_request: Option<bool>,

    /// The debug adapter supports the 'completions' request.
    #[serde(rename = "supportsCompletionsRequest")]
    supports_completions_request: Option<bool>,

    /// The set of characters that should trigger completion in a REPL. If not
    /// specified, the UI should assume the '.' character.
    #[serde(rename = "completionTriggerCharacters")]
    completion_trigger_characters: Option<Vec<String>>,

    /// The debug adapter supports the 'modules' request.
    #[serde(rename = "supportsModulesRequest")]
    supports_modules_request: Option<bool>,

    /// The set of additional module information exposed by the debug adapter.
    #[serde(rename = "additionalModuleColumns")]
    additional_module_columns: Option<Vec<ColumnDescriptor>>,

    /// Checksum algorithms supported by the debug adapter.
    #[serde(rename = "supportedChecksumAlgorithms")]
    supported_checksum_algorithms: Option<Vec<ChecksumAlgorithm>>,

    /// The debug adapter supports the 'restart' request. In this case a client
    /// should not implement 'restart' by terminating and relaunching the adapter
    /// but by calling the RestartRequest.
    #[serde(rename = "supportsRestartRequest")]
    supports_restart_request: Option<bool>,

    /// The debug adapter supports 'exceptionOptions' on the
    /// setExceptionBreakpoints request.
    #[serde(rename = "supportsExceptionOptions")]
    supports_exception_options: Option<bool>,

    /// The debug adapter supports a 'format' attribute on the stackTraceRequest,
    /// variablesRequest, and evaluateRequest.
    #[serde(rename = "supportsValueFormattingOptions")]
    supports_value_formatting_options: Option<bool>,

    /// The debug adapter supports the 'exceptionInfo' request.
    #[serde(rename = "supportsExceptionInfoRequest")]
    supports_exception_info_request: Option<bool>,

    /// The debug adapter supports the 'terminateDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(rename = "supportTerminateDebuggee")]
    support_terminate_debuggee: Option<bool>,

    /// The debug adapter supports the 'suspendDebuggee' attribute on the
    /// 'disconnect' request.
    #[serde(rename = "supportSuspendDebuggee")]
    support_suspend_debuggee: Option<bool>,

    /// The debug adapter supports the delayed loading of parts of the stack, which
    /// requires that both the 'startFrame' and 'levels' arguments and an optional
    /// 'totalFrames' result of the 'StackTrace' request are supported.
    #[serde(rename = "supportsDelayedStackTraceLoading")]
    supports_delayed_stack_trace_loading: Option<bool>,

    /// The debug adapter supports the 'loadedSources' request.
    #[serde(rename = "supportsLoadedSourcesRequest")]
    supports_loaded_sources_request: Option<bool>,

    /// The debug adapter supports logpoints by interpreting the 'logMessage'
    /// attribute of the SourceBreakpoint.
    #[serde(rename = "supportsLogPoints")]
    supports_log_points: Option<bool>,

    /// The debug adapter supports the 'terminateThreads' request.
    #[serde(rename = "supportsTerminateThreadsRequest")]
    supports_terminate_threads_request: Option<bool>,

    /// The debug adapter supports the 'setExpression' request.
    #[serde(rename = "supportsSetExpression")]
    supports_set_expression: Option<bool>,

    /// The debug adapter supports the 'terminate' request.
    #[serde(rename = "supportsTerminateRequest")]
    supports_terminate_request: Option<bool>,

    /// The debug adapter supports data breakpoints.
    #[serde(rename = "supportsDataBreakpoints")]
    supports_data_breakpoints: Option<bool>,

    /// The debug adapter supports the 'readMemory' request.
    #[serde(rename = "supportsReadMemoryRequest")]
    supports_read_memory_request: Option<bool>,

    /// The debug adapter supports the 'writeMemory' request.
    #[serde(rename = "supportsWriteMemoryRequest")]
    supports_write_memory_requestt: Option<bool>,

    /// The debug adapter supports the 'disassemble' request.
    #[serde(rename = "supportsDisassembleRequest")]
    supports_disassemble_requestt: Option<bool>,

    /// The debug adapter supports the 'cancel' request.
    #[serde(rename = "supportsCancelRequest")]
    supports_cancel_requestt: Option<bool>,

    /// The debug adapter supports the 'breakpointLocations' request.
    #[serde(rename = "supportsBreakpointLocationsRequest")]
    supports_breakpoint_locations_request: Option<bool>,

    /// The debug adapter supports the 'clipboard' context value in the 'evaluate'
    /// request.
    #[serde(rename = "supportsClipboardContext")]
    supports_clipboard_contextt: Option<bool>,

    /// The debug adapter supports stepping granularities (argument 'granularity')
    /// for the stepping requests.
    #[serde(rename = "supportsSteppingGranularity")]
    supports_stepping_granularity: Option<bool>,

    /// The debug adapter supports adding breakpoints based on instruction
    /// references.
    #[serde(rename = "supportsInstructionBreakpoints")]
    supports_instruction_breakpoints: Option<bool>,

    /// The debug adapter supports 'filterOptions' as an argument on the
    /// 'setExceptionBreakpoints' request.
    #[serde(rename = "supportsExceptionFilterOptions")]
    supports_exception_filter_options: Option<bool>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// An ExceptionBreakpointsFilter is shown in the UI as an filter option
/// for configuring how exceptions are dealt with
struct ExceptionBreakpointsFilter {/* todo */}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// A ColumnDescriptor specifies what module attribute to show in a column of the ModulesView,
/// how to format it, and what the column’s label should be.
/// It is only used if the underlying UI actually supports this level of customization.
struct ColumnDescriptor {/* todo */}

#[derive(Debug, Clone, Serialize, Deserialize)]
/// Names of checksum algorithms that may be supported by a debug adapter. Values: ‘MD5’, ‘SHA1’, ‘SHA256’, ‘timestamp’, etc.
struct ChecksumAlgorithm {/* todo */}
