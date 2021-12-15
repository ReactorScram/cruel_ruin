# Cruel Ruin

![](CruelRuin.jpeg)

A downloadable bad idea, named for the party-wiping final boss attack in 
[Golden Sun](https://en.wikipedia.org/wiki/Golden_Sun).

```rust
#[no_mangle]
pub extern "C" fn wow_rs () {
	println! ("Wow, it's Rust");
}
```

```cpp
extern "C" {
	void wow_rs ();
}

void wow_cpp () {
	cout << "Wow, it's C++\n";
	wow_rs ();
}
```

This is a template / reference for manually binding a Rust library to a C++ executable,
because I keep forgetting the exact CMake and Cargo syntax to use, and how
to pass strings each way.
