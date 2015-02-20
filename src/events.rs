///struct containing event description and other event information
//TODO: add all neccessary events and convert sdl2 events to these events, to be backend agnostic
pub enum Event{
	MouseDown{x: u64, y: u64}
}
