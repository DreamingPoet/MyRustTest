

use std::future::Future;
// 异步任务执行器
use futures::executor::block_on;

#[tokio::main]
async fn main() {

    // 任务(Task) 
    // 由 tokio::spawn或 Runtime::block_on 函数创建
    // https://skyao.io/learning-tokio/docs/introduction/glossary.html
    // async 并不创建新的任务，每个组合的部分被说成是 “在同一个任务中”。


    // 调用，但是内部代码不会执行
    // future_print();

    // 用一个阻塞的执行器，调用函数，内部代码会执行
    block_on(future_print());
    block_on(future_print1());


    // 用Tokio 执行器执行, 内部代码会执行
    // 但是有可能还没等待返回结果，主线程就结束运行了，就会看不到打印结果
    // 所以需要调用.await 让主线程等待返回结果了，才往下执行
    // future.await 表示任务执行到此处的时候，让任务调度器等待 future 的返回结果后，再往下执行
    let handle = tokio::spawn(future_print());
    let handle1 = tokio::spawn(future_print1());
    let _ = handle.await.unwrap();
    let _ = handle1.await.unwrap();
    
    // 异步执行块，归属于同一个可调度任务
    let world = async {
        println!(" world!");
    };
    let my_future = async {
        print!("Hello");
        world.await;
    };
    block_on(my_future);


    let world2 = async {
        println!(" world2!");
    };
    let say_world = tokio::spawn(world2);

    let _ = say_world.await.unwrap();


    // 当tokio spawn a task, 它的类型必须是 'static 的。
    // 这意味着生成的任务不能包含对任务之外拥有的数据的任何引用。
    // 默认情况下，变量不会被 move 到异步块中
    // 改为 task::spawn(async move { 将指示编译器将v move 到任务中。

    // 如果数据必须被多个任务同时访问，那么它必须使用同步原语（如Arc）进行共享。
    let v = vec![1, 2, 3];
    // error!
    // task::spawn(async {
    //     println!("Here's a vec: {:?}", v);
    // });
    
    tokio::spawn(async move {
        println!("Here's a vec: {:?}", v);
    });


}


fn future_print() -> impl Future<Output = ()> {
    // 返回类型是impl Future<Output=()>，这意味着可以返回任意实现了std::future::Future特征的类型
    // async 函数或者 async 块 返回[Future] 
    // std::future::Future只是标识了异步计算的任务，并不会立即执行
    async {
        println!(" future_print !");
    }
}

// 语法糖 async, 等同于上面的 future_print
async fn future_print1() {
    // 返回类型是impl Future<Output=()>，这意味着可以返回任意实现了std::future::Future特征的类型
    // async 函数或者 async 块 返回[Future] 
    // std::future::Future只是标识了异步计算的任务，并不会立即执行
    println!(" future_print1 !");
}

// 异步函数(Async function)

async fn do_stuff(i: i32) -> String {
    // do stuff
    format!("The integer is {}.", i)
}

// 异步函数都可以被改写成返回一个future的普通函数
// the async function above is the same as this:
fn do_stuff1(i: i32) -> impl Future<Output = String> {
    async move {
        // do stuff
        format!("The integer is {}.", i)
    }
}


// 由异步块创建的 future 在执行之前不会做任何事情
// 所以调用异步函数在其返回的 future 被执行之前不会做任何事情