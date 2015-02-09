// Name resolution

extern crate collections;
extern crate core;

use racer;

use racer::{SearchType, Match, Namespace};
use racer::SearchType::{ExactMatch, StartsWith};
use racer::MatchType::{Module, Function, Struct, Enum, FnArg, Trait, StructField, Impl, MatchArm};
use racer::Namespace::{TypeNamespace, ValueNamespace, BothNamespaces};

use racer::typeinf;
use racer::matchers;
use racer::codeiter;
use racer::ast;
use racer::util;
use racer::util::{symbol_matches, txt_matches, find_ident_end};
use racer::scopes;
use std::old_io::{File};
use std::{vec};
use std::iter::Iterator;
use std;

#[cfg(unix)]
const PATH_SEP: &'static str = ":";
#[cfg(windows)]
const PATH_SEP: &'static str = ";";

 fn search_struct_fields(searchstr: &str, structmatch: &Match,
                         search_type: SearchType) -> vec::IntoIter<Match> {
    let src = racer::load_file(&structmatch.filepath);
    let opoint = scopes::find_stmt_start(&*src, structmatch.point);
    let structsrc = scopes::end_of_next_scope(&src[opoint.unwrap()..]);

    let fields = ast::parse_struct_fields(String::from_str(structsrc), 
                                          racer::Scope::from_match(structmatch));

    let mut out = Vec::new();
    
    for (field, fpos, _) in fields.into_iter() {

        if symbol_matches(search_type, searchstr, &field[]) {
            out.push(Match { matchstr: field.to_string(),
                                filepath: structmatch.filepath.clone(),
                                point: fpos + opoint.unwrap(),
                                local: structmatch.local,
                                mtype: StructField,
                                contextstr: field.to_string(),
                                generic_args: Vec::new(), generic_types: Vec::new()
            });
        }
    }
    return out.into_iter();
}

pub fn search_for_impl_methods(implsearchstr: &str,
                           fieldsearchstr: &str, point: usize, 
                           fpath: &Path, local: bool,
                           search_type: SearchType) -> vec::IntoIter<Match> {
    
    debug!("searching for impl methods |{}| |{}| {:?}",implsearchstr, fieldsearchstr, fpath.as_str());

    let mut out = Vec::new();

    for m in search_for_impls(point, implsearchstr, fpath, local, true) {
        debug!("found impl!! |{:?}| looking for methods",m);
        let src = racer::load_file(&m.filepath);

        // find the opening brace and skip to it. 
        (&src[m.point..]).find_str("{").map(|n|{
            let point = m.point + n + 1;
            for m in search_scope_for_methods(point, &*src, fieldsearchstr, &m.filepath, search_type) {
                out.push(m);
            }
        });
    };
    return out.into_iter();
}

fn search_scope_for_methods(point: usize, src:&str, searchstr:&str, filepath:&Path, 
                      search_type: SearchType) -> vec::IntoIter<Match> {
    debug!("searching scope for methods {} |{}| {:?}",point, searchstr, filepath.as_str());
    
    let scopesrc = &src[point..];
    let mut out = Vec::new();
    for (blobstart,blobend) in codeiter::iter_stmts(scopesrc) { 
        let blob = &scopesrc[blobstart..blobend];

        if txt_matches(search_type, &format!("fn {}", searchstr)[], blob) 
            && typeinf::first_param_is_self(blob) {
            debug!("found a method starting |{}| |{}|",searchstr,blob);
            // TODO: parse this properly
            let start = blob.find_str(&format!("fn {}", searchstr)[]).unwrap() + 3;
            let end = find_ident_end(blob, start);
            let l = &blob[start..end];
            // TODO: make a better context string for functions
            blob.find_str("{").map(|n| { // only matches if is a method implementation
                let ctxt = &blob[..n -1];
                let m = Match {matchstr: l.to_string(),
                           filepath: filepath.clone(), 
                           point: point + blobstart + start,
                           local: true,
                           mtype: Function,
                           contextstr: ctxt.to_string(),
                           generic_args: Vec::new(), generic_types: Vec::new()
                };
                out.push(m);
            });
        }
    }
    out.into_iter()
}


