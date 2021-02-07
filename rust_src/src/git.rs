use std::path::Path;

use git2::{Blame, BlameHunk, Commit, Oid, Repository};

use lisp_macros::lisp_fn;

use lisp::multibyte::LispStringRef;

use lisp::remacs_sys::{
    call0, call1, call2, call3, make_string, make_string_from_utf8, EmacsInt, Ffuncall,
};

use lisp::{
    lisp::{ExternalPtr, LispObject},
    symbol::intern,
};

use libc;

pub fn git_repo(path: LispStringRef) -> Repository {
    match Repository::init(Path::new(path.to_utf8().as_str())) {
        Ok(repo) => repo,
        Err(e) => {
            error!("Error initializing repository {:?}", e);
        }
    }
}

pub fn git_blame_file_rs<'a>(repo: &'a Repository, path: LispStringRef) -> Blame<'a> {
    match repo.blame_file(&Path::new(path.to_utf8().as_str()), None) {
        Err(e) => {
            println!("{:?}", e);
            error!("Error getting blame {:?}", e);
        }
        Ok(b) => b,
    }
}

// pub fn git_blame_hunk<'a>(blame: &'a Blame, line: EmacsInt) -> BlameHunk<'a> {
//     match blame.get_line(line as usize) {
//         None => error!("Error getting hunk {:?}", "summary"),
//         Some(hunk) => hunk,
//     }
// }

#[lisp_fn]
pub fn git_blame(path: LispStringRef, file: LispStringRef) -> LispObject {
    unsafe {
        let p = git_repo(path);

        let buf = call0(LispObject::from(intern("current-buffer")));

        let blame_iter = git_blame_file_rs(&p, file);

        for hunk in blame_iter.iter() {
            let oid = hunk.orig_commit_id();
            let orig_rev = git_commit(&p, oid);

            let rev_list = list!(
                ("summary", git_summary(&orig_rev)),
                ("author", git_author(&orig_rev)),
                ("committer-tz", git_committer_tz(&orig_rev)),
                ("committer-time", git_committer_time(&orig_rev)),
                ("committer-mail", git_committer_mail(&orig_rev)),
                ("committer", git_committer(&orig_rev)),
                ("author-tz", git_author_tz(&orig_rev)),
                ("author-time", git_author_time(&orig_rev)),
                ("author-mail", git_author_mail(&orig_rev))
            );

            let id = make_string(oid.as_bytes().to_vec().as_mut_ptr() as *mut libc::c_char, 20);
            let chunk_list = list!(("commit", id));

            return id;

            // let chunk_list = list!(
            //     ("commit", ),
            //     ("orig-line", ),
            //     ("final-line", ),
            //     ("num-lines", )
            // );

            // let chunk: LispObject = call0(LispObject::from(intern("my-magit-make-chunk")));

            // call3(
            //     LispObject::from(intern("magit-blame--make-overlays")),
            //     buf,
            //     chunk,
            //     rev_list,
            // );
        }
    }
    LispObject::from(1)
}

// #[lisp_fn]
// pub fn git_make_chunk() -> LispObject {
//     unsafe {
//         return call!(
//             LispObject::from(intern("magit-blame-chunk")),
//             LispObject::from(intern("orig-rev")),
//             LispObject::from(121)
//         );
//     }
// }

pub fn git_commit<'a>(repo: &'a Repository, oid: Oid) -> Commit<'a> {
    match repo.find_commit(oid) {
        Ok(commit) => commit,
        Err(e) => {
            error!("Error initializing repository {:?}", e);
        }
    }
}

pub fn git_summary(c: &Commit) -> LispObject {
    unsafe {
        match c.summary() {
            None => error!("Error getting author name {:?}", "summary"),

            Some(s) => make_string_from_utf8(
                s.as_ptr() as *const libc::c_char,
                s.chars().count() as isize,
            ),
        }
    }
}

pub fn git_author(c: &Commit) -> LispObject {
    let a = c.author();
    unsafe {
        match a.name() {
            None => error!("Error getting author name {:?}", "summary"),
            Some(s) => make_string_from_utf8(
                s.as_ptr() as *const libc::c_char,
                s.chars().count() as isize,
            ),
        }
    }
}

pub fn git_author_mail(c: &Commit) -> LispObject {
    let a = c.author();
    unsafe {
        match a.email() {
            None => error!("Error getting author name {:?}", "foo"),
            Some(s) => make_string_from_utf8(
                s.as_ptr() as *const libc::c_char,
                s.chars().count() as isize,
            ),
        }
    }
}

pub fn git_author_time(c: &Commit) -> LispObject {
    let a = c.author();
    unsafe {
        let fuu = a.when().seconds();
        let seconds = fuu.to_string();
        let seconds = seconds.as_str();

        make_string_from_utf8(
            seconds.as_ptr() as *const libc::c_char,
            seconds.chars().count() as isize,
        )
    }
}

pub fn git_author_tz(c: &Commit) -> LispObject {
    let a = c.author();
    unsafe {
        let fuu = a.when().offset_minutes();
        let seconds = fuu.to_string();
        let seconds = seconds.as_str();

        make_string_from_utf8(
            seconds.as_ptr() as *const libc::c_char,
            seconds.chars().count() as isize,
        )
    }
}

pub fn git_committer(c: &Commit) -> LispObject {
    let a = c.committer();
    unsafe {
        match a.name() {
            None => error!("Error getting author name {:?}", "foo"),
            Some(s) => make_string_from_utf8(
                s.as_ptr() as *const libc::c_char,
                s.chars().count() as isize,
            ),
        }
    }
}

pub fn git_committer_mail(c: &Commit) -> LispObject {
    let a = c.committer();
    unsafe {
        match a.email() {
            None => error!("Error getting author name {:?}", "foo"),
            Some(s) => make_string_from_utf8(
                s.as_ptr() as *const libc::c_char,
                s.chars().count() as isize,
            ),
        }
    }
}

pub fn git_committer_time(c: &Commit) -> LispObject {
    let a = c.committer();
    unsafe {
        let fuu = a.when().seconds();
        let seconds = fuu.to_string();
        let seconds = seconds.as_str();

        make_string_from_utf8(
            seconds.as_ptr() as *const libc::c_char,
            seconds.chars().count() as isize,
        )
    }
}

pub fn git_committer_tz(c: &Commit) -> LispObject {
    let a = c.committer();
    unsafe {
        let fuu = a.when().offset_minutes();
        let seconds = fuu.to_string();
        let seconds = seconds.as_str();

        make_string_from_utf8(
            seconds.as_ptr() as *const libc::c_char,
            seconds.chars().count() as isize,
        )
    }
}

include!(concat!(env!("OUT_DIR"), "/git_exports.rs"));
