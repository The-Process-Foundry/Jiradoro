//! The LongRunner is a asynchronous task manager for handling messages sent through the Tauri IPC
//! layer. It is meant to consolidate and simplify the communication pattern for the frontend
//! speaking to the server and vice versa.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use uuid::Uuid;

pub struct ProcessTools {
  // The receiver for incoming messages
  // A sender to emit messages to the longrunner
  // A pointer to the log
}

pub trait LongRunnerProcess {
  fn start(proc: Arc<Mutex<ProcessTools>>);
}

pub enum Action {
  /// Add a new process to the end of the run line
  Queue,
  /// Start the process on the next available worker. This will continue a paused
  Run,
  /// Stop a process in the middle of running (if allowed), or pull the process out of the queue to
  /// be run later
  Pause,
  /// Gracefully exit a process if available or remove it from the queue. If no graceful exit is
  /// configured, it will be the same as using Kill.
  Cancel,
  /// Immediately exit a process or remove it from the run queue.
  Kill,
  /// Retrieve the current status of a process
  Status,
}

pub enum Status {
  /// Waiting to be run.
  Queued,
  /// Process is currently active
  Running,
  /// Process completed successfully and a serialized result which can be retrieved.
  Finished(String),
  /// The process was paused and can be resumed
  Paused,
  /// The process was ended before it had a chance to complete
  Stopped,
  /// The process failed with an error
  Errored,
}

/// Commands that can be sent into a running process via channel
pub enum ProcessRequest {}

/// Messages that can be returned from within a running process.
pub enum ProcessEmission {
  /// Trigger sending an event with the string as the payload. The payload should be pre-serialized
  /// as nothing cares about its contents outside of the frontend listener
  Emit(String),
  /// Add the vector of strings to the log
  Log(Vec<String>),
}

/// A wrapper for describing a long running task.
pub struct Process {
  /// The unique identifier for this process to be referenced by
  guid: Uuid,
  /// The current state of the process
  status: Status,
  /// The channel to send messages into the running process
  input: Option<mpsc::Sender<ProcessRequest>>,
  /// A channel to listen for emissions from the running process
  listener: Option<mpsc::Receiver<ProcessEmission>>,
  /// A collection of log messages that may be retrieved by the frontend when desired
  log: Vec<String>,
  /// A token that is used to kill the running process
  cancellation_token: String,
}

pub struct LongRunner {
  /// The ordered set of processes waiting to be run
  queue: Mutex<Vec<Process>>,
  /// The pool of processes that have been started
  running: Mutex<HashMap<Uuid, Process>>,
  /// The pool of processes that have been paused
  paused: Mutex<HashMap<Uuid, Process>>,
  /// The pool of processes that have been completed, both success and failures.
  finished: Mutex<HashMap<Uuid, Process>>,
}

impl LongRunner {
  pub fn queue(&self) {}
}

pub mod prelude {
  pub use super::{
    Action, LongRunner, LongRunnerProcess, Process, ProcessEmission, ProcessRequest, ProcessTools,
    Status,
  };
}
