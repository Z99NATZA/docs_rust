// 16/10/2025 (dmy)


/*
--------------------------------------------------
| Trait (1)
--------------------------------------------------
| - Trait คือ "พฤติกรรม" (behavior) หรือ "สัญญา" (contract)
| - คล้าย interface ในภาษาอื่น เช่น Java / Go / TypeScript
| - ใช้กำหนด method ที่ struct (หรือ type อื่น) ต้องมี
| - ไม่เก็บข้อมูล มีแต่ฟังก์ชัน (method signature)
--------------------------------------------------
*/

trait Speak {
    fn speak(&self);
}




/*
--------------------------------------------------
| Trait (2)
--------------------------------------------------
| - Struct ที่ต้องการใช้พฤติกรรมนี้ ต้อง implement trait
| - ใช้คำสั่ง impl TraitName for StructName
--------------------------------------------------
*/

struct Person {
    name: String,
}

impl Speak for Person {
    fn speak(&self) {
        println!("สวัสดีครับ ผมชื่อ {}", self.name);
    }
}

fn main() {
    let p = Person { name: "Z99NATZA".into() };
    p.speak();
}




/*
--------------------------------------------------
| Trait (3)
--------------------------------------------------
| - Trait มี "default implementation" ได้
| - Struct ที่ impl trait นั้นสามารถใช้ default ได้เลย
| - หรือ override method นั้นก็ได้
--------------------------------------------------
*/

trait Greet {
    fn hello(&self) {
        println!("สวัสดีครับ!");
    }
}

struct Dog;

impl Greet for Dog {} // ใช้ default method

fn main() {
    let d = Dog;
    d.hello(); // แสดง "สวัสดีครับ!"
}




/*
--------------------------------------------------
| Trait (4)
--------------------------------------------------
| - Trait Bound: ใช้กับ Generic Function
| - เพื่อจำกัดว่า type นั้นต้องมี trait บางตัว
--------------------------------------------------
*/

trait Shape {
    fn area(&self) -> f64;
}

struct Circle { r: f64 }
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r
    }
}

// ฟังก์ชันที่รับเฉพาะ type ที่มี Shape
fn print_area<T: Shape>(item: T) {
    println!("พื้นที่ = {}", item.area());
}




/*
--------------------------------------------------
| Trait (5)
--------------------------------------------------
| - Trait Inheritance: Trait ซ้อนกันได้
| - Trait ที่สืบต่อ ต้อง implement ทั้งสองตัว
--------------------------------------------------
*/

trait Drawable {
    fn draw(&self);
}

trait Shape: Drawable {
    fn area(&self) -> f64;
}

struct Rectangle { w: f64, h: f64 }

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("วาดสี่เหลี่ยมขนาด {}x{}", self.w, self.h);
    }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 {
        self.w * self.h
    }
}




/*
--------------------------------------------------
| Trait (6)
--------------------------------------------------
| - Trait Object: ใช้ polymorphism runtime
| - เก็บหลาย struct ที่มี trait เดียวกันใน Vec<Box<dyn Trait>>
--------------------------------------------------
*/

fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Rectangle { w: 4.0, h: 5.0 }),
        Box::new(Rectangle { w: 2.0, h: 3.0 }),
    ];

    for s in shapes {
        s.draw();
        println!("พื้นที่ = {}", s.area());
    }
}




/*
--------------------------------------------------
| Trait (7)
--------------------------------------------------
| - Associated Type: trait สามารถกำหนด type ภายในของตัวเองได้
| - ลดความซับซ้อนของ generic parameter
--------------------------------------------------
*/

trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter { current: u32 }

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<Self::Item> {
        self.current += 1;
        Some(self.current)
    }
}




/*
--------------------------------------------------
| Trait (8)
--------------------------------------------------
| - Blanket Implementation: implement trait ครอบทุก type
| - ตัวอย่างจากมาตรฐาน Rust (impl ToString for ทุก type ที่มี Display)
--------------------------------------------------
*/

use std::fmt::Display;

impl<T: Display> ToString for T {
    fn to_string(&self) -> String {
        format!("{}", self)
    }
}




/*
--------------------------------------------------
| Trait (9)
--------------------------------------------------
| - Static Dispatch vs Dynamic Dispatch
|
| Static Dispatch:
|   - ใช้ generic + trait bound เช่น <T: Trait>
|   - compile-time รู้ type ชัดเจน
|   - เร็วกว่า
|
| Dynamic Dispatch:
|   - ใช้ Box<dyn Trait>
|   - runtime ตรวจ type
|   - ยืดหยุ่นกว่า
--------------------------------------------------
*/

fn calc_area<T: Shape>(s: &T) -> f64 { // Static Dispatch
    s.area()
}

fn draw_shape(s: &dyn Shape) { // Dynamic Dispatch
    s.draw();
}




/*
--------------------------------------------------
| Trait (สรุป)
--------------------------------------------------
| ✅ ใช้กำหนดพฤติกรรม (behavior) ที่ type ต้องมี
| ✅ ใช้ได้กับหลาย struct เพื่อให้ polymorphism
| ✅ ใส่ default method ได้
| ✅ ใช้ร่วมกับ generics ผ่าน trait bound
| ✅ รองรับการสืบทอด trait อื่น (trait inheritance)
| ✅ รองรับ associated type และ blanket impl
|
| 🔹 impl Trait for Struct = ทำให้ struct นั้นมีพฤติกรรม trait นั้น
| 🔹 dyn Trait = ใช้ polymorphism runtime
| 🔹 <T: Trait> = จำกัด generic ให้เฉพาะ type ที่มี trait
--------------------------------------------------
*/
