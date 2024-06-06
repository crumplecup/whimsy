use crate::prelude::{Act, Command, CommandOptions, Lens, NamedAct, State};
use polite::Polite;
use std::sync::Arc;
use wgpu::SurfaceError;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    // keyboard::{Key, ModifiersState, NamedKey},
    window::Window,
};

pub struct App {
    window: Arc<Window>,
    state: State,
    exit: bool,
}

impl App {
    pub async fn boot() -> Polite<(Self, EventLoop<()>)> {
        let event_loop = winit::event_loop::EventLoop::new()?;
        let window = winit::window::WindowBuilder::new()
            .with_title("Whimsy")
            .build(&event_loop)?;
        let window = Arc::new(window);
        let mut state = State::new(Arc::clone(&window)).await;
        if let Ok(lens) = Lens::load("data/state.data") {
            state.lens = lens;
        } else {
            tracing::info!("Could not read state from storage.");
        }
        Ok((
            Self {
                window,
                state,
                exit: false,
            },
            event_loop,
        ))
    }
    pub async fn run(mut self, event_loop: EventLoop<()>) -> Polite<()> {
        let _ = event_loop.run(move |event, ewlt| {
            ewlt.set_control_flow(ControlFlow::Wait);
            if self.exit {
                ewlt.exit()
            }

            match event {
                Event::AboutToWait => {
                    self.state.about_to_wait();
                }
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.state.window.id() => {
                    match event {
                        WindowEvent::CloseRequested => {
                            self.close_requested();
                        }
                        WindowEvent::ModifiersChanged(modifiers) => {
                            self.state.modifiers = modifiers.state();
                            tracing::trace!("Modifiers changed to {:?}", self.state.modifiers);
                        }
                        WindowEvent::KeyboardInput {
                            event,
                            is_synthetic: false,
                            ..
                        } => {
                            self.keyboard_input(event);
                        }
                        WindowEvent::Resized(physical_size) => {
                            self.state.resize(*physical_size);
                        }
                        WindowEvent::RedrawRequested => match self.state.render() {
                            Ok(_) => {}
                            Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                                self.state.resize(self.state.size)
                            }
                            Err(SurfaceError::OutOfMemory) => self.exit = true,
                            Err(SurfaceError::Timeout) => {
                                // Ignore timeouts.
                            }
                        },
                        other => {
                            self.state.handle_event(other);
                            self.window.request_redraw();
                            return;
                        }
                    };
                    self.state.handle_event(event);
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
        Ok(())
    }

    pub fn keyboard_input(&mut self, event: &KeyEvent) {
        // Dispatch actions only on press.
        if event.state.is_pressed() {
            // Interpret command.
            let command = match event.logical_key.as_ref() {
                winit::keyboard::Key::Named(k) => Some(Command::from(&k)),
                winit::keyboard::Key::Character(k) => Some(Command::new(&k, &self.state.modifiers)),
                _ => None,
            };

            // If command is valid
            if let Some(command) = command {
                tracing::trace!("{:#?}", &command);
                // Clone the command map
                let choices = self.state.command.clone();
                // Look up the current set of choices using the command key
                if let Some(choices) = choices.choices().0.get(&self.state.command_key) {
                    // Look up the command options given the current command
                    if let Some(opts) = choices.0.get(&command) {
                        match opts {
                            // If a command group, set the command key to the id of the group
                            CommandOptions::Commands(c) => {
                                tracing::trace!("Commands available: {:#?}", c);
                                self.state.command_key = c.id.clone();
                            }
                            // Take action
                            CommandOptions::Acts(a) => {
                                self.act(a);
                            }
                        }
                    } else {
                        tracing::trace!("Command not recognized.");
                    }
                }
            };
        }
    }

    pub fn act(&mut self, acts: &Vec<Act>) {
        tracing::trace!("Acts in queue: {:#?}", acts);
        // If an act, reset the command key to normal
        self.state.command_key = "normal".to_string();
        // for each act in queue
        for act in acts {
            match act {
                // dispatch to the appropriate handler
                Act::App(v) => self.state.act(v),
                Act::Egui(v) => self.state.lens.act(v),
                Act::Named(v) => {
                    tracing::trace!("{:#?}", &v);
                    match v {
                        NamedAct::Escape => {
                            self.close_requested();
                        }
                        NamedAct::Enter => self.state.lens.enter(),
                        _ => tracing::trace!("Named event detected"),
                    }
                }
                Act::Be => {
                    tracing::trace!("Taking no action.")
                }
            }
        }
    }

    pub fn close_requested(&mut self) {
        tracing::info!("Close requested.");
        let state = self.state();
        if state.lens.save("data/state.data").is_ok() {
            tracing::info!("State saved from ref.");
        } else {
            tracing::info!("Unable to save state to file.");
        }
        self.exit = true;
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn state_mut(&mut self) -> &mut State {
        &mut self.state
    }

    pub fn set_exit(set: &mut bool) {
        *set = true;
    }
}
