use std::any::Any;
use std::fmt::Debug;
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use tokio::sync::Notify;

#[derive(Debug, Clone, PartialEq)]
enum ContextError {
    Canceled,
}

#[derive(Debug, Clone)]
enum ContextValueError {
    NotFound,
}

#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
enum ContextKey {}

trait Context: Send + Sync {
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>>;
    fn err(&self) -> Option<ContextError>;
    fn value(&self, key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError>;
}

trait CancelPropagate: Send + Sync {
    fn cancel_propagate(&self, error: ContextError);
}

trait HasContextTree {
    fn context_tree(&self) -> Arc<Mutex<ContextTree>>;
}

struct ContextTree {
    children: Vec<Arc<dyn CancelPropagate>>,
    parent: Option<Arc<dyn Context>>,
    canceled: Option<ContextError>,
}

struct ContextWithCancel {
    cancel_notify: Notify,
    tree_node: Arc<Mutex<ContextTree>>,
}

struct Canceler<C: CancelPropagate> {
    context: Arc<C>,
}

impl<C: CancelPropagate> Canceler<C> {
    pub fn cancel(&self) {
        self.context.cancel_propagate(ContextError::Canceled)
    }
}

impl ContextWithCancel {
    pub fn new<C: 'static + HasContextTree + Context>(
        context: Arc<C>,
    ) -> (Arc<Self>, Canceler<Self>) {
        let this = Arc::new(Self {
            cancel_notify: Notify::new(),
            tree_node: Arc::new(Mutex::new(ContextTree {
                canceled: None,
                children: vec![],
                parent: Some(context.clone()),
            })),
        });
        context
            .context_tree()
            .lock()
            .unwrap()
            .children
            .push(this.clone());
        (this.clone(), Canceler { context: this })
    }

    pub async fn done(&self) -> Result<(), ContextError> {
        let _ = self.cancel_notify.notified().await;
        if let Some(e) = self.tree_node.lock().unwrap().canceled.clone() {
            Err(e)
        } else {
            Ok(())
        }
    }
}

impl Context for ContextWithCancel {
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>> {
        Box::pin(ContextWithCancel::done(self))
    }

    fn err(&self) -> Option<ContextError> {
        self.tree_node.lock().unwrap().canceled.clone()
    }

    fn value(&self, key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError> {
        self.tree_node
            .lock()
            .unwrap()
            .parent
            .as_ref()
            .expect("WithCancelは必ず親を持つ")
            .value(key)
    }
}

impl CancelPropagate for ContextWithCancel {
    fn cancel_propagate(&self, error: ContextError) {
        let mut body = self.tree_node.lock().unwrap();

        for child in &body.children {
            child.cancel_propagate(error.clone())
        }
        body.canceled.replace(error);

        self.cancel_notify.notify_waiters();
    }
}

struct BackgroundContext {
    body: Arc<Mutex<ContextTree>>,
}

impl BackgroundContext {
    fn new() -> Arc<Self> {
        Arc::new(BackgroundContext {
            body: Arc::new(Mutex::new(ContextTree {
                parent: None,
                children: vec![],
                canceled: None,
            })),
        })
    }
}

impl Context for BackgroundContext {
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>>>> {
        todo!()
    }

    fn err(&self) -> Option<ContextError> {
        todo!()
    }

    fn value(&self, _key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError> {
        Err(ContextValueError::NotFound)
    }
}

impl HasContextTree for BackgroundContext {
    fn context_tree(&self) -> Arc<Mutex<ContextTree>> {
        self.body.clone()
    }
}

#[tokio::main]
async fn main() {
    println!("start sub()");

    let (ctx, canceler) = ContextWithCancel::new(BackgroundContext::new()); // Backgroundはrootとなる空のContext

    tokio::spawn(async move {
        println!("sub() is finished");
        canceler.cancel();
    });

    let done = ctx.done().await;
    assert_eq!(done.unwrap_err(), ContextError::Canceled);
    println!("all tasks are finished");
}
