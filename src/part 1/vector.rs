// Vector (Vec<T>) are growable lists of T. Using vectors is extremely common in Rust code.
// These incur a small runtime penalty compared to arrays because of the extra bookkeeping that must to be done to enable their size t change
// their size over the time.

// let's re implement an extended version of grep-line project.

fn main(){
  let ctx_lines = 2;
  let needle = "oo";
  let haystack = "\
  Every face, every shop,
  bedroom window, public-house, and
  dark square is a picture
  feverishly turned--in search of what?
  It is the same with books.
  What do we seek
  throught millions of pages?";

  let mut tags: Vec<usize> = vec![];  // holds line numbers where matches occur
  let mut ctx: Vec<Vec<(usize, String)>> = vec![]; // contains a vector per match to hold the context lines.

  for (i, line) in haystack.lines().enumerate() {
    if line.contains(needle) {
      tags.push(i);
      let v = Vec::with_capacity(2 * ctx_lines + 1);
      ctx.push(v);
    }
  }

  if tags.is_empty(){
    return;
  }

  for (i, line) in haystack.lines().enumerate(){
    for(j, tag) in tags.iter().enumerate() {
      let lower_bound = tag.saturating_sub(ctx_lines);
      println!("{}", lower_bound);
      let upper_bound = tag + ctx_lines;
      println!("result: {}, tag: {}, ctx_lines: {}", upper_bound, tag, ctx_lines);
      if(i >=lower_bound) && (i<= upper_bound) {
        let line_as_string = String::from(line);
        let local_ctx = (i, line_as_string);
        ctx[j].push(local_ctx);

      }
    }
  }

  for local_ctx in ctx.iter() {
    for &(i, ref line) in local_ctx.iter(){
      let line_num = i + 1;
      println!("{} : {}", line_num, line);
    }
  }
}