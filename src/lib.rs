#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn query_db() {
        // See http://libcddb.sourceforge.net/tutorial.html#section5 for
        // the sample length and sample offsets used here:
        let disc_length = 3822;

        let disc = unsafe { cddb_disc_new() };
        assert_eq!(disc.is_null(), false);

        unsafe { cddb_disc_set_length(disc, disc_length) }
        // TODO: ...
    }
}
