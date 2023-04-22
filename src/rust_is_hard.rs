use futures::future::BoxFuture;


#[derive(Debug)]
struct Update;

type Handler = Box<dyn for<'a> Fn(&'a Update) -> BoxFuture<'a, ()> + Send + Sync>;

struct Dispatcher(Vec<Handler>);

// H: Fn(&'a Update) -> Fut + Send + Sync + 'a,
// Fut: Future<Output = ()> + Send + 'a,
impl Dispatcher {
    fn push_handler<H>(&mut self, handler: H)
    where
        H: for<'a> Fn(&'a Update) -> BoxFuture<'a, ()> + Send + Sync + 'static,
    {
        self.0.push(Box::new(move |upd| Box::pin(handler(upd))));
    }
}

fn main() {
    let mut dp = Dispatcher(vec![]);

    dp.push_handler(|upd| {
        Box::pin(async move {
            println!("{:?}", upd);
        })
    });
}
