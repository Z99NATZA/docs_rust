/*
--------------------------------------------------
| dyn
--------------------------------------------------
| dynamic dispatch เรียกใช้เมธอดของ trait แบบ runtime 
| - (ไม่ใช่ compile-time เหมือน generic ปกติ)
|
| อันนี้คือ static dispatch - compiler จะรู้ตอน compile ว่า T คืออะไร
| - เช่น String หรือ &str แล้วมันจะ generate โค้ดเฉพาะของแต่ละ type 
| - (เรียกว่า monomorphization)
--------------------------------------------------
*/

fn print_len<T: AsRef<str>>(val: T) {
    println!("{}", val.as_ref().len());
}

/*
--------------------------------------------------
| dyn
--------------------------------------------------
| แต่บางครั้งเราไม่รู้ type ล่วงหน้า
| - เช่น สมมุติเรามีหลาย object ต่างชนิด แต่ทุกตัว implement trait เดียวกัน
--------------------------------------------------
*/

trait Animal {
    fn speak(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn speak(&self) { println!("woof"); }
}

impl Animal for Cat {
    fn speak(&self) { println!("meow"); }
}

// ถ้าเราอยากเก็บ Dog และ Cat ไว้ใน Vec เดียวกัน - จะไม่ได้ เพราะขนาดต่างกัน
let animals = vec![Dog, Cat]; // error

// เราจึงต้องใช้ trait object ผ่าน dyn
// ตอนนี้ animals คือ vector ของ "pointer ไปยัง object ที่ implement Animal"
let animals: Vec<Box<dyn Animal>> = vec![
    Box::new(Dog),
    Box::new(Cat),
];

// แต่ละตัวข้างในอาจเป็น Dog หรือ Cat ก็ได้ - และเมื่อเรียก speak()
// ตรงนี้แหละเรียกว่า dynamic dispatch
for a in animals {
    a.speak(); // จะเลือกเรียก Dog::speak หรือ Cat::speak ตอน runtime
}

/*
--------------------------------------------------
| dyn
--------------------------------------------------
| ทำไมต้องใช้
| - ไม่รู้ชนิดจริงล่วงหน้า
|   เช่น หลาย handler ในเว็บ server, หลาย plugin
|
| - ต้องเก็บ trait ที่มีขนาดไม่แน่นอน
|   เช่น Box<dyn Future<Output=T>>, Box<dyn Fn()>
|
| - ต้องการ polymorphism runtime
|   เช่น router, plugin system
--------------------------------------------------
*/

// จริง ๆ ยังไม่ค่อยเข้าใจ มันเยอะมาก อ่านไม่หมด