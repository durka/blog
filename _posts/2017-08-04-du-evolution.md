---
layout: post
title: Evolution of a simple `du -s` clone
commentIssueId: 1
---

# {{ page.title }}

So, the other day I was writing a simple utility to automate some repetitive tasks in my research, involving processing raw data files and transferring them to a remote server. The transfer itself uses `rsync`, and `rsync` doesn't have a progress bar. OK, it _does_ have [a way][so-answer] to output progress, but it isn't a bar and the percentage tends to jump around randomly, even when you pass flags that supposedly make it [not do that][so-comment]. Part of the issue is that measuring progress is hard: `rsync` might not even transfer all the files, since it diffs as it goes, and estimating progress when filesystems and networks are involved [can be fraught][xekcedixapare]. My idea was to show two progress bars simultaneously: one counting the files transferred as a percentage of the total number, and one watching the size of the destination directory as a percentage of the total data to be transferred. The actual progress are produced by [`indicatif`][indicatif], which I recommend -- but that's a subject for another post. For the latter calculation, I needed a function to measure the size of a directory on the local filesystem. Yes, I could just shell out to `du` and parse the output, but that's no fun. And so, several yaks later, we have this post.

[so-answer]: https://serverfault.com/a/441724
[so-comment]: https://serverfault.com/questions/219013/showing-total-progress-in-rsync-is-it-possible#comment921231_441724
[indicatif]: https://docs.rs/indicatif

[xekcedixapare]: https://xkcd.com/612/

What I want to show is the process of iteratively making the code better and more expressive using several crates from crates.io. The code is [available][code].

[code]: https://github.com/durka/blog/blob/master/_posts/du-evolution/code/du

## Boilerplate

First, some boilerplate for errors and a driving function. This is mostly shared between all versions of the code that I'll show here.

We'll use [`error-chain`][error-chain] for dead-simple error propagation. Add a dependency...

[error-chain]: https://docs.rs/error-chain

{% highlight toml %}
{% include includemethod file='du-evolution/code/du/Cargo.toml' item='[dependencies]' after=1 %}
{% endhighlight %}

...and make an `Error` enum with just one clause for chaining `std::io::Error`s...

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v1.rs' item='error_chain!' %}
{% endhighlight %}

...and finally, here's a simple `main` to drive the `local_du` function.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v1.rs' item='fn main' %}
{% endhighlight %}

## v1: quick and dirty

Here is the first version of my `local_du` function. You'll notice that I like chaining iterators. The implementation is a fairly straightforward depth-first search of the directory structure. We list the directory entries, recurse when encountering a directory (line 9), and use `fold` to add up the results.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v1.rs' item='#[macro_use]' after=4 %}
{% endhighlight %}

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v1.rs' item='fn local_du' %}
{% endhighlight %}

Now, this code is bad. It's ugly and highly indented -- you had to scroll to the right to read it, if you even bothered. I had to clumsily add in the size of the original directory as the initial value for `fold`. It handles errors eagerly because that was the simplest thing that occurred to me at the time. The actual functionality that I advertised (depth-first search and summation) is obscured by irrelevant details and closing braces. However, it does work!

At this point... well, at this point I committed the working[^1] code and continued with my research. But when I came back to the issue, my first idea was to use the awesome [`walkdir`][walkdir] crate to invert the function structure. `walkdir` provides an iterator that factors out the recursive depth-first search, letting you deal directly with a linear stream of files and directories.

[walkdir]: https://docs.rs/walkdir

[^1]: It did not actually work 100% correctly, as I neglected to count directory sizes in the first version (not shown), but it was close.

## v2: flip the script

We'll need one more dependency.

{% highlight toml %}
{% include includemethod file='du-evolution/code/du/Cargo.toml' item='[dependencies]' after=2 %}
{% endhighlight %}

And import the iterator struct.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v2.rs' item='extern crate walkdir' after=2 %}
{% endhighlight %}

