use crate::AuthenticationStateItem::AuthenticationStateItem;
use crate::MainMenuStateItem::MainMenuStateItem;
use crate::StartupStateItem::StartupStateItem;
use crate::StateItem::StateItem;
use crate::Transition::Transition;

pub struct StateManager{
	state: Option<Box<dyn StateItem>>,
	active: bool
}

impl StateManager{
	pub fn new() -> Self{
		let mut sm = StateManager{
			state: Some(Box::new(StartupStateItem::new())),
			active: true,
		};
		sm.setup_current_state();
		sm
	}

	fn setup_current_state(&mut self){
		if let Some(state) = &mut self.state{
			state.setup();
		}
	}

	fn transition_to(&mut self, next_state: Box<dyn StateItem>){
		if let Some(state) = &mut self.state{
			state.shutdown();
		}
		self.state = Some(next_state);
		self.setup_current_state();
	}

	pub fn run(&mut self){
		loop{
			if !self.active{
				break;
			}

			if let Some(state) = &mut self.state{
				state.display();
				state.register_input();

				if let Some(transition) = state.next_state(){

					self.transition(transition)
				}
			}
		}
	}

	fn transition(&mut self, transition: Transition){
		match transition {
			Transition::ToStartup => self.transition_to(Box::new(StartupStateItem::new())),
			Transition::ToAuthentication => self.transition_to(Box::new(AuthenticationStateItem::new())),
			Transition::ToAddEntry => todo!(),
			Transition::ToListEntries => todo!(),
			Transition::ToSearchEntry => todo!(),
			Transition::ToMainMenu => self.transition_to(Box::new(MainMenuStateItem::new())),
			Transition::ToExit => self.active = false,
		}
	}


}
