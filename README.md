# ExpressionEvaluator
Rust Web App using Yew to perform the functions of a 4-function calculator

# How to access from web
The web app can be accessed [here](https://likey00.github.io/ExpressionEvaluator/)

# How to run locally
First, download rust from [here](https://www.rust-lang.org/tools/install)

Next, use cargo to install trunk by running:
```cargo install trunk```

Finally, add the WASM build target by running:
```rustup target add wasm32-unknown-unknown```

Now you can run the app locally from the project folder with:
```trunk serve --open```
