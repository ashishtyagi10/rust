fn main() {
    // fizzbuzz_to(100);

    let     rectangle = Rectangle{
        p1: Point::origin(),
        p2: Point::new(4.0, 3.0),
    };

    println!("Rectangle perimeter is {}", rectangle.perimeter());
    println!("Rectangle area is {}", rectangle.area());

    let mut square = Rectangle{
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    println!("Square perimeter is {}", square.perimeter());
    println!("Square area is {}", square.area());
    square.translate(1.0, 1.0);
    println!("Square perimeter is {}", square.perimeter());
    println!("Square area is {}", square.area());

    let pair = Pair(Box::new(1), Box::new(1));
    pair.destroy();
}

fn fizzbuzz_to(p0: i32) {
    for n in 1..p0 +1{
        fizzbuzz(n);
    }
}

fn fizzbuzz(p0: i32) -> () {
    if is_divisible_by(p0, 15){
        println!("fizzbuzz");
    }else if is_divisible_by(p0, 5){
        println!("fizz");
    }else if is_divisible_by(p0, 3){
        println!("buzz");
    }else{
        println!("nothing");
    }
}

fn is_divisible_by(p0: i32, p1: i32) -> bool {
    if p1==0 {
     return false;
    }
    p0 % p1 == 0
}

struct Point{
    x: f64,
    y: f64,
}

impl Point{
    fn origin() -> Point{
        Point{x: 0.0, y: 0.0}
    }

    fn new(x: f64, y: f64 ) -> Point{
        Point{x, y }
    }
}

struct Rectangle{
    p1: Point,
    p2: Point,
}

impl Rectangle{
    fn area(&self) -> f64{
        let Point{x:x1, y:y1} = self.p1;
        let Point{x:x2, y:y2} = self.p2;

        ((x1-x2) * (y1-y2)).abs()
    }

    fn perimeter(&self) -> f64{
        let Point{x:x1, y:y1} = self.p1;
        let Point{x:x2, y:y2} = self.p2;

        2.0*((x1-x2).abs() + (y1-y2).abs())
    }

    fn translate(&mut self, x:f64, y:f64){
        self.p1.x +=x;
        self.p2.x +=x;

        self.p1.y +=y;
        self.p2.y +=y;
    }
}

struct  Pair(Box<i32>, Box<i32>);

impl Pair{
    fn destroy(self){
        let Pair(first, second) = self;
        println!("Destroying Pair ({} {})", first, second);
    }
}