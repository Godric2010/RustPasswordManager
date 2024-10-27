use crate::terminal_context::TerminalContext;

pub trait Widget {
	fn display(&self, context: &mut TerminalContext, pos_x: u16, pos_y: u16);

	fn display_as_footer(&self, context: &mut TerminalContext);
}