extern crate cgmath;
extern crate mint;
extern crate rand;
extern crate three;

use cgmath::prelude::*;
use three::Object;


const COLOR_BACKGROUND: three::Color = 0xf0e0b6;
const COLOR_RED: three::Color = 0xf25346;
const COLOR_WHITE: three::Color = 0xd8d0d1;
const COLOR_BROWN: three::Color = 0x59332e;
//const COLOR_PINK: three::Color = 0xF5986E;
const COLOR_BROWN_DARK: three::Color = 0x23190f;
const COLOR_BLUE: three::Color = 0x68c3c0;

fn main(){

    let mut win = three::Window::new("3d bug");
    win.scene.background = three::Background::Color(COLOR_BACKGROUND);
    
    let cam = win.factory.perspective_camera(60.0, 1.0 .. 1000.0);
    cam.set_position([0.0, 100.0, 200.0]);
    win.scene.add(&cam);

    
    let hemi_light = win.factory.hemisphere_light(0xaaaaaa, 0x000000, 0.9);
    win.scene.add(&hemi_light);
    let mut dir_light = win.factory.directional_light(0xffffff, 0.9);
    dir_light.look_at([150.0, 350.0, 350.0], [0.0, 0.0, 0.0], None);
    let shadow_map = win.factory.shadow_map(2048, 2048);
    dir_light.set_shadow(shadow_map, 400.0, 1.0 .. 1000.0);
    win.scene.add(&dir_light);
    let ambient_light = win.factory.ambient_light(0xdc8874, 0.5);
    win.scene.add(&ambient_light);

    let sea = {
        let geo = three::Geometry::cylinder(600.0, 600.0, 800.0, 40);
        let material = three::material::Lambert {
            color: COLOR_BLUE,
            flat: true,
        };
        win.factory.mesh(geo, material)
    };
    //let sea_base_q = cgmath::Quaternion::from_angle_x(-cgmath::Rad::turn_div_4());
    //sea.set_transform([0.0, -600.0, 0.0], sea_base_q, 1.0);
    win.scene.add(&sea);

    while win.update() && !win.input.hit(three::KEY_ESCAPE) {

        win.render(&cam);
    }

}