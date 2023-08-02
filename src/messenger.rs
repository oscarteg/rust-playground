use futures::{future::BoxFuture, Future};

#[derive(Debug)]
struct Update;

type Handler = Box<dyn for<'a> Fn(&'a Update) -> BoxFuture<'a, ()> + Send + Sync>;

struct Dispatcher(Vec<Handler>);

impl Dispatcher {
    fn push_handler<'a, H, Fut>(&mut self, handler: H)
    where
        H: Fn(&'a Update) -> Fut + Send + Sync + 'a,
        Fut: Future<Output = ()> + Send + 'a,
    {
        self.0.push(Box::new(move |upd| Box::pin(handler(upd))));
    }
}

fn main() {
    let mut dp = Dispatcher(vec![]);

    dp.push_handler(|upd| async move {
        println!("{:?}", upd);
    });
}