pub fn search_for_impls(pos: usize, searchstr: &str, filepath: &Path, local: bool, include_traits: bool) -> vec::IntoIter<Match> {
    debug!("search_for_impls {}, {}, {:?}", pos, searchstr, filepath.as_str());
    let s = racer::load_file(filepath);
    let src = &s[pos..];

    let mut out = Vec::new();
    for (start,end) in codeiter::iter_stmts(src) { 
        let blob = &src[start..end];

        if blob.starts_with("impl") {
            blob.find_str("{").map(|n|{
                let mut decl = String::from_str(&blob[..n+1]);
                decl.push_str("}");
                if txt_matches(ExactMatch, searchstr, &decl[]) {
                    debug!("impl decl {}",decl);
                    let implres = ast::parse_impl(decl);

                    implres.name_path.map(|name_path| {
                        name_path.segments.last().map(|name| {
                            let m = Match {matchstr: name.name.clone(),
                                       filepath: filepath.clone(), 
                                       point: pos + start + 5,
                                       local: local,
                                       mtype: Impl,
                                       contextstr: "".to_string(),
                                       generic_args: Vec::new(), 
                                       generic_types: Vec::new()
                            };
                            out.push(m);
                        });
                    });

                    // find trait
                    if include_traits && implres.trait_path.is_some() {
                        let trait_path = implres.trait_path.unwrap();
                        let m = resolve_path(&trait_path, 
                                             filepath, pos + start, ExactMatch, TypeNamespace).nth(0);
                        debug!("found trait |{:?}| {:?}",
                                 trait_path, m);
                        m.map(|m| out.push(m));
                    }
                }
            });
        }
    }
    return out.into_iter();
}

// scope headers include fn decls, if let, while let etc..
fn search_scope_headers(point: usize, scopestart: usize, msrc:&str, searchstr:&str, 
                        filepath:&Path, search_type: SearchType, 
                        local: bool) -> vec::IntoIter<Match> {
    debug!("search_scope_headers for |{}| pt: {}",searchstr, scopestart);
    if let Some(stmtstart) = scopes::find_stmt_start(msrc, scopestart) { 
        let preblock = &msrc[stmtstart..scopestart];
        debug!("PHIL search_scope_headers preblock is |{}|",preblock);

        if preblock.starts_with("fn") || preblock.starts_with("pub fn") {
            return search_fn_args(stmtstart, scopestart, msrc, searchstr, filepath, search_type, local);

        // 'if let' can be an expression, so might not be at the start of the stmt
        } else if let Some(n) = preblock.find_str("if let") {
            let ifletstart = stmtstart + n;
            let s = (&msrc[ifletstart..scopestart+1]).to_string() + "}";
            if txt_matches(search_type, searchstr, &*s) {
                let mut out = matchers::match_if_let(&*s, 0, s.len(), searchstr, 
                                                     filepath, search_type, local);
                for m in out.iter_mut() {
                    m.point += ifletstart;
                }
                return out.into_iter();
            }

        } else if let Some(n) = util::find_last_str("match ", preblock) {
            // TODO: this code is crufty. refactor me!
            let matchstart = stmtstart + n;

            let matchstmt = typeinf::get_first_stmt(&msrc[matchstart..]);
            // The definition could be in the match LHS arms. Try to find this
            debug!("PHIL found a match statement, examining match arms |{}|", matchstmt);
            
            let masked_matchstmt = mask_matchstmt(matchstmt, 
                                                  scopestart+1 - matchstart);
            debug!("PHIL masked match stmt is |{}|", masked_matchstmt);


            // Locate the match arm LHS by finding the => just before point and then backtracking
            let mut rhs = &*masked_matchstmt;
            let mut arm = 0;
            while let Some(n) = rhs.find_str("=>") {
                debug!("PHIL match arm n is {}, {}, {}, {}", arm, n, matchstart, point);
                if arm + n + matchstart > point {
                    break;
                } else {
                    arm += n + 2;
                    rhs = &rhs[n+2..];
                }
            }
            debug!("PHIL matched arm rhs is |{}|", &masked_matchstmt[arm-2..]);

            let lhs_start = scopes::get_start_of_pattern(msrc, matchstart + arm -2);

            let lhs = &msrc[lhs_start..matchstart + arm - 2];

            // Now create a pretend match expression with just the one match arm in it
            let mut fauxmatchstmt = (&msrc[matchstart..scopestart]).to_string();
            fauxmatchstmt = fauxmatchstmt + "{";
            let faux_prefix_size = fauxmatchstmt.len();
            fauxmatchstmt = fauxmatchstmt + lhs + " => () };";

            debug!("PHIL arm lhs is |{}|", lhs);
            debug!("PHIL arm fauxmatchstmt is |{}|, {}", fauxmatchstmt, faux_prefix_size);
            let mut out = Vec::new();
            for &(start,end) in ast::parse_pat_idents(fauxmatchstmt).iter() {
                let (start,end) = (lhs_start + start - faux_prefix_size, 
                                   lhs_start + end - faux_prefix_size);
                let s = &msrc[start..end];

                if symbol_matches(search_type, searchstr, s) {

                    out.push(Match {matchstr: s.to_string(),
                                    filepath: filepath.clone(), 
                                    point: start, 
                                    local: local,
                                    mtype: MatchArm,
                                    contextstr: lhs.trim().to_string(),
                                    generic_args: Vec::new(), 
                                    generic_types: Vec::new()
                    });
                    if let SearchType::ExactMatch = search_type {
                        break;
                    }
                }
            }
            return out.into_iter();
        }
    }

    let out = Vec::new();
    return out.into_iter();
}

