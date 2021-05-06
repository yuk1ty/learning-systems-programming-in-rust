use futures;
use futures::future::Either;
use futures::FutureExt;
use std::any::Any;
use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio;
use tokio::sync::watch::error::RecvError;
use tokio::sync::{watch, Notify};
use tokio::time::Instant;

#[derive(Debug, Clone)]
enum ContextError {
    Canceled,
}

#[derive(Debug, Clone)]
enum ContextValueError {
    NotFound,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
enum ContextKey {
    String(String),
    CancelContext,
}

trait Context {
    fn deadline(&self, deadline: Instant, ok: bool);
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>>;
    fn err(&self) -> Option<ContextError>;
    fn value(&self, key: &ContextKey) -> Result<&dyn Any, ContextValueError>;
}

trait Canceler {
    fn cancel(&self, remove_from_parent: bool, error: ContextError);
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>>;
}

trait HasContextBody {
    fn context_body(&self) -> Arc<Mutex<ContextBody>> {
        self.context_body().clone()
    }
}

struct ContextBody {
    children: Vec<Arc<dyn Canceler>>,
    parent: Option<Arc<dyn Context>>,
    canceled: Option<ContextError>,
}

struct WithCancel {
    cancel_notify: Notify,
    body: Arc<Mutex<ContextBody>>,
}

type CancelFunc = fn();

impl WithCancel {
    pub fn new<C: 'static + HasContextBody + Context>(context: Arc<C>) -> (Arc<Self>, CancelFunc) {
        let this = Arc::new(Self {
            cancel_notify: Notify::new(),
            body: Arc::new(Mutex::new(ContextBody {
                canceled: None,
                children: vec![],
                parent: Some(context.clone()),
            })),
        });
        context
            .context_body()
            .lock()
            .unwrap()
            .children
            .push(this.clone());
        (this, move || {})
    }

    pub async fn done(&self) -> Result<(), ContextError> {
        let _ = self.cancel_notify.notified().await;
        if let Some(e) = self.body.lock().unwrap().canceled.clone() {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl Context for WithCancel {
    fn deadline(&self, deadline: Instant, ok: bool) {
        todo!()
    }

    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>> {
        Box::pin(WithCancel::done(self))
    }

    fn err(&self) -> Option<ContextError> {
        self.body.lock().unwrap().canceled.clone()
    }

    fn value(&self, key: &ContextKey) -> Result<&dyn Any, ContextValueError> {
        todo!()
    }
}

impl Canceler for WithCancel {
    fn cancel(&self, remove_from_parent: bool, error: ContextError) {
        let mut body = self.body.lock().unwrap();

        for child in &body.children {
            child.cancel(false, error.clone())
        }
        body.canceled.replace(error);

        self.cancel_notify.notify_waiters();
    }

    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>> {
        Context::done(self)
    }
}

struct Background {
    body: Arc<Mutex<ContextBody>>,
}

impl Background {
    fn new() -> Arc<Self> {
        Arc::new(Background {
            body: Arc::new(Mutex::new(ContextBody {
                parent: None,
                children: vec![],
                canceled: None,
            })),
        })
    }
}

impl Context for Background {
    fn deadline(&self, deadline: Instant, ok: bool) {
        todo!()
    }

    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>>>> {
        todo!()
    }

    fn err(&self) -> Option<ContextError> {
        todo!()
    }

    fn value(&self, _key: &ContextKey) -> Result<&dyn Any, ContextValueError> {
        Err(ContextValueError::NotFound)
    }
}

impl HasContextBody for Background {}

#[tokio::main]
async fn main() {
    println!("start sub()");

    let (ctx, canceler) = WithCancel::new(Background::new()); // context.Background()に関してあまり理解できていない

    tokio::spawn(async move {
        println!("sub() is finished");
        canceler();
    });

    ctx.done().await.unwrap();
    println!("all tasks are finished");
}
