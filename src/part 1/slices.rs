// Slices are dynamically sized array-like objects. The term dynamically sized means that their size is not known at compile time. 
// Yet, like arrays, these don't expand or contract. The use of the word dynamic in dynamically sized is closer in meanings to dynamic typing.
// Another important use for slices in their ability to act as a view on arrays. The term "view" here is taken from database technology
// and means that slices can gain fast, read-only access to data without needing to copy anything around.

// The problem with slices is that Rust wants to know the size of every object in your program, and slices are defined as not having a compile-time size
// References to the rescue, As mentioned before in the discussion of dinamically sized, slices sizes is fixed in memory/ These are made up of two usize components, a pointer and a lenght, That's why you  typically see slices referred to in their referred form (&[T]) such as strings slices that takes the notation &str

