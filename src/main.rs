extern crate regex;

#[global_allocator]
static ALLOCATOR: std::alloc::System = std::alloc::System;

use std::collections::HashMap;
use std::fs::File;
use std::io::{ prelude::*, self };
use std::env;

use regex::Regex;

mod graph;
use graph::*;

mod layout;
use layout::*;

pub type Stemma<V> = DirectedGraph<V, u32>;

fn main() -> io::Result<()> {
    let mut input: Box<Read> = Box::new(io::stdin());
    let mut output: Box<Write> = Box::new(io::stdout());
    
    let mut args = env::args();
    args.next(); // clear executable name
    while let Some(param) = args.next() {
        match &param as &str {
            "-i" | "--input" => {
                input = Box::new(File::open(args.next().unwrap_or_else(|| help_text(1)))?)
            }
            "-o" | "--output" => {
                output = Box::new(File::create(args.next().unwrap_or_else(|| help_text(1)))?)
            }
            "--help" | "-h" | _ => {
                help_text(0);
            }
        }
    }

    let mut graph = Stemma::new();

    let mut nodes = HashMap::new();

    let link_matcher = Regex::new(r"Link: (\S+) [-=]> (\S+)\s+\|\s+(\d+)").unwrap();

    let mut s = String::new();
    input.read_to_string(&mut s)?;
    drop(input);
    for caps in link_matcher.captures_iter(&s) {
        if !nodes.contains_key(&caps[1]) {
            nodes.insert(caps[1].to_owned(), graph.add_vertex(caps[1].to_owned()));
        }
        if !nodes.contains_key(&caps[2]) {
            nodes.insert(caps[2].to_owned(), graph.add_vertex(caps[2].to_owned()));
        }
        graph.add_edge(nodes[&caps[1]], nodes[&caps[2]], caps[3].parse()
                .expect(&format!("Could not parse '{}' as an integer", &caps[3])));
    }

    svg_output(layout(graph), &mut output);

    Ok(())
}

fn help_text(code: i32) -> ! {
    eprintln!("Usage: {} {}", env::args().next().unwrap(), include_str!("help-text"));
    std::process::exit(code);
}

fn svg_output((layed_out, width, height): (LayoutGraph<String>, u32, u32), output: &mut Write) {
    writeln!(output, 
        concat!(
            "<svg xmlns='http://www.w3.org/2000/svg' ",
            "width='{}' height='{}' ",
            "viewBox='-10 -10 {} {}'>"
        ),
        (width+20)*3, (height+20)*3,
        width+20, height+20
    ).unwrap();
    writeln!(output, "{}", include_str!("svg-header")).unwrap();

    for edge in layed_out.edges() {
        let from = layed_out.vertex(edge.from()).data.1;
        let to = layed_out.vertex(edge.to()).data.1;
        writeln!(output, 
            "<path marker-end='url(#head)' class='link' d='M{},{} L{},{}' />",
            from.x, from.y, to.x, to.y
        ).unwrap();
    }

    for vertex in layed_out.vertices() {
        writeln!(output, concat!(
            "<circle cx='{x}' cy='{y}' r='5' fill='white' />",
            "<text x='{x}' y='{y}' class='node'>{}</text>"
        ), vertex.data.0, x=vertex.data.1.x, y=vertex.data.1.y).unwrap();
    }

    writeln!(output, "</svg>").unwrap();
}