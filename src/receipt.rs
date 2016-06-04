//! Types for the *m.receipt* event.

use std::collections::HashMap;

use core::EventType;

/// Informs the client of new receipts.
pub struct ReceiptEvent {
    content: ReceiptEventContent,
    event_type: EventType,
    room_id: String,
}

/// The payload of a `ReceiptEvent`.
///
/// A mapping of event ID to a collection of receipts for this event ID. The event ID is the ID of
/// the event being acknowledged and *not* an ID for the receipt itself.
pub type ReceiptEventContent = HashMap<String, Receipts>;

/// A collection of receipts.
pub struct Receipts {
    /// A collection of users who have sent *m.read* receipts for this event.
    m_read: UserReceipts,
}

/// A mapping of user ID to receipt.
///
/// The user ID is the entity who sent this receipt.
pub type UserReceipts = HashMap<String, Receipt>;

/// An acknowledgement of an event.
pub struct Receipt {
    /// The timestamp the receipt was sent at.
    ts: u64,
}
