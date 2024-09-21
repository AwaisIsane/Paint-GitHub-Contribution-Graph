# Paint GitHub Contribution Graph

This Rust project allows you to generate a custom GitHub contribution graph by creating commits in a separate repository.

## Overview

The program creates a `git_history` folder and makes commits to paint your desired pattern on the GitHub contribution graph. If the folder already exists, it will use the existing one without creating a new folder or deleting old commits. push the git_history folder to github to see the changes

![Demo](./demo.gif)

## Usage

To run the program, use one of the following commands:

```
cargo run
```

or

```
cargo run -- <year>
```

Replace `<year>` with the desired year for which you want to generate the contribution graph.
\*For the year 2000, there were 54 weeks. GitHub will remove the first Saturday from the contribution graph to maintain consistency with other years.

## Resetting the Graph

To reset the contribution graph, simply delete the `git_history` folder. The next time you run the program, it will create a new folder and start fresh.

i created this tool while i was learning more about rust and tui interfaces and decided to create this .
