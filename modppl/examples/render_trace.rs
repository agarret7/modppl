#![allow(non_upper_case_globals)]

use modppl::prelude::*;
use rand::rngs::ThreadRng;

#[derive(Clone, Debug, PartialEq)]
struct Camera {
    z: Real,
}

type Rgb = (Real, Real, Real);
type Xy = [Real; 2];

#[derive(Clone, Debug, PartialEq)]
struct RGBDImage {
    width: usize,
    height: usize,
    camera_z: Real,
    cone: Cone,
    sphere: Sphere
}

#[derive(Clone, Debug, PartialEq)]
struct Cone {
    height: Real,
    radius: Real,
    rgb: Rgb,
    xy: Xy,
}

#[derive(Clone, Debug, PartialEq)]
struct Sphere {
    radius: Real,
    rgb: Rgb,
    xy: Xy,
}

struct PointMass;

const point_mass: PointMass = PointMass;

impl<T> Distribution<T, T> for PointMass
where
    T: Clone + PartialEq,
{
    fn logpdf(&self, x: &T, value: T) -> Real {
        if *x == value {
            0.
        } else {
            Real::NEG_INFINITY
        }
    }

    fn random(&self, _: &mut ThreadRng, value: T) -> T {
        value
    }
}

dyngen!(
    fn render_cone() -> Cone {
        let height = point_mass(0.4) %= "height";
        let radius = point_mass(0.3) %= "radius";
        let rgb = point_mass((0.3, 0.5, 0.2)) %= "rgb";
        let xy = point_mass([0.1, 0.2]) %= "xy";
        Cone { height, radius, rgb, xy }
    }
);

dyngen!(
    fn render_sphere() -> Sphere {
        let radius = point_mass(0.2) %= "radius";
        let rgb = point_mass((0.9, 0.2, 0.2)) %= "rgb";
        let xy = point_mass([0.2, 0.3]) %= "xy";
        Sphere { radius, rgb, xy }
    }
);

dyngen!(
    fn render_scene(width: usize, height: usize) -> RGBDImage {
        let camera = point_mass(Camera { z: 0.2 }) %= "camera";
        let cone = render_cone() /= "cone";
        let sphere = render_sphere() /= "sphere";

        RGBDImage {
            width,
            height,
            camera_z: camera.z,
            cone,
            sphere
        }
    }
);

fn main() {
    let mut constraints = DynTrie::new();
    constraints.observe("cone/xy", Arc::new([0.2, 0.3]));
    let (trace, _weight) = render_scene.generate((4, 3), constraints);

    let formatters: &[DynValueFormatter] = &[
        dyn_debug_formatter::<Camera>,
        dyn_debug_formatter::<Cone>,
        dyn_debug_formatter::<Sphere>
    ];

    println!("PRINT_DYNTRACE_MIN_VERBOSE");
    print_dyntrace_with_options(&trace, formatters, DynTracePrintOptions::verbose(false));

    println!("PRINT_DYNTRACE_DEFAULT");
    print_dyntrace_with(&trace, formatters);
    // or equivalently,
    // print_dyntrace_with_options(&trace, formatters, DynTracePrintOptions::default());

    println!("PRINT_DYNTRACE_MAX_VERBOSE");
    print_dyntrace_with_options(&trace, formatters, DynTracePrintOptions::verbose(true));

    println!("READ BACK SELECTED VALUES");

    let cone_height: Real;
    let cone_radius: Real;
    let mut cone_rgb: Rgb;
    let mut cone_xy: Xy;
    let sphere_radius: Real;
    let sphere_rgb: Rgb;
    let sphere_xy: Xy;
    let camera: Camera;

    // # Safety: all these choices are simply typed
    //   (literals, literal tuples, and literal arrays)
    //
    //   Therefore, they can be autocast with scoped unsafety.
    unsafe {
        cone_height = trace.data.auto("cone/height");
        cone_radius = trace.data.auto("cone/radius");
        cone_rgb = trace.data.auto("cone/rgb");
        cone_xy = trace.data.auto("cone/xy");
        sphere_radius = trace.data.auto("sphere/radius");
        sphere_rgb = trace.data.auto("sphere/rgb");
        sphere_xy = trace.data.auto("sphere/xy");

        // this however, does not work
        // camera = trace.data.auto("camera");
    };

    // safe reads always work, because they verify the *type*.
    cone_rgb = trace.data.read::<Rgb>("cone/rgb");
    cone_xy = trace.data.read::<Xy>("cone/xy");
    camera = trace.data.read::<Camera>("camera");
    println!("camera = {:?}", camera);

    // otherwise print simple values without friction
    println!("cone/height = {}", cone_height);
    println!("cone/radius = {}", cone_radius);
    println!("cone/rgb = {:?}", cone_rgb);
    println!("cone/xy = {:?}", cone_xy);
    println!("sphere/radius = {}", sphere_radius);
    println!("sphere/rgb = {:?}", sphere_rgb);
    println!("sphere/xy = {:?}", sphere_xy);
}
