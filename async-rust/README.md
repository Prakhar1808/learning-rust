found this helpful video from let's get rusty, so let's get rusty and learn async rust
> i am going to document my learning along with some code snippets if needed, i am very rusty rn (pun intended), but i really want to learn something interesting after today EndSem lmao

### Basics
#### tokio


#### Futures


#### Synchronous and Asynchronous

### `.await`
+ use await if you need to wait for a result to be returned, await grants more control rather than blocking
```rust
async fn fetch_user(){
    println!("Fetching user");
}

#[tokio::main]
async fn main() {
    let user = fetch_user().await;
}
```
+ it benefits from using `.await` because we can about using the result of the code later

### `tokio::spawn`
+ use spawn if you need a completely different task to be created
we also have the ability to make "fire-and-forget" tasks
```rust
async fn logging(){
    println!("Logging Event");
}

#[tokio::main]
async fn main() {
    tokio::spawn(logging());
}
```

### `tokio::join!`
+ use join if you have independent futures that can run concurrently

```rust
async fn load_config(){
    println!("Loading Config");
}

async fn connect_to_db(){
    println!("Connecting to db");
}

#[tokio::main]
async fn main() {
    let (config, db) = tokio::join!(
        load_config(),
        connect_to_db(),
    );
}
```

### `tokio::join!` vs `.await`
+ because of interleaving `tokio::join!` can be faster than `.await`
+ for await for example

```rust
async fn load_config(){
    println!("Loading Config");
}

async fn connect_to_db(){
    println!("Connecting to db");
}

#[tokio::main]
async fn main() {
    let config = load_config().await;
    let db = connect_to_db().await;
}
```

### `tokio::select!`
+ use select if you plan to race futures against each other

+ similar to join but only one `future` "WINS", the one which runs fastest wins and the other future is cancelled
+ "you are not selecting the fastest winner, you're also cancelling the losers"
+ cancellation means the losing futures are automatically dropped (unlike go-routines)
+ the losers will be cancelled just as the fastest wins, so maybe left in broken state?

+ shutting down the stream can be very important

#### cancellation safety
```rust
tokio::select!{
        _ = send_message(&mut stream, payload) => {
            // message sent
        }
        _ = sleep(Duration::from_secs(5)) => {
            // timeout wins

            // critical
            // treating connection as corrupted
            let _ = stream.shutdown().await;
        }
    }
```

+ what makes this pattern safe is that we refuse to keep using the connection afterward and that's what gives us cancellation safety

+ best for timers or reads
+ can be dangerous for writes because it can get interupted halfway through

### Sync <-> Async Interop
+ Mixing Sync and Async Code

```rust
#[tokio::main]
async fn main(){
fetch_options_and_calc_greeks("SPQ").await;
fetch_options_and_calc_greeks("AWD").await;
fetch_options_and_calc_greeks("PSY").await;

//....
// consider the below to be a very cpu intesive/heavy task
let result = calc_greeks(quotes);
};
```

can be changed to

```rust
#[tokio::main]
async fn main(){
    let a = fetch_options_and_calc_greeks("SPY");
    let b = fetch_options_and_calc_greeks("QQQ");
    let c = fetch_options_and_calc_greeks("INN");

    // Futures run concurrently in Root Task
    let (_a, _b, _c) = tokio::join!(a. b. c);

    //....
    // the cpu heavy task would block the root tasks and futures and hence we use spawn_blocking and create a block thread pool (named blocking thread pool) separately for this heavy task which was blocking the futures (on non-blocking thread pool)
    // basically: Tokio has worker threads for async tasks and separate blocking pool for synchronous work

    let result = tokio::task::spawn_blocking(move || calc_greeks(quotes)).await?;
    
    // this frees up the non-blocking thread pool for other async task
    // once the blocking's job is complete the root task is resumed on any avaiable thread (on the non-blocking thread pool)
}
```
#### how it works (The mental model)
+ start on the async non-blocking pool
+ offload cpu-heavy work to blocking pool
+ let the root task set off the thread while it runs
+ then resume later on any available thread

hence the runtime is not slowed down by one cpu-heavy task
