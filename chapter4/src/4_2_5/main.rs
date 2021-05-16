use std::any::Any;
use std::fmt::{Debug, Display, Formatter};
use std::future::Future;
use std::hash::Hash;
use std::pin::Pin;
use std::sync::{Arc, Mutex, Weak};
use tokio::sync::Notify;

/// contextがdoneになった理由を表す型
#[derive(Debug, Clone, PartialEq)]
pub enum ContextError {
    Canceled,
}

impl Display for ContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ContextError {}

/// Context::valueの返り値のエラーを表す型
#[derive(Debug, Clone)]
pub enum ContextValueError {
    NotFound,
}

impl Display for ContextValueError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl std::error::Error for ContextValueError {}

/// Context::valueで指定するkeyの型
///
/// FIXME(higumachan): traitで実装するという案もあるが
///
/// ```
/// trait ContextKey: Eq + Any + Sized {
///     fn dynamic_equal(&self, other: &dyn Any) -> bool {
///         if self.type_id() != other.type_id() {
///             return false
///         }
///         
///        let other = other.downcast_ref::<Self>().expect("type_idで比較しているのでここは必ずSome");
///    
///         self == other
///     }
/// }
///
/// impl<T: Eq + Any + Sized> ContextKey for T {
/// }
///
/// pub trait Context: Send + Sync {
///     ...
///     value<K: ContextKey>(key: &K) ->  Result<Arc<dyn Any>, ContextValueError>;
/// }
/// ```
///
/// のようにtraitを定義した場合に
/// https://doc.rust-lang.org/error-index.html#method-has-generic-type-parameters
/// のルールに抵触してしまうためtrait出来ていない。
#[derive(Debug, PartialOrd, PartialEq, Eq, Hash)]
pub enum ContextKey {}

/// ContextはAPIの境界を超えて設定データやcancel signal,deadlineを運ぶ
pub trait Context: Send + Sync {
    /// このContextがキャンセルされたときに値が返ってくるようなFutureを返す
    ///
    /// # Example
    ///
    /// ```
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     println!("start sub()");
    ///
    ///     let (ctx, canceler) = ContextWithCancel::new(BackgroundContext::new());
    ///
    ///     tokio::spawn(async move {
    ///         println!("sub() is finished");
    ///         canceler.cancel();
    ///     });
    ///
    ///     let done = ctx.done().await;
    ///     assert_eq!(done.unwrap_err(), ContextError::Canceled);
    ///     println!("all tasks are finished");
    /// }
    /// ```
    fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>>;

    /// Contextが終了した理由を返す
    ///
    /// # Example
    ///
    /// ```
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     println!("start sub()");
    ///
    ///     let (ctx, canceler) = ContextWithCancel::new(BackgroundContext::new());
    ///
    ///     assert_eq!(ctx.err(), None);
    ///
    ///     tokio::spawn(async move {
    ///         println!("sub() is finished");
    ///         canceler.cancel();
    ///     }).await;
    ///
    ///     assert_eq!(ctx.err(), Some(ContextError::Canceled));
    ///     println!("all tasks are finished");
    /// }
    /// ```
    fn err(&self) -> Option<ContextError>;

    /// Contextに紐付けられたvalueを返す
    /// TODO(higumachan): 細かい活用は後で行う
    ///
    /// # Example
    ///
    /// ```
    /// #[tokio::main]
    /// async fn main() {
    ///     let ctx = ContextWithValue::new(BackgroundContext::new(), ContextKey::from("key", Arc::new("value".to_string()))); // Backgroundはrootとなる空のContext
    ///
    ///     assert_eq!(ctx.value(ContextKey::from("key")), None);
    ///
    ///     tokio::spawn(async move {
    ///         println!("sub() is finished");
    ///         canceler.cancel();
    ///     }).await;
    ///
    ///     assert_eq!(ctx.err(), Some(ContextError::Canceled));
    ///     println!("all tasks are finished");
    /// }    
    /// ```
    fn value(&self, key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError>;
}

mod tree_context {
    use super::*;

    /// trait Contextの代表的な実装として、ContextをTreeとして以下のようなノードで構成する
    /// - ContextWithCancel
    /// - ContextWithValue(unimplemented)
    /// - ContextWithDeadline(unimplemented)
    /// - BackgroundContext
    ///
    /// これらの各ノードは単一の機能を提供する。
    ///
    /// 木構造としては以下のような特性を持つ
    /// - cancelは子孫要素に伝播する
    /// - valueは祖先要素に問い合わせる
    pub trait TreeContext: Context {
        fn node(&self) -> Arc<Mutex<Node>>;
    }

