use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::fmt::Debug;
use std::io::{Error, ErrorKind};
use std::net::{Ipv4Addr, Ipv6Addr, SocketAddr};
use std::vec::IntoIter;
use std::{convert::Infallible, net::SocketAddr};

pub(crate) trait IsLocalhost {
    fn is_localhost(&self) -> bool;
}

async fn handle(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let addr = req.as_str();
    let addr = (addr, 0).to_socket_addrs();

    // if let Ok(addresses) = addr {
    // for a in addresses {
    // if a.ip().eq(&Ipv4Addr::new(127, 0, 0, 1)) {
    // return Box::pin(async { Err(Error::from(ErrorKind::Other)) });
    // }
    // }
    // }

    if let Ok(true) = addr.map(|mut el| el.is_localhost()) {
        return Box::pin(async { Err(Error::from(ErrorKind::Other)) });
    }
}

impl IsLocalhost for Ipv4Addr {
    fn is_localhost(&self) -> bool {
        Ipv4Addr::new(127, 0, 0, 1).eq(self) || Ipv4Addr::new(0, 0, 0, 0).eq(self)
    }
}

impl IsLocalhost for Ipv6Addr {
    fn is_localhost(&self) -> bool {
        Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1).eq(self)
    }
}

impl IsLocalhost for Ipv4Addr {
    fn is_localhost(&self) -> bool {
        match self {
            Ipv4Addr::V4(ref a) => a.is_localhost(),
            Ipv6Addr::V6(ref a) => a.is_localhost(),
        }
    }
}

impl IsLocalhost for SocketAddr {
    fn is_localhost(&self) -> bool {
        self.ip().is_localhost()
    }
}

pub(crate) trait HasLocalhost {
    fn has_localhost(&mut self) -> bool;
}

impl HasLocalhost for IntoIter<SocketAddr> {
    fn has_localhost(&mut self) -> bool {
        self.any(|el| el.is_localhost())
    }
}

struct Foo;

impl HasLocalhost for Foo {
    fn has_localhost(&mut self) -> bool {
        let testing = "foo";
        println!("{testing:?}");

        true
    }
}

#[tokio::main]
async fn main() {
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));

    let make_svc = make_service_fn(|_conn| async { Ok::<_, Infallible>(service_fn(handle)) });

    let server = Server::bind(&addr).serve(make_svc);

    if let Err(e) = server.await {
        eprintln!("server error: {}", e);
    }
}

trait Print {
    fn print(&self);
}

impl<T: Debug> Print for T {
    fn print(&self) {
        println!("{self:?}");
    }
}
