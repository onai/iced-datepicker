# DatePicker

DatePicker is an interactive calendar built with Iced. It lets the user pick a date in the calendar. 

![](./images/date_picker.gif)

# How to run

Run it with `cargo run`:
```
cargo run
```

# About the warning

![](./images/warning.png)

This warning pops up because our widget depends on two sub folders of **Iced**. However, **Iced** doesn't export these two modules. The version of these two crates in crates.io is also too old to use, so we have to use ```git + path``` to specify the version and location. In the future, this warning can be easily removed by bumping the versions of **Iced**, **Iced_native**, and **Iced_graphics**. For more information, refer to this issue https://github.com/rust-lang/cargo/issues/1462

# Dependencies

- [Iced: 94af34884667e78e231fb1904ae3e9fa785c9a7a](https://github.com/hecrj/iced/tree/94af34884667e78e231fb1904ae3e9fa785c9a7a)
