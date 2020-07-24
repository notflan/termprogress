# Termprogress - Terminal progress bars
Simple and customiseable terminal progress bars for Rust.

## Features

 - Customiseable, has a traits system to allow for passing any type of progress bar around
 - Prevents long titles from overflowing the terminal by using the [terminal_size][terminal-size] crate
 - Interfaces for easily manipulating bar
 
[terminal-size]: https://crates.io/crates/terminal_size

## How it looks

The bar at 50, 75, and 100%:

``` shell
[=========================                         ]: 50.00% some title
[=====================================             ]: 75.00% some other title
[==================================================]: 100.00% some other title
```

The spinner in 4 stages:

``` shell
Some title /
Some title -
Some title \
Some title |
```

## Getting started.
To quickly use the default bar and spinner, you can include the `prelude`:

``` rust
use termprogress::prelude::*;

let mut progress = Bar::default(); // Create a new progress bar

progress.set_title("Work is being done...");
/// *does work*
progress.set_progress(0.25);
progress.set_progress(0.5);
progress.println("Something happened");
progress.set_progress(0.75);
progress.println("Almost done...");
progress.set_progress(1.0);

/// completes
progress.complete();
```

Spinner:

``` rust
use termprogress::prelude::*;

let mut spinner = Spin::default(); //Create a new spinner
/// *does work*
spinner.bump();
spinner.bump();
progress.println("Something happened");
spinner.bump();
spinner.bump();

/// completes
progress.complete_with("Done!");
```

## Traits
The library comes with traits for progress bars: [`ProgressBar`][progress-bar], and [`Spinner`][spinner].

The default implementations for these are `Bar` and `Spin`, but you can provide your own implementations too for more customisability

[progress-bar]: ./src/inter.rs
[spinner]: ./src/inter.rs

``` rust
pub fn does_work<P: ProgressBar>(bar: &mut P) 
{
	//does work...
	bar.set_progress(0.5);
	//more work...
	bar.set_progress(1.0);
}

does_work(&mut Bar::default());
does_work(&mut MyBar::new());
```

## License
GPL'd with love <3

