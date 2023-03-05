#[allow(dead_code)]
#[derive(Debug)]
struct RealRoots {
    root1: f32,
    root2: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct OnlyRoot {
    root: f32,
}

#[allow(dead_code)]
#[derive(Debug)]
struct ImaginaryRoots {
    root1: String,
    root2: String,
}

#[allow(dead_code)]
#[derive(Debug)]
struct NoRoots;

#[allow(dead_code)]
#[derive(Debug)]
enum EquationRoots {
    RealRoots(RealRoots),
    OnlyRoot(OnlyRoot),
    ImaginaryRoots(ImaginaryRoots),
    NoRoots(NoRoots),
}

fn solve_qe(a: f32, b: f32, c: f32) -> EquationRoots {
    let discriminant: f32 = (b*b)-(4.0*a*c);
    if discriminant > 0.0 {
        let x1: f32 = (-b+(((b*b)-(4.0*a*c)).sqrt())) / (2.0*a);
        let x2: f32 = (-b-(((b*b)-(4.0*a*c)).sqrt())) / (2.0*a);
        EquationRoots::RealRoots(RealRoots {
            root1: x1,
            root2: x2
        })
    } else if discriminant == 0.0 {
        let x1: f32 = -b / (2.0*a);
        EquationRoots::OnlyRoot(OnlyRoot {
            root: x1,
        })
    } else if discriminant < 0.0 {
        let denominator = 2.0 *a;
        let real_part = -b / (2.0 *a);
        let imaginary_part = format!("i((srqt {})/{})", discriminant, denominator);
        EquationRoots::ImaginaryRoots(ImaginaryRoots {
            root1: format!("{}+{}", real_part, imaginary_part),
            root2: format!("{}-{}", real_part, imaginary_part),
        })
    } else {
        EquationRoots::NoRoots(NoRoots)
    }
}

fn print_solver(a: f32, b: f32, c: f32) {
    let solutions = solve_qe(a, b, c);
    println!("{}x^2 + {}x + {} = 0", a, b, c);
    println!("{:?}", solutions);
    println!("");
}

fn main() {
    println!("Quadratic equation solver");
    println!("");
    print_solver(4.0, 3.0, 2.0);
    print_solver(1.0, -8.0, 5.0);
    print_solver(-4.0, 12.0, -9.0);
}