It turns out we don't need to chain `io::Error`s anymore, so that stanza can be deleted, leaving just one for `walkdir::Error`. I could just remove the whole chain and return `Result<u64, walkdir::Error>`, but in a real program, there would likely be more error types.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v2.rs' item='error_chain!' %}
{% endhighlight %}

As I mentioned, using the `walkdir` iterator inverts the control flow, so you won't see any recursion in this iterator chain.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v2.rs' item='fn local_du' %}
{% endhighlight %}

Hey, this code is looking a lot better! It's almost 64% shorter, and I think it is way more expressive. You can read it from top to bottom:

- Iterate over the paths,
    - for each one:
        - try getting its metadata;
        - if that worked, get the size;
        - if it didn't, chain the error;
    - then add up all the sizes while propagating errors.

This time, errors are handled lazily, waiting until the fold to do anything. The behavior is different from v1: in this case, any error will cause the whole thing to fail, while previously erroring files or directories would just be skipped. But for this application that doesn't matter to me, and I haven't run into any such errors while testing, so if one does happen, it probably indicates something alarming and maybe it's better to fail loudly.

Anyway, a few aspects of this are still annoying to me. For one, there's friction when using iterator adapters, because the elements being yielded are `Result`s. So I have `Result::and_then` and such nested inside `Iterator::map`. We're doing fallible operations inside the closures, but we can't use `?`. The friction gets worse during the `fold` operation, which has to do some gymnastics to propagate the first error until it can be question-marked afterwards. I dimly remembered a crate that could help with this. Enter [`fallible-iterator`][fiter]!

[fiter]: https://docs.rs/fallible-iterator

## v3: final form

It's one more dependency.

{% highlight toml %}
{% include includemethod file='du-evolution/code/du/Cargo.toml' item='[dependencies]' after=3 %}
{% endhighlight %}

And we import the trait plus a free function that converts any `Iterator<Item=Result<T,E>>` to a `FallibleIterator<Item=T,Error=E>`.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v3.rs' item='extern crate fallible' after=1 %}
{% endhighlight %}

When you have a `FallibleIterator`, the familiar iterator adapters, like `map`, `filter`, and `fold` only need to deal with `Ok` values, and errors are propagated automatically. `FallibleIterator::and_then` is like `map`, but much like `Result::and_then` it returns a new `Result`, so it can turn `Ok` values into `Err`s.

{% highlight rust %}
{% include includemethod file='du-evolution/code/du/src/bin/v3.rs' item='fn local_du' %}
{% endhighlight %}

This is the final form of the code, and I find it beautiful. The intent reads directly from the code: iterate over directory entries, try to look up the metadata and pull out the size, then add up the results. Error handling behavior is identical to v2, but it is entirely abstracted away, by the built-in `?` and by `fallible-iterator`.

## Free lunch

Finally, a word about zero-cost abstractions. You might think I paid a price for simplifying the code and lazifying the error handling. But I didn't! Compiling in debug mode, you would be right. But nobody compiles in debug mode, right? It's just too slow for real work. In release mode, these programs have very nearly identical runtimes, validating Rust's core promise of _zero-cost abstractions_. Here's the data:

I ran each version of the code on the same directory. For each, I ran it 5 times to warm up whatever caches there might be, then 25 times to collect timing data. Incidentally, there is 2.29 GB of data in the subject directory, which is my `~/.cargo`.

| Version | Mean      | Std     |
|---------|-----------|---------|
|   v1    | 7.7860 s  | 0.28918 |
|   v2    | 7.9552 s  | 0.21973 |
|   v3    | 7.7670 s  | 0.26790 |

These differences are not statistically significant (p > 0.05).

## Unresolved questions

- My `local_du` consistently gives values that are slightly lower, about 5%, than `du -s`. Any suggestions about what I might be missing?
- Compliments, flames, comments about my code, writing or CSS (first post here!), please comment below or on reddit!

