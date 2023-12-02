

#[derive(PartialEq, Debug, Clone)]
struct Body {
    position: [i32; 3],
    velocity: [i32; 3],
}

impl Body {
    fn new() -> Body {
        Body{position: [0; 3], velocity: [0; 3]}
    }

    fn from(pos: [i32; 3]) -> Body {
        Body{position: pos, velocity: [0; 3]}
    }
}

type System = Vec<Body>;

fn apply_gravity_1d(p1: i32, p2: i32) -> i32 {
    let diff = p1 - p2;
    if diff < 0 {
        1
    } else if diff > 0 {
        -1
    } else {
        0
    }
}

fn apply_gravity(system: &mut System) -> () {
    let bodies = system.clone();
    for body1 in system.iter_mut() {
        for body2 in bodies.iter() {
            if body1 == body2 {
                continue;
            }
            body1.velocity[0] += apply_gravity_1d(body1.position[0], body2.position[0]);
            body1.velocity[1] += apply_gravity_1d(body1.position[1], body2.position[1]);
            body1.velocity[2] += apply_gravity_1d(body1.position[2], body2.position[2]);
        }
    }
}

fn apply_velocity(system: &mut System) -> () {
    for body in system.iter_mut() {
        body.position[0] += body.velocity[0];
        body.position[1] += body.velocity[1];
        body.position[2] += body.velocity[2];
    }
}

fn total_energy(system: &System) -> i32 {
    let mut energy = 0;
    for body in system.iter() {
        let pot = body.position.iter().map(|x| x.abs()).sum::<i32>();
        let kin = body.velocity.iter().map(|x| x.abs()).sum::<i32>();
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
    println!("Hello, world!");
    let mut system = create_system(input);
    for _ in 0..1000 {
        apply_gravity(&mut system);
        apply_velocity(&mut system);
    }
    println!("total energy: {}", total_energy(&system));
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
}