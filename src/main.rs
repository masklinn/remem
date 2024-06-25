

#[global_allocator]
static GLOBAL: alloc::TotalAllocator = alloc::TotalAllocator;

fn main() {
    let mut quiet = false;
    let mut bytes = false;
    let mut unicode = true;
    for arg in std::env::args_os().skip(1) {
        if arg == "-q" || arg == "--quiet" {
            quiet = true;
        } else if arg == "-h" || arg == "--help" {
            eprintln!("Pass a single regex to stdin.

Options:

-h, --help
\tthis help
-a, --ascii
\tdisable unicode handling
-b, --bytes
\tuse bytes module
-q, --quiet
\tdisable reminder to send the regex to stdin
");
            return;
        } else if arg == "-b" || arg == "--bytes" {
            bytes = true;
        } else if arg == "-a" || arg == "--ascii" {
            unicode = false;
        } else {
            eprintln!("Unknown option {arg:?}");
            return;
        }
    }
    if !quiet {
        eprintln!("Pass a single regex to stdin");
    }
    let Some(s) = std::io::read_to_string(std::io::stdin()).ok().filter(|s| !s.is_empty()) else {
        eprintln!("Prints the allocation increase from base to compiling the regex, then the decrease from dropping it.");
        return;
    };

    if bytes {
        let before = alloc::allocated();

        let r = regex::bytes::RegexBuilder::new(&s)
            .unicode(unicode)
            .build()
            .unwrap();

        let after = alloc::allocated();
        
        let c = r.captures(b"this is a test");
        let after2 = alloc::allocated();

        // ensures the values are not discarded
        drop(std::hint::black_box(c));
        drop(std::hint::black_box(r));

        println!("{} {}", after - before, after2 - after);
    } else {
        let before = alloc::allocated();

        let r = regex::RegexBuilder::new(&s)
            .unicode(unicode)
            .build()
            .unwrap();

        let after = alloc::allocated();

        let c = r.captures("this is a test");
        let after2 = alloc::allocated();

        // ensures the values are not discarded
        drop(std::hint::black_box(c));
        drop(std::hint::black_box(r));

        println!("{} {}", after - before, after2 - after);
    }
}

mod alloc {
    use std::alloc::{GlobalAlloc, Layout, System};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static TOTAL_ALLOC: AtomicUsize = AtomicUsize::new(0);
    static TOTAL_DEALLOC: AtomicUsize = AtomicUsize::new(0);

    pub struct TotalAllocator;

    unsafe impl GlobalAlloc for TotalAllocator {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            TOTAL_ALLOC.fetch_add(layout.size(), Ordering::Relaxed);
            System.alloc(layout)
        }

        unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
            TOTAL_DEALLOC.fetch_add(layout.size(), Ordering::Relaxed);
            System.dealloc(ptr, layout)
        }
    }

    pub fn allocated() -> usize {
        TOTAL_ALLOC.load(Ordering::Relaxed) - TOTAL_DEALLOC.load(Ordering::Relaxed)
    }
}
