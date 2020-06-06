## Custom widget - DatePicker

![](./images/date_picker.gif)

### Run steps

1. Add dependencies

>```[warning]``` The iced library is updated quikly. This widget is based on the master branch of iced repository.<br/>```Commit number```: 94af34884667e78e231fb1904ae3e9fa785c9a7a

Download the iced project, put the date_picker into ```example``` folder.

2. Change the ```Cargo.toml``` of iced

Add the following content at **[workspace] members**:
```
"examples/date_picker",
```

3. run it with `cargo run`:
```
cargo run --package date_picker
```

### Denpendencies

- [Iced: 94af34884667e78e231fb1904ae3e9fa785c9a7a](https://github.com/hecrj/iced/tree/94af34884667e78e231fb1904ae3e9fa785c9a7a)

### License - TODO

[`main`]: src/main.rs
