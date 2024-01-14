# Clihoot

A quiz app 🤔 for activating students in the classroom 🏫, working purely in the terminal 💻.

## Quickstart

To host a game:
`cargo run --bin server -- -p 8080 -q sample_quizzes/quiz-1.yml`

To join a game:
`cargo run --bin client -- --addr="127.0.0.1:8080"`

## Project Overview

Clihoot is a Rust-based application that brings the fun of Kahoot quizzes to the
command line. The goal is to provide an interactive and engaging way for
teachers to conduct quizzes and for students to participate using the terminal.
We have beautiful terminal UI 🖥️, syntax highlighting of code 🌈 and even fancy 8-bit music 🎧🎵.

## Teacher manual

Since clihoot is terminal App and we expect that the users know how to operate computers,
we decided for the most user friendly way to define quizzes - Your Favorite Text Editor 😎.
The structure is defined in YAML format to be easily readable and editable.
This is the most basic template for the quiz file:

```yaml
quiz_name: Quiz
questions:
  - text: Sample question?
    time_seconds: 42
    is_multichoice: false
    choices:
      - text: yes
        is_correct: true
      - text: no
  - text: What does this code do?
    code_block:
      language: rust
      code: >
        fn main() {
            println!("42");
        }
    time_seconds: 60
    is_multichoice: true
    choices:
      - text: Nothing useful
      - text: It prints 42
        is_correct: true
      - text: It fails to compile
      - text: It answers to the ultimate question of life
        is_correct: true
```

The file starts with the name of the quiz, followed by sequence of questions.
Each question can have from 2 to 4 answers. At least one answer has to be correct.
Question can be either single or multi choice, meaning that the student can select
at most one or any number of answers respectively.

Each question can optionally contain a block of code defined by the `code_block`.
The code will be plotted with pretty colors on the terminal screen.
We support all common programming languages for the syntax highlighting.

The `language` can be either just name of the language (e.g. `rust`, `python`, `perl`)
or the file extension expected for the given language (e.g `rs`, `py`, `pl`).
For full list of supported languages see: <https://github.com/slimsag/Packages>.

By using the `-t|--theme` option teacher and each student can choose
their favorite theme for the syntax highlighting (we even have light themes 🤮).
We provided multiple sample quizzes that you can look through in the `sample_quizzes` folder.

When the quiz file is finished, we can launch the server with `-q, --questions-file <QUESTIONS_FILE>`
option to load the file and start the quiz. When the game is launched, the
questions will appear in the same order as they are defined in the file.
But the order can be randomized if the server is launched with the `-r|--randomize-questions`
option. The same applies for the answers, they will appear in the same order as they are
defined, but that can be changed with the `-a|--randomize-answers` option.
The server can also be launched with option `-p|--port` to define other than port than the default `8080`.

## Student manual

Your goal is to score as many points as you can. You score points for correct answers.
You can score even more points if you are faster than your classmates.

How to play the game:

1. First, open terminal on your computer (program can be named Terminal, Konsole, xterm, cmd - on Windows, or something similar).

2. Locate the clihoot client program (usually named clihoot-client) and execute it in your terminal. Your teacher
   will give you the address, it can be something like `teacher.example.com` or `192.168.0.60:4444` - which you can specify using option `--addr=<address>`.

   You can also turn off the music by passing `--silent` option. But we will be sad 😢.

3. When you successfully connect to the clihoot server, you will be asked to enter the nickname and color. The choice is up to you 😉.

4. Then wait until all your classmates also connect and then you will choose answers you
   think are correct with `Spacebar` key
   and send those answers with `Enter` key. There is a single-choice question
   where you can select only one answer or multi-choice questions
   where more answers can be correct.

5. After each round the score will show up informing you about your ranking.

6. If you need help during the game, press `h` key. Good luck.

## Build & deploy instructions

Project can be easily built using `cargo` (assuming you have rust compiler correctly setup).

1. Go into clihoot root source directory. From there, you can
   run `cargo build --bin client` for client
   and `cargo build --bin server` for server.

2. This will create `client` and `server` executable files
   in `target/`. If you distribute these executables
   for other different computers,
   make sure to compile for correct architecture.

3. Copy the `server` executable into the teacher's computer.
   All students must have ability to connect to this computer
   via internet or LAN. Watch out for firewalls 🔥🔥🔥.

4. Copy the `client` executable into the students'
   computers and give them the IP address
   (and port if custom) of the teacher's computer.

5. Start the `server` program from the teacher's
   computer and tell the students to join.

## Project Architecture

The project is a cargo workspace with three members:

- `client` (binary),
- `server` (binary)
- and `common` (library, referenced by both `client` and `server`).

### Important used libraries

- `tokio` for async support
- `actix` actor pattern for good abstraction in an async environment (both server and client
  have to constantly listen for incoming messages while re-rendering the UI)
- `tungstenite` for websocket support
- `ratatui` for terminal UI
- `syntect` for syntax highlighting in the terminal
- `log`, `clap`, `anyhow`, `serde_json`, ...