fn mask_matchstmt(matchstmt_src: &str, innerscope_start: usize) -> String {
    let s = scopes::mask_sub_scopes(&matchstmt_src[innerscope_start..]);
    (&matchstmt_src[..innerscope_start]).to_string() + &*s
}

#[test]
fn does_it() {
    let src : &str = "
    match foo {
        Some(a) => { something }
    }";
    let res = mask_matchstmt(src, src.find('{').unwrap()+1);
    debug!("PHIL res is |{}|",res);
}

fn search_fn_args(fnstart: usize, open_brace_pos: usize, msrc:&str, searchstr:&str, 
                   filepath:&Path, 
                   search_type: SearchType, local: bool) -> vec::IntoIter<Match> {
    let mut out = Vec::new();
    let mut fndecl = String::new();
    // wrap in 'impl blah {}' so that methods get parsed correctly too
    fndecl.push_str("impl blah {");
    let impl_header_len = fndecl.len();
    fndecl.push_str(&msrc[fnstart..(open_brace_pos+1)]);
    fndecl.push_str("}}");
    debug!("search_fn_args: found start of fn!! {} |{}| {}",fnstart, fndecl, searchstr);
    if txt_matches(search_type, searchstr, &fndecl[]) {
        let coords = ast::parse_fn_args(fndecl.clone());
        
        for &(start,end) in coords.iter() {
            let s = &fndecl[start..end];
            debug!("search_fn_args: arg str is |{}|", s);
            
            if symbol_matches(search_type, searchstr, s) {                
                let m = Match { matchstr: s.to_string(),
                                filepath: filepath.clone(),
                                point: fnstart + start - impl_header_len,
                                local: local,
                                mtype: FnArg,
                                contextstr: s.to_string(),
                                generic_args: Vec::new(), 
                                generic_types: Vec::new()
                };
                debug!("search_fn_args matched: {:?}", m);
                out.push(m);
            }
        }
    }
    return out.into_iter();
}

pub fn do_file_search(searchstr: &str, currentdir: &Path) -> vec::IntoIter<Match> {
    debug!("do_file_search {}",searchstr);
    let mut out = Vec::new();
    let srcpaths = std::env::var_string("RUST_SRC_PATH").unwrap_or("".to_string());
    debug!("do_file_search srcpaths {}",srcpaths);
    let mut v = (&srcpaths[]).split_str(PATH_SEP).collect::<Vec<_>>();
    v.push(currentdir.as_str().unwrap());
    debug!("do_file_search v is {:?}",v);
    for srcpath in v.into_iter() {
        match std::old_io::fs::readdir(&Path::new(srcpath)) {
            Ok(v) => {
                for fpath in v.iter() {
                    //debug!("fpath {}",fpath.as_str());
                    let fname = fpath.str_components().rev().next().unwrap().unwrap();
                    if fname.starts_with(&format!("lib{}", searchstr)[]) {
                        //debug!("Yeah found {}",fpath.as_str());
                        let filepath = Path::new(fpath).join_many(&[Path::new("lib.rs")]);
                        if File::open(&filepath).is_ok() {
                            let m = Match {matchstr: (&fname[3..]).to_string(),
                                           filepath: filepath.clone(), 
                                           point: 0,
                                           local: false,
                                           mtype: Module,
                                           contextstr: (&fname[3..]).to_string(),
                                           generic_args: Vec::new(), 
                                           generic_types: Vec::new()
                            };
                            out.push(m);
                        }
                    }

                    if fname.starts_with(searchstr) {
                        {
                            // try <name>/<name>.rs, like in the servo codebase
                            let filepath = Path::new(fpath).join_many(&[Path::new(format!("{}.rs", fname))]);

                            if File::open(&filepath).is_ok() {
                                let m = Match {matchstr: fname.to_string(),
                                               filepath: filepath.clone(), 
                                               point: 0,
                                               local: false,
                                               mtype: Module,
                                               contextstr: filepath.as_str().unwrap().to_string(),
                                               generic_args: Vec::new(), 
                                               generic_types: Vec::new()
                                };
                                out.push(m);
                            }
                        }
                        {
                            // try <name>/mod.rs
                            let filepath = Path::new(fpath).join_many(&[Path::new("mod.rs")]);
                            if File::open(&filepath).is_ok() {
                                let m = Match {matchstr: fname.to_string(),
                                               filepath: filepath.clone(), 
                                               point: 0,
                                               local: false,
                                               mtype: Module,
                                               contextstr: filepath.as_str().unwrap().to_string(),
                                               generic_args: Vec::new(), 
                                               generic_types: Vec::new()
                                };
                                out.push(m);
                            }
                        }
                        {
                            // try <name>/lib.rs
                            let filepath = Path::new(srcpath).join_many(&[Path::new("lib.rs")]);
                            if File::open(&filepath).is_ok() {
                                let m = Match {matchstr: fname.to_string(),
                                               filepath: filepath.clone(), 
                                               point: 0,
                                               local: false,
                                               mtype: Module,
                                               contextstr: filepath.as_str().unwrap().to_string(),
                                               generic_args: Vec::new(), 
                                               generic_types: Vec::new()
                                };
                                out.push(m);
                            }
                        }
                        {            
                            // try just <name>.rs
                            if fname.ends_with(".rs") {
                                let m = Match {matchstr: (&fname[..(fname.len()-3)]).to_string(),
                                               filepath: fpath.clone(),
                                               point: 0,
                                               local: false,
                                               mtype: Module,
                                               contextstr: fpath.as_str().unwrap().to_string(),
                                               generic_args: Vec::new(), 
                                               generic_types: Vec::new()
                                };
                                out.push(m);
                            }

                        }

                    }

                }
            }
            Err(_) => ()
        }
    }
    return out.into_iter();
}

