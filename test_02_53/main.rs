// trait 示例

struct Duck;
struct Pig;
trait Fly {
    fn fly(&self) -> bool;
}

impl Fly for Duck {
    fn fly(&self) -> bool {
        return true;
    }
}

impl Fly for Pig {
    fn fly(&self) -> bool {
        return false;
    }
}

fn fly_static<T: Fly>(s: T) -> bool {
    s.fly()
}

fn fly_dyn(s: &dyn Fly) -> bool {
    s.fly()
}

fn main() {
    let pig = Pig;
    println!("fly_static::<Pig>(pig) {}", fly_static::<Pig>(pig));
    assert_eq!(fly_static::<Pig>(pig), false);
    let duck = Duck;

    println!("fly_static::<Duck>(duck) {}", fly_static::<Duck>(duck));
    assert_eq!(fly_static::<Duck>(duck), true);

    println!("fly_dyn(&Pig) {}", fly_dyn(&Pig));
    assert_eq!(fly_dyn(&Pig), false);

    println!("fly_dyn(&Duck) {}", fly_dyn(&Duck));
    assert_eq!(fly_dyn(&Duck), true);
}
