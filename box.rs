// 11/10/2025 (dmy)

/*
--------------------------------------------------
| Box (1)
--------------------------------------------------
| - Box<T> คือ smart pointer ที่เก็บข้อมูลบน heap
|   ปกติ ตัวแปรใน Rust จะอยู่บน stack
|   แต่ถ้าใช้ Box<T> มันจะ "ย้ายค่าจริง ๆ ไปไว้บน heap"
|   แล้ว stack จะเก็บแค่ pointer (address) ชี้ไปหามัน
--------------------------------------------------
*/

let x = Box::new(5);
// ค่าจริง 5 จะอยู่บน heap
// ตัวแปร x บน stack จะเก็บ pointer ไปยังตำแหน่งใน heap




/*
--------------------------------------------------
| Box (2.1)
--------------------------------------------------
| - ทำไมต้องใช้ Box
| - (1) เมื่อขนาดของข้อมูลไม่แน่นอนตอน compile
|   Rust ต้องรู้ขนาดของทุก type ตอน compile-time
|   แต่บาง type เช่น recursive type (self-referential) จะมีขนาดไม่แน่นอน
--------------------------------------------------
*/

struct Node {
    next: Option<Node>, // ❌ ไม่รู้ขนาด เพราะ Node มี Node ซ้ำในตัว
}

struct Node {
    next: Option<Box<Node>>, // ✅ ใส่ Box ทำให้ขนาดแน่นอน
}
// เพราะ Box<Node> มีขนาดคงที่เสมอ - แค่ pointer 1 ตัวเท่านั้น




/*
--------------------------------------------------
| Box (2.2)
--------------------------------------------------
| - ทำไมต้องใช้ Box
| - (2) เมื่ออยากเก็บข้อมูลใหญ่ใน heap แทน stack
--------------------------------------------------
*/

let big = Box::new([0u8; 1024 * 1024]); // 1 MB
// ถ้าเก็บบน stack อาจจะ "overflow" ได้
// แต่พอใส่ใน Box ข้อมูลจริงอยู่ใน heap ปลอดภัยกว่า




/*
--------------------------------------------------
| Box (2.3.1)
--------------------------------------------------
| - ทำไมต้องใช้ Box
| - (3) เมื่ออยากใช้ dynamic dispatch (dyn Trait)
| - ตัวอย่างนี้เจอบ่อยมากในระบบ plugin หรือ polymorphism
|   เพราะ trait object (dyn Trait) ต้องอยู่หลัง pointer เท่านั้น
--------------------------------------------------
*/

// Shape ต้องมีฟังก์ชัน area()
trait Shape {
    fn area(&self) -> f64;
}

struct Circle { r: f64 } // Circle (r รัศมี)

// implement Trait Shape ให้กับ Circle "ต้องมี area()"
impl Shape for Circle {
    fn area(&self) -> f64 {
        3.14 * self.r * self.r // สูตรคำนวณพื้นที่วงกลม = π * r^2
    }
}

struct Square { s: f64 } // Square (s ความยาวด้าน)

// implement Trait Shape ให้กับ Square
impl Shape for Square {
    fn area(&self) -> f64 {
        self.s * self.s // สูตรคำนวณพื้นที่สี่เหลี่ยมจัตุรัส = s^2
    }
}



/*
--------------------------------------------------
| Box (2.3.2)
--------------------------------------------------
| - เราต้องการเก็บ Circle และ Square ไว้ใน Vec เดียวกัน
|   ดังนั้นเราจึงใช้ Trait Object: Box<dyn Shape>
| - แต่ทั้งสอง struct มี type ต่างกัน
| - อธิบาย:
|   - dyn Shape หมายถึง "object ที่มี trait Shape"
|   - Box<dyn Shape> คือ "กล่อง" (pointer) ที่ชี้ไปยัง object ใด ๆ 
|     ที่ implement Shape อยู่บน heap
|     เพราะ Vec ต้องการเก็บข้อมูลที่มีขนาดเท่ากันทุก element 
|     (ซึ่ง trait object มีขนาดไม่แน่นอน)
--------------------------------------------------
*/
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        // เก็บ (Circle, Square) ใน heap แล้วใส่ pointer ลง Vec
        Box::new(Circle { r: 2.0 }),
        Box::new(Square { s: 3.0 }),
    ];

    /*
     วนลูปผ่าน Vec ที่เก็บ trait object (Box<dyn Shape>)
     แต่ละ shape อาจเป็น Circle หรือ Square ก็ได้
     เรียกใช้ polymorphism - Rust จะเลือกเรียก method area() 
     ที่ตรงกับ type จริงตอน runtime
     */
    for shape in shapes {
        println!("Area = {}", shape.area());
    }
}



/*
--------------------------------------------------
| Box (2.4.1)
--------------------------------------------------
| - ทำไมต้องใช้ Box
| - (4) เมื่ออยากย้าย ownership แต่ไม่อยาก copy ของใหญ่ ๆ
| - เช่น มี struct ขนาดใหญ่ที่ต้องส่งไปให้ฟังก์ชันอื่นโดยไม่ copy
| - Box<T> ทำให้ย้าย pointer ได้โดยไม่ต้องย้ายข้อมูลจริงทั้งก้อน
--------------------------------------------------
*/

// ดูตัวอย่างไม่ใช้ Box (ข้อมูลขนาดใหญ่ถูก copy ย้ายไปมา)
struct BigData {
    data: Vec<u8>,
}

// ฟังก์ชันนี้รับค่า BigData โดยตรง "ย้าย ownership ของ struct ทั้งตัว"
fn process(data: BigData) {
    println!("Processing {} bytes...", data.data.len());
}

fn main() {
    let big = BigData { data: vec![0u8; 1_000_000] };

    // ❌ ย้าย ownership ของ big ทั้งก้อนเข้าไปใน process()
    process(big);
}



/*
--------------------------------------------------
| Box (2.4.2)
--------------------------------------------------
| - ใช้ Box เพื่อย้าย pointer แทน struct ทั้งตัว
--------------------------------------------------
*/

struct BigData {
    data: Vec<u8>,
}

// ฟังก์ชันนี้รับ Box<BigData> "ย้ายแค่ pointer ขนาดเล็กมาก (~8 bytes)"
fn process(data: Box<BigData>) {
    println!("Processing {} bytes...", data.data.len());
}

fn main() {
    // ข้อมูลจริงอยู่บน heap เพราะเราห่อด้วย Box
    let big = Box::new(BigData { data: vec![0u8; 1_000_000] });

    // ✅ ย้าย pointer ของ Box เข้าไปใน process()
    process(big);
}
