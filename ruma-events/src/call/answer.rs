//! Types for the *m.call.answer* event.

use js_int::UInt;
use ruma_events_macros::MessageEventContent;
use serde::{Deserialize, Serialize};

use super::SessionDescription;
use crate::MessageEvent;

/// This event is sent by the callee when they wish to answer the call.
pub type AnswerEvent = MessageEvent<AnswerEventContent>;

/// The payload for `AnswerEvent`.
#[derive(Clone, Debug, Deserialize, Serialize, MessageEventContent)]
#[ruma_event(type = "m.call.answer")]
pub struct AnswerEventContent {
    /// The VoIP session description object. The session description type must be *answer*.
    pub answer: SessionDescription,

    /// The ID of the call this event relates to.
    pub call_id: String,

    /// The version of the VoIP specification this messages adheres to.
    pub version: UInt,
}