pub fn search_crate_root(pathseg: &racer::PathSegment, modfpath: &Path, 
                         searchtype: SearchType, namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("search_crate_root |{:?}| {:?}", pathseg, modfpath.as_str());

    let crateroots = find_possible_crate_root_modules(&modfpath.dir_path());
    let mut out = Vec::new();
    for crateroot in crateroots.iter() {
        if crateroot == modfpath {
            continue;
        }
        debug!("going to search for {:?} in crateroot {:?}",pathseg, crateroot.as_str());
        for m in resolve_name(pathseg, crateroot, 0, searchtype, namespace) {
            out.push(m);
            if let ExactMatch = searchtype {
                break;
            }
        }
        break
    }
    return out.into_iter();
}

pub fn find_possible_crate_root_modules(currentdir: &Path) -> Vec<Path> {
    let mut res = Vec::new();
    
    {
        let filepath = currentdir.join_many(&[Path::new("lib.rs")]);
        if File::open(&filepath).is_ok() {
            res.push(filepath);
            return res;   // for now stop at the first match
        }
    }
    {
        let filepath = currentdir.join_many(&[Path::new("main.rs")]);
        if File::open(&filepath).is_ok() {
            res.push(filepath);
            return res;   // for now stop at the first match
        }
    }
    {
        // recurse up the directory structure
        let parentdir = currentdir.dir_path();
        if parentdir != *currentdir {
            res.push_all(&find_possible_crate_root_modules(&parentdir)[]);
            return res;   // for now stop at the first match
        }
    }

    return res;
}

pub fn search_next_scope(mut startpoint: usize, pathseg: &racer::PathSegment, 
                         filepath:&Path, search_type: SearchType, local: bool, 
                         namespace: Namespace) -> vec::IntoIter<Match> {
    let filesrc = racer::load_file(filepath);
    if startpoint != 0 {
        // is a scope inside the file. Point should point to the definition 
        // (e.g. mod blah {...}), so the actual scope is past the first open brace.
        let src = &filesrc[startpoint..];
        //debug!("search_next_scope src1 |{}|",src);
        // find the opening brace and skip to it. 
        src.find_str("{").map(|n|{
            startpoint = startpoint + n + 1;
        });
    }

    return search_scope(startpoint, startpoint, &*filesrc, pathseg, filepath, search_type, local, namespace);
}

pub fn get_crate_file(name: &str) -> Option<Path> {
    let srcpaths = std::env::var_string("RUST_SRC_PATH").unwrap();
    let v = (&srcpaths[]).split_str(PATH_SEP).collect::<Vec<_>>();
    for srcpath in v.into_iter() {
        {
            // try lib<name>/lib.rs, like in the rust source dir
            let cratelibname = format!("lib{}", name);
            let filepath = Path::new(srcpath).join_many(&[Path::new(cratelibname), 
                                                        Path::new("lib.rs")]);
            if File::open(&filepath).is_ok() {
                return Some(filepath);
            }
        }

        {
            // try <name>/lib.rs
            let filepath = Path::new(srcpath).join_many(&[Path::new(name),
                                                     Path::new("lib.rs")]);
            if File::open(&filepath).is_ok() {
                return Some(filepath);
            }
        }
    }
    return None;
}

