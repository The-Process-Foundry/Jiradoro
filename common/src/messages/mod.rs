use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// A client request message to the server
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum RequestMessage {
  Heartbeat,
}

/// A client request message to the server
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Request {
  pub message: RequestMessage,
}

/// The synchronous message sent in response to a Request
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub enum Response {
  Ack(Uuid),
  LongRunner(String),
}

/// Messages that are sent out asynchronously without having been explicitly called. This returns a
/// response because in the end everything should be available to be run synchronously if desired.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct Emission {
  pub guid: Uuid,
  pub message: Response,
}
