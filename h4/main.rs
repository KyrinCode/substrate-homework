// 第一题

fn main() {
	let light = TrafficLight::Red;
    println!("{:?}", light.get_time());
}

enum TrafficLight {
	Red,
	Green,
	Yellow
}

trait Time {
	fn get_time(&self);
}

impl Time for TrafficLight {
	fn get_time(&self) {
		match self {
			TrafficLight::Red => 3,
			TrafficLight::Green => 2,
			TrafficLight::Yellow => 1
		}
	}
}


// 第二题

fn get_sum(v: &[u32]) -> Option<u32> {
	let mut sum: u32 = 0;
	for i in v.iter() {
		sum += i;
	}
	let mut ret: Option<u32> = Option::None;
	if sum > 0 {
		ret = Option::Some(sum);
	}
	ret
}

fn main() { 
    let v: Vec<u32> = vec![1, 2, 3];
    println!("{:?}", get_sum(&v[..]));
}


// 第三题
struct Circle {
	r: f64
}

struct Square {
	edge: f64
}

struct Triangle {
	a: f64,
	h: f64
}

trait Area {
	fn area(&self) -> f64;
}

impl Area for Circle {
	fn area(&self) -> f64 {
		self.r * self.r * 3.14
	}
}

impl Area for Square {
	fn area(&self) -> f64 {
		self.edge * self.edge
	}
}

impl Area for Triangle {
	fn area(&self) -> f64 {
		self.a * self.h / 2.0
	}
}

fn get_area<T: Area>(item: &T) -> f64 {
    item.area()
}

fn get_area2(item: impl Area) -> f64 {
    item.area()
}

fn main() {
    let c = Circle{r: 3.0};
    let s = Square{edge: 3.0};
    let t = Triangle{a: 3.0, h: 3.0};
    println!("{}", get_area(&c));
    println!("{}", get_area(&s));
    println!("{}", get_area(&t));
    println!("{}", get_area2(c));
    println!("{}", get_area2(s));
    println!("{}", get_area2(t));
}