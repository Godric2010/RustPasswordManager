use crate::terminal_context::TerminalContextOld;

pub trait Widget {
	fn display(&self, context: &mut TerminalContextOld, pos_x: u16, pos_y: u16);

	fn display_as_footer(&self, context: &mut TerminalContextOld);
}