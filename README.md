# OptArrayVec

A vector-like container that does not store its length directly, but rather retrieves them through inner options.
I highly discourage using this if `size_of::<Yourtype> != size_of::<Option<Yourtype>>`

# MSRV
Currently,  1.58.1