pub fn get_module_file(name: &str, parentdir: &Path) -> Option<Path> {
    {            
        // try just <name>.rs
        let filepath = parentdir.join_many(&[Path::new(format!("{}.rs", name))]);
        if File::open(&filepath).is_ok() {
            return Some(filepath);
        }
    }
    {
        // try <name>/mod.rs
        let filepath = parentdir.join_many(&[Path::new(name),
                                            Path::new("mod.rs")]);
        if File::open(&filepath).is_ok() {
            return Some(filepath);
        }
    }

    return None;
}


pub fn search_scope(start: usize, point: usize, src: &str, 
                    pathseg: &racer::PathSegment, 
                    filepath:&Path, search_type: SearchType, local: bool,
                    namespace: Namespace) -> vec::IntoIter<Match> {

    let searchstr = &pathseg.name[];
    let mut out = Vec::new();

    debug!("searching scope {:?} start: {} point: {} '{}' {:?} {:?} local: {}",
           namespace, start, point, searchstr, 
           filepath.as_str(), search_type, local);
    
    let scopesrc = &src[start..];

    let mut skip_next_block = false;

    let mut delayed_use_globs = Vec::new();

    let codeit = codeiter::iter_stmts(scopesrc);
    let mut v = Vec::new();

    // collect up to point so we can search backwards for let bindings
    //  (these take precidence over local fn declarations etc..
    for (blobstart, blobend) in codeit {
        //  (e.g. #[cfg(test)])
        if skip_next_block {
            skip_next_block = false;
            continue;
        }

        let blob = &scopesrc[blobstart..blobend];

        // for now skip stuff that's meant for testing. Often the test
        // module hierarchy is incompatible with the non-test
        // hierarchy and we get into recursive loops
        if blob.starts_with("#[cfg(test)") {
            skip_next_block = true;
            continue;
        }

        v.push((blobstart,blobend));

        if blobstart > point {
            break;
        }
    }

    // search backwards from point for let bindings
    for &(blobstart, blobend) in v.iter().rev() {
        if blobstart > point {
            continue;
        }

        // for m in matchers::match_let_bindings(src, start+blobstart,
        //                                       start+blobend, 
        //                                       searchstr, 
        //                                       filepath, search_type, local) {
        for m in matchers::match_let(src, start+blobstart,
                                     start+blobend, 
                                     searchstr, 
                                     filepath, search_type, local).into_iter() {
            out.push(m);
            if let ExactMatch = search_type {
                return out.into_iter();
            }
        }
    }

    // now search from top of scope for items etc..
    let codeit = v.into_iter().chain(codeit);
    for (blobstart,blobend) in codeit { 
        // sometimes we need to skip blocks of code if the preceeding attribute disables it
        //  (e.g. #[cfg(test)])
        if skip_next_block {
            skip_next_block = false;
            continue;
        }

        let blob = &scopesrc[blobstart..blobend];

        // for now skip stuff that's meant for testing. Often the test
        // module hierarchy is incompatible with the non-test
        // hierarchy and we get into recursive loops
        if blob.starts_with("#[cfg(test)") {
            skip_next_block = true;
            continue;
        }

        let is_a_use_glob = (blob.starts_with("use") || blob.starts_with("pub use")) 
              && blob.find_str("::*").is_some();

        if is_a_use_glob {
            // globs are *really* expensive to process. delay them until later
            delayed_use_globs.push((blobstart, blobend));
            continue;
        }

        // Optimisation: if the search string is not in the blob and it is not 
        // a 'use glob', this cannot match so fail fast!
        if blob.find_str(searchstr).is_none() {
            continue;
        }

        // There's a good chance of a match. Run the matchers

        out = out + &*run_matchers_on_blob(src, start+blobstart, start+blobend, 
                                      searchstr,
                                      filepath, search_type, local, namespace);
        if let ExactMatch = search_type {
            if !out.is_empty() {
                return out.into_iter();
            }
        }
    }

    // finally process any use-globs that we skipped before
    for &(blobstart, blobend) in delayed_use_globs.iter() {
        // There's a good chance of a match. Run the matchers
        for m in run_matchers_on_blob(src, start+blobstart, start+blobend, 
                                      searchstr,filepath, search_type, 
                                      local, namespace).into_iter() {
            out.push(m);
            if let ExactMatch = search_type {
                return out.into_iter();
            }
        }
    }

    debug!("search_scope found matches {:?} {:?}",search_type, out);
    return out.into_iter();
}

