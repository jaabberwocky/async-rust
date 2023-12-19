use futures::future::Future;

fn foo2() -> impl Future<Output = i32> {
    async {
        println!("foo2");
        42
    }
}

trait SimpleFuture {
    type Output;
    fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
}

enum Poll<T> {
    Ready(T),
    Pending,
}

pub struct SimpleSum {
    sum: i32,
    limit: i32,
}

impl SimpleFuture for SimpleSum {
    type Output = i32;

    fn poll(&mut self, wake: fn()) -> Poll<Self::Output> {
        if self.sum < self.limit {
            self.sum += 1;
            Poll::Pending
        } else {
            wake();
            Poll::Ready(self.sum)
        }
    }
}

#[tokio::main]
async fn main() {
    let fut = foo2().await;
    println!("main");
    println!("fut: {}", fut);

    let mut ss: SimpleSum = SimpleSum { sum: 0, limit: 10 };
    while match ss.poll(|| println!("wake")) {
        Poll::Ready(val) => {
            println!("val: {}", val);
            false
        }
        Poll::Pending => {
            println!("pending");
            true
        }
    } {}
}
