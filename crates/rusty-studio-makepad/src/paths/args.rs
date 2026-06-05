use std::path::PathBuf;

pub(crate) fn project_path_from_args() -> Option<PathBuf> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--project" {
            return args.next().map(PathBuf::from);
        }
    }
    None
}

pub(crate) fn initial_graph_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--graph" {
            return args.next();
        }
    }
    None
}

pub(crate) fn initial_issue_check_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--issue" {
            return args.next();
        }
    }
    None
}

pub(crate) fn initial_node_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--node" {
            return args.next();
        }
    }
    None
}

pub(crate) fn initial_edge_id_from_args() -> Option<String> {
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        if arg == "--edge" {
            return args.next();
        }
    }
    None
}
