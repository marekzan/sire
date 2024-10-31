# SIRE

A SImple REminder.  
This project is primarily a learning experience. If it fits your workflow, that's even better.

It was created to fulfill the need for a note-taking system that:

1. Bounds its notes to a specific folder.
2. Reminds the user of saved notes every time they enter a folder containing notes.

You can save notes, and `sire` will store them within the context of the current folder. For example, if you are in `/home/you/myWorkspace` and you add a note, `sire` will create an entry in its data format:

```yaml
- name: /home/you/myWorkspace
  content: mynote is now added
```

Now, every time you enter this folder, `sire` will display your notes in your terminal.

## Table of Contents

- [Installation](#installation)
- [Usage](#usage)
- [Commands](#commands)
- [Contributing](#contributing)
- [License](#license)

## Installation

To install this project, follow these steps:

1. Clone the repository:

   ```sh
   git clone https://github.com/marekzan/sire.git
   cd sire
   ```

2. Build the project:
   ```sh
   cargo build --release
   ```

## Usage

`sire` supports the following commands:

### check

Checks the current reminders. This will happen automatically every time you enter a folder with reminders.
Though you can also manually check for reminders.

```sh
sire check
```

### add

Adds a new reminder.

- `--content`: The content of the reminder.
- `--detached`: Whether the reminder is detached. Meaning it will not be bound to the current folder. This is useful for global reminders which are saved under a special \*global\* folder tag.

```sh
sire add --content "Buy groceries" --detached
```

### remove

TODO

```sh
sire remove
```

### Contributing

Contributions are welcome! Please follow these steps to contribute:

Fork the repository.  
Create a new feature branch.
Commit your changes.
Open a pull request.

## License

This project is licensed under either of:

- Apache License, Version 2.0, ([LICENSE-APACHE](./LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](./LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