    /// 子要素に対してCancelを伝播する事ができるContext
    pub trait CancelPropagate: Send + Sync + TreeContext {
        fn cancel_propagate(&self, error: ContextError);
    }

    pub struct Node {
        children: Vec<Arc<dyn CancelPropagate>>,
        parent: Option<Weak<dyn Context>>,
        canceled: Option<ContextError>,
    }

    /// Contextのcancelを行う単一機能を持つContext
    ///
    /// # Example
    ///
    /// ```
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     println!("start sub()");
    ///
    ///     let (ctx, canceler: Canceler) = ContextWithCancel::new(BackgroundContext::new()); // Contextと一緒にCancelerが返ってくる
    ///
    ///     tokio::spawn(async move {
    ///         println!("sub() is finished");
    ///         canceler.cancel();
    ///     });
    ///
    ///     let done = ctx.done().await;
    ///     assert_eq!(done.unwrap_err(), ContextError::Canceled);
    ///     println!("all tasks are finished");
    /// }
    /// ```
    pub struct ContextWithCancel {
        cancel_notify: Notify,
        tree_node: Arc<Mutex<Node>>,
    }

    impl ContextWithCancel {
        pub fn new<C: 'static + TreeContext>(context: Arc<C>) -> (Arc<Self>, Canceler<Self>) {
            let c: Arc<dyn Context> = context.clone();
            let this = Arc::new(Self {
                cancel_notify: Notify::new(),
                tree_node: Arc::new(Mutex::new(Node {
                    canceled: None,
                    children: vec![],
                    parent: Some(Arc::downgrade(&c)),
                })),
            });
            context.node().lock().unwrap().children.push(this.clone());
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

    /// Contextのcancelを実際に行うための構造体
    /// # Example
    ///
    /// ```
    ///
    /// #[tokio::main]
    /// async fn main() {
    ///     println!("start sub()");
    ///
    ///     let (ctx, canceler: Canceler) = ContextWithCancel::new(BackgroundContext::new()); // Contextと一緒にCancelerが返ってくる
    ///
    ///     tokio::spawn(async move {
    ///         println!("sub() is finished");
    ///         canceler.cancel();
    ///     });
    ///
    ///     let done = ctx.done().await;
    ///     assert_eq!(done.unwrap_err(), ContextError::Canceled);
    ///     println!("all tasks are finished");
    /// }
    /// ```
    pub struct Canceler<C: CancelPropagate> {
        context: Arc<C>,
    }

    impl<C: CancelPropagate> Canceler<C> {
        /// Contextのcancelを実際に行う関数
        pub fn cancel(&self) {
            self.context.cancel_propagate(ContextError::Canceled)
        }
    }

    impl TreeContext for ContextWithCancel {
        fn node(&self) -> Arc<Mutex<Node>> {
            self.tree_node.clone()
        }
    }

    impl Context for ContextWithCancel {
        fn done(&self) -> Pin<Box<dyn Future<Output = Result<(), ContextError>> + '_>> {
            Box::pin(ContextWithCancel::done(self))
        }

        fn err(&self) -> Option<ContextError> {
            self.tree_node.lock().unwrap().canceled.clone()
        }

        /// ContextWithCancelは値を持たないので親に問い合わせる
        fn value(&self, key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError> {
            self.tree_node
                .lock()
                .unwrap()
                .parent
                .as_ref()
                .expect("WithCancelは必ず親を持つ")
                .upgrade()
                .expect("親から先に開放されることはない想定")
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

    /// rootのContextになる空のContext
    pub struct BackgroundContext {
        root: Arc<Mutex<Node>>,
    }

    impl BackgroundContext {
        pub fn new() -> Arc<Self> {
            Arc::new(BackgroundContext {
                root: Arc::new(Mutex::new(Node {
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
            None
        }

        fn value(&self, _key: &ContextKey) -> Result<Arc<dyn Any>, ContextValueError> {
            Err(ContextValueError::NotFound)
        }
    }

    impl TreeContext for BackgroundContext {
        fn node(&self) -> Arc<Mutex<Node>> {
            self.root.clone()
        }
    }
}

#[tokio::main]
async fn main() {
    println!("start sub()");

    let (ctx, canceler) =
        tree_context::ContextWithCancel::new(tree_context::BackgroundContext::new()); // Backgroundはrootとなる空のContext

    tokio::spawn(async move {
        println!("sub() is finished");
        canceler.cancel();
    });

    let done = ctx.done().await;
    assert_eq!(done.unwrap_err(), ContextError::Canceled);
    println!("all tasks are finished");
}
