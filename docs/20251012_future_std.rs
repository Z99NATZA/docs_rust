// 2025-10-12 (ymd)

/*
--------------------------------------------------
| std::future::Future (1)
--------------------------------------------------
| Future = "งานที่ยังไม่เสร็จ" ซึ่งจะเสร็จในอนาคต
| ในมาตรฐาน Rust ใช้ trait:
|
|   pub trait Future {
|       type Output;
|       fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
|   }
|
| - poll() คืนค่า Poll::Pending ถ้ายังไม่เสร็จ
| - poll() คืนค่า Poll::Ready(T) เมื่อเสร็จ
| - cx.waker() คือกลไกปลุกให้ executor กลับมาพollใหม่เมื่อพร้อม
| - Pin<&mut Self> : ออบเจกต์ของ Future มักต้อง "ตรึงตำแหน่ง" (self-referential safety)
--------------------------------------------------
*/

/*
--------------------------------------------------
| std::future::Future (2) — async/await
--------------------------------------------------
| คีย์เวิร์ด async จะ "สร้าง Future อัตโนมัติ"
| ส่วน await คือการ "รอให้ Future เสร็จ" (executor จะคอย poll ให้)
--------------------------------------------------
*/

use std::time::Duration;
use tokio::time::sleep;     // ใช้เป็นตัวอย่าง runtime/IO non-blocking

async fn say_hello() -> String {
    sleep(Duration::from_millis(500)).await; // จำลองงานรอ IO
    "สวัสดีจาก Future!".to_string()
}

#[tokio::main]
async fn main() {
    println!("เริ่มโปรแกรม");
    let fut = say_hello();       // แค่ "สร้าง" Future (ยังไม่รันจริง)
    println!("สร้าง Future แล้ว แต่ยังไม่ทำงาน");
    let result = fut.await;      // ตอน await => executor จะ poll จนเสร็จ
    println!("ผลลัพธ์: {result}");
}

/*
--------------------------------------------------
| std::future::Future (3) — รอหลาย Future พร้อมกัน
--------------------------------------------------
| Future ช่วยให้ "ไม่บล็อก thread" (non-blocking)
| เราจึงรอหลายงานพร้อมกันได้ (เช่น join!)
--------------------------------------------------
*/

use tokio::join;

async fn task(name: &str, sec: u64) -> String {
    println!("{name} เริ่ม...");
    sleep(Duration::from_secs(sec)).await;
    format!("{name} เสร็จใน {sec}s")
}

#[tokio::main]
async fn demo_join() {
    let a = task("🍎 งาน A", 1);
    let b = task("🍌 งาน B", 2);
    let (ra, rb) = join!(a, b);  // await พร้อมกัน
    println!("done:\n- {ra}\n- {rb}");
}

/*
--------------------------------------------------
| std::future::Future (4) — เขียน Future เอง (manual impl)
--------------------------------------------------
| ด้านล่างเป็นตัวอย่าง Future แบบ "กำหนด poll เอง"
| โดยเราจะห่อ tokio::time::Sleep ไว้ข้างใน แล้ว forward poll
| จุดสำคัญ:
| - ใช้ Pin<Box<...>> เพื่อปักตำแหน่ง Future ภายใน
| - executor จะเรียก poll ให้เราเอง (ผ่าน await)
--------------------------------------------------
*/

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct Delay {
    inner: Pin<Box<tokio::time::Sleep>>,
}

impl Delay {
    fn new(d: Duration) -> Self {
        Self { inner: Box::pin(tokio::time::sleep(d)) }
    }
}

impl Future for Delay {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<()> {
        // ส่งต่อการ poll ไปยัง Sleep (ซึ่งจัดการ waker เอง)
        match self.inner.as_mut().poll(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(_) => Poll::Ready(()),
        }
    }
}

#[tokio::main]
async fn demo_manual_future() {
    println!("เริ่ม Delay 300ms (Future แบบ custom)");
    Delay::new(Duration::from_millis(300)).await; // await => executor จะ poll ให้จน Ready
    println!("Delay เสร็จ!");
}

/*
--------------------------------------------------
| std::future::Future (5) — สรุปหัวใจ
--------------------------------------------------
| - Future = งานที่ยังไม่เสร็จ (มี Output เมื่อพร้อม)
| - async fn = ตัวสร้าง Future แบบอัตโนมัติ
| - await = รอให้ Future เสร็จ (executor จะ poll ให้)
| - Waker = ตัวปลุกให้ executor กลับมาพollต่อเมื่อพร้อม
| - Pin = ป้องกันการย้ายตำแหน่งของ Future ที่ self-referential
|
| เปรียบเทียบ:
| - Thread: เริ่มทำงานทันที, ใช้ CPU/OS thread จริง
| - Future: เป็น state machine; จะเดินหน้าก็ตอนถูก poll (by executor)
--------------------------------------------------
*/
