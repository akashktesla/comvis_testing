

use winit::event::{ElementState, VirtualKeyCode,KeyboardInput};
use winit::event_loop::{EventLoop,ControlFlow};
use winit::window::WindowBuilder;
use winit::event::Event::WindowEvent;
    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("valzkai ae")
        .build(&event_loop).unwrap();
    event_loop.run(
        move|event,_,control_flow|{
            *control_flow = ControlFlow::Wait;
            match event{
                WindowEvent{window_id,event} if window_id == window.id() => {
                    match event{
                        winit::event::WindowEvent::Resized(_)=>{},
                        winit::event::WindowEvent::CloseRequested =>{ 
                            *control_flow=ControlFlow::Exit
                        },
                        winit::event::WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput{
                                    state:ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                            },
                            ..
                        }=>*control_flow=ControlFlow::Exit,
                        winit::event::WindowEvent::ScaleFactorChanged {
                            scale_factor:_, 
                            new_inner_size:_,
                        }=>{},
                        _ => ()
                    }
                },
                winit::event::Event::RedrawRequested(_) => {},
                _ => ()

            };
        }
        );
