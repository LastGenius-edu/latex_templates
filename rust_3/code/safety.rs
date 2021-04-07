// SAFETY: This can be risky if called from separate threads, but `Cell`
// is `!Sync` so this won't happen. This also won't invalidate any
// pointers since `Cell` makes sure nothing else will be pointing into
// either of these `Cell`s.
unsafe {
    ptr::swap(self.value.get(), other.value.get());
}

// SAFETY: This can cause data races if called from a separate thread,
// but `Cell` is `!Sync` so this won't happen.
mem::replace(unsafe { &mut *self.value.get() }, val)
