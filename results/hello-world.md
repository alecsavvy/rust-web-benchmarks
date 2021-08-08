```
# In "frameworks/" tab
cargo run --release --bin=hyper-hello-world

# In "rewrk/" tab
echo "Results for hyper:" && ./rewrk -t 12 -c 500 -d 10s -h http://localhost:3000/
Results for hyper:
Beginning round 1...
Benchmarking 500 connections @ http://localhost:3000/ for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    4.46ms   0.80ms   0.11ms   19.85ms
  Requests:
    Total: 1104394 Req/Sec: 110406.86
  Transfer:
    Total: 92.68 MB Transfer Rate: 9.27 MB/Sec

# In "frameworks/" tab
cargo run --release --bin=axum-hello-world

# In "rewrk/" tab
echo "Results for axum:" && ./rewrk -t 12 -c 500 -d 10s -h http://localhost:3000/
Results for axum:
Beginning round 1...
Benchmarking 500 connections @ http://localhost:3000/ for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    4.46ms   0.95ms   0.12ms   12.97ms
  Requests:
    Total: 1100380 Req/Sec: 110003.32
  Transfer:
    Total: 93.40 MB Transfer Rate: 9.34 MB/Sec

# In "frameworks/" tab
cargo run --release --bin=actix-hello-world

# In "rewrk/" tab
echo "Results for actix:" && ./rewrk -t 12 -c 500 -d 10s -h http://localhost:3000/
Results for actix:
Beginning round 1...
Benchmarking 500 connections @ http://localhost:3000/ for 10 second(s)
  Latencies:
    Avg      Stdev    Min      Max
    4.87ms   0.55ms   0.09ms   13.70ms
  Requests:
    Total: 1010920 Req/Sec: 101059.34
  Transfer:
    Total: 124.37 MB Transfer Rate: 12.43 MB/Sec
```
