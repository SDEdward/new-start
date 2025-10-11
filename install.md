## How to Compile Rust Stuff

If you're here, you better know how to compile Rust. If not, here's the bare minimum:

### Linux:

```bash
sudo [your_package_manager] install cargo
```

### Windows:

idk man, probably install Rust from [https://rustup.rs](https://rustup.rs) and pray; im not a dirty windows user

---

### Setup:

1. Go into any empty folder:

   ```bash
   cargo init
   ```

2. Replace the auto-generated `src/main.rs` with the `main.rs` from one of our shitty projects.

3. If you're using `_num_guess`, `_gamble`, or `_decho`, you **must** add the `rand` crate:

   ```bash
   cargo add rand
   ```

4. Then just run it:
   ```bash
   cargo run
   ```
