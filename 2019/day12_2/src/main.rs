
use packed_simd::*;

#[derive(PartialEq, Debug, Clone)]
struct Body {
    position: i32x4,
    velocity: i32x4,
}

impl Body {
    fn new() -> Body {
        Body{position: i32x4::new(0, 0, 0, 0), velocity: i32x4::new(0, 0, 0, 0)}
    }

    fn from(pos: [i32; 3]) -> Body {
        let p = [pos[0], pos[1], pos[2], 0];
        Body{position: i32x4::from_slice_aligned(&p), velocity: i32x4::new(0, 0, 0, 0)}
    }
}

type System = Vec<Body>;

fn apply_gravity(system: &mut System) -> () {
    let bodies = system.clone();
    let zeros = i32x4::splat(0);
    let positive = i32x4::splat(1);
    let negative = i32x4::splat(-1);
    for body1 in system.iter_mut() {
        for body2 in bodies.iter() {
            let diff = body2.position - body1.position;
            let m1 = diff.gt(zeros);
            let m2 = diff.lt(zeros);
            let d = m1.select(positive, zeros);
            d += m2.select(negative, zeros);
            body1.velocity += d;
        }
    }
}

fn apply_velocity(system: &mut System) -> () {
    for body in system.iter_mut() {
        body.position += body.velocity;
    }
}

fn total_energy(system: &System) -> i32 {
    let mut energy = 0;
    for body in system.iter() {
        let pot = (body.position as f32x4).abs().sum() as i32;
        let kin = (body.velocity as f32x4).abs().sum() as i32;
        //println!("Body {:?} Pot: {} Kin: {}", body, pot, kin);
        energy += pot * kin;
    }
    energy
}

fn create_system(input: &str) -> System {
    let mut system = Vec::new();
    for line in input.lines() {
        let trim_line = line.trim();
        if trim_line.len() < 1 { continue; }
        let parts: Vec<&str> = trim_line.trim_matches(|c| c == '<' || c == '>').split(',').collect();
        assert_eq!(parts.len(), 3);
        let mut pos = [0; 3];
        for p in parts {
            let pp: Vec<&str> = p.split('=').collect();
            let num: i32 = pp[1].trim().parse().unwrap();
            match pp[0].trim() {
                "x" => { pos[0] = num; },
                "y" => { pos[1] = num; },
                "z" => { pos[2] = num; },
                e => panic!("bad coord: {}", e),
            };
        }
        system.push(Body::from(pos));
    }
    system
}

fn main() {
    let input = "
<x=15, y=-2, z=-6>
<x=-5, y=-4, z=-11>
<x=0, y=-6, z=0>
<x=5, y=9, z=6>";
    let mut system = create_system(input);
    let orig_system = system.clone();
    for step in 1..100000000000u64 {
        apply_gravity(&mut system);
        apply_velocity(&mut system);
        if system == orig_system {
            println!("took {} steps", step);
            break;
        }
    }
    println!("done");
}


mod tests {
    use super::*;

    #[test]
    fn test_create_system() {
        let input = "
<x=15, y=-2, z=-6>
<x=-5, y=-4, z=-11>
<x=0, y=-6, z=0>
<x=5, y=9, z=6>";
        let system = create_system(input);
        assert_eq!(system.len(), 4);
        assert_eq!(system[0], Body::from([15,-2,-6]));
        assert_eq!(system[1], Body::from([-5,-4,-11]));
        assert_eq!(system[2], Body::from([0,-6,0]));
        assert_eq!(system[3], Body::from([5,9,6]));
    }

    #[test]
    fn test_apply_gravity() {
        let input = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut system = create_system(input);
        apply_gravity(&mut system);
        assert_eq!(system[0].velocity, [3, -1, -1]);
        assert_eq!(system[1].velocity, [1, 3, 3]);
        assert_eq!(system[2].velocity, [-3, 1, -3]);
        assert_eq!(system[3].velocity, [-1, -3, 1]);
    }

    #[test]
    fn test_apply_velocity() {
        let input = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut system = create_system(input);
        apply_gravity(&mut system);
        apply_velocity(&mut system);
        assert_eq!(system[0].position, [2, -1, 1]);
        assert_eq!(system[1].position, [3, -7, -4]);
        assert_eq!(system[2].position, [1, -7, 5]);
        assert_eq!(system[3].position, [2, 2, 0]);
    }

    #[test]
    fn test_total_energy() {
        let input = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut system = create_system(input);
        for _ in 0..10 {
            apply_gravity(&mut system);
            apply_velocity(&mut system);
        }
        assert_eq!(total_energy(&system), 179);
    }

    #[test]
    fn test_num_steps() {
        let input = "
<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";
        let mut system = create_system(input);
        let system_orig = system.clone();
        let mut nsteps = 0;
        for i in 1..4000 {
            apply_gravity(&mut system);
            apply_velocity(&mut system);
            if i >= 2770 {
                println!("step {}, system {:?}", i, system);
            }
            if system == system_orig {
                nsteps = i;
                break;
            }
        }
        assert_eq!(nsteps, 2772);
    }
}