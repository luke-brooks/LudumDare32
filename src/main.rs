extern crate piston;
extern crate ai_behavior;
extern crate sprite;
extern crate graphics;
extern crate sdl2_window;
extern crate opengl_graphics;

use std::path::Path;
use std::cell::RefCell;
use std::rc::Rc;

use sprite::*;
use ai_behavior::{
    Action,
    Sequence,
    Wait,
    WaitForever,
    While,
};

use sdl2_window::Sdl2Window;
use opengl_graphics::{
    GlGraphics,
    OpenGL,
    Texture,
};
use piston::window::{ WindowSettings, Size };
use piston::event::*;
use piston::input::{ Button, Key, MouseButton };

fn main() {
    //window dimensions
    let (width, height) = (800, 600);
    //openGL version
    let opengl = OpenGL::_3_2;
    //create a new SDL window
    let window = Sdl2Window::new(
        opengl,
        WindowSettings::new(
            "Ludum Dare 32".to_string(),
            Size { width: width, height: height }
        ).exit_on_esc(true)
    );

    //create a new scene
    let mut scene = Scene::new();
    //path to the texture
    let tex = Path::new("./bin/assets/rust-logo.png");
    //load the texture from the path
    let tex = Rc::new(Texture::from_path(&tex).unwrap());
    //create the sprite form the texture
    let mut sprite = Sprite::from_texture(tex.clone());
    //set sprite position to center screen
    sprite.set_position(width as f64 / 2.0, height as f64 / 2.0);
    //add the sprite to the scene, assign id to the sprite.
    let id = scene.add_child(sprite);

    // Run a sequence of animations.
    let seq = Sequence(vec![
        Action(Ease(EaseFunction::CubicOut, Box::new(ScaleTo(2.0, 0.5, 0.5)))),
        Action(Ease(EaseFunction::BounceOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Ease(EaseFunction::ElasticOut, Box::new(MoveBy(2.0, 0.0, -100.0)))),
        Action(Ease(EaseFunction::BackInOut, Box::new(MoveBy(1.0, 0.0, -100.0)))),
        Wait(0.5),
        Action(Ease(EaseFunction::ExponentialInOut, Box::new(MoveBy(1.0, 0.0, 100.0)))),
        Action(Blink(1.0, 5)),
        While(Box::new(WaitForever), vec![
            Action(Ease(EaseFunction::QuadraticIn, Box::new(FadeOut(1.0)))),
            Action(Ease(EaseFunction::QuadraticOut, Box::new(FadeIn(1.0)))),
        ]),
    ]);
    //run the sequence of animations on the sprite
    scene.run(id, &seq);

    // This animation and the one above can run in parallel.
    let rotate = Action(Ease(EaseFunction::ExponentialInOut,
        Box::new(RotateTo(2.0, 360.0))));
    scene.run(id, &rotate);

    println!("Press space to pause/resume the animation!");

    let ref mut gl = GlGraphics::new(opengl);
    let window = Rc::new(RefCell::new(window));
    //poll for events
    for e in window.events() {
        scene.event(&e);

        //get the rendering event and draw the scene to the screen
        if let Some(args) = e.render_args() {
            use graphics::*;
            gl.draw([0, 0, args.width as i32, args.height as i32], |c, gl| {
                graphics::clear([1.0, 1.0, 1.0, 1.0], gl);
                scene.draw(c.transform, gl);
            });
        }
        //get mouse press to pause animation
        if let Some(Button::Mouse(key)) = e.press_args() {
            if key == MouseButton::Left
            {
                scene.pause(id, &seq);
                scene.pause(id, &rotate);

            }
        }
        //get spacebar press to resume animation
        if let Some(Button::Keyboard(key)) = e.press_args() {
            if key == Key::Space
            {
                scene.resume(id, &seq);
                scene.resume(id, &rotate);
            }
        }
    }
}