fn run_matchers_on_blob(src: &str, start: usize, end: usize, searchstr: &str, 
                         filepath:&Path, search_type: SearchType, local: bool,
                         namespace: Namespace) -> Vec<Match> {
    let mut out = Vec::new();
    match namespace {
        TypeNamespace => 
            for m in matchers::match_types(src, start, 
                                           end, searchstr, 
                                           filepath, search_type, local) {
                out.push(m);
                if let ExactMatch = search_type {
                    return out;
                }
            },
        ValueNamespace => 
            for m in matchers::match_values(src, start,
                                            end, searchstr, 
                                            filepath, search_type, local) {
                out.push(m);
                if let ExactMatch = search_type {
                    return out;
                }
            },
        BothNamespaces => {
            for m in matchers::match_types(src, start,
                                           end, searchstr, 
                                           filepath, search_type, local) {
                out.push(m);
                if let ExactMatch = search_type {
                    return out;
                }
            }
            for m in matchers::match_values(src, start,
                                            end, searchstr, 
                                            filepath, search_type, local) {
                out.push(m);
                if let ExactMatch = search_type {
                    return out;
                }
                
            }
        }
    }
    return out;
}

fn search_local_scopes(pathseg: &racer::PathSegment, filepath: &Path, 
                       msrc: &str, point: usize, search_type: SearchType, 
                       namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("search_local_scopes {:?} {:?} {} {:?} {:?}",pathseg, filepath.as_str(), point, 
           search_type, namespace);

    let is_local = true;
    if point == 0 {
        // search the whole file
        return search_scope(0, 0, msrc, pathseg, filepath, search_type, is_local, namespace);
    } else {

        let mut out = Vec::new();

        let mut start = point;
        // search each parent scope in turn
        while start > 0 {
            start = scopes::scope_start(msrc, start);
            for m in search_scope(start, point, msrc, pathseg, filepath, search_type, is_local, namespace) {
                out.push(m);
                if let ExactMatch = search_type {
                    return out.into_iter();
                }
            }
            if start == 0 { 
                break; 
            }
            start = start-1;
            let searchstr = &pathseg.name[];

            // scope headers = fn decls, if let, match, etc..
            for m in search_scope_headers(point, start, msrc, searchstr, filepath, search_type, is_local){
                out.push(m);
                if let ExactMatch = search_type {
                    return out.into_iter();
                }
            };
        }
        return out.into_iter();
    }
}

pub fn search_prelude_file(pathseg: &racer::PathSegment, search_type: SearchType, 
                           namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("search_prelude file {:?} {:?} {:?}", pathseg, search_type, namespace);
//    debug!("PHIL searching prelude file, backtrace: {}",util::get_backtrace());

    let mut out : Vec<Match> = Vec::new();

    // find the prelude file from the search path and scan it
    let srcpaths = match std::env::var_string("RUST_SRC_PATH") { 
        Ok(paths) => paths,
        Err(_) => return out.into_iter()
    };

    let v = (&srcpaths[]).split_str(PATH_SEP).collect::<Vec<_>>();

    for srcpath in v.into_iter() {
        let filepath = Path::new(srcpath).join_many(&[Path::new("libstd"), 
                                                      Path::new("prelude"),
                                                      Path::new("v1.rs")]);
        if File::open(&filepath).is_ok() {
            let msrc = racer::load_file_and_mask_comments(&filepath);
            let is_local = true;
            for m in search_scope(0, 0, &msrc[], pathseg, &filepath, search_type, is_local, namespace){
                out.push(m);
            }
        }
    }
    return out.into_iter();
}

pub fn resolve_path_with_str(path: &racer::Path, filepath: &Path, pos: usize, 
                                   search_type: SearchType, namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("resolve_path_with_str {:?}", path);
    
    let mut out = Vec::new();

    // HACK
    if path.segments.len() == 1 && &path.segments[0].name[] == "str" {
        debug!("{:?} == {:?}", path.segments[0], "str");
        let str_pathseg = racer::PathSegment{ name: "Str".to_string(), types: Vec::new() };
        let str_match = resolve_name(&str_pathseg, filepath, pos, ExactMatch, namespace).nth(0);
        debug!("str_match {:?}", str_match);
        
        str_match.map(|str_match|{
            debug!("found Str, converting to str");
            let m = Match {matchstr: "str".to_string(),
                           filepath: str_match.filepath.clone(), 
                           point: str_match.point,
                           local: false,
                           mtype: Struct,
                           contextstr: "str".to_string(),
                           generic_args: Vec::new(), 
                           generic_types: Vec::new()
            };
            out.push(m);
        });
    } else {
        for m in resolve_path(path, filepath, pos, search_type, namespace) {
            out.push(m);
            if let ExactMatch = search_type {
                break;
            }
        }
    }
    return out.into_iter();
}

