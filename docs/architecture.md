# Architecture

## Application-level architecture

The application is driven by an event loop which reads user input and passes it on to a UI coordinator. The UI coordinator has a reference to a calculator that in turn stores and evaluates the user input. Evaluation is divided into two separate steps: parsing and evaluation. 

Each line of user input may take up several lines in the UI. The UI coordinator keeps track of which line in the UI belongs to which line of user input and updates and draws them accordingly.

To make sure that each update is reflected on all affected line, each update will lead to every line being parsed and evaluated. With the current grammar on my machine it takes less than 7µs to parse and evaluate a relatively complex input. As the grammar becomes more complex and we add operations that take more time it might make sense to keep track of dependencies for each line and only re-evaluate the lines that need updating.

## Crates

### UI

The UI layer uses `winit` for windowing and `wpgu` and `wgpu-glyph` for drawing.

User interaction leads to `winit` window events that are passed to a UI coordinator. The UI coordinator keeps track of the necessary state, like dimensions and the content on the screen, and has some input handlers that updates the state according to the user events.

The UI coordinator doesn't actually store the textual content that's shown in the window. Single lines of user input can potentially be split across multiple lines in the UI. The UI coordinator keeps track of which line and region in the storage a line in the UI corresponds to, and that information is later used to create the `wgpu-glyph` data necessary to draw the right text in the right place.

### Storage

The storage is represented as a list of strings, that always must contain at least one string. There is a wrapper around `Vec` that takes care of all necessary house keeping.

### Parser

The parser consists of two consecutive steps. The first step is a lexing step that uses `nom` to build a token tree consisting of a list of tokens. A token in this case can be, for example, a nested token tree, a literal or an operator. The second step takes the token tree produced by the lexer and builds an expression from it.

Since `raekna` supports infix notation for the basic arithmetic operations we have to make sure to follow the order of operations. To support this the operators are separated from the other tokens and then combined in the correct order.

### Evaluator

The evaluator crate contains everything necessary to evaluate an expression into a literal, including the functions for evaluating the different operations on different combinations of literals.