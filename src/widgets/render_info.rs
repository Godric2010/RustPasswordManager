#[derive(Clone)]
pub enum RenderStyle {
	Default,
	Bold,
	Italic,
	Inverse,
}

pub struct RenderInfo {
	pub pos_x: u16,
	pub pos_y: u16,
	pub content: String,
	pub style: RenderStyle,
}