### Logging

In the project, the crate `log` is used as a logging facade. `simplelog` is then
used as the implementation. The logging is configured in the `main.rs` files at the very top.

Logs are saved to the directory where the binary was run from. The log files are named
`clihoot_server_logs.log` and `clihoot_client_logs_<player-uuid>.log`.

### Communication protocol

Server and client exchange messages over the network.
Client is allowed to send `ClientNetworkMessage`
and server is allowed to send `ServerNetworkMessage`.

First, the server (`Lobby` actor and `Teacher` actor) must be running.
Server listens for incoming TCP connections on
the given port and accepts them. For each connection, a `Websocket` actix Actor is spawned
to handle the connection.

The both sides then promote the TCP connection to a websocket connection. Then:

- The client sends a `TryJoinRequest`, asking the server whether it can join the quiz.
- The server responds with a `TryJoinResponse`, either accepting or rejecting the request.
- If the request was accepted, the client MAY send a `JoinRequest`, containing the name of the player and chosen color.
- The server responds with a `JoinResponse`, admitting the player to the quiz.
- If the player was admitted to the lobby, the server sends a `PlayersUpdate` message to all players,
  informing them about the new player.

The game consists of multiple questions. Each question has several phases: first, we get the question,
then we answer it, then we see the correct answers, then we see the leaderboard.

PHASE 1: When the teacher chooses to start a question, the `Lobby` actor instructs all joined players
(i.e., their `Websocket` actors) to start the question with a `NextQuestion` message.

PHASE 2: When a student answers a question, the client sends a `AnswerSelected` message to the server.
Server registers this and sends `QuestionUpdate` message to all joined players, so that everyone
knows how many players have already answered.

PHASE 3: After the question is finished (either via timeout, or that everybody answered,
or that the teacher chose to end it sooner), the server sends `QuestionEnded` message to all joined players
(and the `Teacher` actor). The message contains the correct answer and the statistics of the question.

PHASE 4: When the teacher chooses to move on, the server sends a `ShowLeaderboard` message to everyone. This
contains the current leaderboard.

The teacher can then move to next question, repeating the cycle.

Additional options:

- Teacher can kick a player when they are in the lobby or on the leaderboard screen.
- Teacher can end the entire game with CTRL+C

### Rendering

There is some non-trivial amount of setup when working with `ratatui`. Therefore,
in crate `common`, there is `actor.rs` containing a generic `TerminalActor<T>` implementation
which handles startup and teardown of the terminal. Admittedly, there is a lot of boilerplate
code, but the resulting pattern is quite nice.

Both client and server then only provide `StudentTerminal` and `TeacherTerminal` structs,
which implement necessary traits for the `TerminalActor` to work. Specifically, they
provide functions to:

- draw the actual UI (trait `TerminalDraw`),
- handle inputs from the keyboard (trait `TerminalHandleInput`),
- extra steps to end the program (trait `TerminalStop`)
- handle clock ticks (2 per second, trait `TerminalHandleTick`)
- and other traits for handling individual messages

## Testing

The project is tested using `cargo test --features test`. The are some unit tests covering basic
logic of input/output etc, but most tests are integration tests which test the communication protocol.

Because the program draws in a terminal, but this would be badly displayed during tests,
the tests are run with a flag `--features test`, which conditionally compile in a `TestBackend`
of the `ratatui` library. This backend does not draw anything to the actual terminal, but to a memory buffer.

To introduce timeout and fixture injection into integration tests, a crate `rstest` is used.

## Other design decisions

These are notes which should be here, but did not fit anywhere else.

### Why Server = Teacher

The teacher's machine becomes the server, where students connect to.
We thought about having a dedicated permanently-running server, and then having
a teacher which would connect and authenticate to it, but we decided that this
would be too needlessly complicated. We wanted users to host their own server,
and this is the simplest way to do it.

However, it does mean that the teacher must provide connectivity (and e.g. configure his
own firewall).

### Why ratatui

When choosing a crate to draw the UI, we considered factors:

- async support
- support for syntax highlighting
- support for creating custom widgets
- documentation

From the available crates, we chose `ratatui` because it excelled in all of these factors.

### Why server is 2 threaded but the client only single-threaded

In one thread, the server handles all network communication (`Lobby` and all `Websocket`s). In the other thread,
it handles all the UI (`Teacher`). There is little communication between these two threads, so we decided
to keep them separate.

However, the client is single-threaded, because it is not as complex as the server. It only has one
network connection, so it made sense to keep it single-threaded.

### Why syntect_tui was not used

Originally, `ratatui` was selected because we knew that `syntect_tui` provided easy
syntax highlighting. However, we found out that `syntect_tui` was not compatible with
the latest version of `ratatui`. Therefore, we looked at their Github and implemented
the necessary functionality ourselves, even improving their implementation 😎. See `translate_font_style`
in `highlight.rs` where modifiers are inserted.
