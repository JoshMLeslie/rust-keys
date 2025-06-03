// see MIDI_BYTES.md
pub type MessageData = [u8; 3];
// time-stamped data
pub type Message = (u64, MessageData);
pub struct MessageLog<const L: usize> {
	pub data: [(u64, [u8; 3]); L],
}
