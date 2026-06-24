# my-project

#### Those are all warnings, not errors — note the last line: Finished 'dev' profile [unoptimized + debuginfo] target(s). The build succeeded. Cargo distinguishes warnings (compiles fine, just a heads-up) from errors (compilation fails) — you'd see error[E0...] and a nonzero exit if it actually failed.
#### Why you're seeing 15 of them: every function/struct field we wrote in this chapter is a private library stub that nothing ever calls — main.rs is still just Hello, world! and doesn't touch the library at all. The book's own walkthrough produces this exact same wall of warnings; it's testing that the module/path/privacy rules work, not building something runnable yet.
