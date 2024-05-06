use crate::prelude::{Act, Action, App, Choices, Command, CommandOptions, Modifiers, NamedAct};
use std::sync::Arc;
use wgpu::SurfaceError;
use winit::{
    event::{Event, KeyEvent, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    keyboard::{Key, ModifiersState, NamedKey},
    window::Window,
};

pub async fn run(window: Window, event_loop: EventLoop<()>) {
    let window = Arc::new(window);

    let mut state = App::new(Arc::clone(&window)).await;
    let mut exit = false;

    let _ = event_loop.run(move |event, ewlt| {
        ewlt.set_control_flow(ControlFlow::Wait);

        match event {
            Event::AboutToWait => {
                state.about_to_wait();
            }
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == state.window.id() => {
                match event {
                    WindowEvent::CloseRequested => ewlt.exit(),
                    // | WindowEvent::KeyboardInput {
                    //     event:
                    //         KeyEvent {
                    //             logical_key: Key::Named(NamedKey::Escape),
                    //             ..
                    //         },
                    //     ..
                    // } => ewlt.exit(),
                    WindowEvent::ModifiersChanged(modifiers) => {
                        state.modifiers = modifiers.state();
                        tracing::info!("Modifiers changed to {:?}", state.modifiers);
                    }
                    WindowEvent::KeyboardInput {
                        event,
                        is_synthetic: false,
                        ..
                    } => {
                        let mods = state.modifiers;

                        // Dispatch actions only on press.
                        if event.state.is_pressed() {
                            let opt = if !mods.is_empty() { Some(mods) } else { None };

                            // if let winit::keyboard::Key::Named(e) = event.logical_key.as_ref() {
                            //     tracing::info!("You pressed key: {:#?}", e);
                            // }
                            // let command = if let Key::Character(ch) = event.logical_key.as_ref() {
                            //     Some(Command::new(&ch, &mods))
                            // } else {
                            //     None
                            // };
                            let command = match event.logical_key.as_ref() {
                                winit::keyboard::Key::Named(k) => Some(Command::from(&k)),
                                winit::keyboard::Key::Character(k) => Some(Command::new(&k, &mods)),
                                _ => None,
                            };

                            if let Some(command) = command {
                                tracing::info!("{:#?}", &command);
                                let choices = state.command.clone();
                                let choice_map = state.command.choices().0.clone();

                                if let Some(choices) = choices.choices().0.get(&state.command_key) {
                                    if let Some(opts) = choices.0.get(&command) {
                                        match opts {
                                            CommandOptions::Commands(c) => {
                                                tracing::info!("Commands available: {:#?}", c);
                                                state.command_key = c.id.clone();
                                                
                                            }
                                            CommandOptions::Acts(a) => {
                                                tracing::info!("Acts in queue: {:#?}", a);
                                                state.command_key = "normal".to_string();
                                                for act in a {
                                                    match act {
                                                        Act::App(v) => state.act(v),
                                                        Act::Egui(v) => state.ui_state.act(v),
                                                        Act::Named(v) => {
                                                            tracing::info!("{:#?}", &v);
                                                            match v {
                                                                NamedAct::Escape => ewlt.exit(),
                                                                _ => tracing::info!("Named event detected"),
                                                            }
                                                        },
                                                        Act::Be => tracing::info!("Taking no action."),
                                                    }
                                                }
                                            }
                                        }
                                    } else {
                                        tracing::info!("Command not recognized.");
                                    }

                                }
                            };
                            // let action = if let Key::Character(ch) = event.logical_key.as_ref() {
                            //     App::process_key_binding(&ch.to_uppercase(), &mods)
                            // } else {
                            //     None
                            // };
                            //
                            // if let Some(action) = action {
                            //     tracing::info!("{:#?}", &action);
                            //     match action {
                            //         Action::Minimize => state.minimize(),
                            //         Action::PrintHelp => state.print_help(),
                            //         Action::ShowWindowMenu => state.show_menu(),
                            //         Action::ToggleDecorations => state.toggle_decorations(),
                            //         Action::ToggleFullscreen => state.toggle_fullscreen(),
                            //         Action::ToggleMaximize => state.toggle_maximize(),
                            //         _ => tracing::info!("Other action."),
                            //     }
                            //     // state.handle_action(&event_loop, window_id, action);
                            // }
                        }
                    }
                    WindowEvent::Resized(physical_size) => {
                        state.resize(*physical_size);
                    }
                    WindowEvent::RedrawRequested => match state.render() {
                        Ok(_) => {}
                        Err(SurfaceError::Lost | SurfaceError::Outdated) => {
                            state.resize(state.size)
                        }
                        Err(SurfaceError::OutOfMemory) => ewlt.exit(),
                        Err(SurfaceError::Timeout) => {
                            // Ignore timeouts.
                        }
                    },
                    other => {
                        state.handle_event(other);
                        window.request_redraw();
                        return;
                    }
                };
                state.handle_event(event);
                window.request_redraw();
            }
            _ => {}
        }
    });
}
