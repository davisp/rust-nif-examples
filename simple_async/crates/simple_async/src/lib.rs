extern crate rustler;

mod atoms;

use std::future::Future;
use std::pin::Pin;
use std::sync::{Mutex, MutexGuard};

use genawaiter::GeneratorState;
use genawaiter::sync::{Gen, Co};

use rustler::resource::ResourceArc;
use rustler::{Encoder, Env, NifResult, Term};

type Coroutine = Gen<i32, i32, Pin<Box<dyn Future<Output = i32> + Send>>>;

struct AsyncNIF {
    gen: Coroutine
}

#[repr(transparent)]
struct AsyncNIFResource(Mutex<AsyncNIF>);

impl AsyncNIFResource {
    fn lock(&self) -> MutexGuard<'_, AsyncNIF> {
        self.0.lock().unwrap()
    }
}

impl From<AsyncNIF> for AsyncNIFResource {
    fn from(other: AsyncNIF) -> Self {
        AsyncNIFResource(Mutex::new(other))
    }
}

pub fn on_load(env: Env, _load_info: Term) -> bool {
    rustler::resource!(AsyncNIFResource, env);
    true
}

async fn do_things(co: Co<i32, i32>) -> i32 {
    let mut curr = 0;
    loop {
        curr = co.yield_(curr).await;
        if curr > 100 {
            break;
        }
    }
    curr
}

#[rustler::nif]
fn new<'a>(env: Env<'a>) -> NifResult<Term<'a>> {
    let nif = AsyncNIF{gen: Gen::new_boxed(do_things)};
    Ok((atoms::ok(), ResourceArc::new(AsyncNIFResource::from(nif))).encode(env))
}

#[rustler::nif]
fn next<'a>(env: Env<'a>, resource: ResourceArc<AsyncNIFResource>, val: i32) -> NifResult<Term<'a>> {
    let mut nif = resource.lock();
    match nif.gen.resume_with(val) {
        GeneratorState::Yielded(val) => {
            Ok((atoms::more(), val).encode(env))
        }
        GeneratorState::Complete(val) => {
            Ok((atoms::complete(), val).encode(env))
        }
    }
}

rustler::init!("simple_async", [new, next], load = on_load);