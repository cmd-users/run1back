# run1back

Backend for the first run of our training for Hackathon 2023.  

## Setup

- 1. Install [Rust](https://www.rust-lang.org/).  

- 2. This project requires the `nightly` version of Rust. If it's not installed on your system yet, you can run this command to install it:  
```command
rustup toolchain install nightly
```

- 3. While inside the project directory, run the following command to use the `nightly` version of Rust for the project:  
```command
rustup override set nightly
```

- 4. Install the `diesel_cli`  
```command
cargo install diesel_cli --no-default-features --features postgres
```

## Run

- 1. Run postgres  
```command
docker-compose up -d postgres
```

- 2. Run db migrations  
```command
DATABASE_URL=postgres://rocket:rocket@localhost:5432/rocket diesel migration run
```

- 1. Start the server  
```command
cargo run
```

- 2. Open your browser at [`http://127.0.0.1:8000/`](http://127.0.0.1:8000).  
