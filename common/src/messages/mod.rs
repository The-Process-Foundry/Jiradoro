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

/// A wrapper to use an emission as the payload for a web event. The deserializer requires the
/// keyword payload to work as it is automatically added when throwing the Emission event.
#[derive(PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct EmissionEvent {
  pub payload: Emission,
}
