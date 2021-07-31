# sphluid

SPH fluid dynamics

## Parallelization

MPI like [book](https://doc.rust-lang.org/book/ch16-00-concurrency.html).

- init thread pool for each available processor
- assign tasks to threads 
  - if possible assign chunks, with decreasing chunk size
  - this way the scheduling is adaptive
  - e.g. each thread it's initialized with a chunk of size $tasks / (2*t)$ (t
    the number of threads), every time one has finished than is assigned a new
    chunk of size $min(remaining / t, threshold)$

## Serialization

Simply use [`serde`](https://serde.rs/) with
[`bincode`](https://github.com/bincode-org/bincode#example) backend.

Serialize a simple structure, json-like:

- HashMap: labels -> values
- values: strings, scalars, ndarrays

What to store?

- A: configuration parameters
- A: full final configuration
- B: time dumps

A is for reproducibility and continuation, B is for analysis.
