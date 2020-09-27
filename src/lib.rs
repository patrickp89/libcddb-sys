#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn query_db() {
        // See http://libcddb.sourceforge.net/tutorial.html#section5 for
        // the sample length and offsets used here:
        let disc_length = 3822;
        let frame_offsets = vec![
            150, 28690, 51102, 75910, 102682, 121522, 149040, 175772, 204387, 231145, 268065,
        ];

        // initialize a connection to our mock CDDB server (see
        // http://libcddb.sourceforge.net/tutorial.html#section4 for details):
        let conn = unsafe { cddb_new() };
        assert!(!conn.is_null());

        let server_name = CString::new("gnudb.gnudb.org").expect("Could not create CString!");
        unsafe { cddb_set_server_name(conn, server_name.as_ptr()) };
        let actual_server_name = unsafe { CStr::from_ptr(cddb_get_server_name(conn)) };
        assert_eq!(
            actual_server_name.to_str().expect("to_str() failed!"),
            server_name.to_str().expect("to_str() failed!")
        );

        let server_port = 8880 as u16;
        unsafe { cddb_set_server_port(conn, server_port as i32) };
        let actual_server_port: u32 = unsafe { cddb_get_server_port(conn) };
        assert_eq!(actual_server_port, server_port as u32);

        // initialize disc and tracks:
        let disc = unsafe { cddb_disc_new() };
        assert!(!disc.is_null());

        unsafe { cddb_disc_set_length(disc, disc_length) }
        let actual_length = unsafe { cddb_disc_get_length(disc) };
        assert_eq!(actual_length, disc_length);

        for offset in frame_offsets.iter() {
            let track = unsafe { cddb_track_new() };
            assert!(!track.is_null());

            unsafe { cddb_track_set_frame_offset(track, *offset) };
            let actual_offset: i32 = unsafe { cddb_track_get_frame_offset(track) };
            assert_eq!(actual_offset, *offset);

            let track_count_before: i32 = unsafe { cddb_disc_get_track_count(disc) };
            unsafe { cddb_disc_add_track(disc, track) };
            let track_count_after: i32 = unsafe { cddb_disc_get_track_count(disc) };
            assert_eq!(track_count_before + 1, track_count_after);
        }

        // query the CD database:
        /* TODO: mock the server responses!
        let matches: i32 = unsafe { cddb_query(conn, disc) };
        assert_ne!(matches, -1);

        // check the first match:
        let raw_disc_title = unsafe { CStr::from_ptr(cddb_disc_get_title(disc)) };
        let disc_title = raw_disc_title.to_str().expect("to_str() failed!");
        assert_ne!(disc_title, "");

        // and its first track:
        let first_track = unsafe { cddb_disc_get_track_first(disc) };
        assert!(!first_track.is_null());
        let raw_first_track_title = unsafe { CStr::from_ptr(cddb_track_get_artist(first_track)) };
        // TODO: cddb_track_get_artist() works, but cddb_track_get_title() does not?!
        let first_track_title = raw_first_track_title.to_str().expect("to_str() failed!");
        assert_ne!(first_track_title, "");
        println!("First track's title is {:?}", first_track_title);

        // check the second match (i.e. the second matching disc):
        let second_query_matches: i32 = unsafe { cddb_query_next(conn, disc) };
        assert_ne!(second_query_matches, -1);
        let second_raw_disc_title = unsafe { CStr::from_ptr(cddb_disc_get_title(disc)) };
        let second_disc_title = second_raw_disc_title.to_str().expect("to_str() failed!");
        assert_ne!(second_disc_title, "");
        */

        // free pointer:
        unsafe { cddb_disc_destroy(disc) };
    }
}