thread_local!(pub static SEARCH_STACK: Vec<Search> = Vec::new());

#[derive(PartialEq,Debug)]
pub struct Search {
    path: Vec<String>,
    filepath: String,
    pos: usize
}

pub fn is_a_repeat_search(new_search: &Search) -> bool {
    SEARCH_STACK.with(|v| {
        for s in v.iter() {
            if s == new_search {
                debug!("is a repeat search {:?} Stack: {:?}", new_search, v);
                return true;
            }
        }
        return false;
    })
}


pub fn resolve_name(pathseg: &racer::PathSegment, filepath: &Path, pos: usize, 
                    search_type: SearchType, namespace: Namespace) -> vec::IntoIter<Match> {

    let mut out = Vec::new();
    let searchstr = &pathseg.name[];
    
    debug!("resolve_name {} {:?} {} {:?} {:?}",searchstr, filepath.as_str(), pos, search_type, namespace);

    let msrc = racer::load_file_and_mask_comments(filepath);


    let is_exact_match = match search_type { ExactMatch => true, StartsWith => false };

    if (is_exact_match && searchstr == "std") || (!is_exact_match && "std".starts_with(searchstr)) {
        get_crate_file("std").map(|cratepath|{
            out.push(Match { matchstr: "std".to_string(),
                        filepath: cratepath.clone(), 
                        point: 0,
                        local: false,
                        mtype: Module,
                        contextstr: cratepath.as_str().unwrap().to_string(),
                        generic_args: Vec::new(), generic_types: Vec::new()
            });
        });
        
        if let ExactMatch = search_type {
            if !out.is_empty() {
                return out.into_iter();
            }
        }
    }


    for m in search_local_scopes(pathseg, filepath, &msrc[], pos,
                                          search_type, namespace) {
        out.push(m);
        if let ExactMatch = search_type {
            if !out.is_empty() {
                return out.into_iter();
            }
        }
    }

    for m in search_crate_root(pathseg, filepath, search_type, namespace) {
        out.push(m);
        if let ExactMatch = search_type {
            if !out.is_empty() {
                return out.into_iter();
            }
        }        
    }

    for m in search_prelude_file(pathseg, search_type, namespace) {
        out.push(m);
        if let ExactMatch = search_type {
            if !out.is_empty() {
                return out.into_iter();
            }
        }        
    }
    // filesearch. Used to complete e.g. extern crate blah or mod foo
    if let StartsWith = search_type {
        for m in do_file_search(searchstr, &filepath.dir_path()) {
            out.push(m);
        }
    }

    return out.into_iter();
}

pub fn resolve_path(path: &racer::Path, filepath: &Path, pos: usize, 
                  search_type: SearchType, namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("resolve_path {:?} {:?} {} {:?}", path, filepath.as_str(), pos, search_type);
    let len = path.segments.len();
    if len == 1 {
        let ref pathseg = path.segments[0];
        return resolve_name(pathseg, filepath, pos, search_type, namespace);
    } else {
        let mut out = Vec::new();
        let mut parent_path: racer::Path = path.clone();
        parent_path.segments.remove(len-1);
        let context = resolve_path(&parent_path, filepath, pos, ExactMatch, TypeNamespace).nth(0);
        context.map(|m| {
            match m.mtype {
                Module => {
                    debug!("searching a module '{}' (whole path: {:?})",m.matchstr, path);
                    let ref pathseg = path.segments[len-1];
                    for m in search_next_scope(m.point, pathseg, &m.filepath, search_type, false, namespace) { 
                        out.push(m);
                    }
                }
                Enum => {
                    let ref pathseg = path.segments[len-1];
                    debug!("searching an enum '{}' (whole path: {:?}) searchtype: {:?}",m.matchstr, path, search_type);

                    let filesrc = racer::load_file(&m.filepath);
                    let scopestart = scopes::find_stmt_start(&*filesrc, m.point).unwrap();
                    let scopesrc = &filesrc[scopestart..];
                    codeiter::iter_stmts(scopesrc).nth(0).map(|(blobstart,blobend)|{
                        for m in matchers::match_enum_variants(&*filesrc, 
                                                               scopestart+blobstart,
                                                               scopestart+ blobend,
                                                      &*pathseg.name, &m.filepath, search_type, true) {
                            debug!("Found enum variant: {}", m.matchstr);
                            out.push(m);
                        }
                    });
                }
                Struct => {
                    debug!("found a struct. Now need to look for impl");
                    for m in search_for_impls(m.point, &m.matchstr[], &m.filepath, m.local, false) {
                        debug!("found impl!! {:?}",m);
                        let ref pathseg = path.segments[len-1];
                        let src = racer::load_file(&m.filepath);
                        // find the opening brace and skip to it. 
                        (&src[m.point..]).find_str("{").map(|n|{
                            let point = m.point + n + 1;
                            for m in search_scope(point, point, &*src, pathseg, &m.filepath, search_type, m.local, namespace) {
                                out.push(m);
                            }
                        });
                        
                    };
                }
                _ => () 
            }
        });
        debug!("resolve_path returning {:?}",out);
        return out.into_iter();
    }
}

pub fn do_external_search(path: &[&str], filepath: &Path, pos: usize, search_type: SearchType, namespace: Namespace) -> vec::IntoIter<Match> {
    debug!("do_external_search path {:?} {:?}",path, filepath.as_str());
    let mut out = Vec::new();
    if path.len() == 1 {
        let searchstr = path[0];
        // hack for now
        let pathseg = racer::PathSegment{name: searchstr.to_string(), 
                                         types: Vec::new()};

        for m in search_next_scope(pos, &pathseg, filepath, search_type, false, namespace) { 
            out.push(m);
        }

        get_module_file(searchstr, &filepath.dir_path()).map(|path|{
            out.push(Match {matchstr: searchstr.to_string(),
                           filepath: path.clone(), 
                           point: 0,
                           local: false,
                           mtype: Module,
                           contextstr: path.as_str().unwrap().to_string(),
                           generic_args: Vec::new(),
                           generic_types: Vec::new()
                           });
        });

    } else {
        let parent_path = &path[..(path.len()-1)];
        let context = do_external_search(parent_path, filepath, pos, ExactMatch, TypeNamespace).nth(0);
        context.map(|m| {
            match m.mtype {
                Module => {
                    debug!("found an external module {}",m.matchstr);
                    let searchstr = path[path.len()-1];
                    let pathseg = racer::PathSegment{name: searchstr.to_string(), 
                                         types: Vec::new()};
                    for m in search_next_scope(m.point, &pathseg, &m.filepath, search_type, false, namespace) {
                        out.push(m);
                    }
                }

                Struct => {
                    debug!("found a pub struct. Now need to look for impl");
                    for m in search_for_impls(m.point, &m.matchstr[], &m.filepath, m.local, false) {
                        debug!("found  impl2!! {}",m.matchstr);
                        let searchstr = path[path.len()-1];
                        let pathseg = racer::PathSegment{name: searchstr.to_string(), 
                                         types: Vec::new()};
                        debug!("about to search impl scope...");
                        for m in search_next_scope(m.point, &pathseg, &m.filepath, search_type, false, namespace) {
                            out.push(m);
                        }
                    };
                }
                _ => ()
            }
        });
    }
    return out.into_iter();
}

pub fn search_for_field_or_method(context: Match, searchstr: &str, search_type: SearchType) -> vec::IntoIter<Match> {
    let m = context;
    let mut out = Vec::new();
    match m.mtype {
        Struct => {
            debug!("got a struct, looking for fields and impl methods!! {}",m.matchstr);
            for m in search_struct_fields(searchstr, &m, search_type) {
                out.push(m);
            }
            for m in search_for_impl_methods(&m.matchstr[],
                                    searchstr,
                                    m.point,
                                    &m.filepath,
                                    m.local,
                                    search_type) {
                out.push(m);
            }
        },
        Enum => {
            debug!("got an enum, looking for impl methods {}",m.matchstr);
            for m in search_for_impl_methods(&m.matchstr[],
                                    searchstr,
                                    m.point,
                                    &m.filepath,
                                    m.local,
                                    search_type) {
                out.push(m);
            }
        },
        Trait => {
            debug!("got a trait, looking for methods {}",m.matchstr);
            let src = racer::load_file(&m.filepath);
            (&src[m.point..]).find_str("{").map(|n|{
                let point = m.point + n + 1;
                for m in search_scope_for_methods(point, &*src, searchstr, &m.filepath, search_type) {
                    out.push(m);
                }
            });
        }
        _ => { debug!("WARN!! context wasn't a Struct, Enum or Trait {:?}",m);}
    };
    return out.into_iter();
}
